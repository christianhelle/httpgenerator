#!/bin/bash

echo "Testing VS Code extension..."

# Navigate to extension directory
cd "$(dirname "$0")"

# Ensure the extension is built
./build.sh

VSIX_PATH="$(ls -t http-file-generator-*.vsix | head -n 1)"

if [[ -z "$VSIX_PATH" ]]; then
  echo "No VSIX package was produced by build.sh." >&2
  exit 1
fi

# Start VS Code with the extension
code --install-extension "$VSIX_PATH" --force

# Open the test folder that contains OpenAPI specs
code "$(dirname "$0")/../../test"

echo "VS Code has been launched with the HTTP File Generator extension installed."
echo "Please test the extension by right-clicking on an OpenAPI file (.json, .yaml, or .yml) and selecting the HTTP File Generator options."
