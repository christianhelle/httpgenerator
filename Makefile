# Default target
all: build

# Build target
build:
	dotnet build --configuration Debug src\dotnet\HttpGenerator.sln

# Optional release build target
release:
	dotnet build --configuration Release src\dotnet\HttpGenerator.sln

# Test target
test:
	dotnet test --configuration Debug src\dotnet\HttpGenerator.sln

# Clean target
clean:
	dotnet clean src\dotnet\HttpGenerator.sln

