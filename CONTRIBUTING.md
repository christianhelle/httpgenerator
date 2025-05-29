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

- **.NET 8.0 SDK** or later
- **Visual Studio 2022**, **VS Code**, or **JetBrains Rider**
- **Git** for version control

### Setting Up the Development Environment

1. Clone your fork:
   ```bash
   git clone https://github.com/[your-username]/httpgenerator.git
   cd httpgenerator
   ```

2. Restore dependencies and build:
   ```bash
   dotnet restore
   dotnet build --configuration Release
   ```

3. Run tests to ensure everything works:
   ```bash
   dotnet test --configuration Release
   ```

### Project Structure

```
src/
├── HttpGenerator/              # CLI application
├── HttpGenerator.Core/         # Core library with generation logic
├── HttpGenerator.Tests/        # Unit and integration tests
├── HttpGenerator.VSIX/         # Visual Studio extension
└── VSCode/                     # VS Code extension
```

## Code Patterns and Style

**Use the existing code patterns consistently** throughout the codebase. Here are the key patterns to follow:

### General C# Guidelines

- **Target Framework**: Use .NET Standard 2.0 for libraries, .NET 8.0 for applications
- **Nullable Reference Types**: Enabled - use nullable annotations appropriately
- **File-scoped namespaces**: Use file-scoped namespace declarations (`namespace HttpGenerator.Core;`)
- **Top-level statements**: Use in Program.cs where appropriate

### Naming Conventions

- **Classes**: PascalCase (`HttpFileGenerator`, `GeneratorSettings`)
- **Methods**: PascalCase (`GenerateAsync`, `CreateFromFile`)
- **Properties**: PascalCase (`BaseUrl`, `OutputType`)
- **Fields**: camelCase with underscore prefix for private fields (`_httpClient`)
- **Parameters**: camelCase (`settings`, `document`)
- **Local variables**: camelCase (`generateCode`, `serverUrl`)

### Code Organization

- **Static classes** for utility functions (e.g., `HttpFileGenerator`, `OpenApiDocumentFactory`)
- **Extension methods** in dedicated classes (e.g., `StringExtensions`)
- **Async/await patterns** for I/O operations
- **Factory patterns** for object creation (e.g., `OpenApiDocumentFactory`)

### Example Code Pattern

```csharp
namespace HttpGenerator.Core;

public static class HttpFileGenerator
{
    public static async Task<GeneratorResult> Generate(GeneratorSettings settings)
    {
        var document = await OpenApiDocumentFactory.CreateAsync(settings.OpenApiPath);
        
        return settings.OutputType switch
        {
            OutputType.OneRequestPerFile => GenerateMultipleFiles(settings, document),
            OutputType.OneFile => GenerateSingleFile(settings, document),
            OutputType.OneFilePerTag => GenerateFilePerTag(settings, document),
            _ => throw new ArgumentOutOfRangeException(nameof(settings.OutputType))
        };
    }
}
```

### Error Handling

- Use **meaningful exceptions** with descriptive messages
- Implement **validation** at public API boundaries
- Use **Result patterns** where appropriate for error handling
- Follow existing patterns in `OpenApiValidator` and `GenerateCommand`

### Dependencies

- **Minimize external dependencies** - only add what's absolutely necessary
- Use **Microsoft.Extensions.*** packages for dependency injection, logging, etc.
- Follow existing patterns with **NSwag** and **Microsoft.OpenApi** libraries

## Testing Guidelines

### Test Structure

- **Unit tests** in `HttpGenerator.Tests` project
- **Test classes** should mirror the structure of the code being tested
- **Theory/InlineData** for parameterized tests (follow `SwaggerPetstoreTests` pattern)

### Testing Patterns

```csharp
[Theory]
[InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
[InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneFile)]
public async Task Can_Generate_Code(Samples version, string filename, OutputType outputType)
{
    var generateCode = await GenerateCode(version, filename, outputType);

    using var scope = new AssertionScope();
    generateCode.Should().NotBeNull();
    generateCode.Files.Should().NotBeNullOrEmpty();
    generateCode.Files
        .All(file => file.Content.Contains("client.assert"))
        .Should()
        .BeTrue();
}
```

### Test Requirements

- **All new functionality** must include comprehensive tests
- **Use FluentAssertions** for readable assertions
- **Test both success and failure scenarios**
- **Include edge cases** and boundary conditions
- **Maintain high code coverage** (current coverage is maintained via CodeCov)

### Sample Data

- Add new test samples to `HttpGenerator.Tests/Resources/Samples.cs`
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

- **XML documentation** for public APIs
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
- `docs: Update installation instructions for .NET 8`
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