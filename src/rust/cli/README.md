# httpgenerator

`httpgenerator` is the Rust CLI for generating `.http` files from OpenAPI specifications.

- Homepage: <https://christianhelle.com/httpgenerator/>
- Repository: <https://github.com/christianhelle/httpgenerator>

## Install

```bash
cargo install httpgenerator
```

Prebuilt standalone installers are also available:

```bash
curl -fsSL https://christianhelle.com/httpgenerator/install | bash
```

```powershell
irm https://christianhelle.com/httpgenerator/install.ps1 | iex
```

## Usage

```bash
httpgenerator ./openapi.json --output ./generated --no-logging
```
