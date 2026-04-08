# Hicks — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** .NET 8.0 CLI, C#, NSwag, Microsoft.OpenApi, Spectre.Console.Cli, xUnit, FluentAssertions

HTTP File Generator generates `.http` files from OpenAPI specs. Core logic is in `src/HttpGenerator.Core/`, CLI in `src/HttpGenerator/`.

**Key files:**
- `src/HttpGenerator/GenerateCommand.cs` — CLI command, reads Settings, invokes HttpFileGenerator
- `src/HttpGenerator.Core/HttpFileGenerator.cs` — Core generation logic
- `src/HttpGenerator.Core/OpenApiDocumentFactory.cs` — OpenAPI parsing (NSwag + Microsoft.OpenApi)
- `src/HttpGenerator/Settings.cs` — CLI options (output, output-type, base-url, custom-header, etc.)
- `src/HttpGenerator.VSIX/` — Visual Studio extension
- `src/VSCode/` — VS Code extension

**Build & validate:**
- Build: `dotnet build HttpGenerator.sln --configuration Release`
- Quick test: `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.0/petstore.json --output /tmp/test --no-logging`
- Expected output: 19 .http files for petstore

**Known patterns:**
- OpenAPI v3.1 requires `--skip-validation` flag
- Output types: `OneRequestPerFile` (default), `OneFile`, `OneFilePerTag`
- Variables: `@baseUrl`, `@contentType` defined at top of generated files
- Custom headers passed via `--custom-header` flag

## Learnings Summary

### Early Work (2025-01-17)
**PR #319, #322, #323 batch:** Fixed query parameter dropping bug (path vs query param distinction), enhanced JSON sample generation for OpenAPI composition keywords (allOf/oneOf/anyOf), and aligned test assertions for OneFile mode operation-qualified variable names. Pattern learned: Distinguish path params from query params; recursive handling for composition keywords; parameter name qualification in multi-request files.

### NuGet Audit & OpenAPI v3 Migration Planning (2026-03-20)
**Task:** Comprehensive dependency surface audit (13 packages, 4 project files), breaking-change analysis from Microsoft.OpenApi v1→v3 migration, and safe update chunking strategy.

**Key findings:**
- 5 safe update chunks: metadata (SourceLink), CLI patch (Spectre), test-only (Atc.Test), VSIX SDK, and breaking OpenAPI migration
- OpenAPI v3 breaking changes: Namespace collapse (Models→OpenApi), reader factory pattern (StreamReader→LoadAsync), visitor interface change, async serialization, collection null-safety
- High-risk code paths: OpenApiDocumentFactory, OpenApiValidator, OpenApiStats, GenerateCommand, HttpFileGenerator, OperationNameGenerator, test expectations
- VSIX validation deferred to Visual Studio environment

---

## Learnings — Recent

### CLI Surface Parity Audit: C# Spectre vs Rust clap (2026-04-08)

- **User preference:** Christian wants the Rust CLI surface to align with the legacy C# CLI and, if possible, reach full feature parity for banners, colors, ASCII/Unicode chrome, and status output.
- **Primary rich-output oracle:** `src\HttpGenerator\GenerateCommand.cs` owns the legacy Spectre.Console UX (panels, rules, tables, colors, emojis, link rendering, success/error formatting). `src\HttpGenerator\Program.cs` and `src\HttpGenerator\Settings.cs` define the public help/version/examples surface.
- **Primary Rust CLI surface:** `crates\httpgenerator-cli\src\args.rs` owns parsing/help text, `crates\httpgenerator-cli\src\main.rs` owns user-visible stdout/stderr, `crates\httpgenerator-cli\src\lib.rs` owns execution flow, and `crates\httpgenerator-cli\tests\help_contract.rs` hard-locks the current plain-text output contract.
- **Architecture constraint learned:** the Rust CLI currently renders from `main.rs` *after* `execute()` returns a summary, so Azure-auth and file-writing messages are post-facto and cannot become real progress/prologue output without introducing an event/reporting abstraction in `crates\httpgenerator-cli`.
- **Validation parity gap:** the .NET CLI now validates OpenAPI 3.1 successfully (`src\HttpGenerator.Tests\GenerateCommandTests.cs` and direct runtime check), while Rust still rejects 3.1 in `crates\httpgenerator-cli\src\lib.rs` and `crates\httpgenerator-openapi\src\inspect.rs` unless `--skip-validation` is used.
- **Stats parity gap:** C# statistics come from `src\HttpGenerator\Validation\OpenApiStats.cs` via `OpenApiWalker`, while Rust uses manual raw-JSON counting in `crates\httpgenerator-openapi\src\inspect.rs`; this already shows concrete petstore drift (`Request Bodies 9 vs 7`, `Schemas 73 vs 109`).
- **Host integration constraint:** VS Code (`src\VSCode\src\extension.ts`) runs the Rust CLI in an interactive terminal, so rich ANSI/Unicode output is welcome there. Visual Studio (`src\HttpGenerator.VSIX\HttpGeneratorCli.cs`) captures stdout/stderr and only surfaces failures as plain text, so any parity work must keep a redirected/plain-safe mode.
- **Coverage constraint:** `crates\httpgenerator-compat\src\runner.rs` and `crates\httpgenerator-compat\tests\differential_petstore.rs` compare generated files, not CLI UI/output. CLI parity work needs its own Rust-side output tests in addition to existing generator parity checks.

### Second Implementation Batch: Issues #329, #330, #331 Kickoff (2026-03-20)

#### Issue #329 (deps-003): Spectre.Console.Cli 0.53.0 → 0.53.1
- **Branch:** `feature/deps-003-spectre-cli-rescue`
- **PR #338:** Single-line version bump in `src/HttpGenerator/HttpGenerator.csproj`
- **Validation:** Release build ✅, CLI --help ✅, petstore generation (19 files) ✅
- **Pattern:** Patch version bumps of CLI frameworks follow fast-track review: diff scope check + build + CLI smoke tests. Ripley's review gating continues efficient execution tempo.
- **Status:** ✅ COMPLETED, merged by Ripley

#### Issue #330 (deps-004): Atc.Test 1.1.18 → 2.0.17
- **Competing Approaches:**
  - **PR #339 (Bishop retry):** Compatibility shim (ExcludeAssets="all" + 52-line re-implementation) to keep xUnit 2
  - **PR #340 (Hicks rescue):** Minimal xUnit v3 migration (OutputType=Exe, 5 CancellationToken additions, 8 Assert→FluentAssertions conversions)
- **Decision:** PR #340 selected (8× smaller, no maintenance debt, uses Atc.Test as designed)
- **Override:** Team decision "xUnit stays on legacy family (defer v3)" superseded because Atc.Test 2.x has hard transitive dependency on xUnit v3. Minimal migration is cleaner than compatibility shim.
- **Pattern:** When modern test infrastructure pulls forward a major framework version, minimal forward migration wins over shims
- **Status:** ✅ COMPLETED, PR #340 merged; PR #339 closed as superseded

#### Issue #331 (deps-005): OpenAPI Reader Pipeline Migration [IN PROGRESS]
- **Branch:** `feature/deps-005-openapi-reader-pipeline`
- **Scope:** Critical OpenAPI v3 migration requiring factory pattern refactoring, property name updates, namespace moves, async serialization
- **High-Risk Paths:** OpenApiDocumentFactory.cs (entry), OpenApiValidator.cs (validation), HttpFileGenerator.cs (generation), GenerateCommand.cs (CLI)
- **Status:** 🔄 IN PROGRESS — kickoff complete, implementation underway

---


### PR TBD: Microsoft.SourceLink.GitHub metadata refresh (issue #328)
**Date:** 2026-03-20

**Problem:** `Microsoft.SourceLink.GitHub` was pinned to `8.0.0` in both the CLI and core library projects and needed a safe refresh as part of the staged dependency plan.

**Solution:** Updated the direct package references in:
- `src\HttpGenerator\HttpGenerator.csproj`
- `src\HttpGenerator.Core\HttpGenerator.Core.csproj`

**Pattern learned:** `Microsoft.SourceLink.GitHub` is a metadata-only dependency in this repo, so safe refreshes stay tightly scoped to the project files and can be validated with the standard solution pipeline (`restore`, Release `build`, Release `test`) plus the local `petstore.json` CLI generation check.

**Testing:** `dotnet restore HttpGenerator.sln`, `dotnet build HttpGenerator.sln --configuration Release`, `dotnet test HttpGenerator.sln --configuration Release`, and a local petstore generation run all passed. The petstore validation still produced 19 `.http` files.

### PR #322: Enhanced JSON sample generation for allOf/oneOf/anyOf schemas (issue #313)
**Date:** 2025-01-17

**Problem:** `GenerateSampleJson()` only handled schemas with explicit `type` fields, returning a useless `{}` for schemas using `allOf`, `oneOf`, or `anyOf` composition keywords. Complex APIs like GitHub's (35,493 schemas) heavily use these composition patterns.

**Solution:** Enhanced `GenerateSampleJson()` in `HttpFileGenerator.cs` to:
1. Recursively delegate to first non-null sub-schema for `allOf`, `oneOf`, `anyOf`
2. Generate property-aware JSON samples when `schema.Properties` is populated
3. Added `GetPropertySampleValue()` helper for type-appropriate property values
4. Limit properties to 3 for readability

**Pattern learned:** OpenAPI composition keywords (`allOf`, `oneOf`, `anyOf`) are common in complex APIs and need recursive handling. When a schema has `.Properties` populated, generate JSON with actual property names instead of generic placeholders. Always check composition keywords before falling back to basic type-based generation.

**Example output:** Instead of `{"property": "value"}`, now generates:
```json
{
  "id": 0,
  "name": "example",
  "category": {"property": "value"}
}
```

**Testing:** All 180 passing tests remain green (4 pre-existing PathLevelParametersTests failures unrelated to JSON generation). Validated with petstore.json showing property-aware samples.

### PR #319: Fixed query parameter dropping bug (issue #315)
**Date:** 2025-01-17

**Problem:** Query parameters were silently dropped when an operation had both path and query parameters. The URL construction logic checked `url.Contains("{")` AFTER escaping `{` to `{{`, so the condition was always true for paths with parameters, making the query string append branch unreachable.

**Solution:** Modified `GenerateRequest()` in `HttpFileGenerator.cs` to:
1. Separate path params (those in the URL template) from query params (those not in URL template)
2. Replace path parameter placeholders first: `{{owner}}` → `{{ownerVarName}}`
3. Always append query params as query string: `?key={{varName}}&key2={{varName2}}`

**Pattern learned:** When building URLs from OpenAPI operations, distinguish between path parameters (appear in URL template as `{param}`) and query parameters (must be appended as `?key=value`). Check parameter location against the original URL template BEFORE escaping braces.

**Testing:** All 171 unit tests pass. Validated with petstore.json showing correct output for mixed param types:        
- Query only: `GET {{baseUrl}}/user/login?username={{username}}&password={{password}}`
- Path + query: `POST {{baseUrl}}/pet/{{petId}}?name={{name}}&status={{status}}`

### PR TBD: VSIX SDK package refresh (issue #335 / deps-009)
**Date:** 2026-03-20

**Problem:** `src\HttpGenerator.VSIX\HttpGenerator.VSIX.csproj` still pinned `Microsoft.VisualStudio.SDK` to `17.0.32112.339` and `Microsoft.VSSDK.BuildTools` to `17.11.435`, so the VSIX dependency track lagged the planned 17.x refresh.

**Solution:** Updated only the two VSIX `<PackageReference>` entries to `Microsoft.VisualStudio.SDK` `17.14.40265` and `Microsoft.VSSDK.BuildTools` `17.14.2120`.

**Pattern learned:** For legacy VSIX projects in this repo, validate package-only refreshes by running `dotnet restore src\VSIX.sln` and then comparing `dotnet build src\VSIX.sln --configuration Release --no-restore` on both the updated branch and a clean `origin/main` baseline worktree. If the build fails with the same missing Visual Studio type/reference errors in both places, treat it as a headless-environment blocker instead of a regression from the package bump.

**Testing:** `dotnet restore src\VSIX.sln` succeeded after the update. `dotnet build src\VSIX.sln --configuration Release --no-restore` still failed in this CLI/MSBuild environment with 18 missing Visual Studio reference/type errors, and the same 18 errors reproduced unchanged from a detached `origin/main` baseline worktree.
### NuGet dependency audit and OpenAPI v3 migration map
**Date:** 2026-03-20

**Task:** Audit the repo's NuGet dependency surface, excluding FluentAssertions, with special attention to Microsoft.OpenApi / Readers / OData upgrades and likely breakages.

**Outcome:**
- Main solution baseline is healthy: `dotnet restore`, `dotnet build --configuration Release`, and `dotnet test --configuration Release` all passed for `HttpGenerator.sln` (204 tests), and `dotnet list package --vulnerable` reported no known vulnerabilities in the main solution.
- `HttpGenerator.sln` only covers `HttpGenerator`, `HttpGenerator.Core`, and `HttpGenerator.Tests`; the VSIX ships from `src\VSIX.sln` and needs separate validation.
- The safe direct package bumps are `Microsoft.SourceLink.GitHub` `8.0.0 -> 10.0.201`, `Spectre.Console.Cli` `0.53.0 -> 0.53.1`, and `Atc.Test` `1.1.18 -> 2.0.17` (test-only).
- The high-risk track is the OpenAPI stack: `Microsoft.OpenApi` `1.6.28 -> 3.4.0`, `Microsoft.OpenApi.OData` `1.7.5 -> 3.2.0`, and replacing `Microsoft.OpenApi.Readers` `1.6.28` with `Microsoft.OpenApi.YamlReader` `3.4.0`.

**Key migration findings:**
1. `Microsoft.OpenApi.Models` / `Microsoft.OpenApi.Services` collapse into the root `Microsoft.OpenApi` namespace, while reader-specific types move to `Microsoft.OpenApi.Reader`.
2. `OpenApiStreamReader` is replaced by `OpenApiDocument.LoadAsync(...)`, which returns `ReadResult`; code must switch from `result.OpenApiDocument` / `result.OpenApiDiagnostic` to `result.Document` / `result.Diagnostic`.
3. `OpenApiVisitorBase` overrides become interface-based (`IOpenApiParameter`, `IOpenApiSchema`, `IOpenApiPathItem`, `IOpenApiRequestBody`, `IOpenApiLink`, `IOpenApiCallback`, `IOpenApiReferenceHolder`, `IDictionary<string, IOpenApiHeader>`).
4. `OpenApiPathItem.Operations` is keyed by `System.Net.Http.HttpMethod` in v3, so any code assuming the older enum-based surface needs review.
5. OpenAPI.NET v3.4.0 successfully parses the local OpenAPI 3.1 webhook sample without downgrade hacks, so `OpenApiDocumentFactory.cs`'s 3.1 downgrade path and `GenerateCommandTests.cs`'s current "v3.1 fails validation" assumptions should be revisited.

**Files most likely to change for the OpenAPI migration:**
- `src\HttpGenerator.Core\OpenApiDocumentFactory.cs` — reader replacement and removal of 3.1 downgrade hack.
- `src\HttpGenerator\Validation\OpenApiValidator.cs` — new load API, new `ReadResult` property names, reader namespace move.
- `src\HttpGenerator\Validation\OpenApiStats.cs` — visitor override signatures switch to `IOpenApi*`.
- `src\HttpGenerator\Validation\OpenApiValidationResult.cs` — `OpenApiDiagnostic` namespace move.
- `src\HttpGenerator\GenerateCommand.cs` — catch `Microsoft.OpenApi.OpenApiUnsupportedSpecVersionException` from the new namespace and revisit `--skip-validation` messaging.
- `src\HttpGenerator.Core\HttpFileGenerator.cs` and `src\HttpGenerator.Core\OperationNameGenerator.cs` — namespace updates plus extra null-guards because v3 no longer auto-initializes some collections.
- `src\HttpGenerator.Tests\OpenApiDocumentFactoryTests.cs`, `src\HttpGenerator.Tests\OpenApiValidatorTests.cs`, `src\HttpGenerator.Tests\GenerateCommandTests.cs` — expectation changes around v3.1 parsing/validation behavior.

**VSIX constraint learned:** `dotnet list package` against `src\HttpGenerator.VSIX\HttpGenerator.VSIX.csproj` fails in this environment because the old-style project imports `Microsoft.VsSDK.targets`; VSIX package audits/builds need a real Visual Studio/MSBuild environment. Keep VSIX SDK updates on the 17.x line until they are validated there.

### Dependency Refresh Planning: NuGet Audit & OpenAPI v3 Migration Risk Mapping (2026-03-20)

**Task:** Audit the repo's NuGet dependency surface (excluding FluentAssertions), identify breaking changes from Microsoft.OpenApi / Readers / OData upgrades, and produce code-change risk guidance.

**Outcome:**
- Ran full baseline validation: `dotnet restore HttpGenerator.sln`, Release build, Release tests (204/204 green), `dotnet list package --vulnerable` (no known vulns)
- Audited direct packages across 4 project files: `HttpGenerator.csproj`, `HttpGenerator.Core.csproj`, `HttpGenerator.Tests.csproj`, `HttpGenerator.VSIX.csproj`
- Categorized 13 outdated packages into 5 safe update chunks:
  1. **Chunk 1 — Safe metadata:** `Microsoft.SourceLink.GitHub` `8.0.0 -> 10.0.201` (no code usage)
  2. **Chunk 2 — Safe CLI patch:** `Spectre.Console.Cli` `0.53.0 -> 0.53.1` (patch release)
  3. **Chunk 3 — Test-only package:** `Atc.Test` `1.1.18 -> 2.0.17` (isolated to tests, medium risk)
  4. **Chunk 4 — VSIX SDK refresh:** `Microsoft.VisualStudio.SDK`, `Microsoft.VSSDK.BuildTools` on 17.x line (separate validation)
  5. **Chunk 5 — Breaking OpenAPI migration:** `Microsoft.OpenApi` `1.6.28 -> 3.4.0`, replace `Microsoft.OpenApi.Readers` with `Microsoft.OpenApi.YamlReader`, `Microsoft.OpenApi.OData` `1.7.5 -> 3.2.0` (high-risk, dedicated PR stream)

**OpenAPI v3 Breaking-Change Analysis (from refitter#907, oasreader#148):**
- **Namespace moves:** `Microsoft.OpenApi.Models` / `Microsoft.OpenApi.Services` → root `Microsoft.OpenApi`; reader types → `Microsoft.OpenApi.Reader`
- **Reader API change:** `OpenApiStreamReader` → `OpenApiDocument.LoadAsync(...)`; returns `ReadResult` with new properties (`ReadResult.Document` instead of `ReadResult.OpenApiDocument`)
- **Visitor pattern change:** `OpenApiVisitorBase` overrides → interface-based (`IOpenApiParameter`, `IOpenApiSchema`, `IOpenApiPathItem`, `IOpenApiRequestBody`, `IOpenApiLink`, `IOpenApiCallback`, `IOpenApiReferenceHolder`)
- **Serialization:** sync → async (`SerializeAsYamlAsync()`)
- **Null-safety:** v3 does not auto-initialize collections (HttpFileGenerator, OperationNameGenerator need extra guards)
- **OpenAPI.NET v3.4.0 benefit:** Successfully parses local OpenAPI 3.1 webhook sample without downgrade hacks; current 3.1 workarounds in OpenApiDocumentFactory and GenerateCommandTests may become unnecessary

**High-Risk Code Paths Identified:**
- `src\HttpGenerator.Core\OpenApiDocumentFactory.cs` — reader replacement, 3.1 downgrade hack removal
- `src\HttpGenerator\Validation\OpenApiValidator.cs` — new load API, ReadResult property names, reader namespace
- `src\HttpGenerator\Validation\OpenApiStats.cs` — visitor override signature changes to IOpenApi*
- `src\HttpGenerator\Validation\OpenApiValidationResult.cs` — OpenApiDiagnostic namespace move
- `src\HttpGenerator\GenerateCommand.cs` — exception namespace move from OpenApiValidationException to new location, v3.1 messaging
- `src\HttpGenerator.Core\HttpFileGenerator.cs` — namespace updates, null-guard additions for collections
- `src\HttpGenerator.Core\OperationNameGenerator.cs` — null guards for component collections
- Test fixtures: `OpenApiDocumentFactoryTests`, `OpenApiValidatorTests`, `GenerateCommandTests` — expectation reassessment

**Status:** Audit complete and ready for integration into dependency-refresh execution plan. VSIX validation deferred to Visual Studio/MSBuild environment.

**Decision Document:** `.squad/decisions/inbox/hicks-nuget-audit.md`

---

### First Implementation Batch: Issue #328 Implementation (2026-03-20)

**Task:** Execute `deps-002` — Microsoft.SourceLink.GitHub upgrade from 8.0.0 to 10.0.201 and open PR #337.

**Outcome:**
- ✅ Branch `feature/deps-002-sourcelink` created: single commit, two .csproj PackageReference updates
- ✅ PR #337 opened with clean diff scope (metadata-only package, PrivateAssets="All" preserved)
- ✅ Validation performed locally:
  - `dotnet restore` succeeded
  - `dotnet build --configuration Release` succeeded
  - `dotnet test --configuration Release` — 204/204 passed
  - CLI spot check: `petstore.json` → 19 .http files (expected)
- ✅ PR #337 approved by Ripley, merged with regular merge commit
- ✅ Issue #328 auto-closed by merge

**Pattern Learned:** Metadata-only packages follow a minimal validation path: restore/build/test + optional CLI generation spot check. Ripley's fast-track review gate worked efficiently. Coordination between implementation and review gating is smooth.

**Status:** Hicks ready for next implementation (deps-003 or OpenAPI migration stream). First completed dependency-refresh item demonstrates execution tempo is good.

**Orchestration Log:** `.squad/orchestration-log/20260320T143102Z-hicks-deps002.md`

---

### Second Implementation Batch: Issue #329 Implementation (2026-03-20)

**Task:** Execute `deps-003` — Spectre.Console.Cli upgrade from 0.53.0 to 0.53.1 and open PR #338.

**Outcome:**
- ✅ Branch `feature/deps-003-spectre-cli-rescue` created: single commit, one-line version bump in `src/HttpGenerator/HttpGenerator.csproj`
- ✅ PR #338 opened with minimal diff scope
- ✅ Validation performed:
  - Release build: ✅ Succeeded
  - CLI `--help`: ✅ All options render correctly (Spectre.Console.Cli surface intact)
  - CLI generation: ✅ petstore.json → 19 .http files (rich output tables and panels working)
- ✅ PR #338 approved by Ripley, merged with regular merge commit
- ✅ Issue #329 auto-closed by merge

**Pattern Learned:** Patch version bumps of CLI framework dependencies follow the same fast-track review path as metadata-only packages. CLI smoke tests (--help, generation) are sufficient validation for framework patches. Ripley's review gate continues to work efficiently.

**Status:** Completed deps-003. Available for next implementation (OpenAPI reader pipeline migration, deps-005, is critical path).

**Orchestration Log:** `.squad/orchestration-log/20260320T150843Z-hicks-deps329.md`

---

### Second Implementation Batch: Issue #330 Rescue — Atc.Test xUnit v3 Minimal Migration (2026-03-20)

**Task:** Execute `deps-004` — Atc.Test upgrade from 1.1.18 to 2.0.17 with minimal xUnit v3 migration (rescue approach after Bishop's initial attempt).

**Outcome:**
- ✅ Branch `feature/deps-004-atc-test-rescue` created: single commit, 21 additions/17 deletions
- ✅ Minimal xUnit v3 alignment strategy:
  - Upgraded `xunit` from 2.9.3 to `xunit.v3 3.1.0`
  - Added `OutputType=Exe` to test project
  - Updated 5 async test methods with `CancellationToken` parameter
  - Converted 8 `Assert.Equal` calls to FluentAssertions equivalents
  - No test logic or structure changes
- ✅ Validation: 204/204 tests green, Release build succeeded
- ✅ PR #340 opened, approved by Ripley, merged with regular merge commit
- ✅ Issue #330 auto-closed by merge

**Decision Override Rationale:** Atc.Test 2.x has hard dependency on xUnit v3 (AutoFixture.Xunit3, xunit.v3.extensibility.core). The team's earlier decision "xUnit stays on legacy family (v3 migration deferred)" is superseded because:
1. Staying on xUnit 2 requires a 52-line compatibility shim that defeats using Atc.Test 2.x benefits
2. The xUnit v3 migration here is minimal (5 lines, 8 conversions) and maintainable
3. Future test work should target xUnit v3 patterns (e.g., TestContext.Current.CancellationToken)

**Pattern Learned:** When a modern test package (Atc.Test 2.x) has hard transitive dependency on a major version (xUnit v3), deferring the migration creates maintenance debt. Minimal migration is safer and cleaner than compatibility shims.

**Status:** Completed deps-004 with xUnit v3 migration. Available for critical OpenAPI reader pipeline migration (deps-005).

**Orchestration Log:** `.squad/orchestration-log/20260320T150843Z-bishop-deps330-rescue.md`

---

### Second Implementation Batch: Issue #331 Kickoff — OpenAPI Reader Pipeline Migration (2026-03-20)

**Task:** Start `deps-005` — Migrate OpenAPI reader pipeline to Microsoft.OpenApi v3 (CRITICAL PATH).

**Status:** 🔄 **IN PROGRESS** — Branch created: `feature/deps-005-openapi-reader-pipeline`

**Scope:** This is the highest-risk migration in the refresh plan. Will require refactoring:
- **OpenApiDocumentFactory.cs:** OpenApiStreamReader → OpenApiDocument.LoadAsync() (factory pattern change)
- **OpenApiValidator.cs:** ReadResult property names (ReadResult.OpenApiDocument → ReadResult.Document)
- **HttpFileGenerator.cs:** Namespace updates (Microsoft.OpenApi.Models → Microsoft.OpenApi), null-safety guards for collections
- **GenerateCommand.cs:** Exception namespace moves, v3.1 validation behavior changes
- Serialization: Sync → Async (SerializeAsYamlAsync)

**High-Risk Code Paths:**
- `OpenApiDocumentFactory.cs` (parsing entry point)
- `OpenApiValidator.cs` (validation logic)
- `HttpFileGenerator.cs` (core generation)
- `GenerateCommand.cs` (CLI integration)

**Next Steps:**
1. Migrate factory pattern to OpenApiDocument.LoadAsync()
2. Update property references throughout
3. Add null-safety guards for collection initialization
4. Test with v3.0 and v3.1 specs
5. Open PR when ready for Ripley review

**Orchestration Log:** `.squad/orchestration-log/20260320T150843Z-hicks-deps331-kickoff.md`



## Code Coverage Audit - 2026-03-20

### Changes Made
- Removed dead GetStream method from OpenApiValidator.cs (lines 22-61) - method was unreachable after migration to OpenApiMultiFileReader.Read
- Added [ExcludeFromCodeCoverage] to TryWriteLine in GenerateCommand.cs - console fallback logic in catch block is not testable

### Files Already Properly Excluded
- Program.cs - entry point already marked
- RedactedEnvironmentInfoPlugin.cs - telemetry plugin already marked
- SupportKeyInitializer.cs - kept testable, Bishop has test coverage

### Key Decision
Prioritized removing dead code over marking it excluded. The GetStream method was completely unreachable after the OpenAPI parsing refactor to use OpenApiMultiFileReader.Read.

---

### Session: 2026-03-20 Code Coverage Improvement — Completion (2026-03-20)

**Task:** Execute code coverage audit and clean up untestable code infrastructure.

**Outcome:** ✅ **COMPLETED** — Dead code removed, appropriate coverage exclusions applied, build green.

**Execution Summary:**
- Removed `GetStream` method (41 lines, completely unreachable) from `src/HttpGenerator/Validation/OpenApiValidator.cs` — was made dead by OpenApiMultiFileReader.Read migration
- Added `[ExcludeFromCodeCoverage]` to `TryWriteLine` method in `src/HttpGenerator/GenerateCommand.cs` — catch block with Console.WriteLine fallback is infrastructure code difficult to unit test
- Validated: Release build green, no regressions

**Coverage Audit Decisions:**
1. **Remove over exclude:** Unreachable code deleted rather than marked; cleaner codebase
2. **Exclude infrastructure:** Console fallbacks, entry points, telemetry plugins appropriately excluded
3. **Keep testable:** SupportKeyInitializer left uncovered but Bishop validated it with NSubstitute mocks

**Commits:** 2 commits
- `3f14302` — "chore: remove dead GetStream method from OpenApiValidator"
- `4082f2b` — "chore: exclude TryWriteLine console fallback from code coverage"

**Impact:** More accurate code coverage metrics focusing on testable business logic; cleaner codebase; no dead code debt.

**Orchestration Reference:** `.squad/log/2026-03-20-coverage-improvement-complete.md`
