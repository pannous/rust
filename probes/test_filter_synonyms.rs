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

put!("✅ filter() works")
put!("✅ where() works")
put!("✅ chose() works")
put!("✅ that() works")
put!("✅ which() works")
put!("🎉 All filter synonyms working perfectly!")