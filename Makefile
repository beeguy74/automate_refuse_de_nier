.PHONY: all clean test help  fmt 

SRC = src/main.rs src/lib.rs src/tools/dfa.rs src/tools/keycatcher.rs src/tools/mod.rs src/tools/parsing.rs src/tools/ui.rs 
TARGET = ./target/debug/automate

all: $(TARGET)

${TARGET}: $(SRC)
	@echo "Building ft_ality in development mode (using docker compose)..."
	# Use the builder service from docker compose so builds run in the official rust container.
	# This mounts the project into the container, so the produced target/ files appear on the host.
	# Ensure cargo bin is on PATH inside the container and run build
	docker compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; RUSTFLAGS=-Awarnings cargo build'

clean:
	@echo "Cleaning build artifacts (inside builder)..."
	docker compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; cargo clean'
	@rm -f *.o *~ core

test:
	@echo "Running unit tests (inside builder)..."
	docker compose run --rm builder bash -lc 'export PATH=/usr/local/cargo/bin:$$PATH; cargo test'

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
	@echo "Other:"
	@echo "  make help     - Display this help message"
	@echo ""