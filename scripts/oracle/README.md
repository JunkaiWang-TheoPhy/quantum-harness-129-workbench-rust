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

All atom strings in `generate.py` are interpreted as Angstrom because every
system carries `coordinate_unit="angstrom"` and the generator passes that
value explicitly to PySCF. PySCF/libcint converts coordinates internally to
Bohr. Total energies, orbital energies, nuclear repulsion, and FCIDUMP
integrals are in Hartree; overlap, orbital coefficients, CI coefficients, and
CC amplitudes are dimensionless.

The primary challenge fixture can be regenerated in isolation with:

```bash
.venv/bin/python scripts/oracle/generate.py h2o-631g-fc
```

It uses H2O/6-31G, `R(O-H)=0.967 Å`, `angle(H-O-H)=107.6°`, and freezes the
oxygen 1s orbital after RHF. Regeneration is an oracle audit, not a prerequisite
for the Rust calculations. Before replacing any committed primary fixture,
require the generated FCIDUMP checksum and every numerical reference field to
match or document and review the platform-dependent difference.

The review benchmark reference is deliberately RHF-only:

```bash
.venv/bin/python scripts/oracle/generate.py h2o-ccpvdz-ae
```

It uses all 10 electrons, 24 cc-pVDZ spatial orbitals, `symmetry=False`, and
records the fixed-`Nalpha=Nbeta=5` determinant dimension. It does not create
FCIDUMP, run FCI, CCSD, or MP2, or allocate a full CI vector.
