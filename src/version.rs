use std::ffi::CString;

/// Returns the version of the Prisma library.
#[no_mangle]
pub extern "C" fn prisma_version() -> *const libc::c_char {
    let version = env!("CARGO_PKG_VERSION");
    
    CString::new(version).unwrap().into_raw()
}
