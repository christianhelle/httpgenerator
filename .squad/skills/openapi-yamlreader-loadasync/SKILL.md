---
name: "openapi-yamlreader-loadasync"
description: "Migrate HTTP File Generator to OpenAPI.NET 3.4 LoadAsync plus Microsoft.OpenApi.YamlReader"
domain: "dependencies"
confidence: "high"
source: "issue #331 / deps-005"
---

## Context

Use this pattern when replacing `Microsoft.OpenApi.Readers` 1.x with `Microsoft.OpenApi` 3.4.x + `Microsoft.OpenApi.YamlReader` in this repo.

## Patterns

### Package scope

- Replace direct `Microsoft.OpenApi.Readers` references with `Microsoft.OpenApi.YamlReader`.
- Keep `Microsoft.OpenApi` pinned to the matching major/minor version (`3.4.0` here).
- Search the repo before upgrading `Microsoft.OpenApi.OData`; if no code uses it, remove it instead of carrying extra migration surface.

### Reader migration

- Switch from `OpenApiStreamReader` to `OpenApiDocument.LoadAsync(...)`.
- When you use the **stream** overload, always pass an explicit format string (`"json"` or `"yaml"`).
- Use `ReadResult.Document` and `ReadResult.Diagnostic` instead of the older `OpenApiDocument` / `OpenApiDiagnostic` property names.
- Call `readerSettings.AddYamlReader()` before loading YAML. Without that registration, YAML parses fail with `Format 'yaml' is not supported.`

### Safe transport pattern

- In this repo, do **not** rely on `OpenApiReaderSettings.HttpClient` during the migration path.
- The stable pattern is:
  1. Download bytes yourself with a repo-controlled `HttpClient`
  2. Detect the format from file extension or first meaningful character
  3. Wrap the bytes in a `MemoryStream`
  4. Call `OpenApiDocument.LoadAsync(stream, format, settings, cancellationToken)`

### Model-surface adaptation

- Expect `IOpenApi*` interfaces in hot paths (`IOpenApiPathItem`, `IOpenApiParameter`, `IOpenApiSchema`, etc.).
- `OpenApiSchema.Type` is now `JsonSchemaType?`, so generation code should map it explicitly instead of calling string methods directly.
- Add null guards when moving old generation helpers onto the new interface-based surface.

### Repo-specific behavior

- Keep OpenAPI 3.1 validation failing unless the corresponding user-facing docs and tests are being updated in the same issue.
- For this repo's staged dependency plan, parsing can support 3.1 while `OpenApiValidator` still throws `OpenApiUnsupportedSpecVersionException("3.1.0")` so `--skip-validation` remains accurate.

## Validation

- `dotnet restore HttpGenerator.sln`
- `dotnet build HttpGenerator.sln --configuration Release`
- `dotnet test HttpGenerator.sln --configuration Release --no-build --filter "FullyQualifiedName~OpenApiDocumentFactoryTests|FullyQualifiedName~OpenApiValidatorTests"`
- `dotnet test HttpGenerator.sln --configuration Release --no-build --filter "FullyQualifiedName!~GenerateCommandTests"`
- Local CLI check: `dotnet run --project src\HttpGenerator\HttpGenerator.csproj --configuration Release -- test\OpenAPI\v3.0\petstore.json --output <temp> --no-logging`

## Anti-Patterns

- Do not assume installing `Microsoft.OpenApi.YamlReader` automatically enables YAML for existing stream-based loads.
- Do not call `ToLowerInvariant()` directly on `OpenApiSchema.Type`; it is no longer a string.
- Do not change the repo's OpenAPI 3.1 validation contract implicitly in the same branch unless the wider CLI/docs fallout is part of scope.
