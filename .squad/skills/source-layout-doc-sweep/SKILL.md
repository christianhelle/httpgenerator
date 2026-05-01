---
name: "source-layout-doc-sweep"
description: "Update documentation after a source-tree move without breaking repo-root command guidance"
domain: "documentation"
confidence: "high"
source: "manual"
---

## Context

Use this when a repository reorganizes implementation code under new source folders but intentionally keeps root-invoked commands stable. It applies best to mixed-language repos where user docs, contributor docs, and agent instructions all describe paths differently.

## Patterns

- Document the canonical source layout explicitly (`src\rust`, `src\dotnet`, `src\VSCode`) instead of relying on scattered path mentions.
- Pair every layout note with the preserved root entrypoints so contributors do not infer that `cargo`, `dotnet`, or packaging commands moved too.
- Re-audit nested READMEs and marketplace copy after directory moves; relative links and development-time binary lookup paths are common hidden breakpoints.
- When host docs mention development builds, prefer wording like **repo-root workspace** `target\debug` / `target\release` if the root Cargo workspace still owns build output.

## Examples

- Add a `Repository layout` section to the root `README.md`
- Update `.github\copilot-instructions.md` to describe both moved source roots and unchanged repo-root commands
- Fix moved nested links such as `src\dotnet\HttpGenerator\README.md` → `../../../README.md`
- Sync `docs\index.html` and `docs\Marketplace.md` with the same path vocabulary used in `README.md`

## Anti-Patterns

- Updating only contributor docs while leaving user-facing docs with stale source paths
- Describing moved source roots without clarifying that root command entrypoints still work
- Forgetting relative markdown links inside moved project subdirectories
