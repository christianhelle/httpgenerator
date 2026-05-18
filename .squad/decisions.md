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

### 2026-05-18T13:14:38.236+02:00: Standing Commit Cadence Enforcement
**By:** Ripley (Lead)
**What:** Treat small logical commits as a standing squad workflow rule, not a session-only preference. Future sessions must commit each completed logical slice immediately using terse exact Conventional Commit messages, stage only the files for that slice, never amend or rewrite existing commits to regroup work, and never add `Co-authored-by` or other attribution trailers. Encode this rule in `.squad/copilot-instructions.md`, `.github/agents/squad.agent.md`, `.squad/templates/squad.agent.md`, `.squad/issue-lifecycle.md`, and `.squad/templates/issue-lifecycle.md`.
**Why:** The prior decision captured the intent, but future sessions still had stale instruction surfaces. The rule now lives in the coordinator and issue workflow prompts that agents actually read, so detailed progress history survives across sessions instead of depending on memory.

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


---
recorded_at: 2026-05-18T11:53:21.547+02:00
author: Bishop
topic: VSIX first-slice revalidation
status: approved
---

# VSIX first-slice revalidation

## Verdict

- **Approve**

## Why

1. The revised implementation now lands the previously missing first-slice notification contract: duplicate runs raise a non-blocking `Open Activity` prompt, successful runs raise a non-blocking `Open Folder` prompt, and failures raise a non-blocking `View Details` prompt that routes to the activity/tool window.
2. The Solution Explorer entry point is now aligned with the approved seam by using `VsctParent(... id: 521 ...)` for file-context placement while preserving the Tools menu fallback.
3. The revised state produced credible validation signal in this environment: `dotnet build src\dotnet\VSIX.slnx --configuration Release`, `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release`, and `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release` all passed.

## Notes

- I did not find a remaining blocker in the previously rejected areas.
- This is still a headless tester verdict; the manual Visual Studio interaction checks from the approved plan remain valuable follow-up coverage outside this pass.

# Bishop VSIX validation inbox

- **Recorded at:** 2026-05-18T11:53:21.547+02:00
- **Scope:** First Visual Studio extension implementation wave in `src\dotnet\HttpGenerator.VSIX`
- **Verdict:** Reject

## Blocking findings

1. The approved slice requires non-blocking success/failure notifications with actions (`Open Folder` on success, details/log on failure, duplicate-run notice). The current implementation routes those outcomes only into the tool window state and `ShowToolWindowAsync(...)`, so the approved notification UX is not implemented yet.
2. Because the notification/action contract is missing, the duplicate-run policy is only partially implemented: same-spec runs are blocked in the coordinator, but the user does not get the approved notification signal that generation is already in progress.

## Validation signal

- `cargo test --workspace` ✅
- `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release` ✅
- `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release` ✅
- `dotnet build src\dotnet\VSIX.slnx --configuration Release` ✅
- `test\smoke-tests.ps1` ✅

## Coverage note

- I did not add automated coverage. There is no existing low-risk VSIX test seam in the repo for the new coordinator/tool-window flow, and creating a new harness would exceed the tester-only scope for this validation pass.

### 2026-05-18T11:53:21.547+02:00: User directive
**By:** Christian Helle (via Copilot)
**What:** Have all agents use Claude Opus 4.7 for the rest of this session only.
**Why:** User request — captured for team memory

# 2026-05-18T11:53:21.547+02:00 — VSIX async flow detail surface

- The first VSIX redesign slice uses the preview settings API plus a custom Remote UI tool window for persisted defaults.
- Background generation uses Task Status Center progress for lifecycle and cancellation, while success/failure follow-up actions live in the same non-blocking tool window instead of a blocking prompt.
- Executable resolution is deterministic and fail-fast: `HTTPGENERATOR_PATH` → bundled `httpgenerator.exe` payload when present → repo `target\debug|release` outputs → `PATH`.

---
recorded_at: 2026-05-18T11:53:21.547+02:00
author: Ripley
topic: VSIX first-slice review verdict
status: rejected
---

# VSIX first-slice review verdict

## Artifact scope rejected

- Hicks' current VSIX implementation wave under `src\dotnet\HttpGenerator.VSIX\`

## Verdict

- **Reject**

## Why

1. The implementation does not land the approved notification contract for the first slice. The current coordinator/tool-window flow records status in the tool window, but it does not surface the required non-blocking success/failure notifications, the success-side `Open Folder` action, or the failure-side details/log action.
2. The Solution Explorer placement seam is not locked to the reviewed SDK guidance. The approved seam check called for the documented `VsctParent(... id: 521 ...)` file-context placement path plus Tools fallback; the current implementation introduces `KnownVsctIds.ItemNodeContextMenu = 0x0430`, which is a risky unreviewed assumption for the primary entry point.

## Notes

- The architectural direction is otherwise aligned: request snapshotting, a background coordinator, persisted settings, a Remote UI tool window, duplicate-run blocking, and fail-fast CLI lookup all move in the approved direction.
- Headless validation in this environment passed for `dotnet build src\dotnet\VSIX.slnx --configuration Release` and `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release`, so the rejection is scope/contract based rather than a compile failure.

---
recorded_at: 2026-05-18T11:53:21.547+02:00
author: Ripley
topic: VSIX revision notification seam
status: applied
---

# VSIX revision notification seam

## Decision

- Use the current `ShellExtensibility.ShowPromptAsync(...)` prompt surface for the first-slice non-blocking success/failure/duplicate-run notifications.
- Keep the Remote UI tool window as the richer details/activity surface behind notification actions such as `View Details` and `Open Activity`.

## Why

- The approved first slice requires action-bearing notifications now, while the current SDK surface available in this repo already exposes prompt choices without forcing a return to modal dialog-era command flow.
- The tool window remains the approved home for persisted settings and richer failure/activity details, so notification actions can stay lightweight while preserving the non-blocking background coordinator shape.

## Applied scope

- Success prompt offers `Open Folder`.
- Failure prompt offers `View Details` and routes to the activity/tool window.
- Duplicate-run prompt offers `Open Activity`.
- Solution Explorer primary placement is locked to the documented file-context-menu `VsctParent(... id: 521 ...)` seam, while the Tools menu fallback remains intact.

---
recorded_at: 2026-05-18T11:53:21.547+02:00
author: Ripley
topic: VSIX SDK seams
status: proposed
---

# VSIX SDK seam check

## What Ripley is locking for this implementation pass

1. **Primary entry point:** place the generate command on the Solution Explorer file context menu with `CommandPlacement.VsctParent(...)`, using the documented/new-model command placement path rather than dialog-era command plumbing.
2. **Fallback entry point:** keep a `CommandPlacement.KnownPlacements.ToolsMenu` placement on the same command for discoverability and fallback targeting.
3. **Settings editing:** use a dedicated non-blocking `ToolWindow` with Remote UI as the first supported settings editor surface.
4. **Failure/details surface:** use a non-modal `ToolWindow` for rich details/logs; do not make the initial slice depend on the preview Output Window API.

## Why

- In the current SDK generation, built-in known placements cover top-level menu locations like Tools, but Solution Explorer placement still requires `VsctParent(...)`.
- Microsoft sample code for the new extensibility model uses `VsctParent(... id: 521 ...)` for file-in-project context menu placement and uses tool windows for richer extension UI.
- The settings API exists but is still preview/experimental, and the SDK does not provide a production-ready built-in extension settings UI.
- Dialogs and prompts are modal, which conflicts with the approved non-blocking flow.

## Guardrails for Hicks

- Safe now: add explicit command placements, add a tool-window-based settings surface, and add a tool-window-based details/log surface.
- Avoid now: relying on Output Window preview APIs as the primary details UX, or expecting built-in Options/settings pages to cover the approved editor experience.
- Adjustment for the next wave: tighten selection resolution before wiring Tools fallback, because the current helper resolves the active project path instead of the selected spec file.
