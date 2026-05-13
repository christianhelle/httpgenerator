param (
    [string]$Version = "0.1.0",
    [string]$Target
)

$ErrorActionPreference = "Stop"
Set-Location -Path $PSScriptRoot

if (-not $Target) {
    if ($IsWindows) {
        $Target = "win32-x64"
    }
    elseif ($IsMacOS) {
        if ([System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture -eq [System.Runtime.InteropServices.Architecture]::Arm64) {
            $Target = "darwin-arm64"
        }
        else {
            $Target = "darwin-x64"
        }
    }
    else {
        $Target = "linux-x64"
    }
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "../..")).Path
$exeName = if ($Target -like "win32-*") { "httpgenerator.exe" } else { "httpgenerator" }
$sourceBinary = Join-Path $repoRoot (Join-Path "target/release" $exeName)
$bundleDir = Join-Path $PSScriptRoot (Join-Path "bin" $Target)
$bundleBinary = Join-Path $bundleDir $exeName
$outputVsix = "http-file-generator-$Version-$Target.vsix"

Push-Location $repoRoot
cargo build --release -p httpgenerator
Pop-Location

New-Item -ItemType Directory -Path $bundleDir -Force | Out-Null
Copy-Item -Path $sourceBinary -Destination $bundleBinary -Force

if ((-not $IsWindows) -and (-not ($Target -like "win32-*"))) {
    chmod +x $bundleBinary
}

npm ci --ignore-scripts
npm run compile
npx @vscode/vsce package --target $Target --out $outputVsix

Write-Host "HTTP File Generator for VS Code extension has been built to $outputVsix"
