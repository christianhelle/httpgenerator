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

### Facade contract test slice (2026-05-08T13:19:39.287+02:00)
- For the first Rust module-refactor safety slice, the smallest durable guard is an integration test that imports `httpgenerator_core::{generator, model, normalized, openapi}` directly and exercises those facades together.
- Existing Rust CLI binary-contract coverage in `src\rust\cli\tests\help_contract.rs` already pins help, version, no-args, stderr warning, and validation-guidance behavior, so this slice should avoid widening that matrix unless a concrete public gap appears.
- New facade-stability coverage lives in `src\rust\core\tests\facade_contracts.rs` and is intended to survive internal file moves behind the same public module paths.

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

### Rust module restructure validation sweep (2026-05-08T13:19:39.287+02:00)
- Current protection layers for Rust-only module splits are `cargo test --workspace`, `dotnet build src\dotnet\HttpGenerator.slnx -c Release`, `dotnet test src\dotnet\HttpGenerator.slnx -c Release`, and `test\smoke-tests.ps1`; this baseline was green during the investigation.
- The highest-risk public seams are the Rust facades in `src\rust\core\src\lib.rs`, `src\rust\core\src\openapi\mod.rs`, and `src\rust\cli\src\lib.rs`, plus CLI entrypoint wiring in `src\rust\cli\src\main.rs`.
- `src\rust\cli\tests\differential_petstore.rs` and `test\smoke-tests.ps1` intentionally mirror the same output-mode and option permutations, so refactors that move execution or argument wiring should update both together.
- New tester-owned coverage for future bounded module splits should emphasize facade/re-export contract checks and seam-local unit tests for extracted folders instead of widening the fixture matrix when behavior is unchanged.

### VS Code packaged Rust host final approval (2026-05-13T21:06:43Z)
- Final verdict: approved the revised artifact after Hudson's packaging/build revision.
- Approval basis stayed narrow: win32-x64 now ships the matching x64 Rust binary, win32-arm64 fails fast locally instead of bundling a host-built binary, CI exercises real win32-arm64 packaging under the matching MSVC environment, and resolver/fail-fast explicit-path behavior remained intact.
- Only residual coverage gap is manual: install the produced VSIX on native x64 and ARM64 VS Code hosts and smoke the Command Palette plus Explorer menu generation flows end-to-end.


### VSIX async first-slice validation closeout (2026-05-18T09:53:21Z)
- Rejected the first VSIX redesign wave until the approved notification contract and placement seam were actually present.
- Approved the revised state after dotnet build src\dotnet\VSIX.slnx --configuration Release, dotnet build src\dotnet\HttpGenerator.slnx --configuration Release, and dotnet test src\dotnet\HttpGenerator.slnx --configuration Release passed again.

