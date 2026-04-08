---
name: "cli-surface-parity-audit"
description: "Audit Rust CLI UX parity against the legacy C# Spectre.Console surface in HTTP File Generator"
domain: "cli-ux"
confidence: "high"
source: "manual"
---

## Context

Use this skill when the Rust CLI in `crates/` needs to match or intentionally diverge from the legacy C# CLI in `src/`. It is especially useful for help text, version output, banners, colors, progress/status messaging, validation output, and host integration checks.

## Patterns

### Compare the public surface first

- Legacy C# CLI surface:
  - `src/HttpGenerator/Program.cs`
  - `src/HttpGenerator/Settings.cs`
  - `src/HttpGenerator/GenerateCommand.cs`
- Rust CLI surface:
  - `crates/httpgenerator-cli/src/args.rs`
  - `crates/httpgenerator-cli/src/main.rs`
  - `crates/httpgenerator-cli/src/lib.rs`
  - `crates/httpgenerator-cli/tests/help_contract.rs`

### Audit runtime behavior with direct commands

- Prefer direct binaries or built executables over wrapper commands when possible so output is not polluted by build noise.
- Compare at least these scenarios:
  - `--help`
  - `--version`
  - successful petstore generation
  - missing-file failure
  - OpenAPI 3.1 fixture with and without `--skip-validation`

### Check host constraints before recommending ANSI output

- VS Code host (`src/VSCode/src/extension.ts`) launches the CLI in a terminal; rich output is visible.
- Visual Studio host (`src/HttpGenerator.VSIX/HttpGeneratorCli.cs`) captures stdout/stderr; fatal errors must stay on stderr, and any success-path warnings written to stderr must be surfaced explicitly by the host.
- Smoke tests (`test/smoke-tests.ps1`) are the right place to model redirected stdout/stderr capture for host checks.
- Compatibility tests (`crates/httpgenerator-compat/src/runner.rs`) compare generated files, not CLI UI.

### Keep Windows PowerShell smoke checks ASCII-safe

- `test/smoke-tests.ps1` runs under Windows PowerShell in this repo. Avoid literal box-drawing and emoji markers in the script source because UTF-8-without-BOM files can parse incorrectly there.
- Build rich-marker lists with `[char]0x....` or `[System.Char]::ConvertFromUtf32(...)` so the script stays parseable while still asserting plain redirected output.
- When you need split-stream host validation, prefer `Start-Process -RedirectStandardOutput/-RedirectStandardError` into repo-local files over shell redirection so stdout/stderr remain distinct.

### Look for execution-flow blockers, not just missing colors

- If Rust prints phase messages only after `execute()` returns, rich progress cannot be truly live.
- In this repo, `crates/httpgenerator-cli/src/main.rs` prints validation before execution, but Azure-auth and file-writing messages are emitted after `execute()` has already completed.
- Full Spectre-style parity needs a renderer/reporter abstraction, not just new color crates.

### Validate content parity, not only chrome

- Compare validation behavior and stats values, not only visual formatting.
- In this repo, Rust stats in `crates/httpgenerator-openapi/src/inspect.rs` do not currently match the C# `OpenApiWalker` counts in `src/HttpGenerator/Validation/OpenApiStats.cs`.
- OpenAPI 3.1 validation is another parity check: the C# oracle currently succeeds while Rust still blocks validation.

## Examples

- Legacy rich renderer: `src/HttpGenerator/GenerateCommand.cs`
- Rust output contract: `crates/httpgenerator-cli/tests/help_contract.rs`
- Host constraints:
  - `src/VSCode/src/extension.ts`
  - `src/HttpGenerator.VSIX/HttpGeneratorCli.cs`
- Output parity blind spot:
  - `crates/httpgenerator-compat/tests/differential_petstore.rs`

## Anti-Patterns

- Do not assume generator-file parity means CLI-surface parity.
- Do not add ANSI/hyperlink output unconditionally; redirected VSIX error output must remain readable.
- Do not treat `help_contract.rs` as incidental; it is the Rust CLI text contract.
- Do not plan “pretty output” only in `main.rs` if the execution API cannot emit real-time phase events.
