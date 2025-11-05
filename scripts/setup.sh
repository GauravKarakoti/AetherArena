#!/bin/bash

echo "🚀 Setting up AetherArena..."

# Build Linera application
echo "📦 Building Linera application..."
cd linera
cargo build --release
cd ..

# Build Rust orchestrator
echo "🔧 Building Rust orchestrator..."
cd rust-orchestrator
cargo build --release
cd ..

# Install frontend dependencies
echo "📱 Installing frontend dependencies..."
cd frontend
npm install
cd ..

echo "✅ Setup complete!"
echo ""
echo "To start development:"
echo "1. Start Linera network: linera net up"
echo "2. Start orchestrator: cd rust-orchestrator && cargo run"
echo "3. Start frontend: cd frontend && npm run dev"