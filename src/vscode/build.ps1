param (
    [string]$Version = "",
    [string]$Target = ""
)

$ErrorActionPreference = "Stop"

$extensionRoot = $PSScriptRoot
$repoRoot = (Resolve-Path (Join-Path $extensionRoot "..\..")).Path

function Assert-Success([string]$Step) {
    if ($LASTEXITCODE -ne 0) {
        throw "$Step failed with exit code $LASTEXITCODE."
    }
}

function Resolve-RustTarget([string]$VsCodeTarget) {
    switch ($VsCodeTarget) {
        "win32-x64" { return "x86_64-pc-windows-msvc" }
        "win32-arm64" { return "aarch64-pc-windows-msvc" }
        "win32-ia32" { return "i686-pc-windows-msvc" }
        "linux-x64" { return "x86_64-unknown-linux-gnu" }
        "linux-arm64" { return "aarch64-unknown-linux-gnu" }
        "linux-armhf" { return "armv7-unknown-linux-gnueabihf" }
        "darwin-x64" { return "x86_64-apple-darwin" }
        "darwin-arm64" { return "aarch64-apple-darwin" }
        default { throw "Unsupported VS Code target '$VsCodeTarget'." }
    }
}

function Resolve-CompilerArchitecture([string]$VsCodeTarget) {
    switch ($VsCodeTarget) {
        "win32-x64" { return "x64" }
        "win32-arm64" { return "arm64" }
        "win32-ia32" { return "x86" }
        default { throw "Unsupported Windows target '$VsCodeTarget'." }
    }
}

function Get-VisualStudioInstallationPaths() {
    $paths = [System.Collections.Generic.List[string]]::new()
    $vswherePath = Join-Path ${env:ProgramFiles(x86)} "Microsoft Visual Studio\Installer\vswhere.exe"

    if (Test-Path $vswherePath -PathType Leaf) {
        $discoveredPaths = & $vswherePath -all -products * -property installationPath 2>$null
        foreach ($discoveredPath in $discoveredPaths) {
            if (-not [string]::IsNullOrWhiteSpace($discoveredPath) -and (Test-Path $discoveredPath -PathType Container)) {
                $paths.Add((Resolve-Path $discoveredPath).Path)
            }
        }
    }

    foreach ($root in @("C:\Program Files\Microsoft Visual Studio", "C:\Program Files (x86)\Microsoft Visual Studio")) {
        if (-not (Test-Path $root -PathType Container)) {
            continue
        }

        Get-ChildItem $root -Directory -ErrorAction SilentlyContinue | ForEach-Object {
            Get-ChildItem $_.FullName -Directory -ErrorAction SilentlyContinue | ForEach-Object {
                $paths.Add($_.FullName)
            }
        }
    }

    return $paths | Where-Object { $_ } | Sort-Object -Unique
}

function Resolve-DeveloperShell([string]$CompilerArchitecture) {
    foreach ($installationPath in Get-VisualStudioInstallationPaths) {
        $devShellPath = Join-Path $installationPath "Common7\Tools\Launch-VsDevShell.ps1"
        $msvcRoot = Join-Path $installationPath "VC\Tools\MSVC"

        if (-not (Test-Path $devShellPath -PathType Leaf) -or -not (Test-Path $msvcRoot -PathType Container)) {
            continue
        }

        $matchingCompiler = Get-ChildItem $msvcRoot -Recurse -Filter cl.exe -ErrorAction SilentlyContinue |
            Where-Object { (Split-Path (Split-Path $_.FullName -Parent) -Leaf) -eq $CompilerArchitecture } |
            Sort-Object FullName -Descending |
            Select-Object -First 1

        if ($matchingCompiler) {
            return @{
                InstallationPath = $installationPath
                DevShellPath = $devShellPath
                CompilerPath = $matchingCompiler.FullName
            }
        }
    }

    throw "Unable to find a Visual Studio toolchain with a '$CompilerArchitecture' compiler. Install the matching MSVC C++ tools before packaging this target."
}

function Enter-WindowsDeveloperShell([string]$VsCodeTarget) {
    if (-not $VsCodeTarget.StartsWith("win32-")) {
        return
    }

    $compilerArchitecture = Resolve-CompilerArchitecture $VsCodeTarget
    $developerShell = Resolve-DeveloperShell $compilerArchitecture
    $hostArchitecture = if ([System.Environment]::Is64BitProcess) { "x64" } else { "x86" }

    & $developerShell.DevShellPath `
        -VsInstallationPath $developerShell.InstallationPath `
        -SkipAutomaticLocation `
        -Arch $compilerArchitecture `
        -HostArch $hostArchitecture `
        -NoLogo

    $compiler = Get-Command cl.exe -ErrorAction SilentlyContinue
    if ($null -eq $compiler) {
        throw "Visual Studio developer shell did not expose cl.exe for target '$VsCodeTarget'."
    }

    $resolvedCompilerArchitecture = Split-Path (Split-Path $compiler.Source -Parent) -Leaf
    if ($resolvedCompilerArchitecture -ne $compilerArchitecture) {
        throw "Visual Studio developer shell resolved '$($compiler.Source)' for target '$VsCodeTarget', but expected compiler architecture '$compilerArchitecture'."
    }
}

function Assert-VsixContainsBundledBinary([string]$VsixPath, [string]$Target, [string]$BinaryName) {
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $archive = [System.IO.Compression.ZipFile]::OpenRead($VsixPath)
    try {
        $expectedEntry = "extension/bin/$Target/$BinaryName"
        $entry = $archive.Entries | Where-Object { $_.FullName -eq $expectedEntry } | Select-Object -First 1
        if ($null -eq $entry) {
            throw "Expected bundled CLI entry '$expectedEntry' inside '$VsixPath'."
        }

        if ($entry.Length -le 0) {
            throw "Bundled CLI entry '$expectedEntry' inside '$VsixPath' is empty."
        }
    }
    finally {
        $archive.Dispose()
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

$binaryName = if ($Target.StartsWith("win32-")) { "httpgenerator.exe" } else { "httpgenerator" }
$rustTarget = Resolve-RustTarget $Target
$stagedBinaryDirectory = Join-Path $extensionRoot (Join-Path "bin" $Target)
$releaseBinary = Join-Path $repoRoot (Join-Path (Join-Path "target" $rustTarget) (Join-Path "release" $binaryName))
$vsixPath = Join-Path $extensionRoot "http-file-generator-$Version-$Target.vsix"

if (Test-Path (Join-Path $extensionRoot "bin")) {
    Remove-Item (Join-Path $extensionRoot "bin") -Recurse -Force
}

if (Test-Path $vsixPath) {
    Remove-Item $vsixPath -Force
}

Set-Location -Path $repoRoot
Enter-WindowsDeveloperShell $Target
rustup target add $rustTarget
Assert-Success "rustup target add"
cargo build --locked --release --target $rustTarget -p httpgenerator
Assert-Success "cargo build"

if (-not (Test-Path $releaseBinary -PathType Leaf)) {
    throw "Expected bundled CLI at '$releaseBinary' after cargo build."
}

New-Item -ItemType Directory -Path $stagedBinaryDirectory -Force | Out-Null
Copy-Item -Path $releaseBinary -Destination (Join-Path $stagedBinaryDirectory $binaryName) -Force

Set-Location -Path $extensionRoot
npm ci
Assert-Success "npm ci"
npm run compile
Assert-Success "npm run compile"
npx vsce package --target $Target --out $vsixPath
Assert-Success "vsce package"
Assert-VsixContainsBundledBinary -VsixPath $vsixPath -Target $Target -BinaryName $binaryName

Write-Host "HTTP File Generator for VS Code extension has been built to $vsixPath"
