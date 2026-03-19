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
