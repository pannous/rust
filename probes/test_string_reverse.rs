#!/usr/bin/env rust
// Test string reverse method

// Basic cases
eq!("hello".reverse(), "olleh");

// Unicode (multi-byte chars)
eq!("café".reverse(), "éfac");
eq!("日本語".reverse(), "語本日");
eq!("🎉🎊".reverse(), "🎊🎉");

put!("All checks passed!\n")
