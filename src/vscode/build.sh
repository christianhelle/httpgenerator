#!/bin/bash

VERSION=${1:-"0.1.0"}
TARGET=${2:-""}            # vsce platform target, e.g. linux-x64, darwin-x64, darwin-arm64
RUST_BINARY_PATH=${3:-""}  # path to a pre-built httpgenerator binary to bundle

cd "$(dirname "$0")"

# Stage Rust binary if a path was provided
if [ -n "$RUST_BINARY_PATH" ]; then
    BIN_DIR="$(pwd)/bin"
    rm -rf "$BIN_DIR"
    mkdir -p "$BIN_DIR"
    cp "$RUST_BINARY_PATH" "$BIN_DIR/httpgenerator"
    chmod +x "$BIN_DIR/httpgenerator"
    echo "Staged Rust binary to $BIN_DIR/httpgenerator"
fi

# Install dependencies
npm install

# Compile the extension
npm run compile

# Package the extension
if [ -n "$TARGET" ]; then
    npx vsce package --target "$TARGET"
    echo "HTTP File Generator for VS Code extension has been built to http-file-generator-$VERSION-$TARGET.vsix"
else
    npx vsce package
    echo "HTTP File Generator for VS Code extension has been built to http-file-generator-$VERSION.vsix"
fi
