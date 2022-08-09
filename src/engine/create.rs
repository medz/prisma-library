use libc::c_char;
use tokio::sync::RwLock;

use crate::{c_char_to_string, string_to_c_char};

use super::{core::{EngineDatamodel, EngineBuilder, Inner}, instance::INSTANCES};

/// Create engine result
#[repr(C)]
pub enum EngineCreateResult {
    /// Engine created successfully
    Success(i64),
    /// Engine creation failed
    Failure(*const c_char),
}

/// Create a query engine.
#[no_mangle]
pub extern "C" fn engine_create(
  datamodel: *const c_char,
  datasource: *const c_char,
) -> EngineCreateResult {
  let datamodel = c_char_to_string(datamodel);
  let ast = datamodel::parse_datamodel(&datamodel);
  if !ast.is_ok() {
    let err = ast.unwrap_err();
    let err = err.errors();
    let err = err.first();
    let err = err.unwrap();
    let err = err.message();

    return EngineCreateResult::Failure(string_to_c_char(err));
  }

  let ast = ast.unwrap().subject;
  let datamodel = EngineDatamodel { ast, raw: datamodel };

  let builder = EngineBuilder {
    datamodel,
    datasource: c_char_to_string(datasource),
  };
  let builder = Inner::Builder(builder);
  let builder = RwLock::new(builder);

  let id = unsafe {
    let id = INSTANCES.len() as i64;
    INSTANCES.push(builder);

    id
  };

  EngineCreateResult::Success(id)
}