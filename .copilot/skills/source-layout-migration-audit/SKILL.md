---
name: "source-layout-migration-audit"
description: "Audit repo-wide source-tree relocations while preserving root entrypoints and catching hidden path-bearing surfaces."
domain: "architecture"
confidence: "high"
source: "manual"
tools:
  - name: "rg"
    description: "Find hard-coded path references across workflows, scripts, docs, and host code."
    when: "When a repo layout change affects more than one ecosystem."
---

## Context
Use this skill when a repository is moving source directories without changing the top-level developer contract. It is especially useful for mixed Rust/.NET/Node repos where workflows, packaging, and host runtime lookup code all embed paths differently.

## Patterns
- Split the audit into four buckets:
  1. root entrypoints (`Cargo.toml`, `Makefile`, root-invoked scripts)
  2. CI/release surfaces (workflow path filters, packaging, publish manifests)
  3. runtime host lookup code (development-time executable discovery)
  4. imported relative asset paths (`Directory.Build.props`, shared manifests, docs links)
- For final reviewer sign-off, clear three gates explicitly: repo-root commands still work, active build/release/runtime surfaces no longer mention retired source roots, and any development-time host probing still reaches repo-root `target\...` outputs after the move.
- Keep build outputs and user-facing repo-root commands stable unless the plan explicitly moves them.
- Move source trees in one coordinated pass, then update all path-bearing surfaces in the same change set.
- Treat release workflows and ignored bundled binaries as first-class migration surfaces; they are easy to miss because they are outside the main build/test loop.
- After moving Rust crates deeper, audit `env!("CARGO_MANIFEST_DIR")` helpers and `include_str!` fixture paths; they usually need one more `..` even when workspace manifests are already correct.
- Runtime repo-root probing is a separate migration surface from CI paths: any host or compatibility runner that climbs parent directories to find `target\...` or sibling projects must be retargeted explicitly.

## Examples
- `Cargo.toml` workspace members referencing `crates\`
- `.github\workflows\release-template.yml` and `.github\workflows\release-vsix.yml`
- `.github\workflows\codecov.yml`
- `.gitignore`
- `src\VSCode\build.ps1` / `src\VSCode\build.sh`
- `legacy\HttpGenerator.VSIX\HttpGeneratorCli.cs`
- `legacy\Directory.Build.props`
- `legacy\publish-manifest.json`
- Rust fixture helpers using `env!("CARGO_MANIFEST_DIR")`
- `include_str!` references to `test\OpenAPI\...`

## Anti-Patterns
- Approving a PR that leaves both old and new source roots active.
- Updating only build/test workflows but not release or coverage workflows.
- Assuming solution files are the main risk while ignoring runtime executable lookup and manifest-relative asset paths.
