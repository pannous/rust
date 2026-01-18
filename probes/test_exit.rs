#!/usr/bin/env rust
# Test exit! macro in script context

put!("Testing exit! macro")

# Test exit!() - default exit code 0
put!("Calling exit!()...")
exit!()
