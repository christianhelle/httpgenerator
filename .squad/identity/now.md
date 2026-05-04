---
updated_at: 2026-05-01T11:49:02Z
focus_area: Source layout migration closeout for src/rust + src/dotnet + src/VSCode
completed_issues:
  - Planning complete: target topology locked and implementation plan saved
  - relocate-rust-workspace: moved crates/ to src/rust while preserving repo-root cargo entrypoints
  - relocate-legacy-dotnet: moved legacy/ to src/dotnet and updated solution/project/package paths
  - normalize-ide-host-layout: kept src/VSCode in place and repointed dependent scripts and hosts
  - sweep-path-dependencies: updated CI, docs, smoke tests, release scripts, and instructions
  - validate-migration: cargo, dotnet, smoke, and VS Code validation completed against the new layout
active_issues:
  - squad-followup: continue reducing stale old-path references inside .squad notes and skills as they surface
---

# What We're Focused On

## Current Focus: Source Layout Migration Closeout

The team has completed the repo reorganization that moves product source under `src` while preserving root-level commands. The canonical layout is now `src/rust`, `src/dotnet`, and existing `src/VSCode`; tests, docs, and fixtures remain outside `src`.

## Status Summary

- ✅ Ripley: reviewer gates cleared and final approval recorded
- ✅ Hicks: source relocation and runtime/path rewrites completed
- ✅ Bishop: validation and workflow retargeting completed
- ✅ Hudson: docs and instruction surfaces updated
- 📌 Session directive: use GPT-5.4 for all agents during this session only

## Next Steps

1. Keep internal `.squad` guidance converged on `src/rust` and `src/dotnet`
2. Carry the preserved repo-root entrypoint contract into future docs and workflow edits
3. Treat the pre-existing VS Code packaging metadata issue as separate follow-up work

