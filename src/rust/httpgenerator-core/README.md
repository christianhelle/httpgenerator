# httpgenerator-core

`httpgenerator-core` provides the OpenAPI loading, inspection, normalization, and `.http` rendering primitives used by HTTP File Generator.

- Docs: https://docs.rs/httpgenerator-core
- Homepage: https://christianhelle.github.io/httpgenerator/
- Repository: https://github.com/christianhelle/httpgenerator

## Add to your project

```bash
cargo add httpgenerator-core
```

## API surface

- load and inspect OpenAPI documents from files, URLs, or in-memory sources
- detect content format and specification version
- normalize OpenAPI-derived models into generator-friendly types
- generate `.http` files with `generate_http_files`
- reuse filename, privacy, and support-information helpers
