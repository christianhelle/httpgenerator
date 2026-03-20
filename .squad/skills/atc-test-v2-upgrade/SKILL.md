# Skill: Atc.Test v2 Upgrade

**Owner:** Bishop (Tester)  
**Context:** HTTP File Generator test-project dependency refreshes involving `Atc.Test` 2.x  
**Last Updated:** 2026-03-20

---

## What This Skill Does

Captures the repo-specific steps for upgrading `Atc.Test` from the 1.x line to `2.0.17` without moving the suite off the pinned `FluentAssertions` 7.2.0 reference.

---

## Core Rules

1. Keep `FluentAssertions` pinned at `7.2.0` in `src/HttpGenerator.Tests/HttpGenerator.Tests.csproj`.
2. Suppress `NU1605` in that test project because `Atc.Test` 2.x transitively requests `FluentAssertions >= 7.2.1`.
3. Replace the direct `xunit` package reference with `xunit.v3` because `Atc.Test` 2.x is built on xUnit v3 extensibility APIs.
4. Fix xUnit v3 analyzer violations instead of suppressing them; in this repo that means passing `TestContext.Current.CancellationToken` into `GenerateCommand.ExecuteAsync(...)`.

---

## Validation

Run:

```powershell
dotnet restore HttpGenerator.sln
dotnet build HttpGenerator.sln --configuration Release
dotnet test HttpGenerator.sln --configuration Release
```

Expected result for this repo:

- Restore succeeds with the pinned `FluentAssertions` reference still present
- Release build succeeds
- `204/204` tests pass

---

## Key Files

- `src/HttpGenerator.Tests/HttpGenerator.Tests.csproj`
- `src/HttpGenerator.Tests/GenerateCommandTests.cs`
