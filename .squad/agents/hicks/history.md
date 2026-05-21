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

### VSIX async first-slice implementation closeout (2026-05-18T09:53:21Z)
- Landed the first Visual Studio extension redesign wave with a background coordinator, persisted defaults in the tool window, deterministic CLI lookup, cancellation, and right-click plus Tools fallback entry points.
- Ripley rejected the first cut for missing notifications and the wrong placement seam; the revised state was approved after the notification contract and VsctParent(... id: 521 ...) placement were restored.

### VSIX visibility fix follow-up (2026-05-21T18:18:31Z)
- Investigated and fixed Tools submenu regression where `GenerateHttpCommand` lost its always-visible fallback path after earlier refactor moved it to a standalone `ToolsMenu` placement guarded by `ActiveSelectionFileName`.
- Committed `7753820 fix(vsix): restore tools fallback` to restore the `GenerateCommand` as a submenu item under `ExtensionEntrypoint.GenerateMenu` while preserving Solution Explorer file-context placement.
- Coordinated with Ripley's concurrent case-sensitivity fix (`344e49b`) and Bishop's multi-pass validation to close the regression.

