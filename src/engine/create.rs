use libc::c_char;

use crate::c_char_to_string;

/// Create a query engine.
#[no_mangle]
pub extern "C" fn engine_create(
  datamodel: *const c_char,
  datasource: *const c_char,
) -> i64 {
  let datamodel = c_char_to_string(datamodel);
  let ast = datamodel::parse_datamodel(&datamodel);
  // TODO
}