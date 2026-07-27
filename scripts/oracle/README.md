# PySCF Level 0 Oracle

Python is used only to generate independent fixtures. The Rust executable and
test suite do not import PySCF.

From the repository root:

```bash
uv venv --python 3.12
uv pip install --python .venv/bin/python -r scripts/oracle/requirements.txt
.venv/bin/python scripts/oracle/generate.py
```

The generator writes `FCIDUMP`, `reference.json`, and `ao_reference.json` into
the selected fixture directories. Generate only the equilibrium H2 fixture
without touching existing fixtures with:

```bash
.venv/bin/python scripts/oracle/generate.py h2-equilibrium-sto3g
```

The equilibrium fixture uses an H-H distance of 0.7414 Å. The original
`h2-sto3g` regression fixture is retained separately at 1.4 Å. Re-running a
fixture should preserve its numerical fields and FCIDUMP SHA-256 checksum when
the pinned PySCF version and platform math stack are unchanged.
