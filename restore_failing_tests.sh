#!/bin/bash
# Restore previously hidden failing tests back to probes/

PROBES_DIR="probes"
FAILING_DIR="probes/todo/"

if [[ ! -d "$FAILING_DIR" ]]; then
    echo "No failing tests to restore ($FAILING_DIR not found)"
    exit 0
fi

count=0
for file in "$FAILING_DIR"/*.rust; do
    [[ -f "$file" ]] || continue
    mv "$file" "$PROBES_DIR/"
    echo "Restored: $(basename "$file")"
    ((count++))
done

if [[ $count -gt 0 ]]; then
    rmdir "$FAILING_DIR" 2>/dev/null
    echo "---"
    echo "Restored $count tests to $PROBES_DIR/"
else
    echo "No .rust files found in $FAILING_DIR"
fi
