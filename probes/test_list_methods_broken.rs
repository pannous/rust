#!/usr/bin/env rustc
aList:=[2, 1, 3]

# TODOs
aList.sort!() // sorts in place! ok, but:
eq!( aList , [1,2,3]);
eq!( [3,1,2].sort() , aList );
eq!( aList.sortDesc() , [3,2,1]);
eq!( aList.shift() , 1 ); // does not remove it because no '!' todo confusing? remove?
eq!( aList , [1, 2 ,3] );
eq!( aList.shift!() , 1 ); // TODO and removes it:
// eq!( aList , [2 ,3] ); // TODO ^^

eq!( aList.insert(1,1) , [1,1,2,3]  ); // 0: before index 0 == prepend
// assert!()aList.insert!(1,1) !TODO_implement_runtime_function_for_list_insert!_unknown list method(aList)
// eq!( aList , [1,1,2,3]  ); // 0: before index 0 == prepend
// eq!( aList.removeAt(0) , 1 ); // TODO_implement_runtime_function_for_list_removeAt_remove_at_index
aList.reverse!() 
eq!( aList , [3, 2, 1]);
aList.reverse!() // ok, but:
put!(aList.reverse() )
eq!( aList.reverse() , [3, 2, 1]);
eq!( aList , [1, 2, 3] # unaffected without '!' ^^);
aList.reverse!() 
eq!( aList.pop!() , 1 ); // TODO and removes it
put!(aList)
// eq!( aList , [3, 2] ); // TODO ^^


#put!("cannot use aList (variable of type []int) as []string value in argument to strings.Join")

eq!( ["1","2","3"].join("-") , "1-2-3"  ); // ok, but:
eq!( [1,2,3].join("-") , "1-2-3"  ); // if all elements stringifiable TODO cannot use aList (variable of type []int) as []string value in argument to strings.Join
# eq!( aList.prepend(0) , [0,1,2,3] ); //TODO  missing type in composite literal


put!("All tests completed successfully.\n")
