# Default target
all: build

# Build target
build:
	dotnet build --configuration Debug src\dotnet\HttpGenerator.slnx

# Optional release build target
release:
	dotnet build --configuration Release src\dotnet\HttpGenerator.slnx

# Test target
test:
	dotnet test --configuration Debug src\dotnet\HttpGenerator.slnx

# Clean target
clean:
	dotnet clean src\dotnet\HttpGenerator.slnx

