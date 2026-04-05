Write-Host "Testing VS Code extension..."

Set-Location -Path $PSScriptRoot

function Get-VsCodeTarget {
    if ($IsWindows) {
        return "win32-x64"
    }

    if ($IsMacOS) {
        return "darwin-x64"
    }

    return "linux-x64"
}

$vsCodeTarget = Get-VsCodeTarget

# Ensure the extension is built
./build.ps1

# Start VS Code with the extension
code --install-extension "http-file-generator-0.1.0-$vsCodeTarget.vsix" --force

# Open the test folder that contains OpenAPI specs
code $PSScriptRoot/../../test

Write-Host "VS Code has been launched with the HTTP File Generator extension installed."
Write-Host "Please test the extension by right-clicking on an OpenAPI file (.json, .yaml, or .yml) and selecting the HTTP File Generator options."
