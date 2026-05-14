# PRD: VS Code Extension Migration from .NET Tool to Rust CLI

## 1. Background

### 1.1 Current State

The VS Code extension `http-file-generator` (published as `ChristianResmaHelle.http-file-generator`) currently depends on the **legacy .NET global tool** (`httpgenerator` from NuGet). The extension:

- Installs/updates the .NET tool via `dotnet tool install --global httpgenerator`
- Executes the tool by creating a VS Code terminal and sending `httpgenerator` commands
- Checks for the .NET tool version via `dotnet tool list -g`
- Prompts users to install the .NET tool if not found
- Requires .NET 8.0 SDK to be installed on the user's machine

### 1.2 Problem

- Users must have .NET 8.0 SDK installed just to run the extension
- The .NET tool is marked as "legacy" in the project README
- The project's primary engine is now the **Rust CLI** (`httpgenerator`)
- The VSIX (Visual Studio 2022 extension) already bundles the Rust binary and works correctly
- The .NET tool installation adds ~30s overhead on first use
- Cross-platform compatibility issues with .NET tool installation

### 1.3 Target State

The VS Code extension will:
- **No longer depend on .NET or the .NET tool**
- Download the Rust CLI binary from GitHub Releases at runtime (platform-specific)
- Cache the binary in the extension's globalStorage directory
- Execute the binary via VS Code terminal (same UX as before)
- Support user-configurable `executablePath` setting for power users

---

## 2. Requirements

### 2.1 Functional Requirements

| # | Requirement | Priority |
|---|---|---|
| FR-1 | Extension must detect and use `http-file-generator.executablePath` setting if configured | Must |
| FR-2 | Extension must search PATH for `httpgenerator` (or `httpgenerator.exe` on Windows) before downloading | Must |
| FR-3 | Extension must download the correct platform-specific binary from GitHub Releases when not found on PATH | Must |
| FR-4 | Downloaded binary must be cached in extension's globalStorage directory | Must |
| FR-5 | Extension must check for CLI updates on every `activate()` call | Must |
| FR-6 | Extension must show progress notification during download | Should |
| FR-7 | Extension must show status bar indicator after successful CLI detection | Should |
| FR-8 | Extension must show error message with retry option if download fails | Must |
| FR-9 | Extension must provide "Reset CLI" command to delete cached binary | Should |
| FR-10 | Extension must provide "Show CLI Path" command to display detected binary path | Should |
| FR-11 | Extension must preserve existing command IDs and behavior (no breaking changes to user-facing API) | Must |
| FR-12 | Extension must continue using VS Code terminal for CLI execution output | Should |

### 2.2 Non-Functional Requirements

| # | Requirement | Priority |
|---|---|---|
| NFR-1 | Extension must work on: Windows x64, Linux x64, macOS x64, macOS ARM64 | Must |
| NFR-2 | Extension must work offline after first download | Must |
| NFR-3 | Download must complete within 30s on typical connections | Should |
| NFR-4 | Extension activation must not block due to download (async, non-blocking) | Must |
| NFR-5 | No .NET SDK or .NET runtime dependency | Must |
| NFR-6 | VSIX package size increase must be < 5MB (download-only approach = minimal increase) | Should |

---

## 3. Architecture

### 3.1 Executable Resolution Chain

```
┌─────────────────────────────────────────────────────────┐
│                  findExecutable()                        │
│                                                          │
│  1. Check executablePath setting (package.json config)  │
│         ↓ (not found / doesn't exist)                   │
│  2. Check PATH for 'httpgenerator' (or .exe on Win)    │
│         ↓ (not found on PATH)                           │
│  3. Check globalStorage for cached binary               │
│         ↓ (not in cache)                                │
│  4. Download from GitHub Releases                       │
│         ↓                                               │
│  5. Return resolved path (or undefined on failure)      │
└─────────────────────────────────────────────────────────┘
```

### 3.2 GitHub Releases API Contract

**Endpoint:**
```
GET https://api.github.com/repos/christianhelle/httpgenerator/releases/tags/<version>
```

**Response (key fields):**
```json
{
  "tag_name": "v1.1.0",
  "assets": [
    {
      "name": "httpgenerator-win-x64.zip",
      "browser_download_url": "https://github.com/christianhelle/httpgenerator/releases/download/v1.1.0/httpgenerator-win-x64.zip",
      "size": 12345678
    },
    {
      "name": "httpgenerator-linux-x64",
      "browser_download_url": "https://github.com/christianhelle/httpgenerator/releases/download/v1.1.0/httpgenerator-linux-x64",
      "size": 8765432
    },
    {
      "name": "httpgenerator-darwin-x64",
      "browser_download_url": "https://github.com/christianhelle/httpgenerator/releases/download/v1.1.0/httpgenerator-darwin-x64",
      "size": 9012345
    },
    {
      "name": "httpgenerator-darwin-arm64",
      "browser_download_url": "https://github.com/christianhelle/httpgenerator/releases/download/v1.1.0/httpgenerator-darwin-arm64",
      "size": 8901234
    }
  ]
}
```

**Artifact naming convention:**

| Platform | Architecture | Asset Name | Extension | Extract? |
|---|---|---|---|---|
| Windows | x64 | `httpgenerator-win-x64.zip` | .zip | Yes (unzip) |
| Linux | x64 | `httpgenerator-linux-x64` | none | No |
| macOS | x64 | `httpgenerator-darwin-x64` | none | No |
| macOS | ARM64 | `httpgenerator-darwin-arm64` | none | No |

### 3.3 Storage Location

The cached binary is stored in VS Code's globalStorage directory:

```
<globalStorageDir>/christianresmahelle.http-file-generator/
├── httpgenerator          # Linux/macOS binary
├── httpgenerator.exe      # Windows binary
├── cli-version.json       # Stores: {"version": "1.1.0", "downloadedAt": "2025-01-15T10:00:00Z"}
└── .cache-ready           # Marker file indicating cache is valid
```

**globalStorage path resolution:**
```typescript
const storageUri = vscode.extensions.getExtension('ChristianResmaHelle.http-file-generator')!
    .extensionUri;
const cliDir = vscode.Uri.joinPath(storageUri, 'cli');
const binaryUri = vscode.Uri.joinPath(cliDir, platformBinaryName);
```

### 3.4 Component Diagram

```
┌──────────────────────────────────────────────────────┐
│                    extension.ts                       │
│                                                       │
│  activate()                                           │
│    ├── checkCLIAvailable()  ──► resolveAndEnsureCLI() │
│    │                              ├── findExecutable() │
│    │                              ├── downloadCLI()    │
│    │                              └── verifyCLI()      │
│    │                                                   │
│    ├── registerCommand('...generateSingleHttpFile')   │
│    ├── registerCommand('...generateMultipleHttpFiles') │
│    ├── registerCommand('...resetCLI')                 │
│    └── registerCommand('...showCLIPath')              │
│                                                       │
│  executeHttpGenerator(filePath, outputType)            │
│    ├── getExecutablePath()                            │
│    └── runInTerminal(executable, args)               │
└──────────────────────────────────────────────────────┘
```

---

## 4. Implementation Plan

### 4.1 File Structure

```
src/vscode/
├── package.json                          # UPDATE: version, engines, contributes.config
├── src/
│   ├── extension.ts                      # MAIN: Replace .NET tool code with CLI management
│   ├── cli-manager.ts                    # NEW: CLI download, caching, version management
│   ├── cli-executor.ts                   # NEW: Terminal execution of CLI
│   ├── progress.ts                       # NEW: Progress notification helpers
│   └── test/
│       ├── runTest.ts                    # (unchanged)
│       └── suite/
│           ├── extension.test.ts         # UPDATE: Update tests for new CLI flow
│           └── index.ts                  # (unchanged)
├── build.ps1                             # UPDATE: Remove .NET tool bundling
├── build.sh                              # UPDATE: Remove .NET tool bundling
├── .vscodeignore                         # (unchanged)
├── tsconfig.json                         # (unchanged)
└── README.md                             # UPDATE: Remove .NET requirements, add CLI info
```

### 4.2 New File: `src/cli-manager.ts`

**Purpose:** Manage CLI binary lifecycle (resolve, download, cache, verify).

**Key functions:**

```typescript
// Resolves the CLI executable path following the search chain:
// setting → PATH → globalStorage cache → GitHub download
export async function resolveCLIPath(
    context: vscode.ExtensionContext
): Promise<string | undefined>

// Downloads the CLI binary from GitHub Releases
export async function downloadCLI(
    context: vscode.ExtensionContext,
    version: string,
    onProgress?: (message: string) => void
): Promise<vscode.Uri>

// Verifies the cached binary exists and is executable
export function verifyCLI(path: string): Promise<boolean>

// Gets the cached CLI version (from cli-version.json)
export function getCachedCLIVersion(context: vscode.ExtensionContext): string | undefined

// Sets the cached CLI version after successful download
export function setCachedCLIVersion(context: vscode.ExtensionContext, version: string): void

// Resets (deletes) the cached CLI binary
export async function resetCLI(context: vscode.ExtensionContext): Promise<boolean>

// Gets the platform-specific artifact name for GitHub Releases
export function getArtifactName(version: string): string

// Gets the platform architecture identifier
export function getPlatformArch(): 'win-x64' | 'linux-x64' | 'darwin-x64' | 'darwin-arm64'
```

### 4.3 New File: `src/cli-executor.ts`

**Purpose:** Execute the CLI binary in a VS Code terminal.

**Key functions:**

```typescript
// Executes the CLI in a named VS Code terminal
export function executeInTerminal(
    executablePath: string,
    args: string[],
    terminalName?: string
): void

// Creates or reuses the "HTTP File Generator" terminal
export function getOrCreateTerminal(name?: string): vscode.Terminal
```

### 4.4 New File: `src/progress.ts`

**Purpose:** VS Code progress notification helpers.

**Key functions:**

```typescript
// Shows a progress notification that can be updated
export function showProgress(
    title: string,
    step: (progress: vscode.Progress<{ message?: string; increment?: number }>) => Promise<void>
): Promise<void>
```

### 4.5 Modified File: `src/extension.ts`

**Changes:**

1. **Remove all .NET tool code:**
   - Delete `getInstalledHttpGeneratorVersion()`
   - Delete `installHttpGenerator()`
   - Delete all `dotnet tool` related logic

2. **Add CLI manager initialization:**
   ```typescript
   export function activate(context: vscode.ExtensionContext) {
       // Ensure CLI is available (downloads if needed)
       const cliPath = await resolveAndEnsureCLI(context);
       
       if (cliPath) {
           // Show status bar indicator
           showCLIStatusBar(cliPath);
       }
       
       // Register commands (same IDs as before)
       context.subscriptions.push(
           vscode.commands.registerCommand('http-file-generator.generateSingleHttpFile', ...),
           vscode.commands.registerCommand('http-file-generator.generateMultipleHttpFiles', ...),
           vscode.commands.registerCommand('http-file-generator.generateSingleHttpFileMenu', ...),
           vscode.commands.registerCommand('http-file-generator.generateMultipleHttpFilesMenu', ...),
           vscode.commands.registerCommand('http-file-generator.resetCLI', () => resetCLI(context)),
           vscode.commands.registerCommand('http-file-generator.showCLIPath', () => showCLIPath(context)),
       );
   }
   ```

3. **Modify `executeHttpGenerator()`:**
   ```typescript
   async function executeHttpGenerator(filePath: string, outputType: string) {
       const cliPath = await resolveAndEnsureCLI(context);
       
       if (!cliPath) {
           vscode.window.showErrorMessage(
               'Unable to locate or download the httpgenerator CLI. ' +
               'Try "HTTP File Generator: Reset CLI" or set "http-file-generator.executablePath".'
           );
           return;
       }
       
       const outputFolder = await vscode.window.showInputBox({
           prompt: 'Select output folder',
           value: path.join(path.dirname(filePath), 'HttpFiles'),
       });
       
       if (!outputFolder) return;
       
       const command = `"${cliPath}" "${filePath}" --output "${outputFolder}" --output-type ${outputType}`;
       executeInTerminal(cliPath, [filePath, '--output', outputFolder, '--output-type', outputType]);
   }
   ```

### 4.6 Modified File: `package.json`

**Changes:**

1. **Update `engines`:**
   ```json
   "engines": {
       "vscode": "^1.85.0"
   }
   ```

2. **Add `contributes.configuration`:**
   ```json
   "configuration": {
       "title": "HTTP File Generator",
       "properties": {
           "http-file-generator.executablePath": {
               "type": "string",
               "default": "",
               "markdownDescription": "Path to the httpgenerator CLI executable. Leave empty to auto-download."
           }
       }
   }
   ```

3. **Add new commands to `contributes.commands`:**
   ```json
   {
       "command": "http-file-generator.resetCLI",
       "title": "HTTP File Generator: Reset CLI"
   },
   {
       "command": "http-file-generator.showCLIPath",
       "title": "HTTP File Generator: Show CLI Path"
   }
   ```

4. **Add commands to `contributes.menus.commandPalette`:**
   ```json
   "commandPalette": [
       {
           "command": "http-file-generator.resetCLI",
           "when": "false"
       },
       {
           "command": "http-file-generator.showCLIPath",
           "when": "false"
       }
   ]
   ```

### 4.7 Build Pipeline Changes

**`build.ps1` and `build.sh`:**
- Remove any .NET tool build/bundling steps
- No Rust build steps needed (download-only approach)
- Keep: `npm install`, `npm run compile`, `npm run package`

**GitHub workflow (`vscode.yml`):**
- No changes needed for Rust build (no longer bundling)
- Keep existing VS Code extension publish steps

---

## 5. Migration Notes

### 5.1 Breaking Changes

| Change | Impact | Mitigation |
|---|---|---|
| .NET tool dependency removed | Users without .NET SDK won't need it anymore | Positive: removes dependency |
| `dotnet tool install` removed | No more auto-install of .NET tool | Users see download progress instead |
| Minimum VS Code version bumped to 1.85.0 | Users on older VS Code can't update | Rare cases; VS Code auto-updates |

### 5.2 User Migration Path

1. User updates extension
2. On next command use, extension auto-downloads Rust CLI
3. If download fails, user gets error with retry option
4. User can set `http-file-generator.executablePath` to point to existing CLI
5. User can run "HTTP File Generator: Reset CLI" to clear cache

---

## 6. Testing Strategy

### 6.1 Unit Test Scenarios

| # | Scenario | Expected Result |
|---|---|---|
| UT-1 | CLI found via `executablePath` setting | Use configured path, no download |
| UT-2 | CLI found on PATH | Use PATH binary, no download |
| UT-3 | CLI not found, download succeeds | Binary cached, used for generation |
| UT-4 | CLI not found, download fails | Error shown, retry offered |
| UT-5 | Cached CLI version matches extension | No re-download |
| UT-6 | Cached CLI version differs from extension | Re-download triggered |
| UT-7 | Cached CLI is corrupted/missing | Re-download triggered silently |
| UT-8 | "Reset CLI" command executed | Cached binary deleted, next command re-downloads |
| UT-9 | "Show CLI Path" command executed | Shows path in notification |
| UT-10 | Extension activates with CLI already available | No download, status bar shows version |

### 6.2 Integration Test Scenarios

| # | Scenario | Expected Result |
|---|---|---|
| IT-1 | Generate single file from .json OpenAPI spec | .http file created in output folder |
| IT-2 | Generate multiple files from .yaml OpenAPI spec | Multiple .http files created |
| IT-3 | Context menu on .yml file | Command executes correctly |
| IT-4 | Command Palette execution (no file URI) | File picker shown, works correctly |
| IT-5 | Custom `--base-url` passed to CLI | CLI receives correct argument |
| IT-6 | Custom `--content-type` passed to CLI | CLI receives correct argument |
| IT-7 | Output folder selection (default) | Default path suggested, works |
| IT-8 | Output folder selection (custom) | Custom path accepted, works |

### 6.3 Platform Test Matrix

| Platform | Architecture | Test |
|---|---|---|
| Windows 10/11 | x64 | Download + execute + generate |
| Ubuntu 22.04 | x64 | Download + execute + generate |
| macOS (Intel) | x64 | Download + execute + generate |
| macOS (Apple Silicon) | ARM64 | Download + execute + generate |

---

## 7. Open Questions

| # | Question | Status |
|---|---|---|
| Q1 | What happens if GitHub is unreachable (firewall, offline)? | Extension should show clear error message suggesting manual download from GitHub Releases |
| Q2 | Should the extension verify binary integrity (checksum)? | Out of scope for v1. Consider adding in future |
| Q3 | Should there be a timeout for CLI execution? | Yes, use existing `--timeout` flag (default 120s) |
| Q4 | What about ARM64 Windows? | Currently uses `win-x64` binary (x64 emulation on ARM64 Windows) |
| Q5 | Should the extension validate CLI version compatibility? | Yes, compare extension version with cached version. Mismatch triggers download. |

---

## 8. Acceptance Criteria

- [ ] Extension no longer depends on .NET SDK or .NET tool
- [ ] Extension downloads Rust CLI on first use (when not on PATH)
- [ ] Extension reuses cached CLI across sessions
- [ ] Extension checks for CLI updates on activation
- [ ] Extension respects `executablePath` setting
- [ ] Extension shows progress during download
- [ ] Extension shows error + retry on download failure
- [ ] "Reset CLI" command works correctly
- [ ] "Show CLI Path" command shows correct path
- [ ] All existing commands work identically (same IDs, same behavior)
- [ ] Extension works on all 4 platforms (win-x64, linux-x64, darwin-x64, darwin-arm64)
- [ ] Build pipeline produces working VSIX without Rust toolchain
- [ ] README updated to reflect new architecture
- [ ] All existing tests pass

---

## 9. Appendix

### 9.1 Current Extension Commands (preserved)

| Command ID | Title | Description |
|---|---|---|
| `http-file-generator.generateSingleHttpFile` | HTTP File Generator: Generate single HTTP file | Generate one .http file from OpenAPI spec |
| `http-file-generator.generateMultipleHttpFiles` | HTTP File Generator: Generate multiple HTTP files | Generate one .http file per request |
| `http-file-generator.generateSingleHttpFileMenu` | Generate single HTTP file | Context menu variant |
| `http-file-generator.generateMultipleHttpFilesMenu` | Generate multiple HTTP files | Context menu variant |

### 9.2 New Extension Commands (added)

| Command ID | Title | Description |
|---|---|---|
| `http-file-generator.resetCLI` | HTTP File Generator: Reset CLI | Delete cached CLI binary, force re-download |
| `http-file-generator.showCLIPath` | HTTP File Generator: Show CLI Path | Display the detected/used CLI executable path |

### 9.3 CLI Arguments Mapping

| VS Code Input | CLI Argument | Example |
|---|---|---|
| OpenAPI file path | positional arg | `httpgenerator ./openapi.json` |
| Output folder | `--output` | `--output ./HttpFiles` |
| Output type `OneFile` | `--output-type OneFile` | `--output-type OneFile` |
| Output type `OneRequestPerFile` | `--output-type OneRequestPerFile` | `--output-type OneRequestPerFile` |

### 9.4 Reference: VSIX Binary Resolution (for pattern)

The VSIX extension resolves the CLI using this chain (reference for consistent approach):
1. `HTTPGENERATOR_PATH` environment variable
2. Bundled VSIX payload (`httpgenerator.exe` alongside `.vsix`)
3. Workspace root `target/debug` / `target/release`
4. `PATH`

The VS Code extension follows a similar but adapted chain:
1. `http-file-generator.executablePath` setting
2. `PATH`
3. Downloaded cache (globalStorage)
4. `PATH` (post-download, the cached binary is on a known path)
