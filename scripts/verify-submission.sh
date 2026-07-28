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

benchmark_json=fixtures/h2o-ccpvdz-ae/benchmark-m4.json
jq -e '
  .schema_version == 1 and
  .norb == 24 and
  .nelec == 10 and
  .nalpha == 5 and
  .nbeta == 5 and
  .space.alpha_strings == 42504 and
  .space.beta_strings == 42504 and
  .space.determinants == 1806590016 and
  .point_group_symmetry == false and
  .full_fci_executed == false and
  .memory_budget_bytes == 2147483648 and
  .bounded_memory.conservative_peak_bytes < .memory_budget_bytes and
  .rhf_absolute_error < 1e-8
' "$benchmark_json" >/dev/null

benchmark_summary_json=fixtures/h2o-ccpvdz-ae/benchmark-m4-summary.json
jq -e '
  .schema_version == 1 and
  .artifact_kind == "h2o-ccpvdz-five-process-summary" and
  .measured_commit == "025a6dd27836f2e9011ef63ee35630a667bdd786" and
  .measured_release == "v0.1.1" and
  .determinants == 1806590016 and
  .sources_per_run == 16 and
  .raw_contributions_per_run == 640016 and
  (.runs | length) == 5 and
  .aggregate.maximum_peak_rss_bytes == 468975616 and
  .aggregate.maximum_peak_rss_bytes < 2147483648
' "$benchmark_summary_json" >/dev/null

python_bin=${PYTHON:-.venv/bin/python}
if [[ ! -x "$python_bin" ]] && ! command -v "$python_bin" >/dev/null 2>&1; then
  printf 'Python oracle environment not found: %s\n' "$python_bin" >&2
  exit 1
fi
"$python_bin" -m unittest scripts.oracle.test_units -v

git diff --check
