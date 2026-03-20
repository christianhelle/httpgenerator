# deps-005 rescue decisions

- Preserve the existing CLI policy that OpenAPI 3.1+ requires `--skip-validation`, even though `Microsoft.OpenApi` 3.4.0 can parse 3.1 documents. The validator now loads with the new reader pipeline, then throws `OpenApiUnsupportedSpecVersionException` for 3.1+ so the user-facing behavior and docs stay stable in this milestone.
- Remove the unused `Microsoft.OpenApi.OData` package from `src/HttpGenerator/HttpGenerator.csproj` instead of carrying it forward, because there are no source references to OData in this repo.
- Keep follow-up behavior changes out of this branch. In particular, broader 3.1 sample-generation semantics (for example multi-type schema nuances) should be handled in a later dependency milestone instead of expanding deps-005.
