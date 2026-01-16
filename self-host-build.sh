#!/bin/bash
# Self-hosting sanity check: verify the compiler can compile itself
# Stage 1 (built by downloaded rustc) builds Stage 2 (proving self-hosting works)

set -e

echo "=== Self-hosting build (stage 2) ==="
echo "This builds the compiler with itself to verify self-hosting capability"
echo "Expected time: ~5 minutes"
echo ""

./x.py build --stage 2 compiler --warnings warn

STAGE2="./build/host/stage2/bin/rustc"

if [[ ! -x "$STAGE2" ]]; then
    echo "ERROR: Stage 2 compiler not found at $STAGE2"
    exit 1
fi

echo ""
echo "=== Verifying stage 2 compiler ==="
"$STAGE2" --version

echo ""
echo "=== Quick functionality test ==="
"$STAGE2" -Z script probes/test_main.rs -o /tmp/self_host_test && /tmp/self_host_test

echo ""
echo "=== Self-hosting check PASSED ==="
echo "Stage 2 compiler: $STAGE2"
