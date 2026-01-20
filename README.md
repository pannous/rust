Fork of official Rust

Rust is a conceptually beautiful language with sometimes horrible syntax. 
We take the power of Rust and just make it more beautiful by removing avoidable bloat wherever we see it. 

Philosophy:
• 𓃀 𓄤 𓏤 Beauty without compromising correctness

# Features 
## working features

### Syntax Sugar
• optional trailing semicolon;
• # comments (Python/shell style)
• := operator for let mut
• var keyword for let mut
• def add(a: int, b: int) int { a + b }     # no -> needed
• fun keyword as fn synonym
• class keyword with auto-derives (Debug, Clone, Copy)
• comma optional between struct/class fields
• automatic derives for enums/structs in script mode
• Go-style return type annotation (-> optional)
• JS-style arrow functions

### Boolean Operators
• and, or, not, xor, ¬, ∧, ∨ synonyms for archaic symbols && || !
• truthy and falsy values in conditions  if 1 { }
• truthy optionals   let z : i32? = None; if z { ... } else { put!("🗸") }

### Comparison & Range Operators
• ≤ ≥ ≠ comparison operators
• ... and … inclusive range operators
• `in` operator with auto-borrow:  `2 in [1,2,3]`

### Math Features
• ** power operator with int and float support
• Approximate equality   .1 + .2 ≈ .3   (also ~ as synonym)
• Julia-style implicit multiplication: 2π → 2*π
• τ (tau) and π (pi) constants baked in
• int-float coercion and leading dot floats: .5 instead of 0.5

### Strings
• "strings" auto-convert to String (no more .to_string())
• "year "+2026  string concatenation with + for various types
• modulo strings and printf format specifiers "%d" % i
• curly quote strings "hello" work globally
• string case conversion: .upper() .lower() .capitalize()
• 100+ convenience functions: "hello".reverse() = "olleh"

### Optionals & Null Safety
• Optionals via '?' as in other sane languages: i32?
• Optional chaining via ?. and ??
• nil as alias for None

### Collections
• Magic lists @["hello", 42, true] → auto-wrapped Vec<Val>
• Simple lists @[1, 2, 3] → homogeneous Vec<i32>
• @{key: value} map literal syntax
• {key: value} untyped map literal syntax
• mapped() and filtered() methods for arrays/slices
• first_cloned() method for owned first element
• seq! macro and slice_eq() for array-vec comparison

### Type Aliases & Casting
• int = i32   float = f64   bool = boolean
• unicode and codepoint type aliases
• as type casting (including int to bool)

### Convenience
• i++ and i-- increment/decrement
• put!(...) macro for generous printing
• exit!() and exit() function
• eqs! macro for string comparison with enums
• shebang support: #!/usr/bin/env rust
• run rust as scripts with implicit main
• dynamic linking with wit-like objects via dlsym C-ABI


## Future features
• functions return Results, yes, no need to write it
• dynamic linking Swift ABI ...

See [Goo](https://github.com/pannous/goo) the Go++ language extensions for a list of some planned features. 

🐓 roost ? .roo 🦘 ?
      <!-- 28 +            extensions="rx;roo;🦀;🐓;🦘"/>     🍠 roast rost 
🐀 󳥫 󳩉 󳩊   RAT  rodents 🐁 🐭 🖱
      -->