#!/bin/bash
set -e

# Cross-compilation build script for both architectures

echo "Building for x86_64..."
cargo build --release --target x86_64-unknown-linux-gnu

echo "Building for aarch64..."
cargo build --release --target aarch64-unknown-linux-gnu

echo ""
echo "Build complete! Binaries are located at:"
echo "  - target/x86_64-unknown-linux-gnu/release/json-viewer"
echo "  - target/aarch64-unknown-linux-gnu/release/json-viewer"
