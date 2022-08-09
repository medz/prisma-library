use std::sync::Arc;

use futures::executor as future_executor;
use libc::c_char;
use prisma_models::InternalDataModelBuilder;
use query_core::{executor, schema_builder};

use crate::string_to_c_char;

use super::{instance::INSTANCES, core::{Inner, ConnectedEngine}};

/// Engine connect.
#[no_mangle]
pub extern "C" fn engine_connect(
  id: i64,
  callback: extern "C" fn (error: *const c_char),
) {
  let inner = unsafe {
    INSTANCES.get_mut(id as usize)
  };
  if inner.is_none() {
    let err = "Invalid engine id";
    let err = string_to_c_char(err);
    callback(err);
    return;
  }
  let inner = inner.unwrap();

  future_executor::block_on(async {
    let mut inner = inner.write().await;
    match inner.as_builder() {
      Ok(builder) => {
          let datasource = builder.config.subject.datasources.first().unwrap();
          let url = datasource.url.value.as_ref().unwrap();

          let executor = executor::load(datasource, &[], &url).await;
          if executor.is_err() {
            let err = executor.err().unwrap();
            let err = err.to_string();
            let err = string_to_c_char(&err);
            callback(err);
            return;
          }
          
          let (dbname, executor) = executor.unwrap();

          let connector = executor.primary_connector();
          let result = connector.get_connection().await;
          if result.is_err() {
            let err = result.err().unwrap();
            let err = err.to_string();
            let err = string_to_c_char(&err);
            callback(err);
            return;
          }

          let internal_data_model = InternalDataModelBuilder::from(&builder.datamodel.ast).build(dbname);

          let query_schema = schema_builder::build(
            internal_data_model,
            true, // enable raw queries
            datasource.capabilities(),
            ([]).to_vec(),
            datasource.referential_integrity(),
          );

          let engine = ConnectedEngine {
            datamodel: builder.datamodel.clone(),
            query_schema: Arc::new(query_schema),
            executor,
          };

          *inner = Inner::Connected(engine);

          callback(std::ptr::null());
      },
      Err(err) => {
        let err = string_to_c_char(&err);
        callback(err);
      }
    };
  });
}