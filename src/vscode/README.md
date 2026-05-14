# HTTP File Generator for VS Code

Generate `.http` files from OpenAPI specifications to work with VS Code's REST Client and other HTTP client extensions.

## Features

- Right-click on OpenAPI specification files (.json, .yaml, .yml) in the Explorer
- Access commands from the VS Code Command Palette
- Generate a single HTTP file containing all requests
- Generate multiple HTTP files (one request per file)
- Use the Rust `httpgenerator` CLI through the bundled native binary, a configured executable path, repo-root development builds, or `PATH`

## Requirements

- No .NET SDK is required for normal usage
- [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension (recommended but not required)

## Usage

1. You can use HTTP File Generator in two ways:
   - Right-click on an OpenAPI specification file (.json, .yaml, or .yml) in the VS Code Explorer and select "HTTP File Generator" from the context menu
   - Open the Command Palette (Ctrl+Shift+P or Cmd+Shift+P) and search for "HTTP File Generator"

2. Choose either:
   - "Generate single HTTP file"
   - "Generate multiple HTTP files (one request per file)"

3. If running from the Command Palette, you'll be prompted to select an OpenAPI file from your workspace.

4. You'll be prompted to select an output folder. By default, it will suggest creating a "HttpFiles" subfolder in the same directory as your input file, but you can choose any location.

5. The extension resolves `httpgenerator` in this order:
   - `http-file-generator.executablePath`
   - the bundled native binary
   - repo-root workspace `target\debug` / `target\release` outputs during development
   - `httpgenerator` on `PATH`

6. If the executable cannot be found, reinstall the extension to restore the bundled CLI or point `http-file-generator.executablePath` to an existing `httpgenerator` binary.

## About HTTP Files

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which was later adopted by JetBrains IDEs and Visual Studio 2022.

## Related Projects

This extension is a VS Code host for the Rust [httpgenerator](https://github.com/christianhelle/httpgenerator) CLI.

The CLI in this project was recently migrated to Rust for performance reasons and because I am using Rust more and more these days while working on older hardware. On that hardware, the Rust based CLI currently runs the smoke tests about 60x faster than the legacy .NET tool.

The legacy .NET CLI will still be maintained for compatibility, but new features will only be implemented in the Rust CLI and the .NET tool will eventually be retired.

For more information, visit the [httpgenerator GitHub repository](https://github.com/christianhelle/httpgenerator).

## License

[MIT](https://github.com/christianhelle/httpgenerator/blob/main/LICENSE)
