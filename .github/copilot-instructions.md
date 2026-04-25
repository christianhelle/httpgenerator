# HTTP File Generator

HTTP File Generator is a Rust CLI tool that generates .http files from OpenAPI specifications for use with REST Client extensions in IDEs like VS Code, JetBrains, and Visual Studio 2022. The previous .NET implementation is preserved under `legacy/`.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Build and Test (CRITICAL TIMING)
- **NEVER CANCEL BUILD OR TEST COMMANDS** - Always wait for completion
- Dependency restore is handled by Cargo during build/test
- Release build: `cargo build --release`
- Debug build: `cargo build`
- Unit tests: `cargo test`
- Smoke tests: `cd test && ./smoke-tests.sh` (4.5 minutes - NEVER CANCEL, set timeout to 10+ minutes)

### Development Environment Setup
- Install the Rust stable toolchain
- Install PowerShell Core (`pwsh`) for smoke tests
- Clone repository: `git clone https://github.com/christianhelle/httpgenerator.git`
- Build immediately after clone:
  ```bash
  cargo build --release
  ```

### CLI Tool Usage and Validation
- Run tool: `cargo run -- [options]`
- **Basic usage**: `cargo run -- test/OpenAPI/v3.0/petstore.json --output /tmp/output --no-logging`
- **Network limitations**: External URLs may fail, use local OpenAPI files from `test/OpenAPI/` directory
- **OpenAPI version support**: 
  - v2.0 and v3.0: Full support
  - v3.1: Requires `--skip-validation` flag
- **Output types**: `OneRequestPerFile` (default), `OneFile`, `OneFilePerTag`
- **Always test changes** by running the CLI with local OpenAPI files after modifications

## Validation Scenarios

### Manual Testing Requirements
Always validate changes by running these complete scenarios:

1. **Basic Generation Test**:
   ```bash
   cargo run -- test/OpenAPI/v3.0/petstore.json --output /tmp/test --no-logging
   ```
   Expected: 19 .http files generated successfully

2. **Single File Mode Test**:
   ```bash
   cargo run -- test/OpenAPI/v3.0/petstore.json --output /tmp/test-single --output-type OneFile --no-logging
   ```
   Expected: 1 Requests.http file with all endpoints

3. **Custom Headers and IntelliJ Tests**:
   ```bash
   cargo run -- test/OpenAPI/v3.0/petstore.json --output /tmp/test-custom --generate-intellij-tests --custom-header "X-API-Key: test123" --base-url https://api.example.com --no-logging
   ```
   Expected: Files with custom headers and JavaScript test blocks

4. **OpenAPI v3.1 with Skip Validation**:
   ```bash
   cargo run -- test/OpenAPI/v3.1/webhook-example.json --output /tmp/test-v31 --skip-validation --no-logging
   ```
   Expected: Files generated without validation errors

### Expected Generated Content Validation
Generated .http files should contain:
- Proper variable definitions (`@baseUrl`, `@contentType`)
- Request headers including custom headers
- Path parameters with example values
- Request bodies with example JSON
- IntelliJ test assertions when `--generate-intellij-tests` is used

## Project Structure and Navigation

### Core Projects
- `src/main.rs` - CLI application entry point
- `src/cli.rs` - CLI options and parsing
- `src/generator.rs` - Core generation logic
- `src/openapi.rs` - OpenAPI loading, parsing, and statistics
- `src/naming.rs` - Operation naming helpers
- `src/VSCode/` - VS Code extension
- `legacy/` - Previous .NET CLI, core library, tests, and Visual Studio extension

### Key Files to Know
- `src/main.rs` - Main CLI command implementation
- `src/generator.rs` - Core generation logic
- `src/openapi.rs` - OpenAPI document parsing
- `test/OpenAPI/` - Test OpenAPI specifications (v2.0, v3.0, v3.1)
- `test/smoke-tests.sh` - Comprehensive integration tests

### Common Navigation Patterns
- **When modifying CLI options**: Check `src/cli.rs` and `src/main.rs`
- **When changing generation logic**: Focus on `src/generator.rs`
- **When adding tests**: Add Rust unit tests beside the relevant module
- **When testing OpenAPI support**: Use files from `test/OpenAPI/v[version]/`

## Common Tasks and Commands

### Build Tasks
- Clean: `cargo clean`
- Build Debug: `make build` or `cargo build`
- Build Release: `make release` or `cargo build --release`
- Full rebuild: `make clean && make build`

### Testing Tasks
- Unit tests: `make test` or `cargo test`
- Smoke tests: `cd test && ./smoke-tests.sh` (4.5 minutes)
- **CRITICAL**: Always run smoke tests after core changes to generation logic

### Development Workflow
1. Make code changes
2. Build: `cargo build --release`
3. Quick test: `cargo run -- test/OpenAPI/v3.0/petstore.json --output /tmp/test --no-logging`
4. Run unit tests: `cargo test`
5. Run smoke tests: `cd test && ./smoke-tests.sh` (ONLY if core generation logic changed)

### Package and Distribution
- CLI tool is packaged as a Cargo binary crate
- Build creates binaries in `target/release/`
- Install globally: `cargo install httpgenerator`

## Known Issues and Workarounds

### Build and Test Issues
- **Network tests**: External URLs may fail in restricted environments; prefer local fixtures

### CLI Tool Limitations
- **External URLs**: May fail due to network restrictions, prefer local files
- **OpenAPI 3.1**: Webhook-only specs generate empty path outputs for parity with the legacy CLI
- **Large specifications**: Some complex OpenAPI specs may take longer to process

### Development Environment
- **PowerShell required**: Smoke tests use PowerShell scripts (`pwsh`)
- **Test directory**: Don't remove `test/` directory as it breaks Makefile

## Dependencies and Technology Stack

### Primary Dependencies
- **Rust stable toolchain**: Required for CLI application
- **clap**: CLI parsing
- **serde_json/serde_yaml**: OpenAPI JSON/YAML parsing
- **reqwest**: HTTP(S) OpenAPI loading

### External Tool Requirements
- **PowerShell Core (pwsh)**: Required for smoke tests
- **Git**: Version control
- **Any Rust-compatible IDE**: VS Code, JetBrains Rider, or similar

## Troubleshooting

### Common Build Errors
- **Missing Rust**: Install Rust using `rustup`
- **Package restore fails**: Run `cargo fetch` or retry `cargo build`
- **Test failures**: Run `cargo test` locally for details

### CLI Tool Errors
- **Network/URL errors**: Use local OpenAPI files from `test/OpenAPI/` directory
- **Empty output**: Check if OpenAPI spec has valid operations and paths

### Performance Issues
- **Slow builds**: First build after clone downloads and compiles Cargo dependencies; subsequent builds are faster
- **Slow tests**: Smoke tests take 4.5 minutes - this is normal, never cancel

## Git Commit Policy

**Never add `Co-authored-by` trailers to git commits.** Commits in this repository should not include any co-author attribution lines.
