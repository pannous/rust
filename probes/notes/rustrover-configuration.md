# RustRover Configuration for Probes Directory

## Problems Solved
1. RustRover was configured to load the parent Rust compiler workspace (`/opt/other/rust/Cargo.toml`) instead of the local probes workspace, causing it to attempt indexing all compiler internals including `rustc_macros`
2. The default test configuration used standard cargo, not the custom rustc toolchain needed for custom syntax

## Root Cause
The `.idea/workspace.xml` file contained:
```xml
<cargoProject FILE="$PROJECT_DIR$/../Cargo.toml">
```

This pointed to the parent workspace, loading 100+ packages including all compiler crates.

## Solution
Updated `.idea/workspace.xml` to point to the local Cargo.toml:
```xml
<cargoProject FILE="$PROJECT_DIR$/Cargo.toml" />
```

## How to Fix If It Happens Again

If RustRover auto-detects the parent workspace again after cache invalidation:

1. **Manual fix**: Close RustRover, then edit `.idea/workspace.xml`:
   ```bash
   cd /opt/other/rust/probes/.idea
   # Find the CargoProjects component and change FILE path to local Cargo.toml
   ```

2. **Proper project opening**: When opening the project in RustRover:
   - Open specifically `/opt/other/rust/probes` directory
   - NOT the parent `/opt/other/rust` directory
   - RustRover should then use the local Cargo.toml as configured in workspace.xml

3. **Verify correct workspace**:
   - In RustRover, open Cargo tool window
   - Should show only "probes" package, not compiler packages
   - If you see `rustc_macros` or other compiler crates, the wrong Cargo.toml is loaded

## Test Run Configurations

Added two new Shell Script run configurations that use the custom rustc:

### 1. Test All (Custom Rustc) - DEFAULT
- Runs: `./run_all_tests.sh --quick`
- Tests only known-working files
- Fast feedback for development
- Set as default run configuration

### 2. Test All Verbose
- Runs: `./run_all_tests.sh --quick --verbose`
- Shows detailed compilation and execution output
- Useful for debugging test failures

### How to Use
- Press Shift+F10 (Run) or click the green play button to run tests with custom rustc
- The configurations automatically use `/opt/other/rust/rustc` which wraps the stage1 toolchain
- Tests will fail gracefully if the custom toolchain is being rebuilt

## Toolchain Configuration

Uses standard cargo from `~/.cargo/bin` but configured to use custom rustc:
```xml
<option name="toolchainHomeDirectory" value="$USER_HOME$/.cargo/bin" />
```

**Cargo uses custom rustc via `.cargo/config.toml`:**
```toml
[build]
rustc = "/opt/other/rust/rustc"
```

This means:
- Standard `cargo` command is used (from stable toolchain)
- But it invokes the custom `rustc` wrapper at `/opt/other/rust/rustc`
- Which points to `/opt/other/rust/build/aarch64-apple-darwin/stage1/bin/rustc`
- Best of both worlds: cargo's features + custom rustc syntax

**RustRover's Cargo Test:**
- Will now use your custom rustc automatically
- Works for standard `.rs` tests in the probes package
- Custom syntax `.rust` files still need `run_all_tests.sh` (see below)

**To test custom syntax files:**
```bash
cd /opt/other/rust
./run_all_tests.sh --quick        # Fast, known-working only
./run_all_tests.sh --quick -v     # Verbose output
```

## Files Modified

### Committed to Git (shared with team)
- `.idea/runConfigurations/Test_All__Custom_Rustc_.xml` - Main test runner configuration
- `.idea/runConfigurations/Test_All_Verbose.xml` - Verbose test runner configuration
- `.gitignore` - Added exception to allow tracking run configurations

### Local Only (not committed)
- `.idea/workspace.xml` - Updated:
  - CargoProjects component (points to local Cargo.toml)
  - RustProjectSettings (custom toolchain path)
- This file is ignored by `.idea/.gitignore` and won't be committed

## Related Fixes
Also implemented in the same fix:
- Made probes an independent workspace in `Cargo.toml`
- Configured `rust-analyzer.toml` to disable auto-checking
- Removed probes from parent workspace members list
