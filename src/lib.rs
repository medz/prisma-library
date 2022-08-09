pub mod dmmf;
pub mod version;


/// c_char to String
pub fn c_char_to_string(c_char: *const libc::c_char) -> String {
    let c_char = unsafe { std::ffi::CStr::from_ptr(c_char) };
    let c_char = c_char.to_str().unwrap();
    c_char.to_string()
}

/// String to c_char
pub fn string_to_c_char(string: &str) -> *const libc::c_char {
    let string = std::ffi::CString::new(string).unwrap();
    string.into_raw()
}

