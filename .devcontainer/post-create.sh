#!/bin/bash

echo "🦀 Setting up Rust development environment..."

# Add rust components
rustup component add clippy rustfmt rust-src

# Add cross-compilation targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Pre-fetch dependencies
echo "📦 Fetching dependencies..."
cargo fetch

echo "✅ Setup complete! You can now:"
echo "   - Run: cargo run -- examples/"
echo "   - Build: cargo build --release"
echo "   - Test: cargo test"
echo ""
echo "To create a .deb package:"
echo "   1. chmod +x build.sh"
echo "   2. ./build.sh"
