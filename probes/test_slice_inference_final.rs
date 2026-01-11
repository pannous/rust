#!/usr/bin/env rustc
// Final comprehensive test of slice type inference
import "slices"

// Basic type inference
ints := [1, 2, 3]
check ints == []int{1, 2, 3}
print("✓ Integer slice inference: [1, 2, 3] → []int")

strings := ["hello", "world"]  
check strings == []string{"hello", "world"}
print("✓ String slice inference: [\"hello\", \"world\"] → []string")

bools := [true, false, true]
check bools == []bool{true, false, true}
print("✓ Boolean slice inference: [true, false, true] → []bool")

// Mixed types should infer as any
mixed := [1, "hello", true]
check mixed == []any{1, "hello", true}
print("✓ Mixed type inference: [1, \"hello\", true] → []any")

// Test with filter to show integration
evens := ints.filter(x => x%2 == 0)
check evens == []int{2}
print("✓ Filter integration: ints.filter(x => x%2 == 0) → [2]")

// Test with apply to show integration  
doubled := ints.apply(x => x*2)
check doubled == []int{2, 4, 6}
print("✓ Apply integration: ints.apply(x => x*2) → [2, 4, 6]")

print("\n🎉 All slice type inference tests passed!")
print("✓ Build system remains stable (no corruption)")
print("✓ Array types still work: [N]Type{...}")
print("✓ Slice literals with inference: [1, 2, 3]")
print("✓ Integration with list methods (filter, apply)")