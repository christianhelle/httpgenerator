#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys
import time
import urllib.error
import urllib.request


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Wait for, or assert the absence of, a crate version on crates.io."
    )
    parser.add_argument("--crate", required=True, help="Crate name to query on crates.io.")
    parser.add_argument("--version", required=True, help="Version to look for.")
    parser.add_argument(
        "--state",
        choices=("present", "absent"),
        required=True,
        help="Expected visibility state for the requested version.",
    )
    parser.add_argument(
        "--retries",
        type=int,
        default=1,
        help="Number of checks to perform before failing.",
    )
    parser.add_argument(
        "--delay-seconds",
        type=int,
        default=0,
        help="Delay between retries.",
    )
    return parser.parse_args()


def fetch_versions(crate_name: str) -> set[str]:
    url = f"https://crates.io/api/v1/crates/{crate_name}"
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            payload = json.load(response)
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return set()
        raise

    return {item["num"] for item in payload.get("versions", [])}


def main() -> int:
    args = parse_args()

    for attempt in range(1, args.retries + 1):
        versions = fetch_versions(args.crate)
        is_present = args.version in versions

        if args.state == "present" and is_present:
            print(f"{args.crate} {args.version} is visible on crates.io after {attempt} check(s).")
            return 0

        if args.state == "absent" and not is_present:
            print(f"{args.crate} {args.version} is not present on crates.io.")
            return 0

        if attempt < args.retries and args.delay_seconds > 0:
            print(
                f"Waiting for {args.crate} {args.version} to become {args.state} "
                f"(attempt {attempt}/{args.retries})."
            )
            time.sleep(args.delay_seconds)

    expected = "visible on" if args.state == "present" else "absent from"
    print(
        f"::error::{args.crate} {args.version} was not {expected} crates.io after {args.retries} check(s).",
        file=sys.stderr,
    )
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
