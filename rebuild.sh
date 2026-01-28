#!/bin/bash
# echo "rust command Temporarily unavailable during build. "
# the Rust bootstrap system deletes the sysroot to prevent stale artifacts from causing subtle bugs. The comment at line 1960-1962 explains this.

# cp ./build/host/stage1/bin/rustc rustc # current dyld[123]: Library not loaded: doesn't help
# cp ./build/host/stage1/bin/rustc /usr/local/bin/rust
# ln -s /opt/other/rust/build/host/stage1/bin/rustc /usr/local/bin/rust
if [[ "$*" == *"cache"* ]]; then
    export CARGO_INCREMENTAL=""
    env -u RUSTC_WRAPPER ./x.py build --stage 1
else
	echo "use ./rebuild.sh cache   for sccache instead of INCREMENTAL build!"
    ./x.py build --stage 1 compiler
    ./x.py build --stage 1 library
fi

# Build rand for script mode extensions
echo "Building rand for script mode..."
SCRIPT_DEPS_DIR="./build/script-deps"
mkdir -p "$SCRIPT_DEPS_DIR"

# Only rebuild if Cargo.toml doesn't exist or rand version changed
if [ ! -f "$SCRIPT_DEPS_DIR/Cargo.toml" ]; then
    cat > "$SCRIPT_DEPS_DIR/Cargo.toml" << 'EOF'
[package]
name = "script-deps"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10.0-rc.8"
EOF

    mkdir -p "$SCRIPT_DEPS_DIR/src"

    cat > "$SCRIPT_DEPS_DIR/src/lib.rs" << 'EOF'
// Dummy library to build rand as dependency
pub use rand;
EOF
fi

# Build with the system cargo (or use new rustc if available)
(cd "$SCRIPT_DEPS_DIR" && cargo build --release 2>/dev/null) || true

# Find and copy rand rlibs to compiler lib directory
if [ -d "$SCRIPT_DEPS_DIR/target/release/deps" ]; then
    TARGET_TRIPLE=$(./build/host/stage1/bin/rustc --version --verbose | grep host | cut -d' ' -f2)
    LIB_DIR="./build/host/stage1/lib/rustlib/$TARGET_TRIPLE/lib"

    # Copy rand and its dependencies
    find "$SCRIPT_DEPS_DIR/target/release/deps" -name "librand-*.rlib" -exec cp {} "$LIB_DIR/" \; 2>/dev/null || true
    find "$SCRIPT_DEPS_DIR/target/release/deps" -name "librand_core-*.rlib" -exec cp {} "$LIB_DIR/" \; 2>/dev/null || true

    echo "✓ rand available for scripts in $LIB_DIR"
fi

# cp ./build/host/stage1/bin/rustc rustc # current
# cp ./build/host/stage1/bin/rustc /usr/local/bin/rust

# echo "test with ./rustc -Z script probes/test_main.rs -o test && test"
# echo "rust command available again after build via ./build/host/stage1/bin/rustc"
# ⏺ Just the compiler: 5 seconds!

# ⏺ Final summary - single line change rebuild times:
#   ┌─────────────┬───┬───────────┐
#   │             Command             │  Time  │               Notes                │
#   ├─────────────┼───┼───────────┤
#   │ ./x.py build --stage 2          │ ~5 min │ Full bootstrap, production quality │
#   ├─────────────┼───┼───────────┤
#   │ ./x.py build --stage 1          │ 1:05   │ Includes std + rustdoc rebuild     │
#   ├─────────────┼───┼───────────┤
#   │ ./x.py build --stage 1 compiler │ 5 sec  │ Compiler only, fastest iteration   │
#   └─────────────┴───┴───────────┘


# ⏺ Summary of single-line change rebuild times:
#   ┌───────────┬──────┐
#   │          Method          │ Time │
#   ├───────────┼──────┤
#   │ sccache (no incremental) │ 5:19 │
#   ├───────────┼──────┤
#   │ Incremental (no sccache) │ 4:44 │
#   └───────────┴──────┘
#   Only ~35 seconds faster with incremental. The bottleneck is bootstrapping - changing the compiler means:
#   1. Stage 1 compiler rebuild (fast, ~4-5s)
#   2. Stage 1 library full rebuild (~15-19s) - built by new compiler
#   3. Stage 2 compiler full rebuild (~3-4min) - built by new stage 1
#   4. Rustdoc rebuild (~35-40s)

#   Incremental helps within each crate, but stage 2 is always rebuilt fresh because it's compiled by the changed stage 1.

#   For faster iteration during compiler development:
#   - ./x.py build --stage 1 - Skip stage 2 entirely (~20s for small changes)
#   - ./x.py check - Type-check only, no codegen (fastest)
#   - Test with stage 1: ./build/host/stage1/bin/rustc
