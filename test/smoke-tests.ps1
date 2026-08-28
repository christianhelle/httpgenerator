param (
    [Parameter(Mandatory=$false)]
    [switch]
    $Production = $false,

    [Parameter(Mandatory=$false)]
    [switch]
    $Benchmark = $false
)

function ThrowOnNativeFailure {
    if (-not $?)
    {
        throw "Native Failure"
    }
}

function Get-HttpGeneratorExecutableName {
    if ($IsWindows) {
        return "httpgenerator.exe"
    }

    return "httpgenerator"
}

function Get-LocalHttpGeneratorPath {
    $executableName = Get-HttpGeneratorExecutableName
    return Join-Path (Join-Path $PSScriptRoot "bin") $executableName
}

function PrepareLocalRustCli {
    $executableName = Get-HttpGeneratorExecutableName
    $binDirectory = Join-Path $PSScriptRoot "bin"
    $sourcePath = [System.IO.Path]::GetFullPath(
        [System.IO.Path]::Combine($PSScriptRoot, "..", "target", "release", $executableName))

    if (-not (Test-Path $binDirectory)) {
        New-Item -ItemType Directory -Path $binDirectory | Out-Null
    }

    Write-Output "cargo build --release -p httpgenerator"
    $process = Start-Process "cargo" -Args "build --release -p httpgenerator" -NoNewWindow -PassThru
    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "cargo build failed"
    }

    Copy-Item $sourcePath (Get-LocalHttpGeneratorPath) -Force
}

function Get-DotNetExecutableName {
    if ($IsWindows) {
        return "httpgenerator.exe"
    }

    return "httpgenerator"
}

function Get-LocalDotNetGeneratorPath {
    $executableName = Get-DotNetExecutableName
    return [System.IO.Path]::GetFullPath(
        [System.IO.Path]::Combine($PSScriptRoot, "bin", "dotnet", $executableName))
}

function PrepareLocalDotNetCli {
    $executableName = Get-DotNetExecutableName
    $solutionPath = [System.IO.Path]::GetFullPath(
        [System.IO.Path]::Combine($PSScriptRoot, "..", "src", "dotnet", "HttpGenerator.slnx"))

    Write-Host "dotnet build --configuration Release `"$solutionPath`""
    $process = Start-Process "dotnet" -Args "build --configuration Release `"$solutionPath`"" -NoNewWindow -PassThru
    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "dotnet build failed"
    }

    $sourceDir = [System.IO.Path]::GetFullPath(
        [System.IO.Path]::Combine($PSScriptRoot, "..", "src", "dotnet", "HttpGenerator", "bin", "Release", "net10.0"))
    $dotnetBinDir = [System.IO.Path]::GetFullPath(
        [System.IO.Path]::Combine($PSScriptRoot, "bin", "dotnet"))

    if (Test-Path $dotnetBinDir) {
        Remove-Item -Path $dotnetBinDir -Recurse -Force
    }

    Copy-Item -Path $sourceDir -Destination $dotnetBinDir -Recurse

    if (!(Test-Path (Get-LocalDotNetGeneratorPath))) {
        throw "dotnet executable not found at $(Get-LocalDotNetGeneratorPath)"
    }
}

function Invoke-CliCapture {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $app,

        [Parameter(Mandatory=$true)]
        [string[]]
        $arguments
    )

    $captured = @(& $app @arguments 2>&1 | ForEach-Object { "$_" })
    $exitCode = $LASTEXITCODE

    return @{
        ExitCode = $exitCode
        Output = [string]::Join([Environment]::NewLine, $captured)
    }
}

function Invoke-CliCaptureStreams {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $app,

        [Parameter(Mandatory=$true)]
        [string[]]
        $arguments
    )

    $captureRoot = Join-Path (Join-Path $PSScriptRoot "Generated") "cli-capture"
    $stdoutPath = Join-Path $captureRoot "stdout.txt"
    $stderrPath = Join-Path $captureRoot "stderr.txt"

    try {
        if (Test-Path $captureRoot) {
            Remove-Item -Path $captureRoot -Recurse -Force
        }

        New-Item -ItemType Directory -Path $captureRoot | Out-Null

        $process = Start-Process $app `
            -ArgumentList $arguments `
            -NoNewWindow `
            -PassThru `
            -RedirectStandardOutput $stdoutPath `
            -RedirectStandardError $stderrPath

        $process | Wait-Process

        return @{
            ExitCode = $process.ExitCode
            StdOut = if (Test-Path $stdoutPath) { Get-Content -Path $stdoutPath -Raw } else { "" }
            StdErr = if (Test-Path $stderrPath) { Get-Content -Path $stderrPath -Raw } else { "" }
        }
    }
    finally {
        if (Test-Path $captureRoot) {
            Remove-Item -Path $captureRoot -Recurse -Force
        }
    }
}

function Assert-PlainRedirectedOutput {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $Output,

        [Parameter(Mandatory=$true)]
        [string]
        $Context
    )

    if ($Output -match "`e") {
        throw "$Context should not contain ANSI escape sequences"
    }

    $richMarkers = @(
        [char]0x250C,
        [char]0x2510,
        [char]0x2514,
        [char]0x2518,
        [char]0x251C,
        [char]0x2524,
        [char]0x2502,
        [char]0x2500,
        [System.Char]::ConvertFromUtf32(0x1F680),
        [System.Char]::ConvertFromUtf32(0x1F50D),
        [char]0x2705,
        [System.Char]::ConvertFromUtf32(0x1F4CA),
        [System.Char]::ConvertFromUtf32(0x1F4C1),
        [System.Char]::ConvertFromUtf32(0x1F389),
        [char]0x23F1,
        [System.Char]::ConvertFromUtf32(0x1F511),
        [char]0x26A0,
        [char]0x274C
    )
    foreach ($marker in $richMarkers) {
        if ($Output.Contains($marker)) {
            throw "$Context should stay plain when redirected; found rich marker '$marker'"
        }
    }
}

function ValidateCliOutputStructure {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $app
    )

    $structureOutput = Join-Path (Join-Path $PSScriptRoot "Generated") "cli-output-structure"
    $petstorePath = Join-Path (Join-Path (Join-Path $PSScriptRoot "OpenAPI") "v3.0") "petstore.json"

    try {
        if (Test-Path $structureOutput) {
            Remove-Item -Path $structureOutput -Recurse -Force
        }

        $help = Invoke-CliCapture -app $app -arguments @("--help")
        if ($help.ExitCode -ne 0) {
            throw "httpgenerator --help failed"
        }

        Assert-PlainRedirectedOutput -Output $help.Output -Context "Help output"
        foreach ($expected in @(
            "Usage: httpgenerator [URL or input file] [OPTIONS]",
            "Examples:",
            "--output-type <OUTPUT-TYPE>"
        )) {
            if (-not $help.Output.Contains($expected)) {
                throw "Help output is missing expected text: $expected"
            }
        }

        if ($help.Output.Contains("httpgenerator-cli")) {
            throw "Help output should use the public command identity"
        }

        $generation = Invoke-CliCapture -app $app -arguments @(
            $petstorePath,
            "--output",
            $structureOutput,
            "--no-logging"
        )

        if ($generation.ExitCode -ne 0) {
            throw "Redirected petstore generation failed"
        }

        Assert-PlainRedirectedOutput -Output $generation.Output -Context "Generation output"
        foreach ($expected in @(
            "HTTP File Generator v",
            "Support key: Unavailable when logging is disabled",
            "Validating OpenAPI specification...",
            "Validated OpenAPI 3.0.x specification successfully",
            "Path Items: 13",
            "Operations: 19",
            "Writing 19 file(s)...",
            "Files written successfully:",
            "Generation completed successfully!",
            "Duration: "
        )) {
            if (-not $generation.Output.Contains($expected)) {
                throw "Generation output is missing expected text: $expected"
            }
        }

        $writtenFileLineCount = @(
            $generation.Output -split "\r?\n" |
                Where-Object { $_.TrimEnd().EndsWith(".http") }
        ).Count
        if ($writtenFileLineCount -ne 19) {
            throw "Expected redirected generation output to list 19 written files, found $writtenFileLineCount"
        }
    }
    finally {
        if (Test-Path $structureOutput) {
            Remove-Item -Path $structureOutput -Recurse -Force
        }
    }
}

function ValidateCliWarningStreamCapture {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $app
    )

    $warningOutput = Join-Path (Join-Path $PSScriptRoot "Generated") "cli-warning-streams"
    $petstorePath = Join-Path (Join-Path (Join-Path $PSScriptRoot "OpenAPI") "v3.0") "petstore.json"
    $expectedWarning = "Azure Entra ID scope is required to acquire an authorization header."

    try {
        if (Test-Path $warningOutput) {
            Remove-Item -Path $warningOutput -Recurse -Force
        }

        $capture = Invoke-CliCaptureStreams -app $app -arguments @(
            $petstorePath,
            "--output",
            $warningOutput,
            "--no-logging",
            "--azure-tenant-id",
            "tenant-id"
        )

        if ($capture.ExitCode -ne 0) {
            throw "Generation with Azure warning capture failed"
        }

        Assert-PlainRedirectedOutput -Output $capture.StdOut -Context "Azure warning stdout"
        Assert-PlainRedirectedOutput -Output $capture.StdErr -Context "Azure warning stderr"

        if ($capture.StdOut.Contains($expectedWarning)) {
            throw "Azure warning should stay on stderr so redirected hosts can surface it separately"
        }

        foreach ($expected in @(
            "HTTP File Generator v",
            "Generation completed successfully!",
            "Writing 19 file(s)..."
        )) {
            if (-not $capture.StdOut.Contains($expected)) {
                throw "Azure warning stdout is missing expected text: $expected"
            }
        }

        if (-not $capture.StdErr.Contains("Error: $expectedWarning")) {
            throw "Azure warning stderr is missing the expected message"
        }
    }
    finally {
        if (Test-Path $warningOutput) {
            Remove-Item -Path $warningOutput -Recurse -Force
        }
    }
}

function Generate {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $app,

        [Parameter(Mandatory=$true)]
        [string]
        $format,

        [Parameter(Mandatory=$true)]
        [string]
        $output,

        [Parameter(Mandatory=$false)]
        [string]
        $args = ""
    )

    Write-Output "$app ./openapi.$format --output ./Generated/$output --no-logging $args"
    $process = Start-Process $app `
        -Args "./openapi.$format --output ./Generated/$output --no-logging $args" `
        -NoNewWindow `
        -PassThru

    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "HttpGenerator failed"
    }

    Write-Output "$app ./openapi.$format --output ./Generated/$output --output-type OneFile --no-logging $args"
    $process = Start-Process $app `
        -Args "./openapi.$format --output ./Generated/$output --output-type OneFile --no-logging $args" `
        -NoNewWindow `
        -PassThru

    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "HttpGenerator failed"
    }

    Write-Output "$app ./openapi.$format --output ./Generated/$output --output-type OneFilePerTag --no-logging $args"
    $process = Start-Process $app `
        -Args "./openapi.$format --output ./Generated/$output --output-type OneFilePerTag --no-logging $args" `
        -NoNewWindow `
        -PassThru

    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "HttpGenerator failed"
    }
}

function GenerateWithSpecificArgs {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $app,

        [Parameter(Mandatory=$true)]
        [string]
        $format,

        [Parameter(Mandatory=$true)]
        [string]
        $output,

        [Parameter(Mandatory=$true)]
        [string]
        $outputType,

        [Parameter(Mandatory=$false)]
        [string]
        $args = ""
    )

    Write-Output "$app ./openapi.$format --output ./Generated/$output --output-type $outputType --no-logging $args"
    $process = Start-Process $app `
        -Args "./openapi.$format --output ./Generated/$output --output-type $outputType --no-logging $args" `
        -NoNewWindow `
        -PassThru

    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "HttpGenerator failed with args: $args"
    }
}

function Get-TestApplication {
    param (
        [Parameter(Mandatory=$true)]
        [ValidateSet("RustCli", "HttpGenerator")]
        [string]
        $Method,

        [Parameter(Mandatory=$false)]
        [bool]
        $Production = $false
    )

    if ($Production -eq $true) {
        if (-not (Get-Command "httpgenerator" -ErrorAction SilentlyContinue)) {
            throw "httpgenerator was not found on PATH"
        }

        return "httpgenerator"
    }

    if ($Method -eq "RustCli") {
        $null = PrepareLocalRustCli
        return Get-LocalHttpGeneratorPath
    }

    $null = PrepareLocalDotNetCli
    return Get-LocalDotNetGeneratorPath
}

function RunTests {
    param (
        [Parameter(Mandatory=$true)]
        [ValidateSet("RustCli", "HttpGenerator")]
        [string]
        $Method,
        
        [Parameter(Mandatory=$false)]
        [bool]
        $Production = $false,

        [Parameter(Mandatory=$false)]
        [bool]
        $SkipValidation = $false,

        [Parameter(Mandatory=$false)]
        [string]
        $App = ""
    )

    $filenames = @(
        "petstore",
        "petstore-expanded",
        "petstore-minimal",
        "petstore-simple",
        "petstore-with-external-docs",
        "api-with-examples",
        "callback-example",
        "link-example",
        "uber",
        "uspto",
        "hubspot-events",
        "hubspot-webhooks",
        "non-oauth-scopes",
        "webhook-example"
    )

    Get-ChildItem '*.http' -Recurse | ForEach-Object { Remove-Item -Path $_.FullName }

    if ([string]::IsNullOrEmpty($App)) {
        $App = Get-TestApplication -Method $Method -Production $Production
    }

    if (-not $SkipValidation) {
        ValidateCliOutputStructure -app $App
        ValidateCliWarningStreamCapture -app $App
    }

    "v2.0", "v3.0", "v3.1" | ForEach-Object {
        $version = $_
        "json", "yaml" | ForEach-Object { 
            $format = $_
            $filenames | ForEach-Object {
                Invoke-TestGeneration -app $App -version $version -format $format -name $_
            }
        }
    }
}

function Invoke-TestGeneration {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $app,

        [Parameter(Mandatory=$true)]
        [string]
        $version,

        [Parameter(Mandatory=$true)]
        [string]
        $format,

        [Parameter(Mandatory=$true)]
        [string]
        $name

    )

    $filename = "./OpenAPI/$version/$name.$format"
    if (-not (Test-Path -Path $filename -PathType Leaf)) {
        return
    }

    Write-Output "Testing $filename"
    Copy-Item $filename ./openapi.$format
    if ($version -eq "v3.1") {
        Generate -app $app -format $format -output "$name/$version/$format" -args "--skip-validation --generate-intellij-tests --custom-header ""X-Custom-Header: 1234"" --base-url https://api.example.io/"
        return
    }

    Generate -app $app -format $format -output "$name/$version/$format" -args "--generate-intellij-tests --custom-header ""X-Custom-Header: 1234"" --base-url https://api.example.io/"

    # Additional parameter combination tests for v2.0 and v3.0
    if ($name -eq "petstore") {
        Invoke-ExtraParameterTests -app $app -name $name -version $version -format $format
    }
}

function Invoke-ExtraParameterTests {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $app,

        [Parameter(Mandatory=$true)]
        [string]
        $name,

        [Parameter(Mandatory=$true)]
        [string]
        $version,

        [Parameter(Mandatory=$true)]
        [string]
        $format

    )

    $filename = "./OpenAPI/$version/$name.$format"
    Write-Output "Testing $filename with --authorization-header"
    GenerateWithSpecificArgs -app $app -format $format -output "$name/$version/$format/auth-header" -outputType "OneFile" -args "--authorization-header ""Bearer test token"""

    Write-Output "Testing $filename with --load-authorization-header-from-environment"
    GenerateWithSpecificArgs -app $app -format $format -output "$name/$version/$format/auth-env" -outputType "OneFile" -args "--load-authorization-header-from-environment --authorization-header-variable-name ""my_token"""

    Write-Output "Testing $filename with --skip-headers"
    GenerateWithSpecificArgs -app $app -format $format -output "$name/$version/$format/skip-headers" -outputType "OneFile" -args "--skip-headers"

    Write-Output "Testing $filename with --content-type application/xml"
    GenerateWithSpecificArgs -app $app -format $format -output "$name/$version/$format/xml" -outputType "OneFile" -args "--content-type ""application/xml"""

    Write-Output "Testing $filename with environment variable base URL"
    GenerateWithSpecificArgs -app $app -format $format -output "$name/$version/$format/env-baseurl" -outputType "OneFile" -args "--base-url ""{{MY_BASE_URL}}"""
}

Push-Location $PSScriptRoot
try {
    if ($Benchmark) {
        Write-Host "=== Benchmark Mode: Testing both Rust and .NET CLIs ==="
        Write-Host ""

        $rustApp = Get-TestApplication -Method "RustCli" -Production $false
        $dotnetApp = Get-TestApplication -Method "HttpGenerator" -Production $false

        # Warm-up: populate caches, then discard timing
        Write-Host ">>> Warm-up run (Rust)..." 
        RunTests -Method "RustCli" -SkipValidation $true -App $rustApp
        Write-Host ">>> Warm-up run (.NET)..."
        RunTests -Method "HttpGenerator" -SkipValidation $true -App $dotnetApp
        Write-Host ""

        # Timed runs
        Write-Host ">>> Benchmarking Rust CLI..."
        $rustTime = Measure-Command {
            RunTests -Method "RustCli" -SkipValidation $true -App $rustApp
        }
        Write-Host ""

        Write-Host ">>> Benchmarking .NET CLI..."
        $dotnetTime = Measure-Command {
            RunTests -Method "HttpGenerator" -SkipValidation $true -App $dotnetApp
        }
        Write-Host ""

        $rustSec = $rustTime.TotalSeconds
        $dotnetSec = $dotnetTime.TotalSeconds
        $comparison = if ($rustSec -eq $dotnetSec) {
            "Rust and .NET completed in the same time"
        } elseif ($rustSec -lt $dotnetSec) {
            if ($rustSec -gt 0) {
                "Rust is {0:F2}x faster than .NET" -f ($dotnetSec / $rustSec)
            } else {
                "Rust completed faster than .NET"
            }
        } elseif ($dotnetSec -gt 0) {
            ".NET is {0:F2}x faster than Rust" -f ($rustSec / $dotnetSec)
        } else {
            ".NET completed faster than Rust"
        }

        Write-Host "=================================="
        Write-Host "   Performance Comparison Report"
        Write-Host "=================================="
        Write-Host ""
        Write-Host ("{0,-20} {1,15}" -f "Platform", "Duration (sec)")
        Write-Host ("{0,-20} {1,15}" -f "--------", "--------------")
        Write-Host ("{0,-20} {1,15:F3}" -f "Rust CLI", $rustSec)
        Write-Host ("{0,-20} {1,15:F3}" -f ".NET CLI", $dotnetSec)
        Write-Host ""
        Write-Host $comparison
        Write-Host ""
    } else {
        Measure-Command { RunTests -Method "RustCli" -Production $Production }
        Write-Host "`r`n"
    }
}
finally {
    Pop-Location
}
