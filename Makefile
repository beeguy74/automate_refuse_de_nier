.PHONY: all build run clean test

all: build

build:
	cargo build

run:
	cargo run -- grammars/mk9.gmr

clean:
	cargo clean

test:
	cargo test

run-file:
	@echo "Usage: make run-file FILE=<filename>"
	cargo run -- $(FILE)