# httpgenerator-core

`httpgenerator-core` provides the normalized model, `.http` rendering primitives, and `httpgenerator_core::openapi::*` helpers used by HTTP File Generator.

- Homepage: <https://christianhelle.com/httpgenerator/>
- Repository: <https://github.com/christianhelle/httpgenerator>

## Add to your project

```bash
cargo add httpgenerator-core
```

## API surface

- normalize OpenAPI-derived models into generator-friendly types
- generate `.http` files with `generate_http_files`
- read and normalize OpenAPI documents through `httpgenerator_core::openapi::*`, which re-exports the [oasreader](https://crates.io/crates/oasreader) crate, so specifications split across multiple files are merged before normalization
- reuse filename, privacy, and support-information helpers
