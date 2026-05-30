#!/usr/bin/env bash
# Live System Test Runner for Left-Right Language
# Runs .lr files through `lr run` and validates exact output
#
# Usage: ./tests/live_runner.sh [test_file_or_dir]
#
# Test file format:
#   First line: LR expression/code
#   Optional comment lines starting with // that are ignored
#   Expected output on stdout is validated exactly
#
# Two modes:
#   1. Comment-based: LR code with // expect: <output> comment
#   2. Equality-based: expression = expected (returns true)
#   3. Match-based: .lr file paired with .lr.expected file containing exact output

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LR_BIN="${SCRIPT_DIR}/../target/release/lr"
TEST_DIR="${SCRIPT_DIR}/live"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m'

passed=0
failed=0
errors=0

run_test_file() {
    local test_file="$1"
    local basename=$(basename "$test_file" .lr)
    
    # Mode 1: Check for .lr.expected file with exact output
    local expected_file="${test_file%.lr}.expected"
    
    # Mode 2: Check for // expect: comment in file
    local expected_output=""
    local mode="equality"  # default: truthiness check
    
    if [[ -f "$expected_file" ]]; then
        mode="exact"
        expected_output=$(cat "$expected_file")
    else
        # Check for // expect: comment
        local expect_line=$(grep -m1 '// expect:' "$test_file" 2>/dev/null || true)
        if [[ -n "$expect_line" ]]; then
            mode="exact"
            expected_output=$(echo "$expect_line" | sed 's/.*\/\/ expect: *//')
        fi
    fi
    
    # Run lr and capture output
    local actual_output
    local run_rc=0
    actual_output=$("$LR_BIN" run "$test_file" 2>&1) || run_rc=$?
    if [[ $run_rc -ne 0 ]]; then
        if [[ $mode == "exact" ]]; then
            actual_output="$actual_output"
        else
            echo -e "  ${basename} ... ${RED}ERROR${NC}"
            echo "    Runtime error: $actual_output"
            ((errors++))
            return 1
        fi
    fi
    
    if [[ $mode == "exact" ]]; then
        if [[ "$actual_output" == "$expected_output" ]]; then
            echo -e "  ${basename} ... ${GREEN}PASS${NC} (exact)"
            ((passed++))
        else
            echo -e "  ${basename} ... ${RED}FAIL${NC} (exact)"
            echo "    Expected: $expected_output"
            echo "    Actual:   $actual_output"
            ((failed++))
            return 1
        fi
    else
        # Equality mode: check truthiness (like lr test)
        if [[ "$actual_output" == "true" ]]; then
            echo -e "  ${basename} ... ${GREEN}PASS${NC} (equality)"
            ((passed++))
        elif [[ "$actual_output" == "false" || "$actual_output" == "0" || "$actual_output" == "undefined" ]]; then
            echo -e "  ${basename} ... ${RED}FAIL${NC} (equality)"
            echo "    Result: $actual_output"
            ((failed++))
            return 1
        else
            # Non-boolean truthy result — pass for truthiness
            echo -e "  ${basename} ... ${GREEN}PASS${NC} (truthy: $actual_output)"
            ((passed++))
        fi
    fi
    
    return 0
}

echo -e "${CYAN}Left-Right Live System Tests${NC}"
echo ""

# Find test files
if [[ $# -gt 0 ]]; then
    target="$1"
    if [[ -f "$target" ]]; then
        files=("$target")
    elif [[ -d "$target" ]]; then
        mapfile -t files < <(find "$target" -name "*.lr" | sort)
    else
        echo -e "${RED}Not found: $target${NC}"
        exit 1
    fi
else
    mapfile -t files < <(find "$TEST_DIR" -name "*.lr" | sort)
fi

if [[ ${#files[@]} -eq 0 ]]; then
    echo -e "${YELLOW}No test files found in $TEST_DIR${NC}"
    exit 0
fi

echo -e "${CYAN}Running ${#files[@]} live system test(s)...${NC}"
echo ""

for f in "${files[@]}"; do
    run_test_file "$f" || true
done

echo ""
echo -e "${CYAN}Live Test Results:${NC}"
echo -e "  ${GREEN}${passed} passed${NC}, ${RED}${failed} failed${NC}, ${YELLOW}${errors} errors${NC}"

if [[ $((failed + errors)) -gt 0 ]]; then
    exit 1
fi

exit 0
