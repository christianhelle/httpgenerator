# Hicks — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** .NET 8.0 CLI, C#, NSwag, Microsoft.OpenApi, Spectre.Console.Cli, xUnit, FluentAssertions

HTTP File Generator generates `.http` files from OpenAPI specs. Core logic is in `src/HttpGenerator.Core/`, CLI in `src/HttpGenerator/`.

**Key files:**
- `src/HttpGenerator/GenerateCommand.cs` — CLI command, reads Settings, invokes HttpFileGenerator
- `src/HttpGenerator.Core/HttpFileGenerator.cs` — Core generation logic
- `src/HttpGenerator.Core/OpenApiDocumentFactory.cs` — OpenAPI parsing (NSwag + Microsoft.OpenApi)
- `src/HttpGenerator/Settings.cs` — CLI options (output, output-type, base-url, custom-header, etc.)
- `src/HttpGenerator.VSIX/` — Visual Studio extension
- `src/VSCode/` — VS Code extension

**Build & validate:**
- Build: `dotnet build HttpGenerator.sln --configuration Release`
- Quick test: `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.0/petstore.json --output /tmp/test --no-logging`
- Expected output: 19 .http files for petstore

**Known patterns:**
- OpenAPI v3.1 requires `--skip-validation` flag
- Output types: `OneRequestPerFile` (default), `OneFile`, `OneFilePerTag`
- Variables: `@baseUrl`, `@contentType` defined at top of generated files
- Custom headers passed via `--custom-header` flag

## Learnings
