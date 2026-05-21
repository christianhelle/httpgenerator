---
updated_at: 2026-05-21T14:35:15.308+02:00
focus_area: Plan a comprehensive rustdoc pass for httpgenerator-core on docs.rs
completed_issues:
  - Source layout migration closeout remains complete across src/rust, src/dotnet, and src/vscode
active_issues:
  - docs-rs-rustdoc-pass: define scope and execute a comprehensive inline documentation pass for httpgenerator-core
  - session-directive: use GPT-5.5 for spawned agents during this session only
  - session-directive: preserve detailed history with small logical commits and no co-author during implementation
---

# What We're Focused On

## Current Focus: httpgenerator-core docs.rs documentation plan

The team is planning a documentation-focused pass over the public `httpgenerator-core` Rust API so docs.rs becomes useful for external consumers. The repo topology work is complete; the active concern is documentation quality and discoverability.

## Status Summary

- 📌 Session directive: use GPT-5.5 for all spawned agents during this session only
- 📌 Session directive: prefer small logical commits without a co-author during implementation
- 🔎 Planning question: docs-only rustdoc pass versus docs-surface cleanup on docs.rs

## Next Steps

1. Confirm the acceptable scope for the documentation pass
2. Fan out inventory and authoring work across the Rust core public surface
3. Validate the docs changes with the standard Rust, .NET, and smoke commands
