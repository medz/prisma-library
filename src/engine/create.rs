use std::sync::Arc;

use libc::c_char;

use crate::{c_char_to_string, error::ApiError};

use super::{core::Engine, instance};

/// Create engine result
#[repr(C)]
pub enum EngineCreateResult {
    /// Engine created successfully
    Success(i64),
    /// Engine creation failed
    Failure(ApiError),
}

/// Create a query engine.
#[no_mangle]
pub extern "C" fn engine_create(
    datamodel: *const c_char,
    datasource: *const c_char,
) -> EngineCreateResult {
    let datamodel = c_char_to_string(datamodel);
    let datasource_url = c_char_to_string(datasource);
    let engine = Engine::new(datamodel, datasource_url);

    match engine {
        Ok(engine) => EngineCreateResult::Success(instance::insert(Arc::new(engine))),
        Err(err) => EngineCreateResult::Failure(err),
    }
}
