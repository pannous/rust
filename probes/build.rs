// Build script to auto-generate [[test]] entries for .rust files
// This allows cargo test to discover and run .rust test files with custom syntax

use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=tests");

    // Find all .rust test files in tests/ directory
    let tests_dir = Path::new("tests");
    if !tests_dir.exists() {
        return;
    }

    if let Ok(entries) = fs::read_dir(tests_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rust") {
                // Tell cargo to watch this file
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}
