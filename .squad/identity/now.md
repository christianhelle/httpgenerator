---
updated_at: 2026-03-20T15:08:43Z
focus_area: Low-risk dependency batch complete (3/10), OpenAPI migration in progress
completed_issues:
  - deps-002: SourceLink upgrade ✅ merged PR #337
  - deps-003: Spectre.Console.Cli patch ✅ merged PR #338
  - deps-004: Atc.Test upgrade ✅ merged PR #340 (xUnit v3 minimal migration, superseding defer decision)
active_issues:
  - deps-005: OpenAPI reader pipeline migration (IN PROGRESS, CRITICAL)
  - deps-006: OpenAPI visitor/stats migration (planned)
  - deps-007: OpenAPI generator/CLI reconciliation (planned)
  - deps-008: OpenAPI regression tests (planned)
  - deps-009: VSIX SDK refresh (planned, parallelizable)
  - deps-010: Final regression + docs closeout (planned)
---

# What We're Focused On

## Current Focus: Low-Risk Dependency Batch Complete — OpenAPI Migration Stream In Progress

Three low-risk dependency updates landed. The critical OpenAPI v3 migration stream is in progress. Major decision made: xUnit v3 migration pulled forward due to Atc.Test 2.x hard dependency (vs deferral).

- ✅ **Issue #328 (deps-002):** Microsoft.SourceLink.GitHub — PR #337 merged
- ✅ **Issue #329 (deps-003):** Spectre.Console.Cli — PR #338 merged
- ✅ **Issue #330 (deps-004):** Atc.Test — PR #340 merged (chose over PR #339; xUnit v3 alignment, see decision doc)
- 🔄 **Issue #331 (deps-005):** OpenAPI reader pipeline migration — IN PROGRESS (critical blocker)
- 📋 **Issues #332–#336 (deps-006–deps-010):** Ready for implementation

## Status Summary

- ✅ Ripley: 3/10 deps-001 checklist items complete; ready for next review gate
- ✅ Hicks: IN PROGRESS on deps-005 (OpenAPI reader pipeline) — critical path
- ✅ Bishop: Available for deps-008 (OpenAPI regression tests) after deps-005 lands
- 🎯 Baseline: 204/204 tests passing on xUnit v3, build green

## Next Steps

1. **Hicks** to continue `deps-005` (Migrate OpenAPI reader pipeline to v3) — HIGH PRIORITY
2. **Hicks** can parallelize `deps-009` (VSIX SDK refresh) once deps-005 is moving
3. **Bishop** prepares for `deps-008` (OpenAPI regression tests) — ready once deps-005 lands
4. **Ripley** gates all PRs

## Key Decisions Locked

- ✅ Keep `FluentAssertions` pinned during refresh
- ✅ Isolate `Microsoft.OpenApi` migration as dedicated stream
- ✅ VSIX SDK updates separate from main solution track
- ✅ **xUnit v3 migration complete** (SUPERSEDES earlier "defer xUnit v3" decision — pulled forward by Atc.Test 2.x hard dependency)
- ✅ Regular merge commits (no squash/rebase)
- ✅ Feature/fix branches per issue with Ripley PR gates

## Session Artifacts

- **Orchestration Logs:** 6 entries documenting completed PRs #337, #338, #340, and deps-005 kickoff
- **Session Log:** `.squad/log/20260320-second-implementation-batch.md` — comprehensive summary of batch progress
- **Decision Documents (merged to decisions.md):**
  - PR #338 review (deps-003)
  - PR #339 vs #340 verdict (deps-004, xUnit v3 override)

