---
name: "dependency-metadata-refresh"
description: "Safely implement metadata-only NuGet dependency bumps in HTTP File Generator"
domain: "dependencies"
confidence: "high"
source: "issue #328 / deps-002"
---

## Context

Use this pattern for direct NuGet package bumps that do not require code changes, especially repo metadata or build-time packages like `Microsoft.SourceLink.GitHub`.

## Patterns

### Scope

- Keep the change limited to the exact `<PackageReference>` entries that need the version bump.
- For `Microsoft.SourceLink.GitHub`, the repo currently references it directly in:
  - `src\HttpGenerator\HttpGenerator.csproj`
  - `src\HttpGenerator.Core\HttpGenerator.Core.csproj`

### Validation

- Run the standard solution pipeline after the bump:
  - `dotnet restore HttpGenerator.sln`
  - `dotnet build HttpGenerator.sln --configuration Release`
  - `dotnet test HttpGenerator.sln --configuration Release`
- Also run the local CLI generation check against `test\OpenAPI\v3.0\petstore.json` and confirm 19 generated `.http` files.

### Workflow

- Use a dedicated feature branch and worktree for each dependency issue.
- Keep the PR small and infrastructure-only when no production code changes are required.

## Anti-Patterns

- Do not bundle unrelated package upgrades into the same PR.
- Do not skip the petstore generation check just because the package is metadata-only.
