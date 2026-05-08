---
name: "rust-module-restructure-validation"
description: "Validate internal Rust module splits without accidentally treating them like feature work."
domain: "testing"
confidence: "high"
source: "observed"
---

## Context

Use this when `httpgenerator` or `httpgenerator-core` is being reorganized into deeper folders or `mod.rs` facades while the product behavior is supposed to stay unchanged. The main risk is not new business logic; it is breaking public re-exports, CLI contracts, oracle parity wiring, or smoke-test assumptions.

## Patterns

### Keep the external contract frozen

- Preserve crate names, binary name (`httpgenerator`), and the public facades in:
  - `src\rust\core\src\lib.rs`
  - `src\rust\core\src\openapi\mod.rs`
  - `src\rust\cli\src\lib.rs`
- If internal files move, keep existing `pub use` paths stable unless the change is intentionally breaking.

### Let the existing regression stack do most of the work

- First-line validation:
  1. `cargo test --workspace`
  2. `dotnet build src\dotnet\HttpGenerator.slnx --configuration Release`
  3. `dotnet test src\dotnet\HttpGenerator.slnx --configuration Release`
  4. `test\smoke-tests.ps1`
- Run host packaging validation only when executable discovery or host wiring changes.

### Protect the module boundaries directly

- Add or retain facade/re-export contract tests when splitting large files behind `mod.rs`.
- Add seam-local unit tests for extracted folders (validation, auth resolution, rendering, normalization) instead of growing the fixture matrix by default.
- Keep `help_contract.rs` focused on public CLI text and stream behavior, not internal structure.

### Treat parity and smoke matrices as wiring alarms

- `src\rust\cli\tests\differential_petstore.rs` catches Rust/.NET output drift across representative scenarios.
- `test\smoke-tests.ps1` catches release-binary regressions across the broader local fixture matrix plus CLI output/stream expectations.
- If option wiring changes, update both surfaces together because they intentionally cover overlapping but not identical contracts.

## Examples

- Differential parity runner hard-codes the `.NET` oracle project at `src\dotnet\HttpGenerator\HttpGenerator.csproj`.
- Smoke tests assume the release binary is copied from `target\release\httpgenerator(.exe)` into `test\bin\`.
- `help_contract.rs` locks help/version text, plain redirected output, stderr warning routing, and OpenAPI 3.1 guidance.

## Anti-Patterns

- Do not widen the smoke matrix just because files moved internally.
- Do not remove crate-root re-exports without adding replacement coverage.
- Do not assume green unit tests are enough; parity and smoke layers catch different breakpoints.
- Do not change binary/package identity during a pure module restructure unless the plan explicitly includes host and packaging fallout.
