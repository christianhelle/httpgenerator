param (
    [string]$Version = "0.1.0",
    [string]$Target = "",           # vsce platform target, e.g. win32-x64, linux-x64, darwin-x64, darwin-arm64
    [string]$RustBinaryPath = ""    # path to a pre-built httpgenerator.exe to bundle
)

Set-Location -Path $PSScriptRoot

# Stage Rust binary if a path was provided
if ($RustBinaryPath) {
    $BinDir = Join-Path $PSScriptRoot "bin"
    if (Test-Path $BinDir) { Remove-Item -Recurse -Force $BinDir }
    New-Item -ItemType Directory -Path $BinDir | Out-Null
    Copy-Item -Path $RustBinaryPath -Destination (Join-Path $BinDir "httpgenerator.exe")
    Write-Host "Staged Rust binary to $BinDir\httpgenerator.exe"
}

# Install dependencies
npm install

# Compile the extension
npm run compile

# Package the extension
if ($Target) {
    npx vsce package --target $Target
    Write-Host "HTTP File Generator for VS Code extension has been built to http-file-generator-$Version-$Target.vsix"
} else {
    npx vsce package
    Write-Host "HTTP File Generator for VS Code extension has been built to http-file-generator-$Version.vsix"
}
