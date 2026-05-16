# HTTP File Generator for VS Code

Generate `.http` files from OpenAPI specifications to work with VS Code's REST Client and other HTTP client extensions.

## Features

- Right-click on OpenAPI specification files (`.json`, `.yaml`, `.yml`) in the Explorer
- Access commands from the VS Code Command Palette
- Generate a single HTTP file containing all requests
- Generate multiple HTTP files (one request per file)
- Use the Rust `httpgenerator` CLI through the bundled native binary, a configured executable path, repo-root development builds, or `PATH`

## Installation

Install the VS Code extension from the Visual Studio Marketplace or from a platform-targeted `.vsix` package. VS Code extension packages bundle the native Rust CLI for their target platform.

## Requirements

- No .NET SDK is required for normal usage
- [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension (recommended but not required)

## Executable Resolution

The extension resolves the `httpgenerator` executable in this order:

1. `http-file-generator.executablePath`
2. The bundled binary inside the installed extension
3. Repo-root workspace `target\debug` / `target\release` outputs during local development
4. `httpgenerator` on `PATH`

If `http-file-generator.executablePath` is set but invalid, the extension stops and asks you to fix that setting instead of falling back silently.

The VS Code extension is a bundled-binary distribution. It does not install the CLI from crates.io or the legacy `.NET Tool`. A Cargo-installed or release-downloaded `httpgenerator` binary is only used when you point `http-file-generator.executablePath` to it or make it discoverable on `PATH`.

## Usage

1. Use HTTP File Generator in one of these ways:
   - Right-click an OpenAPI specification file (`.json`, `.yaml`, or `.yml`) in the VS Code Explorer and select **HTTP File Generator**
   - Open the Command Palette (`Ctrl+Shift+P` / `Cmd+Shift+P`) and search for **HTTP File Generator**
2. Choose either:
   - **Generate single HTTP file**
   - **Generate multiple HTTP files (one request per file)**
3. If you start from the Command Palette, select an OpenAPI file from your workspace.
4. Choose an output folder. By default, the extension suggests an `HttpFiles` subfolder beside the input file.

## Local Development

For local extension development, you can either set `http-file-generator.executablePath` explicitly or build the Rust CLI from the repository root so the extension can discover the repo-root workspace `target\debug` / `target\release` outputs.

## About HTTP Files

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which was later adopted by JetBrains IDEs and Visual Studio 2022.

## Related Projects

This extension is a thin VS Code host for the Rust-native [httpgenerator](https://github.com/christianhelle/httpgenerator) CLI.

For more information, visit the [httpgenerator GitHub repository](https://github.com/christianhelle/httpgenerator).

## License

[MIT](https://github.com/christianhelle/httpgenerator/blob/main/LICENSE)
