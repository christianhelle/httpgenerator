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

### crates.io Validation Sweep (2026-05-05)
- The expected validation matrix on the Rust-first branch is green for `cargo test`, `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release`, `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release`, and `test\smoke-tests.ps1`.
- `test\smoke-tests.ps1` must anchor execution to `$PSScriptRoot` so the root entrypoint actually exercises `test\OpenAPI\**` instead of silently skipping the fixture matrix.
- Pre-publish Cargo validation needs two passes when the branch is intentionally dirty: first record the clean-worktree failure, then rerun with `--allow-dirty` to isolate real packaging issues from the expected CI/version-injection dirtiness gate.
- `httpgenerator-core` currently passes both `cargo package --allow-dirty` and `cargo publish --dry-run --allow-dirty`.
- `httpgenerator-openapi` and `httpgenerator` currently fail publish-style validation for the expected pre-publish reason: Cargo tries to resolve `httpgenerator-core` from crates.io during package preparation, and that crate/version is not published yet.
- Key validation-facing files for this area: root `Cargo.toml`, per-crate manifests under `src\rust\httpgenerator-*\Cargo.toml`, `.github\workflows\build.yml`, `.github\workflows\release-template.yml`, and `test\smoke-tests.ps1`.

### Team Closeout — crates.io publishing (2026-05-05)
- Ripley signed off on the crates.io path as release-ready once Hicks' metadata/workflow changes and Hudson's docs updates aligned with the validation evidence.
- The lasting tester-facing rule is to separate expected pre-publish dependency-order failures from true regressions and keep smoke execution rooted at `$PSScriptRoot`.
