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
