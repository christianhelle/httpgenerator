# Hicks — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** .NET 8.0 CLI, C#, Rust workspace, Microsoft.OpenApi, Spectre.Console.Cli, xUnit, FluentAssertions

**Canonical product roots:**
- Rust workspace members: `src\rust\`
- .NET solution and apps: `src\dotnet\`
- VS Code extension: `src\VSCode\`

**Key implementation files:**
- `src\dotnet\HttpGenerator\GenerateCommand.cs`
- `src\dotnet\HttpGenerator.Core\HttpFileGenerator.cs`
- `src\dotnet\HttpGenerator.Core\OpenApiDocumentFactory.cs`
- `src\dotnet\HttpGenerator.VSIX\HttpGeneratorCli.cs`
- `src\rust\httpgenerator-cli\src\lib.rs`
- `src\rust\httpgenerator-openapi\src\inspect.rs`

**Build and validate:**
- `cargo test`
- `dotnet build src\dotnet\HttpGenerator.sln --configuration Release`
- `dotnet test src\dotnet\HttpGenerator.sln --configuration Release`
- `dotnet run --project src\dotnet\HttpGenerator\HttpGenerator.csproj -- test\OpenAPI\v3.0\petstore.json --output .\artifacts\http-out --no-logging`

**Historical summary:**
- Delivered small-scope dependency refreshes (SourceLink, Spectre.Console.Cli) and the minimal xUnit v3 migration needed by Atc.Test 2.x.
- Investigated and implemented host-sensitive CLI output parity work across Rust, VS Code, and VSIX surfaces.
- Cleaned up dead code and coverage exclusions in the .NET CLI after the OpenAPI pipeline work.

## Learnings

### Source Layout Migration Closeout (2026-05-01)
- The layout move succeeded because the contract was preserved at the repo root: `Cargo.toml` stayed the Rust entrypoint, `.NET` commands still run from the root while targeting `src\dotnet\HttpGenerator.sln`, and host tooling still resolves repo-root `target\debug` / `target\release` outputs.
- Path fixes had to cover more than manifests: moved Rust crates needed deeper fixture-relative paths, compatibility-runner references had to target `src\dotnet`, and `src\dotnet\HttpGenerator.VSIX\HttpGeneratorCli.cs` needed one extra parent climb to keep development-time probing correct.
- Bishop's workflow retargeting and Hudson's docs sweep closed the tester/docs surfaces in parallel; Ripley's final gate confirmed no active build/runtime surfaces still depended on `crates\` or `legacy\`.
- Root validation that passed after the move: `cargo test`, `dotnet build src\dotnet\HttpGenerator.sln -c Release`, `dotnet test src\dotnet\HttpGenerator.sln -c Release`, `test\smoke-tests.ps1`, and `dotnet run --project src\dotnet\HttpGenerator\HttpGenerator.csproj -- test\OpenAPI\v3.0\petstore.json --output .\artifacts\http-out --no-logging`.
- Remaining known issue is pre-existing and unrelated to the move: VS Code packaging via `vsce --target` still requires `engines.vscode >= 1.61` while `package.json` declares `^1.50.0`.
- Session directive: all spawned agents used GPT-5.4 for this session only.
