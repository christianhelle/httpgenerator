# HTTP File Generator for VS Code

Generate HTTP files from OpenAPI specifications to easily test your APIs using VS Code's REST Client extension.

## Features

* Generate HTTP files from OpenAPI specification files (JSON or YAML)
* Choose between:
  * Generate a single HTTP file containing all requests
  * Generate multiple HTTP files (one request per file)
* Access via explorer context menu or command palette

## Requirements

* [.NET SDK](https://dotnet.microsoft.com/download) installed
* The extension will install the `httpgenerator` .NET tool automatically if not already installed

## Usage

### From Explorer Context Menu

1. Right-click on an OpenAPI specification file (JSON or YAML) in Explorer
2. Select "HTTP File Generator" > "Generate single HTTP file" or "Generate multiple HTTP files (one request per file)"
3. Enter the output directory when prompted
4. HTTP files will be generated in the specified directory

### From Command Palette

1. Open Command Palette (Ctrl+Shift+P / Cmd+Shift+P)
2. Type "HTTP File Generator" and select the desired command
3. Select an OpenAPI specification file from the list
4. Enter the output directory when prompted
5. HTTP files will be generated in the specified directory

## Extension Settings

This extension doesn't add any VS Code settings.

## Known Issues

None yet! Please report any issues you encounter on the [GitHub repository](https://github.com/christianhelle/httpgenerator).

## Release Notes

### 0.1.0

Initial release with basic functionality:
* Generate single or multiple HTTP files from OpenAPI specs
* Context menu integration
* Command palette integration