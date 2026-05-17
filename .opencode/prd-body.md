## Problem Statement

The Visual Studio extension (HttpGenerator.VSIX) currently depends on the HttpGenerator.Core .NET library for all code generation logic. This creates a tightly coupled architecture where the extension must ship with the .NET library as a project reference, increasing the extension's binary size and dependency surface. Meanwhile, the VS Code extension has already been migrated to use the Rust-based httpgenerator CLI, achieving a decoupled, platform-independent architecture with a thin host layer.

The VS extension faces the same opportunity: replace the HttpGenerator.Core dependency with the Rust CLI to achieve the same architectural benefits — smaller dependency footprint, faster code generation, and architectural parity with the VS Code extension.

## Solution

Replace the HttpGenerator.Core library invocation in the Visual Studio extension with a Rust httpgenerator CLI execution layer. The CLI will be downloaded on first run (or updated check) using an embedded install.ps1 script, cached in the user's %APPDATA% directory, and invoked as an external process to generate .http files from OpenAPI specifications.

## User Stories

1. As a Visual Studio user, I want the extension to automatically download the httpgenerator CLI on first use, so that I can generate .http files without manual setup.
2. As a Visual Studio user, I want the CLI to be cached in my %APPDATA% directory, so that it persists across extension updates and doesn't need to be re-downloaded every time.
3. As a Visual Studio user, I want the extension to check for CLI updates on every activation, so that I always have the latest version without manual intervention.
4. As a Visual Studio user, I want to see a progress indicator while the CLI is running, so that I know the extension is working and hasn't frozen.
5. As a Visual Studio user, I want to see the generated .http files opened in the editor after generation completes, so that I can immediately review and use them.
6. As a Visual Studio user, I want the same generation options I currently have (OpenAPI file path, base URL, content type, authorization header, output type), so that my existing workflow is preserved.
7. As a Visual Studio user, I want to see CLI error output in the VS Output window, so that I can debug generation issues.
8. As a Visual Studio user, if the CLI download fails, I want to see an error dialog with an option to download the latest version manually, so that I can recover from failures.
9. As a Visual Studio user, if I cancel the CLI download, I want to be shown a message and have the option to retry later, so that I'm not forced into a broken state.
10. As a Visual Studio user, I want the Azure authentication I currently use to continue working, so that I can generate .http files for Azure-protected APIs without losing this functionality.
11. As a Visual Studio user, I want the extension to verify the downloaded CLI binary by running --version, so that I can be confident the binary is valid and not corrupted.
12. As a Visual Studio user, I want the extension to work without embedding the CLI binary in the VSIX, so that the extension download is smaller and the CLI can be updated independently.
13. As a Visual Studio user, I want the generation dialog to look and behave the same as before, so that I don't need to learn a new interface.
14. As a developer, I want the HttpGenerator.Core library dependency to be completely removed from the VSIX project, so that the extension has no .NET library dependencies for code generation.
15. As a developer, I want the Azure authentication code to be moved into the VSIX project, so that it remains available after removing HttpGenerator.Core.
16. As a developer, I want the CLI to be invoked via ProcessStartInfo with a string array of arguments, so that argument escaping is handled correctly by .NET without manual escaping.
17. As a developer, I want to parse the CLI's plain text output for the "Files written successfully:" section, so that I can extract the generated file paths for opening in the editor.
18. As a developer, I want the pinned CLI version to be stored in the VSIX manifest metadata, so that it is centralized and discoverable.
19. As a developer, I want the build script to remain unchanged, so that VSIX publishing infrastructure doesn't need modification.
20. As a developer, I want the CLI install script to be embedded as a resource in the VSIX, so that installation works even without network access to the install script URL.
21. As a Visual Studio user, I want the output type options (OneFile and OneRequestPerFile) to work exactly as before, so that my generation preferences are preserved.
22. As a Visual Studio user, I want the CLI to run on a background thread with progress UI, so that the Visual Studio UI remains responsive during generation.
23. As a Visual Studio user, I want to see the count of generated files in the result, so that I know how many files were created.
24. As a Visual Studio user, I want the extension version to be bumped to 1.1.0, so that I can see the update is available in Visual Studio.

## Implementation Decisions

### Execution Model
- The CLI will be spawned as an external process using ProcessStartInfo with UseShellExecute = false and a string array of arguments.
- The CLI will be invoked on a background thread with progress UI to avoid blocking the VS UI thread.
- No FFI or P/Invoke — the extension communicates with the CLI purely through process spawning and stdout/stderr capture.

### Binary Distribution
- The CLI binary will NOT be embedded in the VSIX package.
- The CLI will be downloaded on first activation using an embedded install.ps1 script.
- The install.ps1 script will be invoked with -Version (pinned version), -InstallDir (target cache directory), and -AddToPath $false.
- The binary will be cached at %APPDATA%\httpgenerator\{pinned-version}\httpgenerator.exe.
- The pinned version will be stored in the VSIX manifest metadata (source.extension.vsixmanifest).

### Version Management
- The extension will check for CLI updates on every activation.
- The check will use a remote version file from christianhelle.com to get the latest version.
- If the cached version differs from the remote version, the extension will download and install the new version.
- The download URL will be constructed from the pinned version string (GitHub releases URL pattern).

### Binary Verification
- After download, the extension will verify the binary by running httpgenerator.exe --version and comparing the output to the expected version.
- If verification fails, the extension will show an error dialog.

### Output Handling
- The CLI will be invoked with stdout redirected and stderr redirected.
- The extension will parse the plain text stdout for the "Files written successfully:" section to extract generated file paths.
- The extension will also scan the output folder for .http files as a backup.
- stderr will be written to a VS Output window panel for debugging.
- The CLI's non-terminal stdout auto-detection will produce plain text output (no ANSI codes or emoji).

### UI Changes
- The existing GenerateDialog will be kept unchanged — only the OnOk_Click handler will be modified.
- The HttpFileGenerator.Generate() call will be replaced with an async CLI execution call.
- After CLI completes successfully, the generated .http files will be opened in the VS editor.
- A progress indicator will be shown during CLI execution.

### Azure Authentication
- The current Azure authentication flow will be preserved.
- The AzureEntraID class will be moved from HttpGenerator.Core into the VSIX project.
- The acquired Azure token will be passed to the CLI via the --authorization-header argument.

### HttpGenerator.Core Dependency
- The project reference to HttpGenerator.Core will be removed entirely from the VSIX project.
- All HttpFileGenerator code will be removed from the extension.
- The AzureEntraID class will be the only code moved from HttpGenerator.Core.

### Build and CI
- No changes to the build script (publish.ps1) — CLI download happens at activation time, not build time.
- No CI/CD changes needed — the VSIX build and publish pipeline remains unchanged.
- The pinned CLI version will be stored in the VSIX manifest metadata.

### Version Bump
- The VSIX version will be bumped to 1.1.0 (minor version bump).

### Error Handling
- If CLI download fails: show error dialog with stderr content and option to download latest version.
- If user cancels download: show message, leave command enabled, retry on next activation.
- If CLI execution fails: show error dialog with stderr content.

## Testing Decisions

### What Makes a Good Test
Tests should focus on external behavior — what the user sees and experiences — not internal implementation details. Tests should verify:
- CLI download and caching behavior (given a missing cache, CLI is downloaded; given a cached CLI, it is reused)
- CLI argument mapping (given input settings, the correct CLI arguments are produced)
- Output parsing (given CLI stdout, the correct file paths are extracted)
- Error handling (given a failed CLI execution, the correct error dialog is shown)
- Version update detection (given a newer remote version, the CLI is updated)

### Modules to Test
1. **CLI Resolution Module** — Tests the download, cache, and version check logic. This is the most complex new module and should have the most comprehensive test coverage.
2. **Argument Builder Module** — Tests the mapping from extension settings to CLI arguments. This should have unit tests for each setting mapping.
3. **Output Parser Module** — Tests the parsing of CLI stdout for file paths and status. This should have unit tests for various output formats.
4. **Azure Authentication Module** — Tests the moved AzureEntraID class to ensure functionality is preserved.

### Testing Approach
- Unit tests for the argument builder and output parser modules (pure logic, no dependencies).
- Integration tests for the CLI resolution module (requires file system and network mocking).
- Manual testing for the UI flow (progress indicator, error dialogs, file opening).

## Out of Scope

- Adding new CLI features to the VS extension UI (e.g., OneFilePerTag output type, custom headers, IntelliJ test assertions). Only existing settings are mapped.
- Adding CLI auto-update settings (e.g., disable auto-update, check interval). The extension always checks on activation.
- Supporting offline mode. The extension requires network access to download the CLI.
- Adding a settings page for CLI configuration. The CLI is managed entirely by the extension.
- Migrating the .NET CLI tool (HttpGenerator project) to use the Rust CLI. This PRD only covers the Visual Studio extension.
- Adding telemetry or usage analytics for the CLI invocation.

## Further Notes

### Architecture Parity
This migration brings the Visual Studio extension to architectural parity with the VS Code extension (PR #399). Both extensions now use the same pattern: a thin host layer that resolves, downloads, and invokes the Rust httpgenerator CLI as an external process.

### Pinned Version
The pinned CLI version should be set to a stable release tag (e.g., v1.0.0) and updated when a new CLI release is ready. The pinned version is stored in source.extension.vsixmanifest and used for both the download URL and the cache directory name.

### Embedded install.ps1
The install.ps1 script from docs/install.ps1 is embedded as an EmbeddedResource in the VSIX project. At runtime, it is extracted to a temp directory, invoked with the pinned version and target directory, and the temp directory is cleaned up. This approach reuses the same installation logic as the public installer, ensuring consistency.

### Module Boundaries
The key deep modules that will be created:
1. **HttpGeneratorCli** — Encapsulates all CLI lifecycle: resolution, download, cache, version check, execution. Interface: GetExecutablePath(), ExecuteAsync(settings, cancellationToken).
2. **CliArgumentBuilder** — Maps GeneratorSettings to CLI arguments. Interface: BuildArguments(settings). Pure function, no side effects.
3. **CliOutputParser** — Parses CLI stdout/stderr. Interface: ParseSuccessOutput(stdout), ParseErrorOutput(stderr). Pure function, no side effects.
4. **CliVersionManager** — Handles version comparison and update detection. Interface: GetCachedVersion(), GetRemoteVersion(), NeedsUpdate().
5. **CliDownloader** — Handles the download and extraction using the embedded install.ps1. Interface: DownloadAsync(version, targetDir, cancellationToken).
