#!/bin/bash

set -euo pipefail

VERSION=${1:-"0.1.0"}
TARGET=${2:-""}

cd "$(dirname "$0")"

if [[ -z "$TARGET" ]]; then
  case "$(uname -s)-$(uname -m)" in
    Linux-x86_64) TARGET="linux-x64" ;;
    Darwin-x86_64) TARGET="darwin-x64" ;;
    Darwin-arm64|Darwin-aarch64) TARGET="darwin-arm64" ;;
    MINGW*|MSYS*|CYGWIN*) TARGET="win32-x64" ;;
    *)
      echo "Unsupported host platform for automatic target selection: $(uname -s)-$(uname -m)"
      exit 1
      ;;
  esac
fi

REPO_ROOT="$(cd ../.. && pwd)"
EXE_NAME="httpgenerator"
if [[ "$TARGET" == win32-* ]]; then
  EXE_NAME="httpgenerator.exe"
fi

SOURCE_BINARY="$REPO_ROOT/target/release/$EXE_NAME"
BUNDLE_DIR="$(pwd)/bin/$TARGET"
BUNDLE_BINARY="$BUNDLE_DIR/$EXE_NAME"
OUTPUT_VSIX="http-file-generator-$VERSION-$TARGET.vsix"

(
  cd "$REPO_ROOT"
  cargo build --release -p httpgenerator
)

mkdir -p "$BUNDLE_DIR"
cp "$SOURCE_BINARY" "$BUNDLE_BINARY"

if [[ "$TARGET" != win32-* ]]; then
  chmod +x "$BUNDLE_BINARY"
fi

npm ci --ignore-scripts
npm run compile
npx @vscode/vsce package --target "$TARGET" --out "$OUTPUT_VSIX"

echo "HTTP File Generator for VS Code extension has been built to $OUTPUT_VSIX"
