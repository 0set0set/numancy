#!/usr/bin/env python3
import sys
from pathlib import Path

# All crates in this workspace share a single version, inherited from
# [workspace.package] in the root Cargo.toml.
WORKSPACE_PACKAGES = ("numancy", "numancy-cli")


def set_workspace_version(cargo_toml: Path, version: str) -> None:
  lines = cargo_toml.read_text().splitlines()
  in_workspace_package = False
  updated = False

  for index, line in enumerate(lines):
    stripped = line.strip()

    if stripped == "[workspace.package]":
      in_workspace_package = True
      continue

    if in_workspace_package and stripped.startswith("[") and stripped.endswith("]"):
      break

    if in_workspace_package and stripped.startswith("version ="):
      lines[index] = f'version = "{version}"'
      updated = True
      break

  if not updated:
    raise RuntimeError("could not find [workspace.package].version in Cargo.toml")

  cargo_toml.write_text("\n".join(lines) + "\n")


def set_lockfile_versions(cargo_lock: Path, version: str) -> None:
  lines = cargo_lock.read_text().splitlines()
  current_package = None
  updated = set()

  for index, line in enumerate(lines):
    stripped = line.strip()

    if stripped == "[[package]]":
      current_package = None
      continue

    if stripped.startswith("name ="):
      current_package = stripped.split('"')[1] if '"' in stripped else None
      continue

    if (
      current_package in WORKSPACE_PACKAGES
      and stripped.startswith("version =")
      and current_package not in updated
    ):
      lines[index] = f'version = "{version}"'
      updated.add(current_package)

  missing = [pkg for pkg in WORKSPACE_PACKAGES if pkg not in updated]
  if missing:
    raise RuntimeError(f"could not find packages in Cargo.lock: {', '.join(missing)}")

  cargo_lock.write_text("\n".join(lines) + "\n")


def main() -> None:
  if len(sys.argv) != 2:
    raise SystemExit("usage: scripts/set-cargo-version.py <version>")

  version = sys.argv[1]
  set_workspace_version(Path("Cargo.toml"), version)
  set_lockfile_versions(Path("Cargo.lock"), version)


if __name__ == "__main__":
  main()
