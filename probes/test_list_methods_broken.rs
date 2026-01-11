#!/usr/bin/env rustc
aList:=[2, 1, 3]

# TODOs
aList.sort!() // sorts in place! ok, but:
assert_eq!( aList , [1,2,3]);
assert_eq!( [3,1,2].sort() , aList );
assert_eq!( aList.sortDesc() , [3,2,1]);
assert_eq!( aList.shift() , 1 ); // does not remove it because no '!' todo confusing? remove?
assert_eq!( aList , [1, 2 ,3] );
assert_eq!( aList.shift!() , 1 ); // TODO and removes it:
// assert_eq!( aList , [2 ,3] ); // TODO ^^

assert_eq!( aList.insert(1,1) , [1,1,2,3]  ); // 0: before index 0 == prepend
// check aList.insert!(1,1) !TODO_implement_runtime_function_for_list_insert!_unknown list method(aList)
// assert_eq!( aList , [1,1,2,3]  ); // 0: before index 0 == prepend
// assert_eq!( aList.removeAt(0) , 1 ); // TODO_implement_runtime_function_for_list_removeAt_remove_at_index
aList.reverse!() 
assert_eq!( aList , [3, 2, 1]);
aList.reverse!() // ok, but:
put(aList.reverse() )
assert_eq!( aList.reverse() , [3, 2, 1]);
assert_eq!( aList , [1, 2, 3] # unaffected without '!' ^^);
aList.reverse!() 
assert_eq!( aList.pop!() , 1 ); // TODO and removes it
put(aList)
// assert_eq!( aList , [3, 2] ); // TODO ^^


#put("cannot use aList (variable of type []int) as []string value in argument to strings.Join")

assert_eq!( ["1","2","3"].join("-") , "1-2-3"  ); // ok, but:
assert_eq!( [1,2,3].join("-") , "1-2-3"  ); // if all elements stringifiable TODO cannot use aList (variable of type []int) as []string value in argument to strings.Join
# assert_eq!( aList.prepend(0) , [0,1,2,3] ); //TODO  missing type in composite literal


printf("All tests completed successfully.\n")
