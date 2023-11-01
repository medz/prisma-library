use std::ffi::CString;

/// The version of the library.
#[repr(C)]
pub struct PrismaVersion {
    /// The library commit hash.
    commit: *const libc::c_char,

    /// The library semantic version.
    /// 
    /// Example: `0.1.0`
    semver: *const libc::c_char,
}

impl PrismaVersion {
    pub(crate) fn commit() -> &'static str {
        env!("GIT_HASH")
    }

    pub(crate) fn semver() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}

/// Returns the version of the library.
#[no_mangle]
extern "C" fn get_prisma_version() -> PrismaVersion {
    let commin = CString::new(PrismaVersion::commit()).unwrap();
    let semver = CString::new(PrismaVersion::semver()).unwrap();

    PrismaVersion {
        commit: commin.as_ptr(),
        semver: semver.as_ptr(),
    }
}
