  # Compile library with forked rustc
/opt/other/rust/build/host/stage1/bin/rustc --crate-type cdylib test_dynexport_lib.rs -o libdynexport_test.dylib
# /opt/other/rust/build/host/stage1/bin/rustc  --edition 2021 --crate-type cdylib test_dynexport_lib.rs -o libdynexport_test.dylib

  # Compile and run test
  rustc test_dynexport_user.rs && ./test_dynexport_user
  # rustc --edition 2021 test_dynexport_user.rs && ./test_dynexport_user