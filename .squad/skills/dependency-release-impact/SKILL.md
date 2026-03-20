# Skill: Dependency Release Impact Assessment

**Owner:** Hudson (DevRel/Docs)  
**Context:** HTTP File Generator with three distribution channels (CLI, Core lib, VSIX extensions)  
**Last Updated:** 2025-01-25

---

## What This Skill Does

Evaluates docs, packaging, and release consequences when dependency versions change in HTTP File Generator. Focuses on:
- Documentation updates required
- Release workflow implications
- GitHub issue planning for release teams
- Validation checkpoints

---

## When to Use

When Renovate opens a PR with dependency updates OR when planning a release cycle that includes new dependency versions.

**Trigger Phrases:**
- "What docs work is needed for this dependency update?"
- "Does [library] version bump need release notes?"
- "Plan the release impact of updating [dependency]"
- "Create GitHub issues for dependency upgrade preparation"

---

## Dependency Categories & Impact Matrix

### Category 1: OpenAPI Parsing Libraries
**Packages:** `Microsoft.OpenApi.*`, `Microsoft.OpenApi.Readers`, `Microsoft.OpenApi.OData`

**Impact Areas:**
- Supported OpenAPI versions (v2.0, v3.0, v3.1)
- `--skip-validation` flag requirements
- Generated .http file validity
- Example command output in README

**Checklist:**
```
- [ ] Run all test/OpenAPI/v*/ specs through generation
- [ ] Verify README examples still work
- [ ] Check if `--skip-validation` requirement changed
- [ ] Document any removed/deprecated format support
- [ ] Test with petstore.json (v2.0 and v3.0)
```

**Docs Updates:**
- README: Update supported versions matrix
- CHANGELOG: Note breaking changes if any
- Add migration guide if parsing behavior changes

---

### Category 2: CLI Framework & Command Structure
**Packages:** `Spectre.Console.Cli`

**Impact Areas:**
- CLI command syntax
- `--help` output format
- Command options and flags
- Error message styling

**Checklist:**
```
- [ ] Run `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- --help`
- [ ] Compare output against README usage examples
- [ ] Verify all flags from README still exist
- [ ] Test example commands from README still work
- [ ] Check if new options should be documented
```

**Docs Updates:**
- README: Update usage section if command structure changes
- README: Add new flag examples if significant features added
- CHANGELOG: Document user-facing option additions/removals

---

### Category 3: Framework & Runtime Versions
**Packages:** `Microsoft.NET.Sdk` (implicit), target frameworks in `.csproj`

**Impact Areas:**
- Installation via `dotnet tool install`
- System requirements (.NET version)
- VSIX compatibility (Visual Studio version)
- Core library portability (netstandard2.0)

**Checklist:**
```
- [ ] If CLI target changes from net8.0, update README "Installation"
- [ ] If VSIX target changes from net4.7.2, update VS 2022 compat notes
- [ ] Test CLI installs with `dotnet tool install --global httpgenerator`
- [ ] Verify VSIX still loads in Visual Studio 2022
- [ ] Check if .NET SDK minimum version requirement changed
```

**Docs Updates:**
- README: System requirements section
- NuGet package description (Directory.Build.props)
- VSIX marketplace description
- CHANGELOG: Note runtime requirement changes

---

### Category 4: Authentication & Azure Integration
**Packages:** `Microsoft.Extensions.Azure`, `Exceptionless`

**Impact Areas:**
- `--azure-scope` parameter behavior
- `--azure-tenant-id` parameter behavior
- `--no-logging` opt-out effectiveness
- Telemetry collection behavior
- Error reporting mechanism

**Checklist:**
```
- [ ] Test `--azure-scope [scope]` still functions
- [ ] Test `--azure-tenant-id [id]` still functions
- [ ] Verify `--no-logging` still disables telemetry completely
- [ ] Check if Exceptionless privacy/redaction still works (headers redacted, etc.)
- [ ] Verify no new telemetry added without opt-out
```

**Docs Updates:**
- README: "Error Logging, Telemetry, and Privacy" section
- CHANGELOG: Any privacy/security-related changes
- Examples: Update Azure auth examples if credential flow changed

---

### Category 5: JSON Serialization & Parsing
**Packages:** `System.Text.Json`

**Impact Areas:**
- Generated .http file JSON formatting
- Custom header JSON encoding
- Request body JSON generation
- Escape character handling

**Checklist:**
```
- [ ] Generate .http files and inspect JSON formatting
- [ ] Test with special characters in --custom-header values
- [ ] Verify request body JSON is valid and pretty-printed
- [ ] Test Unicode and escape sequences in schemas
```

**Docs Updates:**
- CHANGELOG: Breaking formatting changes (if any)
- README examples: Update if generated output format changes

---

## GitHub Issue Template: Release-Prep Validation

```markdown
### Title
[RELEASE-PREP] Validate {Package} {Old Version} → {New Version} impact

### Body
**Dependency:** {Package}  
**Change:** {Old Version} → {New Version}  
**Type:** [Major|Minor|Patch]

**Validation Tasks:**
- [ ] Smoke tests pass: `cd test && ./smoke-tests.sh`
- [ ] CLI help unchanged: `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- --help`
- [ ] Petstore generation works: Basic test with v2.0 and v3.0 specs
- [ ] README examples all execute successfully
- [ ] No regressions in generated .http file format
- [ ] [If applicable] Breaking change assessment

**Docs Impact:**
- [ ] README requires updates
- [ ] NuGet description requires updates
- [ ] CHANGELOG entry needed (user-facing)
- [ ] Migration guide needed

**Release Notes:**
[Describe user-facing changes or link to upgrade guide]

**Labels:** release-prep, documentation, {category-tag}
```

---

## GitHub Issue Template: Docs-Required Update

```markdown
### Title
[DOCS] {Update Description} for {Feature/Version}

### Body
**Dependency/Change:** {Context}  
**Files to Update:**
- [ ] README.md
  - [ ] Installation section
  - [ ] Usage examples
  - [ ] Feature availability
- [ ] CHANGELOG.md (if not auto-generated)
- [ ] NuGet package descriptions (.csproj)
- [ ] VSIX marketplace description
- [ ] docs/ folder (if applicable)

**Changes Needed:**
- {Specific line/section and change}

**Validation:**
- [ ] All links work
- [ ] Examples execute correctly
- [ ] No outdated version references remain

**Labels:** documentation, help-wanted
```

---

## Release Workflow Integration

### Step 1: Dependency Update (Renovate PR)
- [ ] Wait for CI to pass (smoke tests, build, unit tests)
- [ ] Hudson (DevRel) assesses doc impact
- [ ] If breaking: Create `[RELEASE-PREP]` GitHub issue
- [ ] If low-risk: Approve for merge

### Step 2: Merge to Main
- [ ] Automated changelog generation runs
- [ ] Review generated CHANGELOG.md for accuracy

### Step 3: Release Planning
- [ ] Pull version from release.yml
- [ ] Ensure CHANGELOG.md includes all updates
- [ ] Review release notes for user-facing changes
- [ ] Push to release branch (triggers publishing)

### Step 4: Post-Release
- [ ] Verify NuGet package reflects dependency updates
- [ ] Monitor marketplace (VSIX/VSCode) for version updates
- [ ] Check that README examples work with published tool

---

## Decision Tree: Is This a Docs Task?

```
Does dependency update change...?

├─ Supported OpenAPI versions (v2/v3/v3.1)?
│  └─ YES → Update README feature matrix, add migration guide
│
├─ CLI command structure or options?
│  └─ YES → Update README usage section, example commands
│
├─ System requirements (.NET, VS version)?
│  └─ YES → Update README "Installation" section
│
├─ Azure auth behavior?
│  └─ YES → Update README Azure examples, verify flow still works
│
├─ Telemetry/privacy opt-out?
│  └─ YES → Update README "Error Logging" section, test --no-logging
│
├─ Generated .http file format?
│  └─ YES → Update README example output, CHANGELOG entry
│
└─ Build/test infrastructure only?
   └─ NO DOCS TASK → Just update dependency, document in CHANGELOG
```

---

## Validation Checklist (General)

Always run before releasing:

```
- [ ] `dotnet build HttpGenerator.sln --configuration Release` succeeds
- [ ] `dotnet test HttpGenerator.sln --configuration Release` passes
- [ ] `cd test && ./smoke-tests.sh` passes
- [ ] `dotnet run --project src/HttpGenerator/HttpGenerator.csproj -- test/OpenAPI/v3.0/petstore.json --output /tmp/test --no-logging` generates 19 .http files
- [ ] README examples execute without modification
- [ ] NuGet package description reflects latest changes
- [ ] CHANGELOG.md is up-to-date
- [ ] No stale version references in docs
```

---

## Anti-Patterns to Avoid

❌ **Don't manually edit CHANGELOG.md** — Let github_changelog_generator handle it  
❌ **Don't change version in .csproj files** — Version lives in release.yml only  
❌ **Don't commit dependency changes to release branch** — Only release.yml touch it  
❌ **Don't skip smoke tests** — They validate end-to-end generation correctness  
❌ **Don't forget README examples** — They're the primary user documentation

---

## Key Files & Locations

| Purpose | File |
|---------|------|
| Main docs | `README.md` |
| Release history | `CHANGELOG.md` (auto-generated) |
| CLI metadata | `src/HttpGenerator/HttpGenerator.csproj` |
| Core lib metadata | `src/HttpGenerator.Core/HttpGenerator.Core.csproj` |
| Shared metadata | `src/Directory.Build.props` |
| VSIX version | `src/HttpGenerator.VSIX/Properties/AssemblyInfo.cs` |
| Release version | `.github/workflows/release.yml` |
| Changelog generation | `.github/workflows/changelog.yml` |
| Dependency rules | `renovate.json` |

---

## Examples from HTTP File Generator History

### Example 1: Spectre.Console.Cli 0.52.0 → 0.53.0
**Type:** Minor update  
**Action:** Merge PR, no docs task (unless help format changes)  
**Why:** CLI command structure stable in minor updates

### Example 2: Microsoft.OpenApi v1.x → v2.0
**Type:** Major update  
**Action:** Create `[RELEASE-PREP]` issue, assess OpenAPI version support  
**Docs:** Update supported versions in README, add migration notes if parsing changes  
**Why:** Major versions often include breaking changes in core functionality

### Example 3: System.Text.Json 9.0.x → 10.0.x
**Type:** Minor update  
**Action:** Verify generated .http JSON still valid, test special characters  
**Docs:** Only if output format changes (unlikely in minor update)  
**Why:** JSON serialization affects user-visible generated files

---

## See Also

- `.squad/decisions/inbox/hudson-release-impact.md` — Full release impact analysis framework
- `.squad/agents/hudson/history.md` — Hudson's learnings and context
- HTTP File Generator custom instructions — Build & test patterns, known issues
