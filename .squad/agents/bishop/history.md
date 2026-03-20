# Bishop — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** xUnit, FluentAssertions, .NET 8.0

**Test project:** `src/HttpGenerator.Tests/`
**Test fixtures:** `test/OpenAPI/v2.0/`, `test/OpenAPI/v3.0/`, `test/OpenAPI/v3.1/`
**Smoke tests:** `test/smoke-tests.sh` (~4.5 minutes — never cancel)

**Run tests:**
- Unit: `dotnet test HttpGenerator.sln --configuration Release`
- Smoke: `cd test && ./smoke-tests.sh`

**Known issues:**
- 1 unit test fails due to network restriction (external URL test) — expected, not a bug
- OpenAPI v3.1 tests require `--skip-validation` flag
- `make test` fails due to directory name collision — use `make -B test`

## Learnings

### Test Fixture Format for Path-Level Parameters (2025-01-17)

Created regression tests for issue #312 (path-level parameter merging) in PR #320:
- **Test file:** `src/HttpGenerator.Tests/PathLevelParametersTests.cs`
- **Fixture:** `src/HttpGenerator.Tests/Resources/V3/PathLevelParameters.json`
- **Pattern:** Use GitHub API-style paths (`/repos/{owner}/{repo}/issues`) to test path-level params
- **Key insight:** Path-level parameters (defined in `OpenApiPathItem.parameters`) are correctly merged into operation URLs as template variables (`{{owner}}`), but do NOT get variable definitions (@owner = str). Only operation-level query/header params get variable definitions.
- **Test coverage:** 13 tests covering all output types, parameter inheritance, override behavior, and mixed parameter sources
- **PR:** https://github.com/christianhelle/httpgenerator/pull/320

### Operation-Qualified Variable Names in OneFile Mode (2025-03-19)

Fixed 4 failing tests in `PathLevelParametersTests.cs` that had incorrect expectations for `OneFile` output mode:
- **Root cause:** In `OneFile` and `OneFilePerTag` modes, `GetParameterName()` in `HttpFileGenerator.cs` prefixes variable names with the operation name (e.g., `{{GetListIssues_owner}}`, `{{PostCreateIssue_repo}}`) to avoid collisions when multiple requests in the same file share parameter names.
- **This is INTENTIONAL behavior, not a bug** — required to prevent variable name conflicts across different operations in the same .http file
- **OneRequestPerFile mode:** Uses unqualified names (`{{owner}}`, `{{repo}}`) since each request is in its own file with no collision risk
- **Fixed tests:** Updated assertions in 4 tests to expect operation-qualified names in OneFile mode
- **Test results:** All 13 PathLevelParametersTests pass, full suite: 184 tests pass
- **PR:** https://github.com/christianhelle/httpgenerator/pull/323

### Repo-Wide NuGet Refresh Regression Baseline (2026-03-20)

- **User preference:** Keep `FluentAssertions` pinned at `7.2.0` during NuGet refresh work.
- **Baseline commands:** `dotnet restore HttpGenerator.sln`, `dotnet build HttpGenerator.sln --configuration Release`, and `dotnet test HttpGenerator.sln --configuration Release --no-build --logger "console;verbosity=minimal"` all passed locally; current unit baseline is **204 passing tests**.
- **Smoke validation on Windows:** `test/smoke-tests.sh` is a noisy wrapper in this environment because Bash trips over CRLF and `pwsh` resolution; use `test\smoke-tests.ps1` directly. `.\smoke-tests.ps1 -Parallel:$false` completed successfully in about **25 seconds**.
- **Fast CLI spot checks:** Current local behavior is `petstore.json` → **19 files** in default mode, **1 file** in `OneFile`, custom headers + IntelliJ assertions are present, and `v3.1\webhook-example.json` with `--skip-validation` currently generates **0 files** in default mode.
- **Highest-risk code paths for dependency refreshes:** `src/HttpGenerator.Core/OpenApiDocumentFactory.cs`, `src/HttpGenerator.Core/HttpFileGenerator.cs`, `src/HttpGenerator/Validation/OpenApiValidator.cs`, and `src/HttpGenerator/GenerateCommand.cs`.
- **First regression gates:** `src/HttpGenerator.Tests\OpenApiDocumentFactoryTests.cs`, `OpenApiValidatorTests.cs`, `GenerateCommandTests.cs`, `SwaggerPetstoreTests.cs`, `NullParametersTests.cs`, `PathLevelParametersTests.cs`, and `QueryParametersTests.cs`.
- **OpenAPI 3.1 coverage nuance:** `test\smoke-tests.ps1` only exercises `test\OpenAPI\v3.1\webhook-example.*`; `non-oauth-scopes.*` is commented out, and webhook-only specs currently act mostly as a no-crash signal because `OneRequestPerFile` and `OneFilePerTag` can legitimately write zero files.
- **Dependency staging insight:** `dotnet list HttpGenerator.sln package --outdated` shows a major jump from `Microsoft.OpenApi`/`Microsoft.OpenApi.Readers` `1.6.28` to `3.4.0`, and `Microsoft.OpenApi.OData` `1.7.5` to `3.2.0`; that migration should be isolated from lower-risk package refresh PRs.

### Dependency Refresh Planning Session (2026-03-20)

**Task:** Map baseline validation, regression sequencing, and final test matrix for planned NuGet refresh.

**Outcome:**
- Ran full baseline: `dotnet restore`, Release build, Release tests (204/204 green), `test\smoke-tests.ps1` (clean pass in ~25s)
- Proposed 4-stage validation split per PR, with specific test gates at each stage
- Mapped test hotspots: `OpenApiDocumentFactoryTests`, `OpenApiValidatorTests`, `GenerateCommandTests`, v3.1 fixtures, invalid-spec fixture
- Designed final regression matrix: v2.0/v3.0/v3.1 across JSON/YAML, all 3 output types, custom headers, validation modes
- Created concise per-PR validation and regression matrix summary in `.squad/decisions/inbox/bishop-validation-fast.md`
- Key insight: Windows PowerShell smoke tests (`test\smoke-tests.ps1`) preferred over bash wrapper in this environment

**Status:** Ready to execute validation gates per PR merge sequence. Test strategy aligned with 10-issue dependency-refresh backlog.

---

### First Implementation Batch: Issue #330 Attempt — STALLED (2026-03-20)

**Task:** Execute `deps-004` — Atc.Test upgrade from 1.1.18 to 2.0.17 (test-only package with FluentAssertions pinned).

**Outcome:** ⚠️ **STALLED** — No PR opened, no visible progress, work appears abandoned after background task launch.

**Expected deliverable:** 
- Branch: `feature/deps-004-atc-test`
- Changes: `src\HttpGenerator.Tests\HttpGenerator.Tests.csproj` version bump only
- Validation: standard `dotnet restore` → Release `build` → Release `test` cycle

**Status:** First implementation batch had one stalled item (Bishop/deps-004). This blocks parallel lane execution. Requires retry or escalation to team lead for clarification of blockers.

**Orchestration Log:** `.squad/orchestration-log/20260320T143102Z-bishop-deps004-stalled.md`

**Next action:** Coordinator to retry deps-004 or escalate to Ripley.

---

### Second Implementation Batch: Issue #330 Rescue — Atc.Test xUnit v3 Minimal Migration (2026-03-20)

**Task:** After Bishop's deps-004 stalled, Hicks opened competing PR (#340) with a rescue approach using minimal xUnit v3 migration.

**Approach:** When Atc.Test 2.x upgrade requires xUnit v3, and compatibility shim approach (PR #339) proves too heavy, opt for minimal direct xUnit v3 migration:
- Upgrade `xunit` from 2.9.3 to `xunit.v3 3.1.0`
- Add `OutputType=Exe` to test project
- Update async test method signatures with `CancellationToken`
- Replace `Assert.Equal` with FluentAssertions equivalents

**Outcome:**
- ✅ Branch `feature/deps-004-atc-test-rescue` created: single commit, 21 additions/17 deletions
- ✅ Validation: 204/204 tests green, Release build succeeded
- ✅ PR #340 opened, approved by Ripley, merged with regular merge commit
- ✅ PR #339 (Bishop's compatibility shim) closed as superseded (not rejected)
- ✅ Issue #330 auto-closed by merge

**Decision Override:** The team's earlier decision "xUnit stays on legacy family (v3 migration deferred)" is superseded because Atc.Test 2.x has a hard transitive dependency on xUnit v3. The minimal migration approach (PR #340) is cleaner than a 52-line compatibility shim.

**Pattern Learned:** When a test package mandates a major dependency version and deferral creates maintenance debt, prefer minimal forward migration over compatibility shims. The xUnit v3 pattern is now the standard for future test work.

**Status:** Deps-004 complete with xUnit v3 foundation. Available for regression testing and downstream test work.

**Orchestration Log:** `.squad/orchestration-log/20260320T150843Z-bishop-deps330-rescue.md`

---

### PR #343 Tester Gate — Rejected on Hosted Quality Gates (2026-03-20)

**Task:** Review PR #343 (`[deps-005] Migrate OpenAPI reader pipeline`) for issue #331.

**Outcome:** ❌ **REJECTED** — local behavior looked healthy, but the PR is not safe to merge yet.

- **Local validation signal:** Release restore/build passed, local regression evidence was green, and Hicks' branch kept the intended `--skip-validation` behavior for OpenAPI 3.1 generation paths.
- **Merge blocker:** SonarCloud raised a **new high-severity security alert** on `src/HttpGenerator.Core/OpenApiDocumentFactory.cs:81` because the rewritten download path still disables TLS certificate validation with `ServerCertificateCustomValidationCallback = (message, cert, chain, errors) => true`.
- **Secondary gate failure:** Codecov failed on the same PR (`72.02%` diff coverage, `79.29%` project coverage), so reviewer work on OpenAPI migrations must include hosted quality-gate inspection, not just local test runs.
- **Dependency sequencing impact:** `deps-006` (#332), `deps-007` (#333), and `deps-008` (#334) remain blocked behind `deps-005` / issue #331; `deps-009` (#335) is still the next clean parallel lane.
- **Process note:** Rejection was posted as a PR comment because GitHub blocks requesting changes on your own pull request; per squad rule, Hicks is locked out of the next revision for this artifact.

---

### Comprehensive Code Coverage Improvement (2026-03-20)

**Task:** Add comprehensive unit tests to maximize code coverage and improve smoke tests with more parameter combinations.

**Outcome:** ✅ **COMPLETED** — Successfully added 36 new unit tests across 4 new test classes and improved smoke tests with additional parameter combinations.

**New Test Files Created:**
- `OpenApiStatsTests.cs` — 9 tests covering OpenApiStats visitor pattern, counter validation, and ToString() formatting
- `HttpFileGeneratorEdgeCasesTests.cs` — 14 tests covering BaseUrl environment variable templates, SkipHeaders flag, authorization header handling, unique filename generation, custom content types, and empty spec edge cases
- `GeneratedContentTests.cs` — 8 tests covering sample JSON generation, query parameter variables, parameter default values, custom headers, and IntelliJ test generation
- Enhanced `PrivacyHelperTests.cs` — Added 5 tests for empty input, non-authorization text passthrough, and multiple header redaction
- Enhanced `SupportKeyInitializerTests.cs` — Added test for non-ISupportProperties telemetry path
- Enhanced `StringExtensionsTests.cs` — Added 5 tests for empty strings, null inputs, and edge cases
- Enhanced `OpenApiValidatorTests.cs` — Added 2 tests for IsValid property true/false branches

**Coverage Gaps Addressed:**
- HttpFileGenerator: BaseUrl environment variable template logic, SkipHeaders flag, AuthorizationHeaderFromEnvironmentVariable paths, custom AuthorizationHeaderVariableName, GetUniqueFilename collision handling, empty specs
- OpenApiStats: All Visit methods, ToString() formatting, counter validation
- PrivacyHelper: Empty input, non-authorization text, multiple headers
- SupportKeyInitializer: Non-ISupportProperties telemetry branch
- StringExtensions: Empty strings, null inputs, empty prefix edge cases
- OpenApiValidationResult: IsValid property branches

**Smoke Test Improvements:**
- Added `GenerateWithSpecificArgs` function for targeted parameter testing
- Added 5 additional test scenarios for petstore fixtures (v2.0 and v3.0):
  - `--authorization-header "Bearer test-token-123"`
  - `--load-authorization-header-from-environment --authorization-header-variable-name "my_token"`
  - `--skip-headers`
  - `--content-type "application/xml"`
  - `--base-url "{{MY_BASE_URL}}"` (environment variable template)

**Test Results:**
- Baseline: 204 tests → New total: 246 tests (+42 tests, +20.6% increase)
- All tests pass in Release configuration
- Build time: ~1s, Test time: ~8s

**Commits:**
1. `3bae55a` — test: add OpenApiStats comprehensive tests
2. `c96fd8d` — test: add HttpFileGenerator edge case tests
3. `c1d3661` — test: add edge cases for PrivacyHelper, SupportKeyInitializer, StringExtensions, OpenApiValidator
4. `194a947` — test: improve smoke tests with additional parameter combinations
5. `2ef4a13` — test: add generated content validation tests

**Key Patterns Learned:**
- xUnit v3 pattern with `OutputType=Exe` and `CancellationToken` parameters
- FluentAssertions patterns for collection uniqueness and content assertions
- AtcTest `[AutoNSubstituteData]` for auto-mocking dependencies
- Minimal OpenAPI spec creation for isolated edge case testing
- PowerShell `GenerateWithSpecificArgs` pattern for smoke test parameterization
- Variable name qualification in OneFile/OneFilePerTag modes (operation-prefixed names)

**Known Test Behaviors:**
- Network-dependent tests may fail in restricted environments (expected)
- OpenAPI v3.1 specs require `--skip-validation` flag
- One test may fail due to external URL restrictions (ExampleTests.cs) — this is expected and documented
