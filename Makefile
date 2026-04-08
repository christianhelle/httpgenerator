# Default target
all: build

# Build target
build:
	dotnet build --configuration Debug legacy\HttpGenerator.sln

# Optional release build target
release:
	dotnet build --configuration Release legacy\HttpGenerator.sln

# Test target
test:
	dotnet test --configuration Debug legacy\HttpGenerator.sln

# Clean target
clean:
	dotnet clean legacy\HttpGenerator.sln

