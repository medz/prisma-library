use std::ffi::{CStr, CString};

/// Format a schema.
/// 
/// ## Arguments
/// 
/// - `schema` - The schema to format.
/// - `params` - The formatting parameters as a JSON string.
#[no_mangle]
pub extern "C" fn prisma_schema_format(
    schema: *const libc::c_char,
    params: *const libc::c_char,
) -> *const libc::c_char {
    let schema = unsafe { CStr::from_ptr(schema) };
    let schema = schema.to_str().unwrap();

    let params = unsafe { CStr::from_ptr(params) };
    let params = params.to_str().unwrap();

    let formated = prisma_fmt::format(schema, params);
    let formated = CString::new(formated).unwrap();

    formated.into_raw()
}
