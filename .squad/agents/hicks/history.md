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

### PR #319: Fixed query parameter dropping bug (issue #315)
**Date:** 2025-01-17

**Problem:** Query parameters were silently dropped when an operation had both path and query parameters. The URL construction logic checked `url.Contains("{")` AFTER escaping `{` to `{{`, so the condition was always true for paths with parameters, making the query string append branch unreachable.

**Solution:** Modified `GenerateRequest()` in `HttpFileGenerator.cs` to:
1. Separate path params (those in the URL template) from query params (those not in URL template)
2. Replace path parameter placeholders first: `{{owner}}` → `{{ownerVarName}}`
3. Always append query params as query string: `?key={{varName}}&key2={{varName2}}`

**Pattern learned:** When building URLs from OpenAPI operations, distinguish between path parameters (appear in URL template as `{param}`) and query parameters (must be appended as `?key=value`). Check parameter location against the original URL template BEFORE escaping braces.

**Testing:** All 171 unit tests pass. Validated with petstore.json showing correct output for mixed param types:
- Query only: `GET {{baseUrl}}/user/login?username={{username}}&password={{password}}`
- Path + query: `POST {{baseUrl}}/pet/{{petId}}?name={{name}}&status={{status}}`
