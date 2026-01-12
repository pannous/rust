#!/usr/bin/env rustc
// import "fmt"
// Test map dot notation (m.key) for maps with string keys

// Test basic map dot notation with map[string]any
user := map[string]any{"name": "John", "age": 30, "city": "New York"}
put!("Name: %v\n", user.name)
put!("Age: %v\n", user.age)
put!("City: %v\n", user.city)

// Test map dot notation with map[string]int
scores := map[string]int{"math": 95, "english": 87, "science": 92}
put!("Math score: %v\n", scores.math)
put!("English score: %v\n", scores.english)
put!("Science score: %v\n", scores.science)

// Test map dot notation with map[string]string
config := map[string]string{"theme": "dark", "language": "en", "timezone": "UTC"}
put!("Theme: %v\n", config.theme)
put!("Language: %v\n", config.language)

// Test map dot notation with map[string]bool
flags := map[string]bool{"debug": true, "verbose": false, "production": true}
put!("Debug: %v\n", flags.debug)
put!("Verbose: %v\n", flags.verbose)

// Test that original indexing still works
put!("Name (index): %v\n", user["name"])
put!("Math (index): %v\n", scores["math"])
put!("Theme (index): %v\n", config["theme"])

// Test in expressions
totalScore := scores.math + scores.english + scores.science
put!("Total score: %v\n", totalScore)

// Test in conditionals
if flags.debug {
    put!("Debug mode is enabled\n")
}

if !flags.verbose {
    put!("Verbose mode is disabled\n")
}

// Test assignment from map dot notation
userName := user.name
userAge := user.age
put!("Assigned name: %v, age: %v\n", userName, userAge)

// Test function calls with map dot notation
nameLength := len(config.language)
put!("Language name length: %v\n", nameLength)

put!("All map dot notation tests completed successfully.\n")