# Rust-Analyzer Indexing Fix

## Problem
When rebuilding the custom Rust compiler, rust-analyzer would attempt to re-index the entire workspace, including compiler internals like `rustc_macros`. This would fail when the custom toolchain at `/opt/other/rust/build/host/stage1/bin/rustc` was temporarily unavailable during the rebuild process.

## Root Cause
The `/opt/other/rust/probes/` directory was part of the parent workspace defined in `/opt/other/rust/Cargo.toml`. When RustRover/rust-analyzer opened the probes directory, it discovered the parent workspace and attempted to index all members, including:
- `compiler/rustc`
- `compiler/rustc_macros`
- Other compiler internals that require bootstrap build

## Solution
Three changes were made to isolate the probes directory:

### 1. Made probes an independent workspace
Added `[workspace]` declaration to `/opt/other/rust/probes/Cargo.toml`:
```toml
[workspace]
# Declare this as an independent workspace to prevent IDEs from
# indexing the parent Rust compiler workspace during rebuilds
```

### 2. Removed probes from parent workspace
Removed `"probes"` from the members list in `/opt/other/rust/Cargo.toml` so the parent workspace no longer manages it.

### 3. Configured rust-analyzer to skip checking
Updated `/opt/other/rust/probes/rust-analyzer.toml`:
- Disabled automatic cargo check
- Disabled build scripts
- Disabled diagnostics for custom syntax
- Suppressed notifications

## Result
- Probes is now an independent workspace
- rust-analyzer won't try to index compiler internals
- No re-indexing failures during toolchain rebuilds
- The probes directory can still be used normally for testing custom Rust syntax

## Verification
```bash
# Check that probes is its own workspace
cd /opt/other/rust/probes && cargo metadata --no-deps | jq -r '.workspace_root'
# Should output: /opt/other/rust/probes

# Check that probes is not in parent workspace
cd /opt/other/rust && cargo metadata --no-deps | jq -r '.workspace_members[]' | grep probes
# Should output nothing
```
