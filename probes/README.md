# Probes - Custom Rust Syntax Tests

This directory contains tests for custom Rust syntax extensions.

## IDE Integration

This directory is configured as a Cargo workspace member so your IDE can:
- Recognize test file structure
- Provide basic navigation
- Show function/macro definitions in `lib.rs`

## Important Notes

⚠️ **Tests use custom syntax** (`:=`, `class`, `def`, etc.) that standard Rust doesn't support.

- **Run tests**: Use `../run_all_tests.sh` (not `cargo test`)
- **IDE errors**: The IDE will show syntax errors for custom syntax - this is expected
- **Test format**: Use `#[test]` attributes in `.rs` files

## Running Tests

```bash
# Run all tests
../run_all_tests.sh

# Run quick mode (known-working tests only)
../run_all_tests.sh --quick

# Run specific test pattern
../run_all_tests.sh test_assign

# Verbose output
../run_all_tests.sh --verbose
```

## Adding New Tests

1. Create `test_feature.rs` in this directory
2. Use `#[test]` attribute for test functions
3. Import macros from lib: `use probes::*;` (if needed)
4. Run with `../run_all_tests.sh test_feature`

Example:
```rust
#[test]
fn test_custom_syntax() {
    x := 42
    eq!(x, 42)
    put!("test passed")
}
```
