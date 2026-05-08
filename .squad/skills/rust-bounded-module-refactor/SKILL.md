---
name: "rust-bounded-module-refactor"
description: "Refactor Rust crates toward httprunner-style bounded-context directories with thin mod.rs facades."
domain: "rust-architecture"
confidence: "high"
source: "observed — compared httpgenerator and christianhelle/httprunner module layouts"
---

## Context

Use this skill when a Rust crate has grown around large flat files and needs a clearer internal structure without changing its external behavior. It fits especially well when a reference repo already demonstrates the preferred taste, as `httprunner` does for `httpgenerator`.

## Patterns

### Promote domains, not files
- Create a directory module when one file owns multiple sub-concerns or keeps growing past a comfortable review size.
- Name folders after bounded contexts (`parser`, `types`, `cli`, `upgrade`, `openapi`) rather than after implementation details.

### Keep the facade thin
- Put `mod.rs` at the folder root.
- Declare internal leaf files with `mod ...;`.
- Re-export the stable public surface from `mod.rs` with `pub use ...;`.
- Keep crate-root `lib.rs` / `main.rs` focused on wiring, not internal logic.
- When a published crate already exposes `pub mod foo;`, prefer turning `foo.rs` into `foo\mod.rs` instead of renaming the namespace; that preserves downstream paths while still enabling internal splits.

### Split by responsibility inside the domain
- Separate parsing, normalization, rendering, platform handling, and data types into distinct files.
- Use `#[cfg(...)]` leaf files for OS-specific behavior instead of burying platform branches in one large file.
- Co-locate tests with the module boundary (`tests.rs` or focused `*_tests.rs` files inside the folder).

### Document important directories
- Add a module-local `README.md` for significant multi-file folders.
- Capture ownership and file responsibilities there so future agents know where to place code.

## Examples

- Reference repo:
  - `christianhelle/httprunner:src/core/src/parser/`
  - `christianhelle/httprunner:src/core/src/types/`
  - `christianhelle/httprunner:src/cli/src/cli/`
  - `christianhelle/httprunner:src/cli/src/upgrade/`
- Candidate seams in this repo:
  - `src\rust\core\src\openapi\normalize.rs` → server/operation/parameter/request-body/schema submodules
  - `src\rust\core\src\generator.rs` → file modes/rendering/samples/helpers
  - `src\rust\cli\src\ui.rs` → presenter/rendering/formatting/table helpers
  - `src\rust\cli\src\execution.rs` → orchestration/validation/auth resolution
  - `src\rust\cli\src\telemetry.rs` → events/sinks/recorder/redaction while keeping `httpgenerator_cli::telemetry::*` stable

## Anti-Patterns

- Do not create folders that only wrap one trivial file.
- Do not break public APIs accidentally while chasing internal neatness.
- Do not move every helper into a directory; keep truly small leaf modules flat.
- Do not leave the old giant file in place as a grab bag after introducing submodules.
