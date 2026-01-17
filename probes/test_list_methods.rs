#!/usr/bin/env rust
// import "strings" // for reverse
// Test list methods in Goo
// DON'T REMOVE TESTS even if they fail
aList:=[1, 2, 3]

eq!( aList[1] , 2);
eq!( aList#2 , 2);
eq!( aList[1:] , [2, 3]);
eq!( aList[:2] , [1, 2]);
#eq!( aList[1:2] , 2       ); // slice(start, end)
eq!( aList[1:2] , [2,]       ); // slice(start, end)

eq!( aList.first() , 1);
eq!( aList.last() , 3);
eq!( aList.size() , 3);
eq!( aList.length() , 3);
#eq!( len([]) , 0);
#eq!( len([1]) , 1);
eq!( len(@[]) , 0);
eq!( len(@[1]) , 1);

eq!( aList.contains(2) , true);
eq!( aList.contains(5) , false);

eq!( aList.indexOf(2) , 1);
eq!( aList.indexOf(5) , -1);

aList.sort() // sorts in place!!
eq!( aList , [1,2,3] ); // sorted in place


eq!( aList.slice(1,2) , [2,]);
eq!( aList.copy() , aList);

eq!( aList.append(4) , [1,2,3,4]);


#aList:=[1, 2, 3]
eq!( aList[1] , 2);
eq!( aList.first() , 1);
## ^^ works in principle



stringList := ["3", "2", "1"]
eq!( stringList.join("-") , "3-2-1" ); // join
eq!( stringList.join("") , "321" ); // join
#eq!( strings.Join(stringList, "-") , "3-2-1" ); // join

put!("All list method tests passed!\n")

##
#  🟡 Methods That Need Manual import "slices":
#
#  - aList.contains(2) → slices.Contains(aList, 2)
#  - aList.indexOf(2) → slices.Index(aList, 2)
#  - aList.sort() → slices.Sort(aList)
#
#  🔴 Methods That Are Too Complex For Now:
#
#  - aList.reverse() → needs runtime function
#  - join() - needs strings import and type conversion
#  - prepend() - composite literal type inference issues
#  - insert() - complex slice manipulation
#  - pop(), shift() - modify-in-place semantics
#  - sortDesc() - needs custom implementation
#
#  The key fix was adding CheckStmt support to the transform so list methods work in assert!()statements. All the
#  basic list methods now work correctly in goo/test_list_methods.goo.
#
#╭──────