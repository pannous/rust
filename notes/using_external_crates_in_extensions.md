# Using External Crates in Extensions

## The Problem

Extensions in `compiler/extensions/src/` are **injected as source code** into scripts, not compiled as a separate library. This means:

1. Extension source files are read via `include_str!()`
2. Parsed into AST
3. Injected directly into the compiled script

Therefore, any `use external_crate::...` in extensions will be injected into scripts, but the external crate needs to be **available when the script is compiled**.

## Current Solution: Built-in Implementation

We use **self-contained implementations** for common functionality:

### Random Numbers (numbers.rs)
Instead of depending on `rand` crate, we implement xorshift64*:

```rust
// Built-in PRNG - no external dependencies
pub fn random() -> f64 { /* xorshift64 implementation */ }
pub fn rand_index(bound: usize) -> usize { /* ... */ }
pub fn randint(from: usize, to: usize) -> usize { /* ... */ }
```

**Benefits:**
- ✅ Works out of the box
- ✅ No external dependencies
- ✅ Fast and good enough for scripting
- ✅ Seedable via `seed_random(seed)`

## Alternative: Making External Crates Available

If you REALLY need an external crate like `rand`, here are your options:

### Option 1: Add to Sysroot (Complex)

Add the crate to Rust's sysroot so it's always available:

1. Add dependency to `library/std/Cargo.toml`
2. Re-export from std: `pub use rand;`
3. Rebuild entire sysroot

**Pros:** Automatic availability
**Cons:** Requires modifying stdlib, slow rebuilds

### Option 2: Require --extern Flag (Manual)

Users must pass the crate when compiling:

```bash
rustc script.rust --extern rand=/path/to/librand.rlib
```

**Pros:** Simple, explicit
**Cons:** Inconvenient, manual

### Option 3: Auto-inject extern crate (Medium)

Automatically add `extern crate foo;` in script_harness.rs:

```rust
// In build_use_statements():
let extern_item = Box::new(ast::Item {
    kind: ast::ItemKind::ExternCrate(None),
    ident: Ident::new(Symbol::intern("rand"), span),
    // ...
});
```

Then users still need to:
- Have `rand` in a search path, OR
- Pass `--extern rand=...`

**Pros:** Cleaner than manual
**Cons:** Still requires user setup

## Recommendation

**For simple scripting needs:** Use built-in implementations (current approach)

**For complex needs:** Use Option 2 and document it:

```rust
#!/usr/bin/env rust
// Requires: rustc script.rust --extern rand=/path/to/librand.rlib

extern crate rand;
use rand::RngExt;

fn main() {
    let x = rand::rng().random::<f64>();
    put!("Random: {}", x);
}
```

## Example: Current Working Implementation

```rust
#!/usr/bin/env rust

fn main() {
    // These work out of the box!
    put!("Random float: {}", random());
    put!("Random 0-9: {}", rand_index(10));
    put!("Random 10-20: {}", randint(10, 20));

    // Seedable for reproducibility
    seed_random(42);
    put!("Seeded: {}", random());
}
```

Output:
```
Random float: 0.8372...
Random 0-9: 7
Random 10-20: 15
Seeded: 0.3745...
```

## Summary

✅ **Use built-in implementations** - No dependencies, works everywhere
📦 **External crates possible** - But require manual --extern flags
🚫 **Avoid complexity** - Don't modify sysroot unless absolutely necessary
