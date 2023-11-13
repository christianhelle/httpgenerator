# HTTP File Generator
Generate .http files from OpenAPI specifications

`.http` files were made populate by the Visual Studio Code extension REST Client, which then was adopted by the JetBrains IDE's, and later on Visual Studio 2022

## Installation

This is tool is distrubuted as a .NET Tool on NuGet.org

To install, simply use the following command

```bash
dotnet tool install --global httpgenerator
```

## Usage

```
USAGE:
    httpgenerator [URL or input file] [OPTIONS]

EXAMPLES:
    httpgenerator ./openapi.json
    httpgenerator https://petstore3.swagger.io/api/v3/openapi.yaml
    httpgenerator ./openapi.json --output ./

ARGUMENTS:
    [URL or input file]    URL or file path to OpenAPI Specification file

OPTIONS:
                             DEFAULT                                         
    -h, --help                          Prints help information              
    -o, --output <OUTPUT>    ./         Output directory                     
        --no-logging                    Don't log errors or collect telemetry
```