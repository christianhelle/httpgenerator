# Scribe — History

## Core Context

**Project:** HTTP File Generator (`httpgenerator`)
**User:** Christian Helle

Scribe maintains `.squad` memory: orchestration logs, session logs, decision merges, and cross-agent history upkeep.

## Learnings

### Historical Summary
- Team initialized 2026-03-19 with Ripley (Lead), Hicks (Core Dev), Bishop (Tester), Hudson (DevRel/Docs), and Scribe.

### Source Layout Migration Session Closeout (2026-05-01)
- Recorded orchestration outcomes for Ripley, Hicks, Bishop, and Hudson under `.squad\orchestration-log\` using UTC ISO-style timestamps.
- Logged the session in `.squad\log\` and merged the current decision inbox into `decisions.md`, then cleared `.squad\decisions\inbox\`.
- Refreshed affected agent histories and summarized oversized history files into compact core context so internal notes stay useful after the source-layout migration.
- Session directive: all spawned agents used GPT-5.4 for this session only.
