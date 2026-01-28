# Automatic External Crate Dependencies

## Feature: Auto-Detect and Inject `extern crate` Declarations

When extension files (`compiler/extensions/src/*.rs`) use external crates, the compiler now **automatically detects** them and injects `extern crate` declarations into compiled scripts.

## How It Works

### 1. Extension Uses External Crate

```rust
// compiler/extensions/src/numbers.rs
use rand::RngExt;

pub fn random() -> f64 {
    rand::rng().random()
}
```

### 2. Compiler Auto-Detects Dependency

The parser (`compiler/rustc_parse/src/transformer/extensions.rs`) scans extension source files for `use` statements and extracts external crate names:

```rust
fn extract_external_crates(source: &str) -> Vec<&str> {
    // Finds: use rand::RngExt;
    // Extracts: "rand"
    // Skips: std, core, alloc, crate, self, super
}
```

### 3. Auto-Inject `extern crate`

Before injecting extension code, an `extern crate rand;` declaration is automatically added:

```rust
// Generated in every script that uses extensions:
extern crate rand;

// Then extension code follows...
pub fn random() -> f64 {
    rand::rng().random()
}
```

##Making the Crate Available

The `extern crate` declaration is only half the solution. You still need to make the crate **available for linking**.

### Option 1: Use `--extern` Flag (Manual)

```bash
rustc script.rust --extern rand=/path/to/librand.rlib
```

### Option 2: Add to Cargo Dependencies

If your script is part of a Cargo project:

```toml
[dependencies]
rand = "0.10.0-rc.8"
```

Then compile with cargo:
```bash
cargo run --example script
```

### Option 3: Set RUSTFLAGS (Per-User)

Add to `~/.cargo/config.toml`:

```toml
[build]
rustflags = ["--extern", "rand=/path/to/librand.rlib"]
```

### Option 4: Bundle with Sysroot (Recommended for Distribution)

For a custom Rust distribution, add `rand` to the sysroot:

1. Add to `library/std/Cargo.toml`:
   ```toml
   [dependencies]
   rand = { version = "0.10.0-rc.8", optional = true }
   ```

2. Re-export in `library/std/src/lib.rs`:
   ```rust
   #[cfg(feature = "rand")]
   pub use rand;
   ```

3. Build with feature:
   ```bash
   ./x build --stage 1 library/std --features rand
   ```

Then `rand` is always available to all scripts!

## Testing

### Test Script (test_random.rust)

```rust
#!/usr/bin/env rust

fn main() {
    // random() function from extensions uses rand::rng()
    put!("Random float: {}", random());
    put!("Random 0-9: {}", rand_index(10));
    put!("Random 10-20: {}", randint(10, 20));
}
```

### Compile with --extern

```bash
# Assuming you have rand compiled as rlib
rustc test_random.rust --extern rand=path/to/librand.rlib

# Run
./test_random
```

Output:
```
Random float: 0.7284...
Random 0-9: 4
Random 10-20: 17
```

## Benefits

✅ **Automatic** - No manual `extern crate` needed in scripts
✅ **Centralized** - Dependencies managed in extensions, not user code
✅ **Flexible** - Works with any external crate used by extensions
✅ **Clean** - User scripts remain simple and focused

## Implementation Details

**File:** `compiler/rustc_parse/src/transformer/extensions.rs`

**Functions:**
- `extract_external_crates()` - Scans extension sources for `use` statements
- `create_extern_crate_item()` - Generates AST for `extern crate` declarations
- `parse_extensions()` - Injects extern crate items before extension code

**Detection Logic:**
1. Parse each line looking for `use crate_name::...`
2. Extract first path segment (crate name)
3. Filter out stdlib crates (std, core, alloc)
4. Filter out keywords (crate, self, super)
5. Deduplicate and return list

**Injection Order:**
```rust
// Script compilation order:
1. extern crate rand;        // Auto-injected
2. use std::collections::HashMap;  // Built-in imports
3. type int = i64;           // Type aliases
4. [Extension code]          // Traits, functions, etc.
5. [User code]               // Script content
```

## Current Status

✅ **Implemented** - Auto-detection and injection working
⚠️ **Requires** - User must provide crate via --extern or sysroot
🔄 **Future** - Could bundle common crates in sysroot by default

## Example: Full Workflow

### 1. Extension Uses rand

```rust
// compiler/extensions/src/numbers.rs
use rand::RngExt;

pub fn random() -> f64 {
    rand::rng().random()
}
```

### 2. User Writes Script

```rust
#!/usr/bin/env rust
put!("Random: {}", random());
```

### 3. Compiler Generates

```rust
extern crate rand;  // Auto-injected!

// Extension code with rand::rng() calls
pub fn random() -> f64 {
    rand::rng().random()
}

// User code
put!("Random: {}", random());
```

### 4. Link with rand

```bash
rustc script.rust --extern rand=librand.rlib
```

Perfect! ✨
