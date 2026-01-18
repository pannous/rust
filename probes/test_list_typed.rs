#!/usr/bin/env rust
xs := [1, 2, 3]
ys := @[1, 2, 3]
# Vec vs array - Vec already impls PartialEq<[T; N]>
eq!( ys , xs );
# Array vs Vec - use seq! (slice eq) for comparing via AsRef<[T]>
seq!( xs , ys );
