Custom fork of Rust. 

See @README.md for features of our custom build

Rebuild in 5 seconds with 
./rebuild.sh

for now we can put some extensions into
./compiler/rustc_builtin_macros/src/script_harness.rs

Before and after your work do
./run_all_tests.sh | tee test-results.log | grep Results
and 
git diff test-results.log | grep "\-✓"
To get the base line and make sure that no regression occurred. 