# Bishop — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** xUnit, FluentAssertions, .NET 8.0

**Test project:** `src/HttpGenerator.Tests/`
**Test fixtures:** `test/OpenAPI/v2.0/`, `test/OpenAPI/v3.0/`, `test/OpenAPI/v3.1/`
**Smoke tests:** `test/smoke-tests.sh` (~4.5 minutes — never cancel)

**Run tests:**
- Unit: `dotnet test HttpGenerator.sln --configuration Release`
- Smoke: `cd test && ./smoke-tests.sh`

**Known issues:**
- 1 unit test fails due to network restriction (external URL test) — expected, not a bug
- OpenAPI v3.1 tests require `--skip-validation` flag
- `make test` fails due to directory name collision — use `make -B test`

## Learnings

### Test Fixture Format for Path-Level Parameters (2025-01-17)

Created regression tests for issue #312 (path-level parameter merging) in PR #320:
- **Test file:** `src/HttpGenerator.Tests/PathLevelParametersTests.cs`
- **Fixture:** `src/HttpGenerator.Tests/Resources/V3/PathLevelParameters.json`
- **Pattern:** Use GitHub API-style paths (`/repos/{owner}/{repo}/issues`) to test path-level params
- **Key insight:** Path-level parameters (defined in `OpenApiPathItem.parameters`) are correctly merged into operation URLs as template variables (`{{owner}}`), but do NOT get variable definitions (@owner = str). Only operation-level query/header params get variable definitions.
- **Test coverage:** 13 tests covering all output types, parameter inheritance, override behavior, and mixed parameter sources
- **PR:** https://github.com/christianhelle/httpgenerator/pull/320

### Operation-Qualified Variable Names in OneFile Mode (2025-03-19)

Fixed 4 failing tests in `PathLevelParametersTests.cs` that had incorrect expectations for `OneFile` output mode:
- **Root cause:** In `OneFile` and `OneFilePerTag` modes, `GetParameterName()` in `HttpFileGenerator.cs` prefixes variable names with the operation name (e.g., `{{GetListIssues_owner}}`, `{{PostCreateIssue_repo}}`) to avoid collisions when multiple requests in the same file share parameter names.
- **This is INTENTIONAL behavior, not a bug** — required to prevent variable name conflicts across different operations in the same .http file
- **OneRequestPerFile mode:** Uses unqualified names (`{{owner}}`, `{{repo}}`) since each request is in its own file with no collision risk
- **Fixed tests:** Updated assertions in 4 tests to expect operation-qualified names in OneFile mode
- **Test results:** All 13 PathLevelParametersTests pass, full suite: 184 tests pass
- **PR:** https://github.com/christianhelle/httpgenerator/pull/323

### Atc.Test v2 upgrade with pinned FluentAssertions (2026-03-20)

- `Atc.Test` `2.0.17` keeps working in `src/HttpGenerator.Tests/HttpGenerator.Tests.csproj` while the repo pins `FluentAssertions` at `7.2.0`, but the test project must suppress `NU1605` because upstream now asks for `FluentAssertions >= 7.2.1`.
- The package is now xUnit v3-only, so `src/HttpGenerator.Tests/HttpGenerator.Tests.csproj` must reference `xunit.v3`; current `origin/main` already carries the matching project settings for that move.
- `src/HttpGenerator.Tests/GenerateCommandTests.cs` must pass `TestContext.Current.CancellationToken` into `GenerateCommand.ExecuteAsync(...)` to satisfy xUnit analyzer rule `xUnit1051`.
- Validation signal for the refreshed stack stays `204/204` green via `dotnet restore HttpGenerator.sln`, `dotnet build HttpGenerator.sln --configuration Release`, and `dotnet test HttpGenerator.sln --configuration Release`.

