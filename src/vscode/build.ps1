param (
    [string]$Version = "0.1.0"
)

Set-Location -Path $PSScriptRoot

# Install dependencies
npm install

# Compile the extension
npm run compile

# Package the extension
npm run package

Write-Host "HTTP File Generator for VS Code extension has been built to http-file-generator-$Version.vsix"
