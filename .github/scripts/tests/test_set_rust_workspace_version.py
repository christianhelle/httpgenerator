from __future__ import annotations

import subprocess
import sys
import tempfile
import textwrap
import unittest
from pathlib import Path


SCRIPT_PATH = Path(__file__).resolve().parents[1] / "set-rust-workspace-version.py"


class SetRustWorkspaceVersionTests(unittest.TestCase):
    def _run_script(self, version: str, manifest_content: str) -> subprocess.CompletedProcess[str]:
        with tempfile.TemporaryDirectory() as temp_dir:
            manifest_path = Path(temp_dir) / "Cargo.toml"
            manifest_path.write_text(manifest_content, encoding="utf-8")
            return subprocess.run(
                [
                    sys.executable,
                    str(SCRIPT_PATH),
                    "--version",
                    version,
                    "--manifest",
                    str(manifest_path),
                ],
                text=True,
                capture_output=True,
                check=False,
            )

    def test_updates_only_workspace_package_version(self) -> None:
        manifest = textwrap.dedent(
            """\
            [workspace.package]
            version = "0.1.0"

            [workspace.dependencies]
            httpgenerator-core = { version = "0.1.0", path = "src/rust/core" }
            """
        )
        with tempfile.TemporaryDirectory() as temp_dir:
            manifest_path = Path(temp_dir) / "Cargo.toml"
            manifest_path.write_text(manifest, encoding="utf-8")
            result = subprocess.run(
                [
                    sys.executable,
                    str(SCRIPT_PATH),
                    "--version",
                    "1.2.3",
                    "--manifest",
                    str(manifest_path),
                ],
                text=True,
                capture_output=True,
                check=False,
            )

            self.assertEqual(result.returncode, 0, result.stderr)
            rewritten = manifest_path.read_text(encoding="utf-8")

        self.assertIn('version = "1.2.3"', rewritten)
        self.assertIn(
            'httpgenerator-core = { version = "0.1.0", path = "src/rust/core" }',
            rewritten,
        )

    def test_rejects_empty_version(self) -> None:
        manifest = "[workspace.package]\nversion = \"0.1.0\"\n"
        result = self._run_script("   ", manifest)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("must not be empty", result.stderr)

    def test_fails_when_workspace_package_is_missing(self) -> None:
        manifest = "[workspace]\nmembers = []\n"
        result = self._run_script("1.2.3", manifest)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Could not find [workspace.package]", result.stderr)


if __name__ == "__main__":
    unittest.main()
