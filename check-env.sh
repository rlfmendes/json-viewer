#!/bin/bash

# Environment verification script for json-viewer

set -e

echo "🔍 JSON Viewer - Environment Check"
echo "===================================="
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

check_command() {
    if command -v "$1" &> /dev/null; then
        echo -e "${GREEN}✓${NC} $1 is installed"
        if [ ! -z "$2" ]; then
            version=$($1 $2 2>&1 | head -n 1)
            echo "  └─ $version"
        fi
        return 0
    else
        echo -e "${RED}✗${NC} $1 is not installed"
        return 1
    fi
}

echo "Checking required tools..."
echo ""

# Check Rust
check_command "rustc" "--version"
check_command "cargo" "--version"

echo ""
echo "Checking optional tools..."
echo ""

# Check optional tools
check_command "rustfmt" "--version" || echo -e "  ${YELLOW}→ Install with: rustup component add rustfmt${NC}"
check_command "clippy-driver" "--version" || echo -e "  ${YELLOW}→ Install with: rustup component add clippy${NC}"

echo ""
echo "Checking Rust targets..."
echo ""

# Check targets
if rustup target list | grep -q "x86_64-unknown-linux-gnu (installed)"; then
    echo -e "${GREEN}✓${NC} x86_64-unknown-linux-gnu target installed"
else
    echo -e "${YELLOW}!${NC} x86_64-unknown-linux-gnu target not installed"
    echo -e "  ${YELLOW}→ Install with: rustup target add x86_64-unknown-linux-gnu${NC}"
fi

if rustup target list | grep -q "aarch64-unknown-linux-gnu (installed)"; then
    echo -e "${GREEN}✓${NC} aarch64-unknown-linux-gnu target installed"
else
    echo -e "${YELLOW}!${NC} aarch64-unknown-linux-gnu target not installed"
    echo -e "  ${YELLOW}→ Install with: rustup target add aarch64-unknown-linux-gnu${NC}"
fi

echo ""
echo "Checking project structure..."
echo ""

# Check important files
files=("Cargo.toml" "src/main.rs" "examples/sample-data.json")
for file in "${files[@]}"; do
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓${NC} $file exists"
    else
        echo -e "${RED}✗${NC} $file missing"
    fi
done

echo ""
echo "Checking dependencies..."
echo ""

if cargo fetch &> /dev/null; then
    echo -e "${GREEN}✓${NC} Dependencies fetched successfully"
else
    echo -e "${RED}✗${NC} Failed to fetch dependencies"
fi

echo ""
echo "Running quick build test..."
echo ""

if cargo check &> /dev/null; then
    echo -e "${GREEN}✓${NC} Project builds successfully"
else
    echo -e "${RED}✗${NC} Build check failed"
    echo ""
    echo "Try running: cargo check"
    exit 1
fi

echo ""
echo "===================================="
echo -e "${GREEN}✓ Environment check complete!${NC}"
echo ""
echo "You can now:"
echo "  • Run the app: cargo run -- examples/"
echo "  • Build release: cargo build --release"
echo "  • Run tests: cargo test"
echo "  • Create package: ./build.sh"
echo ""
echo "For more information, see:"
echo "  • QUICKSTART.md - Quick start guide"
echo "  • TESTING.md - Testing guide"
echo "  • DEVELOPMENT.md - Development guide"
