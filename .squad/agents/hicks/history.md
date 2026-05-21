# Hicks — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** .NET 8.0 CLI, C#, Rust workspace, VS Code extension host, xUnit, PowerShell packaging scripts

**Canonical product roots:**
- Rust workspace members: `src\rust\`
- .NET solution and apps: `src\dotnet\`
- VS Code extension: `src\vscode\`

**Primary implementation surfaces:**
- `src\rust\httpgenerator-cli\src\lib.rs`
- `src\dotnet\HttpGenerator.VSIX\HttpGeneratorCli.cs`
- `src\vscode\src\extension.ts`
- `src\vscode\build.ps1`

**Build and validate:**
- `cargo test --locked --workspace`
- `dotnet build`
- `dotnet test`
- `test\smoke-tests.ps1`
- `src\vscode\build.ps1 -Version 0.1.0 -Target win32-x64`

## Historical Summary
- Delivered source-layout migration follow-through, crates.io packaging/workflow changes, and the approved Rust modularization slices while keeping repo-root entrypoints stable.
- Archive detail moved to `history-archive.md` on 2026-05-13T21:06:43Z after the live history exceeded 15 KB.

## Learnings

### Rust packaging and workspace contract
- Rust publish work follows Ripley's package gate: public crates are `httpgenerator`, `httpgenerator-core`, and `httpgenerator-openapi`; `src\rust\httpgenerator-compat` stays private with `publish = false`.
- Release-time version injection must rewrite every `version = "0.1.0"` anchor in the root `Cargo.toml` so workspace package metadata and sibling dependency pins stay aligned.
- Stable crates.io publication order is `httpgenerator-core` → `httpgenerator-openapi` → `httpgenerator` with crates.io visibility checks between publishes.

### Rust modularization direction
- Safe module refactors keep public facades stable while moving internals into same-named directory modules with thin `mod.rs` re-exports.
- Use the existing validation matrix as the boundary for internal refactors: Rust tests, .NET build/test, and smoke coverage should stay intact unless the user explicitly broadens scope.

### VS Code Rust Host Packaging Contract (2026-05-13T21:06:43Z)
- Stage the native Rust CLI into `src\vscode\bin\<target>\httpgenerator(.exe)` during each package run and build one VS Code target per job so each `.vsix` only carries its matching binary.
- Runtime lookup must stay aligned with packaging: `http-file-generator.executablePath` → bundled binary → repo-root `target\debug` / `target\release` development outputs → `PATH`.
- Keep the extension host documentation and code on the canonical lowercase `src\vscode` path; treat stale `src\VSCode` references as cleanup opportunities when they block this workflow.
- Validation evidence for the migration: `cargo test --locked --workspace`, `dotnet build`, `dotnet test`, `test\smoke-tests.ps1`, and `src\vscode\build.ps1 -Version 0.1.0 -Target win32-x64`.

### docs.rs rustdoc batch 1 (2026-05-21T14:35:15.308+02:00)
- The first `httpgenerator-core` docs pass is intentionally docs-only: improve crate/module/item/field rustdoc without changing public visibility or hiding internals yet.
- High-value docs entry points for docs.rs live in `src\rust\core\src\lib.rs`, `src\rust\core\src\generator\mod.rs`, `src\rust\core\src\generator\modes.rs`, and `src\rust\core\src\model\*.rs`; helper API narratives belong in the flat leaf modules such as `base_url.rs`, `file_naming.rs`, `operation_name.rs`, `privacy.rs`, `string_extensions.rs`, and `support_information.rs`.
- Prefer runnable doctests for self-contained helpers and normalized-model generation examples, but keep heavier flows as narrative or `no_run` examples so docs.rs stays practical without introducing brittle setup.
- User preference for this stream: keep commits in small logical groups and do not include a co-author trailer.
- Hudson's docs.rs structure guidance fits this crate well: crate root should explain purpose, feature gating, and the load -> normalize -> generate workflow, while `generator` and `model` docs should lead with when-to-use, then current behavior/limits, then a minimal example.
- Ripley's next docs priority is the normalized handoff layer: document the minimal public `normalized` types that sit between crate-level workflows and `generate_http_files` before expanding broader OpenAPI coverage.

### docs.rs batching and validation gates (2026-05-21T13:00:01Z)
- Completed commits 2f6faad (`docs: expand core rustdoc entrypoints`) and 2925ddf (`docs: align core rustdoc structure`); both validate locally with `cargo test -p httpgenerator-core --doc` passed (doctests and code fences).
- Ripley's canonical batching plan approved for implementation: (1) crate root + docs.rs feature-gate signaling, (2) generator/model + minimal normalized types, (3) root helpers, (4) remaining normalized types, (5) openapi/mod.rs + workflow docs, (6) inspection/error/reference detail pages.
- Bishop approved incremental doctests validation (fast signal: `cargo test -p httpgenerator-core --doc`) plus full repository sequence for final approval.
- Current work in progress: normalized-docs-batch (batch 2 per Ripley's plan).
- Final validation gate remains full matrix: `cargo test --workspace`, `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release`, `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release`, `test\smoke-tests.ps1`.

### OpenAPI entrypoint docs batch (2026-05-21T15:00:01.518+02:00)
- Added the `openapi` module overview for docs.rs with a pipeline narrative, a "Which function should I call?" guide, and explicit fallback limits for Swagger 2 and OpenAPI 3.1 raw loading.
- Documented the public loader and normalization entry points so docs.rs callers can choose between source strings, classified sources, raw decoded documents, and direct normalization.
- Enabled docs.rs `doc(cfg)` surfacing for the optional `openapi` module by adding crate metadata plus a gated module annotation in `src\rust\core`.
- Validation for this batch: `cargo test --workspace`.

### OpenAPI reference docs batch (2026-05-21T15:00:01.518+02:00)
- Documented the remaining public OpenAPI reference surfaces in `raw`, `inspect`, `typed`, `version`, `format`, `source`, and `error`, with short module overviews plus concise item docs for docs.rs navigation.
- Kept examples on boundary APIs only: raw decode/load, inspection entry points, typed front-door parsing, source classification, content-format detection, and specification-version detection.
- Used reference-style docs for stats fields, version enums, and public error variants so the remaining OpenAPI pages stay scan-friendly instead of repeating workflow prose.
- Validation for this batch: `cargo test --workspace`.

### docs.rs closeout — final consolidated validation (2026-05-21T13:00:01Z)
- All three documentation batches (commits `7e5125d`, `5d16a45`, `70f5975`) completed and validated.
- `cargo test --workspace` passed including 38/38 httpgenerator_core doctests.
- Ripley final audit confirmed: no meaningful public docs.rs gaps remain outside this batch.
- Bishop final validation confirmed: full repository matrix green (cargo test, dotnet build, dotnet test, smoke tests).
- Feature-gate signaling for `openapi` module now explicitly visible on docs.rs.
- httpgenerator-core public documentation complete; `author-rustdoc-batches` closed as DONE.
