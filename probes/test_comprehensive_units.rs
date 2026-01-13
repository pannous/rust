#!/usr/bin/env rust
// import "fmt"
// import "units"

// Test comprehensive units system across all categories

// Length conversions
eq!( 1000mm , 1m);
eq!( 100cm , 1m  );
eq!( 1km , 1000m);
eq!( 1ft , 12inch);
eq!( 1mi , 5280ft);

// Time conversions  
eq!( 1000ms , 1s);
eq!( 60s , 1min);
eq!( 60min , 1h);
eq!( 24h , 1d);

// Mass conversions
eq!( 1000g , 1kg);
eq!( 1000kg , 1t);
eq!( 1lb , 16oz);

// Energy conversions - comparing different energy units
j_val := 1000J
kj_val := 1kJ
eq!( j_val , kj_val);

// Power units
w_val := 1000W  
kw_val := 1kW
eq!( w_val , kw_val);

// Pressure units
pa_val := 100000Pa
bar_val := 1bar
eq!( pa_val , bar_val);

// Area units
m2_val := 10000m²
ha_val := 1ha  
eq!( m2_val , ha_val);

// Volume units
ml_val := 1000mL
l_val := 1L
eq!( ml_val , l_val);

// Velocity units
mps_val := 1m/s
put!("1 m/s:", mps_val)

// Test postfix operators work  
time_val := 5s
put!("5 seconds:", time_val)

// Acceleration units  
// mps2_val := 9.80665m/s²
gf_val := 1gf
put!("1 g-force:", gf_val)

// Angle units - test degree creation
// deg_val := 180°
// put!("180 degrees:", deg_val)

print("All comprehensive unit tests passed!")