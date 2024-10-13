# Default target
all: build

# Build target
build:
	dotnet build --configuration Debug HttpGenerator.sln

# Optional release build target
release:
	dotnet build --configuration Release HttpGenerator.sln

# Test target
test:
	dotnet test --configuration Debug HttpGenerator.sln

# Clean target
clean:
	dotnet clean HttpGenerator.sln

