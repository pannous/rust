#!/usr/bin/env rust
# Test all implemented synonyms

nums := [1, 2, 3, 4, 5]
text := "Hello World"

put!("🎯 SLICE/VEC METHOD SYNONYMS:")

# Map synonyms - transform array with closure
put!("✅ mapped/apply/transform/convert all work")
eq!( nums.mapped(|x| x * 2) , nums.apply(|x| x * 2) )
eq!( nums.apply(|x| x * 2) , nums.transform(|x| x * 2) )
eq!( nums.transform(|x| x * 2) , nums.convert(|x| x * 2) )

# Filter synonyms - filter array with predicate
put!("✅ filtered/select/chose/that/which all work")
eq!( nums.filtered(|x| *x > 2) , nums.select(|x| *x > 2) )
eq!( nums.select(|x| *x > 2) , nums.chose(|x| *x > 2) )
eq!( nums.chose(|x| *x > 2) , nums.that(|x| *x > 2) )
eq!( nums.that(|x| *x > 2) , nums.which(|x| *x > 2) )

put!("\n🎯 STRING METHOD SYNONYMS:")

# Element access - first character
put!("✅ first/head/start/begin all work")
eq!( text.first() , text.head() )
eq!( text.head() , text.start() )
eq!( text.start() , text.begin() )

# Element access - last character
put!("✅ last/tail/end all work")
eq!( text.last() , text.tail() )
eq!( text.tail() , text.end() )

# Size synonyms
put!("✅ size/length work")
eq!( text.size() , text.length() )
eq!( text.length() , text.len() )

# Reverse
put!("✅ reverse works")
eq!( text.reverse() , "dlroW olleH" )

# Search synonyms (contains)
put!("✅ contains/includes/has/holds all work")
eq!( text.contains("World") , text.includes("World") )
eq!( text.includes("World") , text.has("World") )
eq!( text.has("World") , text.holds("World") )

# Find synonyms
put!("✅ find/search/locate all work")
eq!( text.find("World") , text.search("World") )
eq!( text.search("World") , text.locate("World") )

# Replace synonyms
put!("✅ replace/substitute/swap all work")
eq!( text.replace("World", "Go") , text.substitute("World", "Go") )
eq!( text.substitute("World", "Go") , text.swap("World", "Go") )

put!("\n🎉 ALL SYNONYMS WORKING!")
put!("Slice: mapped/apply/transform/convert, filtered/select/chose/that/which")
put!("String: first/head/start/begin, last/tail/end, size/length, reverse")
put!("String: includes/has/holds, search/locate, substitute/swap")