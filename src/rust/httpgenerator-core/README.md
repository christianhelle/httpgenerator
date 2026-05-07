# httpgenerator-core

`httpgenerator-core` provides the OpenAPI loading, inspection, parsing, normalization, and `.http` rendering primitives used by HTTP File Generator.

- Docs: https://docs.rs/httpgenerator-core
- Homepage: https://christianhelle.github.io/httpgenerator/
- Repository: https://github.com/christianhelle/httpgenerator

## Add to your project

```bash
cargo add httpgenerator-core
```

## API surface

- load, inspect, parse, and normalize OpenAPI documents into generator-friendly types
- generate `.http` files with `generate_http_files`
- reuse filename, privacy, and support-information helpers
