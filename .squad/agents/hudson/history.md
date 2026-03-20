# Hudson — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle

CLI tool + VS extensions for generating `.http` files from OpenAPI specs.

**Key docs:**
- `README.md` — installation, usage, examples
- `CHANGELOG.md` — Keep a Changelog format
- `docs/` — additional docs

**Package distribution:**
- NuGet global tool: `dotnet tool install --global httpgenerator`
- Visual Studio Marketplace (VSIX)
- VS Code Marketplace

## Learnings

### Release Infrastructure & Versioning Patterns (2025-01-25)
- **Three release channels:** NuGet CLI global tool (`httpgenerator`), NuGet class library (`HttpGenerator.Core` netstandard2.0), and VSIX/VSCode extensions
- **Version control split:** Release version lives in `.github/workflows/release.yml` (manual input), not in project files
- **VSIX versioning:** Set in `src/HttpGenerator.VSIX/Properties/AssemblyInfo.cs` as separate assembly version
- **Changelog automation:** Uses Ruby `github_changelog_generator` tool on `main` branch; never manually edit CHANGELOG.md
- **NuGet metadata:** Centralized in `Directory.Build.props` (authors, license, repo), per-project in `.csproj` files (description, tags)

### Dependency Impact Zones
- **High sensitivity:** OpenAPI parsers (affect file generation), CLI framework (affects command structure), Azure auth (affects user workflows)
- **Medium sensitivity:** Telemetry SDK (opt-out path must stay clear), JSON serialization (affects generated file formatting)
- **Low sensitivity:** Test frameworks, build tools (documentation-agnostic)
- **Validation pattern:** Always run smoke tests + README examples after major dependency bumps

### Key Documentation Touchpoints for Releases
1. `README.md` — Installation instructions, CLI usage examples, feature showcase with generated .http output
2. `CHANGELOG.md` — Auto-generated, reflects PR title structure (use standard prefixes: `chore(deps)`, `feat:`, `fix:`)
3. NuGet/Marketplace descriptions — Pulled from `.csproj` Description fields during build
4. Feature parity matrix — If dependency adds/removes support for OpenAPI versions, major flags needed
5. System requirements section — Update when targeting frameworks change (.NET version, VS version)

### Dependency Refresh: Release & Docs Impact Assessment (2026-03-20)

**Task:** Assess docs/release consequences for scheduled dependency refresh across NuGet CLI, Core library, and VSIX channels.

**Outcome:**
- Mapped 3 distribution channels (NuGet CLI `httpgenerator`, NuGet Core library `HttpGenerator.Core`, VSIX/VSCode extensions) with distinct release workflows
- Identified docs checkpoints per dependency category:
  - **Breaking OpenAPI parser changes:** May change `--skip-validation` requirement for v3.1; requires README verification and example validation
  - **CLI framework updates:** Patch releases unlikely to change options; check `--help` formatting
  - **.NET Framework/Runtime:** Unchanged (net8.0 CLI, net4.7.2 VSIX); no system requirement updates needed
  - **Auth/Azure integration:** No changes expected to Exceptionless or Application Insights
  - **Test infrastructure:** Atc.Test bump may affect test formatting; low user-facing impact
- Deferred final docs pass to `deps-010` (closeout PR) after behavior is validated
- Created full docs/release consequence mapping in `.squad/decisions/inbox/hudson-release-impact.md`

**Key Decision:** No immediate docs changes during implementation; final docs review happens post-merge as part of `deps-010` closeout.

