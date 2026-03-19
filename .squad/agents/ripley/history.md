# Ripley — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** .NET 8.0 CLI, C#, NSwag, Microsoft.OpenApi, Spectre.Console.Cli, xUnit, FluentAssertions

HTTP File Generator is a CLI tool and Visual Studio extension that generates `.http` files from OpenAPI specs (v2.0, v3.0, v3.1) for use with REST Client extensions in VS Code, JetBrains, and Visual Studio 2022.

**Key files:**
- `src/HttpGenerator/GenerateCommand.cs` — CLI command implementation
- `src/HttpGenerator.Core/HttpFileGenerator.cs` — Core generation logic
- `src/HttpGenerator.Core/OpenApiDocumentFactory.cs` — OpenAPI parsing
- `src/HttpGenerator/Settings.cs` — CLI options
- `test/OpenAPI/` — Test OpenAPI specs

**Build commands:**
- Build: `dotnet build HttpGenerator.sln --configuration Release`
- Test: `dotnet test HttpGenerator.sln --configuration Release`
- Run: `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- [options]`

## Learnings
