.PHONY: all build release clean test run debug gui gui-debug help stress fmt lint check run-file

all: build

build:
	@echo "Building ft_ality in development mode (using docker-compose)..."
	# Use the builder service from docker-compose so builds run in the official rust container.
	# This mounts the project into the container, so the produced target/ files appear on the host.
	# Ensure cargo bin is on PATH inside the container and run build
	docker-compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; RUSTFLAGS=-Awarnings cargo build'

clean:
	@echo "Cleaning build artifacts (inside builder)..."
	docker-compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; cargo clean'
	@rm -f *.o *~ core

test:
	@echo "Running unit tests (inside builder)..."
	docker-compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; cargo test'


run: build
	@echo "Running in console mode with mk9_with_moves.gmr..."
	./target/debug/automate_refuse_de_nier grammars/mk9_with_moves.gmr

debug: build
	@echo "Running console mode with debug tracing enabled..."
	./target/debug/automate_refuse_de_nier grammars/mk9_with_moves.gmr --debug

.PHONY: run-container debug-container
run-container: build
	@echo "Running in container (console mode) using cargo run..."
	docker-compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; cargo run -- grammars/mk9_with_moves.gmr'

debug-container: build
	@echo "Running in container (console mode) with debug tracing..."
	docker-compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; cargo run -- grammars/mk9_with_moves.gmr --debug'

gui: build
	@echo "Running with GUI mode (SDL window)..."
	./target/debug/automate_refuse_de_nier grammars/mk9_with_moves.gmr --gui

gui-debug: build
	@echo "Running with GUI and debug mode..."
	./target/debug/automate_refuse_de_nier grammars/mk9_with_moves.gmr --gui --debug

fmt:
	@echo "Formatting code (inside builder)..."
	docker-compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; cargo fmt'

check:
	@echo "Checking code (inside builder)..."
	docker-compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; cargo check'

help:
	@echo "ft_ality Makefile - Available targets:"
	@echo ""
	@echo "Build targets:"
	@echo "  make (all)    - Build in development mode (default)"
	@echo "  make build    - Build in development mode"
	@echo "  make clean    - Remove build artifacts"
	@echo ""
	@echo "Test targets:"
	@echo "  make test     - Run unit tests"
	@echo ""
	@echo "Run targets:"
	@echo "  make run        - Run in console mode (default, no GUI)"
	@echo "  make debug      - Run in console mode with debug tracing"
	@echo "  make gui        - Run with SDL GUI window"
	@echo "  make gui-debug  - Run with GUI and debug tracing"
	@echo ""
	@echo "Code quality:"
	@echo "  make fmt      - Format code with rustfmt"
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