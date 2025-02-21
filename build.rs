use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dialect_matcher.rs");

    // Run Python script to generate Rust lexer definitions
    let output = Command::new("python3")
        .arg("utils/build_lexers.py") // Python script that converts lexers to Rust
        .output()
        .expect("Failed to execute Python script");

    if !output.status.success() {
        panic!("Python script execution failed.");
    }

    // Write the generated Rust code to a file
    fs::write(dest_path, output.stdout).expect("Failed to write dialect_matcher.rs");

    println!("cargo:rerun-if-changed=utils/build_lexers.py");
}
