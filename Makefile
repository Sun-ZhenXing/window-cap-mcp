.PHONY: all build build-rust build-python build-example clean install test help

# Default target
all: build

# Build everything
build: build-rust build-example build-python

# Build Rust library
build-rust:
	@echo "=== Building Rust library ==="
	cargo build --release
	@echo "Rust library built successfully"
	@echo ""

# Build Rust example
build-example:
	@echo "=== Building Rust example ==="
	cargo build --example rust_library_example --release
	@echo "Rust example built successfully"
	@echo ""

# Build Python library (requires maturin)
build-python:
	@echo "=== Building Python library ==="
	uv build
	@echo "Python library built successfully"
	@echo "Python wheels available in target/wheels/"
	@echo ""

# Build for development (debug mode)
build-dev:
	@echo "=== Building in debug mode ==="
	cargo build
	@echo "Debug build complete"
	@echo ""

# Run the MCP server
run:
	cargo run --release

# Run the Rust example
run-example:
	cargo run --example rust_library_example --release

# Install Python package locally
install-python: build-python
	@echo "=== Installing Python package ==="
	pip install --force-reinstall target/wheels/window_cap_mcp-*.whl
	@echo "Python package installed"
	@echo ""

# Run tests
test:
	@echo "=== Running Rust tests ==="
	cargo test
	@echo ""
	@echo "=== Running Python tests ==="
	uv run pytest
	@echo "All tests passed"
	@echo ""

# Clean build artifacts
clean:
	@echo "=== Cleaning build artifacts ==="
	cargo clean
	rm -rf python/window_cap_mcp/*.pyd
	rm -rf python/window_cap_mcp/__pycache__
	rm -rf tests/__pycache__
	@echo "Clean complete"
	@echo ""

# Format code
fmt:
	cargo fmt
	uv run ruff format .

# Check code
check:
	cargo check
	cargo clippy -- -D warnings

# Display help
help:
	@echo "Window Capture MCP - Build System"
	@echo ""
	@echo "Available targets:"
	@echo "  all              - Build everything (default)"
	@echo "  build            - Build Rust library, example, and Python bindings"
	@echo "  build-rust       - Build Rust library only"
	@echo "  build-python     - Build Python bindings only"
	@echo "  build-example    - Build Rust example only"
	@echo "  build-dev        - Build in debug mode"
	@echo "  run              - Run the MCP server"
	@echo "  run-example      - Run the Rust example"
	@echo "  test-cli         - Test Python CLI"
	@echo "  install-python   - Build and install Python package"
	@echo "  test             - Run all tests (Rust + Python)"
	@echo "  clean            - Remove build artifacts"
	@echo "  fmt              - Format code"
	@echo "  check            - Check code and run clippy"
	@echo "  help             - Display this help message"
	@echo ""
	@echo "Usage:"
	@echo "  MCP Server: ./target/release/window-cap-mcp"
	@echo "  Rust Example: make run-example"
	@echo "  Python: make install-python"
