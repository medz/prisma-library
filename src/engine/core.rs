use std::sync::Arc;

use datamodel::dml::Datamodel;
use query_core::{QueryExecutor, schema::QuerySchema, MetricRegistry};

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
  pub datasource: String,
  pub metrics: Option<MetricRegistry>,
}

/// Everything needed to connect to the database and have the core running.
pub struct EngineBuilder {
  pub datamodel: EngineDatamodel,
  pub datasource: String,
}

/// The state of the engine.
pub enum Inner {
  /// Not connected, holding all data to form a connection.
  Builder(EngineBuilder),
  /// A connected engine, holding all data to disconnect and form a new
  /// connection. Allows querying when on this state.
  Connected(ConnectedEngine),
}
