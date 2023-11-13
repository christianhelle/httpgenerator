param (
    [Parameter(Mandatory=$false)]
    [bool]
    $Parallel = $true
)

function ThrowOnNativeFailure {
    if (-not $?)
    {
        throw "Native Failure"
    }
}

function Generate {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $format
    )
    
    Get-ChildItem '*.http' -Recurse | ForEach-Object { Remove-Item -Path $_.FullName }

    if ($args.Contains("settings-file")) {        
        Write-Host "HttpGenerator --no-logging"
        $process = Start-Process "./bin/HttpGenerator" `
            -Args "--no-logging $args" `
            -NoNewWindow `
            -PassThru
    } else {        
        Write-Host "HttpGenerator ./openapi.$format --output ./GeneratedCode/$outputPath --no-logging"
        $process = Start-Process "./bin/HttpGenerator" `
            -Args "./openapi.$format --output ./GeneratedCode --no-logging" `
            -NoNewWindow `
            -PassThru
    }

    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "HttpGenerator failed"
    }
}

function RunTests {
    param (
        [Parameter(Mandatory=$true)]
        [ValidateSet("dotnet-run", "HttpGenerator")]
        [string]
        $Method,
        
        [Parameter(Mandatory=$false)]
        [bool]
        $Parallel = $false
    )

    $filenames = @(
        "petstore",
        "petstore-expanded",
        "petstore-minimal",
        "petstore-simple",
        "petstore-with-external-docs",
        "ingram-micro",
        "api-with-examples",
        "callback-example",
        "link-example",
        "uber",
        "uspto",
        "hubspot-events",
        "hubspot-webhooks"
    )
    
    Write-Host "dotnet publish ../src/HttpGenerator/HttpGenerator.csproj -p:TreatWarningsAsErrors=true -p:PublishReadyToRun=true -o bin"
    Start-Process "dotnet" -Args "publish ../src/HttpGenerator/HttpGenerator.csproj -p:TreatWarningsAsErrors=true -p:PublishReadyToRun=true -o bin" -NoNewWindow -PassThru | Wait-Process
    
    "v3.0", "v2.0" | ForEach-Object {
        $version = $_
        "json", "yaml" | ForEach-Object {            
            $format = $_
            $filenames | ForEach-Object {
                $filename = "./OpenAPI/$version/$_.$format"
                $exists = Test-Path -Path $filename -PathType Leaf
                if ($exists -eq $true) {
                    Copy-Item $filename ./openapi.$format
                    Generate -format $format
                }
            }
        }
    }
}

Measure-Command { RunTests -Method "dotnet-run" -Parallel $Parallel }
Write-Host "`r`n"