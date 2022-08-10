use libc::c_char;
use std::ffi::CString;

/// The Prisma query engine dynamid liobrary version info.
#[repr(C)]
pub struct Version {
    /// The commit hash of https://github.com/odroe/prisma repository.
    pub commit: *const c_char,

    /// The version of the library.
    pub version: *const c_char,
}

/// Get current version of the library.
#[no_mangle]
pub extern "C" fn version() -> Version {
    // Repository commit hash.
    let commit = env!("GIT_HASH");
    let commit = CString::new(commit).unwrap();

    // Library version.
    let version = env!("CARGO_PKG_VERSION");
    let version = CString::new(version).unwrap();

    Version {
        commit: commit.as_ptr(),
        version: version.as_ptr(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_version() {
        let info = version();

        let commit = env!("GIT_HASH");
        let commit = CString::new(commit).unwrap();
        let commit_str = unsafe { CStr::from_ptr(info.commit).to_str().unwrap() };
        assert_eq!(commit_str, commit.to_str().unwrap());
    }
}
