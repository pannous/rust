#!/usr/bin/env rust
class User {
    name:string,
    age:int
}

// users := @[User{name: "Bob", age: 17}, User{name: "Charlie", age: 22}, User{name: "Alice", age: 20}]
users := vec![User{name: "Bob", age: 17}, User{name: "Charlie", age: 22}, User{name: "Alice", age: 20}]
// let user = users.chose(|u| u.age > 18).first_cloned() // todo vs .head()
let user = users.chose(|u| u.age > 18).first().cloned() // todo vs .head()
// let user = users.chose(|u| u.age > 18).first()

let result:string
if user and user.?name {
    result = user.?name ?? "Anonymous"
} else {
    result = "Anonymous"
}

put!("First user over 18: ", result)