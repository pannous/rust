# Export Function Feature

## Implementation Summary

Implemented `export fn` syntax for automatic dynamic library exports in script mode.

### Key Changes

1. **Symbol Addition** (compiler/rustc_span/src/symbol.rs:1037)
   - Added `export` to the symbols list for proper keyword handling

2. **Parser Modification** (compiler/rustc_parse/src/parser/item.rs:510-555)
   - Detects `export fn` token sequence before visibility parsing
   - Automatically sets visibility to Public
   - Adds #[no_mangle] attribute for C-compatible symbol names
   - Sets extern "C" ABI for FFI compatibility

3. **Attribute Generator** (compiler/rustc_parse/src/transformer/mod.rs:27-49)
   - Created `create_no_mangle_attr()` helper function
   - Generates proper AST attribute structure for #[no_mangle]

### Usage Example

```rust
#!/usr/bin/env rust

export fn foo42() -> int {
    return 42;
}

export fn double(x: int) -> int {
    return x * 2;
}
```

### Compilation

```bash
rustc script.rust --crate-type dylib -o libscript.dylib
```

### Generated Symbols

The exported functions produce clean, unmangled symbols:
- `_foo42` instead of mangled Rust names
- `_double` with standard C calling convention
- Callable from C, Python (ctypes), and other FFI interfaces

### Verification

```bash
nm -g libscript.dylib | grep -E "(foo42|double)"
# Output shows clean symbols:
# 0000000000004d54 T _double
# 0000000000004da8 T _foo42
```

### Test Results

- Test: probes/test_library.rust ✓ passed
- Clean symbol generation verified with nm
- No regressions in existing test suite (111 passed, same 2 pre-existing failures)

### Implementation Details

The feature works by:
1. Intercepting `export` keyword during item parsing
2. Consuming the `export` token and proceeding to parse `fn`
3. Forcing public visibility on the parsed function
4. Post-processing the function AST to:
   - Inject #[no_mangle] attribute
   - Set extern "C" ABI via StrLit struct
5. Resulting in C-compatible, unmangled function exports

### Future Enhancements

Could be extended to:
- Detect any `export fn` and automatically build as dylib
- Add `export` support for constants/statics
- Integrate with script mode to auto-detect library vs binary

### C FFI Verification

Successfully tested calling exported Rust functions from C:

```c
#include <stdio.h>
#include <stdint.h>

extern int64_t foo42(void);
extern int64_t double_value(int64_t x) __asm__("_double");

int main() {
    printf("Calling foo42() from C: %lld\n", foo42());
    printf("Calling double(21) from C: %lld\n", double_value(21));
    return 0;
}
```

Compilation and output:
```bash
$ cc test_ffi.c test_library.dylib -o test_ffi && ./test_ffi
Calling foo42() from C: 42
Calling double(21) from C: 42
```

✓ Confirmed: export fn generates true C-compatible extern "C" functions
✓ FFI calls work correctly across Rust-to-C boundary
✓ Function arguments and return values properly marshalled
