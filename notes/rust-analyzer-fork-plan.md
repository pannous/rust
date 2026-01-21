# rust-analyzer Fork Modification Plan

This document describes the exact modification points to fork rust-analyzer to support custom Rust syntax.

## Repository Location
`/opt/other/rust-analyzer`

## Overview of Changes Needed

| Feature | Complexity | Files to Modify |
|---------|------------|-----------------|
| `and`/`or` operators | Low | 1 file |
| `not` operator | Low | 1 file |
| `xor` operator | Low | 1 file |
| Unicode operators | Medium | 2 files |
| Power operator `**` | Medium | 2-3 files |
| Semicolon inference | High | 4 files |

---

## 1. Operator Aliases: `and`, `or`, `xor`

### File: `crates/parser/src/grammar/expressions.rs`

**Function: `current_op()` (lines 203-243)**

This is the Pratt parser's operator binding power table. Add cases for identifier-based operators.

```rust
// Current code (line 228):
T![&] if p.at(T![&&])  => (4,  T![&&],  Left),

// Add after the existing cases (before the `_` catchall):
// Check for contextual keyword operators
_ if p.at(SyntaxKind::IDENT) => {
    match p.inp.text_at(p.pos) {  // Need to add text access method
        "and" => (4, T![&&], Left),   // Same precedence as &&
        "or"  => (3, T![||], Left),   // Same precedence as ||
        "xor" => (7, T![^], Left),    // Same precedence as ^
        _ => NOT_AN_OP,
    }
}
```

**Challenge**: The parser doesn't have direct text access. Two options:

**Option A (Simpler)**: Use contextual keyword detection
```rust
// In current_op(), add before the `_` catchall:
_ if p.at_contextual_kw(T![and]) => (4, T![&&], Left),
_ if p.at_contextual_kw(T![or])  => (3, T![||], Left),
_ if p.at_contextual_kw(T![xor]) => (7, T![^], Left),
```

But this requires adding `and`, `or`, `xor` to the contextual keywords in:
- `crates/parser/src/syntax_kind/generated.rs` - Add `AND_KW`, `OR_KW`, `XOR_KW`
- Regenerate with `cargo xtask codegen grammar`

**Option B (Cleaner)**: Check IDENT + contextual kind
```rust
// Add to current_op():
SyntaxKind::IDENT => {
    match p.inp.contextual_kind(p.pos) {
        // Need to add contextual kinds for and/or/xor
        AND_KW => (4, T![&&], Left),
        OR_KW  => (3, T![||], Left),
        XOR_KW => (7, T![^], Left),
        _ => NOT_AN_OP,
    }
}
```

---

## 2. Prefix Operator: `not`

### File: `crates/parser/src/grammar/expressions.rs`

**Function: `lhs()` (lines 329-410)**

Current prefix operator handling (line 367):
```rust
T![*] | T![!] | T![-] => {
    m = p.start();
    p.bump_any();
    PREFIX_EXPR
}
```

Add a check for `not` as an identifier:
```rust
// Add new case before the existing `T![*] | T![!] | T![-]`:
_ if p.at(SyntaxKind::IDENT) && p.at_contextual_kw(NOT_KW) => {
    // Check that `not` is followed by an expression-starting token
    // and NOT followed by `.` (to allow `not.method()`)
    if p.nth(1) != T![.] && can_start_expr(p.nth(1)) {
        m = p.start();
        p.bump_remap(T![!]);  // Remap `not` to `!`
        PREFIX_EXPR
    } else {
        // Fall through to normal identifier handling
        // ...
    }
}
```

**Also update `LHS_FIRST` TokenSet** (line 326):
```rust
const LHS_FIRST: TokenSet =
    atom::ATOM_EXPR_FIRST.union(TokenSet::new(&[T![&], T![*], T![!], T![.], T![-], T![_], IDENT]));
//                                                                                       ^^^^^ Add IDENT
```

---

## 3. Unicode Operators: `≤`, `≥`, `≠`, `…`

### File: `crates/parser/src/lexed_str.rs`

The rust-analyzer lexer uses `rustc_lexer`, which doesn't recognize Unicode operators. They come through as `Unknown` tokens.

**Option A**: Modify `extend_token()` (line 198)

```rust
fn extend_token(&mut self, kind: &rustc_lexer::TokenKind, mut token_text: &str) {
    // Add at the start of the function:
    // Handle Unicode operators
    if *kind == rustc_lexer::TokenKind::Unknown && token_text.len() <= 4 {
        let syntax_kind = match token_text.chars().next() {
            Some('≤') => Some(T![<=]),
            Some('≥') => Some(T![>=]),
            Some('≠') => Some(T![!=]),
            Some('…') => Some(T![..]),
            Some('¬') => Some(T![!]),
            _ => None,
        };
        if let Some(kind) = syntax_kind {
            self.push(kind, token_text.len(), vec![]);
            return;
        }
    }
    // ... rest of existing code
}
```

### File: `crates/parser/src/shortcuts.rs`

**Function: `to_input()` (line 28)**

Ensure Unicode tokens are treated as joint when appropriate (they shouldn't have whitespace after).

---

## 4. Power Operator: `**`

### File: `crates/parser/src/grammar/expressions.rs`

**Function: `current_op()` (lines 203-243)**

```rust
// Add case for ** (two stars that are joint):
T![*] if p.at_composite2(0, T![*], T![*]) => (13, STAR2, Right),  // Higher than *, right-assoc
T![*] if p.at(T![*=])  => (1, T![*=], Right),
T![*]                  => (11, T![*], Left),
```

This requires:
1. Adding `STAR2` to SyntaxKind (or reuse existing token)
2. The `at_composite2` check ensures both stars are adjacent

**Alternative**: Keep using `STAR` but handle in `expr_bp()`:
```rust
// In expr_bp(), after bumping the operator:
let op_kind = if op == T![*] && p.at(T![*]) {
    p.bump(T![*]);  // Consume second star
    POWER_OP  // Custom marker
} else {
    op
};
```

### Files to update for STAR2/POWER:
- `crates/parser/src/syntax_kind/generated.rs` - Add variant
- `crates/syntax/rust.ungram` - Add to BinExpr operators

---

## 5. Semicolon Inference (Most Complex)

This requires tracking whether there was a newline between the previous and current token.

### File 1: `crates/parser/src/input.rs`

Add newline tracking similar to `joint`:

```rust
pub struct Input {
    kind: Vec<SyntaxKind>,
    joint: Vec<bits>,
    contextual_kind: Vec<SyntaxKind>,
    edition: Vec<Edition>,
    preceded_by_newline: Vec<bits>,  // NEW: bit vector for newline info
}

impl Input {
    // Add setter method:
    pub fn had_newline(&mut self) {
        let n = self.len() - 1;
        let (idx, b_idx) = self.bit_index(n);
        self.preceded_by_newline[idx] |= 1 << b_idx;
    }

    // Add getter method:
    pub(crate) fn is_preceded_by_newline(&self, n: usize) -> bool {
        let (idx, b_idx) = self.bit_index(n);
        self.preceded_by_newline.get(idx).map_or(false, |&v| v & (1 << b_idx) != 0)
    }
}
```

### File 2: `crates/parser/src/shortcuts.rs`

**Function: `to_input()` (line 28)**

Track newlines in trivia:

```rust
pub fn to_input(&self, edition: Edition) -> crate::Input {
    let mut res = crate::Input::with_capacity(self.len());
    let mut was_joint = false;
    let mut had_newline = false;  // NEW

    for i in 0..self.len() {
        let kind = self.kind(i);
        if kind.is_trivia() {
            was_joint = false;
            // NEW: Check if trivia contains newline
            if kind == SyntaxKind::WHITESPACE && self.text(i).contains('\n') {
                had_newline = true;
            }
        } else if kind == SyntaxKind::IDENT {
            // ... existing code ...
            // NEW: Set newline flag after push
            if had_newline {
                res.had_newline();
                had_newline = false;
            }
        } else {
            // ... existing code ...
            // NEW: Set newline flag after push
            if had_newline {
                res.had_newline();
                had_newline = false;
            }
        }
    }
    res
}
```

### File 3: `crates/parser/src/parser.rs`

Add method to check for preceding newline:

```rust
impl<'t> Parser<'t> {
    // NEW: Check if current token is preceded by newline
    pub(crate) fn preceded_by_newline(&self) -> bool {
        self.inp.is_preceded_by_newline(self.pos)
    }
}
```

### File 4: `crates/parser/src/grammar/expressions.rs`

**Function: `stmt()` (lines 46-113)**

Modify semicolon handling:

```rust
pub(super) fn stmt(p: &mut Parser<'_>, semicolon: Semicolon) {
    // ... existing code until line 80 ...

    if let Some((cm, blocklike)) = expr_stmt(p, Some(m))
        && !(p.at(T!['}']) || (semicolon != Semicolon::Required && p.at(EOF)))
    {
        let m = cm.precede(p);
        match semicolon {
            Semicolon::Required => {
                if blocklike.is_block() {
                    p.eat(T![;]);
                } else if can_infer_semi(p) {  // NEW: Check for semicolon inference
                    // Semicolon inferred from newline - don't require it
                } else {
                    p.expect(T![;]);
                }
            }
            // ... rest unchanged ...
        }
        m.complete(p, EXPR_STMT);
    }
}

// NEW: Helper function for semicolon inference
fn can_infer_semi(p: &Parser<'_>) -> bool {
    // Don't infer if next token is `}` (tail expression)
    if p.at(T!['}']) {
        return false;
    }
    // Infer semicolon if there was a newline before the current token
    p.preceded_by_newline()
}
```

---

## Build & Test

```bash
cd /opt/other/rust-analyzer

# Build
cargo build --release

# Run tests
cargo test

# Install (replaces system rust-analyzer)
cargo xtask install --server

# Or specify custom path
cargo xtask install --server --server-path ~/.local/bin/rust-analyzer-custom
```

## IDE Configuration

### VS Code
```json
{
    "rust-analyzer.server.path": "~/.local/bin/rust-analyzer-custom"
}
```

### RustRover
1. Install "rust-analyzer" plugin from Marketplace
2. Settings → Languages → Rust → rust-analyzer
3. Set custom server path

---

## Suggested Implementation Order

1. **`and`/`or`/`xor`** - Easiest, good warmup
2. **`not`** - Similar to above but prefix
3. **Unicode operators** - Straightforward lexer change
4. **Power operator** - Requires AST changes
5. **Semicolon inference** - Most complex, do last

## Notes

- The generated file `syntax_kind/generated.rs` is created by `cargo xtask codegen grammar`
- Test files live in `crates/parser/test_data/`
- The grammar definition is in `crates/syntax/rust.ungram`
