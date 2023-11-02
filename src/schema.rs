use std::{ffi::{CStr, CString}, ptr};

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

/// Schame linting.
#[no_mangle]
pub extern "C" fn prisma_schema_lint(schema: *const libc::c_char) -> *const libc::c_char {
    let schema = unsafe { CStr::from_ptr(schema) };
    let schema = schema.to_str().unwrap();

    let result = prisma_fmt::lint(schema.to_string());
    let result = CString::new(result).unwrap();

    result.into_raw()
}

/// Returns the schema configuration.
#[no_mangle]
pub extern "C" fn prisma_schema_get_config(params: *const libc::c_char) -> *const libc::c_char {
    let params = unsafe { CStr::from_ptr(params) };
    let params = params.to_str().unwrap();

    let result = prisma_fmt::get_config(params.to_string());
    let result = match result {
        Ok(data) => data,
        Err(err) => serde_json::json!({ "error": err }).to_string(),
    };
    let result = CString::new(result).unwrap();

    result.into_raw()
}

/// Returns the schema DMMF.
#[no_mangle]
pub extern "C" fn prisma_schema_get_dmmf(params: *const libc::c_char) -> *const libc::c_char {
    let params = unsafe { CStr::from_ptr(params) };
    let params = params.to_str().unwrap();

    let result = prisma_fmt::get_dmmf(params.to_string());
    let result = match result {
        Ok(data) => data,
        Err(err) => serde_json::json!({ "error": err }).to_string(),
    };
    let result = CString::new(result).unwrap();

    result.into_raw()
}

/// Validates a schema.
/// 
/// If the schema is valid, returns a null pointer.
#[no_mangle]
pub extern "C" fn prisma_schema_validate(schema: *const libc::c_char) -> *const libc::c_char {
    let schema = unsafe { CStr::from_ptr(schema) };
    let schema = schema.to_str().unwrap();

    let result = prisma_fmt::validate(schema.to_string());
    
    match result {
        Ok(_) => ptr::null(),
        Err(err) => {
            let result = serde_json::json!({ "error": err }).to_string();
            let result = CString::new(result).unwrap();

            result.into_raw()
        },
    }
}
