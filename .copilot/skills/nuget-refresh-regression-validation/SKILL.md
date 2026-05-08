# Skill: NuGet Refresh Regression Validation

**Owner:** Bishop (Tester)  
**Context:** HTTP File Generator dependency refreshes, especially when OpenAPI packages move independently of test packages  
**Last Updated:** 2026-03-20

---

## What This Skill Does

Builds a safe, staged validation plan for repo-wide NuGet updates while preserving a known-good assertion stack (`FluentAssertions` pinned). It helps split refreshes into reviewable PRs and maps each package bucket to the cheapest tests that can catch regressions early.

---

## When to Use

- Planning a Renovate sweep across the repo
- Reviewing a dependency-update PR that touches `Microsoft.OpenApi*`
- Deciding whether to batch or split NuGet refreshes
- Building a pre-merge or pre-release regression matrix

---

## Core Principle

**Do not treat all NuGet updates as equal risk.**  
For this repo, `Microsoft.OpenApi*` is a code-change migration risk; test tooling and metadata packages are mostly verification risk; `FluentAssertions` should stay pinned unless the team explicitly wants assertion churn.

---

## Risk Buckets

### Bucket A — Breaking Parser/Model Migrations
**Packages:** `Microsoft.OpenApi`, `Microsoft.OpenApi.Readers`, `Microsoft.OpenApi.OData`

**Why this is high risk:**
- `HttpGenerator.Core` relies on the current 1.x reader/model API
- `OpenApiDocumentFactory` parses specs directly
- `OpenApiValidator` depends on `OpenApiReaderSettings`, `OpenApiDiagnostic`, `OpenApiWalker`, and visitor APIs
- `HttpFileGenerator` depends on current `OpenApiDocument`, `OpenApiOperation`, `OpenApiParameter`, and `OpenApiSchema` behavior

**Rule:** Put these updates in a dedicated PR.

### Bucket B — CLI Surface Changes
**Packages:** `Spectre.Console.Cli`

**Why this is medium risk:**
- `GenerateCommand`, `Settings`, and `Program` depend on Spectre command binding and `AsyncCommand<T>` behavior
- Prior changelog history already includes a Spectre signature regression/revert

**Rule:** Allow only patch-level bumps in mixed PRs; anything else gets a focused validation pass.

### Bucket C — Test Infrastructure
**Packages:** `Atc.Test`, `Microsoft.NET.Test.Sdk`, `xunit`, `xunit.runner.visualstudio`, `coverlet.collector`

**Why this is medium/low risk:**
- Mostly affects discovery/execution rather than product behavior
- Still needs a full test pass because the project relies on parameterized theories and auto-data attributes

**Rule:** Safe to group together if `FluentAssertions` remains pinned.

### Bucket D — Build/Packaging Metadata
**Packages:** `Microsoft.SourceLink.GitHub`

**Why this is low risk:**
- Build/package only
- Should not change runtime behavior

**Rule:** Can ship in the first PR with only build + full unit validation.

---

## Baseline Commands

Run these before touching packages:

```powershell
dotnet restore HttpGenerator.sln
dotnet build HttpGenerator.sln --configuration Release
dotnet test HttpGenerator.sln --configuration Release --no-build --logger "console;verbosity=minimal"
Set-Location test; .\smoke-tests.ps1 -Parallel:$false
```

### Repo-Specific Baseline Expectations

- Unit baseline: **204 passing tests**
- Quick CLI spot checks:
  - `test/OpenAPI/v3.0/petstore.json` → **19 files** in default mode
  - `--output-type OneFile` → **1 file**
  - custom headers + IntelliJ tests should be present in generated output
- `test/OpenAPI/v3.1/webhook-example.json --skip-validation` can currently succeed while generating **0 files** in default mode

---

## Per-Issue Validation Map

### If only build/metadata packages changed
Run:

```powershell
dotnet build HttpGenerator.sln --configuration Release
dotnet test HttpGenerator.sln --configuration Release --no-build --logger "console;verbosity=minimal"
```

### If Spectre.Console.Cli changed
Run:

```powershell
dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- --help
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~GenerateCommandTests
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~SwaggerPetstoreTests
```

### If test infrastructure changed
Run:

```powershell
dotnet test HttpGenerator.sln --configuration Release --no-build --logger "console;verbosity=minimal"
```

### If Microsoft.OpenApi* changed
Run in this order:

```powershell
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~OpenApiDocumentFactoryTests
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~OpenApiValidatorTests
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~NullParametersTests
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~PathLevelParametersTests
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~QueryParametersTests
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~SwaggerPetstoreTests
dotnet test HttpGenerator.sln --configuration Release --no-build --filter FullyQualifiedName~GenerateCommandTests
Set-Location test; .\smoke-tests.ps1 -Parallel:$false
```

---

## High-Value Test Files

| Purpose | File |
|---|---|
| Parse local/remote specs | `src/HttpGenerator.Tests/OpenApiDocumentFactoryTests.cs` |
| Validate diagnostics/stats | `src/HttpGenerator.Tests/OpenApiValidatorTests.cs` |
| End-to-end CLI command behavior | `src/HttpGenerator.Tests/GenerateCommandTests.cs` |
| Broad generation coverage | `src/HttpGenerator.Tests/SwaggerPetstoreTests.cs` |
| Null parameter regression | `src/HttpGenerator.Tests/NullParametersTests.cs` |
| Path-level parameter merge regression | `src/HttpGenerator.Tests/PathLevelParametersTests.cs` |
| Query-string regression | `src/HttpGenerator.Tests/QueryParametersTests.cs` |
| Smoke matrix driver | `test/smoke-tests.ps1` |

---

## Known Noise / Caveats

- `test/smoke-tests.sh` is not the best Windows entry point; Bash can trip over CRLF and `pwsh` lookup, so use `test\smoke-tests.ps1` directly when testing on Windows.
- Remote URL tests are legitimate but noisy in restricted environments:
  - `demo.netbox.dev`
  - GitHub raw content URLs
  - third-party swagger endpoints such as the GZIP IntelliHR spec
- The v3.1 smoke matrix is intentionally incomplete:
  - `non-oauth-scopes.*` is commented out in `test/smoke-tests.ps1`
  - webhook-only fixtures are better for “does not crash” than for validating generated request content

## Hosted Gate Review

- **Do not stop at local green runs** for `Microsoft.OpenApi*` migration PRs. Always inspect hosted PR gates (`gh pr checks`) before approving.
- If a refactor rewrites legacy transport code, SonarCloud can classify long-standing insecure behavior as **new code**. Example: PR #343 surfaced a new high-severity alert when `src/HttpGenerator.Core/OpenApiDocumentFactory.cs` carried forward `ServerCertificateCustomValidationCallback = (message, cert, chain, errors) => true`.
- Treat **new high-severity security alerts** on changed code as merge blockers, even if the behavior existed before the refactor.
- Treat a **Codecov diff-coverage miss** as a sign that targeted regression tests are still missing around the new branches. Either raise coverage or narrow the PR before merge.

---

## PR Splitting Template

1. **PR 1:** Low-risk metadata/build refresh  
   Validate build + full unit suite.
2. **PR 2:** Test infrastructure refresh with `FluentAssertions` pinned  
   Validate full unit suite again.
3. **PR 3:** Dedicated `Microsoft.OpenApi*` migration  
   Validate targeted parser/generator tests first, then full suite, then smoke.
4. **PR 4:** Final rollup confidence pass  
   Re-run the whole matrix and document any environmental-only failures.

---

## Anti-Patterns

- ❌ Don’t batch `Microsoft.OpenApi*` 1.x → 3.x with routine dependency bumps
- ❌ Don’t use remote URL tests as the first signal for parser regressions
- ❌ Don’t unpin `FluentAssertions` in the same PR unless assertion churn is the explicit goal
- ❌ Don’t treat webhook-only v3.1 smoke output as proof of content correctness
