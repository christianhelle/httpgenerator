# HTTP File Generator for VS Code

Generate `.http` files from OpenAPI specifications to work with VS Code's REST Client and other HTTP client extensions.

## Features

- Right-click on OpenAPI specification files (.json, .yaml, .yml) in the Explorer
- Access commands from the VS Code Command Palette
- Generate a single HTTP file containing all requests
- Generate multiple HTTP files (one request per file)
- Automatically installs the required .NET Tool if not present

## Requirements

- [.NET 6.0 SDK](https://dotnet.microsoft.com/download/dotnet/6.0) or later
- [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client) extension (recommended but not required)

## Usage

1. You can use HTTP File Generator in two ways:
   - Right-click on an OpenAPI specification file (.json, .yaml, or .yml) in the VS Code Explorer and select "HTTP File Generator" from the context menu
   - Open the Command Palette (Ctrl+Shift+P or Cmd+Shift+P) and search for "HTTP File Generator"

2. Choose either:
   - "Generate single HTTP file"
   - "Generate multiple HTTP files (one request per file)"

3. If running from the Command Palette, you'll be prompted to select an OpenAPI file from your workspace.

4. If the `httpgenerator` .NET tool is not installed, you'll be prompted to install it.

The generated HTTP files will be created in the same directory as the specification file.

## About HTTP Files

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which was later adopted by JetBrains IDEs and Visual Studio 2022.

## Related Projects

This extension is a VS Code wrapper around the [httpgenerator](https://github.com/christianhelle/httpgenerator) .NET Tool.

For more information, visit the [httpgenerator GitHub repository](https://github.com/christianhelle/httpgenerator).

## License

[MIT](https://github.com/christianhelle/httpgenerator/blob/main/LICENSE)
