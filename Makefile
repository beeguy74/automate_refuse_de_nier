.PHONY: all build run clean test

all: build

build:
	cargo build

run:
	cargo run

clean:
	cargo clean

test:
	cargo test