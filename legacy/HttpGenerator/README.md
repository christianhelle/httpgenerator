# HttpGenerator .NET CLI (legacy)

This project contains the legacy .NET CLI kept in the repository as a compatibility oracle during the Rust rewrite.

## Current product surface

- **Primary CLI**: the Rust `httpgenerator` binary built from `crates/httpgenerator-cli`
- **VS Code**: a thin host that resolves and invokes the Rust CLI
- **Visual Studio 2022**: a thin VSIX host that shells out to `httpgenerator.exe`
- **Release artifacts**: platform-specific Rust CLI archives, platform-targeted VS Code `.vsix` packages, and a bundled Visual Studio `.vsix`

## When to use this project

Use this .NET project when you need to compare legacy behavior against the Rust implementation or inspect the original command surface during migration work.

```powershell
dotnet run --project legacy\HttpGenerator\HttpGenerator.csproj -- test\OpenAPI\v3.0\petstore.json --output .\HttpFiles --no-logging
```

## Azure auth

The supported command-line surface still includes `--azure-scope` and `--azure-tenant-id`. In the Rust implementation, those values are passed through to the CLI, which acquires tokens from Azure CLI or Azure Developer CLI during generation.

## Telemetry and privacy

The rewrite keeps redacted feature and error recording behind an internal telemetry seam. `--no-logging` disables collection and suppresses the support key header.

## See also

For current installation and user-facing usage guidance, see the repository root [README](../../README.md).
