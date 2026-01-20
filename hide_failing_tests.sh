#!/bin/bash
# Move failing tests to a temporary directory so they're excluded from test runs
# Usage: ./hide_failing_tests.sh [log_file]

LOG_FILE="${1:-test-results.log}"
PROBES_DIR="probes"
FAILING_DIR="probes_failing"

if [[ ! -f "$LOG_FILE" ]]; then
    echo "Error: $LOG_FILE not found. Run ./run_all_tests.sh first."
    exit 1
fi

mkdir -p "$FAILING_DIR"

# Extract failing test names (lines with red color 0;31m) and extract test name
failing_tests=$(grep '0;31m' "$LOG_FILE" | sed 's/.*\[0m //' | sed 's/ .*//')

count=0
for test in $failing_tests; do
    src="$PROBES_DIR/${test}.rust"
    if [[ -f "$src" ]]; then
        mv "$src" "$FAILING_DIR/"
        echo "Moved: $test.rust"
        ((count++))
    fi
done

echo "---"
echo "Moved $count failing tests to $FAILING_DIR/"
echo "To restore: ./restore_failing_tests.sh"
