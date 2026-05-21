---
name: "docs-rs-guide-reference-balance"
description: "Structure public Rust crate docs so docs.rs reads as both a quick-start guide and an API reference"
domain: "documentation"
confidence: "high"
source: "observed"
---

## Context
Use this when a Rust crate has grown into a public library surface and docs.rs needs to help external users understand both the workflow and the individual APIs. It is especially useful when the crate exposes a mix of top-level helpers, facade modules, and intermediate data models.

## Patterns
- Start with crate-level rustdoc that answers four questions in order: what the crate does, the main workflow, the most important entry points, and where to go next.
- Give each major public module one job in docs.rs navigation. For `httpgenerator-core`, that means `openapi` explains ingestion, `normalized` explains the intermediate model, `generator` explains rendering, and `model` explains inputs/outputs.
- Put richer examples on boundary APIs that readers are likely to start with, such as load/normalize/generate entry points and settings/result types.
- If the crate-root example depends on intermediate public types, document those types in the same early batch so docs.rs links do not drop readers onto empty pages.
- Keep helper-module docs short and purpose-first. Explain when to use them and any surprising behavior, but do not let helper pages carry the main product narrative.
- Use mixed examples: runnable doctests for deterministic string/value helpers and `no_run` or prose examples for file/network-driven flows.
- For feature-gated public modules, add docs.rs-visible gating hints (`doc(cfg)` or equivalent) so readers understand why a module may be conditional.
- Reuse the same terminology everywhere (`raw`, `typed`, `normalized`, `generated`) so docs.rs feels like one guided system instead of disconnected reference pages.

## Examples
- Crate root overview: “Load an OpenAPI document, normalize it into `NormalizedOpenApiDocument`, then render one or more `.http` files with `generate_http_files`.”
- Module overview: “Use `openapi` when you need to classify a source, inspect metadata, or normalize an OpenAPI/Swagger document before generation.”
- Helper docs: “Use `resolve_base_url` to combine the source document location, server URL, and optional caller override into the base URL written to generated files.”

## Anti-Patterns
- Writing item-by-item rustdoc without a crate-level story
- Repeating the same long prose on every re-exported type
- Spending more docs space on internal helpers than on workflow entry points
- Mixing future aspirations into public docs instead of describing current behavior and limits
