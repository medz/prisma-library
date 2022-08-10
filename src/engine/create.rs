use super::{
    core::{Engine, EngineBuilder, EngineDatamodel, Inner},
    instance, Result,
};
use crate::{c_char_to_string, error::ApiError};
use libc::c_char;
use std::sync::Arc;
use tokio::sync::RwLock;

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
}

/// Create engine result
#[repr(C)]
pub enum EngineCreateResult {
    /// Engine created successfully
    Success(i64),
    /// Engine creation failed
    Failure(ApiError),
}

/// Create a query engine.
#[no_mangle]
pub extern "C" fn create_engine(
    datamodel: *const c_char,
    datasource: *const c_char,
) -> EngineCreateResult {
    let datamodel = c_char_to_string(datamodel);
    let datasource_url = c_char_to_string(datasource);
    let engine = Engine::new(datamodel, datasource_url);

    match engine {
        Ok(engine) => EngineCreateResult::Success(instance::insert(Arc::new(engine))),
        Err(err) => EngineCreateResult::Failure(err),
    }
}
