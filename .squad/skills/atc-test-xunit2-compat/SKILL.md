# Skill: Atc.Test 2.x on xUnit 2

**Owner:** Bishop (Tester)  
**Context:** HTTP File Generator test-project dependency refreshes where `Atc.Test` advances to 2.x before the repo is ready to migrate to xUnit 3  
**Last Updated:** 2026-03-20

---

## What This Skill Does

Preserves the existing xUnit 2 test suite when `Atc.Test` 2.x introduces xUnit 3 compile-time dependencies. It upgrades the NuGet package while keeping current `[Theory, AutoNSubstituteData]` and inline auto-data test patterns working inside `src/HttpGenerator.Tests/`.

---

## When to Use

- A dependency-refresh PR upgrades `Atc.Test` from 1.x to 2.x
- The repo is still pinned to xUnit 2
- Build errors mention `FactAttribute`, `TheoryAttribute`, or `InlineDataAttribute` existing in both `xunit.core` and `xunit.v3.core`
- `FluentAssertions` must stay pinned at `7.2.0`

---

## Why This Happens

`Atc.Test` 1.1.18 depended on `AutoFixture.Xunit2`, but `Atc.Test` 2.0.17 depends on `AutoFixture.Xunit3` and `xunit.v3.extensibility.core`. In this repo that creates compile-time clashes with the existing xUnit 2 packages long before any real test logic runs.

---

## Repo Pattern

Update `src/HttpGenerator.Tests/HttpGenerator.Tests.csproj` like this:

- bump `Atc.Test` to `2.0.17`
- keep `FluentAssertions` at `7.2.0`
- suppress `NU1605` because `Atc.Test` now wants `FluentAssertions >= 7.2.1`
- add explicit `AutoFixture.AutoNSubstitute` and `AutoFixture.Xunit2` references
- exclude `Atc.Test` assets from compilation so xUnit 3 attributes do not collide

Then add a local compatibility shim:

- file: `src/HttpGenerator.Tests/AtcTestCompatibility.cs`
- namespace: `Atc.Test`
- provide:
  - `FixtureFactory.Create()`
  - `AutoNSubstituteDataAttribute : AutoDataAttribute`
  - `InlineAutoNSubstituteDataAttribute : CompositeDataAttribute`

Use the old 1.1.18 behavior as the template:

- `OmitOnRecursionBehavior`
- `AutoNSubstituteCustomization`
  - `ConfigureMembers = false`
  - `GenerateDelegates = true`

This keeps existing tests compiling without touching individual test methods.

---

## Validation

Run:

```powershell
dotnet restore HttpGenerator.sln
dotnet build HttpGenerator.sln --configuration Release
dotnet test HttpGenerator.sln --configuration Release
```

Expected current result in this repo:

- Build passes
- Test suite passes with **204/204** green

---

## Anti-Patterns

- ❌ Don’t silently upgrade the whole repo to xUnit 3 inside the Atc.Test refresh
- ❌ Don’t unpin `FluentAssertions` in the same PR when the explicit goal is to keep it unchanged
- ❌ Don’t hand-edit dozens of tests when a namespace-level compatibility shim can preserve the existing attribute API
