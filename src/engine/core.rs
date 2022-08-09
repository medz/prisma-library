use std::sync::Arc;

use datamodel::{dml::Datamodel, ValidatedConfiguration};
use query_core::{QueryExecutor, schema::QuerySchema};

type Executor = Box<dyn QueryExecutor + Send + Sync>;

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

impl Inner {
  // Returns a builder if the engine is not connected
  pub fn as_builder(&self) -> Result<&EngineBuilder, &str> {
    match self {
      Inner::Builder(ref builder) => Ok(builder),
      Inner::Connected(_) => Err("Engine is already connected"),
    }
  }
}
