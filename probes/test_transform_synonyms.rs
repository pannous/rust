#!/usr/bin/env rust
// import "slices"

nums := [1, 2, 3]

// Test transform synonyms (without conflicting "map" keyword)
result1 := nums.apply(x => x*2)
result2 := nums.transform(x => x*2) 
result3 := nums.convert(x => x*2)

expected := []int{2, 4, 6}

eq!( result1 , expected);
eq!( result2 , expected  );
eq!( result3 , expected);

print("✅ apply() works")
print("✅ transform() works")
print("✅ convert() works")
print("🛡️ No map keyword conflict!")
print("🎉 Transform synonyms working safely!")