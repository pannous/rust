#!/usr/bin/env rust
// import fmt
// import slices

# depends on test_list_typed.goo and map function test_list_map.goo
xs := @[1, 2, 3]
ys := xs.apply(x=>x*2) // should return @[2, 4, 6]

eq!( ys , @[2, 4, 6]);

nums := @[1, 2, 3, 4, 5]
eq!( nums.filter(x => x%2,1) == @[1, 3, 5]);
#eq!( nums.reduce((a,b)=>a+b, 0) , 15);
put!("All tests passed")
