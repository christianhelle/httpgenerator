#!/bin/bash

echo "Testing VS Code extension..."

# Navigate to extension directory
cd "$(dirname "$0")"

# Ensure the extension is built
./build.sh

# Start VS Code with the extension
code --install-extension http-file-generator-*.vsix --force

# Open the test folder that contains OpenAPI specs
code "$(dirname "$0")/../../test"

echo "VS Code has been launched with the HTTP File Generator extension installed."
echo "Please test the extension by right-clicking on an OpenAPI file (.json, .yaml, or .yml) and selecting the HTTP File Generator options."
