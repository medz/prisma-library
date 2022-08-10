use super::{Executor, Result};
use crate::{error::ApiError, string_to_c_char};
use datamodel::{dml::Datamodel, ValidatedConfiguration};
use futures::{Future, FutureExt};
use query_core::{schema::QuerySchema, QueryExecutor};
use std::{panic::AssertUnwindSafe, sync::Arc};
use tokio::sync::RwLock;
use user_facing_errors::Error;

/// Holding the information to reconnect the engine if needed.
#[derive(Debug, Clone)]
pub struct EngineDatamodel {
    pub ast: Datamodel,
    pub raw: String,
}

/// Internal structure for querying and reconnecting with the engine.
pub struct ConnectedEngine {
    pub datamodel: EngineDatamodel,
    pub query_schema: Arc<QuerySchema>,
    pub executor: Executor,
}

impl ConnectedEngine {
    /// The schema AST for Query Engine core.
    pub fn query_schema(&self) -> &Arc<QuerySchema> {
        &self.query_schema
    }

    /// The query executor.
    pub fn executor(&self) -> &(dyn QueryExecutor + Send + Sync) {
        &*self.executor
    }
}

/// Everything needed to connect to the database and have the core running.
pub struct EngineBuilder {
    pub datamodel: EngineDatamodel,
    pub config: ValidatedConfiguration,
}

/// The state of the engine.
pub enum Inner {
    /// Not connected, holding all data to form a connection.
    Builder(EngineBuilder),
    /// A connected engine, holding all data to disconnect and form a new
    /// connection. Allows querying when on this state.
    Connected(ConnectedEngine),
}

pub struct Engine {
    pub inner: RwLock<Inner>,
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

pub async fn async_panic_to_error<F, R>(future: F) -> Result<R>
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
