use libc::c_char;
use query_core::TxId;

use crate::{
    c_char_to_string,
    engine::{
        core::{async_panic_to_error, Engine},
        instance, Result,
    },
    error::ApiError,
    string_to_c_char,
};

use super::map_known_error;

impl Engine {
    /// If connected, attempts to commit a transaction with id `tx_id` in the core.
    async fn commit_transaction(&self, tx_id: String) -> Result<String> {
        async_panic_to_error(async {
            let inner = self.inner.read().await;
            let engine = inner.as_engine()?;

            async move {
                match engine.executor().commit_tx(TxId::from(tx_id)).await {
                    Ok(_) => Ok("{}".to_string()),
                    Err(err) => Ok(map_known_error(err)?),
                }
            }
            .await
        })
        .await
    }
}

/// Commits a transaction with id `tx_id` in the core.
#[no_mangle]
pub extern "C" fn commit_transaction(
    id: i64,
    tx_id: *const c_char,
    error: extern "C" fn(ApiError),
    done: extern "C" fn(*const c_char),
) {
    let tx_id = c_char_to_string(tx_id);
    let lock = instance::lock();
    let engine = lock.get(id.unsigned_abs());

    if let Some(engine) = engine {
        let result = futures::executor::block_on(engine.commit_transaction(tx_id));
        if result.is_ok() {
            let value = result.unwrap();
            let value = string_to_c_char(&value);
            done(value);
        } else {
            let err = result.unwrap_err();
            error(err);
        }
    } else {
        let err = "Engine not found";
        let err = string_to_c_char(err);
        error(ApiError::Connector(err));
    }
}
