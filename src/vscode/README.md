# HTTP File Generator for VS Code

Generate `.http` files from OpenAPI specifications for REST Client and other HTTP client extensions.

## Features

- Right-click OpenAPI files (`.json`, `.yaml`, `.yml`) from Explorer
- Run generation from the Command Palette
- Generate one consolidated `.http` file
- Generate one request per `.http` file
- Uses the Rust `httpgenerator` CLI

## Executable resolution

The extension resolves the CLI in this order:

1. `http-file-generator.executablePath`
2. Bundled platform binary from the extension package
3. Repo-root development builds: `target/debug` then `target/release`
4. `httpgenerator` on `PATH`

If `http-file-generator.executablePath` is set but invalid, the extension fails fast and shows an error.

## Requirements

- VS Code 1.61 or later
- [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) (recommended)

No .NET SDK or .NET global tool installation is required for normal extension use.

## Usage

1. Use the extension from:
   - Explorer context menu on an OpenAPI file, or
   - Command Palette (`HTTP File Generator`)
2. Choose output mode:
   - Generate single HTTP file
   - Generate multiple HTTP files (one request per file)
3. Pick or accept the suggested output folder (`HttpFiles` under the input file directory).

## Configuration

- `http-file-generator.executablePath`: Optional absolute path override for the `httpgenerator` executable.

## Local development workflow

For local extension testing in this repository:

1. Build the Rust CLI (`cargo build --release -p httpgenerator`)
2. Package the extension from repo root (`src\vscode\build.ps1` or `src/vscode/build.sh`)
3. Launch an Extension Development Host and run generation commands

During development, the extension can discover repo-root CLI outputs from `target/debug` or `target/release`.

## License

[MIT](https://github.com/christianhelle/httpgenerator/blob/main/LICENSE)
