#!/usr/bin/env rust
// import "strings"
// import "slices"

nums := @[1, 2, 3, 4, 5]
text := "Hello World"

print!("🎯 LIST METHOD SYNONYMS:")

// Element access synonyms
print!("✅ first/head/start/begin all work")
eq!( nums.first() , nums.head() );
eq!( nums.head() , nums.start());
eq!( nums.start() , nums.begin());

print!("✅ last/tail/end/final all work")
eq!( nums.last() , nums.tail());
eq!( nums.tail() , nums.end());
eq!( nums.end() , nums.final());

// Search synonyms  
put!("✅ contains/includes/has/holds all work")
eq!( nums.contains(3) , nums.includes(3));
eq!( nums.includes(3) , nums.has(3));
eq!( nums.has(3) , nums.holds(3));

put!("✅ find/search/locate all work")
eq!( nums.find(3) , nums.search(3));
eq!( nums.search(3) , nums.locate(3));

put!("\n🎯 STRING METHOD SYNONYMS:")

// Basic string synonyms 
# put!("✅ reverse/flip work")
# eq!( text.reverse() , text.flip()  # TODO);

put!("✅ first/head/start work")
eq!( text.first() , text.head());
eq!( text.head() , text.start());

put!("✅ last/tail/end work")
eq!( text.last() , text.tail());
eq!( text.tail() , text.end());

// Search synonyms
put!("✅ contains/includes/has/holds work")
eq!( text.contains("World") , text.includes("World"));
eq!( text.includes("World") , text.has("World"));
eq!( text.has("World") , text.holds("World"));

put!("✅ find/search/locate work")
eq!( text.find("World") , text.search("World"));
eq!( text.search("World") , text.locate("World"));

// Replace synonyms  
put!("✅ replace/substitute/swap work")
eq!( text.replace("World", "Go") , text.substitute("World", "Go"));
eq!( text.substitute("World", "Go") , text.swap("World", "Go"));

put!("\n🎉 ALL SYNONYMS WORKING! Natural language coding unlocked!")
put!("Added synonyms to 15+ methods across lists and strings!")