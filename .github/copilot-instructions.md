# HTTP File Generator

HTTP File Generator is now a Rust-first CLI and compatibility-host repository. The Rust workspace under `src/rust/` is the primary implementation. The .NET projects under `src/dotnet/HttpGenerator*` remain in the repo as the legacy oracle and thin host surfaces during the migration.

Always reference these instructions first and fall back to deeper code search only when the current repo state differs from what is documented here.

## Working Effectively

### Build and Test

- **Never cancel builds or smoke tests.**
- Rust validation:
  - `cargo test --workspace`
  - `cargo run -q -p httpgenerator -- test\OpenAPI\v3.0\petstore.json --output <dir> --no-logging`
- .NET oracle validation:
  - `dotnet restore src/dotnet/HttpGenerator.slnx`
  - `dotnet build src/dotnet/HttpGenerator.slnx --configuration Release`
  - `dotnet test src/dotnet/HttpGenerator.slnx --configuration Release`
- Smoke validation on Windows:
  - `test\smoke-tests.ps1`
- VS Code packaging validation:
  - `src\vscode\build.ps1`
- Root entrypoints are preserved at the repository root:
  - Use the root `Cargo.toml` / `Cargo.lock` for Cargo commands even though crates now live under `src\rust`
  - Invoke `.NET` commands from the repo root against `src/dotnet/HttpGenerator.slnx` / `src/dotnet/VSIX.slnx`
  - Invoke VS Code packaging from the repo root with `src\vscode\build.ps1`

### Validation expectations

Always validate generator-affecting changes with:

1. `cargo test --workspace`
2. `dotnet build src/dotnet/HttpGenerator.slnx --configuration Release`
3. `dotnet test src/dotnet/HttpGenerator.slnx --configuration Release`
4. `test\smoke-tests.ps1`

- Plain `cargo publish --dry-run --allow-dirty` from the repo root attempts both Rust packages.
- Use `cargo publish --dry-run -p httpgenerator-core` plus `cargo check -p httpgenerator` when you need CI-style publish-readiness validation in dependency order.

Use local OpenAPI fixtures from `test\OpenAPI\` for manual verification. OpenAPI 3.1 scenarios still require `--skip-validation`.

## Project Structure

### Primary implementation

- `src/rust/core` - normalized model, `.http` renderer, and `httpgenerator_core::openapi::*`
- `src/rust/cli` - Rust CLI surface

### Compatibility surfaces

- `src/dotnet/HttpGenerator` - legacy .NET CLI oracle
- `src/dotnet/HttpGenerator.Core` - legacy .NET generation library
- `src/dotnet/HttpGenerator.Tests` - legacy .NET test suite
- `src/dotnet/HttpGenerator.VSIX` - Visual Studio host over `httpgenerator.exe`
- `src/vscode` - VS Code host over the Rust CLI
- Repo-root entrypoints remain in place (`Cargo.toml`, `Cargo.lock`, `Makefile`, `test\`, and `docs\`)

## Common Tasks

### Generator changes

- Prefer editing Rust crates under `src/rust` first.
- Use `src/rust/cli/tests/differential_petstore.rs` to catch byte-for-byte parity regressions against the .NET oracle.

### CLI and host changes

- Rust CLI entry point: `src/rust/cli/src/lib.rs`
- VS Code executable setting: `http-file-generator.executablePath`
- VS Code host resolves `httpgenerator` from `http-file-generator.executablePath`, the bundled extension binary, repo-root workspace `target\debug` / `target\release`, or `PATH`; an invalid explicit setting should fail fast instead of silently falling back
- Visual Studio host resolves `httpgenerator.exe` from `HTTPGENERATOR_PATH`, the bundled VSIX payload, repo-root workspace `target\debug` / `target\release`, or `PATH`

### Packaging and release

- Release artifacts are Rust CLI archives:
  - `httpgenerator-<version>-linux-x64.tar.gz`
  - `httpgenerator-<version>-darwin-x64.tar.gz`
  - `httpgenerator-<version>-win-x64.zip`
- VS Code packages are platform-targeted `.vsix` files because they bundle native binaries.
- The Visual Studio `.vsix` bundles `httpgenerator.exe`.
- The repo no longer treats `dotnet tool install --global httpgenerator` as the primary install path.

## Known Issues and Workarounds

- External URL tests can still fail in restricted environments. Prefer local fixtures.
- OpenAPI 3.1 generation still requires `--skip-validation`.
- Headless VSIX builds are environment-sensitive. If `src/dotnet/VSIX.slnx` fails with missing Visual Studio SDK/toolkit types, compare the failure to a clean baseline before treating it as a regression.

## Technology Stack

- Rust workspace for the current product implementation
- .NET for the legacy oracle and Visual Studio host
- TypeScript for the VS Code extension
- xUnit for legacy .NET tests
