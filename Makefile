# Default target
all: build

# Build target
build:
	cargo build

# Optional release build target
release:
	cargo build --release

# Test target
test:
	cargo test

# Clean target
clean:
	cargo clean
