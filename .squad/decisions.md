# Squad Decisions

## Active Decisions

### 2026-03-19: Team initialized
**By:** Christian Helle
**What:** Hired Ripley (Lead), Hicks (Core Dev), Bishop (Tester), Hudson (DevRel/Docs), Scribe, Ralph. Universe: Aliens.
**Why:** Kickoff of AI team for HTTP File Generator project.

### 2026-03-20: Dependency Refresh Implementation Strategy
**By:** Ripley (Lead), Hicks (Core Dev), Bishop (Tester), Hudson (DevRel/Docs)
**What:** Staged NuGet dependency update plan with 10 GitHub issues (`deps-001` through `deps-010`), isolated OpenAPI v3 migration stream, feature/fix branches per issue, small frequent commits, and PR review gates (Ripley approval required). Regular merge commits, no squash/rebase.
**Key Decisions:**
- Keep `FluentAssertions` pinned during refresh
- Treat `Microsoft.OpenApi` major upgrade as dedicated migration stream (not bundled with general refresh)
- VSIX SDK updates isolated from main solution track
- ~~xUnit stays on legacy family (v3 migration deferred)~~ → **Superseded:** xUnit v3 migration pulled forward by Atc.Test 2.x dependency (PR #340, 2026-03-20)
- Use Windows PowerShell smoke tests (`test\smoke-tests.ps1`) for Windows validation
**Reference:** `.squad/decisions/inbox/ripley-dependency-refresh-plan.md`, `.squad/log/20260320-dependency-refresh-plan.md`

### 2026-03-20: Baseline Validation & Test Strategy
**By:** Bishop (Tester)
**What:** 4-stage validation split across planned PRs: (1) metadata/CLI patch, (2) test infrastructure, (3) OpenAPI migration, (4) final regression. Final regression matrix covers v2.0/v3.0/v3.1, JSON/YAML, all 3 output types, custom headers, validation modes.
**Reference:** `.squad/decisions/inbox/bishop-regression-plan.md`, `.squad/decisions/inbox/bishop-validation-fast.md`

### 2026-03-20: PR Cascade Review Verdict
**By:** Ripley (Lead)
**What:** Sequential review of PRs #321, #322, #323.
- **PR #323:** ✅ APPROVED (test assertion fixes for OneFile mode; unblocks others)
- **PR #321:** ❌ REJECTED (critical branch mismatch: title claims dedup logic but code contains JSON schema composition handling; missing HashSet deduplication)
- **PR #322:** ❌ REJECTED (cascading failure due to PR #321 branch contamination)
**Root Cause:** PR #321 branch content diverged from intent; likely rebase error or branch created from wrong base.
**Required Actions:** Recreate PR #321 with correct filename deduplication logic (HashSet + suffix appending).
**Reference:** `.squad/decisions/inbox/ripley-cascade-merge-313-314-323.md`

### 2026-03-20: NuGet Audit & OpenAPI v3 Breaking-Change Mapping
**By:** Hicks (Core Dev)
**What:** Audited 13 outdated packages. Baseline green: 204/204 tests, Release build, smoke tests. Identified 5 safe update chunks and mapped exact OpenAPI v3 migration path from reference PRs (refitter#907, oasreader#148). High-risk code paths: OpenApiDocumentFactory, OpenApiValidator, HttpFileGenerator, GenerateCommand.
**Key Findings:**
- Microsoft.OpenApi.Models → Microsoft.OpenApi (namespace)
- OpenApiStreamReader → OpenApiDocument.LoadAsync() (factory pattern)
- ReadResult.OpenApiDocument → ReadResult.Document (property name)
- Visitor pattern: OpenApiVisitorBase → IOpenApi* interfaces
- Serialization: sync → async (SerializeAsYamlAsync)
- Null-safety: v3 no longer auto-initializes collections
**VSIX Constraint:** dotnet list fails for VSIX project (needs VS/MSBuild environment); VSIX validation deferred to real VS environment.
**Reference:** `.squad/decisions/inbox/hicks-nuget-audit.md`

### 2026-03-20: Release & Documentation Impact Assessment
**By:** Hudson (DevRel/Docs)
**What:** Mapped release workflow across 3 channels (NuGet CLI, NuGet Core library, VSIX), identified docs checkpoints per dependency category, deferred final docs pass to `deps-010` closeout PR.
**Docs Checkpoints:**
- Update README if OpenAPI version support changes
- Update README if --skip-validation requirement changes post-migration
- Verify CLI examples work with new parser
- Prioritize local fixture coverage over remote URL tests
**Reference:** `.squad/decisions/inbox/hudson-release-impact.md`

### 2026-03-20: User Directive — Feature Branch & PR Workflow
**By:** Christian Helle (via Copilot CLI)
**What:** Plan and execute dependency updates in small chunks with small, frequent commits; never work directly on main; always use feature/fix branches and pull requests; have the squad review, approve, and merge PRs using regular merge commits (not squash or rebase).
**Why:** Ensure clean, reviewable git history and staged implementation with clear checkpoints.
**Reference:** `.squad/decisions/inbox/copilot-directive-20260320T124038Z.md`

### 2026-03-20: PR #338 — Spectre.Console.Cli 0.53.1 Upgrade (deps-003)
**By:** Ripley (Lead), Hicks (Core Dev)
**What:** Upgraded Spectre.Console.Cli from 0.53.0 to 0.53.1 in the CLI project. Single-line version bump in `src/HttpGenerator/HttpGenerator.csproj`.
**Validation:** Release build green, CLI `--help` renders correctly, petstore.json generation produces 19 .http files with intact rich output.
**Decision:** APPROVED & MERGED with regular merge commit.
**Impact:** Issue #329 closed. Tracking issue #327 updated (deps-003 ✅, 2/10 complete).

### 2026-03-20: PR #339 vs PR #340 — Atc.Test 2.0.17 Upgrade (deps-004) — PR #340 Selected
**By:** Ripley (Lead), Hicks (Core Dev), Bishop (Tester)
**What:** Two competing approaches to Atc.Test 2.x upgrade (issue #330):
  - **PR #339** (Bishop): Compatibility shim using ExcludeAssets="all" to preserve xUnit 2 (173 additions, 52-line re-implementation)
  - **PR #340** (Hicks): Minimal xUnit v3 alignment (OutputType=Exe, CancellationToken, Assert→FluentAssertions; 21 additions, 17 deletions)
**Validation:** Both PRs: 204/204 tests green, Release build succeeded. PR #340 is 8× smaller and introduces no maintenance debt.
**Decision:** MERGED PR #340, CLOSED PR #339 as superseded (not rejected; approach valid but suboptimal).
**Rationale:** Atc.Test 2.x has hard dependency on xUnit v3 (AutoFixture.Xunit3, xunit.v3.extensibility.core). Shim approach contradicts using modern Atc.Test. xUnit v3 migration is minimal and safe.
**Decision Override:** Previous decision "xUnit stays on legacy family (v3 migration deferred)" is now SUPERSEDED. xUnit v3 migration complete; future test work targets xUnit v3 patterns (e.g., TestContext.Current.CancellationToken).
**Impact:** Issue #330 closed. Both feature branches deleted. Tracking issue #327 updated (deps-004 ✅, 3/10 complete).

### 2026-03-20: PR Queue Cleanup — Reviewer Rejection Lockout Enforcement
**By:** Ripley (Lead)
**What:** Cleaned up OpenAPI PR queue to enforce reviewer-gate governance and maintain issue trail clarity.
  - **PR #344:** Closed (Hicks-authored duplicate follow-up after PR #343 rejection; violates "new-author revision" rule)
  - **PR #343:** Left open, clarified as rejected pending new-author revision (Bishop's security/coverage gates stand)
  - **Issue #331:** Blocked pending corrected deps-005 revision from non-Hicks author
**Key Principle:** Reviewer rejection triggers author lockout. Next revision of rejected work must come from different team member to prevent author fixup loops and maintain review discipline.
**Reference:** `.squad/decisions/inbox/ripley-cleanup-pr-343-344.md`

### 2026-03-20: Code Coverage Exclusion Audit (Hicks)
**By:** Hicks (Core Dev)
**What:** Conducted audit of production source files to add `[ExcludeFromCodeCoverage]` to genuinely untestable code and remove dead code.
**Key Actions:**
- Removed `GetStream` method (41 lines, completely unreachable) from `src/HttpGenerator/Validation/OpenApiValidator.cs` (commit `3f14302`)
- Added `[ExcludeFromCodeCoverage]` to `TryWriteLine` catch block in `src/HttpGenerator/GenerateCommand.cs` (console fallback logic difficult to unit test; commit `4082f2b`)
**Outcome:** Build green. Cleaner codebase with accurate coverage metrics focusing on testable logic.

### 2026-03-20: Comprehensive Code Coverage Improvement (Bishop)
**By:** Bishop (Tester)
**What:** Added 42 new unit tests (204 → 246 tests, +20.6%) across 5 new/enhanced test files and improved smoke tests with additional parameter combinations.
**New Test Files:**
- `OpenApiStatsTests.cs` — 9 tests covering OpenApiStats visitor pattern, counter validation, ToString() formatting
- `HttpFileGeneratorEdgeCasesTests.cs` — 14 tests covering BaseUrl env templates, SkipHeaders, auth headers, unique filename generation, custom content types, empty specs
- `GeneratedContentTests.cs` — 8 tests covering sample JSON generation, query parameters, defaults, custom headers, IntelliJ tests
- Enhanced `PrivacyHelperTests.cs` — 5 tests for empty input, non-auth text, multiple headers
- Enhanced `SupportKeyInitializerTests.cs` — test for non-ISupportProperties telemetry
- Enhanced `StringExtensionsTests.cs` — 5 tests for empty/null edge cases
- Enhanced `OpenApiValidatorTests.cs` — 2 tests for IsValid property branches

**Smoke Test Additions:** 5 new parameter combination scenarios (authorization headers, skip-headers, custom content-type, base-url env templates)
**Commits:** 5 commits; all 246 tests pass in Release configuration
**Rationale:** Maximize code coverage by targeting untestable code removal (Hicks) and comprehensive edge-case testing (Bishop).
**Impact:** Build green, smoke tests complete successfully, foundation for downstreamregressions.

### 2026-03-20: PR #342 Closure (Spectre.Console.Cli Duplicate)
**By:** Ripley (Lead)
**What:** Closed duplicate PR #342 (same Spectre.Console.Cli 0.53.1 upgrade as PR #338 which already merged)
**Rationale:** PR #338 was already merged with the identical dependency upgrade; PR #342 became redundant
**Outcome:** PR #342 closed; issue #329 resolved by PR #338

### 2026-03-20: PR #346 Closure (Stale Documentation PR)
**By:** Ripley (Lead)
**What:** Closed documentation-only PR #346 capturing post-merge learnings from PR #340 (Atc.Test upgrade)
**Rationale:** PR #340 already merged the complete Atc.Test 2.0.17 upgrade with xUnit v3 alignment; documentation is nice-to-have but redundant post-implementation
**Outcome:** PR #346 closed; no functional impact. Atc.Test upgrade complete via PR #340.

### 2026-03-20: User Directive — No Co-Author Trailers
**By:** Christian Helle (via Copilot CLI)
**What:** Never add Co-authored-by trailers to git commits in this repository
**Why:** User preference and instruction for commit message format
**Impact:** Squad commit convention updated to exclude Co-authored-by lines

### 2026-04-08: User Directive — Small Logical Commits
**By:** Christian Helle (via Copilot CLI)
**What:** Commit changes in small logical groups for detailed progress history in this and future sessions
**Why:** User request — enables clean, reviewable git history with clear checkpoints for team accountability
**Impact:** Standing rule for all squad members; enables detailed session-to-session traceability

### 2026-04-08: CLI Output Parity — Rust Rich & Plain Dual Modes
**By:** Ripley (Lead), Hicks (Core Dev), Bishop (Tester), Hudson (DevRel/Docs)
**Status:** ✅ COMPLETE — Implementation merged, all validation passing
**What:** Implemented context-aware output rendering in Rust CLI:
- **Rich mode** (interactive terminal): Colors, emojis (🚀, ✅, 📊, 📁, 🎉, etc.), box-drawing characters, formatted tables via Spectre-inspired `comfy-table` + `console` crates
- **Plain mode** (redirected/piped stdout): Semantic text only, no ANSI codes, no special characters, single-line file listings
- **Detection:** `io::stdout().is_terminal()` respects `$TERM`, pipes, and file redirection
**Key Implementation Details:**
- Rust CLI presenter layer in `main.rs` + `ui.rs` (existing `lib.rs` execution logic unchanged)
- Help contract tests validate both modes; all passing
- VSIX host surfaces Azure diagnostics correctly (success-path warnings no longer dropped)
- VS Code extension remains compatible (TTY detection handles rich output correctly)
**Validation:** cargo test ✅, dotnet test ✅, test\smoke-tests.ps1 ✅; VSIX build deferred (known environment limitation)
**Documentation:** README already accurate—no changes needed. Correctly conveys both output modes.
**Files Updated:** crates/httpgenerator-cli/{src/ui.rs, tests/help_contract.rs}, test/smoke-tests.ps1, src/HttpGenerator.VSIX/{HttpGeneratorCli.cs, GenerateDialog.cs}
**Decision:** Approved & ready for release. Pattern established: context-aware rendering + help contract validation for future CLI work.

## Governance

- All meaningful changes require team consensus
- Document architectural decisions here
- Keep history focused on work, decisions focused on direction
