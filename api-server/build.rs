/// Extract the git revision hash to use as the version number in builds
// https://stackoverflow.com/questions/43753491/include-git-commit-hash-as-string-into-rust-program
use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
