![Ranger: determinant states around a gravitationally lensed accretion disk](https://raw.githubusercontent.com/JunkaiWang-TheoPhy/quantum.harness/refs/heads/media/ranger-pr-banners/assets/ranger/pr-217-ed-fci-accretion-states.png)

# Ranger: exact CC(8) to 451M-determinant FCI in Rust

Ranger transforms three core electronic-structure scaling barriers into three
composable algorithms. The result is a public Rust research engine spanning
FCI, CC, CI, MBPT, UCC, direct integrals, symmetry, deterministic parallelism,
restartable Davidson, and verified HPC execution.

## Breakthrough in one view

| Published-series accuracy | Largest exact sector | Large-run wall time | Verified HPC campaign |
|---:|---:|---:|---:|
| **36/36** | **451,681,246 determinants** | **3:55:43** | **560 CPUs** |

| Field | Value |
|---|---|
| Team | Ranger |
| Members | Chenxi Wan, Yedi Shen, Junkai Wang |
| Challenge | [#129: Exact diagonalization workbench in Rust](https://github.com/QuantumBFS/quantum.harness/issues/129) |
| Public source | [`JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust`](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust) |
| Release | [`v0.5.0`](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/releases/tag/v0.5.0) |
| License | AGPL-3.0 |

## Three barriers, three algorithms

### 1. Wave-function construction -> exact ranked subset convolution

The challenge's Taylor construction is upgraded to an excitation-rank
recurrence for `exp(T)|HF>`. Ranger precomputes alpha/beta partitions, assembles
every target coefficient from compatible amplitude/source subsets and exact
fermionic phases, and evaluates independent targets in parallel.

This project algorithm powers CC(1)-CC(8), terminates exactly at the finite
electron rank, and is checked coefficient-by-coefficient against the Taylor
oracle. The complete primary CC sequence runs in `186.94 s` on the recorded
Apple M4 environment.

### 2. Hamiltonian scale -> symmetry-compact matrix-free FCI

The spin-free direct sigma kernel applies the Hamiltonian through string links
and same-spin transitions. Compact `ORBSYM`/`ISYM` addresses propagate across
FCI, CI, MBPT, CC, and UCC.

For all-electron H2O/cc-pVDZ:

```text
1,806,590,016 determinants  ->  451,681,246 exact C2v/A1 determinants
```

The fourfold exact representation gain preserves every determinant in the
target ground-state sector, all ten electrons, and the same finite-basis
Hamiltonian.

### 3. Production reproducibility -> deterministic restartable Davidson

Fixed source blocks, thread-local vectors, and ordered reduction make a fixed
parallel policy bitwise repeatable. The primary sigma benchmark records a
**3.236817x** median timing ratio and a maximum serial/parallel difference of
`5.969e-13`.

Versioned memory/disk stores add atomic checkpoint generations, fingerprints,
memory preflight, and resume. Block Davidson extends the same engine to
several orthogonal roots.

## Why this reaches a new scale

The original target contains 245,025 determinants. Ranger's final exact sector
contains 451,681,246 determinants - more than 1,800 times the primary space.
Four advances compound:

1. rank recursion accelerates high-order CC wave-function construction;
2. matrix-free sigma removes determinant-matrix storage from the scale model;
3. point-group compact addressing delivers a fourfold exact representation gain;
4. deterministic parallelism and restartable storage support production HPC.

This integrated algorithmic stack is the reason Ranger progresses from a
reference implementation to a 451-million-determinant research result.

## Measured result ladder

| Hamiltonian and exact sector | Determinants | Rust result |
|---|---:|---:|
| H2O/6-31G, O 1s frozen | 245,025 | `-76.121174204141980 Eh` |
| H2O/DZ, all electron | 1,002,708 | `-76.156699030930056 Eh` |
| H2O/DZP, O 1s frozen | 28,233,466 | `-76.256624441300147 Eh` |
| H2O/cc-pVDZ, all electron, C2v/A1 | **451,681,246** | **`-76.24321859 Eh`** |

The largest solve reaches residual `6.602e-8` in 21 Davidson iterations and
`3:55:43`. Same-input PySCF through CCSD(T) supplies a method hierarchy;
CCSD(T) lies `0.647144 mEh` above the Rust FCI result.

The companion **symmetry-free resource characterization** covers the full
1,806,590,016-determinant representation, `13.460145 GiB` vector size,
integral generation, Rust RHF, AO-to-MO, determinant links, and sampled sparse
Hamiltonian columns. Together, the two cc-pVDZ results connect resource theory
to a completed exact calculation.

## Primary challenge: 36/36

The submitted H2O/6-31G Hamiltonian contains 12 active spatial orbitals, eight
active electrons, and 245,025 determinants.

- Matrix-free FCI: `-76.121174204141980 Eh`.
- CC(1)-CC(8): **8/8** Hirata-Bartlett entries match.
- CI(1)-CI(8): **8/8** entries match.
- MBPT(1)-MBPT(20): **20/20** entries match.
- CC(2) agrees with the independent PySCF CCSD oracle within `3.025e-10 Eh`.
- CC(8) reaches `7.998e-9 Eh` from FCI.
- CI(8) reaches `2.004e-12 Eh` from FCI.

The complete total is **36/36 published entries** at the precision printed in
Hirata and Bartlett 2000.

## Verified SCNet campaign

The pinned Rust implementation was rebuilt with a fully offline toolchain on
AMD EPYC 7742 nodes.

- **18/18** Davidson parameter cases converged.
- **216/216** cross-node repeat solves converged.
- Maximum energy range: `8.10e-13 Eh`.
- **560 CPUs** ran concurrently across ten tasks.
- 37 task-level SHA-256 manifests verify 960 evidence files.

Utilization measurements motivate four 14-thread solver processes per
56-core node. The resulting **1,008-CPU campaign design** schedules 72
independent processes across 18 nodes and provides a promising throughput
architecture for large method-development studies.

## One Rust engine, many methods

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

PySCF supplies independent fixture construction and cross-checks. The checked
production algorithms execute in Rust.

## Promising research platform

The shared determinant interface prepares a **selected-determinant frontier**:

1. deterministic HCI/iCI-style selection;
2. variational selected-space Davidson;
3. Epstein-Nesbet PT2 with explicit numerical budgets;
4. threshold extrapolation against the exact primary oracle;
5. natural-orbital and orbital-optimized stretched-water studies;
6. quantum-sampled determinant import through the same address layer.

The existing exact solver supplies the calibration oracle, symmetry labels,
sparse source action, eigensolver, and evidence schema for this next release.

## Public review package

- [Final technical report (PDF)](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/output/pdf/quantum-harness-129-final-technical-report.pdf)
- [Innovation-led technical article](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/reports/final-competition-summary.md)
- [Plain-text result card](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/output/data/quantum-harness-129-final-results.txt)
- [SHA-256 submission manifest](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/output/quantum-harness-129-submission-manifest.txt)
- [451M FCI machine record](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json)
- [C2v/A1 production report](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/reports/h2o-ccpvdz-c2v-fci.md)
- [SCNet campaign report](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/reports/scnet-hpc-benchmark.md)
- [Standalone reproduction prompt](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/docs/reproduction-prompt.md)
- [Continuous verification](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/actions/workflows/ci.yml)

## Reviewer tour

- [ ] Run `scripts/verify-submission.sh`.
- [ ] Review the 36/36 published-series matches.
- [ ] Inspect the exact ranked subset-convolution oracle tests.
- [ ] Inspect the 1.806B resource characterization and 451M exact C2v/A1 result.
- [ ] Review the deterministic parallel sigma measurement.
- [ ] Review the 18/18 robustness and 216/216 repeatability campaign.
- [ ] Open the PDF, result card, machine record, and SHA-256 manifest.

Ranger carries the project from equations to algorithms, from algorithms to
451 million determinants, and from one result to a reusable platform for the
next generation of exact and selected electronic-structure methods.
