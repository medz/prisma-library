use futures::executor as future_executor;

use crate::{error::ApiError, string_to_c_char};

use super::instance::INSTANCES;

/// Engine connect.
#[no_mangle]
pub extern "C" fn engine_connect(
    id: i64,
    error: extern "C" fn(error: ApiError),
    done: extern "C" fn(),
) {
    let key = id.unsigned_abs();
    let lock = unsafe { INSTANCES.write().unwrap() };
    let engine = lock.get(key);

    if let Some(engine) = engine {
        let result = future_executor::block_on(engine.connect());
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
