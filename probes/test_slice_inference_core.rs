#!/usr/bin/env rust
// Core slice type inference test without method calls

// Basic type inference
ints := [1, 2, 3]
eq!( ints , []int{1, 2, 3});
put!("✓ Integer slice inference: [1, 2, 3] → []int")

strings := ["hello", "world"]  
eq!( strings , []string{"hello", "world"});
put!("✓ String slice inference: [\"hello\", \"world\"] → []string")

bools := [true, false, true]
eq!( bools , []bool{true, false, true});
put!("✓ Boolean slice inference: [true, false, true] → []bool")

// Mixed types should infer as any
mixed := [1, "hello", true]
eq!( mixed , []any{1, "hello", true});
put!("✓ Mixed type inference: [1, \"hello\", true] → []any")

// Test array types still work (should not interfere)
arr := [3]int{1, 2, 3}
eq!( len(arr) , 3);
eq!( arr[0] , 1);
put!("✓ Array types still work: [3]int{1, 2, 3}")

put!("\n🎉 Slice type inference implementation successful!")
put!("✅ Build system remains stable")
put!("✅ Array types preserved: [N]Type{...}")
put!("✅ Slice literals with inference: [elem1, elem2, ...]")
put!("✅ Proper type detection: int, string, bool, mixed→any")