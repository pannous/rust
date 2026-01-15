#!/bin/bash
# Test runner for probe files
# Usage: ./run_all_tests.sh [options] [pattern]
#
# Options:
#   --compile-only    Only test compilation, don't run
#   --run             Also run the compiled binaries (default)
#   --verbose, -v     Show detailed output
#   --quick           Only test known-working files
#   --all             Test all files (may have many failures)
#   --list-working    List files that compile successfully
#   --list-failing    List files that fail to compile

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RUSTC="${SCRIPT_DIR}/../rustc"
TEMP_DIR="/tmp/probe_tests"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Options
COMPILE_ONLY=false
VERBOSE=false
QUICK_MODE=false
LIST_WORKING=false
LIST_FAILING=false
PATTERN=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --compile-only) COMPILE_ONLY=true; shift ;;
        --run) COMPILE_ONLY=false; shift ;;
        --verbose|-v) VERBOSE=true; shift ;;
        --quick) QUICK_MODE=true; shift ;;
        --all) QUICK_MODE=false; shift ;;
        --list-working) LIST_WORKING=true; shift ;;
        --list-failing) LIST_FAILING=true; shift ;;
        -*) echo "Unknown option: $1"; exit 1 ;;
        *) PATTERN="$1"; shift ;;
    esac
done

mkdir -p "$TEMP_DIR"

# Known-working files (compile AND run successfully)
WORKING_FILES=(
    test_and_or.rs
    test_const_pow.rs
    test_debug_imports.rs
    test_main.rs
    test_minimal_conflict.rs
    test_not.rs
    test_null_coalesce.rs
    test_optional_chain.rs
    test_optional_syntax.rs
    test_pow3.rs
    test_power.rs
    test_precedence.rs
    test_put.rs
    test_put_eq.rs
    test_script_complex.rs
    test_simple.rs
    test_string_replace.rs
    test_unicode_ops.rs
    test_with_main.rs
)

is_working_file() {
    local name="$1"
    for w in "${WORKING_FILES[@]}"; do
        [[ "$name" == "$w" ]] && return 0
    done
    return 1
}

# List modes
if $LIST_WORKING; then
    echo "Files that compile successfully:"
    for f in "${WORKING_FILES[@]}"; do
        [[ -f "$SCRIPT_DIR/$f" ]] && echo "  $f"
    done
    exit 0
fi

if $LIST_FAILING; then
    echo "Files that fail to compile (WIP/need features):"
    for f in "$SCRIPT_DIR"/test_*.rs; do
        name=$(basename "$f")
        is_working_file "$name" || echo "  $name"
    done
    exit 0
fi

PASSED=0
FAILED=0
SKIPPED=0
FAILED_TESTS=()

compile_test() {
    local file="$1"
    local name=$(basename "$file" .rs)
    local output_bin="$TEMP_DIR/$name"

    "$RUSTC" "$file" -o "$output_bin" 2>&1
}

run_test() {
    local file="$1"
    local name=$(basename "$file" .rs)
    local output_bin="$TEMP_DIR/$name"

    if $VERBOSE; then
        echo -n "Testing $name... "
    fi

    local compile_output
    local compile_status=0

    compile_output=$(compile_test "$file") || compile_status=$?

    if [[ $compile_status -ne 0 ]]; then
        ((FAILED++))
        FAILED_TESTS+=("$name")
        if $VERBOSE; then
            echo -e "${RED}COMPILE FAILED${NC}"
            echo "$compile_output" | head -10
        else
            echo -e "${RED}✗${NC} $name (compile)"
        fi
        return 1
    fi

    if $COMPILE_ONLY; then
        ((PASSED++))
        if $VERBOSE; then
            echo -e "${GREEN}COMPILED${NC}"
        else
            echo -e "${GREEN}✓${NC} $name"
        fi
        return 0
    fi

    # Run the binary
    if [[ -x "$output_bin" ]]; then
        local run_output
        local run_status=0
        run_output=$("$output_bin" 2>&1) || run_status=$?

        if [[ $run_status -ne 0 ]]; then
            ((FAILED++))
            FAILED_TESTS+=("$name")
            if $VERBOSE; then
                echo -e "${RED}RUN FAILED (exit $run_status)${NC}"
                echo "$run_output" | head -10
            else
                echo -e "${RED}✗${NC} $name (exit $run_status)"
            fi
            return 1
        fi
    fi

    ((PASSED++))
    if $VERBOSE; then
        echo -e "${GREEN}PASSED${NC}"
    else
        echo -e "${GREEN}✓${NC} $name"
    fi
    return 0
}

echo "Probe Test Runner"
echo "Using: $RUSTC"
if $QUICK_MODE; then
    echo -e "Mode: ${BLUE}quick${NC} (known-working files only)"
else
    echo -e "Mode: ${YELLOW}all${NC} (includes WIP files)"
fi
echo ""

# Determine which files to test
if $QUICK_MODE; then
    for name in "${WORKING_FILES[@]}"; do
        file="$SCRIPT_DIR/$name"
        [[ -f "$file" ]] || continue

        # Apply pattern filter
        if [[ -n "$PATTERN" && ! "$name" =~ $PATTERN ]]; then
            ((SKIPPED++))
            continue
        fi

        run_test "$file" || true
    done
else
    for file in "$SCRIPT_DIR"/test_*.rs; do
        [[ -f "$file" ]] || continue
        name=$(basename "$file")

        # Apply pattern filter
        if [[ -n "$PATTERN" && ! "$name" =~ $PATTERN ]]; then
            ((SKIPPED++))
            continue
        fi

        run_test "$file" || true
    done
fi

echo ""
echo "========================================"
echo -e "Results: ${GREEN}$PASSED passed${NC}, ${RED}$FAILED failed${NC}, $SKIPPED skipped"
echo "========================================"

if [[ ${#FAILED_TESTS[@]} -gt 0 && ${#FAILED_TESTS[@]} -le 20 ]]; then
    echo ""
    echo "Failed:"
    for t in "${FAILED_TESTS[@]}"; do
        echo "  - $t"
    done
fi

[[ $FAILED -eq 0 ]] && exit 0 || exit 1
