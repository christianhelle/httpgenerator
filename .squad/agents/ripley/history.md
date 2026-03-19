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

### PR Cascade Review (2026-03-19)

**Task:** Sequential review of PRs #323 (test fixes), #321 (filename dedup), #322 (JSON schemas)

**Verdicts:**
- **PR #323:** ✅ APPROVED - Test assertions correctly updated for OneFile mode; unblocks other PRs
- **PR #321:** ❌ REJECTED - Critical branch mismatch: title claims "deduplicate filenames" (#314) but code contains JSON schema handling (allOf/oneOf/anyOf). No HashSet deduplication logic found. Branch must be recreated with correct deduplication code.
- **PR #322:** ❌ REJECTED (cascading) - Cannot evaluate independently due to PR #321 failure. Both branches show identical JSON schema changes, suggesting branch contamination or rebase error.

**Key Finding:** PR #321's commit message describes filename deduplication intent but the actual code in the branch diverged—contains JSON schema logic instead. This is a critical mismatch requiring investigation into how the branch got created.

**Decision Document:** `.squad/decisions/inbox/ripley-cascade-merge-313-314-323.md`
