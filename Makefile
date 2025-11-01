.PHONY: help build run test clean clippy fmt release package

help:
	@echo "JSON Viewer - Available targets:"
	@echo "  make build    - Build debug version"
	@echo "  make release  - Build release version"
	@echo "  make run      - Run with example data"
	@echo "  make test     - Run tests"
	@echo "  make clippy   - Run clippy linter"
	@echo "  make fmt      - Format code"
	@echo "  make package  - Create .deb package"
	@echo "  make clean    - Clean build artifacts"

build:
	cargo build

release:
	cargo build --release

run:
	cargo run -- examples/

test:
	cargo test

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

package: release
	chmod +x build.sh
	./build.sh

clean:
	cargo clean
	rm -f target/*.deb
