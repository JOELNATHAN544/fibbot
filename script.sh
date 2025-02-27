#!/bin/sh

# Test case 1: Valid parameters
echo "Test Case 1: Valid parameters"
/app/target/release/fibbot true 100

# Test case 2: Invalid enable_fib (not a boolean)
echo "Test Case 2: Invalid enable_fib (not a boolean)"
/app/target/release/fibbot invalid_value 100

# Test case 3: Invalid max_threshhold (not a positive integer)
echo "Test Case 3: Invalid max_threshhold (not a positive integer)"
/app/target/release/fibbot true invalid_value

# Test case 4: Missing parameters
echo "Test Case 4: Missing parameters"
/app/target/release/fibbot