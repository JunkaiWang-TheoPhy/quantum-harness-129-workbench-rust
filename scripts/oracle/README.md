# PySCF Level 0 Oracle

Python is used only to generate independent fixtures. The Rust executable and
test suite do not import PySCF.

From the repository root:

```bash
uv sync --locked
uv run --frozen python scripts/oracle/generate.py
```

The root `.python-version`, `pyproject.toml`, and `uv.lock` pin CPython
3.12.11, uv 0.11.32, PySCF 2.14.0, and all transitive dependencies. The
`scripts/oracle/requirements.txt` file remains only as a minimal compatibility
input for tools that cannot consume a uv project lock.

The generator writes `FCIDUMP`, `reference.json`, and `ao_reference.json` into
the selected fixture directories. Generate only the equilibrium H2 fixture
without touching existing fixtures with:

```bash
uv run --frozen python scripts/oracle/generate.py h2-equilibrium-sto3g
```

The equilibrium fixture uses an H-H distance of 0.7414 Å. The original
`h2-sto3g` regression fixture is retained separately at 1.4 Å. Re-running a
fixture should preserve its numerical fields and FCIDUMP SHA-256 checksum when
the pinned PySCF version and platform math stack are unchanged.

All atom strings in `generate.py` are interpreted as Angstrom because every
system carries `coordinate_unit="angstrom"` and the generator passes that
value explicitly to PySCF. PySCF/libcint converts coordinates internally to
Bohr. Total energies, orbital energies, nuclear repulsion, and FCIDUMP
integrals are in Hartree; overlap, orbital coefficients, CI coefficients, and
CC amplitudes are dimensionless.

Symmetry-enabled FCIDUMPs use one-based Molpro `ORBSYM` labels in the range
1–8 and a one-based `ISYM` target. When calling PySCF's
`pyscf.tools.fcidump.from_scf`, pass `molpro_orbsym=True`; PySCF's default
zero-based internal labels are a different convention. The Rust determinant
basis retains only alpha/beta string pairs whose direct-product irrep equals
`ISYM`.

The primary challenge fixture can be regenerated in isolation with:

```bash
uv run --frozen python scripts/oracle/generate.py h2o-631g-fc
```

It uses H2O/6-31G, `R(O-H)=0.967 Å`, `angle(H-O-H)=107.6°`, and freezes the
oxygen 1s orbital after RHF. Regeneration is an oracle audit, not a prerequisite
for the Rust calculations. Before replacing any committed primary fixture,
require the generated FCIDUMP checksum and every numerical reference field to
match or document and review the platform-dependent difference.

The extended all-electron H2O/DZ fixture can be regenerated with:

```bash
uv run --frozen python scripts/oracle/generate.py h2o-dz-ae
```

This system uses the exact Bohr coordinates and printed O `(9s5p)/[4s2p]` and
H `(4s)/[2s]` contractions from Bauschlicher and Taylor 1986, not a similarly
named modern basis. Spatial symmetry is enabled and exported with
`molpro_orbsym=True`.
