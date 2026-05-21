# Squad Decisions

## Active Decisions

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
  - 
esult.rs
  - 	ests.rs
- src\rust\core\src\normalized\
  - mod.rs
  - document.rs
  - http.rs
  - parameter.rs
  - 
equest_body.rs
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

### 2026-05-13: Session Directive — Spawned Agents Use Claude Opus 4.7
**By:** Christian Helle (via Copilot)
**What:** All spawned agents in this session must use Claude Opus 4.7.
**Why:** Session-only user directive for consistent agent execution.
**Supersedes:** 2026-05-05 session directive to use GPT-5.5.

### 2026-05-13: VS Code Rust Host Migration Review Gate
**By:** Ripley (Lead)
**What:** Treat the VS Code Rust-host migration as one coordinated cutover. Do not approve a runtime-only rewrite in `src\vscode\src\extension.ts` unless the same change set also lands the locked executable contract, bundled-binary packaging path, workflow retargeting, and extension-specific validation.
**Why:** Splitting runtime lookup from packaging, workflow targeting, or validation would leave the extension on an unreviewable half-migrated host path.
**Locked contract:** `http-file-generator.executablePath` → bundled binary in the installed extension → repo-root `target\debug` / `target\release` outputs for development → `httpgenerator` on `PATH`.
**Guardrail:** An invalid explicit setting must fail fast instead of falling through, and `src\vscode\PRD.md` should be treated as stale for this stream unless Ripley explicitly re-approves that direction.

### 2026-05-13: VS Code Rust Host Packaging Contract
**By:** Hicks (Core Dev)
**What:** Stage the native Rust CLI into `src\vscode\bin\<target>\httpgenerator(.exe)` during each package run, build exactly one VS Code target per job so each `.vsix` carries only its matching binary, and keep runtime resolution aligned with packaging: `http-file-generator.executablePath` → staged per-target bundled binary → repo-root `target\debug` / `target\release` development outputs → `PATH`.
**Why:** Bundled binaries, packaging jobs, and development-time fallback must follow the same contract or extension builds drift from the runtime the user actually executes.
**Validation:** `cargo test --locked --workspace`; `dotnet build`; `dotnet test`; `test\smoke-tests.ps1`; `src\vscode\build.ps1 -Version 0.1.0 -Target win32-x64`.

### 2026-05-13: VS Code Rust Host Docs Contract
**By:** Hudson (DevRel/Docs)
**What:** Keep VS Code documentation aligned on the Rust-host contract across `src\vscode\README.md`, `README.md`, `CONTRIBUTING.md`, `docs\README.md`, `docs\index.html`, and `.github\copilot-instructions.md`: platform-targeted bundled `.vsix` packages, executable resolution order `http-file-generator.executablePath` → bundled binary → repo-root workspace `target\debug` / `target\release` → `PATH`, and no `.NET Tool` or crates.io installer guidance for extension users.
**Why:** The extension ships as a bundled-binary experience, so docs must not drift back into legacy install flows or imply crates.io is the extension delivery channel.

### 2026-05-13T23:06:43.790+02:00: VS Code target drives bundled Rust target
**By:** Vasquez
**What:** VS Code packaging must derive the Rust compilation target from the requested VS Code target and stage the executable from `target\<rust-target>\release`, while PR CI gates the same shipped VSIX target matrix before merge.
**Why:** The VSIX target name is the packaging contract reviewers locked for this migration. Reusing a host-built binary from `target\release` can silently mislabel the bundled CLI, so the build path must either produce the matching Rust binary or fail.

### 2026-05-20T14:40:47.144+02:00: User directive — Agents use GPT-5.5
**By:** Christian Helle (via Copilot)
**What:** Have all agents use GPT-5.5 for the rest of the session.
**Why:** User request — captured for team memory.

### 2026-05-20T15:37:38.935+02:00: VSIX command visibility fix
**By:** Hicks (Core Dev), Ripley (Lead), Bishop (Tester)
**What:** Restore `Generate .http files` as a direct Tools-menu command placement on `GenerateHttpCommand` while keeping the Solution Explorer context-menu placement intact. Add `CommandPlacement.KnownPlacements.ToolsMenu` directly to `GenerateHttpCommand.Placements` alongside the existing `VsctParent(... id: 521 ...)` placement. Remove `MenuChild.Command<Commands.GenerateHttpCommand>()` from `ExtensionEntrypoint.GenerateMenu`. Keep `GenerateMenu` in Tools for `ShowHttpGeneratorToolWindowCommand` only.
**Validation:** Headless validation passed with `dotnet build src\dotnet\VSIX.slnx --configuration Release`.
**Caveat:** Manual IDE smoke check recommended: verify `Generate .http files` appears directly under **Tools** and on the Solution Explorer context menu.
**Why:** The Tools menu route is the stable visibility contract. The extension needs both the Solution Explorer file-context entry and the Tools fallback without displacing either contribution path.

### 2026-05-21T14:35:15.308+02:00: User directive — Commit format and scope
**By:** Christian Helle (via Copilot)
**What:** When implementing, commit changes in small logical groups as often as practical, without a co-author trailer, to preserve detailed progress history.
**Why:** User request — enables clean, reviewable git history with clear checkpoints for team accountability.

### 2026-05-21T14:35:15.308+02:00: docs.rs structure for httpgenerator-core
**By:** Hudson (DevRel/Docs)
**What:** Treat the first `httpgenerator-core` rustdoc pass as a guide-and-reference docs.rs structure. Crate root should explain the library, feature gating, and end-to-end workflow. `openapi` module documents the ingestion pipeline. `normalized` explains the stable intermediate model. `generator` and `model` document the output contract. Helper modules stay short and purpose-first. Concentrate longer examples at workflow boundaries.
**Pitfalls to avoid:** Duplicating workflow prose across crate/module/item docs, treating re-export modules as pure symbol indexes, overusing helper examples while leaving core workflows unexplained, documenting intent vs. current behavior.
**Why:** A public library docs.rs surface feels fragmented when the crate never explains the main path through its APIs. This structure keeps navigation centered on the user journey.

### 2026-05-21T14:35:15.308+02:00: docs.rs batching guardrails for httpgenerator-core
**By:** Ripley (Lead)
**What:** Approve the docs-first implementation order as the canonical batching plan. Do not leave docs.rs-specific feature-gate guidance and minimal normalized-model docs to a later sweep. Expected commit batches: (1) Crate root + feature-gate signaling; (2) Generator/model docs + minimal normalized types; (3) Root helpers; (4) Remaining normalized reference types; (5) openapi/mod.rs + workflow docs; (6) Inspection/error/reference detail pages.
**Why:** The first batch covers crate root, generation entry points, model types, and root-level helpers, giving docs.rs a usable front door quickly. Crate-root example and `generate_http_files` signature both depend on normalized model types. The public `openapi` module is feature-gated, so docs.rs should make that explicit.
**Review guardrails:** Keep docs-only (no API cleanup or hiding). Prefer runnable doctests for deterministic helpers. Reuse workflow vocabulary: `raw` → `typed` → `normalized` → `generated`.

### 2026-05-21T14:35:15.308+02:00: Rustdoc validation sequence for docs.rs work
**By:** Bishop (Tester)
**What:** For incremental docs-only batches in `src\rust\core\src`, use `cargo test -p httpgenerator-core --doc` as the first validation gate. Before docs batch is locally stable, run `cargo test -p httpgenerator-core` to keep doctests and unit/integration coverage aligned. Final approval for documentation passes still uses the repo-standard sequence: (1) `cargo test --workspace`; (2) `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release`; (3) `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release`; (4) `test\smoke-tests.ps1`. Prefer runnable doctests for pure helpers and inline raw-document loading; reserve `no_run` for fixture paths, local files, remote URLs, or environment-sensitive setup.
**Why:** `cargo test -p httpgenerator-core --doc` is the quickest signal for broken code fences, bad imports, and rustdoc drift while authors iterate. Full repository sequence remains necessary because even docs-only edits in public Rust modules can break compilation, packaging, or smoke-test assumptions.

### 2026-05-21T15:00:01.518+02:00: Bishop final docs validation
**By:** Bishop (Tester)
**Decision:** Treat the docs.rs closeout validation as complete based on the standard root sequence passing, and treat the earlier nested Windows PowerShell smoke-script failure as an invocation artifact rather than a product regression.
**Why:**
- `cargo test --workspace` passed, including the new/updated `httpgenerator_core` doctests.
- `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release` passed.
- `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release` passed with 246/246 tests green.
- `test\smoke-tests.ps1` passed when run directly in the active PowerShell 7 session from the repo root.
- The failed nested run used Windows PowerShell 5.1 semantics, which left `$IsWindows` falsey inside `test\smoke-tests.ps1`, causing it to look for `target\release\httpgenerator` instead of `target\release\httpgenerator.exe`; that does not implicate Hicks's docs changes.
**Consequence:** `validate-docs-pass` can be closed as done. No production-file rollback or follow-up is justified from this validation pass alone.

### 2026-05-21T15:00:01.518+02:00: Hicks openapi docs batch
**By:** Hicks (Core Dev)
**Decision:** Surface the optional `openapi` API explicitly on docs.rs by enabling `docsrs` rustdoc cfg metadata and annotating the public module with `doc(cfg(feature = "openapi"))`.
**Context:** The `openapi` feature is default-on today, which makes the optional surface easy to miss on docs.rs even though downstream consumers can disable default features.
**Consequence:** The generated docs keep the current API shape, but readers now see the feature gate and the `openapi` module overview can explain the ingestion pipeline and current raw-fallback behavior.

### 2026-05-21T15:00:01.518+02:00: Hudson openapi reference copy guidance
**By:** Hudson (DevRel/Docs)
**Decision:** Document the remaining `httpgenerator_core::openapi` reference surface as five docs.rs-oriented layers: raw loading, inspection, typed parsing/version detection, source/format classification, and errors. Keep the main workflow story at the module level, put concrete examples only on the boundary APIs readers are likely to call directly, and keep structs/enums that mainly carry state or variants reference-first.
**Implementation guidance:**
1. **Raw loading** (`raw.rs`): Explain the raw stage once covering local path and HTTP URL loading, format detection, and source preservation. Examples: `load_raw_document`, `decode_raw_document`. Reference-only: accessors and format fields.
2. **Inspection** (`inspect\mod.rs`, `inspect\model.rs`): Frame as lightweight inventory pass. Example: `inspect_raw_document` or `inspect_document`. Reference-only: `OpenApiInspection`, `OpenApiStats`.
3. **Typed parsing and version detection** (`typed.rs`, `version.rs`): Bridge from raw JSON to version-specific models. Examples: `parse_typed_document`, `detect_specification_version`. Reference-only: `TypedOpenApiDocument`, `OpenApiSpecificationVersion`.
4. **Source and format classification** (`source.rs`, `format.rs`): Classify input and infer format. Examples: `classify_source`, `detect_content_format`, `sniff_content_format`. Reference-only: enums and accessors.
5. **Errors** (`error.rs`): Orient by pipeline stage. Reference-style: mostly self-documenting enums mapping failures to stages.

### 2026-05-21T15:00:01.518+02:00: Ripley docs.rs follow-up
**By:** Ripley (Lead)
**Decision:** Keep the next rustdoc implementation batch focused on the `normalized` surface before expanding the `openapi` surface.
**Required follow-up:**
1. Finish the normalized handoff docs: `src\rust\core\src\normalized\parameter.rs`, `src\rust\core\src\normalized\request_body.rs`, `src\rust\core\src\normalized\schema.rs`.
2. In the same batch, add docs.rs-visible feature-gate signaling for `openapi`.
3. After that, land the `openapi` narrative pass starting with `src\rust\core\src\openapi\mod.rs`, then load/inspect/normalize entry pages.
**Why:** Commits `2f6faad` and `2925ddf` made the crate root, generator, model, and helper surfaces meaningfully better. The weakest remaining reader path is the normalized bridge. `openapi` already has richer item-level pages but needs proper overview and feature-gate visibility.

### 2026-05-21T15:00:01.518+02:00: Ripley final docs audit
**By:** Ripley (Lead)
**Decision:** Hicks's current `openapi-reference-batch` should be the final meaningful rustdoc authoring slice for the remaining `httpgenerator-core` docs.rs surface, provided it explicitly covers the still-thin OpenAPI reference pages for raw loading, inspection, typed parsing, version detection, source classification, format detection, and errors.
**Why:** Landed batches already cover crate root, generator/model/root helpers, normalized handoff types, and openapi module plus load/normalize entry points. The remaining weak docs.rs pages cluster in `src\rust\core\src\openapi\{raw.rs,typed.rs,version.rs,source.rs,format.rs,error.rs}` plus `inspect\mod.rs` and `inspect\model.rs`. No other public docs.rs-visible surface outside that batch justifies a separate authoring pass once those reference pages are documented.
**Reviewer guidance:**
- Keep `version.rs` in the current reference batch.
- Treat `OpenApiStats`, `OpenApiInspection`, `RawOpenApiDocument`, `OpenApiSource`, `OpenApiContentFormat`, `TypedOpenApiDocument`, `OpenApiSpecificationVersion`, and error enums as the remaining "blank page" risk.
- Leave `author-rustdoc-batches` in progress until Hicks lands that batch; after landing, final validation should be enough unless review finds prose-quality issues.

