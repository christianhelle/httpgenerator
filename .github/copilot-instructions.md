# HTTP File Generator

HTTP File Generator is a .NET 8.0 CLI tool and Visual Studio extension that generates .http files from OpenAPI specifications for use with REST Client extensions in IDEs like VS Code, JetBrains, and Visual Studio 2022.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Build and Test (CRITICAL TIMING)
- **NEVER CANCEL BUILD OR TEST COMMANDS** - Always wait for completion
- Initial restore: `dotnet restore HttpGenerator.sln` (75 seconds)
- Release build: `dotnet build HttpGenerator.sln --configuration Release` (20 seconds) 
- Debug build: `dotnet build HttpGenerator.sln --configuration Debug` (3 seconds after restore)
- Unit tests: `dotnet test HttpGenerator.sln --configuration Release` (17 seconds, 1 test may fail due to network)
- Smoke tests: `cd test && ./smoke-tests.sh` (4.5 minutes - NEVER CANCEL, set timeout to 10+ minutes)
- **KNOWN ISSUE**: Makefile `make test` fails due to test directory name collision - use `make -B test` to force execution

### Development Environment Setup
- Install .NET 8.0 SDK
- Clone repository: `git clone https://github.com/christianhelle/httpgenerator.git`
- Build immediately after clone:
  ```bash
  dotnet restore HttpGenerator.sln
  dotnet build HttpGenerator.sln --configuration Release
  ```

### CLI Tool Usage and Validation
- Run tool: `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- [options]`
- **Basic usage**: `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.0/petstore.json --output /tmp/output --no-logging`
- **Network limitations**: External URLs may fail, use local OpenAPI files from `test/OpenAPI/` directory
- **OpenAPI version support**: 
  - v2.0 and v3.0: Full support
  - v3.1: Requires `--skip-validation` flag
- **Output types**: `OneRequestPerFile` (default), `OneFile`, `OneFilePerTag`
- **Always test changes** by running the CLI with local OpenAPI files after modifications

## Validation Scenarios

### Manual Testing Requirements
Always validate changes by running these complete scenarios:

1. **Basic Generation Test**:
   ```bash
   dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.0/petstore.json --output /tmp/test --no-logging
   ```
   Expected: 19 .http files generated successfully

2. **Single File Mode Test**:
   ```bash
   dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.0/petstore.json --output /tmp/test-single --output-type OneFile --no-logging
   ```
   Expected: 1 Requests.http file with all endpoints

3. **Custom Headers and IntelliJ Tests**:
   ```bash
   dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.0/petstore.json --output /tmp/test-custom --generate-intellij-tests --custom-header "X-API-Key: test123" --base-url https://api.example.com --no-logging
   ```
   Expected: Files with custom headers and JavaScript test blocks

4. **OpenAPI v3.1 with Skip Validation**:
   ```bash
   dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.1/webhook-example.json --output /tmp/test-v31 --skip-validation --no-logging
   ```
   Expected: Files generated without validation errors

### Expected Generated Content Validation
Generated .http files should contain:
- Proper variable definitions (`@baseUrl`, `@contentType`)
- Request headers including custom headers
- Path parameters with example values
- Request bodies with example JSON
- IntelliJ test assertions when `--generate-intellij-tests` is used

## Project Structure and Navigation

### Core Projects
- `src/HttpGenerator/` - CLI application (.NET 8.0, main entry point)
- `src/HttpGenerator.Core/` - Core generation library (.NET Standard 2.0)
- `src/HttpGenerator.Tests/` - Unit tests (xUnit, FluentAssertions)
- `src/HttpGenerator.VSIX/` - Visual Studio extension
- `src/VSCode/` - VS Code extension

### Key Files to Know
- `src/HttpGenerator/GenerateCommand.cs` - Main CLI command implementation
- `src/HttpGenerator.Core/HttpFileGenerator.cs` - Core generation logic
- `src/HttpGenerator.Core/OpenApiDocumentFactory.cs` - OpenAPI document parsing
- `test/OpenAPI/` - Test OpenAPI specifications (v2.0, v3.0, v3.1)
- `test/smoke-tests.sh` - Comprehensive integration tests

### Common Navigation Patterns
- **When modifying CLI options**: Check `src/HttpGenerator/Settings.cs` and `GenerateCommand.cs`
- **When changing generation logic**: Focus on `src/HttpGenerator.Core/HttpFileGenerator.cs`
- **When adding tests**: Add to `src/HttpGenerator.Tests/` following existing patterns
- **When testing OpenAPI support**: Use files from `test/OpenAPI/v[version]/`

## Common Tasks and Commands

### Build Tasks
- Clean: `dotnet clean HttpGenerator.sln` (1 second)
- Build Debug: `make build` or `dotnet build HttpGenerator.sln --configuration Debug`
- Build Release: `make release` or `dotnet build HttpGenerator.sln --configuration Release`
- Full rebuild: `make clean && make build`

### Testing Tasks
- Unit tests: `make -B test` or `dotnet test HttpGenerator.sln --configuration Release` 
- Smoke tests: `cd test && ./smoke-tests.sh` (4.5 minutes)
- **CRITICAL**: Always run smoke tests after core changes to generation logic
- **Test failure note**: 1 unit test fails due to network restrictions (external URL test) - this is expected

### Development Workflow
1. Make code changes
2. Build: `dotnet build HttpGenerator.sln --configuration Release`
3. Quick test: `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.0/petstore.json --output /tmp/test --no-logging`
4. Run unit tests: `dotnet test HttpGenerator.sln --configuration Release`
5. Run smoke tests: `cd test && ./smoke-tests.sh` (ONLY if core generation logic changed)

### Package and Distribution
- CLI tool is packaged as NuGet global tool
- Build creates NuGet packages in `src/*/bin/Release/`
- Install globally: `dotnet tool install --global httpgenerator`

## Known Issues and Workarounds

### Build and Test Issues
- **Makefile test target**: Use `make -B test` instead of `make test` due to directory name collision
- **Network tests fail**: External URL tests fail in restricted environments - this is expected
- **OpenAPI 3.1 support**: Limited support, always use `--skip-validation` flag

### CLI Tool Limitations
- **External URLs**: May fail due to network restrictions, prefer local files
- **OpenAPI 3.1**: Requires `--skip-validation` for most files
- **Large specifications**: Some complex OpenAPI specs may take longer to process

### Development Environment
- **PowerShell required**: Smoke tests use PowerShell scripts (`pwsh`)
- **Test directory**: Don't remove `test/` directory as it breaks Makefile

## Dependencies and Technology Stack

### Primary Dependencies
- **.NET 8.0 SDK**: Required for CLI application
- **NSwag**: OpenAPI document parsing and code generation
- **Spectre.Console.Cli**: CLI framework with rich console output
- **Microsoft.OpenApi**: OpenAPI specification validation
- **xUnit + FluentAssertions**: Testing framework

### External Tool Requirements
- **PowerShell Core (pwsh)**: Required for smoke tests
- **Git**: Version control
- **Any .NET-compatible IDE**: Visual Studio, VS Code, JetBrains Rider

## Troubleshooting

### Common Build Errors
- **Missing .NET 8.0**: Install .NET 8.0 SDK from Microsoft
- **Package restore fails**: Run `dotnet restore HttpGenerator.sln` explicitly
- **Test failures**: Check if it's the expected network test failure

### CLI Tool Errors
- **"OpenAPI specification version '3.1.0' is not supported"**: Add `--skip-validation` flag
- **Network/URL errors**: Use local OpenAPI files from `test/OpenAPI/` directory
- **Empty output**: Check if OpenAPI spec has valid operations and paths

### Performance Issues
- **Slow builds**: First build after clone takes ~75 seconds for restore, subsequent builds are faster
- **Slow tests**: Smoke tests take 4.5 minutes - this is normal, never cancel