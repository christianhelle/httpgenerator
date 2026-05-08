# Squad Decisions

## Active Decisions
### 2026-04-08: User Directive — Small Logical Commits
**By:** Christian Helle (via Copilot CLI)
**What:** Commit changes in small logical groups for detailed progress history in this and future sessions
**Why:** User request — enables clean, reviewable git history with clear checkpoints for team accountability
**Impact:** Standing rule for all squad members; enables detailed session-to-session traceability

### 2026-04-08: CLI Output Parity — Rust Rich & Plain Dual Modes
**By:** Ripley (Lead), Hicks (Core Dev), Bishop (Tester), Hudson (DevRel/Docs)
**Status:** ✅ COMPLETE — Implementation merged, all validation passing
**What:** Implemented context-aware output rendering in Rust CLI:
- **Rich mode** (interactive terminal): Colors, emojis (🚀, ✅, 📊, 📁, 🎉, etc.), box-drawing characters, formatted tables via Spectre-inspired `comfy-table` + `console` crates
- **Plain mode** (redirected/piped stdout): Semantic text only, no ANSI codes, no special characters, single-line file listings
- **Detection:** `io::stdout().is_terminal()` respects `$TERM`, pipes, and file redirection
**Key Implementation Details:**
- Rust CLI presenter layer in `main.rs` + `ui.rs` (existing `lib.rs` execution logic unchanged)
- Help contract tests validate both modes; all passing
- VSIX host surfaces Azure diagnostics correctly (success-path warnings no longer dropped)
- VS Code extension remains compatible (TTY detection handles rich output correctly)
**Validation:** cargo test ✅, dotnet test ✅, test\smoke-tests.ps1 ✅; VSIX build deferred (known environment limitation)
**Documentation:** README already accurate—no changes needed. Correctly conveys both output modes.
**Files Updated:** crates/httpgenerator-cli/{src/ui.rs, tests/help_contract.rs}, test/smoke-tests.ps1, src/HttpGenerator.VSIX/{HttpGeneratorCli.cs, GenerateDialog.cs}
**Decision:** Approved & ready for release. Pattern established: context-aware rendering + help contract validation for future CLI work.

### 2026-05-01: Source Layout Migration to `src\rust` and `src\dotnet`
**By:** Hicks (Core Dev), Bishop (Tester), Hudson (DevRel/Docs), Ripley (Lead)
**What:** Moved Rust crates to `src\rust` and legacy .NET sources to `src\dotnet` while keeping repo-root entrypoints intact and leaving `src\VSCode` in place. Updated build, validation, release, documentation, and runtime path-bearing surfaces in one coordinated pass.
**Validation:** `cargo test`; `dotnet build src\dotnet\HttpGenerator.sln -c Release`; `dotnet test src\dotnet\HttpGenerator.sln -c Release`; `test\smoke-tests.ps1`; `npm ci` + `npm run compile` in `src\VSCode`
**Decision:** APPROVED. Validation matrix shape is unchanged. Only non-blocking follow-up was stale old-path guidance inside `.squad\` notes/history.

### 2026-05-01: Session Directive — Spawned Agents Use GPT-5.4
**By:** Christian Helle (via Copilot CLI)
**What:** All spawned agents in this session must use GPT-5.4.
**Why:** Session-only user directive for consistent agent execution.

### 2026-05-05: Session Directive — Spawned Agents Use GPT-5.5
**By:** Christian Helle (via Copilot CLI)
**What:** Have all spawned agents use GPT-5.5 for the rest of this session only.
**Why:** Session-only user directive for consistent agent execution.
**Supersedes:** 2026-05-01 session directive to use GPT-5.4.

### 2026-05-05: Crates.io Packaging Gate
**By:** Ripley (Lead)
**What:** Keep Rust Edition 2024, set shared `rust-version = "1.85"`, publish `httpgenerator`, `httpgenerator-core`, and `httpgenerator-openapi`, keep `src\rust\httpgenerator-compat` private with `publish = false`, and use GitHub Pages for homepage plus docs.rs for per-crate documentation.
**Why:** This preserves the Rust-first product direction, keeps `cargo install httpgenerator` clean, and aligns crate metadata with the canonical product/docs surfaces.

### 2026-05-05: Publish-safe Workspace Dependency Wiring
**By:** Hicks (Core Dev)
**What:** Internal public-crate dependencies must carry both `path` and `version` in the root workspace manifest, and release-time version injection must replace every `version = "0.1.0"` anchor in the root `Cargo.toml`.
**Why:** Local workspace builds stay ergonomic while packaged crates remain valid for crates.io and release-time version pins stay aligned.

### 2026-05-05: Release Workflow crates.io Publication Path
**By:** Hicks (Core Dev)
**What:** Wire crates.io publication into the reusable release workflow behind `publish-crates`, enable it for stable `release.yml`, keep preview/template callers artifact-only by default, require `CARGO_REGISTRY_TOKEN`, and publish in dependency order `httpgenerator-core` → `httpgenerator-openapi` → `httpgenerator` with crates.io visibility polling between steps.
**Why:** Stable releases should publish crates automatically without risking preview-package pushes or brittle fixed sleeps.

### 2026-05-05: crates.io Documentation Distribution Matrix
**By:** Hudson (DevRel/Docs)
**What:** Treat crates.io as a first-class Rust install/library channel, keep GitHub Releases as the source of prebuilt CLI archives, and keep VS Code / Visual Studio extensions documented as bundled-binary Marketplace / VSIX distributions.
**Why:** Users need the install channels explained as complementary instead of assuming the extensions fetch their runtime from crates.io.

### 2026-05-05: Validation Contract for crates.io Publishing
**By:** Bishop (Tester)
**What:** Treat dirty-worktree publish failures without `--allow-dirty` separately from expected downstream crates.io resolution failures, keep the smoke-test root entrypoint anchored to `$PSScriptRoot`, and preserve publish sequencing `httpgenerator-core` → `httpgenerator-openapi` → `httpgenerator`.
**Why:** Validation reporting should distinguish true regressions from the known pre-publish dependency-order limitation.

### 2026-05-05: Crates.io Publishing Release Readiness
**By:** Ripley (Lead), Hicks (Core Dev), Hudson (DevRel/Docs), Bishop (Tester)
**What:** Approved the crates.io publishing implementation as release-ready after green validation (`cargo test`, `dotnet build src\dotnet\HttpGenerator.slnx -c Release`, `dotnet test src\dotnet\HttpGenerator.slnx -c Release`, `test\smoke-tests.ps1`) and aligned metadata, workflow, and docs.
**Expected limitation:** Only `httpgenerator-core` can pass local publish-style dry-runs before first publication; `httpgenerator-openapi` and `httpgenerator` must wait for crates.io visibility of the newly published dependency version.

### 2026-05-08: Rust Modularization Direction — httprunner-style Restructure
**By:** Ripley (Lead), Hicks (Core Dev), Bishop (Tester)
**Status:** Proposed planning direction pending further team work
**What:** Adopt the `httprunner` Rust module style for future `httpgenerator` refactors:
1. Prefer bounded-context directory modules with `mod.rs` facades over adding more large flat `.rs` files.
2. Keep crate roots and executable entrypoints thin; move internal orchestration into submodules.
3. Split oversized mixed-responsibility files first (`src\rust\core\src\openapi\normalize.rs`, `src\rust\core\src\generator.rs`, `src\rust\cli\src\ui.rs`, `src\rust\cli\src\execution.rs`).
4. Preserve public crate APIs unless the user explicitly approves API reshaping.
5. Add module-local `README.md` files only for significant multi-file directories, matching the `httprunner` documentation taste.

**Target shape for `src\rust\core\src`:**
- Keep small leaf helpers flat: `base_url.rs`, `file_naming.rs`, `operation_name.rs`, `privacy.rs`, `string_extensions.rs`, `support_information.rs`
- Convert large file modules into folders:
  - `generator\mod.rs` (contains `modes.rs`, `render.rs`, `sample.rs`, `text.rs`)
  - `model\mod.rs`
  - `normalized\mod.rs`
  - `openapi\mod.rs` (contains `error.rs`, `load\mod.rs` with sub-modules, `inspect\mod.rs` with sub-modules, `normalize\mod.rs` with sub-modules)

**Target shape for `src\rust\cli\src`:**
- Keep binary entrypoint `main.rs` as thin wiring
- Convert flat feature files into folders:
  - `args\mod.rs` (contains `help.rs`, `types.rs`, `tests.rs`)
  - `execution\mod.rs` (contains `orchestrator.rs`, `validation.rs`, `authorization.rs`, `settings.rs`)
  - `telemetry\mod.rs` (contains `events.rs`, `sink.rs`, `recorder.rs`, `redaction.rs`)
  - `ui\mod.rs` (contains `presenter.rs`, `render.rs`, `format.rs`)
  - Keep smaller leaf files flat: `auth.rs`, `error.rs`, `observer.rs`, `writer.rs`

**Key constraints:**
- Preserve stable public paths such as `httpgenerator_core::openapi::*`, `httpgenerator_core::generate_http_files`, `httpgenerator_core::OutputType`, `httpgenerator_cli::args::*`, and `httpgenerator_cli::telemetry::*`.
- Move tests with their new seams instead of leaving large `*_tests.rs` files pointing back into reshaped modules.
- Treat `test\smoke-tests.ps1` failures as pre-existing baseline noise unless modularization changes the same failure signature.

**Why:** `httprunner` shows a consistent pattern of domain folders with stable re-export surfaces and internal file-level seams. `httpgenerator` already started this approach in `src\rust\core\src\openapi`, so the safest migration is to extend that pattern rather than invent a new structure.

### 2026-05-08: Facade Contract Tests — Module Restructuring Risk Coverage
**By:** Bishop (Tester)
**What:** For the first Rust module-refactor slice, freeze the public seam with `httpgenerator_core::{generator, model, normalized, openapi}` integration tests and rely on the existing CLI binary contract suite instead of adding more CLI cases.
**Why:** The approved plan already treats the binary contract as covered by help/version/no-args/stderr tests. The higher-value new risk is accidental breakage of facade module paths while files move behind `mod.rs` boundaries.
**Scope:** `src\rust\core\tests\facade_contracts.rs`
**Implementation Status:** ✅ COMPLETE — Added `src\rust\core\tests\facade_contracts.rs` with 4 integration tests validating core module facades; targeted test passed; targeted Rust test passed.

### 2026-05-08: Rust Modularization Validation Gates
**By:** Bishop (Tester)
**What:** Treat the restructure as a bounded internal refactor unless Hicks explicitly changes crate names, binary identity, or public `pub use` surfaces.
**Validation gate:**
1. `cargo test --workspace`
2. `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release`
3. `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release`
4. `test\smoke-tests.ps1`

Add `src\vscode\build.ps1` and VSIX validation only if the refactor moves executable resolution, package names, or host-facing wiring.

**Coverage guidance:**
- Add facade/re-export contract coverage for `src\rust\core\src\lib.rs`, `src\rust\core\src\openapi\mod.rs`, and `src\rust\cli\src\lib.rs`.
- Add seam-local unit tests for any new folder/module boundaries that split orchestration, normalization, rendering, or auth logic.
- Do **not** widen the smoke or differential fixture matrix unless external behavior changes; the current matrix already protects output parity and CLI option wiring.

**Why this matters:** The current regression stack already catches the most likely restructuring failures: broken exports/imports, changed CLI text or stderr routing, Rust/.NET output drift, and release-binary regressions across the local fixture matrix.

### 2026-05-08: User Directive — Small Logical Commits & No Co-Author
**By:** Christian Helle (via Copilot CLI)
**What:** Keep a detailed progress history by committing changes as often as possible in small logical groups, without a co-author trailer.
**Why:** User request — enables clean, reviewable git history with clear checkpoints for team accountability, and ensures commit message format consistency.

### 2026-05-08: Hicks — generator extraction

**By:** Hicks (Core Dev)

**What:** Replaced flat `generator.rs` with `generator/` facade and leaf modules:
- `mod.rs` facade
- `modes.rs` for output-mode orchestration and file headers
- `render.rs` for request rendering and parameter/summary formatting
- `sample.rs` for JSON sample generation
- `text.rs` for newline/line-writing helpers
- `tests.rs` for the existing generator-focused unit contract

**Why:**
- Scope held to `src\rust\core\src\generator\` only; no OpenAPI pipeline movement
- Public paths frozen through `pub use modes::generate_http_files`, so `httpgenerator_core::generator::*` and crate-root `generate_http_files` stay unchanged
- Maintains httprunner-style modularization direction
- `cargo test -p httpgenerator-core` passed

**Main review risk:** Keep later extraction work from reaching back into generator internals and re-coupling rendering concerns with normalization concerns.

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction

### 2026-05-08: CLI facade freeze uses one compile-level integration contract

**By:** Bishop (Tester)

**What:** Freeze the intentionally public httpgenerator_cli library facade with one focused integration test in src\rust\cli\tests\facade_contract.rs that compiles against the root re-exports (CliError, xecute, xecute_with_observer, should_attempt_azure_auth, observer types) plus representative rgs::* and 	elemetry::* module paths.

**Why:** This is the smallest durable guard against accidental facade drift during internal module moves in src\rust\cli\src, while avoiding a broader promise on internal behavior or widening the CLI binary-contract matrix.

### 2026-05-08: Hicks — core type extraction

**By:** Hicks (Core Dev)

**What:** For the xtract-core-type-modules slice, keep the public httpgenerator_core::{model, normalized} facades frozen and move only their internal type definitions into same-named directory modules with focused leaf files and mod.rs re-exports.

**Applied shape:**

- src\rust\core\src\model\
  - mod.rs
  - output_type.rs
  - settings.rs
  - esult.rs
  - 	ests.rs
- src\rust\core\src\normalized\
  - mod.rs
  - document.rs
  - http.rs
  - parameter.rs
  - equest_body.rs
  - schema.rs
  - 	ests.rs

**Why:**

- This matches the approved httprunner-style direction without widening scope into generator or OpenAPI extraction.
- Same-name directory modules preserve downstream module paths while creating clear seams for later internal refactors.
- The chosen leaves group stable type families instead of making one-file-per-type noise.

**Follow-up:**

- Keep later slices aligned with these seam boundaries when extracting generator/openapi internals.
- Do not rename the model or 
ormalized public modules unless Ripley explicitly approves a breaking API change.

# 2026-05-08: Hicks — split OpenAPI pipeline slice

## What
- Replaced flat `src\rust\core\src\openapi\normalize.rs` with a bounded `normalize\` module facade split into `servers.rs`, `operations.rs`, `parameters.rs`, `request_body.rs`, `schema.rs`, and `tests.rs`.
- Replaced flat `src\rust\core\src\openapi\inspect.rs` with an `inspect\` facade split into `model.rs`, `paths.rs`, `components.rs`, `schema.rs`, and `tests.rs`.
- Kept `src\rust\core\src\openapi\mod.rs` and the public `httpgenerator_core::openapi::*` exports unchanged.

## Why
- This is the approved small refactor slice for oversized OpenAPI internals.
- The chosen seams isolate server normalization, operation/parameter/request-body normalization, schema resolution, and inspection counting without reaching into CLI wiring.
- Existing facade and generator parity tests stayed intact, so downstream behavior remains frozen while internal files become smaller and reviewable.

## Review risk
- Later CLI or generator work should continue to treat `normalize::schema` and `inspect::{paths,components,schema}` as internal seams only; avoid leaking new public exports from those folders.

### 2026-05-08: Hicks — reshape CLI modules

**By:** Hicks (Core Dev)

**What:** Replaced flat Rust CLI files with bounded directory modules for `args`, `execution`, `telemetry`, and binary-local `ui`:
- `args/mod.rs` facade with `help.rs`, `types.rs`, `tests.rs`
- `execution/mod.rs` with `orchestrator.rs`, `validation.rs`, `authorization.rs`, `settings.rs`
- `telemetry/mod.rs` with `events.rs`, `sink.rs`, `recorder.rs`, `redaction.rs`
- `ui/mod.rs` (binary-local) with `presenter.rs`, `render.rs`, `format.rs`
- Kept smaller leaf files flat: `auth.rs`, `error.rs`, `observer.rs`, `writer.rs`

Kept `src\rust\cli\src\lib.rs` on the same intentional public facade: `args::*`, `telemetry::*`, `CliError`, execution entrypoints, and observer types still flow through the crate root.

Kept `src\rust\cli\src\main.rs` thin by limiting it to argument collection, facade-based parsing/execution wiring, telemetry hookup, and exit-code handling.

**Why:** Follows the approved httprunner-style direction without renaming the binary, widening the public API promise, or touching unrelated host surfaces. The new shape creates stable internal seams for future work in auth resolution, validation, telemetry redaction, and UI rendering while preserving the frozen help/facade contracts.

**Validation:** `cargo test -p httpgenerator` passed.

**Review risk:** The UI folder is still binary-local rather than part of the library facade; future work should keep it private unless Ripley explicitly approves a broader public runtime API.

