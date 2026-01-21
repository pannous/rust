# rust-analyzer Fork for Custom Rust Syntax

**Location:** `/opt/other/rust-analyzer`
**Base:** Cloned from `https://github.com/rust-lang/rust-analyzer` (shallow clone)
**Purpose:** IDE support for custom Rust fork at `/opt/other/rust`

---

## Project Goal

The custom Rust compiler at `/opt/other/rust` has syntax extensions that standard IDEs don't understand. This fork of rust-analyzer adds parser support for those extensions, enabling proper syntax highlighting, error checking, and code intelligence.

## Custom Rust Features Requiring IDE Support

| Feature | Compiler Status | rust-analyzer Status |
|---------|----------------|---------------------|
| `and`/`or`/`xor` operators | Implemented | **Done** |
| `not` prefix operator | Implemented | **Done** |
| Unicode operators (`≤`,`≥`,`≠`,`…`,`¬`) | Implemented | **Done** |
| Power operator `**` | Parser only (no codegen) | **Done** |
| Semicolon inference from newlines | Implemented | **Done** |

---

## Completed: `and`/`or`/`xor` Operators

**Commit:** `aaffb7f` on 2026-01-13

### What Was Changed

```
crates/parser/src/syntax_kind/generated.rs
├── Added AND_KW, OR_KW, XOR_KW, NOT_KW enum variants
├── Added text representations
├── Added to from_contextual_keyword()
├── Added to is_contextual_keyword()
├── Added to is_keyword()
└── Added T![and], T![or], T![xor], T![not] macros

crates/parser/src/grammar/expressions.rs
├── current_op(): Recognize IDENT + contextual keyword as operator
└── expr_bp(): Use bump_remap() for contextual keyword operators

crates/syntax/src/verify_custom_ops.rs (new)
└── 5 verification tests
```

### How It Works

1. Lexer produces `IDENT` token for "and", "or", "xor"
2. `from_contextual_keyword()` maps the text to `AND_KW`, `OR_KW`, `XOR_KW`
3. Parser's `current_op()` checks `p.at_contextual_kw(T![and])` etc.
4. Returns same precedence/associativity as `&&`, `||`, `^`
5. `bump_remap()` converts the IDENT to the operator token in the AST

### Test Results

```
All 300 parser tests pass
All 53 syntax tests pass
All 5 custom operator tests pass
```

---

## Completed: `not` Prefix Operator

**Commit:** `7dc5eb4` on 2026-01-14

### What Was Changed

```
crates/parser/src/grammar/expressions.rs
├── lhs(): Added context-aware 'not' prefix operator detection
└── Proper postfix handling when 'not' is used as identifier

crates/syntax/src/verify_custom_ops.rs
└── Added 5 more tests for 'not' operator
```

### How It Works

1. In `lhs()`, check if current token is IDENT with contextual keyword `not`
2. Look at the next token to determine if it's a prefix operator context:
   - **IS prefix**: followed by IDENT, literal, `!`, `-`, `*`, `&`, `(`, `true`, `false`
   - **NOT prefix**: followed by `.`, `{`, `[`, `:`, `,`, `;`, `)`, `}`, `]`
3. If prefix: `bump_remap(T![!])` to convert to negation operator
4. If not prefix: call `atom::atom_expr` + `postfix_expr` for normal identifier handling

### Context Awareness

```rust
not true        // → PREFIX_EXPR (negation)
not not false   // → nested PREFIX_EXPR
not a and b     // → (not a) and b
not.method()    // → identifier with method call
match not { }   // → identifier as match scrutinee
let x = not;    // → identifier assignment
```

### Test Results

```
All 300 parser tests pass
All 63 syntax tests pass (including 10 custom operator tests)
Self-hosting test passes (parses rust-analyzer source code)
```

---

## Completed: Unicode Operators

**Commit:** `2b3fd9b` on 2026-01-14

### What Was Changed

```
crates/parser/src/lexed_str.rs
└── extend_token(): Intercept Unknown tokens and map Unicode chars

crates/syntax/src/verify_custom_ops.rs
└── Added 6 tests for Unicode operators
```

### How It Works

1. `rustc_lexer` returns `Unknown` for Unicode operators
2. Check first character of unknown token
3. For compound operators (`≤`,`≥`,`≠`,`…`): emit two joint tokens
4. For single operators (`¬`): emit one token
5. Jointness handled automatically by `to_input()`

### Mappings

| Unicode | Name | Maps To |
|---------|------|---------|
| `≤` (U+2264) | Less-than or equal | `< =` → `<=` |
| `≥` (U+2265) | Greater-than or equal | `> =` → `>=` |
| `≠` (U+2260) | Not equal | `! =` → `!=` |
| `…` (U+2026) | Horizontal ellipsis | `. .` → `..` |
| `¬` (U+00AC) | Not sign | `!` |

### Test Results

```
All 300 parser tests pass
All 69 syntax tests pass (including 16 custom operator tests)
Self-hosting test passes
```

---

## Completed: Power Operator `**`

**Commit:** `4b606df` on 2026-01-14

### What Was Changed

```
crates/parser/src/syntax_kind/generated.rs
├── Add STAR2 SyntaxKind
├── Add text representation "**"
├── Add to is_punct()
└── Add T![**] macro

crates/parser/src/parser.rs
├── nth_at(): Add ** detection via at_composite2(*, *)
└── eat(): Add ** to 2-token consumption list

crates/parser/src/grammar/expressions.rs
└── current_op(): Add ** with precedence 13, right-associative

crates/syntax/src/verify_custom_ops.rs
└── Added 4 tests for power operator
```

### How It Works

1. `**` is two joint `*` tokens
2. `at_composite2` checks for adjacent `*` `*` with jointness
3. Precedence 13 (higher than `*` at 11, `as` at 12)
4. Right-associative: `2**3**4` = `2**(3**4)`

### Test Results

```
All 300 parser tests pass
All 73 syntax tests pass (including 20 custom operator tests)
Self-hosting test passes
```

---

## Completed: Semicolon Inference

**Commit:** `dbf04c5` on 2026-01-14

### What Was Changed

```
crates/parser/src/input.rs
├── Add newline_before bit vector to Input struct
├── Add had_newline() setter method
└── Add is_preceded_by_newline() getter method

crates/parser/src/shortcuts.rs
└── to_input(): Track newlines in whitespace trivia

crates/parser/src/parser.rs
└── Add preceded_by_newline() method

crates/parser/src/grammar/expressions.rs
├── stmt(): Allow missing semicolon when preceded by newline
└── let_stmt(): Allow missing semicolon when preceded by newline

crates/syntax/src/verify_custom_ops.rs
└── Added 5 tests for semicolon inference
```

### How It Works

1. During lexing, track if whitespace trivia contains `\n`
2. Mark next non-trivia token as "preceded by newline"
3. When semicolon expected but missing, check newline flag
4. If preceded by newline, accept without error

### Behavior

```rust
// Works - newline before next statement
let x = 1
let y = 2

// Error - no newline, semicolon required
let x = 1 let y = 2

// Mixed works fine
let x = 1;
let y = 2
let z = 3;
```

### Test Results

```
All 300 parser tests pass
All 78 syntax tests pass (including 25 custom operator tests)
Self-hosting test passes
```

---

## All Features Complete!

All custom Rust syntax features now have IDE support in the rust-analyzer fork.

---

## Important Notes

### Codegen Overwrites Changes

Running `cargo xtask codegen grammar` regenerates `syntax_kind/generated.rs` from the grammar definition, **wiping custom changes**.

**Solutions:**
1. Re-apply edits after codegen (current approach)
2. Modify codegen source in `xtask/` to include custom keywords (proper fix)
3. Add custom keywords to `crates/syntax/rust.ungram` (may not work for contextual keywords)

### Building & Testing

```bash
cd /opt/other/rust-analyzer

# Build parser only (fast)
cargo build -p parser

# Run parser tests
cargo test -p parser

# Build full rust-analyzer
cargo build --release

# Install custom rust-analyzer
cargo xtask install --server
# Or to custom path:
cargo xtask install --server --server-path ~/.local/bin/rust-analyzer-custom
```

### IDE Configuration

**VS Code:**
```json
{
    "rust-analyzer.server.path": "/path/to/custom/rust-analyzer"
}
```

**RustRover:**
1. Install rust-analyzer plugin from Marketplace
2. Settings → Languages → Rust → rust-analyzer
3. Set custom server path

---

## Architecture Quick Reference

```
rust-analyzer/
├── crates/
│   ├── parser/              # Hand-written recursive descent parser
│   │   ├── src/
│   │   │   ├── grammar/     # Parsing rules
│   │   │   │   └── expressions.rs  # ← Operator parsing here
│   │   │   ├── syntax_kind/
│   │   │   │   └── generated.rs    # ← Token/keyword definitions
│   │   │   ├── lexed_str.rs        # ← Lexer bridge
│   │   │   ├── input.rs            # ← Parser input (token stream)
│   │   │   └── parser.rs           # ← Parser state machine
│   │   └── test_data/       # Test fixtures
│   ├── syntax/              # Syntax tree, AST
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── rust.ungram      # Grammar definition (for codegen)
│   └── ...
└── xtask/                   # Build tooling, codegen
```

---

## Related Files

- Custom Rust compiler: `/opt/other/rust`
- Modification plan: `/opt/other/rust/notes/rust-analyzer-fork-plan.md`
- This status doc: `/opt/other/rust/notes/rust-analyzer-fork-status.md`
