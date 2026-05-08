# Hicks — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** .NET 8.0 CLI, C#, Rust workspace, Microsoft.OpenApi, Spectre.Console.Cli, xUnit, FluentAssertions

**Canonical product roots:**
- Rust workspace members: `src\rust\`
- .NET solution and apps: `src\dotnet\`
- VS Code extension: `src\VSCode\`

**Key implementation files:**
- `src\dotnet\HttpGenerator\GenerateCommand.cs`
- `src\dotnet\HttpGenerator.Core\HttpFileGenerator.cs`
- `src\dotnet\HttpGenerator.Core\OpenApiDocumentFactory.cs`
- `src\dotnet\HttpGenerator.VSIX\HttpGeneratorCli.cs`
- `src\rust\httpgenerator-cli\src\lib.rs`
- `src\rust\httpgenerator-openapi\src\inspect.rs`

**Build and validate:**
- `cargo test`
- `dotnet build src\dotnet\HttpGenerator.sln --configuration Release`
- `dotnet test src\dotnet\HttpGenerator.sln --configuration Release`
- `dotnet run --project src\dotnet\HttpGenerator\HttpGenerator.csproj -- test\OpenAPI\v3.0\petstore.json --output .\artifacts\http-out --no-logging`

**Historical summary:**
- Delivered small-scope dependency refreshes (SourceLink, Spectre.Console.Cli) and the minimal xUnit v3 migration needed by Atc.Test 2.x.
- Investigated and implemented host-sensitive CLI output parity work across Rust, VS Code, and VSIX surfaces.
- Cleaned up dead code and coverage exclusions in the .NET CLI after the OpenAPI pipeline work.

## Learnings

### Rust modularization seams vs httprunner (2026-05-08T13:19:39.287+02:00)
- Current Rust crate roots are still mostly flat facades: `src\rust\core\src\lib.rs` publicly exposes individual helper files plus the `openapi` subtree, while `src\rust\cli\src\lib.rs` only exposes `args` and `telemetry` as modules and re-exports execution/observer/error types.
- The highest-value split points are the large mixed-responsibility files: `src\rust\core\src\openapi\normalize.rs`, `src\rust\core\src\generator.rs`, `src\rust\core\src\openapi\inspect.rs`, `src\rust\cli\src\execution.rs`, `src\rust\cli\src\ui.rs`, and `src\rust\cli\src\telemetry.rs`.
- A safe httprunner-style refactor should keep published namespace paths stable by turning file modules into same-named directory modules (`generator`, `model`, `normalized`, `args`, `telemetry`, `openapi`) and re-exporting the existing crate-root API from thin `mod.rs` facades.
- Baseline validation before any restructure was green for `cargo test --workspace`, `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release`, and `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release`; `test\smoke-tests.ps1` currently fails on the existing release-binary copy/help-output checks, so modularization work must compare against that pre-existing smoke baseline instead of assuming a new regression.

### Release Workflow crates.io Path (2026-05-05)
- Stable crates.io publication now lives in `.github\workflows\release-template.yml` behind a reusable `publish-crates` boolean input, with `.github\workflows\release.yml` opting in and preview callers staying artifact-only by default.
- Shared Rust release version injection is handled by `.github\scripts\set-rust-workspace-version.py`, which rewrites every `version = "0.1.0"` anchor in the root `Cargo.toml` so `[workspace.package]` and publish-safe sibling dependency pins stay aligned.
- crates.io sequencing is `httpgenerator-core` → `httpgenerator-openapi` → `httpgenerator`, and `.github\scripts\check-crates-io-version.py` polls crates.io between publishes so downstream crates wait for the newly published dependency version instead of relying on a fixed sleep.
- CI publish-readiness coverage now includes `.github\workflows\release.yml`, `.github\workflows\release-template.yml`, and `.github\scripts\**`; `build.yml` validates the version-injection helper with a core `cargo publish --dry-run` plus downstream `cargo check`.

### crates.io Metadata & Workspace Wiring (2026-05-05)
- Ripley's packaging gate is now the governing rule for Rust publish work: keep Edition 2024, `rust-version = "1.85"`, publish `httpgenerator`, `httpgenerator-core`, and `httpgenerator-openapi`, and keep `src\rust\httpgenerator-compat` private with `publish = false`.
- Publish-safe sibling crate wiring belongs in the root workspace manifest: `Cargo.toml` should carry `{ version = "0.1.0", path = "src/rust/..." }` for public internal dependencies so local workspace builds still work while packaged crates remain valid for crates.io.
- Public crate metadata now lives in the per-crate manifests and crate-local README files at `src\rust\httpgenerator-cli\README.md`, `src\rust\httpgenerator-core\README.md`, and `src\rust\httpgenerator-openapi\README.md`.
- Release-time version injection must update every `version = "0.1.0"` anchor in the root `Cargo.toml`, not just `[workspace.package].version`, otherwise internal dependency pins drift once publishable crates stop being path-only.

### Source Layout Migration Closeout (2026-05-01)
- The layout move succeeded because the contract was preserved at the repo root: `Cargo.toml` stayed the Rust entrypoint, `.NET` commands still run from the root while targeting `src\dotnet\HttpGenerator.sln`, and host tooling still resolves repo-root `target\debug` / `target\release` outputs.
- Path fixes had to cover more than manifests: moved Rust crates needed deeper fixture-relative paths, compatibility-runner references had to target `src\dotnet`, and `src\dotnet\HttpGenerator.VSIX\HttpGeneratorCli.cs` needed one extra parent climb to keep development-time probing correct.
- Bishop's workflow retargeting and Hudson's docs sweep closed the tester/docs surfaces in parallel; Ripley's final gate confirmed no active build/runtime surfaces still depended on `crates\` or `legacy\`.
- Root validation that passed after the move: `cargo test`, `dotnet build src\dotnet\HttpGenerator.sln -c Release`, `dotnet test src\dotnet\HttpGenerator.sln -c Release`, `test\smoke-tests.ps1`, and `dotnet run --project src\dotnet\HttpGenerator\HttpGenerator.csproj -- test\OpenAPI\v3.0\petstore.json --output .\artifacts\http-out --no-logging`.
- Remaining known issue is pre-existing and unrelated to the move: VS Code packaging via `vsce --target` still requires `engines.vscode >= 1.61` while `package.json` declares `^1.50.0`.
- Session directive: all spawned agents used GPT-5.4 for this session only.

### Crates.io Publishing Readiness Assessment (2025-01-15)
- **Publishable crates identified:** httpgenerator-core (normalization + rendering) and httpgenerator-openapi (OpenAPI parsing layer) have clean public APIs and zero workspace/CLI-specific dependencies; both are fully standalone libraries suitable for crates.io.
- **Non-publishable:** httpgenerator-cli (binary entry point with Azure auth, telemetry, CLI args) and httpgenerator-compat (internal differential testing harness) must have `publish = false`.
- **Critical blocker:** Root `Cargo.toml` has `edition = "2024"` which is invalid (Rust editions are 2015, 2018, 2021 only). Will fail crates.io validation. Must be changed to `edition = "2021"`.
- **Missing metadata:** All crates lack description, authors, documentation, homepage, categories, keywords, readme, rust-version fields. Per-crate README.md files don't exist.
- **Workspace compatibility:** Root workspace setup is compatible with crates.io publishing (resolver = "2", workspace.package inheritance, path-based members all supported). Dry-run validation must be added to CI/CD before actual publish.
- **Release workflow gap:** Current release-template.yml builds CLI archives and VSIX packages but has no `cargo publish` steps. Must add new `publish-crates` job that publishes core before openapi (due to openapi dependency on core) with 3-5 minute propagation wait.
- **MSRV strategy:** No `rust-version` field in workspace. Recommend `rust-version = "1.70"` (mid-2023, widely available, mature). Add MSRV validation to build.yml (`cargo +1.70 build`).
- **Account setup:** Requires crates.io account + API token, which is external dependency (not in scope of analysis). Token should be added as CARGO_REGISTRY_TOKEN GitHub secret.
- **Effort estimate:** 5–6 hours total (metadata 1h, documentation 1.5h, CI/CD 1.5h, testing 0.5h, external setup 0.5h).
- The earlier crates.io readiness checklist and workflow plan were merged into `.squad\decisions.md` during the 2026-05-05 session closeout.

### Team Closeout — crates.io publishing (2026-05-05)
- Ripley's packaging gate and final approval are now the governing constraints for future Rust publish work in this repo.
- Hudson aligned the user-facing install/docs story and Bishop confirmed the release validation matrix plus the `$PSScriptRoot` smoke-test anchor.

## 2026-05-08T11:56:18Z — Scribe Session: extract-core-type-modules Decision Recorded

**From:** Scribe (Silent Logger)

**Context:** Squad session processed spawn manifest for hicks-1.

**Team Update:**
- Decision **"2026-05-08: Hicks — core type extraction"** archived to active decisions.md
- Decisions merged and deduplicated; decisions.md reduced from 22,218 → 13,967 bytes.
- Archive preserved entries older than 30 days.

**Related Decision:**
- **2026-05-08: Hicks — core type extraction**
  - Frozen public httpgenerator_core::{model, normalized} facades
  - Applied httprunner-style directory module structure
  - Cargo test -p httpgenerator-core passed

**No further action required on Hicks' part.**

## 2026-05-08T11:56:18Z — Scribe Session: extract-core-generator-modules Decision Recorded

**From:** Scribe (Silent Logger)

**Context:** Squad session processed spawn manifest for hicks-2 (extract-core-generator-modules).

**Team Update:**
- Decision **"2026-05-08: Hicks — generator extraction"** merged to active decisions.md
- Decisions inbox cleared; 1 file processed.
- Orchestration log created at `.squad/orchestration-log/2026-05-08T11-56-18Z-hicks-2.md`

**Related Decision:**
- **2026-05-08: Hicks — generator extraction**
  - Replaced flat `generator.rs` with `generator/` facade and leaf modules
  - Applied bounded shape: `mod.rs`, `modes.rs`, `render.rs`, `sample.rs`, `text.rs`, `tests.rs`
  - Public API frozen through `pub use modes::generate_http_files`
  - `cargo test -p httpgenerator-core` passed
  - Maintains httprunner-style modularization direction
