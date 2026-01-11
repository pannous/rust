#!/usr/bin/env rustc
aList := [1, 2, 3]
assert_eq!( aList[1] , 2);
assert_eq!( aList.first() , 1);

assert_eq!( aList.sortDesc() , [3,2,1]);
assert_eq!( aList.pop() , 3);
assert_eq!( aList.shift() , 1);

printf("All tests completed successfully.\n")