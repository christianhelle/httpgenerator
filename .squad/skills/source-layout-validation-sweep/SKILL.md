---
name: "source-layout-validation-sweep"
description: "Update CI validation and smoke-test surfaces when source trees move but root entrypoints must keep working"
domain: "testing"
confidence: "high"
source: "observed"
---

## Context

Use this when a repository reorganizes source folders (for example into `src\...`) without changing the public root-invoked build/test commands. The risky part is usually not the business logic; it is the workflow triggers, solution paths, packaging copies, and smoke-test assumptions that still point at the old folders.

## Patterns

### Separate trigger rewrites from command rewrites

- Update workflow `paths:` filters first so CI still runs when moved source changes.
- Then update explicit solution/project/manifest copy paths inside validation workflows.
- Leave root entrypoint commands alone when the migration explicitly promises compatibility from the repo root.

### Preserve validation breadth

- A pure layout move should not automatically expand the regression matrix.
- Prefer keeping the same smoke scenarios and test permutations unless the move changes runtime packaging behavior.

### Treat packaging hosts as validation surfaces

- VS Code and VSIX workflows often bundle built CLI artifacts from `target\release`.
- If root cargo entrypoints are preserved, those bundle paths may remain valid even after the Rust sources move.
- Only rewrite the host-specific source tree paths (for example moved `.sln` files or manifest locations).
- Audit release and coverage workflows too, not just build/test workflows; they often hide stale source-root paths.

### Check relative-file semantics after the move

- Files that move under `src\dotnet` can keep or break relative paths depending on how many levels they previously climbed.
- Example split:
  - Shared props/import files such as `Directory.Build.props` need extra scrutiny because relative `Include=` paths may be evaluated from consuming project directories, not just from the props file location.
  - `publish-manifest.json` using `../docs/Marketplace.md` does **not** survive a move from `legacy\` to `src\dotnet\`; it must become `../../docs/Marketplace.md`.

### Flag runtime repo-root discovery separately

- A workflow can still build correctly while runtime lookup logic silently breaks after a folder move.
- If host code probes repo-root `target\...` via parent-directory climbing, verify the climb depth still reaches the real repo root from the new location and hand off any fix to implementation owners when production-code edits are out of scope.

### Split tester work from implementation work

- Tester-owned changes: workflow triggers, validation commands, smoke harness references, build packaging references.
- Implementation-owned changes: workspace manifests, crate paths, project references, physical directory moves, and general docs/instructions outside validation scope.

## Examples

- Replace workflow filters from `crates/**` to `src/rust/**`.
- Replace workflow filters from `legacy/**` to `src/dotnet/**`.
- Retarget `dotnet build legacy\HttpGenerator.sln` to `dotnet build src\dotnet\HttpGenerator.sln`.
- Keep `target\release\httpgenerator(.exe)` bundle paths unchanged when root cargo behavior is intentionally preserved.

## Anti-Patterns

- Do not widen the regression matrix just because folders moved.
- Do not rewrite root `target\release` assumptions unless the migration explicitly changes the root cargo contract.
- Do not silently absorb implementation-owned workspace or project-file rewrites into tester-only validation work.
