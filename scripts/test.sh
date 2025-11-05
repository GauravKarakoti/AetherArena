#!/bin/bash

echo "🧪 Testing AetherArena..."

# Test Linera application
echo "Testing Linera application..."
cd linera
cargo test
cd ..

# Test Rust orchestrator
echo "Testing Rust orchestrator..."
cd rust-orchestrator
cargo test
cd ..

echo "✅ Tests completed!"