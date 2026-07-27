# ED Workbench RS

Public AGPL-3.0 workbench for Quantum Harness challenge
[#129](https://github.com/QuantumBFS/quantum.harness/issues/129):
**Exact diagonalization workbench in Rust for electronic structure method
development**.

The goal is to build a Rust reference implementation for determinant-based
FCI/ED machinery, with arbitrary-order coupled cluster as the mandatory
showcase.

## Challenge Status

The mandatory #129 path is complete on the primary H2O/6-31G frozen-core
Hamiltonian. The earlier all-amplitude CC exponential bottleneck was replaced
by an exact excitation-rank subset-convolution recurrence, with the original
Taylor expansion retained as an independent small-system oracle.

- Direct Davidson FCI converges in the full 245,025-determinant space.
- CC(1)-CC(8) matches all eight equilibrium CC entries in Hirata 2000
  Table 2 at the six decimals printed by the paper.
- CI(1)-CI(8) and MBPT(1)-MBPT(20) match all 28 corresponding Table 2
  entries.
- The Level 0 oracle, mandatory Levels 1-2, stretch Levels 3-4, tenferro gap
  list, machine-readable evidence, and upstream reproduction materials are
  committed.

The Kállay 2001 DZ/DZP calculations remain explicitly identified as extended
targets; no primary 6-31G result is presented as evidence for those different
Hamiltonians.

## Level 0 Status

Level 0 is complete for equilibrium and stretched H2 plus linear H4/STO-3G
tiny-system fixtures:

- PySCF-generated RHF, FCI, CCSD, FCIDUMP, and provenance/checksum artifacts;
- Rust FCIDUMP parsing with Mulliken symmetry and Fortran exponent support;
- Rust alpha/beta determinant enumeration and fermionic operator signs;
- Rust explicit dense Hamiltonian construction and symmetric diagonalization;
- automatic `1e-10`-hartree verification against PySCF FCI.

Run the committed fixtures without Python:

```bash
cargo run -- inspect fixtures/h2-sto3g/FCIDUMP
cargo run -- verify fixtures/h2-equilibrium-sto3g/FCIDUMP \
  fixtures/h2-equilibrium-sto3g/reference.json
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
- exact ranked subset-convolution construction of `exp(T)|HF>`, checked
  coefficient-by-coefficient against the independent finite Taylor path;
- projected energy and residual equations;
- orbital-denominator updates, DIIS, and determinant-indexed rank warm starts;
- CC(2) validation against PySCF CCSD;
- full CC(1)-CC(8) validation against Hirata 2000 Table 2.

```bash
RAYON_NUM_THREADS=10 cargo run --release -- cc-series \
  fixtures/h2o-631g-fc/FCIDUMP \
  fixtures/h2o-631g-fc/reference.json \
  --published-reference fixtures/h2o-631g-fc/hirata2000-table2.json \
  --max-rank 8 --residual-tolerance 1e-6 --max-iterations 100
```

All eight published differences pass at the paper's six-decimal precision.
CC(2) is within `3.025e-10` hartree of PySCF CCSD; CC(8) is within
`7.998e-9` hartree of FCI.

See [reports/level2-cc-accuracy.md](reports/level2-cc-accuracy.md).

## Level 3 Status

All three Level 3 method families are implemented:

- CI(n) as an excitation-rank projected matrix-free Davidson problem;
- arbitrary-order MBPT recursion with order-by-order corrections;
- variational unitary CC(n) using `exp(T-T†)` and BFGS optimization.

On the primary 245,025-determinant water target, every CI(1)-CI(8) and
MBPT(1)-MBPT(20) difference matches Hirata 2000 Table 2 at its printed
precision. CI(8) differs from FCI by `2.004e-12` hartree. H4 CI(4) and H2
UCC(2) additionally reproduce their small-system FCI limits. See
[reports/level3-methods.md](reports/level3-methods.md).

```bash
RAYON_NUM_THREADS=10 cargo run --release -- level3-series \
  fixtures/h2o-631g-fc/FCIDUMP \
  fixtures/h2o-631g-fc/reference.json \
  --published-reference fixtures/h2o-631g-fc/hirata2000-table2.json \
  --max-ci-rank 8 --max-mbpt-order 20 \
  --ci-residual-tolerance 1e-7 \
  --max-iterations 100 --max-subspace 24
```

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

The geometry and numerical units are explicit:

| Quantity | Unit |
|---|---|
| committed Cartesian input coordinates | Angstrom (`Å`) |
| PySCF/libcint internal coordinates | Bohr |
| total, orbital, nuclear-repulsion, and integral energies | Hartree (`E_h`) |
| overlap, MO coefficients, and CI/CC amplitudes | dimensionless |
| geometry angles | degree |

The equilibrium H2 fixture has `R(H-H)=0.7414 Å`. The original stretched-H2
fixture uses `z=-0.7 Å` and `z=+0.7 Å`, so its bond length is
`R(H-H)=1.4 Å`—not `0.7 Å`. Linear H4 has `1.0 Å` adjacent spacing. Both
water fixtures use `R(O-H)=0.967 Å` and `angle(H-O-H)=107.6°`.

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

## Verification

Install the pinned Python oracle environment once:

```bash
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python -r scripts/oracle/requirements.txt
```

Then run every normal submission gate—Rust formatting, Clippy, locked tests,
tracked-JSON validation, FCIDUMP checksums, Python unit/geometry tests, and
diff hygiene—with one command:

```bash
scripts/verify-submission.sh
```

Set `PYTHON=python3` when the pinned oracle dependencies are installed in the
active interpreter rather than `.venv`.

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
- [docs/reproduction-prompt.md](docs/reproduction-prompt.md) is the standalone
  submission prompt with exact revision, checksums, commands, tolerances,
  expected tables, and failure-reporting requirements.
- [docs/sync-log.md](docs/sync-log.md) records what was pulled from GitHub and
  how this public workbench relates to the Quantum Harness solution PR.
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
- Active solution PR: https://github.com/QuantumBFS/quantum.harness/pull/217
- Superseded registration PR: https://github.com/QuantumBFS/quantum.harness/pull/210
- Track folder: `tracks/ed/solutions/WangTheoPhys/`
- Registered team: Rewrite It In Rust! (RIIR 2607 Hefei)
- Members: Chenxi Wan, Yedi Shen, Junkai Wang
- Solution directory identifier: WangTheoPhys

## License

This repository is licensed under the GNU Affero General Public License v3.0.
