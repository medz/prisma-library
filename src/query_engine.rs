use std::{sync::Arc, path::PathBuf, collections::{HashMap, BTreeMap}, ffi::CStr};

use psl::Diagnostics;
use query_core::{schema::QuerySchema, QueryExecutor};
use request_handlers::{load_executor, ConnectorMode};
use tokio::sync::RwLock;

use crate::error::PrismaError;

pub struct PrismaQueryEngine {
    inner: RwLock<Inner>,
}

enum Inner {
    Builder(EngineBuilder),
    Connected(ConnectedEngine),
}

struct EngineBuilder {
    schema: Arc<psl::ValidatedSchema>,
    config_dir: PathBuf,
    environment: HashMap<String, String>,
}

struct ConnectedEngine {
    builder: EngineBuilder,
    query_schema: Arc<QuerySchema>,
    executor: Box<dyn QueryExecutor + Send + Sync>,
}

impl ConnectedEngine {
    pub fn executor(&self) -> &(dyn QueryExecutor + Send + Sync) {
        self.executor.as_ref()
    }

    pub fn query_schema(&self) -> &Arc<QuerySchema> {
        &self.query_schema
    }
}

impl Inner {
    fn as_builder(&self) -> Result<&EngineBuilder, PrismaError> {
        match self {
            Inner::Builder(builder) => Ok(builder),
            Inner::Connected(_) => Err(PrismaError::AlreadyConnected),
        }
    }

    fn as_connected(&self) -> Result<&ConnectedEngine, PrismaError> {
        match self {
            Inner::Builder(_) => Err(PrismaError::NotConnected),
            Inner::Connected(engine) => Ok(engine),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConstructorOptions {
    datamodel: String,

    #[serde(default)]
    datasource_overrides: BTreeMap<String, String>,

    config_dir: PathBuf,
}

#[repr(C)]
pub enum PrismaQueryEngineConstructorResponse {
    Ok(*mut PrismaQueryEngine),
    Err(*const libc::c_char),
}

impl From<PrismaError> for PrismaQueryEngineConstructorResponse {
    fn from(err: PrismaError) -> Self {
        Self::Err(err.to_c_char())
    }
}

impl From<PrismaQueryEngine> for PrismaQueryEngineConstructorResponse {
    fn from(engine: PrismaQueryEngine) -> Self {
        Self::Ok(Box::into_raw(Box::new(engine)))
    }
}

impl From<Diagnostics> for PrismaQueryEngineConstructorResponse {
    fn from(value: Diagnostics) -> Self {
        let errors = value.errors().to_vec();
        let errors = errors.into_iter().map(|err| err.message());
        let errors: Vec<_> = errors.collect();

        let warnings = value.warnings().to_vec();
        let warnings = warnings.into_iter().map(|err| err.message());
        let warnings: Vec<_> = warnings.collect();

        let error = PrismaError::Json(
            serde_json::json!({
                "errors": errors,
                "warnings": warnings,
            })
            .to_string(),
        );

        Self::from(error)
    }
}

/// creates a new [PrismaQueryEngine].
#[no_mangle]
pub extern "C" fn prisma_query_engine_constructor(
    environment: *const libc::c_char,
    options: *const libc::c_char,
) -> PrismaQueryEngineConstructorResponse {
    let environment = unsafe { CStr::from_ptr(environment) };
    let environment = environment.to_str().unwrap();
    let environment: HashMap<String, String> = serde_json::from_str(environment).unwrap();

    let options = unsafe { CStr::from_ptr(options) };
    let options = options.to_str().unwrap();

    let ConstructorOptions {
        datamodel,
        datasource_overrides,
        config_dir,
    } = serde_json::from_str(options).unwrap();

    let overrides: Vec<_, _> = datasource_overrides.into_iter().collect();

    let mut schema = psl::validate(datamodel.into());
    let config = &mut schema.configuration;
    let preview_features = &mut config.preview_features();

    schema
        .diagnostics
        .to_result()
        .map_err(|err| PrismaQueryEngineConstructorResponse::from(err))?;

    config
        .resolve_datasource_urls_query_engine(
            &overrides,
            |key| environment.get(key).map(ToString::to_string),
            false,
        )
        .map_err(|err| PrismaQueryEngineConstructorResponse::from(err))?;

    config
        .validate_that_one_datasource_is_provided()
        .map_err(|err| PrismaQueryEngineConstructorResponse::from(err))?;

    let builder = EngineBuilder {
        schema: Arc::new(schema),
        config_dir,
        environment,
    };

    let inner = Inner::Builder(builder);
    let inner = RwLock::new(inner);

    let engine = PrismaQueryEngine { inner };

    PrismaQueryEngineConstructorResponse::from(engine)
}

/// Connects the [PrismaQueryEngine] to the database.
#[no_mangle]
pub extern "C" fn prisma_query_engine_connect(engine: *mut PrismaQueryEngine) -> *const libc::c_char {
    let engine = unsafe { Box::from_raw(engine) };

    let mut inner = engine.inner.write().await;
    let builder = inner.as_builder()?;
    let arced_schema = Arc::clone(&builder.schema);
    let arced_schema_2 = Arc::clone(&builder.schema);

    let url = {
        let datasource = builder
            .schema
            .configuration
            .datasources
            .first()
            .ok_or_else(|| PrismaError::Configuration("No datasource provided".into()))?;

        datasource
            .load_url_with_config_dir(
                &builder.config_dir,
                |key| builder.environment.get(key).map(ToString::to_string),
            )
            .map_err(|err| {
                let errors = value.errors().to_vec();
                let errors = errors.into_iter().map(|err| err.message());
                let errors: Vec<_> = errors.collect();

                let warnings = value.warnings().to_vec();
                let warnings = warnings.into_iter().map(|err| err.message());
                let warnings: Vec<_> = warnings.collect();

                let error = PrismaError::Json(
                    serde_json::json!({
                        "errors": errors,
                        "warnings": warnings,
                    })
                    .to_string(),
                );

                error.to_c_char()
            })?
    };

    let engine = async move {
        let datasource = arced_schema
            .configuration
            .datasources
            .first()
            .ok_or_else(|| PrismaError::Configuration("No datasource provided".into()))?;
        let preview_features = arced_schema.configuration.preview_features();

        let executor = load_executor(ConnectorMode::Rust,&datasource, &url, preview_features).await?;

        let query_schema_fut = tokio::runtime::Handle::current()
            .spawn_blocking(move || {
                query_core::schema::build(arced_schema_2, true)
            }).await;
    };
}