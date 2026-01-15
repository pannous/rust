#!/bin/bash
# Fixed rebuild script that handles sccache issues
#
# Workaround for sccache + CARGO_INCREMENTAL conflict:
# Set CARGO_INCREMENTAL="" and unset RUSTC_WRAPPER

export CARGO_INCREMENTAL=""
env -u RUSTC_WRAPPER ./x.py build --stage 1

echo ""
echo "✅ Build complete!"
echo ""
echo "Test with:"
echo "  cd probes && rust -Z script test_main.rs"
echo "  cd probes && ./test_main.rs"
