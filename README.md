# ED Workbench RS

Private workspace for Quantum Harness challenge
[#129](https://github.com/QuantumBFS/quantum.harness/issues/129):
**Exact diagonalization workbench in Rust for electronic structure method
development**.

The goal is to build a Rust reference implementation for determinant-based
FCI/ED machinery, with arbitrary-order coupled cluster as the mandatory
showcase.

## Level 0 Status

Level 0 is complete for the H2 and linear H4/STO-3G tiny-system fixtures:

- PySCF-generated RHF, FCI, CCSD, FCIDUMP, and provenance/checksum artifacts;
- Rust FCIDUMP parsing with Mulliken symmetry and Fortran exponent support;
- Rust alpha/beta determinant enumeration and fermionic operator signs;
- Rust explicit dense Hamiltonian construction and symmetric diagonalization;
- automatic `1e-10`-hartree verification against PySCF FCI.

Run the committed fixtures without Python:

```bash
cargo run -- inspect fixtures/h2-sto3g/FCIDUMP
cargo run -- verify fixtures/h2-sto3g/FCIDUMP fixtures/h2-sto3g/reference.json
cargo run -- verify fixtures/h4-sto3g/FCIDUMP fixtures/h4-sto3g/reference.json
```

Regenerate the independent oracle only when needed:

```bash
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python -r scripts/oracle/requirements.txt
.venv/bin/python scripts/oracle/generate.py
```

See [reports/level0-accuracy.md](reports/level0-accuracy.md) for exact results.

## Level 1 Status

Level 1 direct FCI is complete:

- lexical alpha/beta string spaces and signed `E_pq` excitation links;
- matrix-free spin-free sigma contraction without storing the Hamiltonian;
- an independently computed Hamiltonian diagonal;
- Davidson with residual preconditioning and restart;
- H2O/STO-3G dense/direct/Davidson cross-validation;
- the 245,025-determinant frozen-core H2O/6-31G target.

```bash
cargo run --release -- davidson fixtures/h2o-631g-fc/FCIDUMP \
  --residual-tolerance 1e-7 --max-iterations 60 --max-subspace 20
```

The resulting energy is `-76.121174204141980` hartree with residual
`5.044e-8`, matching both PySCF and the published `-76.121174` anchor. See
[reports/level1-direct-fci.md](reports/level1-direct-fci.md).

## Scope

- Parse FCIDUMP files generated from PySCF.
- Enumerate alpha/beta determinant strings.
- Build tiny-system dense Hamiltonians for oracle checks.
- Implement string-based direct FCI and Davidson iteration.
- Implement determinant-based arbitrary-order CC(n).
- Record Rust ecosystem and `tenferro-rs` gap notes discovered along the way.

## Repository Map

- [docs/challenge-129-brief.md](docs/challenge-129-brief.md) records the upstream
  challenge metadata, required targets, deliverables, and grading anchors.
- [docs/implementation-roadmap.md](docs/implementation-roadmap.md) turns the
  challenge into an implementation sequence for this Rust repository.
- [docs/resources.md](docs/resources.md) indexes useful upstream GitHub and web
  resources, including PySCF, libcint, Psi4NumPy, tenferro-rs, and the cited
  literature.
- [docs/web-and-github-snapshot.md](docs/web-and-github-snapshot.md) records a
  dated, source-linked snapshot of the upstream issue, registration PR,
  #129 dependency versions, and implementation documentation entry points.
- [docs/reproducibility-notes.md](docs/reproducibility-notes.md) extracts the
  implementation-sensitive conventions, APIs, targets, tolerances, and
  provenance rules needed to reproduce the published calculations.
- [docs/sync-log.md](docs/sync-log.md) records what was pulled from GitHub and
  how this private repo relates to the public Quantum Harness registration PR.
- [docs/upstream-metadata.json](docs/upstream-metadata.json) is the
  machine-readable counterpart of the dated snapshot.
- [reports/level0-accuracy.md](reports/level0-accuracy.md) records the first
  Rust-vs-PySCF numerical acceptance results.
- [reports/level1-direct-fci.md](reports/level1-direct-fci.md) records the
  matrix-free Davidson water benchmarks.

## Upstream Registration

- Challenge issue: https://github.com/QuantumBFS/quantum.harness/issues/129
- Registration PR: https://github.com/QuantumBFS/quantum.harness/pull/210
- Track folder: `tracks/ed/solutions/WangTheoPhys/`
- Registered team: Rewrite It In Rust! (RIIR 2607 Hefei)
- Members: Chenxi Wan, Yedi Shen, Junkai Wang
- Solution directory identifier: WangTheoPhys

## License

This repository is licensed under the GNU Affero General Public License v3.0.
