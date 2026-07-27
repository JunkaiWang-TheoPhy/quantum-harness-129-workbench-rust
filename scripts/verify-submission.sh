#!/usr/bin/env bash

set -euo pipefail

repository_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$repository_root"

cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

while IFS= read -r -d '' json_file; do
  jq empty "$json_file"
done < <(git ls-files -z '*.json')

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

while IFS= read -r -d '' reference_file; do
  expected=$(jq -r '.fcidump_sha256 // empty' "$reference_file")
  if [[ -n "$expected" ]]; then
    fcidump="$(dirname "$reference_file")/FCIDUMP"
    [[ -f "$fcidump" ]]
    actual=$(sha256_file "$fcidump")
    [[ "$actual" == "$expected" ]]
  fi
done < <(find fixtures -name reference.json -type f -print0)

python_bin=${PYTHON:-.venv/bin/python}
if [[ ! -x "$python_bin" ]] && ! command -v "$python_bin" >/dev/null 2>&1; then
  printf 'Python oracle environment not found: %s\n' "$python_bin" >&2
  exit 1
fi
"$python_bin" -m unittest scripts.oracle.test_units -v

git diff --check
