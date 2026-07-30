# Exactness at Scale in Rust

[![CI](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust)](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/releases)
[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue.svg)](LICENSE)

**CC(1)-CC(8. Exact 451,681,246-determinant FCI. Deterministic Davidson.
Direct integrals. One Rust research engine.**

Ranger built this workbench for
[Quantum Harness #129](https://github.com/QuantumBFS/quantum.harness/issues/129).
It turns determinant algebra into a reusable platform for FCI, arbitrary-order
CC, CI, MBPT, UCC, direct `libcint` integrals, Rust RHF/DIIS, symmetry sectors,
multi-root eigensolvers, reproducible parallel execution, and HPC evidence.

## Breakthrough in one view

| Published-series accuracy | Largest exact sector | Large-run wall time | Verified HPC campaign |
|---:|---:|---:|---:|
| **36/36** | **451,681,246 determinants** | **3:55:43** | **560 CPUs** |

Public review package:

- [Final technical PDF](output/pdf/quantum-harness-129-final-technical-report.pdf)
- [Innovation-led technical article](reports/final-competition-summary.md)
- [Plain-text result card](output/data/quantum-harness-129-final-results.txt)
- [SHA-256 submission manifest](output/quantum-harness-129-submission-manifest.txt)
- [Standalone reproduction prompt](docs/reproduction-prompt.md)
- [Upstream submission PR](https://github.com/QuantumBFS/quantum.harness/pull/217)

## Three barriers, three algorithms

### 1. Exact ranked subset convolution

The original challenge presents finite Taylor construction for the
coupled-cluster exponential. Ranger introduces an exact excitation-rank
recurrence for `exp(T)|HF>`. It precomputes alpha/beta partitions, assembles
each target coefficient from compatible amplitude/source subsets and exact
fermionic phases, and processes ranks in dependency order.

The result is one computational primitive for CC(1) through full-rank CC(8),
validated coefficient-by-coefficient against the independent Taylor oracle.
The primary complete CC sequence runs in `186.94 s` on the recorded Apple M4
environment.

### 2. Symmetry-compact matrix-free FCI

The spin-free sigma kernel applies the Hamiltonian directly through string
links and same-spin transitions. `ORBSYM` and `ISYM` propagate from FCIDUMP
through active spaces, determinant addressing, FCI, CI, MBPT, CC, and UCC.

For all-electron H2O/cc-pVDZ, compact C2v/A1 addressing transforms the
representation:

```text
1,806,590,016 determinants  ->  451,681,246 exact A1 determinants
```

This fourfold exact representation gain preserves every determinant in the
target ground-state sector and carries all ten electrons through the same
finite-basis Hamiltonian.

### 3. Deterministic restartable Davidson

Fixed source blocks, thread-local accumulation, and ordered reduction make a
fixed parallel policy bitwise repeatable. On the 245,025-determinant primary
sigma workload, the median timing ratio is **3.236817x** while the maximum
serial/parallel difference remains `5.969e-13`.

Versioned vector stores add atomic checkpoint generations, hashes, memory
preflight, and resume. Block Davidson supplies several orthogonal Ritz roots
through the same operator and storage interfaces.

## Why this reaches a new scale

The initial 245,025-determinant implementation established exact numerical
acceptance. Four advances then compound:

1. ranked subset convolution accelerates high-rank wave-function construction;
2. matrix-free sigma removes Hamiltonian-matrix storage from the scale model;
3. compact symmetry addresses deliver a fourfold exact representation gain;
4. deterministic parallelism and restartable storage support production HPC.

The final exact sector is more than 1,800 times the primary challenge space.
This progression turns a reference implementation into a promising research
platform for new determinant-based methods.

## Measured result ladder

| Hamiltonian and sector | Determinants | Rust result |
|---|---:|---:|
| H2O/6-31G, O 1s frozen | 245,025 | `-76.121174204141980 Eh` |
| H2O/DZ, all electron | 1,002,708 | `-76.156699030930056 Eh` |
| H2O/DZP, O 1s frozen | 28,233,466 | `-76.256624441300147 Eh` |
| H2O/cc-pVDZ, all electron, C2v/A1 | **451,681,246** | **`-76.24321859 Eh`** |

The largest run converges with residual `6.602e-8` in 21 Davidson iterations
and `3:55:43`. The companion **symmetry-free resource characterization**
covers the 1,806,590,016-determinant representation, `13.460145 GiB` vector
size, complete Rust integral/RHF/AO-to-MO path, determinant links, and sampled
sparse Hamiltonian columns.

## Primary 36/36 acceptance

The primary H2O/6-31G Hamiltonian freezes the oxygen 1s orbital and contains
12 active spatial orbitals, eight active electrons, and 245,025 determinants.

| Method family | Range | Published matches |
|---|---:|---:|
| Coupled cluster | CC(1)-CC(8) | **8/8** |
| Configuration interaction | CI(1)-CI(8) | **8/8** |
| Many-body perturbation theory | MBPT(1)-MBPT(20) | **20/20** |

All **36/36** entries match Hirata and Bartlett at the paper's printed
precision. CC(2) agrees with the independent PySCF CCSD oracle within
`3.025e-10 Eh`; CC(8) reaches `7.998e-9 Eh` from FCI; CI(8) reaches
`2.004e-12 Eh` from FCI.

Validation also includes H2, linear H4, H2O/STO-3G, stretched water at
`1.5 R_e` and `2.0 R_e`, full-rank H4 UCC, several H4 roots, symmetry
projection, memory/disk equivalence, checkpoint/resume equivalence, and
serial/parallel equivalence.

## Verified SCNet campaign

The pinned Rust source was rebuilt through a fully offline toolchain on AMD
EPYC 7742 nodes.

| Evidence | Result |
|---|---:|
| Davidson parameter matrix | **18/18 converged** |
| Cross-node repeat solves | **216/216 converged** |
| Maximum energy range | `8.10e-13 Eh` |
| Verified manifests | 37 |
| Archived evidence files | 960 |
| Allocation peak | **560 CPUs across ten tasks** |

Per-solve utilization measurements motivate four 14-thread solver processes
per 56-core node. The resulting **1,008-CPU campaign design** schedules 72
independent processes across 18 nodes. This provides a concrete throughput
architecture for high-volume determinant-method studies.

## Rust-native production path

```text
libcint AO integrals
        |
        v
Rust RHF/DIIS -> AO-to-MO -> determinant basis + symmetry addresses
                                      |
                                      v
                         matrix-free Hamiltonian action
                          /      |       |       \
                        FCI     CI(n)   CC(n)   UCC(n)
                                 |       |
                              MBPT(n)  CC(8)
```

PySCF supplies independent fixtures and cross-checks. The checked production
algorithms execute in Rust.

## Architecture

The crate is organized around small reusable scientific interfaces:

- `ElectronicProblem`: integrals, electrons, spin, orbitals, and symmetry;
- `DeterminantBasis`: alpha/beta strings and compact target-sector addresses;
- `StringSpace`: signed one-body excitation links;
- `LinearOperator`: common matrix-free Hamiltonian contract;
- `DirectFciOperator`: serial or deterministic fixed-block sigma;
- `DavidsonRunConfig`: memory/disk storage and restart policy;
- `ClusterExpansionPlan`: exact ranked subset convolution;
- common result fixtures with explicit units, hashes, residuals, and commands.

Checked combinadic rank/unrank supports general active spaces and large
determinant counts. Direct AO integrals flow through symmetric
orthogonalization, Coulomb/exchange Fock construction, DIIS, and staged
AO-to-MO transformation.

## Quick start

Complete acceptance:

```bash
uv sync --locked
scripts/verify-submission.sh
```

Build and run Rust tests:

```bash
cargo build --release --locked
cargo test --locked
```

Primary Davidson FCI:

```bash
cargo run --release --locked -- davidson \
  fixtures/h2o-631g-fc/FCIDUMP \
  --residual-tolerance 1e-7 \
  --max-iterations 60 \
  --max-subspace 20
```

Primary CC(1)-CC(8):

```bash
cargo run --release --locked -- cc-series \
  fixtures/h2o-631g-fc/FCIDUMP \
  fixtures/h2o-631g-fc/reference.json \
  --published-reference fixtures/h2o-631g-fc/hirata2000-table2.json \
  --max-rank 8 \
  --residual-tolerance 1e-6 \
  --max-iterations 100
```

CI(1)-CI(8) and MBPT(1)-MBPT(20):

```bash
cargo run --release --locked -- level3-series \
  fixtures/h2o-631g-fc/FCIDUMP \
  fixtures/h2o-631g-fc/reference.json \
  --published-reference fixtures/h2o-631g-fc/hirata2000-table2.json \
  --max-ci-rank 8 \
  --max-mbpt-order 20
```

Direct-integral Rust path:

```bash
cargo run --release --locked -- direct-integrals h2o-sto3g
```

Evidence audit:

```bash
python3 scripts/hpc/verify_final_evidence.py
```

## Release progression

| Release | Research advance |
|---|---|
| `v0.1.1` | all-electron cc-pVDZ resource characterization |
| `v0.2.0` | general active spaces and checked numerical contracts |
| `v0.3.0` | restartable disk-backed Davidson storage |
| `v0.4.0` | deterministic memory-budgeted parallel sigma |
| `v0.5.0` | symmetry-resolved exact FCI, multi-root validation, and HPC evidence |

The immutable [v0.5.0 release](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/releases/tag/v0.5.0)
captures the complete research progression.

## Selected-determinant frontier

The shared determinant interface prepares the next algorithmic release:

1. deterministic HCI/iCI-style selection;
2. variational selected-space Davidson;
3. Epstein-Nesbet PT2 with explicit numerical budgets;
4. threshold extrapolation against the exact primary oracle;
5. natural-orbital and orbital-optimized stretched-water studies;
6. quantum-sampled determinant import through the same address layer.

This **selected-determinant frontier** combines the exact Rust calibration
engine with a route to FCI-quality accuracy across much larger orbital spaces.

## Reports and evidence

- [Final competition article](reports/final-competition-summary.md)
- [Level 0 accuracy](reports/level0-accuracy.md)
- [Direct FCI](reports/level1-direct-fci.md)
- [CC accuracy](reports/level2-cc-accuracy.md)
- [CI, MBPT, and UCC](reports/level3-methods.md)
- [Direct integrals](reports/level4-integrals.md)
- [Stretched water](reports/stretched-water.md)
- [Multi-root Davidson and UCC](reports/multiroot-and-ucc.md)
- [H2O/DZ extension](reports/extended-h2o-dz.md)
- [H2O/DZP extension](reports/extended-h2o-dzp.md)
- [cc-pVDZ resource characterization](reports/h2o-ccpvdz-all-electron-benchmark.md)
- [451M C2v/A1 FCI](reports/h2o-ccpvdz-c2v-fci.md)
- [SCNet campaign](reports/scnet-hpc-benchmark.md)
- [Data provenance](reports/data-provenance.md)
- [tenferro operation map](reports/tenferro-gap-list.md)

## Scientific foundations

- Hirata and Bartlett, general-order determinant CC through octuple
  excitations, [DOI](https://doi.org/10.1016/S0009-2614(00)00387-0).
- Knowles and Handy, determinant-based direct FCI,
  [DOI](https://doi.org/10.1016/0009-2614(84)85513-X).
- Holmes, Tubman, and Umrigar, heat-bath configuration interaction,
  [DOI](https://doi.org/10.1021/acs.jctc.6b00407).
- Zhang, Liu, and Hoffmann, iterative configuration interaction with
  selection, [DOI](https://doi.org/10.1021/acs.jctc.9b01200).

## Team

Ranger: Chenxi Wan, Yedi Shen, and Junkai Wang.<br>
License: [AGPL-3.0-only](LICENSE).
