# test-extension.ps1
param (
    [Parameter()]
    [string]
    $WorkspacePath = "",
    
    [Parameter()]
    [switch]
    $SkipBuild
)

$ErrorActionPreference = "Stop"

# Set working directory to extension folder
Push-Location $PSScriptRoot

try {
    # Build extension if not skipped
    if (-not $SkipBuild) {
        Write-Host "Building extension..."
        & "$PSScriptRoot\build-extension.ps1"
        
        if ($LASTEXITCODE -ne 0) {
            Write-Error "Build failed with exit code $LASTEXITCODE"
            exit $LASTEXITCODE
        }
    }
    
    # Find the latest VSIX file
    $vsixFile = Get-ChildItem -Filter "http-file-generator-*.vsix" | 
                Sort-Object -Property LastWriteTime -Descending | 
                Select-Object -First 1
    
    if (-not $vsixFile) {
        Write-Error "No VSIX file found. Please build the extension first."
        exit 1
    }
    
    # Determine workspace to open
    $workspaceArg = ""
    if ($WorkspacePath) {
        if (Test-Path $WorkspacePath) {
            $workspaceArg = """$WorkspacePath"""
        } else {
            Write-Warning "Specified workspace path does not exist: $WorkspacePath"
        }
    }
    
    # Launch VS Code with the extension
    Write-Host "Launching VS Code with extension: $($vsixFile.Name)" -ForegroundColor Green
    $command = "code --extensionDevelopmentPath=""$PSScriptRoot"" --install-extension ""$($vsixFile.FullName)"" $workspaceArg"
    
    Write-Host "Running: $command"
    Invoke-Expression $command
    
} catch {
    Write-Error "Error testing extension: $_"
    exit 1
} finally {
    # Restore original directory
    Pop-Location
}