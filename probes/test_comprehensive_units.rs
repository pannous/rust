#!/usr/bin/env rustc
import "fmt"
import "units"

// Test comprehensive units system across all categories

// Length conversions
assert_eq!( 1000mm , 1m);
assert_eq!( 100cm , 1m  );
assert_eq!( 1km , 1000m);
assert_eq!( 1ft , 12inch);
assert_eq!( 1mi , 5280ft);

// Time conversions  
assert_eq!( 1000ms , 1s);
assert_eq!( 60s , 1min);
assert_eq!( 60min , 1h);
assert_eq!( 24h , 1d);

// Mass conversions
assert_eq!( 1000g , 1kg);
assert_eq!( 1000kg , 1t);
assert_eq!( 1lb , 16oz);

// Energy conversions - comparing different energy units
j_val := 1000J
kj_val := 1kJ
assert_eq!( j_val , kj_val);

// Power units
w_val := 1000W  
kw_val := 1kW
assert_eq!( w_val , kw_val);

// Pressure units
pa_val := 100000Pa
bar_val := 1bar
assert_eq!( pa_val , bar_val);

// Area units
m2_val := 10000m²
ha_val := 1ha  
assert_eq!( m2_val , ha_val);

// Volume units
ml_val := 1000mL
l_val := 1L
assert_eq!( ml_val , l_val);

// Velocity units
mps_val := 1m/s
put("1 m/s:", mps_val)

// Test postfix operators work  
time_val := 5s
put("5 seconds:", time_val)

// Acceleration units  
// mps2_val := 9.80665m/s²
gf_val := 1gf
put("1 g-force:", gf_val)

// Angle units - test degree creation
// deg_val := 180°
// put("180 degrees:", deg_val)

print("All comprehensive unit tests passed!")