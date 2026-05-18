---
updated_at: 2026-05-18T11:53:21.547+02:00
focus_area: VSIX async right-click generation implementation in src/dotnet/HttpGenerator.VSIX
completed_issues:
  - Planning complete: target topology locked and implementation plan saved
  - relocate-rust-workspace: moved crates/ to src/rust while preserving repo-root cargo entrypoints
  - relocate-legacy-dotnet: moved legacy/ to src/dotnet and updated solution/project/package paths
  - normalize-ide-host-layout: kept src/VSCode in place and repointed dependent scripts and hosts
  - sweep-path-dependencies: updated CI, docs, smoke tests, release scripts, and instructions
  - validate-migration: cargo, dotnet, smoke, and VS Code validation completed against the new layout
active_issues:
  - implement-vsix-async-flow: migrate the Visual Studio extension to Solution Explorer right-click generation with background execution, settings UI, deterministic CLI lookup, and non-blocking progress/results
---

# What We're Focused On

## Current Focus: VSIX Async Right-Click Generation

The team is now implementing the Visual Studio extension redesign in `src/dotnet/HttpGenerator.VSIX`: right-click invocation from Solution Explorer, background generation after the initiating UI closes, persisted settings edited through a non-blocking UI, and deterministic `httpgenerator.exe` resolution.

## Status Summary

- 📌 Session directive: use Claude Opus 4.7 for all agents during this session only
- 🎯 Active implementation target: VSIX async background generation plan approved and ready for execution

## Next Steps

1. Confirm the exact SDK seams for context-menu placement, settings UI, and non-modal details
2. Implement the VSIX request/coordinator/settings refactor
3. Validate the new command flow, CLI lookup, cancellation, and notifications
