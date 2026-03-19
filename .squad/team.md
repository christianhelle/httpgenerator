# Squad Team

> httpgenerator

## Coordinator

| Name | Role | Notes |
|------|------|-------|
| Squad | Coordinator | Routes work, enforces handoffs and reviewer gates. |

## Members

| Name    | Role         | Charter                              | Status |
|---------|--------------|--------------------------------------|--------|
| Ripley  | Lead         | .squad/agents/ripley/charter.md      | ✅ Active |
| Hicks   | Core Dev     | .squad/agents/hicks/charter.md       | ✅ Active |
| Bishop  | Tester       | .squad/agents/bishop/charter.md      | ✅ Active |
| Hudson  | DevRel/Docs  | .squad/agents/hudson/charter.md      | ✅ Active |
| Scribe  | Scribe       | .squad/agents/scribe/charter.md      | ✅ Active |
| Ralph   | Work Monitor | —                                    | ✅ Active |

## Project Context

- **Project:** HTTP File Generator (`httpgenerator`)
- **User:** Christian Helle
- **Created:** 2026-03-19
- **Stack:** .NET 8.0 CLI, C#, NSwag, Microsoft.OpenApi, Spectre.Console.Cli, xUnit, FluentAssertions
- **What it does:** Generates `.http` files from OpenAPI specifications (v2.0, v3.0, v3.1) for use with REST Client extensions in VS Code, JetBrains, and Visual Studio 2022. Also ships as a Visual Studio extension (VSIX) and VS Code extension.
- **Repo root:** C:/projects/christianhelle/httpgenerator
- **Key files:**
  - `src/HttpGenerator/GenerateCommand.cs` — CLI command
  - `src/HttpGenerator.Core/HttpFileGenerator.cs` — Core generation logic
  - `src/HttpGenerator.Core/OpenApiDocumentFactory.cs` — OpenAPI parsing
  - `src/HttpGenerator/Settings.cs` — CLI options
  - `test/OpenAPI/` — Test OpenAPI specs (v2.0, v3.0, v3.1)
