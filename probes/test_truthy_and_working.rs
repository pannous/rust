#!/usr/bin/env rust
class User {
    Name string
    Age int
}

users := @[User{Name: "Bob", Age: 17}, User{Name: "Charlie", Age: 22}, User{Name: "Alice", Age: 20}]
user := users.filter(u => u.Age > 18).first()

let result string
if user and user.Name {
    result = user.Name
} else {
    result = "Anonymous"
}

put!("First user over 18: ", result)