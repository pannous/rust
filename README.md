Fork of official Rust

Rust is a conceptually beautiful language with sometimes horrible syntax. 
We take the power of Rust and just make it more beautiful by removing avoidable bloat wherever we see it. 

Philosophy:
• 𓃀 𓄤 𓏤 Beauty without compromising correctness

# Features 
## working features
• optional trailing semicolon;
• and, or, not, xor, ¬, ∧, ∨ synonyms for archaic symbols && || !
• ≤ ≥ ≠ and … range operators
• shebang support: #!/usr/bin/env rust
• run rust as scripts with implicit main
• ./probes/test_main.rs
• # comments
• dynamic linking with wit like objects via dlsym C-ABI (rust ABI dependent on build!)
• Optionals via '?' as in other sane languages
• Optional chaining via ?. and ??
• truthy and falsy values in conditions  if 1 { }
• truthy optionals   let z : i32? = None; if z { ... } else { put!("🗸") }
• Magic lists @["hello", 42, true] → auto-wrapped Vec<Val> with [Str("hello"), Int(42), Bool(true)]
• Simple lists @[1, 2, 3] → homogeneous Vec<i32>
• i++ and i--
• modulo strings adn printf format specifiers "%d" % i
• “strings”   no more .to_string()
• as type casting
• 100 convenience functions "hello".reverse() = "olleh" ...
• "year "+2026  string concatenation with + operator for various types
• := operator for let mut
• var keyword for let mut
• put!(...) macro for generous printing



## Future features
• functions return Results, yes, no need to write it
• dynamic linking Swift ABI ...

See [Goo](https://github.com/pannous/goo) the Go++ language extensions for a list of some planned features. 

🐓 roost ? .roo 🦘 ?
      <!-- 28 +            extensions="rx;roo;🦀;🐓;🦘"/>     🍠 roast rost 
🐀 󳥫 󳩉 󳩊   RAT  rodents 🐁 🐭 🖱
      -->