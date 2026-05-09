#!/usr/bin/env python3

from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys


SEMVER_PATTERN = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+([.-][0-9A-Za-z]+)*$")
WORKSPACE_PACKAGE_HEADER = "[workspace.package]"
SECTION_HEADER_PATTERN = re.compile(r"^\[[^\]]+\]\s*$")
VERSION_LINE_PATTERN = re.compile(r'^(\s*)version\s*=\s*"[^"]*"\s*$')


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
    version = args.version.strip()
    if not version:
        print("::error::The --version input must not be empty.", file=sys.stderr)
        return 1
    if not SEMVER_PATTERN.fullmatch(version):
        print(
            f"::error::Invalid release version '{args.version}'.",
            file=sys.stderr,
        )
        return 1

    manifest_path = Path(args.manifest)
    contents = manifest_path.read_text(encoding="utf-8")

    lines = contents.splitlines(keepends=True)
    in_workspace_package = False
    workspace_package_found = False
    version_updated = False

    for index, line in enumerate(lines):
        stripped_line = line.strip()
        if stripped_line == WORKSPACE_PACKAGE_HEADER:
            in_workspace_package = True
            workspace_package_found = True
            continue

        if in_workspace_package and SECTION_HEADER_PATTERN.match(stripped_line):
            break

        if in_workspace_package:
            match = VERSION_LINE_PATTERN.match(line.rstrip("\r\n"))
            if match:
                newline = "\r\n" if line.endswith("\r\n") else "\n" if line.endswith("\n") else ""
                lines[index] = f'{match.group(1)}version = "{version}"{newline}'
                version_updated = True
                break

    if not workspace_package_found:
        print(
            f"::error::Could not find {WORKSPACE_PACKAGE_HEADER} in {manifest_path}.",
            file=sys.stderr,
        )
        return 1

    if not version_updated:
        print(
            f"::error::Could not find a version entry in {WORKSPACE_PACKAGE_HEADER} within {manifest_path}.",
            file=sys.stderr,
        )
        return 1

    manifest_path.write_text(
        "".join(lines),
        encoding="utf-8",
    )
    print(f"Updated {WORKSPACE_PACKAGE_HEADER} version in {manifest_path}.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
