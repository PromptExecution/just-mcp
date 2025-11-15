#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/.." >/dev/null 2>&1 && pwd)"

cd "${REPO_ROOT}"

export JUST_MCP_REPO_ROOT="${REPO_ROOT}"

python - <<'PY'
import json
import pathlib
import re
import os

repo_root = os.environ.get("JUST_MCP_REPO_ROOT")
if not repo_root:
    raise SystemExit("JUST_MCP_REPO_ROOT is not set")
repo = pathlib.Path(repo_root)

override = os.environ.get("JUST_MCP_OVERRIDE_VERSION")
if override:
    version = override
else:
    cargo_path = repo / "Cargo.toml"
    cargo_text = cargo_path.read_text()
    match = re.search(r'^\s*version\s*=\s*"([^"]+)"', cargo_text, re.MULTILINE)
    if not match:
        raise SystemExit("Unable to determine version from Cargo.toml")
    version = match.group(1)

server_path = repo / "server.json"
server = json.loads(server_path.read_text())
identifier = f"ghcr.io/promptexecution/just-mcp:{version}"

server["version"] = version
for package in server.get("packages", []):
    if package.get("registryType") == "oci":
        package["identifier"] = identifier
        package.setdefault("transport", {"type": "stdio"})

server_path.write_text(json.dumps(server, indent=2) + "\n")
print(f"Updated server.json to version {version}")  # output: log updated version
PY

git add server.json
