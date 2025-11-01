#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}JSON Viewer Build Script${NC}"
echo "=========================="

# Detect architecture (allow overrides via env variables)
ARCH=$(uname -m)

if [ -z "$RUST_TARGET" ] || [ -z "$DEB_ARCH" ]; then
    case $ARCH in
        x86_64)
            RUST_TARGET="x86_64-unknown-linux-gnu"
            DEB_ARCH="amd64"
            ;;
        aarch64|arm64)
            RUST_TARGET="aarch64-unknown-linux-gnu"
            DEB_ARCH="arm64"
            ;;
        *)
            echo -e "${RED}Unsupported architecture: $ARCH${NC}"
            exit 1
            ;;
    esac
fi

echo -e "${GREEN}Detected architecture: $ARCH${NC}"
echo -e "${GREEN}Rust target: $RUST_TARGET${NC}"
echo -e "${GREEN}Debian architecture: $DEB_ARCH${NC}"
echo ""

# Build the project
echo -e "${BLUE}Building project...${NC}"
cargo build --release --target $RUST_TARGET

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

# Create .deb package
echo ""
echo -e "${BLUE}Creating .deb package...${NC}"

# Create package directory structure
PKG_DIR="target/debian-package"
rm -rf $PKG_DIR
mkdir -p $PKG_DIR/DEBIAN
mkdir -p $PKG_DIR/usr/bin

# Copy binary
cp target/$RUST_TARGET/release/json-viewer $PKG_DIR/usr/bin/

# Create control file with correct architecture
sed "s/ARCH_PLACEHOLDER/$DEB_ARCH/" debian/control > $PKG_DIR/DEBIAN/control

# Build the package
PACKAGE_NAME="json-viewer_0.1.0_${DEB_ARCH}.deb"
dpkg-deb --build $PKG_DIR "target/$PACKAGE_NAME"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Package created: target/$PACKAGE_NAME${NC}"
    echo ""
    echo -e "${BLUE}To install:${NC}"
    echo "  sudo dpkg -i target/$PACKAGE_NAME"
    echo ""
    echo -e "${BLUE}Or to install dependencies and the package:${NC}"
    echo "  sudo apt-get install -f ./target/$PACKAGE_NAME"
else
    echo -e "${RED}✗ Package creation failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}Build complete!${NC}"
