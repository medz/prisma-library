use std::{ffi::CString, os::raw::c_char};

/// Prisma query engine C API version.
/// 
/// - hash: The git hash of the current build.
/// - semver: The crate package semver version.
pub struct Version {
    pub hash: &'static str,
    pub semver: &'static str,
}

impl Version {
    pub fn new() -> Self {
        Version {
            hash: env!("GIT_HASH"),
            semver: env!("CARGO_PKG_VERSION"),
        }
    }
}

/// Get the Prisma query engine C API version.
pub extern "C" fn version() -> *const c_char {
    let version = Version::new();
    let version = format!("{{\"hash\":\"{}\",\"semver\":\"{}\"}}", version.hash, version.semver);
    let version = CString::new(version).unwrap();

    version.into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        let version = Version::new();
        assert_eq!(version.hash, env!("GIT_HASH"));
        assert_eq!(version.semver, env!("CARGO_PKG_VERSION"));
    }
}
