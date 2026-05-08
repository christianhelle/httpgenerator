---
name: "vsix-sdk-headless-validation"
description: "Validate VSIX SDK dependency bumps against a headless baseline in HTTP File Generator"
domain: "dependencies"
confidence: "high"
source: "issue #335 / deps-009"
---

## Context

Use this pattern when a change only bumps VSIX-related NuGet packages in `src\HttpGenerator.VSIX\HttpGenerator.VSIX.csproj` and full Visual Studio validation may be unavailable.

## Patterns

### Scope

- Keep the implementation limited to the exact VSIX `<PackageReference>` entries being refreshed.
- For this repo, the primary file is `src\HttpGenerator.VSIX\HttpGenerator.VSIX.csproj`.

### Validation

- Run:
  - `dotnet restore src\VSIX.sln`
  - `dotnet build src\VSIX.sln --configuration Release --no-restore`
- If the build fails in the headless environment, create a clean detached worktree from `origin/main` and rerun the same two commands there.
- If restore succeeds and the build fails with the same missing Visual Studio SDK type/reference errors in both places, record the result as an environment blocker rather than a package regression.

### Failure Signature

- Common blocked-build errors in this environment mention missing Visual Studio symbols such as:
  - `ProvideCodeBase`
  - `ProvideMenuResource`
  - `AsyncPackage`
  - `OleMenuCommandService`
  - `ServiceProgressData`

### Workflow

- Use a dedicated feature branch and worktree for the issue.
- Keep the PR tightly scoped to the VSIX project plus required squad artifacts.
- Call out both the successful restore and the blocked build in the PR body.

## Anti-Patterns

- Do not claim the VSIX project is fully validated when only restore passed.
- Do not assume the package bump caused the build failure without comparing to a clean `origin/main` baseline.
- Do not expand validation to unrelated solution areas for a VSIX-only dependency refresh.
