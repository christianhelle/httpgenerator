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

# Publish target
publish:
	dotnet pack --no-build src/dotnet/HttpGenerator.Core/HttpGenerator.Core.csproj
	dotnet pack --no-build src/dotnet/HttpGenerator/HttpGenerator.csproj
	cargo publish --dry-run --allow-dirty

# Clean target
clean:
	dotnet clean src/dotnet/HttpGenerator.slnx
	cargo clean

