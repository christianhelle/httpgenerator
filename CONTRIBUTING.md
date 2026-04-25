# Contributing to HTTP File Generator

Thank you for your interest in contributing to the HTTP File Generator project! This document provides guidelines and information to help you contribute effectively.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Patterns and Style](#code-patterns-and-style)
- [Testing Guidelines](#testing-guidelines)
- [Documentation Requirements](#documentation-requirements)
- [Pull Request Guidelines](#pull-request-guidelines)
- [Review Process](#review-process)
- [Continuous Integration](#continuous-integration)

## Getting Started

Before contributing, please:

1. **Fork the repository** and create a new branch for your changes
2. **Check existing issues** to see if your contribution addresses an existing problem
3. **Open an issue** if you're planning significant changes to discuss the approach first
4. **Read through this guide** to understand our development practices

## Development Setup

### Prerequisites

- **Rust stable toolchain** (`rustup`, `cargo`, `rustc`)
- **PowerShell Core (`pwsh`)** for smoke tests
- **VS Code**, **JetBrains Rider**, or another Rust-capable editor
- **Git** for version control

### Setting Up the Development Environment

1. Clone your fork:
   ```bash
   git clone https://github.com/[your-username]/httpgenerator.git
   cd httpgenerator
   ```

2. Build:
   ```bash
   cargo build --release
   ```

3. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

### Project Structure

```
src/
├── main.rs                     # CLI application entry point
├── cli.rs                      # Command-line argument model
├── generator.rs                # Core .http generation logic
├── naming.rs                   # Operation and variable naming helpers
├── openapi.rs                  # OpenAPI loading, parsing, and statistics
└── VSCode/                     # VS Code extension

legacy/
├── HttpGenerator.sln           # Previous .NET solution
└── src/                        # Previous .NET CLI, core library, tests, and VSIX source
```

## Code Patterns and Style

**Use the existing code patterns consistently** throughout the codebase. Here are the key patterns to follow:

### General Rust Guidelines

- Use stable Rust and keep the crate compatible with the current stable toolchain
- Run `cargo fmt` before committing
- Prefer explicit error context with `anyhow::Context`
- Preserve the existing CLI and generated `.http` output contract unless a change is intentional and documented

### Naming Conventions

- **Types and enum variants**: PascalCase (`GeneratorSettings`, `OutputType`)
- **Functions, modules, variables**: snake_case (`generate`, `openapi_path`)
- **Constants**: SCREAMING_SNAKE_CASE when needed

### Code Organization

- Keep CLI parsing in `src/cli.rs`
- Keep OpenAPI loading/statistics in `src/openapi.rs`
- Keep generation behavior in `src/generator.rs`
- Keep naming transformations in `src/naming.rs`

### Example Code Pattern

```rust
pub fn generate(settings: &GeneratorSettings, document: &Document) -> anyhow::Result<Vec<HttpFile>> {
    match settings.output_type {
        OutputType::OneRequestPerFile => generate_multiple_files(settings, document),
        OutputType::OneFile => Ok(vec![HttpFile {
            filename: "Requests.http".to_string(),
            content: generate_single_file(settings, document)?,
        }]),
        OutputType::OneFilePerTag => generate_file_per_tag(settings, document),
    }
}
```

### Error Handling

- Return `Result` from fallible operations
- Add context to file, network, parse, and write errors
- Do not silently ignore malformed input

### Dependencies

- **Minimize external dependencies** - only add what's necessary
- Prefer widely used crates for CLI, HTTP, parsing, and test support

## Testing Guidelines

### Test Structure

- **Unit tests** live beside the Rust modules under `src/`
- Tests should cover generation output shape and edge cases from the legacy .NET suite
- Add smoke-test coverage for end-to-end CLI scenarios when behavior changes

### Testing Patterns

```rust
#[test]
fn generates_one_file_with_qualified_parameters() {
    let files = generate(&settings(OutputType::OneFile), &document()).unwrap();

    assert_eq!(files[0].filename, "Requests.http");
    assert!(files[0].content.contains("@GetPet_id = 0"));
}
```

### Test Requirements

- **All new functionality** must include comprehensive tests
- **Use clear assertions** that verify generated filenames and `.http` content
- **Test both success and failure scenarios**
- **Include edge cases** and boundary conditions
- **Maintain high code coverage** (current coverage is maintained via CodeCov)

### Sample Data

- Add focused Rust unit tests beside the module under test
- Place sample OpenAPI files in the `test/OpenAPI/` directory
- Follow existing naming patterns for consistency

## Documentation Requirements

**Keep README up to date with changes** - this is a critical requirement for all contributions.

### README.md Updates

When making changes that affect users:

1. **Update usage examples** if CLI options change
2. **Add new features** to the feature list
3. **Update installation instructions** if package names change
4. **Modify examples** to reflect new functionality
5. **Update version numbers** in examples when appropriate

### Code Documentation

- **Rustdoc documentation** for public APIs when the behavior is not obvious
- **Inline comments** for complex business logic
- **README files** in individual projects when appropriate
- **Update CHANGELOG.md** following existing format

### Examples of Required Updates

- New CLI option → Update usage section in README.md
- New output format → Add examples of generated files
- Breaking changes → Update migration guide
- Performance improvements → Update benchmarks if mentioned

## Pull Request Guidelines

**PR descriptions must be as verbose as possible** - this is a key requirement.

### PR Title Format

Use clear, descriptive titles:
- `feat: Add support for OpenAPI 3.1 specifications`
- `fix: Handle empty response schemas correctly`
- `docs: Update installation instructions for Cargo`
- `test: Add integration tests for Azure authentication`

### Required PR Description Content

Your PR description **must** include:

1. **Detailed Description**
   ```markdown
   ## Description
   
   Explain what this PR does, why it's needed, and how it solves the problem.
   Include context about the issue being addressed.
   ```

2. **Changes Made**
   ```markdown
   ## Changes Made
   
   - List all significant changes
   - Include new files added
   - Mention modified existing functionality
   - Note any breaking changes
   ```

3. **Testing**
   ```markdown
   ## Testing
   
   - Describe test scenarios covered
   - Include sample inputs/outputs if applicable
   - Mention any manual testing performed
   ```

4. **Example OpenAPI Specifications**
   Include relevant OpenAPI specs that demonstrate the changes:
   ```yaml
   swagger: '2.0'
   info:
     title: Example API
     version: v1.0.0
   paths:
     '/example/{id}':
       get:
         summary: Get example
         parameters:
           - name: id
             in: path
             required: true
             type: string
   ```

5. **Generated Output Examples**
   Show the .http files that would be generated:
   ```http
   ### Request: GET /example/{id}
   ### Summary: Get example
   
   @id = example-id
   
   GET https://api.example.com/example/{{id}}
   Content-Type: application/json
   ```

6. **Impact Assessment**
   ```markdown
   ## Impact
   
   - Breaking changes: Yes/No
   - Documentation updates needed: Yes/No
   - New dependencies: List any new packages
   ```

### PR Checklist

Before submitting, ensure:

- [ ] **Code follows existing patterns** and style guidelines
- [ ] **Tests are included** and passing
- [ ] **Documentation is updated** (README.md, code comments)
- [ ] **CHANGELOG.md is updated** following existing format
- [ ] **No breaking changes** without discussion (or clearly documented)
- [ ] **Build passes** locally
- [ ] **All tests pass** locally

## Review Process

### What to Expect

1. **Automated Checks**: GitHub Actions will run builds and tests
2. **Code Review**: Maintainers will review for code quality and adherence to guidelines
3. **Feedback**: You may receive requests for changes or clarifications
4. **Approval**: Once approved, your PR will be merged

### Review Criteria

- **Code Quality**: Follows established patterns and best practices
- **Test Coverage**: Adequate tests for new functionality
- **Documentation**: README and code documentation updated appropriately
- **Backward Compatibility**: Existing functionality remains intact
- **Performance**: No significant performance regressions

## Continuous Integration

The project uses GitHub Actions for CI/CD:

- **Build Workflow**: Compiles the project across multiple platforms
- **Test Workflow**: Runs unit and integration tests
- **Smoke Tests**: Validates basic functionality
- **Code Quality**: SonarCloud analysis for code quality metrics
- **Coverage**: CodeCov integration for test coverage reporting

### Build Requirements

Your PR must:
- **Build successfully** on all target platforms (.NET 8.0)
- **Pass all existing tests**
- **Maintain or improve code coverage**
- **Pass static analysis** (SonarCloud quality gate)

### Dependency Management

- **Renovate** automatically creates PRs for dependency updates
- **Review dependency changes** carefully for breaking changes
- **Keep dependencies up to date** but prioritize stability

## Getting Help

If you need assistance:

1. **Check existing issues** for similar problems
2. **Review the README** for usage examples
3. **Look at existing code** for patterns and examples
4. **Open a discussion** for questions about design decisions
5. **Contact maintainers** through GitHub issues

## License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project.
