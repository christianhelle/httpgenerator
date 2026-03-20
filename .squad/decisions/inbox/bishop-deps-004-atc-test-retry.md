# Bishop Decision Inbox — deps-004 Atc.Test Retry

## Context

Issue #330 requires upgrading `Atc.Test` to `2.0.17` while keeping `FluentAssertions` pinned and respecting the team's decision to defer an xUnit v3 migration.

## Decision

Keep the repo on xUnit 2 for now and isolate the `Atc.Test` 2.x breaking change inside the test project by:

- updating `src/HttpGenerator.Tests/HttpGenerator.Tests.csproj` to `Atc.Test` `2.0.17`
- suppressing `NU1605` for the intentional `FluentAssertions` `7.2.0` pin
- adding explicit `AutoFixture.AutoNSubstitute` and `AutoFixture.Xunit2` references
- providing a local `Atc.Test` compatibility shim in `src/HttpGenerator.Tests/AtcTestCompatibility.cs`

## Why

`Atc.Test` 2.0.17 now depends on xUnit v3-facing packages, which causes compile-time attribute conflicts with the repo's current xUnit 2 test suite. A local shim preserves the existing `[Theory, AutoNSubstituteData]` / `[Inline(...)]` patterns without expanding this issue into the separate xUnit migration stream the team already deferred.

## Validation

- `dotnet restore HttpGenerator.sln`
- `dotnet build HttpGenerator.sln --configuration Release`
- `dotnet test HttpGenerator.sln --configuration Release`

All passed locally with 204/204 tests green.
