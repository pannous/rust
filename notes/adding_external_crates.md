# Adding External Crates to Script Mode

## Overview

The compiler now **automatically detects and provides ANY external crate** placed in the sysroot lib directory. You don't need to modify the compiler code to add support for new crates!

## How It Works

1. **Detection:** When in script mode (`-Z script`), the compiler scans `lib/rustlib/<target>/lib/` for all `.rlib` files
2. **Filtering:** Excludes stdlib crates (std, core, alloc, etc.)
3. **Auto-provision:** Automatically adds `--extern` entries for each external crate found
4. **Zero config:** Extensions can use these crates without any manual flags

## Adding a New External Crate

### Step 1: Add to Cargo.toml

Edit `./build/script-deps/Cargo.toml` and add your dependency:

```toml
[dependencies]
rand = "0.10.0-rc.8"
serde = { version = "1.0", features = ["derive"] }  # Add this!
regex = "1.10"                                       # Or this!
```

### Step 2: Update lib.rs

Edit `./build/script-deps/src/lib.rs` to re-export the crate:

```rust
// Add public re-exports for each dependency
pub use rand;
pub use serde;  // Add this!
pub use regex;  // Or this!
```

### Step 3: Rebuild

```bash
./rebuild.sh
```

That's it! The crate is now automatically available in script mode.

### Step 4: Use in Extensions

Now you can use the crate in your extension modules:

**Example: `compiler/extensions/src/serialization.rs`**
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub value: i32,
}

pub fn to_json<T: Serialize>(data: &T) -> String {
    serde_json::to_string(data).unwrap()
}
```

The compiler will:
1. Detect `use serde::...` in your extension
2. Auto-inject `extern crate serde;`
3. Auto-provide the serde.rlib at link time

## Example: Adding Serde Support

Let's add full serde support as an example:

```bash
# 1. Update Cargo.toml
cat >> ./build/script-deps/Cargo.toml << 'EOF'
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
EOF

# 2. Update lib.rs
cat >> ./build/script-deps/src/lib.rs << 'EOF'
pub use serde;
pub use serde_json;
EOF

# 3. Rebuild
./rebuild.sh
```

Now in your scripts:

```rust
#!/usr/bin/env rust

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
class Person {
    name: String,
    age: i32,
}

fn main() {
    let person = Person { name: "Alice", age: 30 };
    let json = serde_json::to_string(&person).unwrap();
    put!("JSON: {}", json);
}
```

Compile with:
```bash
rustc -Z script script.rust  # serde automatically available!
```

## Current External Crates

Check what's currently available:

```bash
ls build/host/stage1/lib/rustlib/$(rustc --version --verbose | grep host | cut -d' ' -f2)/lib/ | grep -v "std\|core\|alloc"
```

After a fresh rebuild with the default config, you should see:
- `librand-*.rlib`
- `librand_core-*.rlib`
- `libchacha20-*.rlib`
- `libgetrandom-*.rlib`
- And any others you've added!

## Removing a Crate

1. Remove from `Cargo.toml`
2. Remove from `lib.rs`
3. Delete `./build/script-deps/target/` to force rebuild
4. Run `./rebuild.sh`
5. The .rlib files will no longer be copied

## How Detection Works

The compiler extracts crate names from `.rlib` filenames:

- Pattern: `lib<crate_name>-<hash>.rlib`
- Example: `librand-76ac9a837e148bf3.rlib` → crate name is `rand`
- Automatically grouped by crate name (multiple hash variants)
- Sorted alphabetically for deterministic builds

## Stdlib Exclusion List

These crates are automatically excluded (they're already provided by rustc):

- std, core, alloc, proc_macro, test
- panic_abort, panic_unwind, unwind
- rustc_std_workspace_*
- std_detect, addr2line, cfg_if, compiler_builtins
- getopts, gimli, hashbrown, libc, memchr
- miniz_oxide, object, rustc_demangle, sysroot

## Benefits

✅ **Universal:** Works with ANY external crate on crates.io
✅ **No Code Changes:** Don't need to modify the compiler for each crate
✅ **Self-Documenting:** `ls lib/` shows what's available
✅ **Extensible:** Add unlimited crates via Cargo.toml
✅ **Zero Config:** Scripts just work, no flags needed

## Limitations

1. **Version Compatibility:** Crates must be built with the same rustc version
   - Fixed automatically by using `RUSTC=./build/host/stage1/bin/rustc cargo build`
   - rebuild.sh handles this

2. **Feature Selection:** Features must be chosen at build time in Cargo.toml
   - Can't select different features per script
   - Choose features needed for most common use cases

3. **Binary Size:** All dependencies are always linked
   - Even if a specific script doesn't use them
   - Acceptable trade-off for convenience

## Advanced: Custom Build Script

If you need more control, modify `rebuild.sh`:

```bash
# Build with specific target
RUSTC="$(pwd)/build/host/stage1/bin/rustc" \
cargo build --release \
    --target aarch64-apple-darwin \
    --manifest-path="$SCRIPT_DEPS_DIR/Cargo.toml"

# Copy only specific crates
cp "$SCRIPT_DEPS_DIR/target/release/deps/libserde-*.rlib" "$LIB_DIR/"
cp "$SCRIPT_DEPS_DIR/target/release/deps/libserde_json-*.rlib" "$LIB_DIR/"
```

## Troubleshooting

**Problem:** "can't find crate for `xyz`"

**Solution:**
1. Check if .rlib exists: `ls build/host/stage1/lib/rustlib/*/lib/ | grep xyz`
2. If missing, check Cargo.toml and lib.rs
3. Delete `build/script-deps/target/` and rebuild
4. Verify: `./build/host/stage1/bin/rustc --print sysroot`

**Problem:** "found crate compiled by an incompatible version"

**Solution:**
1. Delete `build/script-deps/target/`
2. Rebuild using: `RUSTC=./build/host/stage1/bin/rustc cargo build --release`
3. The rebuild.sh script handles this automatically

## Summary

Adding external crates is now trivial:
1. Add to `build/script-deps/Cargo.toml`
2. Re-export in `build/script-deps/src/lib.rs`
3. Run `./rebuild.sh`
4. Use anywhere in extensions or scripts!

No compiler code changes needed. The system is fully extensible and self-documenting.
