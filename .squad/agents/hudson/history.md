# Hudson — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle

CLI tool + VS extensions for generating `.http` files from OpenAPI specs.

**Primary documentation surfaces:**
- `README.md`
- `CONTRIBUTING.md`
- `.github\copilot-instructions.md`
- `docs\README.md`
- `docs\index.html`
- `docs\Marketplace.md`
- `src\dotnet\HttpGenerator\README.md`

**Distribution channels:**
- NuGet global tool (`httpgenerator`)
- NuGet class library (`HttpGenerator.Core`)
- Visual Studio / VS Code extensions

## Learnings

### Historical Summary
- Release documentation work in this repo usually spans three channels (NuGet CLI, core library, IDE extensions), so layout or dependency changes must be checked across both product docs and packaging guidance.
- Rust CLI output parity closeout established a useful pattern: README images can show the rich interactive experience while text examples should stay plain and redirect-safe.

### Source Layout Docs Sweep (2026-05-01)
- Canonical source roots are now `src\rust`, `src\dotnet`, and existing `src\VSCode`, but contributor commands still start at the repository root.
- The main docs surfaces that needed updates for the move were `README.md`, `CONTRIBUTING.md`, `.github\copilot-instructions.md`, `docs\README.md`, `docs\index.html`, `docs\Marketplace.md`, and `src\dotnet\HttpGenerator\README.md`.
- Cross-agent closeout: Hicks preserved repo-root entrypoints and runtime lookup, Bishop confirmed validation/release matrix continuity, and Ripley approved the migration once the path-bearing active surfaces were clean.
- Session directive: all spawned agents used GPT-5.4 for this session only.

### Crates.io Publishing Pattern Analysis (2026-05-DD)
- **Objective**: Extract patterns from `christianhelle/httprunner` and `christianhelle/azdocli` to guide httpgenerator crates.io publishing.
- **Key findings**:
  - Both repos use single `VERSION` env var in GitHub Actions workflow, injected via PowerShell to `Cargo.toml` before publish
  - Both use `cargo publish --allow-dirty --token ${{ secrets.CRATES_TOKEN }}` for authentication
  - httprunner (multi-crate) publishes core + CLI sequentially in same job; azdocli (single crate) publishes in standalone job
  - httprunner updates per-crate README.md with version; azdocli does not
  - No reference repos have .NET co-release; Rust CLI is isolated from legacy distribution
- **Httpgenerator applicability**:
  - ✅ Version injection pattern works identically (same workspace + per-crate Cargo.toml structure as httprunner)
  - ✅ Multi-crate sequential publish fits 3-crate workspace (core → openapi → cli dependency chain)
  - ✅ Token-based auth via CRATES_TOKEN secret is standard practice
  - ⚠️ Decision needed: Publish httpgenerator-openapi and httpgenerator-core as public crates, or CLI-only?
  - ⚠️ Version source strategy: httprunner uses `0.9.${{ github.run_number }}` (incremental), azdocli hardcodes constant
- **Documentation checklist** (for future release workflow):
  - Add CONTRIBUTING.md section on crates.io prerequisites (token setup, permissions)
  - Link to crates.io package pages from README once released
  - Consider adding per-crate README.md files if libraries are intended for downstream use
- The reference-repo pattern study (httprunner + azdocli) was merged into `.squad\decisions.md` during the 2026-05-05 crates.io publishing closeout.

### Crates.io Install Guidance Sweep (2026-05-05)
- Ripley's packaging gate makes three crates public (`httpgenerator`, `httpgenerator-core`, `httpgenerator-openapi`) and keeps `src\rust\httpgenerator-compat` private with `publish = false`; docs should mirror that split exactly.
- User-facing install guidance now needs a three-lane explanation: crates.io for Rust-native installs (`cargo install httpgenerator` for published releases), GitHub Releases for prebuilt standalone CLI archives, and Marketplace / VSIX packages for editor extensions that bundle native binaries.
- The main surfaces for this message are `README.md`, `docs\index.html`, and `docs\README.md`; extension sections should also remind users that Cargo-installed binaries can be reused through `PATH`, `HTTPGENERATOR_PATH`, or `http-file-generator.executablePath`.

### Team Closeout — crates.io publishing (2026-05-05)
- Ripley approved the public/private crate split and release-readiness gate, while Hicks encoded ordered crates.io publication in the reusable workflow.
- Bishop's validation closeout means the docs can safely describe crates.io as first-class while still calling out the expected pre-publish dry-run limitation.

### VS Code Rust-host Docs Contract (2026-05-13T21:06:43Z)
- Keep the VS Code extension story aligned across `src\vscode\README.md`, `README.md`, `CONTRIBUTING.md`, `docs\README.md`, `docs\index.html`, and `.github\copilot-instructions.md`.
- Document the extension as a platform-targeted bundled `.vsix` flow with executable resolution `http-file-generator.executablePath` → bundled binary → repo-root `target\debug` / `target\release` → `PATH`.
- Do not point extension users at `.NET Tool` or crates.io installs; those are separate product channels from the bundled VS Code runtime.
- Prefer the canonical lowercase `src\vscode` path when describing the current extension host.

### VS Code packaged Rust host final approval (2026-05-13T21:06:43Z)
- Bishop approved the revised artifact after Hudson's packaging/build revision.
- The lasting packaging/docs contract is that the packaged binary must match the requested VS Code target, with local win32-arm64 packaging failing fast rather than silently reusing a host-built executable.
- Remaining closeout is manual-only: install the produced VSIX on native x64 and ARM64 VS Code hosts and smoke Command Palette and Explorer menu generation flows.

