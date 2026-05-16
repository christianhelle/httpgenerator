Write-Host "Testing VS Code extension..."

Set-Location -Path $PSScriptRoot

# Ensure the extension is built
./build.ps1

$vsix = Get-ChildItem -Path $PSScriptRoot -Filter "http-file-generator-*.vsix" |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1

if (-not $vsix) {
    throw "No VSIX package was produced by build.ps1."
}

# Start VS Code with the extension
code --install-extension $vsix.FullName --force

# Open the test folder that contains OpenAPI specs
code $PSScriptRoot/../../test

Write-Host "VS Code has been launched with the HTTP File Generator extension installed."
Write-Host "Please test the extension by right-clicking on an OpenAPI file (.json, .yaml, or .yml) and selecting the HTTP File Generator options."
