#!/usr/bin/env rust
// Test string reverse method

// Basic cases
eq!("hello".reverse(), "olleh");
eq!("world".reverse(), "dlrow");
eq!("a".reverse(), "a");
eq!("ab".reverse(), "ba");
eq!("".reverse(), "");

// Palindromes
eq!("racecar".reverse(), "racecar");
eq!("level".reverse(), "level");
eq!("noon".reverse(), "noon");

// Numbers and mixed
eq!("12345".reverse(), "54321");
eq!("abc123".reverse(), "321cba");

// Spaces and special chars
eq!("hello world".reverse(), "dlrow olleh");
eq!("a b c".reverse(), "c b a");
eq!("!@#$%".reverse(), "%$#@!");

// Unicode (multi-byte chars)
eq!("café".reverse(), "éfac");
eq!("日本語".reverse(), "語本日");
eq!("🎉🎊".reverse(), "🎊🎉");

put!("All checks passed!\n")
