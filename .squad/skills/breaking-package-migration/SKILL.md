# Breaking Package Migration Pattern

**Status:** Documented from httpgenerator dependency refresh  
**Applicable To:** Any major NuGet package upgrade with breaking API changes  
**Example:** Microsoft.OpenApi 1.6.28 → 3.3.1  

---

## Overview

When upgrading a major version of a NuGet package that introduces breaking changes (namespace changes, API surface changes, behavior changes), a naive single-PR approach risks merge conflicts and unclear history. This pattern breaks the migration into three sequential PRs for clarity and rollback safety.

---

## The Pattern: Three-Phase Migration

### Phase 1: Package Upgrade (PR 1)
**Branch:** `chore/upgrade-{package-name}`  
**Scope:** Update `.csproj` files only, no code changes

```xml
<!-- BEFORE -->
<PackageReference Include="Microsoft.OpenApi.Readers" Version="1.6.28" />

<!-- AFTER -->
<PackageReference Include="Microsoft.OpenApi" Version="3.3.1" />
<PackageReference Include="Microsoft.OpenApi.YamlReader" Version="3.3.1" />
```

**Actions:**
1. Update all `.csproj` files referencing the package
2. Run `dotnet restore`
3. Expect compilation errors (don't fix yet!)
4. Document all breaking changes found in PR description
5. Do NOT merge if build fails (resolve in next phase)

**Risk:** LOW — Only touching dependency declarations

**Review Gate:** Ensure package versions are correct, no extraneous changes

---

### Phase 2: Code Refactoring (PR 2)
**Branch:** `refactor/{component}-{package}-migration`  
**Scope:** Update all code to use new API surface  
**Depends On:** Phase 1 PR merged

**Actions:**
1. Create small commits for each migration change:
   - Namespace updates (one logical group per commit)
   - API surface changes (factory patterns, visitor updates)
   - Async/await propagation
   - Null-safety additions
2. Verify build succeeds after each logical group
3. Run unit tests after each group

**Example Commits:**
```
commit 1: refactor: update Microsoft.OpenApi namespaces in HttpFileGenerator.cs
commit 2: refactor: replace OpenApiStreamReader with OpenApiDocument.LoadAsync()
commit 3: refactor: update visitor pattern for IOpenApiReferenceHolder
commit 4: refactor: add null guards for auto-initialized component collections
commit 5: test: verify async document loading with multiple OpenAPI versions
```

**Risk:** MEDIUM → MEDIUM — This is the high-risk phase. Small commits enable quick rollback.

**Review Gate:** Ripley reviews incremental commits; may request changes at any point

**Async Propagation Pattern:**
```csharp
// OLD SYNC
public void LoadDocument(string path)
{
    using var stream = File.OpenRead(path);
    var document = new OpenApiStreamReader().Read(stream, out var diagnostic);
    ProcessDocument(document);
}

// NEW ASYNC (may require upstream refactoring)
public async Task LoadDocumentAsync(string path)
{
    using var stream = File.OpenRead(path);
    var document = await OpenApiDocument.LoadAsync(stream);
    await ProcessDocumentAsync(document);
}
```

---

### Phase 3: Regression Testing (PR 3)
**Branch:** `test/{component}-{package}-regression`  
**Scope:** New tests for updated API  
**Depends On:** Phase 2 PR merged

**Actions:**
1. Add integration tests for new async API surface
2. Add null-safety tests (empty collections, missing refs)
3. Test all supported versions (e.g., OpenAPI v2, v3.0, v3.1)
4. Run smoke tests (full integration test suite)

**Example Tests:**
```csharp
[Fact]
public async Task LoadDocumentAsync_WithValidV3Spec_Succeeds()
{
    // Test: new OpenApiDocument.LoadAsync() factory
    var path = "test-specs/v3.0/petstore.json";
    var document = await OpenApiDocument.LoadAsync(File.OpenRead(path));
    
    Assert.NotNull(document);
    Assert.NotNull(document.Components?.Schemas);
}

[Fact]
public async Task LoadDocumentAsync_WithNullComponentCollections_HandlesGracefully()
{
    // Test: v3 no longer auto-initializes collections
    var path = "test-specs/minimal-spec.json";
    var document = await OpenApiDocument.LoadAsync(File.OpenRead(path));
    
    var schemas = document.Components?.Schemas ?? new Dictionary<string, IOpenApiSchema>();
    Assert.Empty(schemas);
}
```

**Risk:** LOW — Adding tests only, no production code changes

**Review Gate:** Ensure tests are comprehensive, cover null cases, verify smoke tests pass

---

## Benefits of This Pattern

| Aspect | Benefit |
|--------|---------|
| **Rollback** | If Phase 2 fails midway, revert only that PR; Phase 1 package upgrade remains clean |
| **History** | Git log clearly shows: package upgrade → code migration → regression tests |
| **Review** | Phase 1 is trivial; Phase 2 can be reviewed commit-by-commit; Phase 3 is regression baseline |
| **Parallel Testing** | Phase 3 tests can be written while Phase 2 is in review |
| **Documentation** | Each PR serves as documentation of what changed and why |

---

## Checklist for Breaking Package Migrations

### Phase 1 PR
- [ ] Identify all `.csproj` files that reference the package
- [ ] Find all breaking changes in package release notes
- [ ] Document namespace changes, API surface changes, async changes
- [ ] Attempt restore: expect compilation errors (OK for now)
- [ ] List errors in PR description

### Phase 2 PR
- [ ] Create commits in logical groups (namespaces, APIs, async, null-safety)
- [ ] Verify build succeeds after each commit
- [ ] Run unit tests after logical groups
- [ ] Reference specific code files that changed
- [ ] Highlight any async propagation upstream

### Phase 3 PR
- [ ] Add integration tests for new async surface
- [ ] Add null-safety tests
- [ ] Test all supported versions/formats
- [ ] Run full smoke test suite
- [ ] Verify no regressions in generated output

---

## Anti-Patterns to Avoid

❌ **Single Mega-PR:** Package upgrade + all code changes + tests in one PR  
→ Hard to review, hard to bisect if issues arise

❌ **Code-First:** Change code expecting new API, then upgrade package  
→ Compilation fails until you've updated everything; unclear what broke

❌ **Async Everywhere:** Make everything async at once  
→ May introduce unnecessary complexity; consider if partial async is acceptable

❌ **Skipping Regression Tests:** "It compiles, ship it"  
→ Misses edge cases (null collections, async timing, etc.)

---

## Reference Implementation

**Repository:** httpgenerator  
**Issue:** Dependency refresh 2026-03-19  
**PRs:**
1. dep-01: `chore/upgrade-microsoft-openapi-v3` (package update)
2. code-01: `refactor/openapi-v3-migration` (code refactoring)
3. test-01: `test/openapi-v3-regression-tests` (regression tests)

---

## Lessons Learned

- **Namespace updates are tedious but low-risk** → IDE refactoring tools help
- **Async propagation is often the hardest part** → Plan carefully which APIs must be async
- **Null-safety is subtle** → Add guard clauses early, test edge cases
- **Smoke tests are essential** → Verify full integration after migration
- **Small commits matter** → Makes review easier and history clearer

---

## Microsoft.OpenApi v3.4.0 Audit Notes

Concrete migration details captured while auditing HTTP File Generator:

- `OpenApiDocument.LoadAsync(...)` replaces `OpenApiStreamReader`, but it returns `Microsoft.OpenApi.Reader.ReadResult`, not `OpenApiDocument` directly.
- Stream overloads of `OpenApiDocument.LoadAsync` require an explicit format string (`OpenApiConstants.Json` or `OpenApiConstants.Yaml`); format detection must be handled by the caller.
- `ReadResult` property names changed from the older reader surface: use `result.Document` and `result.Diagnostic` instead of `result.OpenApiDocument` and `result.OpenApiDiagnostic`.
- `OpenApiUnsupportedSpecVersionException` moved to the root namespace: `Microsoft.OpenApi.OpenApiUnsupportedSpecVersionException`.
- Reader-specific types now live under `Microsoft.OpenApi.Reader` (for example `OpenApiReaderSettings` and `OpenApiDiagnostic`).
- `OpenApiVisitorBase` is interface-heavy in v3. Common overrides become `Visit(IOpenApiParameter)`, `Visit(IOpenApiSchema)`, `Visit(IOpenApiPathItem)`, `Visit(IOpenApiRequestBody)`, `Visit(IOpenApiLink)`, `Visit(IOpenApiCallback)`, `Visit(IOpenApiReferenceHolder)`, and `Visit(IDictionary<string, IOpenApiHeader>)`.
- `OpenApiPathItem.Operations` becomes `Dictionary<HttpMethod, OpenApiOperation>`. Stringifying the key still works, but enum-specific assumptions do not.
- Async serialization moved to extension methods such as `SerializeAsJsonAsync()` and `SerializeAsYamlAsync()` on `OpenApiSerializableExtensions`.
- YAML support is no longer implicit in the old Readers package migration path; install `Microsoft.OpenApi.YamlReader` alongside `Microsoft.OpenApi`.
- OpenAPI.NET v3.4.0 parses the local OpenAPI 3.1 webhook sample successfully, so any "downgrade 3.1 to 3.0" workaround should be treated as migration debt and re-validated.
