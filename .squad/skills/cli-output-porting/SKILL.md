---
name: "cli-output-porting"
description: "How to port Spectre.Console (C#) rich output to Rust console/comfy-table"
domain: "cli-ux"
confidence: "high"
source: "earned — parity investigation comparing GenerateCommand.cs to main.rs"
---

## Context
When porting a C# CLI that uses Spectre.Console to Rust, map each Spectre component to its Rust equivalent. Not every Spectre feature has a 1:1 counterpart — prioritize visual impression over implementation matching.

## Patterns

### Spectre Component → Rust Crate Mapping
| Spectre Component | Rust Equivalent | Crate |
|---|---|---|
| Panel (bordered box with content) | Manual box-drawing with console::style() | console |
| Table (bordered, styled cells) | comfy_table::Table with presets | comfy-table |
| Rule (horizontal separator with text) | Manual line with console::style() | console |
| Markup/MarkupLine (inline colored text) | console::style("text").color() | console |
| AnsiConsole.Write(panel/table) | println! with formatted content | console + comfy-table |
| Color.Blue/Green/Red/Yellow | Color::Blue/Green/Red/Yellow | console |
| [bold], [dim] | .bold(), .dim() | console |
| [link] (OSC 8 hyperlinks) | console::style("text").for_stderr().linkify() or manual OSC 8 | console |
| TryWriteLine fallback | console crate auto-strips ANSI in non-TTY | console |

### Architecture Pattern
Keep display logic (colors, panels, tables) in a dedicated ui.rs module. The execution function should return structured data (enums, structs), never print directly. The main.rs orchestrates execution + display.

### Terminal Detection
The `console` crate's `Term::stdout().is_term()` handles non-TTY detection. ANSI codes are stripped automatically when piped. No manual fallback needed (unlike C# TryWriteLine try-catch).

## Examples

Reference files:
- C# source of truth: `src/HttpGenerator/GenerateCommand.cs` lines 196-284
- Rust target: `crates/httpgenerator-cli/src/main.rs` + new `ui.rs`

## Anti-Patterns
- Do NOT add `indicatif` spinners if C# doesn't use `AnsiConsole.Status()` or `AnsiConsole.Progress()`
- Do NOT add `figlet-rs` if C# uses `Panel` with `Markup`, not `FigletText`
- Do NOT put display logic in lib.rs — keep it in main.rs + ui.rs
- Do NOT try to match Spectre.Console.Cli help rendering exactly — Clap's colored help is its own convention and is acceptable
