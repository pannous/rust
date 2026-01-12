#!/usr/bin/env rustc
// Test edge cases for map dot notation transformation
// import "fmt"

// Test 1: Nested map access
config := map[string]map[string]string{
    "database": {"host": "localhost", "port": "5432"},
    "redis": {"host": "127.0.0.1", "port": "6379"},
}

// This should transform to config["database"]["host"]
put!("Database host: %v\n", config.database.host)
put!("Redis port: %v\n", config.redis.port)

// Test 2: Map in struct
type Settings struct {
    flags map[string]bool
}

settings := Settings{
    flags: map[string]bool{"debug": true, "verbose": false},
}

// This should transform to settings.flags["debug"]
put!("Debug flag: %v\n", settings.flags.debug)

// Test 3: Map method calls with dot notation
data := map[string]string{"message": "hello world"}
put!("Message length: %v\n", len(data.message))

// Test 4: Complex expressions
users := map[string]map[string]any{
    "john": {"age": 30, "active": true},
    "jane": {"age": 25, "active": false},
}

// Multiple transformations in one expression
if users.john.active {
    put!("John is active and age %v\n", users.john.age)
}

// Test 5: Assignment from map dot notation
johnAge := users.john.age
put!("John's age: %v\n", johnAge)

put!("All edge case tests completed.\n")