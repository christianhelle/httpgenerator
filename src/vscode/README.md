# HTTP File Generator for VS Code

Generate `.http` files from OpenAPI specifications to work with VS Code's REST Client and other HTTP client extensions.

## Features

- Right-click on OpenAPI specification files (.json, .yaml, .yml) in the Explorer
- Access commands from the VS Code Command Palette
- Generate a single HTTP file containing all requests
- Generate multiple HTTP files (one request per file)
- Uses the Rust `httpgenerator` CLI without requiring the .NET SDK or .NET tool
- Automatically downloads and caches the CLI when it is not configured or available on `PATH`

## Requirements

- [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension (recommended but not required)

The extension downloads the platform-specific Rust CLI from GitHub Releases on first use. It works offline after the CLI has been downloaded once.

## Configuration

- `http-file-generator.executablePath`: Optional path to a custom `httpgenerator` executable. Leave empty to search `PATH` and then use the extension-managed cached CLI.

## Commands

- `HTTP File Generator: Generate single HTTP file`
- `HTTP File Generator: Generate multiple HTTP files (one request per file)`
- `HTTP File Generator: Reset CLI` deletes the cached extension-managed CLI so it can be downloaded again.
- `HTTP File Generator: Show CLI Path` displays the executable currently used by the extension.

## Usage

1. You can use HTTP File Generator in two ways:
   - Right-click on an OpenAPI specification file (.json, .yaml, or .yml) in the VS Code Explorer and select "HTTP File Generator" from the context menu
   - Open the Command Palette (Ctrl+Shift+P or Cmd+Shift+P) and search for "HTTP File Generator"

2. Choose either:
   - "Generate single HTTP file"
   - "Generate multiple HTTP files (one request per file)"

3. If running from the Command Palette, you'll be prompted to select an OpenAPI file from your workspace.

4. The extension resolves the CLI by checking `http-file-generator.executablePath`, then `PATH`, then the cached binary in VS Code global storage. If needed, it downloads the Rust CLI from GitHub Releases and shows progress while downloading.

5. You'll be prompted to select an output folder. By default, it will suggest creating a "HttpFiles" subfolder in the same directory as your input file, but you can choose any location.

If automatic download fails, use `HTTP File Generator: Reset CLI` to retry or set `http-file-generator.executablePath` to a manually installed `httpgenerator` binary.

## About HTTP Files

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which was later adopted by JetBrains IDEs and Visual Studio 2022.

## Related Projects

This extension is a VS Code wrapper around the Rust [httpgenerator](https://github.com/christianhelle/httpgenerator) CLI.

For more information, visit the [httpgenerator GitHub repository](https://github.com/christianhelle/httpgenerator).

## License

[MIT](https://github.com/christianhelle/httpgenerator/blob/main/LICENSE)
