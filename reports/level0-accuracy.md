# Level 0 Accuracy Report

Date: 2026-07-27

## Result

The Rust dense-FCI implementation reproduces the committed PySCF 2.14.0
oracles within the required `1e-10` hartree tolerance.

| System | Geometry | Basis | Determinants | Rust dense FCI | PySCF FCI | Absolute error |
|---|---|---|---:|---:|---:|---:|
| H2 | H-H = 1.4 Angstrom | STO-3G | 4 | -1.015468249288245 | -1.015468249288245 | 0.000e0 |
| Linear H4 | adjacent H-H = 1.0 Angstrom | STO-3G | 36 | -2.166387448634769 | -2.166387448634763 | 6.217e-15 |

Both PySCF RHF and CCSD also converged for these fixtures. The CCSD energies
are provenance fields and are not used to validate the dense-FCI kernel.

## Reproduction

Normal verification is Rust-only:

```bash
cargo test
cargo run -- verify fixtures/h2-sto3g/FCIDUMP fixtures/h2-sto3g/reference.json
cargo run -- verify fixtures/h4-sto3g/FCIDUMP fixtures/h4-sto3g/reference.json
```

The verifier checks the FCIDUMP SHA-256 before calculating:

```text
H2: 73c24ec1347eec64fc2fed18aeed44311bef60592d51d97a9582a16371fba467
H4: 801d3dd185e06b09f07927aa7ef961484eda4d7447eafb3288abd76eddfe6989
```

Oracle regeneration:

```bash
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python -r scripts/oracle/requirements.txt
.venv/bin/python scripts/oracle/generate.py
```

Snapshot environment:

- Python 3.12.11
- PySCF 2.14.0
- energies in hartree
- restricted Hartree-Fock canonical orbitals
- charge 0, spin 0, no frozen orbitals
- PySCF SCF and CC convergence tolerance `1e-12`
- FCIDUMP write threshold `1e-15`

## What This Proves

- The parser correctly handles the actual PySCF FCIDUMP layout used here.
- Spatial-to-spin integral expansion and fermionic signs produce a symmetric
  Hamiltonian.
- Dense Rust diagonalization agrees with an independent PySCF FCI solver for
  two nontrivial determinant dimensions.
- Committed fixtures allow Rust tests to run without Python or PySCF.

## Limitations and Level 1 Handoff

This implementation explicitly stores the Hamiltonian and uses a `u64`
spin-orbital determinant representation limited to 32 spatial orbitals. That
is deliberate for the transparent Level 0 oracle and is not the Level 1
algorithm.

Level 1 should retain these fixtures as regression oracles while replacing the
dense matrix with alpha/beta string excitation lists, direct `sigma = H C`,
Hamiltonian diagonal construction, and Davidson iteration.
