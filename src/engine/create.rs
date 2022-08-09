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

  let datasource_url = c_char_to_string(datasource);
  let config = datamodel::parse_configuration(&datamodel).and_then(| mut config | {
    for datasource in &mut config.subject.datasources {
      datasource.url.value = Some(datasource_url.clone());
      datasource.url.from_env_var = None;
    }

    Ok(config)
  });
  if config.is_err() {
    let err = config.unwrap_err();
    let err = err.errors().first().unwrap();
    let err = string_to_c_char(&err.message());
    return EngineCreateResult::Failure(err);
  }

  let ast = ast.unwrap().subject;
  let datamodel = EngineDatamodel { ast, raw: datamodel };

  let builder = EngineBuilder {
    datamodel,
    config: config.unwrap(),
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