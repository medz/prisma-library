/// Returns the semantic version of the library.
#[no_mangle]
pub extern "C" fn get_prisma_semantic_version() -> *const u8 {
    env!("CARGO_PKG_VERSION").as_ptr() 
}

/// Returns the git commit hash of the library.
#[no_mangle]
pub extern "C" fn get_prisma_git_commit_hash() -> *const u8 {
    env!("GIT_HASH").as_ptr()
}
