---
name: "cli-patch-refresh"
description: "Safely implement CLI framework patch-level NuGet bumps in HTTP File Generator"
domain: "dependencies"
confidence: "high"
source: "issue #329 / deps-003"
---

## Context

Use this pattern for direct patch-level NuGet upgrades that primarily affect the CLI entrypoint, such as `Spectre.Console.Cli`.

## Patterns

### Scope

- Keep the code change limited to the exact `<PackageReference>` in `src\HttpGenerator\HttpGenerator.csproj` when the upgrade only affects the CLI app.
- Avoid bundling other dependency bumps or refactors into the same PR.

### Validation

- Run the standard solution pipeline after the bump:
  - `dotnet restore HttpGenerator.sln`
  - `dotnet build HttpGenerator.sln --configuration Release`
  - `dotnet test HttpGenerator.sln --configuration Release`
- Run the CLI help smoke check:
  - `dotnet run --project src\HttpGenerator\HttpGenerator.csproj -- --help`
- Run the local generator regression check against `test\OpenAPI\v3.0\petstore.json` and confirm 19 generated `.http` files.

### Workflow

- Use a dedicated feature branch and worktree for the dependency issue.
- Keep the commit small and dependency-focused so CLI behavior changes are easy to spot in review.

## Anti-Patterns

- Do not skip the `--help` smoke check for CLI framework updates.
- Do not assume a patch-level bump is safe without confirming the generator still works on the local petstore fixture.
