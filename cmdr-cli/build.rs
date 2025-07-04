use std::env;
use std::process::Command;

fn main() {
    // Get the current Git commit hash
    let git_hash = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Check if the Git tree is dirty
    let git_dirty = Command::new("git")
        .args(&["status", "--porcelain"])
        .output()
        .ok()
        .map(|output| !output.stdout.is_empty())
        .unwrap_or(false);

    // Set the version with Git information if dirty
    let version = if git_dirty {
        format!("{}-{}-dirty", env::var("CARGO_PKG_VERSION").unwrap(), git_hash)
    } else {
        format!("{}-{}", env::var("CARGO_PKG_VERSION").unwrap(), git_hash)
    };

    println!("cargo:rustc-env=CMDR_VERSION={}", version);
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");
} 