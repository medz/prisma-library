use super::{
    core::{async_panic_to_error, Engine},
    instance, tx_id_parse, Result,
};
use crate::{c_char_to_string, error::ApiError, string_to_c_char};
use libc::c_char;
use query_core::TxId;
use request_handlers::GraphQlHandler;

impl Engine {
    async fn query(&self, body: String, tx_id: Option<String>) -> Result<String> {
        async_panic_to_error(async {
            let inner = self.inner.read().await;
            let engine = inner.as_engine()?;

            let query = serde_json::from_str(&body).map_err(|err| ApiError::from(err))?;

            async move {
                let handler = GraphQlHandler::new(engine.executor(), engine.query_schema());
                let response = handler.handle(query, tx_id.map(TxId::from), None).await;

                Ok(serde_json::to_string(&response)?)
            }
            .await
        })
        .await
    }
}

/// query function extension for C
#[no_mangle]
pub extern "C" fn query(
    id: i64,
    body: *const c_char,
    tx_id: *const c_char,
    error: extern "C" fn(ApiError),
    done: extern "C" fn(*const c_char),
) {
    let body = c_char_to_string(body);
    let tx_id = tx_id_parse(tx_id);

    let lock = instance::lock();
    let engine = lock.get(id.unsigned_abs());

    if let Some(engine) = engine {
        let result = engine.query(body, tx_id);
        let result = futures::executor::block_on(result);
        if result.is_ok() {
            let result = result.unwrap();
            let result = string_to_c_char(&result);

            done(result);
        } else {
            error(result.err().unwrap());
        }
    } else {
        let err = "Engine not found";
        let err = string_to_c_char(err);
        error(ApiError::Connector(err));
    }
}
