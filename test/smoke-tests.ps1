param (
    [Parameter(Mandatory=$false)]
    [bool]
    $Parallel = $true,

    [Parameter(Mandatory=$false)]
    [switch]
    $Production = $false
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
        $format,

        [Parameter(Mandatory=$true)]
        [string]
        $output,

        [Parameter(Mandatory=$false)]
        [string]
        $args = "",

        [Parameter(Mandatory=$false)]
        [bool]
        $production = $false
    )

    $app = "./bin/httpgenerator"
    if ($production) {
        $app = "httpgenerator"
    }

    Write-Host "$app ./openapi.$format --output ./Generated/$output --no-logging $args"
    $process = Start-Process $app `
        -Args "./openapi.$format --output ./Generated/$output --no-logging $args" `
        -NoNewWindow `
        -PassThru

    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "HttpGenerator failed"
    }

    Write-Host "$app ./openapi.$format --output ./Generated/$output --output-type OneFile --no-logging $args"
    $process = Start-Process $app `
        -Args "./openapi.$format --output ./Generated/$output --output-type OneFile --no-logging $args" `
        -NoNewWindow `
        -PassThru

    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "HttpGenerator failed"
    }

    Write-Host "$app ./openapi.$format --output ./Generated/$output --output-type OneFilePerTag --no-logging $args"
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
        $format,

        [Parameter(Mandatory=$true)]
        [string]
        $output,

        [Parameter(Mandatory=$true)]
        [string]
        $outputType,

        [Parameter(Mandatory=$false)]
        [string]
        $args = "",

        [Parameter(Mandatory=$false)]
        [bool]
        $production = $false
    )

    $app = "./bin/httpgenerator"
    if ($production) {
        $app = "httpgenerator"
    }

    Write-Host "$app ./openapi.$format --output ./Generated/$output --output-type $outputType --no-logging $args"
    $process = Start-Process $app `
        -Args "./openapi.$format --output ./Generated/$output --output-type $outputType --no-logging $args" `
        -NoNewWindow `
        -PassThru

    $process | Wait-Process
    if ($process.ExitCode -ne 0) {
        throw "HttpGenerator failed with args: $args"
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
        $Parallel = $false,

        [Parameter(Mandatory=$false)]
        [bool]
        $Production = $false
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
#         "non-oauth-scopes",
        "webhook-example"
    )

    Get-ChildItem '*.http' -Recurse | ForEach-Object { Remove-Item -Path $_.FullName }
 
    if ($Production -eq $true) {
        dotnet tool update -g HttpGenerator --prerelease
    } else {
        Write-Host "dotnet publish ../src/HttpGenerator/HttpGenerator.csproj -p:TreatWarningsAsErrors=false -p:PublishReadyToRun=true -o bin"
        Start-Process "dotnet" -Args "publish ../src/HttpGenerator/HttpGenerator.csproj -p:TreatWarningsAsErrors=false -p:PublishReadyToRun=true -o bin" -NoNewWindow -PassThru | Wait-Process
    }

    "v2.0", "v3.0", "v3.1" | ForEach-Object {
        $version = $_
        "json", "yaml" | ForEach-Object { 
            $format = $_
            $filenames | ForEach-Object {
                $filename = "./OpenAPI/$version/$_.$format"
                $exists = Test-Path -Path $filename -PathType Leaf
                if ($exists -eq $true) {
                    Write-Host "Testing $filename"
                    Copy-Item $filename ./openapi.$format
                    if ($version -eq "v3.1") {
                        Generate -format $format -output $_/$version/$format -args "--skip-validation --generate-intellij-tests --custom-header ""X-Custom-Header: 1234"" --base-url https://api.example.io/" -production $Production
                    } else {
                        Generate -format $format -output $_/$version/$format -args "--generate-intellij-tests --custom-header ""X-Custom-Header: 1234"" --base-url https://api.example.io/" -production $Production
                        
                        # Additional parameter combination tests for v2.0 and v3.0
                        if ($_ -eq "petstore") {
                            Write-Host "Testing $filename with --authorization-header"
                            GenerateWithSpecificArgs -format $format -output "$_/$version/$format/auth-header" -outputType "OneFile" -args "--authorization-header ""Bearer test-token-123""" -production $Production
                            
                            Write-Host "Testing $filename with --load-authorization-header-from-environment"
                            GenerateWithSpecificArgs -format $format -output "$_/$version/$format/auth-env" -outputType "OneFile" -args "--load-authorization-header-from-environment --authorization-header-variable-name ""my_token""" -production $Production
                            
                            Write-Host "Testing $filename with --skip-headers"
                            GenerateWithSpecificArgs -format $format -output "$_/$version/$format/skip-headers" -outputType "OneFile" -args "--skip-headers" -production $Production
                            
                            Write-Host "Testing $filename with --content-type application/xml"
                            GenerateWithSpecificArgs -format $format -output "$_/$version/$format/xml" -outputType "OneFile" -args "--content-type ""application/xml""" -production $Production
                            
                            Write-Host "Testing $filename with environment variable base URL"
                            GenerateWithSpecificArgs -format $format -output "$_/$version/$format/env-baseurl" -outputType "OneFile" -args "--base-url ""{{MY_BASE_URL}}""" -production $Production
                        }
                    }
                }
            }
        }
    }
}

Measure-Command { RunTests -Method "dotnet-run" -Parallel $Parallel -Production $Production }
Write-Host "`r`n"
