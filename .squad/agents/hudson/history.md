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
