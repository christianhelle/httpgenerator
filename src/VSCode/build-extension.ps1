# build-extension.ps1
param (
    [Parameter()]
    [string]
    $Version = "0.1.0"
)

$ErrorActionPreference = "Stop"

# Set working directory to extension folder
Push-Location $PSScriptRoot

try {
    Write-Host "Building HTTP File Generator VS Code Extension v$Version"
    
    # Install dependencies
    Write-Host "Installing npm dependencies..."
    npm install
    
    # Compile TypeScript
    Write-Host "Compiling TypeScript..."
    npm run compile
    
    # Package the extension
    Write-Host "Packaging extension..."
    npx vsce package --no-dependencies
    
    # Check if the extension was created successfully
    $vsixFile = Get-ChildItem -Filter "http-file-generator-$Version.vsix" | Select-Object -First 1
    if ($vsixFile) {
        Write-Host "Extension packaged successfully: $($vsixFile.Name)" -ForegroundColor Green
    } else {
        Write-Error "Failed to package the extension"
    }
} catch {
    Write-Error "Error building extension: $_"
    exit 1
} finally {
    # Restore original directory
    Pop-Location
}