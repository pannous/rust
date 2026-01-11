#!/usr/bin/env rustc
import "fmt"
import "units"

s := units.S # shorthand for 1 second:
// s := units.Second(1)
ms := units.Ms
assert_eq!( 500*ms + 5*s , 5500*ms);
assert_eq!( 500·ms + 5·s , 5500·ms);
assert_eq!( 500ms + 5s , 5500ms);

m := units.M
put(m)
#m := units.Meter
km := units.Km
assert_eq!( 1200m + 2km , 3.2km);
assert_eq!( 1200*m + 2*km , 3.2*km);

assert_eq!( 3**3 , 27);
assert_eq!( 10m * 10m , 100m²);
assert_eq!( 10m * 10m * 10m , 1000m³);
#assert_eq!( 10m ** 3 , 1000m³);

assert_eq!( 10m / 2s , 5m/s);
assert_eq!( 10m / 2s , 5·m/s);

put(units.Available().area)
put(units.Km)
put("All unit tests passed successfully!")
