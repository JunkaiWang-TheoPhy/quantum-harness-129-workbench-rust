# PySCF Level 0 Oracle

Python is used only to generate independent fixtures. The Rust executable and
test suite do not import PySCF.

From the repository root:

```bash
uv venv --python 3.12
uv pip install --python .venv/bin/python -r scripts/oracle/requirements.txt
.venv/bin/python scripts/oracle/generate.py
```

The generator writes `FCIDUMP` and `reference.json` into
`fixtures/h2-sto3g/` and `fixtures/h4-sto3g/`. Re-running it should preserve
the numerical fields and FCIDUMP SHA-256 checksums when the pinned PySCF
version and platform math stack are unchanged.
