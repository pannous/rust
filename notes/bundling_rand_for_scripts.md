# Bundling rand for Script Mode

## The Challenge

Extensions use `rand::RngExt`, which requires the `rand` crate to be available. We automatically inject `extern crate rand;`, but the crate still needs to be findable by the linker.

## Why Not Add to Sysroot?

Adding external crates to `std` is problematic because:
1. Build ordering issues - `rand_core` needs `core`, but `core` isn't built yet
2. Increases stdlib compile time for everyone
3. Couples the compiler to specific external crate versions

## Recommended Solutions

### Option 1: Compile rand Once, Use Everywhere (Simplest)

**Step 1: Build rand as rlib**

```bash
# Create a minimal Cargo project
mkdir -p /tmp/rand-build
cd /tmp/rand-build

cat > Cargo.toml << 'EOF'
[package]
name = "rand-build"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10.0-rc.8"
EOF

# Build it
cargo build --release

# Copy the .rlib to your compiler's lib directory
cp target/release/deps/librand-*.rlib /opt/other/rust/build/host/stage1/lib/
cp target/release/deps/librand_core-*.rlib /opt/other/rust/build/host/stage1/lib/
```

**Step 2: Set RUSTFLAGS**

Add to `~/.cargo/config.toml`:

```toml
[build]
rustflags = [
    "-L", "/opt/other/rust/build/host/stage1/lib",
    "--extern", "rand=/opt/other/rust/build/host/stage1/lib/librand-<hash>.rlib"
]
```

Or export as environment variable:

```bash
export RUSTFLAGS="-L /path/to/libs --extern rand=/path/to/librand.rlib"
```

**Step 3: Use It**

```rust
#!/usr/bin/env rust

fn main() {
    put!("Random: {}", random());  // Just works!
}
```

### Option 2: Bundle rand in Compiler Distribution

For a distributable compiler, include precompiled `rand.rlib` in the package:

**Directory structure:**
```
your-rust/
├── bin/
│   └── rustc
├── lib/
│   ├── rustlib/
│   │   └── aarch64-apple-darwin/lib/
│   │       ├── libstd-*.rlib
│   │       ├── libcore-*.rlib
│   │       └── librand-*.rlib  ← Bundle here
```

**Configure rustc wrapper:**

Create a `rustc` wrapper script:

```bash
#!/bin/bash
RUST_HOME="$(dirname "$0")/.."
LIB_DIR="$RUST_HOME/lib/rustlib/$(rustc --print target-triple)/lib"

# Auto-add rand if it exists
if [ -f "$LIB_DIR"/librand-*.rlib ]; then
    RAND_LIB=$(ls "$LIB_DIR"/librand-*.rlib | head -1)
    exec "$RUST_HOME/bin/rustc.real" \
        -L "$LIB_DIR" \
        --extern "rand=$RAND_LIB" \
        "$@"
else
    exec "$RUST_HOME/bin/rustc.real" "$@"
fi
```

### Option 3: Auto-Compile rand On Demand

Modify the build system to compile rand during compiler build:

**Add to `rebuild.sh`:**

```bash
# After building compiler, compile rand
echo "Building rand for script mode..."
mkdir -p build/script-deps
cd build/script-deps

cat > Cargo.toml << 'EOF'
[package]
name = "script-deps"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10.0-rc.8"
EOF

cargo build --release --target $(rustc --print target-triple)

# Copy to compiler lib directory
cp target/*/release/deps/librand-*.rlib ../host/stage1/lib/rustlib/*/lib/
cp target/*/release/deps/librand_core-*.rlib ../host/stage1/lib/rustlib/*/lib/

cd ../..
echo "rand available for scripts!"
```

### Option 4: Modify rustc to Auto-Provide rand (Most Integrated)

Modify the compiler driver to automatically add `--extern rand` in script mode:

**File:** `compiler/rustc_driver_impl/src/lib.rs`

Find where script mode is detected and add:

```rust
if sess.is_script_mode() {
    // Auto-provide rand for extension code
    let sysroot = sess.sysroot();
    let rand_path = sysroot.join("lib/rustlib")
        .join(&sess.opts.target_triple)
        .join("lib/librand.rlib");

    if rand_path.exists() {
        sess.opts.externs.insert(
            "rand".to_string(),
            ExternEntry::new(ExternLocation::ExactPaths(vec![rand_path]))
        );
    }
}
```

## Current Recommendation

**For Development:** Use Option 1 with RUSTFLAGS

**For Distribution:** Use Option 2 (bundle with compiler) or Option 4 (modify driver)

## Quick Start (Option 1)

```bash
# 1. Build rand
cd /tmp && cargo new --lib rand-provider
cd rand-provider
echo 'rand = "0.10.0-rc.8"' >> Cargo.toml
cargo build --release

# 2. Copy to somewhere permanent
mkdir -p ~/.rust-script-libs
cp target/release/deps/librand-*.rlib ~/.rust-script-libs/
cp target/release/deps/librand_core-*.rlib ~/.rust-script-libs/

# 3. Configure rustc
export RUSTFLAGS="-L $HOME/.rust-script-libs"

# 4. Use in scripts
cat > test.rust << 'EOF'
#!/usr/bin/env rust
put!("Random: {}", random());
EOF

chmod +x test.rust
./test.rust
```

Output:
```
Random: 0.7284...
```

Perfect! ✨
