#!/bin/bash
set -e

PORT=8080
WASM="dist/service.wasm"
PASSED=0
FAILED=0

# Build
echo "Building..."
./build.sh > /dev/null 2>&1

# Start server
echo "Starting server on port $PORT..."
mik run "$WASM" &
PID=$!
sleep 2

# Check if server started
if ! kill -0 $PID 2>/dev/null; then
    echo "Failed to start server"
    exit 1
fi

cleanup() {
    kill $PID 2>/dev/null || true
}
trap cleanup EXIT

# Test helper
test_endpoint() {
    local method=$1
    local url=$2
    local data=$3
    local expected=$4
    local description=$5

    if [ -n "$data" ]; then
        response=$(curl -sf -X "$method" "$url" -H "Content-Type: application/json" -d "$data" 2>/dev/null || echo "CURL_FAILED")
    else
        response=$(curl -sf -X "$method" "$url" 2>/dev/null || echo "CURL_FAILED")
    fi

    if echo "$response" | grep -q "$expected"; then
        echo "  PASS: $description"
        ((PASSED++))
    else
        echo "  FAIL: $description"
        echo "    Expected: $expected"
        echo "    Got: $response"
        ((FAILED++))
    fi
}

# Run tests
echo ""
echo "Running tests..."
echo ""

test_endpoint "GET" "http://localhost:$PORT/" "" "my-api" "GET / returns api info"
test_endpoint "GET" "http://localhost:$PORT/users" "" "Alice" "GET /users returns user list"
test_endpoint "GET" "http://localhost:$PORT/users/1" "" "alice@example.com" "GET /users/1 returns Alice"
test_endpoint "GET" "http://localhost:$PORT/users/2" "" "bob@example.com" "GET /users/2 returns Bob"
test_endpoint "POST" "http://localhost:$PORT/users" '{"name":"Test","email":"test@example.com"}' "Test" "POST /users creates user"

# Summary
echo ""
echo "Results: $PASSED passed, $FAILED failed"

if [ $FAILED -gt 0 ]; then
    exit 1
fi
