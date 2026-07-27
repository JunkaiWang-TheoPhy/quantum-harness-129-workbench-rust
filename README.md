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

## Level 2 Status

Level 2 arbitrary-order determinant CC(n) is complete:

- runtime-configurable excitation rank;
- generic normalized cluster substitutions on full-FCI vectors;
- finite Taylor construction of `exp(T)|HF>`;
- projected energy and residual equations;
- orbital-denominator updates and DIIS;
- CC(2) validation against PySCF CCSD;
- full-rank CC validation against FCI.

```bash
cargo run --release -- cc \
  fixtures/h2o-sto3g/FCIDUMP fixtures/h2o-sto3g/reference.json \
  --rank 2 --residual-tolerance 1e-7
```

See [reports/level2-cc-accuracy.md](reports/level2-cc-accuracy.md).

## Level 3 Status

All three Level 3 method families are implemented:

- CI(n) as an excitation-rank projected matrix-free Davidson problem;
- arbitrary-order MBPT recursion with order-by-order corrections;
- variational unitary CC(n) using `exp(T-T†)` and BFGS optimization.

H4 CI(4) and H2 UCC(2) reproduce their FCI limits. H2 MP2 matches the
independent PySCF MP2 oracle. See
[reports/level3-methods.md](reports/level3-methods.md).

## Level 4 Status

The direct-integral stack is complete for H2 and H2O/STO-3G:

- Rust calls `libcint` directly for overlap, kinetic, nuclear-attraction, and
  electron-repulsion integrals;
- Rust RHF implements symmetric orthogonalization, Coulomb/exchange Fock
  construction, DIIS, and convergence reporting;
- a staged AO-to-MO transformation feeds the shared matrix-free FCI solver;
- committed PySCF fixtures verify every AO and MO integral, RHF energies,
  orbital energies, and final FCI energies;
- the checked production commands have no Python runtime dependency.

```bash
cargo run --release -- rhf h2o-sto3g
cargo run --release -- direct-integrals-fci h2-sto3g
cargo run --release -- direct-integrals-fci h2o-sto3g
```

H2O/STO-3G gives RHF `-74.962663067690499` and direct FCI
`-75.012918738193051` hartree. The FCI error against PySCF is `1.485e-10`
hartree. See [reports/level4-integrals.md](reports/level4-integrals.md).

The required ecosystem findings are recorded in
[reports/tenferro-gap-list.md](reports/tenferro-gap-list.md). tenferro-rs 0.2.0
already supplies dense tensors, strided views, gather/scatter, division,
reductions, and contractions; the primary determinant-workload gap is
collision-reducing scatter-add with explicit deterministic semantics.

## Scope

- Parse FCIDUMP files generated from PySCF.
- Enumerate alpha/beta determinant strings.
- Build tiny-system dense Hamiltonians for oracle checks.
- Implement string-based direct FCI and Davidson iteration.
- Implement determinant-based arbitrary-order CC(n).
- Implement CI(n), MBPT(n), and unitary CC(n) from the same operator machinery.
- Compute direct libcint AO integrals, RHF, and AO-to-MO transformations.
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
- [reports/level2-cc-accuracy.md](reports/level2-cc-accuracy.md) records
  arbitrary-order CC convergence and oracle comparisons.
- [reports/level3-methods.md](reports/level3-methods.md) records CI(n),
  MBPT(n), and unitary CC(n) results.
- [reports/level4-integrals.md](reports/level4-integrals.md) records the
  Python-free direct-integral pipeline and element-level PySCF comparisons.
- [reports/tenferro-gap-list.md](reports/tenferro-gap-list.md) maps every
  #129 tensor requirement to the current tenferro-rs 0.2.0 API and proposes
  upstreamable reproducer work.

## Upstream Registration

- Challenge issue: https://github.com/QuantumBFS/quantum.harness/issues/129
- Registration PR: https://github.com/QuantumBFS/quantum.harness/pull/210
- Track folder: `tracks/ed/solutions/WangTheoPhys/`
- Registered team: Rewrite It In Rust! (RIIR 2607 Hefei)
- Members: Chenxi Wan, Yedi Shen, Junkai Wang
- Solution directory identifier: WangTheoPhys

## License

This repository is licensed under the GNU Affero General Public License v3.0.
