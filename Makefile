# Makefile

# Default target: build the project
all: build

# Build the Rust project
build:
	cargo build

# Run the project
run:
	cargo run

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
