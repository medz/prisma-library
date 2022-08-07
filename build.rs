use std::process::Command;

fn main() {
  store_git_hash();
}

/// Store the git hash in the environment variable `GIT_HASH`.
fn store_git_hash() {
  let hash = Command::new("git")
    .args(&["rev-parse", "HEAD"])
    .output()
    .expect("Failed to execute git rev-parse")
    .stdout;
  let hash = String::from_utf8_lossy(&hash).trim().to_string();
  println!("cargo:rustc-env=GIT_HASH={}", hash);
}