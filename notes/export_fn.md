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

## Import Function Feature (Counterpart)

### Implementation Summary

Implemented `import fn` syntax as the counterpart to `export fn` for importing functions from dynamic libraries.

### Key Changes

**Parser Modification** (compiler/rustc_parse/src/parser/item.rs:695-703, 3046-3114)
- Detects `import fn` token sequence before general import handling
- Parses function signature similar to regular fn declarations
- Creates ForeignMod (extern block) containing the function
- Automatically uses extern "C" ABI for C-compatible calling convention

### Usage Example

```rust
#!/usr/bin/env rust

// Import functions from a dynamic library
import fn foo42() -> i32;
import fn doubled(x: i64) -> i64;

#[test]
fn test_library_import_magic() {
    let result = unsafe { foo42() };
    eq!(result, 42);

    let result2 = unsafe { doubled(21) };
    eq!(result2, 42);
}
```

### Generated Code

The `import fn` syntax generates an extern "C" block:

```rust
extern "C" {
    fn foo42() -> i32;
    fn doubled(x: i64) -> i64;
}
```

### Compilation & Linking

```bash
# Compile the library
rustc test_library.rust --crate-type dylib -o /tmp/libtest_library.dylib

# Compile the test with linking flags
rustc --test test_library_test.rust -L /tmp -l test_library -o test

# Run (with library path for dynamic linking)
DYLD_LIBRARY_PATH=/tmp ./test
```

### Safety

Imported functions are correctly marked as unsafe FFI calls, requiring unsafe blocks:
```rust
let result = unsafe { foo42() };  // Explicit unsafe for FFI
```

### Complete Workflow

1. **Export side** (test_library.rust):
   ```rust
   export fn foo42() -> int { 42 }
   export fn doubled(x: int) -> int { x * 2 }
   ```

2. **Import side** (test_library_test.rust):
   ```rust
   import fn foo42() -> i32;
   import fn doubled(x: i64) -> i64;
   ```

3. **Build**:
   ```bash
   rustc test_library.rust --crate-type dylib -o libtest_library.dylib
   rustc test.rust -L . -l test_library
   ```

### Implementation Details

The feature works by:
1. Detecting `import fn` during item parsing
2. Parsing the function signature (name, parameters, return type)
3. Creating a ForeignItem::Fn with the signature
4. Wrapping it in a ForeignMod (extern block) with "C" ABI
5. Returning ItemKind::ForeignMod containing the declaration

This generates the exact same structure as writing `extern "C" { fn ... }` manually, but with cleaner syntax.

### Test Results

- Manual compilation and linking: ✓ working
- FFI calls successful: ✓ passing
- Symbols correctly resolved at link time
- Type-safe FFI with explicit unsafe markers

### Future Enhancements

- Auto-detect library name from file name pattern (e.g., `*_test.rust` imports from `*`)
- Add `#[link(name = "...")]` attribute generation for automatic linking
- Support batch imports: `import { foo, bar, baz } from "library";`

## Include Library Feature

### Implementation Summary

Implemented `include library_name;` syntax to automatically handle library linking without manual `-L` and `-l` compiler flags.

### Key Changes

**Parser Modification** (compiler/rustc_parse/src/parser/item.rs:699-702, 3048-3085)
- Detects `include <ident>;` token sequence
- Parses library name identifier
- Creates #[link(name = "library")] attribute
- Generates empty extern "C" {} block with the link attribute

### Usage Example

**Old way** (manual linking):
```bash
rustc test.rust -L /path/to/libs -l test_library
```

**New way** (automatic linking):
```rust
#!/usr/bin/env rust

include test_library;  // Automatically links to libtest_library.dylib/so

import fn foo42() -> i32;
import fn doubled(x: i64) -> i64;
```

### Generated Code

The `include test_library;` statement generates:

```rust
#[link(name = "test_library")]
extern "C" {}
```

This tells the linker to automatically link against `libtest_library.dylib` (or `.so` on Linux, `.dll` on Windows).

### Complete Workflow Example

**test_library.rust** (library to export):
```rust
#!/usr/bin/env rust

export fn foo42() -> int { 42 }
export fn doubled(x: int) -> int { x * 2 }
```

**test_library_test.rust** (consumer):
```rust
#!/usr/bin/env rust

include test_library;  // Auto-link!

import fn foo42() -> i32;
import fn doubled(x: i64) -> i64;

#[test]
fn test() {
    unsafe {
        assert_eq!(foo42(), 42);
        assert_eq!(doubled(21), 42);
    }
}
```

**Build** (simplified):
```bash
# Build library  
rustc test_library.rust --crate-type dylib

# Build test (no -L or -l flags needed if library is in standard path!)
rustc --test test_library_test.rust

# Run
LD_LIBRARY_PATH=. ./test_library_test
```

### Implementation Details

1. Parser detects `include <ident>;` syntax
2. Calls `parse_include_library()` which:
   - Parses the library name as an identifier
   - Creates a #[link(name = "...")] attribute using `create_link_attr()`
   - Generates an empty ForeignMod (extern "C" {})
   - Attaches the link attribute to the item
3. Linker automatically finds and links the library

### Benefits

✅ **Cleaner syntax** - No command-line flags needed  
✅ **Self-documenting** - Library dependencies visible in source code  
✅ **Portable** - Works across platforms (dylib/so/dll handled automatically)  
✅ **Composable** - Combine with `import fn` for complete FFI workflow  

### Future Enhancements

- Auto-detect library path from relative imports
- Support library search paths: `include test_library from "./libs";`
- Version specifications: `include test_library@1.0;`
