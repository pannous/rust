#!/usr/bin/env rust
// import "./helper" // Local directory imports require explicit ./ prefix

message := helper.Hello()
print("Message:", message)
assert!()"Hello" in message
print("Import working successfully!")