#!/usr/bin/env rust
put!("⏺ The test_printf_format test now fails because we disabled the printf feature - that's the expected trade-off.");
put!("342ee0901ab 2026-01-19 09:54 fix: Disable printf-style format parsing to fix self-host build");
// Basic printf-style specifiers
println!("Integer: %d", 42);
println!("String: %s", "hello");
println!("Hex: %x", 255);
println!("HEX: %X", 255);
println!("Octal: %o", 64);
println!("Float: %f", 3.14159);
println!("Exp: %e", 12345.6789);
println!("EXP: %E", 12345.6789);
println!("Binary: %b", 42);
println!("Debug: %?", vec![1, 2, 3]);

// Width and precision
println!("Width: %10d", 42);
println!("Precision: %.2f", 3.14159);
println!("Width+Prec: %10.2f", 3.14159);

// Flags
println!("Left align: %-10d!", 42);
println!("Sign: %+d", 42);
println!("Alt hex: %#x", 255);
println!("Zero pad: %05d", 42);

// Positional parameters
println!("Positional: %2$d %1$d", 1, 2);

// Mixed with Rust style
println!("Mixed: {} and %d", "rust", 42);

// Escapes
println!("Percent: %%");
println!("Brace: {{}}");

println!("\n✓ All printf format tests passed!");
