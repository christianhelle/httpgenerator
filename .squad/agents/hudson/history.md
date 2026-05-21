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

### docs.rs information architecture pass (2026-05-21T14:35:15.308+02:00)
- `httpgenerator-core` currently exposes a broad public Rust API with effectively no inline rustdoc in `src\rust\core\src`, so docs.rs needs a workflow guide at the crate root plus module-level narratives instead of item docs alone.
- The preferred docs.rs shape for this crate is guide-and-reference: crate root explains the library purpose and the load/normalize/generate flow; `openapi` explains ingest, inspection, typed loading, and normalization; `normalized` explains the stable generator-ready model; `generator` and `model` explain output contracts and settings; helper modules stay concise and task-oriented.
- Examples add the most value on boundary APIs such as `generate_http_files`, `GeneratorSettings`, `load_and_normalize_document`, `load_document`, `RawOpenApiDocument`, and helper functions with simple deterministic inputs.
- User/plan constraints for this pass: keep it docs-only, use mixed runnable and `no_run` examples, keep documented behavior aligned with current public APIs, and avoid fragmenting docs.rs with isolated helper trivia.
- Key first-batch files for rustdoc planning are `src\rust\core\src\lib.rs`, `src\rust\core\src\generator\mod.rs`, `src\rust\core\src\generator\modes.rs`, `src\rust\core\src\model\mod.rs`, `src\rust\core\src\model\settings.rs`, `src\rust\core\src\model\result.rs`, `src\rust\core\src\openapi\mod.rs`, `src\rust\core\src\normalized\mod.rs`, and the helper modules at the crate root.

### docs.rs structure guidance completion (2026-05-21T13:00:01Z)
- Hudson completed docs.rs guide-and-reference structure guidance as decision artifact: `2026-05-21T14:35:15.308+02:00: docs.rs structure for httpgenerator-core` (now in decisions.md).
- Structure emphasizes workflow-based navigation: crate root → pipeline modules → stable model → output contract → helpers.
- Ready for implementation by Hicks on normalized and OpenAPI documentation.

### OpenAPI reference copy guidance (2026-05-21T15:00:01.518+02:00)
- Hudson drafted docs.rs-oriented copy guidance for the remaining `openapi` reference pages: raw loading, inspection, typed/version detection, source/format classification, and errors.
- Decision artifact written to `.squad\decisions\inbox\hudson-openapi-reference-copy.md` so Hicks can implement module docs, examples, and reference-only pages without repeating the `openapi` overview.
- Recommended implementation order is module overviews first, then high-value examples on boundary APIs, then concise field/variant docs on structs and error enums.
