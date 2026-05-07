#!/usr/bin/env python3

from __future__ import annotations

import argparse
from pathlib import Path
import sys


PLACEHOLDER = 'version = "0.1.2"'


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Update every shared Rust workspace version placeholder in Cargo.toml."
    )
    parser.add_argument("--version", required=True, help="Semver to inject into the workspace manifest.")
    parser.add_argument(
        "--manifest",
        default="Cargo.toml",
        help="Path to the workspace Cargo.toml file (default: Cargo.toml).",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    manifest_path = Path(args.manifest)
    contents = manifest_path.read_text(encoding="utf-8")
    replacements = contents.count(PLACEHOLDER)
    if replacements == 0:
        print(
            f"No '{PLACEHOLDER}' anchors found in {manifest_path}.",
            file=sys.stderr,
        )
        return 1

    manifest_path.write_text(
        contents.replace(PLACEHOLDER, f'version = "{args.version}"'),
        encoding="utf-8",
    )
    print(f"Updated {replacements} version anchor(s) in {manifest_path}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
