#!/bin/bash
set -e

WASM="dist/service.wasm"

# Build
echo "Building..."
./scripts/build.sh > /dev/null 2>&1

# Start server
echo "Starting server..."
wasmtime serve -S cli=y "$WASM" &
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

# Run tests
echo ""
node --test "tests/*.test.mjs"
