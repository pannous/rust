#!/usr/bin/env rustc
enum Status { OK, BAD }
printf("OK = %v\n", OK)
printf("OK = %d\n", OK)
printf("OK = %s\n", OK) // strict needs String() method
printf("BAD = %v\n", BAD) // if String() method, else int

status := OK
printf("Variable status = %v\n", status)
assert_eq!( status , 0);
assert_eq!( status , OK);
