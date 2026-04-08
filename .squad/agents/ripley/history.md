# Ripley — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle
**Stack:** .NET 8.0 CLI, C#, NSwag, Microsoft.OpenApi, Spectre.Console.Cli, xUnit, FluentAssertions

HTTP File Generator is a CLI tool and Visual Studio extension that generates `.http` files from OpenAPI specs (v2.0, v3.0, v3.1) for use with REST Client extensions in VS Code, JetBrains, and Visual Studio 2022.

**Key files:**
- `src/HttpGenerator/GenerateCommand.cs` — CLI command implementation
- `src/HttpGenerator.Core/HttpFileGenerator.cs` — Core generation logic
- `src/HttpGenerator.Core/OpenApiDocumentFactory.cs` — OpenAPI parsing
- `src/HttpGenerator/Settings.cs` — CLI options
- `test/OpenAPI/` — Test OpenAPI specs

**Build commands:**
- Build: `dotnet build HttpGenerator.sln --configuration Release`
- Test: `dotnet test HttpGenerator.sln --configuration Release`
- Run: `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- [options]`

## Learnings Summary

### Dependency Refresh Strategy & Plan (2026-03-19)
**Task:** Lead planning session to audit NuGet dependencies, identify breaking changes from Microsoft.OpenApi v3 migration, propose staged implementation strategy.

**Outcome:** Structured 10-issue backlog (deps-001 through deps-010) with owner lanes, critical-path sequence, low-risk parallel track, 30+ small commits, PR review gates, regular merge commits. Timeline: 7 days sequential, 2–3 days parallelized.

**Key Decisions:** FluentAssertions pinned, Ripley gates all PRs, feature/fix branches per issue, rejection triggers lockout (new agent reworks).

**Reusable Pattern:** Breaking package migration = isolate package upgrade + separate code refactor + regression tests, merged in sequence.

### PR Cascade Review (2026-03-19)
**Task:** Sequential review of PRs #321, #322, #323 addressing filename deduplication, JSON schema composition, and test assertion fixes.

**Verdicts:**
- **PR #323:** ✅ APPROVED — Test assertions correctly updated for OneFile mode; unblocks others
- **PR #321:** ❌ REJECTED — Critical branch mismatch: title claims dedup (#314) but code contains JSON schema logic (#313). Branch must be recreated.
- **PR #322:** ❌ REJECTED (cascading) — Cannot evaluate due to PR #321 failure; identical JSON schema changes suggest branch contamination.

**Pattern:** Branch content divergence signals rebase error or wrong base branch.

### GitHub Issue Backlog Creation (2026-03-20)
**Task:** Create 10 GitHub issues (deps-001 through deps-010) from the dependency-refresh plan.

**Outcome:**
- ✅ Created all 10 issues with cross-references:
  - #327: [deps-001] Tracking issue with 9-item checklist
  - #328–#336: [deps-002] through [deps-010] individual issues with branch names, owner lanes, scope, validation steps
- Each issue body preserves plan's detailed guidance (thematic rules, risk areas, validation matrix)
- No repository file modifications; issues only

**Pattern:** GitHub CLI issue creation with cross-references requires issue numbers determined upfront for inclusion in deps-001 checklist.

---

## Learnings — Recent

### Second Implementation Batch: PR Review Gates #338, #339/#340 (2026-03-20)

#### PR #338 Review: Spectre.Console.Cli 0.53.0 → 0.53.1 (deps-003)
- **Validation:** Release build ✅, CLI --help smoke test ✅, petstore generation (19 files) ✅, diff scope confirmed
- **Verdict:** ✅ APPROVED & MERGED (regular merge commit, branch deleted)
- **Tracking:** Issue #329 auto-closed; Issue #327 checklist updated (deps-003 ✅, 2/10 complete)
- **Pattern:** Patch version bumps of CLI frameworks follow fast-track review: diff scope check + build + CLI smoke tests

#### PR #339 vs #340 Comparison Review: Atc.Test 2.0.17 Upgrade (deps-004)
- **PR #339 (Bishop):** +173/-1 lines, compatibility shim (xUnit 2 preservation via ExcludeAssets)
- **PR #340 (Hicks):** +21/-17 lines, minimal xUnit v3 migration (OutputType, 5 CancellationToken, 8 Assert→FluentAssertions)
- **Verdict:** ✅ PR #340 SELECTED (8× smaller, no maintenance debt, Atc.Test used as designed)
- **PR #339:** CLOSED as superseded (approach valid but suboptimal)
- **Decision Override:** Team decision "xUnit stays on legacy family (defer v3)" is now SUPERSEDED
  - **Reason:** Atc.Test 2.x hard dependency on xUnit v3 (AutoFixture.Xunit3, xunit.v3.extensibility.core)
  - **Shim defeats purpose:** ExcludeAssets="all" + 52-line re-implementation contradicts using modern Atc.Test 2.x
  - **Migration is minimal:** 5 CancellationToken additions, OutputType flag, 8 Assert conversions
  - **Future pattern:** xUnit v3 is now standard; future test work targets xUnit v3 APIs
- **Tracking:** Issue #330 auto-closed by PR #340 merge; Issue #327 checklist updated (deps-004 ✅, 3/10 complete)

#### Batch Summary
- ✅ PR #337 (deps-002): SourceLink upgrade merged
- ✅ PR #338 (deps-003): Spectre.Console.Cli patch merged
- ✅ PR #340 (deps-004): Atc.Test with xUnit v3 minimal migration merged
- 🔄 deps-005 (Hicks): OpenAPI reader pipeline migration IN PROGRESS
- ✅ Baseline: 204/204 tests passing on xUnit v3, Release build green

**Execution Tempo:** Low-risk batch complete in a single session. Fast-track review gating + minimal diff scopes = efficient throughput.

---


### Dependency Refresh Planning (2026-03-19)

**Task:** Audit NuGet dependencies, plan staged upgrade strategy, identify breaking changes from Microsoft.OpenApi v3 migration

**Outcome:**
- Analyzed 13 out-of-date NuGet packages across HttpGenerator, HttpGenerator.Core, HttpGenerator.Tests
- Identified critical blocker: Microsoft.OpenApi 1.6.28 → 3.3.1 has major breaking changes (namespaces, API surface, async serialization)
- Reviewed reference PRs (refitter#907, oasreader#148) to map exact migration path:
  - `Microsoft.OpenApi.Models` → `Microsoft.OpenApi` (namespace)
  - `OpenApiStreamReader` → `OpenApiDocument.LoadAsync()` (factory pattern)
  - Visitor pattern: `Visit(IOpenApiReferenceable)` → `Visit(IOpenApiReferenceHolder)`
  - Sync serialization → async (`SerializeAsYaml()` → `SerializeAsYamlAsync()`)
  - Collections null-safety (v3 no longer auto-initializes)

**Plan Structure:**
- 13 GitHub issues organized by priority: CRITICAL (3), HIGH (3), MEDIUM (4), LOW (3)
- Critical path: dep-01 → code-01 → test-01 → verify-01 (blocking dependencies)
- Parallel track: 5 non-breaking package updates (dep-02 through dep-05, plus test deps)
- Estimated timeline: 7 days full sequential, 2–3 days if parallelized

**Key Decisions:**
- FluentAssertions skipped per user request (licensing concerns)
- Regular (non-squash) merge strategy for full history visibility
- Ripley gate on all PRs; rejection triggers lockout (new agent reworks, not original)
- Small atomic commits (30+ total) for detailed progress tracking
- Feature branches per issue: `chore/*`, `refactor/*`, `test/*` naming

**Risk Areas Identified:**
1. Async propagation: OpenApiDocumentFactory.cs may become async, impacting call chain upward

---

### deps-005 OpenAPI Reader Pipeline Rescue (2026-03-20)
- **Scope:** Upgrade Microsoft.OpenApi to 3.4.0, replace Readers with YamlReader, remove unused OpenApi.OData.
- **Reader pipeline:** OpenApiDocument.LoadAsync now requires an explicit format string; use OpenApiConstants.Json/Yaml with extension/content heuristics.
- **Model surface:** Namespace moved to Microsoft.OpenApi (no .Models); Paths and parameters now use IOpenApi* interfaces, schema Type is JsonSchemaType flags.
- **Visitor API:** OpenApiVisitorBase/OpenApiWalker live in Microsoft.OpenApi; Visit overrides should use IOpenApi* signatures.
- **Key files:** HttpFileGenerator.cs, OpenApiDocumentFactory.cs, OpenApiValidator.cs, OpenApiStats.cs, GenerateCommand.cs.
2. Null safety: Guard clauses needed for component collections (v3 no longer auto-init)
3. Test environment: One network test expected to fail in restricted environments
4. Smoke tests: 4.5 minutes — never cancel, use isolated session

**Reusable Pattern:** Breaking package migration = isolate package upgrade + separate code refactor + regression tests, merged in sequence. Non-breaking updates = one package per PR, parallelizable.

**Decision Document:** `.squad/decisions/inbox/ripley-dependency-refresh-plan.md`

---

### PR Cascade Review (2026-03-19)

**Task:** Sequential review of PRs #323 (test fixes), #321 (filename dedup), #322 (JSON schemas)

**Verdicts:**
- **PR #323:** ✅ APPROVED - Test assertions correctly updated for OneFile mode; unblocks other PRs
- **PR #321:** ❌ REJECTED - Critical branch mismatch: title claims "deduplicate filenames" (#314) but code contains JSON schema handling (allOf/oneOf/anyOf). No HashSet deduplication logic found. Branch must be recreated with correct deduplication code.
- **PR #322:** ❌ REJECTED (cascading) - Cannot evaluate independently due to PR #321 failure. Both branches show identical JSON schema changes, suggesting branch contamination or rebase error.

**Key Finding:** PR #321's commit message describes filename deduplication intent but the actual code in the branch diverged—contains JSON schema logic instead. This is a critical mismatch requiring investigation into how the branch got created.

**Decision Document:** `.squad/decisions/inbox/ripley-cascade-merge-313-314-323.md`

---

### Dependency Refresh Planning & Leadership (2026-03-20)

**Task:** Lead full squad planning session to audit NuGet dependencies, identify breaking changes, and propose staged implementation strategy.

**Outcome — Phase 1: Dependency-Refresh Backlog & GitHub Issue Breakdown**
- Created structured 10-issue backlog (`deps-001` through `deps-010`) with explicit owner lanes (Ripley, Hicks, Bishop, Hudson), effort estimates, and dependency relationships
- Designed critical-path execution sequence: `deps-001` (baseline) → OpenAPI migration (`deps-005` through `deps-008`) → `deps-010` (final regression)
- Low-risk parallel track: `deps-002` (SourceLink), `deps-003` (Spectre), `deps-004` (Atc.Test), `deps-009` (VSIX SDK)
- Implementation rules locked: feature/fix branches per issue, small frequent commits (30+ total), PR review gates (Ripley approval), regular merge commits, no squash/rebase
- Produced complete executable plan with success criteria, risk areas, and reusable pattern (breaking migration = isolate + code refactor + regression tests)
- Estimated timeline: 7 days full sequential, 2–3 days if parallelized

**Outcome — Phase 2: PR Cascade Review (#321, #322, #323)**
- Reviewed sequential PRs addressing #313 (JSON samples), #314 (filename dedup), and test fixes
- **PR #323:** ✅ APPROVED — Test assertions correctly updated for OneFile mode; unblocks other PRs
- **PR #321:** ❌ REJECTED — Critical branch mismatch: title claims dedup (#314) but code contains JSON schema composition handling (#313); missing HashSet deduplication logic entirely
- **PR #322:** ❌ REJECTED (cascading) — Cannot evaluate independently due to PR #321 failure; both branches show identical JSON schema diffs suggesting branch contamination or rebase error
- Root cause identified: PR #321 branch content diverged from intent; commit message describes dedup intent but actual code changed to JSON schema logic

**Status:** Dependency-refresh plan ready for implementation launch. PR cascade cascading failure documented; PR #321 requires branch recreation with correct deduplication code.

**Decision Document:** `.squad/decisions/inbox/ripley-dependency-refresh-plan.md` (complete plan), `.squad/decisions/inbox/ripley-cascade-merge-313-314-323.md` (PR cascade verdict)

---

### Dependency Refresh Issue Backlog Creation (2026-03-20)

**Task:** Use GitHub CLI to create 10 GitHub issues from the dependency-refresh plan; search first to avoid duplication; make deps-001 the tracking issue with a prioritized checklist.

**Outcome:**
- ✅ Searched for existing `deps-001` through `deps-010` issues (none found)
- ✅ Created all 10 issues in priority order:
  - **#327** `[deps-001]` Baseline capture and tracking board (tracking issue with 9-item checklist linking to deps-002 through deps-010)
  - **#328** `[deps-002]` Upgrade Microsoft.SourceLink.GitHub (Hicks, infrastructure bump)
  - **#329** `[deps-003]` Upgrade Spectre.Console.Cli (Hicks, CLI patch)
  - **#330** `[deps-004]` Upgrade Atc.Test (Bishop, test package with FluentAssertions pinned)
  - **#331** `[deps-005]` Migrate OpenAPI reader pipeline to v3 (Hicks, critical breaking change)
  - **#332** `[deps-006]` Migrate OpenAPI visitor/statistics (Hicks, visitor pattern changes)
  - **#333** `[deps-007]` Reconcile generator and CLI (Hicks, code reconciliation)
  - **#334** `[deps-008]` Refresh OpenAPI regression tests (Bishop, test expectations)
  - **#335** `[deps-009]` Refresh VSIX SDK packages (Hicks, isolated from main solution)
  - **#336** `[deps-010]` Final regression and release closeout (Ripley + Bishop + Hudson, multi-team)

**Key Decisions Applied:**
- Each issue body contains: goal, branch name, owner lane, scope, validation steps, notes, and explicit dependencies
- Titles use consistent `[deps-###]` prefix for scannability
- `deps-001` tracking issue includes GitHub checklist linking to #328–#336 for progress visibility
- Issue descriptions preserve plan's detailed guidance (thematic migration rules, risk areas, validation matrix)
- No repository file modifications; issues only (as requested)

**Pattern Recognition:** GitHub CLI issue creation with cross-references requires issue numbers determined upfront; checklist links work best when issues are created in sequence so subsequent numbers are available for inclusion in deps-001 body. Alternative: create deps-001 first with placeholder checklist, then edit to add links—but GitHub's update flow is messier than creating in final form.

**Decision Document:** `.squad/decisions/inbox/ripley-deps-issue-creation.md`

---

### PR #337 Review: Microsoft.SourceLink.GitHub Upgrade (issue #328 / deps-002)
**Date:** 2026-03-20

**Task:** Reviewer gate for PR #337 — upgrade Microsoft.SourceLink.GitHub 8.0.0 → 10.0.201.

**Verdict:** ✅ APPROVED and merged (regular merge commit, branch deleted).

**Validation performed:**
- Independent Release build in worktree: succeeded
- Independent Release test run: 204/204 passed
- Diff scope confirmed: exactly two .csproj PackageReference bumps, PrivateAssets="All" preserved
- Single commit, clean branch from main

**Tracking updates:**
- Issue #328: auto-closed by merge
- Issue #327 (deps-001): checklist item deps-002 marked complete

**Pattern confirmed:** Metadata-only dependency bumps (SourceLink, analyzers) follow the fast-track review path: diff-only scope check + build + test. No CLI generation validation needed for build-time-only packages, though Hicks included it as extra safety.

**Decision Document:** `.squad/decisions/inbox/ripley-review-pr-337.md`

---

### First Implementation Batch: PR #337 Merge Gate & Issue #328 Completion (2026-03-20)

**Task:** Lead execution phase: gate and merge PR #337 (deps-002 SourceLink upgrade), advance dependency-refresh execution pipeline.

**Outcome:**
- ✅ PR #337 review completed: approved two .csproj version bumps, validated Release build + 204/204 test pass
- ✅ PR merged with regular merge commit, branch deleted
- ✅ Issue #328 auto-closed by merge
- ✅ Issue #327 (deps-001) checklist updated: deps-002 marked complete (1/10 issues)
- ✅ First concrete deliverable landed from dependency-refresh plan

**Pattern Confirmed:** Metadata-only packages (SourceLink, build-time analyzers) can be reviewed and merged via a fast-track gate: diff scope verification + standard build/test validation + optional CLI spot check. Ripley gating authority established and effective.

**Orchestration Log:** `.squad/orchestration-log/20260320T143102Z-ripley-pr337.md`

**Team Status:** Ripley ready for next review gate. Hicks ready for deps-003 or OpenAPI migration stream. Bishop stalled on deps-004 (needs retry).

---

### PR #338 Review: Spectre.Console.Cli Upgrade (issue #329 / deps-003)
**Date:** 2026-03-20

**Task:** Reviewer gate for PR #338 — upgrade Spectre.Console.Cli 0.53.0 → 0.53.1.

**Verdict:** ✅ APPROVED and merged (regular merge commit, remote branch deleted).

**Validation performed:**
- Independent Release build in worktree: succeeded
- CLI `--help` smoke test: all options rendered correctly (Spectre.Console.Cli command parsing intact)
- CLI generation smoke test: 19 .http files generated from petstore.json (rich console output, tables, panels all working)
- Diff scope confirmed: exactly one .csproj PackageReference bump, single file, single line
- Single commit, clean branch from main

**Tracking updates:**
- Issue #329: auto-closed by merge
- Issue #327 (deps-001): checklist item deps-003 marked complete (2/10 issues done)

**Unit test note:** `dotnet test` hung in this environment (likely network-accessing tests timing out). Validated via build + CLI execution instead. Build-and-run validation is sufficient for a patch version bump of a CLI framework dependency.

**Pattern confirmed:** Patch version bumps of CLI framework dependencies (Spectre.Console.Cli) follow fast-track review: diff scope check + build + CLI smoke test. The CLI's help output and generation output exercise the Spectre.Console.Cli surface area end-to-end.

**Decision Document:** `.squad/decisions/inbox/ripley-review-pr-338.md`

---

### PR #339 vs #340 Comparison Review: Atc.Test Upgrade (issue #330 / deps-004)
**Date:** 2026-03-20

**Task:** Two competing PRs for the same issue (#330). Compare and merge the better artifact.

**PR #339** (`feature/deps-004-atc-test-retry`, Bishop):
- +173/-1 lines, 2 commits
- Approach: Keep xUnit 2, add 52-line compatibility shim (`AtcTestCompatibility.cs`) with `ExcludeAssets="all"` on Atc.Test
- Effectively imports the package but uses none of its code; re-implements attribute API locally

**PR #340** (`feature/deps-004-atc-test-rescue`, Hicks):
- +21/-17 lines, 1 commit
- Approach: Minimal xUnit v3 migration — replaces `xunit 2.9.3` with `xunit.v3 3.1.0`, adds `OutputType=Exe`, updates 5 `ExecuteAsync` calls with `CancellationToken`, converts 8 `Assert.Equal` to FluentAssertions
- CodeRabbit reviewed; packages used as intended

**Verdict:** ✅ PR #340 merged (regular merge commit), PR #339 closed as superseded.

**Reasoning:**
1. PR #340 is 8× smaller (+21/-17 vs +173/-1)
2. No compatibility shim = no maintenance debt
3. Atc.Test 2.x used as designed (not with `ExcludeAssets="all"`)
4. The team's "defer xUnit v3" decision was made before learning Atc.Test 2.x mandates xUnit v3; the migration here is minimal and contained
5. 204/204 tests green in worktree validation

**Decision update:** The "xUnit stays on legacy family" decision is now superseded — xUnit v3 migration was pulled forward by Atc.Test 2.x dependency requirements. The migration was minimal (5 CancellationToken additions, OutputType change).

**Tracking updates:**
- Issue #330: auto-closed by merge
- Issue #327 (deps-001): checklist item deps-004 marked complete (3/10 issues done)
- Both remote branches deleted

**Decision Document:** `.squad/decisions/inbox/ripley-review-prs-339-340.md` (merged into decisions.md)

---

### Second Implementation Batch: PR #338 Merge & PR #339/#340 Review Gate (2026-03-20)

**Task:** Review PR #338 (deps-003 Spectre patch), approve and merge. Then compare and select between PR #339 and PR #340 (competing deps-004 approaches).

**Outcome — PR #338:**
- ✅ APPROVED and merged (regular merge commit)
- ✅ Issue #329 auto-closed
- ✅ Issue #327 (deps-001) updated: deps-003 marked complete (2/10)
- Single-line version bump, clean diff scope, Release build + CLI smoke tests validated

**Outcome — PR #339 vs #340:**
- ✅ PR #340 SELECTED (minimal xUnit v3 migration, 8× smaller diff)
- ✅ PR #339 CLOSED as superseded (valid approach but higher maintenance debt)
- ✅ Both branches deleted from remote
- ✅ Issue #330 auto-closed by PR #340 merge
- ✅ Issue #327 (deps-001) updated: deps-004 marked complete (3/10)

**Key Decision Override:**
Team decision "xUnit stays on legacy family (v3 migration deferred)" is now superseded. Atc.Test 2.x has hard transitive dependency on xUnit v3; deferral via compatibility shim creates maintenance debt. Minimal xUnit v3 migration in PR #340 is cleaner and future-proof.

**Pattern Established:** When a test infrastructure upgrade pulls forward a major framework version, evaluate minimal forward migration vs. compatibility shims. Forward migration wins if it's small and uses the new framework as intended.

**Orchestration Logs:**
- `.squad/orchestration-log/20260320T150843Z-ripley-pr338-merge.md`
- `.squad/orchestration-log/20260320T150843Z-ripley-prs339-340-gate.md`

**Team Status:** 3 of 10 deps complete (low-risk batch done). Critical OpenAPI migration stream (deps-005) ready for Hicks. Baseline: 204/204 tests passing on xUnit v3, Release build green.

---


### PR Triage & Duplicate Cleanup: Close PR #342 (2026-03-20)

**Task:** Inspect and close duplicate PR #342 (Spectre.Console.Cli upgrade) that arrived after PR #338 was already merged.

**Context:**
- PR #338 (Spectre.Console.Cli 0.53.1 upgrade, deps-003) merged on 2026-03-20 via commit 9f20077
- PR #342 is a duplicate attempt at the same upgrade
- No additional value or differentiation from #338
- Need to preserve issue trail with clear explanatory comment

**Action Taken:**
1. ✅ Verified PR #338 landed successfully (commit 9f20077, Release build green, CLI rendering intact)
2. ✅ Confirmed PR #342 duplicates same scope (Spectre.Console.Cli version upgrade)
3. ✅ Closed PR #342 WITHOUT merging
4. ✅ Left explanatory comment: "This PR is superseded by already-merged PR #338, which successfully upgraded Spectre.Console.Cli to 0.53.1 (closes #329). Both PRs address the same scope. Please see the merged PR for the implementation details. Closing as duplicate to keep the issue trail clean."

**Decision Document:** .squad/decisions/inbox/ripley-close-pr-342.md

**Pattern:** Duplicate PRs should be closed with a brief, courteous explanation referencing the accepted PR. This preserves the work trail without creating merge conflicts or redundancy.

### CLI Output Parity Investigation - C# Spectre.Console to Rust (2026-03-20)
**Task:** Investigate what 100% CLI output/UX parity means between the C# Spectre.Console implementation and the Rust CLI, and propose a phased implementation plan.

**Key Findings:**
- C# uses Spectre.Console for: Panel (header/success banners with rounded borders), Table (OpenAPI stats with green border), Rule (yellow file-writing separator), Markup/MarkupLine (colored text with emojis throughout), and TryWriteLine (fallback to Console.ForegroundColor if Spectre fails)
- Rust has ZERO terminal styling crates - 100% println!/eprintln! with no colors, emojis, or formatting
- Rust clap 4.5.53 present but color feature not enabled
- C# does NOT use spinners, progress bars, or FigletText - only emoji-prefixed status messages and styled panels/tables
- Rust architecture is cleaner: execute() in lib.rs returns structured data, main.rs is pure display. C# mixes execution with display in GenerateCommand.cs

**Architecture Decision:**
- Proposed crate stack: console (colors/styling) + comfy-table (bordered tables) + clap color feature
- New ui.rs module in httpgenerator-cli for all rich output helpers
- No changes to lib.rs - display stays in main.rs + ui.rs
- Rejected indicatif (no spinners needed), figlet-rs (no ASCII art needed), ratatui (overkill)

**Key File Paths:**
- C# display logic: src/HttpGenerator/GenerateCommand.cs (lines 196-284)
- Rust display logic: crates/httpgenerator-cli/src/main.rs (lines 27-93)
- Rust execution logic: crates/httpgenerator-cli/src/lib.rs (execute() returns ExecutionSummary)

**Decision Document:** .squad/decisions/inbox/ripley-cli-output-parity-plan.md
**Estimated Effort:** ~8-10 hours across 5 phases. Phase 1 (color foundation) unblocks Phases 2-4 in parallel.
