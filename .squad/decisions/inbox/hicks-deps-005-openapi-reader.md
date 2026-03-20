# Hicks deps-005 — OpenAPI reader pipeline decisions

**Date:** 2026-03-20  
**Issue:** #331 (`deps-005`)

## Decisions

1. **Remove `Microsoft.OpenApi.OData` from the CLI project.**  
   A repo-wide search found no production code references to OData types or APIs, so keeping the package would only increase migration scope and runtime surface without delivering functionality.

2. **Preserve the current validation contract for OpenAPI 3.1.**  
   Even though OpenAPI.NET 3.4 can parse 3.1 documents, `OpenApiValidator` continues to throw `OpenApiUnsupportedSpecVersionException("3.1.0")` after parsing so the existing `--skip-validation` guidance remains true until the CLI/docs reconciliation work in `deps-007`.

3. **Use explicit byte loading plus the stream `LoadAsync(...)` overload instead of `OpenApiReaderSettings.HttpClient`.**  
   During the migration, the `HttpClient` property path produced a runtime `MissingMethodException` in this repo. The stable path here is: download bytes with repo-owned `HttpClient`, detect `json` vs `yaml`, call `readerSettings.AddYamlReader()`, then load with `OpenApiDocument.LoadAsync(stream, format, settings, ...)`.

## Why

- Keeps `deps-005` scoped to the reader pipeline and compile/build stability.
- Avoids a runtime-only API trap while restoring YAML support for both file and URL inputs.
- Defers user-visible validation behavior changes to the later CLI reconciliation issue instead of mixing scope in the dependency migration branch.
