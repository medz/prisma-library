use crate::error::ApiError;
use libc::c_char;
use query_core::QueryExecutor;

mod core;
mod instance;

pub mod connect;
pub mod create;
pub mod disconnect;
pub mod query;

pub(crate) type Result<T> = std::result::Result<T, ApiError>;
pub(crate) type Executor = Box<dyn QueryExecutor + Send + Sync>;

pub(crate) fn tx_id_parse(tx_id: *const c_char) -> Option<String> {
    let tx_id = unsafe { std::ffi::CStr::from_ptr(tx_id) };

    match tx_id.to_str() {
        Ok(tx_id) => {
            if tx_id.is_empty() {
                None
            } else {
                Some(tx_id.to_string())
            }
        }
        Err(_) => None,
    }
}
