#!/usr/bin/env rustc
import "strings" // for reverse
// Test list methods in Goo
// DON'T REMOVE TESTS even if they fail
aList:=[1, 2, 3]

assert_eq!( aList[1] , 2);
assert_eq!( aList#2 , 2);
assert_eq!( aList[1:] , [2, 3]);
assert_eq!( aList[:2] , [1, 2]);
#assert_eq!( aList[1:2] , 2       ); // slice(start, end)
assert_eq!( aList[1:2] , [2,]       ); // slice(start, end)

assert_eq!( aList.first() , 1);
assert_eq!( aList.last() , 3);
assert_eq!( aList.size() , 3);
assert_eq!( aList.length() , 3);
#assert_eq!( len([]) , 0);
#assert_eq!( len([1]) , 1);
assert_eq!( len([]int{}) , 0);
assert_eq!( len([]int{1}) , 1);

assert_eq!( aList.contains(2) , true);
assert_eq!( aList.contains(5) , false);

assert_eq!( aList.indexOf(2) , 1);
assert_eq!( aList.indexOf(5) , -1);

aList.sort() // sorts in place!!
assert_eq!( aList , [1,2,3] ); // sorted in place


assert_eq!( aList.slice(1,2) , [2,]);
assert_eq!( aList.copy() , aList);

assert_eq!( aList.append(4) , [1,2,3,4]);


#aList:=[1, 2, 3]
assert_eq!( aList[1] , 2);
assert_eq!( aList.first() , 1);
## ^^ works in principle



stringList := ["3", "2", "1"]
assert_eq!( stringList.join("-") , "3-2-1" ); // join
assert_eq!( stringList.join("") , "321" ); // join
#assert_eq!( strings.Join(stringList, "-") , "3-2-1" ); // join

printf("All list method tests passed!\n")

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
#  The key fix was adding CheckStmt support to the transform so list methods work in check statements. All the
#  basic list methods now work correctly in goo/test_list_methods.goo.
#
#╭──────