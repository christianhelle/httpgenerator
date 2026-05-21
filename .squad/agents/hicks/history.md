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
