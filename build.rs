fn git_hash() -> String {
    use std::process::Command;
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");
    
    String::from_utf8(output.stdout).expect("Failed to parse git output")
}

fn capi_generator() {
    use std::env;
    use cbindgen::{Builder, Language, Config};

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = Config::default();

    Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .with_language(Language::C)
        .with_no_includes()
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/prisma.h");
}

fn main() {
    println!("cargo:rustc-env=GIT_HASH={}", git_hash());

    capi_generator();
}