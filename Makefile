# Default target
all: build

# Build target
build:
	dotnet build --configuration Debug src/dotnet/HttpGenerator.slnx
	cargo build --workspace

# Optional release build target
release:
	dotnet build --configuration Release src/dotnet/HttpGenerator.slnx
	cargo build --release --workspace

# Test target
test:
	dotnet test --configuration Debug src/dotnet/HttpGenerator.slnx
	cargo test --workspace

# Clean target
clean:
	dotnet clean src/dotnet/HttpGenerator.slnx
	cargo clean

