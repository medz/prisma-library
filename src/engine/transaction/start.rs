use super::map_known_error;
use crate::{
    c_char_to_string,
    engine::{
        core::{async_panic_to_error, Engine},
        instance, Result,
    },
    error::ApiError,
    string_to_c_char,
};
use libc::c_char;
use request_handlers::TxInput;

impl Engine {
    /// If connected, attempts to start a transaction in the core and returns its ID.
    async fn start_transaction(&self, input: String) -> Result<String> {
        async_panic_to_error(async {
            let inner = self.inner.read().await;
            let engine = inner.as_engine()?;

            async move {
                let input: TxInput = serde_json::from_str(&input)?;

                match engine
                    .executor()
                    .start_tx(
                        engine.query_schema().clone(),
                        input.max_wait,
                        input.timeout,
                        input.isolation_level,
                    )
                    .await
                {
                    Ok(tx_id) => Ok(tx_id.to_string()),
                    Err(err) => Ok(map_known_error(err)?),
                }
            }
            .await
        })
        .await
    }
}

/// Starts a transaction in the core and returns its ID.
#[no_mangle]
pub extern "C" fn start_transaction(
    id: i64,
    input: *const c_char,
    error: extern "C" fn(ApiError),
    done: extern "C" fn(*const c_char),
) {
    let input = c_char_to_string(input);
    let lock = instance::lock();
    let engine = lock.get(id.unsigned_abs());

    if let Some(engine) = engine {
        let result = futures::executor::block_on(engine.start_transaction(input));
        if result.is_ok() {
            let value = result.unwrap();
            let value = string_to_c_char(&value);
            done(value);
        } else {
            error(result.err().unwrap());
        }
    } else {
        let err = "Engine not found";
        let err = string_to_c_char(err);
        error(ApiError::Connector(err));
    }
}
