---
name: "Rust Multi-Crate Publishing to crates.io"
description: "Automated workflow patterns for publishing Rust crates to crates.io from GitHub Actions, including version injection, token auth, and multi-crate dependency ordering."
domain: "release-automation, crates.io, cargo-publish"
confidence: "high"
source: "observed from httprunner and azdocli reference implementations"
tools:
  - name: "github-mcp-server"
    description: "GitHub Actions workflow inspection and file fetching"
    when: "Analyzing release workflows in reference repos"
---

## Context

Rust projects published to crates.io require automation to handle version updates, multi-platform builds, and credential management. This skill captures proven patterns from two reference implementations (httprunner and azdocli) that apply to multi-crate workspaces like httpgenerator.

## Patterns

### 1. Version Injection via Environment Variable
**Pattern**: Use a single `VERSION` env var in the GitHub Actions workflow, injected to `Cargo.toml` files before publishing.

**Implementation**:
```yaml
env:
  VERSION: 0.9.${{ github.run_number }}  # or hardcoded, or computed from git tags
  
jobs:
  publish-crates:
    steps:
      - name: Update Version
        shell: pwsh
        run: |
          $toml = (Get-Content -Path Cargo.toml -Raw) -replace 'version = "0.1.0"', 'version = "${{ env.VERSION }}"'
          $toml | Set-Content -Path Cargo.toml
```

**Why this works**:
- Single source of truth (env var) avoids version drift across multiple Cargo.toml files
- PowerShell script is cross-platform compatible (Windows CI, Unix CI)
- "0.1.0" placeholder is a convention; both reference repos start with it
- Allows version to be computed, hardcoded, or parameterized without changing workflow structure

**For multi-crate workspaces**: Apply sed/replace to each crate's Cargo.toml individually:
```yaml
$toml = (Get-Content -Path src/rust/httpgenerator-core/Cargo.toml -Raw) -replace 'version = "0.1.0"', 'version = "${{ env.VERSION }}"'
$toml | Set-Content -Path src/rust/httpgenerator-core/Cargo.toml
```

**Workspace dependency variant**: If internal crates are declared once in the root `[workspace.dependencies]` table with both `path` and `version`, replace every `version = "0.1.0"` occurrence in the root `Cargo.toml` so the workspace package version and internal dependency pins stay aligned.

### 2. Token-Based Authentication
**Pattern**: Store `CRATES_TOKEN` as a GitHub Organization Secret, pass to `cargo publish` via `--token` flag.

**Implementation**:
```yaml
- name: Publish to crates.io
  run: cargo publish --allow-dirty --token ${{ secrets.CRATES_TOKEN }}
  env:
    CARGO_REGISTRY_TOKEN: ${{ secrets.CRATES_TOKEN }}  # optional, redundant
```

**Prerequisites**:
- Create account at crates.io
- Generate API token from crates.io account settings
- Store token as `CRATES_TOKEN` in GitHub Organization Secrets (or Repository Secrets if org-level is unavailable)
- Document token setup in CONTRIBUTING.md

**Note**: `--allow-dirty` is required because the workflow modifies Cargo.toml before publish; this is expected and safe in CI/CD.

### 3. Multi-Crate Publishing with Dependency Ordering
**Pattern**: Publish crates sequentially in dependency order (no parallel jobs for the same crate set).

**Implementation**:
```yaml
publish-crates:
  steps:
    - name: Publish Core to crates.io
      run: cargo publish --allow-dirty --token ${{ secrets.CRATES_TOKEN }} -p httpgenerator-core
      
    - name: Publish OpenAPI to crates.io (depends on core)
      run: cargo publish --allow-dirty --token ${{ secrets.CRATES_TOKEN }} -p httpgenerator-openapi
      
    - name: Publish CLI to crates.io (depends on both)
      run: cargo publish --allow-dirty --token ${{ secrets.CRATES_TOKEN }} -p httpgenerator
```

**Why sequential**:
- crates.io index propagation takes ~1 min per publish
- Sequential ensures downstream crates can resolve dependencies immediately
- `cargo publish` with explicit `-p {crate}` targets specific workspace members

### 4. Job Dependencies and Release Flow
**Pattern**: Chain jobs to ensure builds complete before publishing; optionally gate publishing on GitHub Release creation.

**Implementation (Sequential)**):
```yaml
jobs:
  build:
    # ... build binaries, create archives, upload artifacts

  release:
    needs: build
    # ... download artifacts, create GitHub Release with assets

  publish-crates:
    needs: release  # Wait for GitHub Release before publishing to crates.io
    # ... run cargo publish jobs
```

**Alternative (Parallel)**: All publish jobs depend on `build`, not `release` (faster, but less coupled to GitHub Release timing).

**httpgenerator recommendation**: Use sequential (publish-crates depends on release) to ensure GitHub Releases are finalized before crates.io publication.

### 4c. Validation Ladder for Dirty Release Branches
**Pattern**: When validating crates.io readiness on a branch that already contains uncommitted release metadata or workflow edits, run publish-focused checks in two stages: first without `--allow-dirty` to capture the worktree gate, then with `--allow-dirty` to expose the real packaging result.

**Why this works**:
- Separates expected dirty-worktree failures from actual crates.io regressions
- Mirrors CI/CD behavior where version injection intentionally dirties manifests before publish
- Prevents the dirty-tree gate from hiding missing-registry-dependency failures in downstream crates

**Httpgenerator example**:
- `httpgenerator-core` passes `cargo package --allow-dirty` and `cargo publish --dry-run --allow-dirty`
- `httpgenerator-openapi` and `httpgenerator` still fail until `httpgenerator-core` is available from crates.io for package verification

**Interpretation rule**:
- A failure that disappears with `--allow-dirty` is an expected release-workflow condition, not a product regression
- A downstream "no matching package named ... found" against crates.io before sibling publication is an expected pre-publish blocker, not a branch regression

### 4a. Reusable Workflow Opt-In for Stable Publishing
**Pattern**: Put crates.io publication behind a `workflow_call` boolean input so stable release entrypoints can opt in while preview/dry-run callers reuse the same artifact jobs without publishing.

**Implementation**:
```yaml
on:
  workflow_call:
    inputs:
      publish-crates:
        required: false
        type: boolean
        default: false

jobs:
  publish-crates:
    if: ${{ inputs.publish-crates }}
```

**Why this works**:
- Keeps one canonical release workflow instead of forking artifact and publish logic
- Prevents preview releases from accidentally publishing prerelease crate versions
- Lets the stable caller (`release.yml`) opt in explicitly with `publish-crates: true`

### 4b. Poll crates.io Instead of Sleeping Blindly
**Pattern**: After publishing a dependency crate, poll the crates.io API until the new version is visible before dry-running/publishing downstream crates.

**Implementation**:
```bash
python .github/scripts/check-crates-io-version.py \
  --crate httpgenerator-core \
  --version "$RELEASE_VERSION" \
  --state present \
  --retries 30 \
  --delay-seconds 10
```

**Why this works**:
- crates.io indexing latency varies; fixed sleeps are either flaky or wasteful
- downstream `cargo publish --dry-run` only succeeds once the just-published dependency version is visible
- the same helper can also assert a version is absent before starting publication

### 5. README.md Version Anchors (Optional)
**Pattern**: Update per-crate README files with published version, so install instructions always show latest.

**Implementation**:
```yaml
- name: Update README with version
  run: |
    $readme = (Get-Content -Path src/rust/httpgenerator-core/README.md -Raw) -replace 'httpgenerator-core = "0.1.0"', 'httpgenerator-core = "${{ env.VERSION }}"'
    $readme | Set-Content -Path src/rust/httpgenerator-core/README.md
```

**When to use**: Only if per-crate README files exist and contain version-pinned install instructions. httprunner does this; azdocli does not.

### 6. User-Facing Distribution Matrix
**Pattern**: Document crates.io, GitHub Releases, and editor extensions as complementary delivery channels instead of treating one as a full replacement for the others.

**Implementation guidance**:
- Position `cargo install <cli-crate>` as the Rust-native install path for published releases.
- Keep GitHub Releases documented as the source for prebuilt CLI archives for users without a Rust toolchain.
- Call out which crates are public libraries versus internal/private workspace crates.
- Explicitly say that VS Code / Visual Studio extensions bundle native binaries and do not install the CLI from crates.io, even if they can reuse a Cargo-installed binary from `PATH` or an override setting.

**Why this works**:
- Prevents user confusion between package publishing and binary distribution.
- Keeps install guidance accurate for both CLI-only users and extension users.
- Gives downstream docs a stable template when a Rust-first repo still ships Marketplace / VSIX hosts.

## Examples

### Httprunner (4-crate workspace, 2 published)
- Publishes `httprunner-core` + `httprunner` (CLI)
- Uses `0.9.${{ github.run_number }}` versioning (auto-increments)
- Updates both root and per-crate Cargo.toml files
- Updates `src/core/README.md` with version
- Also publishes Docker images in parallel (orthogonal jobs)

### Azdocli (1-crate, multi-distribution)
- Single crate, no workspace complexity
- Uses hardcoded `VERSION: 0.4.1` (manual updates)
- Publishes to crates.io, then generates Homebrew/Chocolatey/WinGet packages in parallel jobs

### Httpgenerator (3-crate, cli-focused)
- Should publish in order: core → openapi → httpgenerator
- Consider using git tags for version source (e.g., `git describe --tags`)
- Simpler than httprunner (no Docker), more complex than azdocli (3 crates vs 1)
- Reusable workflow should default `publish-crates` to `false`; `release.yml` opts in for stable publication while preview callers keep artifact-only behavior
- User-facing docs should explain the split this way:
  - crates.io: `cargo install httpgenerator`
  - GitHub Releases: prebuilt `httpgenerator-<version>-<platform>` archives
  - VS Code / Visual Studio: extension packages that bundle native binaries
  - Public crates: `httpgenerator`, `httpgenerator-core`, `httpgenerator-openapi`
  - Private crate: `httpgenerator-compat`

## Anti-Patterns

**❌ Parallel crate publishing**: Publishing multiple interdependent crates in parallel jobs without waiting for crates.io index propagation.
- **Fix**: Use sequential steps in single job, or add sleep/retry logic between parallel jobs.

**❌ Hardcoding version in Cargo.toml**: Forgetting to inject version from workflow env var, or skipping per-crate Cargo.toml updates in workspace.
- **Fix**: Template ALL Cargo.toml files (workspace root + all members) with "0.1.0" placeholder; replace in workflow.

**❌ Path-only internal workspace dependencies for publishable crates**: Local builds work, but crates.io packages cannot resolve unpublished sibling crates.
- **Fix**: Use `{ version = "0.1.0", path = "..." }` for publishable sibling crates, then update those version anchors during release automation.

**❌ Omitting `--allow-dirty`**: Publishing fails with "working directory is not clean" error.
- **Fix**: Include `--allow-dirty` when publishing from CI/CD after version injection.

**❌ Missing token secret**: Using literal token in workflow file or committing token to git.
- **Fix**: Store `CRATES_TOKEN` in GitHub Secrets, never in workflows or source code.

**❌ Publishing before GitHub Release**: Publishing to crates.io before creating GitHub Release with binaries.
- **Fix**: Create `release` job that depends on `build`, then `publish-crates` depends on `release`.

**❌ Blind fixed sleeps between crates**: Sleeping 60–180 seconds and hoping crates.io has indexed the dependency.
- **Fix**: Poll crates.io until the new version appears, then continue to the downstream crate.

**❌ Conflating crates.io with extension delivery**: Telling users that VS Code or Visual Studio installs the CLI from crates.io.
- **Fix**: State clearly that extensions ship bundled binaries and only optionally reuse a Cargo-installed binary via `PATH` or explicit configuration.

## Decisions for Httpgenerator

1. **All 3 crates public?** Publish `httpgenerator` (CLI crate name), `httpgenerator-core`, and `httpgenerator-openapi`. Keep `httpgenerator-compat` private with `publish = false`.

2. **Edition/MSRV pairing?** Rust 2024 is valid as of Rust 1.85. Keep Edition 2024 if the repo is already on it, and make the corresponding `rust-version = "1.85"` promise explicit instead of downgrading based on stale pre-release guidance.

3. **Homepage/docs split?** Use the GitHub Pages site (`https://christianhelle.github.io/httpgenerator/`) as the human-facing homepage and `https://docs.rs/<crate>` as the canonical per-crate API documentation URL.

4. **Version source?** httprunner uses run number (0.9.NNN), azdocli hardcodes. Consider git tags for semantic versioning (e.g., `v1.0.0` → `1.0.0`).

5. **Per-crate README?** httprunner updates src/core/README.md; azdocli does not. Defer unless per-crate install instructions are needed.

6. **Job chaining?** Recommend sequential (publish-crates depends on release) to couple crates.io publication with GitHub Release finalization.

## Decision Gate Notes

- Do not assume `edition = "2024"` is invalid. That was true before stabilization, but Rust 2024 shipped in Rust 1.85.
- For cargo-install ergonomics, the public CLI crate should be named `httpgenerator`; keeping `httpgenerator-cli` only as a folder name is fine.

---

**Reference Decision**: See `.squad/decisions/inbox/hudson-crates-publishing-patterns.md` for analysis of httprunner and azdocli implementations.
