//! Example: Load and call functions from a dynexport library.
//!
//! Build the test library first:
//! ```sh
//! rustc --edition 2021 --crate-type cdylib -o /tmp/libdyntest.dylib probes/test_dynexport_lib.rs
//! ```
//!
//! Then run this example:
//! ```sh
//! cargo run --example load_test
//! ```

use dynload::{DynLibrary, TypeHash};

fn main() -> dynload::Result<()> {
    // Path to the test library
    let lib_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/tmp/libdyntest8.dylib".to_string());

    println!("Loading library: {}", lib_path);

    // Open the library
    let lib = unsafe { DynLibrary::open(&lib_path)? };

    // Check metadata for exported functions
    println!("\n=== Metadata ===");
    for name in ["add", "multiply", "VERSION"] {
        match lib.get_metadata(name) {
            Ok(meta) => {
                println!("{}:", name);
                println!("  type_hash:     0x{:016x}", meta.type_hash);
                println!("  compiler_hash: 0x{:08x}", meta.compiler_hash);
                println!("  flags:         0x{:08x}", meta.flags);
            }
            Err(e) => println!("{}: {}", name, e),
        }
    }

    // Load functions with metadata verification
    println!("\n=== Function calls ===");

    // Get the actual type hash from the library
    let add_meta = lib.get_metadata("add")?;
    let expected_hash = TypeHash::from_raw(add_meta.type_hash);

    // Load with verification
    let add: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe {
        lib.get_verified("add", expected_hash)?
    };

    let multiply: libloading::Symbol<extern "C" fn(i32, i32) -> i32> = unsafe {
        lib.get_with_metadata("multiply")?
    };

    println!("add(10, 20) = {}", add(10, 20));
    println!("multiply(6, 7) = {}", multiply(6, 7));

    // Demonstrate type mismatch detection
    println!("\n=== Type mismatch test ===");
    let wrong_hash = TypeHash::from_raw(0xDEADBEEF);
    match unsafe { lib.get_verified::<extern "C" fn(i32, i32) -> i32>("add", wrong_hash) } {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("Expected error: {}", e),
    }

    // Check compiler consistency
    println!("\n=== Compiler consistency ===");
    if lib.same_compiler("add", "multiply")? {
        println!("✓ add and multiply were compiled with the same compiler");
    }

    println!("\nDone!");
    Ok(())
}
