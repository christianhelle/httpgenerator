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

