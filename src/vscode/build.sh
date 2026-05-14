#!/bin/bash

set -euo pipefail

VERSION="${1:-}"
TARGET="${2:-}"

SCRIPT_ROOT="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_ROOT/../.." && pwd)"

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

resolve_rust_target() {
  case "$1" in
    win32-x64) echo "x86_64-pc-windows-msvc" ;;
    win32-arm64) echo "aarch64-pc-windows-msvc" ;;
    win32-ia32) echo "i686-pc-windows-msvc" ;;
    linux-x64) echo "x86_64-unknown-linux-gnu" ;;
    linux-arm64) echo "aarch64-unknown-linux-gnu" ;;
    linux-armhf) echo "armv7-unknown-linux-gnueabihf" ;;
    darwin-x64) echo "x86_64-apple-darwin" ;;
    darwin-arm64) echo "aarch64-apple-darwin" ;;
    *)
      echo "Unsupported VS Code target: $1" >&2
      return 1
      ;;
  esac
}

if [[ "$TARGET" == win32-* ]]; then
  BINARY_NAME="httpgenerator.exe"
else
  BINARY_NAME="httpgenerator"
fi

RUST_TARGET="$(resolve_rust_target "$TARGET")"
STAGED_BINARY_DIR="$SCRIPT_ROOT/bin/$TARGET"
RELEASE_BINARY="$REPO_ROOT/target/$RUST_TARGET/release/$BINARY_NAME"
VSIX_PATH="$SCRIPT_ROOT/http-file-generator-$VERSION-$TARGET.vsix"

rm -rf "$SCRIPT_ROOT/bin"
rm -f "$VSIX_PATH"

cd "$REPO_ROOT"
rustup target add "$RUST_TARGET"
cargo build --locked --release --target "$RUST_TARGET" -p httpgenerator

if [[ ! -f "$RELEASE_BINARY" ]]; then
  echo "Expected bundled CLI at '$RELEASE_BINARY' after cargo build." >&2
  exit 1
fi

mkdir -p "$STAGED_BINARY_DIR"
cp "$RELEASE_BINARY" "$STAGED_BINARY_DIR/$BINARY_NAME"

cd "$SCRIPT_ROOT"
npm ci
npm run compile
npx vsce package --target "$TARGET" --out "$VSIX_PATH"

python - "$VSIX_PATH" "$TARGET" "$BINARY_NAME" <<'PY'
import sys
import zipfile

vsix_path, target, binary_name = sys.argv[1:]
expected_entry = f"extension/bin/{target}/{binary_name}"

with zipfile.ZipFile(vsix_path) as archive:
    try:
        entry = archive.getinfo(expected_entry)
    except KeyError as exc:
        raise SystemExit(f"Expected bundled CLI entry '{expected_entry}' inside '{vsix_path}'.") from exc

    if entry.file_size <= 0:
        raise SystemExit(f"Bundled CLI entry '{expected_entry}' inside '{vsix_path}' is empty.")
PY

echo "HTTP File Generator for VS Code extension has been built to $VSIX_PATH"
