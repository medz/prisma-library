use std::{panic::AssertUnwindSafe, sync::Arc};

use datamodel::{dml::Datamodel, ValidatedConfiguration};
use futures::{Future, FutureExt};
use prisma_models::InternalDataModelBuilder;
use query_core::{schema::QuerySchema, schema_builder, QueryExecutor};
use tokio::sync::RwLock;
use user_facing_errors::Error;

use crate::{error::ApiError, string_to_c_char};

type Result<T> = std::result::Result<T, ApiError>;
type Executor = Box<dyn QueryExecutor + Send + Sync>;

/// Holding the information to reconnect the engine if needed.
#[derive(Debug, Clone)]
struct EngineDatamodel {
    ast: Datamodel,
    raw: String,
}

/// Internal structure for querying and reconnecting with the engine.
pub struct ConnectedEngine {
    datamodel: EngineDatamodel,
    query_schema: Arc<QuerySchema>,
    executor: Executor,
}

/// Everything needed to connect to the database and have the core running.
struct EngineBuilder {
    datamodel: EngineDatamodel,
    config: ValidatedConfiguration,
}

/// The state of the engine.
enum Inner {
    /// Not connected, holding all data to form a connection.
    Builder(EngineBuilder),
    /// A connected engine, holding all data to disconnect and form a new
    /// connection. Allows querying when on this state.
    Connected(ConnectedEngine),
}

pub struct Engine {
    inner: RwLock<Inner>,
}

impl Inner {
    // Returns a builder if the engine is not connected
    pub fn as_builder(&self) -> Result<&EngineBuilder> {
        match self {
            Inner::Builder(ref builder) => Ok(builder),
            Inner::Connected(_) => Err(ApiError::AlreadyConnected),
        }
    }

    /// Returns the engine if connected
    pub fn as_engine(&self) -> Result<&ConnectedEngine> {
        match self {
            Inner::Builder(_) => Err(ApiError::NotConnected),
            Inner::Connected(ref engine) => Ok(engine),
        }
    }
}

impl Engine {
    /// Creates a new engine.
    pub fn new(datamodel: String, datasource_url: String) -> Result<Self> {
        let config = datamodel::parse_configuration(&datamodel)
            .and_then(|mut config| {
                for datasource in &mut config.subject.datasources {
                    datasource.url.value = Some(datasource_url.clone());
                    datasource.url.from_env_var = None;
                }

                Ok(config)
            })
            .map_err(|errors| ApiError::conversion(errors, &datamodel))?;

        config
            .subject
            .validate_that_one_datasource_is_provided()
            .map_err(|errors| ApiError::conversion(errors, &datamodel))?;

        let ast = datamodel::parse_datamodel(&datamodel)
            .map_err(|errors| ApiError::conversion(errors, &datamodel))?
            .subject;

        let datamodel = EngineDatamodel {
            ast,
            raw: datamodel,
        };
        let builder = EngineBuilder { datamodel, config };

        let engine = Engine {
            inner: RwLock::new(Inner::Builder(builder)),
        };

        Ok(engine)
    }

    /// Connect to the database, allow queries to be run.
    pub async fn connect(&self) -> Result<()> {
        async_panic_to_error(async {
            let mut inner = self.inner.write().await;
            let builder = inner.as_builder()?;

            let engine = async move {
                // We only support one data source & generator at the moment, so take the first one (default not exposed yet).
                let data_source = builder
                    .config
                    .subject
                    .datasources
                    .first()
                    .ok_or_else(|| ApiError::configuration("No valid data source found"))?;

                let url = data_source
                    .url
                    .value
                    .as_ref()
                    .ok_or_else(|| ApiError::configuration("No valid data source url found"))?;

                let (db_name, executor) =
                    query_core::executor::load(data_source, &[], &url).await?;
                let connector = executor.primary_connector();
                connector.get_connection().await?;

                // Build internal data model
                let internal_data_model =
                    InternalDataModelBuilder::from(&builder.datamodel.ast).build(db_name);

                let query_schema = schema_builder::build(
                    internal_data_model,
                    true, // enable raw queries
                    data_source.capabilities(),
                    (&[]).to_vec(),
                    data_source.referential_integrity(),
                );

                Ok(ConnectedEngine {
                    datamodel: builder.datamodel.clone(),
                    query_schema: Arc::new(query_schema),
                    executor,
                }) as Result<ConnectedEngine>
            }
            .await?;

            *inner = Inner::Connected(engine);

            Ok(())
        })
        .await?;

        Ok(())
    }

    /// Disconnect and drop the core. Can be reconnected later with `#connect`.
    pub async fn disconnect(&self) -> Result<()> {
        async_panic_to_error(async {
            let mut inner = self.inner.write().await;
            let engine = inner.as_engine()?;

            let config = datamodel::parse_configuration(&engine.datamodel.raw)
                .map_err(|errors| ApiError::conversion(errors, &engine.datamodel.raw))?;

            let builder = EngineBuilder {
                datamodel: engine.datamodel.clone(),
                config,
            };

            *inner = Inner::Builder(builder);

            Ok(())
        })
        .await
    }
}

async fn async_panic_to_error<F, R>(future: F) -> Result<R>
where
    F: Future<Output = Result<R>>,
{
    match AssertUnwindSafe(future).catch_unwind().await {
        Ok(result) => result,
        Err(err) => match Error::extract_panic_message(err) {
            Some(message) => {
                let message = format!("PANIC: {}", message);
                let message = string_to_c_char(&message);

                Err(ApiError::Core(message))
            }
            None => {
                let err = "PANIC: unknown panic";
                let err = string_to_c_char(err);
                Err(ApiError::Core(err))
            }
        },
    }
}
