param (
    [string]$Version = "",
    [string]$Target = ""
)

$ErrorActionPreference = "Stop"

$extensionRoot = $PSScriptRoot

function Assert-Success([string]$Step) {
    if ($LASTEXITCODE -ne 0) {
        throw "$Step failed with exit code $LASTEXITCODE."
    }
}

if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = (Get-Content (Join-Path $extensionRoot "package.json") -Raw | ConvertFrom-Json).version
}

if ([string]::IsNullOrWhiteSpace($Target)) {
    $architecture = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString().ToLowerInvariant()
    $Target = switch ($architecture) {
        "arm64" { "win32-arm64" }
        "x64" { "win32-x64" }
        "x86" { "win32-ia32" }
        default { throw "Unsupported Windows architecture '$architecture'." }
    }
}

$vsixPath = Join-Path $extensionRoot "http-file-generator-$Version-$Target.vsix"

if (Test-Path $vsixPath) {
    Remove-Item $vsixPath -Force
}

Set-Location -Path $extensionRoot
npm ci
Assert-Success "npm ci"
npm run compile
Assert-Success "npm run compile"
npx vsce package --target $Target --out $vsixPath
Assert-Success "vsce package"

Write-Host "HTTP File Generator for VS Code extension has been built to $vsixPath"
