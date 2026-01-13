#!/usr/bin/env rust
// import "slices"

nums := [1, 2, 3, 4, 5, 6]

// Test all the synonyms
evens1 := nums.filter(x => x%2 == 0)
evens2 := nums.where(x => x%2 == 0)  
evens3 := nums.chose(x => x%2 == 0)
evens4 := nums.that(x => x%2 == 0)
evens5 := nums.which(x => x%2 == 0)

expected := []int{2, 4, 6}

eq!( evens1 , expected);
eq!( evens2 , expected);
eq!( evens3 , expected  );
eq!( evens4 , expected);
eq!( evens5 , expected);

print("✅ filter() works")
print("✅ where() works") 
print("✅ chose() works")
print("✅ that() works")
print("✅ which() works")
print("🎉 All filter synonyms working perfectly!")