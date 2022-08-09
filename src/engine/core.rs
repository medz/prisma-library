use std::sync::Arc;

use datamodel::dml::Datamodel;
use query_core::{QueryExecutor, schema::QuerySchema, MetricRegistry};

type Executor = Box<dyn QueryExecutor + Send + Sync>;

/// Holding the information to reconnect the engine if needed.
#[derive(Debug, Clone)]
pub struct EngineDatamodel {
    ast: Datamodel,
    raw: String,
}

/// Internal structure for querying and reconnecting with the engine.
pub struct ConnectedEngine {
  datamodel: EngineDatamodel,
  query_schema: Arc<QuerySchema>,
  executor: Executor,
  datasource: String,
  metrics: Option<MetricRegistry>,
}

/// Everything needed to connect to the database and have the core running.
pub struct EngineBuilder {
  datamodel: EngineDatamodel,
  datasource: String,
}

/// The state of the engine.
pub enum Inner {
  /// Not connected, holding all data to form a connection.
  Builder(EngineBuilder),
  /// A connected engine, holding all data to disconnect and form a new
  /// connection. Allows querying when on this state.
  Connected(ConnectedEngine),
}
