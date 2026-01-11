Fork of official Rust

Rust is a conceptually beautiful language with sometimes horrible syntax. 
We take the power of Rust and just make it more beautiful by removing bloat wherever we see it. 

Philosophy:
• beauty without compromising correctness

Features working:
• and, or, not, xor, ¬, ∧, ∨ synonyms for archaic symbols && || !
• shebang support:
• run rust as scripts with implicit main
• ./probes/test_main.rs
• dynamic linking via dlsym C-ABI (rust ABI dependent on build!)

Future:
• Optionals via '?' as in other sane languages
• functions return Results, yes, no need to write it
• dynamic linking Swift ABI ...

See [Goo](https://github.com/pannous/goo) the Go++ language extensions for a list of some planned features. 
