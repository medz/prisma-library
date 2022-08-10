use crate::{error::ApiError, string_to_c_char};

use super::instance;

#[no_mangle]
pub extern "C" fn disconnect(
    id: i64,
    error: extern "C" fn(error: ApiError),
    done: extern "C" fn(),
) {
    let lock = instance::lock();
    let engine = lock.get(id.unsigned_abs());

    if let Some(engine) = engine {
        let result = futures::executor::block_on(engine.disconnect());
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
