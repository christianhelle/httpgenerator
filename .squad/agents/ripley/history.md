# Ripley — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** .NET 8.0 CLI, C#, Rust workspace, Microsoft.OpenApi, Spectre.Console.Cli, xUnit, FluentAssertions

**Reviewer-critical paths:**
- Repo-root entrypoints: `Cargo.toml`, `Makefile`, `test\smoke-tests.ps1`
- .NET solution/runtime surfaces: `src\dotnet\HttpGenerator.sln`, `src\dotnet\HttpGenerator.VSIX\HttpGeneratorCli.cs`, `src\dotnet\Directory.Build.props`, `src\dotnet\publish-manifest.json`
- CI/release surfaces: `.github\workflows\{build,codecov,release-template,release-vsix,smoke-tests,vscode,vsix}.yml`
- Docs/instructions: `README.md`, `CONTRIBUTING.md`, `.github\copilot-instructions.md`, `docs\`

**Review commands:**
- `cargo test`
- `dotnet build src\dotnet\HttpGenerator.sln --configuration Release`
- `dotnet test src\dotnet\HttpGenerator.sln --configuration Release`
- `test\smoke-tests.ps1`

**Historical summary:**
- Established squad governance for staged dependency refresh work: small logical commits, feature branches, review gates, and regular merge commits.
- Enforced reviewer lockout and duplicate-PR cleanup patterns to keep the backlog and merge trail coherent.
- Reviewed the Rust CLI output parity plan and the host-surface closeout work that preserved VS Code and VSIX compatibility.

## Learnings

### Source Layout Migration Approval (2026-05-01)
- Treat repo-wide source moves as coordinated path migrations, not folder renames. The review surface splits naturally into four buckets: repo-root entrypoints, CI/release filters, runtime host lookup code, and imported relative asset paths.
- The highest-risk implementation misses were the hidden plumbing files: `.github\workflows\codecov.yml`, `.github\workflows\release-template.yml`, `.github\workflows\release-vsix.yml`, `.gitignore`, `src\dotnet\publish-manifest.json`, `src\dotnet\Directory.Build.props`, and `src\dotnet\HttpGenerator.VSIX\HttpGeneratorCli.cs`.
- Final approval required all reviewer gates to clear together: root commands remained intact, active build/runtime surfaces no longer pointed at `crates\` or `legacy\`, and the VSIX development-time lookup still reached repo-root `target\...` builds after the extra `src\dotnet` nesting.
- Cross-agent closeout: Hicks landed the move and runtime rewrites, Bishop retargeted validation/release paths without changing matrix shape, Hudson updated the contributor/user-facing docs, and the migration passed the full validation set (`cargo test`, `dotnet build ...`, `dotnet test ...`, `test\smoke-tests.ps1`, `npm ci` + `npm run compile` in `src\VSCode`).
- Only non-blocking follow-up after approval was stale old-path guidance inside `.squad\`; internal notes should keep converging on `src\rust` and `src\dotnet`.
- Session directive: all spawned agents used GPT-5.4 for this session only.

### Crates.io Publishing Readiness Investigation (2026-05-XX)
- **Workspace structure:** Correct for publishing (CLI binary + 3 libraries; httpgenerator-compat is test-only)
- **Critical blocker:** Edition specified as `"2024"` (invalid; must be `"2021"` or similar recognized edition). Blocks all crates.io submissions.
- **Missing metadata:** All 4 crates lack `description`, `homepage`, `documentation`, `authors`, `keywords`, `categories` fields. Crates.io requires at minimum `description`; others improve discoverability.
- **Binary vs library decision pending:** Architectural choice needed on whether to publish CLI to crates.io (for `cargo install`) or continue GitHub Releases only. Option C (Both) recommended for maximum user choice; adds ~5 min to release workflow.
- **Library publication:** httpgenerator-core and httpgenerator-openapi are suitable for publication (stable public APIs, clean external dependency lists). httpgenerator-compat should be marked `publish = false` (test harness only).
- **Dependency ordering:** Release workflow must publish in order: core → openapi → cli. Current release-template.yml has no `cargo publish` steps.
- **Release workflow gaps:** Missing `cargo publish --dry-run` validation, missing CARGO_REGISTRY_TOKEN setup, missing publish steps in sequential order with delays for crates.io indexing.
- **Documentation:** README lacks library usage guidance; no lib.rs root docs in crates. Need crate-level documentation comments and module examples.
- **Versioning:** Currently 0.1.0 with manual sed/PowerShell updates. Semver strategy undefined (0.x.y vs 1.0.0 transition). Recommend staying in 0.x.y until APIs are stable.
- The initial crates.io gap analysis (architecture, blockers, metadata completeness, and release workflow impact) was merged into `.squad\decisions.md` during the 2026-05-05 session closeout.

### Crates.io Packaging Decision Gate (2026-05-05)
- Rust 2024 is valid now; the Edition Guide pegs Rust 2024 to Rust 1.85. The earlier "2024 is invalid" assumption is stale and should not drive packaging decisions anymore.
- Accepted public crate set: `httpgenerator` (CLI crate name for `cargo install httpgenerator`), `httpgenerator-core`, and `httpgenerator-openapi`. `src\rust\httpgenerator-compat` remains internal only with `publish = false`.
- Canonical URL split for crates: `homepage = https://christianhelle.github.io/httpgenerator/` for the human-facing product surface, and `documentation = https://docs.rs/<crate>` for API docs on each public crate.
- Direct decision-encoding paths touched: `Cargo.toml`, `src\rust\httpgenerator-{core,openapi,cli,compat}\Cargo.toml`, workflow/package-call surfaces that build by package name, and the merged decision record in `.squad\decisions.md`.

### Final Crates.io Publishing Reviewer Gate (2026-05-05)
- Final verdict: **approved / release-ready** for the crates.io implementation. No blocker-level defects found against the approved plan or the supplied validation evidence.
- Expected limitation remains explicit: `cargo package` / `cargo publish --dry-run` is only expected to pass for `httpgenerator-core` before publication; downstream `httpgenerator-openapi` and `httpgenerator` dry-runs will fail until the sibling dependency version becomes visible on crates.io.
- Publish sequencing is now encoded where it matters: `.github\workflows\release-template.yml` gates crates behind `publish-crates`, validates the token, publishes after artifact jobs, and publishes in order `httpgenerator-core` → `httpgenerator-openapi` → `httpgenerator` with crates.io polling between steps.
- Metadata/docs contract is coherent across root and crate surfaces: root `Cargo.toml`, `src\rust\httpgenerator-{core,openapi,cli,compat}\Cargo.toml`, crate READMEs, `README.md`, `docs\README.md`, and `docs\index.html` all align on the public crate set, docs.rs/homepage split, and the private status of `httpgenerator-compat`.
- Validation contract to remember for future reviews: `cargo test`, `dotnet build src\dotnet\HttpGenerator.slnx -c Release`, `dotnet test src\dotnet\HttpGenerator.slnx -c Release`, and `test\smoke-tests.ps1`; smoke correctness depends on the repo-root entrypoint staying anchored to `$PSScriptRoot`.

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction

### Team Closeout — crates.io publishing (2026-05-05)
- Hicks, Hudson, and Bishop cleared the implementation, docs, and validation tracks behind the packaging gate; final reviewer verdict stayed release-ready.
- Expected publish-order limitation is now an explicit approved condition: downstream dry-runs wait for `httpgenerator-core` visibility on crates.io.

### Rust Modularization Planning Gate (2026-05-08T13:19:39.287+02:00)
- Current mismatch: `src\rust\core\src\openapi\mod.rs` already follows a bounded-context folder pattern, but most of `httpgenerator-core` and `httpgenerator` still expose large flat files (`generator.rs`, `openapi\normalize.rs`, `execution.rs`, `ui.rs`) instead of `httprunner`-style directory modules with `mod.rs` facades.
- Reusable `httprunner` patterns worth porting: directory-per-domain layout (`parser\`, `types\`, `cli\`, `upgrade\`), thin `mod.rs` re-export surfaces, colocated `README.md` files for significant modules, and cfg/platform splits as leaf files rather than inline branching.
- Planning preference from Christian Helle: investigate only and deliver staged refactor guidance before implementation; do not modify production Rust code during this pass.
- Key review paths for the future refactor: `src\rust\core\src\lib.rs`, `src\rust\core\src\generator.rs`, `src\rust\core\src\normalized.rs`, `src\rust\core\src\model.rs`, `src\rust\core\src\openapi\normalize.rs`, `src\rust\cli\src\lib.rs`, `src\rust\cli\src\execution.rs`, `src\rust\cli\src\ui.rs`, plus `christianhelle/httprunner` reference modules under `src/core/src/{parser,types}` and `src/cli/src/{cli,shutdown,upgrade}`.

### VS Code Rust Host Review Gate (2026-05-13T21:06:43Z)
- Treat the VS Code Rust-host migration as one coordinated cutover across `src\vscode\src\extension.ts`, bundled packaging, workflow targeting, and extension validation; do not approve a runtime-only rewrite.
- Keep the executable contract locked to `http-file-generator.executablePath` → bundled binary in the installed extension → repo-root `target\debug` / `target\release` outputs → `httpgenerator` on `PATH`.
- An invalid explicit setting must fail fast instead of falling through, and the conflicting `src\vscode\PRD.md` draft should be treated as stale unless Ripley explicitly re-approves it.
- Reviewer watch-outs for this stream: stale `src\VSCode` path references and the broken package test script.

### VS Code packaged Rust host final approval (2026-05-13T21:06:43Z)
- Final reviewer closeout held once the packaged Rust binary matched the requested VS Code target and the explicit executable-path fail-fast contract stayed intact.
- Approval basis to remember: win32-x64 ships the matching x64 binary, local win32-arm64 packaging now fails fast instead of shipping the wrong binary, and CI covers real win32-arm64 packaging with the matching MSVC environment.
- Only remaining follow-up is manual host validation on native x64 and ARM64 VS Code installs.


### VSIX async first-slice review/revision closeout (2026-05-18T09:53:21Z)
- Rejected Hicks' first implementation wave because it missed the approved non-blocking notification contract and used an unreviewed Solution Explorer placement seam.
- Revised the artifact under lockout by using ShellExtensibility.ShowPromptAsync(...) for Open Folder, View Details, and Open Activity, restored VsctParent(... id: 521 ...), and reran the .NET validation path before approval.


### Standing small-commit rule closeout (2026-05-18T11:14:38Z)
- Encoded small logical commits as a standing rule across the squad instruction surfaces so future sessions do not depend on stale memory or session-only directives.
- Split the pending VSIX follow-up into three exact commits: c30d0b5 chore(squad): enforce small commits, 84103cc feat(vsix): async generation flow, and 8d33949 docs(vsix): align async workflow.
- Validation rerun to remember for this stream: cargo test --workspace, dotnet build src\dotnet\HttpGenerator.slnx -c Release, dotnet test src\dotnet\HttpGenerator.slnx -c Release, and dotnet build src\dotnet\VSIX.slnx -c Release; test\smoke-tests.ps1 still ends in the known release-path/ANSI baseline failure.

### VSIX visibility regression fix (2026-05-21T18:18:31Z)
- Identified and fixed case-sensitivity mismatch in `GenerateHttpCommand.VisibleWhen` pattern. Committed `344e49b fix(vsix): match openapi extensions case-insensitively` to apply `(?i)\.(json|ya?ml)$` regex.
- Coordinated with Hicks' concurrent Tools submenu restoration and Bishop's multi-pass validation.
- Result: uppercase/mixed-case OpenAPI filenames (e.g., `PETSTORE.JSON`, `spec.YAML`) now pass the visibility gate while maintaining case-insensitive runtime support check in `ClientContextExtensions.IsSupportedOpenApiPath(...)`.

