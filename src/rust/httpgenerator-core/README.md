# httpgenerator-core

`httpgenerator-core` provides the normalized model and `.http` rendering primitives used by HTTP File Generator.

- Homepage: <https://christianhelle.com/httpgenerator/>
- Repository: <https://github.com/christianhelle/httpgenerator>

## Add to your project

```bash
cargo add httpgenerator-core
```

## API surface

- normalize OpenAPI-derived models into generator-friendly types
- generate `.http` files with `generate_http_files`
- reuse filename, privacy, and support-information helpers
