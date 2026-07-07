Fork of official Rust  

Rust is a conceptually beautiful language with sometimes horrible syntax.   
We take the power of Rust and just make it more beautiful by removing avoidable bloat wherever we see it.   

Philosophy:  
• 𓃀 𓄤 𓏤 Beauty without compromising correctness  

# Features   
## working features  

### Convenience
• **shebang** support: #!/usr/bin/env rust  
• run rust as **scripts** with implicit main  
• exit!() and **exit**() function  
• put!(...) macro for generous printing  
• Some(3) == 3  auto **unwrap**  
• unused_mut warnings suppressed in script mode   


### Syntax Sugar
• optional trailing semicolon;  
• **optional comma** between struct/class fields  
• # comments (Python/shell style)  
• := operator for let mut  
• **var** keyword for let mut  
• **def** add(a: int, b: int) int { a + b }     # no -> needed  
• **class** keyword with auto-derives (Debug, Clone, Copy)  
• automatic derives for enums/structs/classes in script mode  
• go-style return type annotation (-> optional)  
• i++ and i-- increment/decrement  
• js-style **arrow functions**  [1,2,3].apply(x=>x*2) == [2,4,6]   


### Boolean Operators
• and, or, not, xor, ¬, ∧, ∨ synonyms for archaic symbols && || !  
• **truthy** and falsy values in conditions **if** 1 { }  
• truthy **optionals**   let z : i32? = None; if z { ... } else { put!("🗸") }  

### Comparison & Range Operators
• ≤ ≥ ≠ comparison operators  
• ... and … inclusive **range operators**  
• `in` operator with auto-borrow:  `2 in [1,2,3]`  

### Math Features
• ** **power** operator with int and float support  
• Approximate equality   .1 + .2 ≈ .3   (also ~ as synonym)  
• Julia-style **implicit** multiplication: 2π → 2*π  
• τ (**tau**) and π (**pi**) constants baked in  τ == 2π  
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
• Unwrap shorthand via .! : val.! → val.unwrap()
• nil as alias for None

### Collections
• Simple lists @[1, 2, 3] → homogeneous Vec<i32>  
      • Magic lists @["hello", 42, true] → auto-wrapped Vec<Val>  
      • {key: value} untyped map literal syntax  
• @{key: value} typed map literal syntax  
• for (key, value) in map.pairs()  
• for (index, value) in list.pairs()  
• [1,2,3].apply(x=>x*2) == [2,4,6]  
• [1, 2, 3, 4].chose(x => x%2 == 0) == [2,4]  

### Type Aliases & Casting
• int = i32   float = f64   bool = boolean  
• as type conversion  '1' as int == 1 ,    '1' as codepoint == 47 ...  

## Future features
• functions return Results, yes, no need to write it  
• dynamic linking Swift ABI ...  
• dynamic linking with wit-like objects via dlsym C-ABI  


See [Goo](https://github.com/pannous/goo) the Go++ language extensions for a list of some planned features.   

