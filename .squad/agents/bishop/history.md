# Bishop — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** xUnit v3, FluentAssertions, .NET 8.0, PowerShell smoke tests

**Current validation roots:**
- .NET solution: `src\dotnet\HttpGenerator.sln`
- Test project: `src\dotnet\HttpGenerator.Tests\`
- OpenAPI fixtures: `test\OpenAPI\v2.0\`, `test\OpenAPI\v3.0\`, `test\OpenAPI\v3.1\`
- Smoke tests: `test\smoke-tests.ps1`

**Standard validation commands:**
- `dotnet test src\dotnet\HttpGenerator.sln --configuration Release`
- `test\smoke-tests.ps1`

**Historical summary:**
- Established the repo-wide dependency-refresh validation strategy, including staged regression gates and final matrix coverage.
- Helped pull the test stack forward to xUnit v3 during the Atc.Test 2.x migration, then expanded coverage with targeted edge-case and smoke-test additions.
- Closed out CLI output parity validation for TTY vs redirected output, including split-stream smoke coverage and host-surface checks.

## Learnings

### Source Layout Migration Closeout (2026-05-01)
- Validation workflows are path-sensitive even when the repo root stays the user-facing entrypoint. For this migration, triggers and command targets had to move from `crates/**` and `legacy/**` to `src/rust/**` and `src/dotnet/**`.
- The validation matrix shape did **not** change. The move required scope retargeting, not new test permutations.
- Hicks closed the implementation-owned rewrites (workspace members, moved solution paths, `publish-manifest.json`, and VSIX runtime probing), Hudson updated the docs surfaces, and Ripley approved after the full root-entrypoint validation set passed.
- Validation confirmed in this session: `cargo test`, `dotnet build src\dotnet\HttpGenerator.sln -c Release`, `dotnet test src\dotnet\HttpGenerator.sln -c Release`, `test\smoke-tests.ps1`, and `npm ci` + `npm run compile` in `src\VSCode`.
- Non-blocking follow-up from review: stale old-path references may still exist in some `.squad\` notes/history, so internal guidance should keep converging on the canonical layout.
- Session directive: all spawned agents used GPT-5.4 for this session only.
