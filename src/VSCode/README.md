# HTTP File Generator for VS Code

Generate `.http` files from OpenAPI specifications to work with VS Code's REST Client and other HTTP client extensions.

## Features

- Right-click on OpenAPI specification files (.json, .yaml, .yml) in the Explorer
- Access commands from the VS Code Command Palette
- Generate a single HTTP file containing all requests
- Generate multiple HTTP files (one request per file)
- Resolves the Rust `httpgenerator` executable from extension settings, a bundled binary, workspace build output, or PATH

## Requirements

- A Rust `httpgenerator` executable available through one of these locations:
  - `http-file-generator.executablePath`
  - a bundled `bin/httpgenerator` binary inside the extension
  - a workspace build output under `target/debug` or `target/release`
  - `httpgenerator` on your system `PATH`
- [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension (recommended but not required)

## Usage

1. You can use HTTP File Generator in two ways:
   - Right-click on an OpenAPI specification file (.json, .yaml, or .yml) in the VS Code Explorer and select "HTTP File Generator" from the context menu
   - Open the Command Palette (Ctrl+Shift+P or Cmd+Shift+P) and search for "HTTP File Generator"

2. Choose either:
   - "Generate single HTTP file"
   - "Generate multiple HTTP files (one request per file)"

3. If running from the Command Palette, you'll be prompted to select an OpenAPI file from your workspace.

4. If the extension cannot find the Rust `httpgenerator` executable, set `http-file-generator.executablePath` in VS Code settings or make `httpgenerator` available on `PATH`.

5. You'll be prompted to select an output folder. By default, it will suggest creating a "HttpFiles" subfolder in the same directory as your input file, but you can choose any location.

## About HTTP Files

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which was later adopted by JetBrains IDEs and Visual Studio 2022.

## Related Projects

This extension is a VS Code host for the Rust-based [httpgenerator](https://github.com/christianhelle/httpgenerator) CLI.

For more information, visit the [httpgenerator GitHub repository](https://github.com/christianhelle/httpgenerator).

## License

[MIT](https://github.com/christianhelle/httpgenerator/blob/main/LICENSE)
