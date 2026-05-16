#!/bin/bash

set -euo pipefail

VERSION="${1:-}"
TARGET="${2:-}"

SCRIPT_ROOT="$(cd "$(dirname "$0")" && pwd)"

if [[ -z "$VERSION" ]]; then
  VERSION="$(node -p "require(process.argv[1]).version" "$SCRIPT_ROOT/package.json")"
fi

if [[ -z "$TARGET" ]]; then
  case "$(uname -s)-$(uname -m)" in
    Linux-x86_64) TARGET="linux-x64" ;;
    Linux-aarch64) TARGET="linux-arm64" ;;
    Linux-armv7l) TARGET="linux-armhf" ;;
    Darwin-x86_64) TARGET="darwin-x64" ;;
    Darwin-arm64) TARGET="darwin-arm64" ;;
    *)
      echo "Unsupported platform: $(uname -s)-$(uname -m)" >&2
      exit 1
      ;;
  esac
fi

VSIX_PATH="$SCRIPT_ROOT/http-file-generator-$VERSION-$TARGET.vsix"

rm -f "$VSIX_PATH"

cd "$SCRIPT_ROOT"
npm ci
npm run compile
npx vsce package --target "$TARGET" --out "$VSIX_PATH"

echo "HTTP File Generator for VS Code extension has been built to $VSIX_PATH"
