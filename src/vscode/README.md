# HTTP File Generator for VS Code

Generate `.http` files from OpenAPI specifications to work with VS Code's REST Client and other HTTP client extensions.

## Features

- Right-click on OpenAPI specification files (.json, .yaml, .yml) in the Explorer
- Access commands from the VS Code Command Palette
- Generate a single HTTP file containing all requests
- Generate multiple HTTP files (one request per file)
- Bundles the native [httpgenerator](https://github.com/christianhelle/httpgenerator) Rust CLI — no additional runtime required

## Requirements

No additional runtime installation required. The extension ships with a bundled native `httpgenerator` binary for each supported platform.

For development or to use a custom build, set the `http-file-generator.executablePath` setting to point to any `httpgenerator` binary.

## Usage

1. You can use HTTP File Generator in two ways:
   - Right-click on an OpenAPI specification file (.json, .yaml, or .yml) in the VS Code Explorer and select "HTTP File Generator" from the context menu
   - Open the Command Palette (Ctrl+Shift+P or Cmd+Shift+P) and search for "HTTP File Generator"

2. Choose either:
   - "Generate single HTTP file"
   - "Generate multiple HTTP files (one request per file)"

3. If running from the Command Palette, you'll be prompted to select an OpenAPI file from your workspace.

4. You'll be prompted to select an output folder. By default, it will suggest creating a "HttpFiles" subfolder in the same directory as your input file.

## Executable Resolution

The extension resolves the `httpgenerator` binary using the following priority order:

1. **`http-file-generator.executablePath` setting** — when set, this path is used directly. If the file does not exist, the extension shows an error and stops instead of falling back silently.
2. **Bundled binary** — the native binary included in the platform-targeted `.vsix` package.
3. **Repo development build** — `target/release/httpgenerator` or `target/debug/httpgenerator` relative to the extension directory (covers the `src/vscode` → repo root layout when running from source).
4. **PATH** — `httpgenerator` resolved from your system `PATH`.

## Development

When running or debugging the extension from source:

1. Build the Rust CLI: `cargo build -p httpgenerator`
2. Open `src/vscode` in VS Code and press **F5** to launch the Extension Development Host.
3. The extension will automatically discover `target/debug/httpgenerator` from the repo root.

Alternatively, set `http-file-generator.executablePath` in your VS Code settings to point to a specific `httpgenerator` binary.

## About HTTP Files

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which was later adopted by JetBrains IDEs and Visual Studio 2022.

## Related Projects

This extension is a VS Code host for the [httpgenerator](https://github.com/christianhelle/httpgenerator) Rust CLI.

For more information, visit the [httpgenerator GitHub repository](https://github.com/christianhelle/httpgenerator).

## License

[MIT](https://github.com/christianhelle/httpgenerator/blob/main/LICENSE)
