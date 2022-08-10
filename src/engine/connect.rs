use super::{
    core::{async_panic_to_error, ConnectedEngine, Engine, Inner},
    instance, Result,
};
use crate::{error::ApiError, string_to_c_char};
use prisma_models::InternalDataModelBuilder;
use query_core::schema_builder;
use std::sync::Arc;

impl Engine {
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
}

/// Engine connect.
#[no_mangle]
pub extern "C" fn connect(id: i64, error: extern "C" fn(error: ApiError), done: extern "C" fn()) {
    let lock = instance::lock();
    let engine = lock.get(id.unsigned_abs());

    if let Some(engine) = engine {
        let result = futures::executor::block_on(engine.connect());
        if result.is_ok() {
            done();
        } else {
            error(result.err().unwrap());
        }
    } else {
        let err = "Engine not found";
        let err = string_to_c_char(err);
        error(ApiError::Connector(err));
    }
}
