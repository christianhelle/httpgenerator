param (
    [string]$Version = "0.1.0"
)

Set-Location -Path $PSScriptRoot

function Get-VsCodeTarget {
    if ($IsWindows) {
        return "win32-x64"
    }

    if ($IsMacOS) {
        return "darwin-x64"
    }

    return "linux-x64"
}

$vsCodeTarget = Get-VsCodeTarget
$rustBinary = if ($IsWindows) { "httpgenerator.exe" } else { "httpgenerator" }
$rustBinaryPath = Join-Path $PSScriptRoot "..\..\target\release\$rustBinary"
$bundleDirectory = Join-Path $PSScriptRoot "bin"
$bundlePath = Join-Path $bundleDirectory $rustBinary

# Build and bundle the Rust CLI
cargo build --release -p httpgenerator-cli
if (!(Test-Path $bundleDirectory)) {
    New-Item -ItemType Directory -Path $bundleDirectory | Out-Null
}
Copy-Item $rustBinaryPath $bundlePath -Force

# Install dependencies
npm ci

# Compile the extension
npm run compile

# Package the extension
npx vsce package --target $vsCodeTarget

Write-Host "HTTP File Generator for VS Code extension has been built to http-file-generator-$Version-$vsCodeTarget.vsix"
