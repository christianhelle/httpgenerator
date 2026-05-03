#!/bin/bash

VERSION=${1:-"0.1.0"}

# Navigate to extension directory
cd "$(dirname "$0")"

# Install dependencies
npm install

# Compile the extension
npm run compile

# Package the extension
npm run package

echo "HTTP File Generator for VS Code extension has been built to http-file-generator-$VERSION.vsix"
