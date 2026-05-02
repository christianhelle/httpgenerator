#!/bin/bash

VERSION=${1:-"0.1.0"}

# Navigate to extension directory
cd "$(dirname "$0")"

case "$(uname -s)" in
  Linux*)
    VSCODE_TARGET="linux-x64"
    RUST_BINARY="httpgenerator"
    ;;
  Darwin*)
    VSCODE_TARGET="darwin-x64"
    RUST_BINARY="httpgenerator"
    ;;
  *)
    echo "Unsupported platform for build.sh"
    exit 1
    ;;
esac

# Build and bundle the Rust CLI
cargo build --release -p httpgenerator-cli
mkdir -p bin
cp "../../target/release/$RUST_BINARY" "bin/$RUST_BINARY"

# Install dependencies
npm ci

# Compile the extension
npm run compile

# Package the extension
npx vsce package --target "$VSCODE_TARGET"

echo "HTTP File Generator for VS Code extension has been built to http-file-generator-$VERSION-$VSCODE_TARGET.vsix"
