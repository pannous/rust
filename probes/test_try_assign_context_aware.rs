#!/usr/bin/env rustc
// Test context-aware try operator adapting to different function signatures

// import "strconv"

// Function that returns nothing - should use panic(err)
def testVoidFunction() {
    try val := strconv.Atoi("invalid")  // Should become: panic(err)
    put!("Got value: %d", val)
}

// Function that returns only an error - should use return err
def testErrorOnlyFunction() error {
    try val := strconv.Atoi("invalid")  // Should become: return err
    put!("Got value: %d", val)
    return nil
}

// Function that returns multiple values - should use return zero values, err
def testMultiReturnFunction() (int, string, error) {
    try val := strconv.Atoi("invalid")  // Should become: return 0, "", err
    return val, "success", nil
}

put!("Testing void function (should panic):\n")
try {
    testVoidFunction()
} catch e {
    put!("Caught panic: %v\n", e)
}

put!("Testing error function:\n")
err := testErrorOnlyFunction()
if err != nil {
    put!("Got error: %v\n", err)
}

put!("Testing multi-return function:\n")
result, msg, err2 := testMultiReturnFunction()
if err2 != nil {
    put!("Got error: %v\n", err2)
} else {
    put!("Result: %d, Message: %s\n", result, msg)
}

put!("Context-aware try tests completed!\n")