# Makefile for ft_ality - Fighting Game Combo Recognizer
# Rust-based automaton project

.PHONY: all build release clean test run debug gui gui-debug help stress fmt lint check run-file

# Default target
all: build

# Build in development mode (unoptimized, with debug symbols)
build:
	@echo "Building ft_ality in development mode..."
	cargo build

# Build in release mode (optimized)
release:
	@echo "Building ft_ality in release mode..."
	cargo build --release

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@rm -f *.o *~ core

# Run tests
test:
	@echo "Running unit tests..."
	cargo test

# Run stress tests with output
stress:
	@echo "Running performance stress tests..."
	cargo test --test stress_test -- --nocapture

# Run with example grammar (console mode)
run: build
	@echo "Running in console mode with mk9_with_moves.gmr..."
	./target/debug/automate_refuse_de_nier grammars/mk9_with_moves.gmr

# Run in debug mode (console)
debug: build
	@echo "Running console mode with debug tracing enabled..."
	./target/debug/automate_refuse_de_nier grammars/mk9_with_moves.gmr --debug

# Run with GUI mode
gui: build
	@echo "Running with GUI mode (SDL window)..."
	./target/debug/automate_refuse_de_nier grammars/mk9_with_moves.gmr --gui

# Run with both GUI and debug
gui-debug: build
	@echo "Running with GUI and debug mode..."
	./target/debug/automate_refuse_de_nier grammars/mk9_with_moves.gmr --gui --debug

# Run with custom file
run-file:
	@echo "Usage: make run-file FILE=<filename>"
	cargo run -- $(FILE)

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run clippy linter
lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Check code without building
check:
	@echo "Checking code..."
	cargo check

# Display help
help:
	@echo "ft_ality Makefile - Available targets:"
	@echo ""
	@echo "Build targets:"
	@echo "  make (all)    - Build in development mode (default)"
	@echo "  make build    - Build in development mode"
	@echo "  make release  - Build optimized release binary"
	@echo "  make clean    - Remove build artifacts"
	@echo ""
	@echo "Test targets:"
	@echo "  make test     - Run unit tests"
	@echo "  make stress   - Run performance stress tests"
	@echo ""
	@echo "Run targets:"
	@echo "  make run        - Run in console mode (default, no GUI)"
	@echo "  make debug      - Run in console mode with debug tracing"
	@echo "  make gui        - Run with SDL GUI window"
	@echo "  make gui-debug  - Run with GUI and debug tracing"
	@echo "  make run-file   - Run with custom grammar (use FILE=path/to/file.gmr)"
	@echo ""
	@echo "Code quality:"
	@echo "  make fmt      - Format code with rustfmt"
	@echo "  make lint     - Run clippy linter"
	@echo "  make check    - Check code without building"
	@echo ""
	@echo "Other:"
	@echo "  make help     - Display this help message"
	@echo ""
	@echo "Example usage:"
	@echo "  make run                    # Console mode"
	@echo "  make gui                    # GUI mode"
	@echo "  make debug                  # Console with debug"
	@echo "  make gui-debug              # GUI with debug"
	@echo "  ./target/debug/automate_refuse_de_nier grammars/mk9.gmr"
	@echo "  ./target/debug/automate_refuse_de_nier grammars/mk9.gmr --gui --debug"