# Exactness at Scale: From CC(8) to 451 Million Determinants in Rust

Date: 2026-07-30<br>
Team: Ranger - Chenxi Wan, Yedi Shen, Junkai Wang<br>
Challenge: Quantum Harness #129

## Breakthrough in one view

| Published-series accuracy | Largest exact sector | Large-run wall time | Verified HPC campaign |
|---:|---:|---:|---:|
| **36/36** | **451,681,246 determinants** | **3:55:43** | **560 allocated CPUs** |

Ranger delivers a complete Rust electronic-structure workbench that advances
from arbitrary-order coupled cluster through CC(8) to matrix-free full
configuration interaction in a 451-million-determinant exact symmetry sector.
The same determinant engine powers FCI, CC, CI, MBPT, UCC, multi-root
Davidson, direct `libcint` integrals, Rust RHF/DIIS, symmetry resolution,
deterministic parallel sigma application, and restartable storage.

The central contribution is an algorithmic ladder rather than one isolated
energy. Exact ranked subset convolution makes full-rank coupled cluster
practical. Compact point-group addressing turns symmetry into a fourfold
exact representation gain. Fixed-block ordered reduction combines parallel
speed with bitwise repeatability. Together these advances carry the original
245,025-determinant challenge target to an all-electron
451,681,246-determinant H2O/cc-pVDZ calculation.

## Three barriers, three algorithms

### 1. Exact ranked subset convolution for `exp(T)|HF>`

The challenge specification follows the general-order determinant strategy of
Hirata and Bartlett and proposes constructing the coupled-cluster wave
function through repeated Taylor applications. Ranger introduces a direct
excitation-rank recurrence. For a target excitation set `R`, the coefficient
is assembled from every compatible amplitude subset `A` and the already
completed coefficient of `R \ A`, including the exact fermionic phase:

```text
C(R) = (1 / |R|) * sum_A |A| * phase(A, R\A) * t(A) * C(R\A)
```

Processing targets in increasing excitation rank makes every source
coefficient available exactly when needed. Alpha and beta partitions are
precomputed, determinant amplitudes use direct address maps, and targets above
the parallel threshold are evaluated concurrently. A finite Taylor
implementation remains an independent coefficient-by-coefficient oracle.

This Ranger algorithm provides three practical gains:

- exact termination at the finite electron rank;
- direct reuse of lower-rank coefficients;
- one implementation for CC(1) through the full-rank CC(8) limit.

The complete primary CC(1)-CC(8) sequence finishes in `186.94 s` on the
recorded Apple M4 environment and reproduces every published equilibrium
entry. The resulting implementation converts the exponential ansatz from a
formal definition into a tested general-rank computational primitive.

### 2. Symmetry-compact matrix-free FCI

The FCI engine applies the Hamiltonian as a spin-free sigma contraction. It
precomputes alpha and beta string links, same-spin transitions, and the
absorbed one-electron contribution, then accumulates destinations directly.
The determinant Hamiltonian therefore remains an operator rather than a
stored matrix.

Ranger propagates `ORBSYM` and `ISYM` metadata through FCIDUMP parsing,
active-space transformations, determinant construction, addressing, dense
and direct FCI, CI, MBPT, CC, and UCC. Determinant pairs receive compact
addresses inside the requested Abelian irrep. For all-electron H2O/cc-pVDZ:

```text
1,806,590,016 determinants  ->  451,681,246 C2v/A1 determinants
```

This is a fourfold exact representation gain. Every determinant in the target
A1 sector remains present, all ten electrons remain correlated, and the
finite-basis Hamiltonian remains the same. Symmetry turns a resource model
into a completed exact C2v/A1 sector calculation.

### 3. Deterministic, restartable Davidson at scale

Parallel sigma application divides source determinants into fixed contiguous
blocks. Each block owns a thread-local partial vector; the final vector is
assembled in block order. This execution policy makes the result independent
of Rayon scheduling while preserving shared-memory throughput.

On the 245,025-determinant primary problem, the measured median times are:

| Execution policy | Median sigma time |
|---|---:|
| Serial | `14.181091542 s` |
| Four fixed blocks | `4.381184834 s` |
| Ratio of medians | **`3.236817x`** |

The maximum serial/parallel element difference is `5.969e-13`, and repeated
fixed-policy parallel runs are bitwise identical. Memory preflight sizes every
thread-local workspace before execution.

Davidson vectors also support a versioned local-NVMe store. Atomic generation
updates, input fingerprints, configuration hashes, checksum validation, and
resume semantics turn a long eigensolve into a recoverable computation. Block
Davidson extends the same framework to several orthogonal low-energy roots.

## Why Ranger reaches a new scale

The original challenge target contains 245,025 determinants and specifies a
Taylor-based CC construction. The first Ranger implementation delivered that
target. The final workbench adds four capabilities that compound rather than
compete:

1. **Rank recursion** transforms high-order CC wave-function construction.
2. **Matrix-free sigma** removes determinant-matrix storage from the scale
   equation.
3. **Compact symmetry addressing** reduces the exact target representation by
   four.
4. **Deterministic parallel and restartable storage** make production runs
   reproducible and operationally resilient.

Each advance unlocks the next one. Matrix-free algebra makes hundreds of
millions of determinants representable; symmetry makes the target vector set
fit the production memory envelope; deterministic parallelism accelerates the
dominant kernel; restartable Davidson supports long, evidence-producing runs.
The result is a coherent algorithmic stack whose largest completed sector is
more than 1,800 times the primary challenge space.

## Primary acceptance: 36/36 published entries

The mandatory H2O/6-31G Hamiltonian uses the submitted equilibrium geometry,
freezes the oxygen 1s orbital, and contains 12 active spatial orbitals, eight
active electrons, and 245,025 determinants.

| Method family | Delivered range | Published matches | Representative result |
|---|---:|---:|---:|
| Matrix-free FCI | ground state | independent acceptance | `-76.121174204142 Eh` |
| Coupled cluster | CC(1)-CC(8) | **8/8** | CC(8): `-76.121174196144139 Eh` |
| Configuration interaction | CI(1)-CI(8) | **8/8** | CI(8): `-76.121174204143969 Eh` |
| Many-body perturbation theory | MBPT(1)-MBPT(20) | **20/20** | all printed orders |

All **36/36** equilibrium values match Hirata and Bartlett at the six decimal
places printed in the source paper. CC(2) agrees with the independent PySCF
CCSD oracle within `3.025e-10 Eh`. CC(8) reaches `7.998e-9 Eh` from FCI, and
CI(8) reaches `2.004e-12 Eh` from FCI.

Stretched-water calculations at `1.5 R_e` and `2.0 R_e` extend the validation
beyond one equilibrium point. Dense/direct, serial/parallel,
symmetry-projected, memory/disk, checkpoint/resume, and multi-root comparisons
exercise independent computational paths.

## Exact scaling ladder

| Hamiltonian and exact sector | Determinants | Rust result | Contribution |
|---|---:|---:|---|
| H2O/6-31G, O 1s frozen | 245,025 | `-76.121174204142 Eh` | complete FCI/CC/CI/MBPT acceptance |
| H2O/DZ, all electron | 1,002,708 | `-76.156699030930056 Eh` | million-determinant all-electron validation |
| H2O/DZP, O 1s frozen | 28,233,466 | `-76.256624441300147 Eh` | 28-million-determinant Davidson extension |
| H2O/cc-pVDZ, all electron, C2v/A1 | **451,681,246** | **`−76.24321859 Eh`** | largest completed exact sector |

The cc-pVDZ calculation uses 24 spherical spatial orbitals, all ten electrons,
`Nalpha=Nbeta=5`, and the water ground-state A1 irrep. Davidson reaches a
residual norm of `6.602e-8` in 21 iterations and `3:55:43` wall time.
Same-input PySCF RHF, MP2, CISD, CCSD, and CCSD(T) calculations provide a
method hierarchy; CCSD(T) lies `0.647144 mEh` above the Rust FCI energy.

The associated **symmetry-free resource characterization** measures the full
1,806,590,016-determinant representation. One `f64` CI vector is
`13.460145 GiB`; the benchmark executes integral generation, Rust RHF,
AO-to-MO transformation, link construction, distributed source sampling, and
sparse Hamiltonian columns under a `2 GiB` kernel budget. This companion result
quantifies exactly why symmetry-compact addressing delivers the decisive
production gain.

## Verified SCNet campaign

The SCNet campaign rebuilds the pinned Rust source in an offline toolchain,
runs the complete test suite, performs numerical smoke checks, sweeps Davidson
parameters, and repeats solves across nodes.

| Evidence | Result |
|---|---:|
| Robustness matrix | **18/18 converged** |
| Repeated solves | **216/216 converged** |
| Maximum energy range | `8.10e-13 Eh` |
| Verified manifests | 37 |
| Archived evidence files | 960 |
| Observed allocation peak | **560 allocated CPUs across ten tasks** |

Per-solve measurements show that a small 245,025-dimensional job reaches its
throughput optimum below a full 56-core node. Ranger therefore develops a
four-process-per-node, 14-thread process-packing strategy. The resulting
**1,008-CPU campaign design** schedules 72 independent solver processes across
18 nodes. The verified 560-CPU campaign establishes the portability,
repeatability, and throughput model that motivates this packing strategy.

The 451-million-determinant production solve uses one 64-thread task with 64
deterministic source blocks. The same fixed-block policy connects the local
bitwise-repeatability tests to the largest HPC result.

## One engine, many electronic-structure methods

The workbench is designed around reusable determinant algebra rather than a
single solver command:

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

This architecture directly addresses the flexibility-efficiency question
raised during review. The optimized primitives - determinant addressing,
fermionic phases, Hamiltonian links, symmetry sectors, vector stores, and
fixed-block reduction - remain shared across the method families. Each new
method inherits the same dense oracles, fixtures, residual checks, and
reproduction workflow.

## Relation to established research

Ranger builds on a strong scientific lineage and contributes a new Rust
algorithmic realization:

| Foundation | Established contribution | Ranger advance |
|---|---|---|
| Hirata-Bartlett general-order CC | determinant formulation through octuple excitations | exact ranked subset-convolution construction, parallel rank layers, integrated CC(1)-CC(8) oracle |
| Knowles-Handy/Olsen determinant FCI | string-based direct Hamiltonian action | symmetry-compact addressing, deterministic fixed-block reduction, memory preflight, restartable storage |
| Davidson eigensolver | iterative extremal eigenpairs | versioned memory/disk stores, restart validation, multi-root block solver |
| HCI and iCI selection | compact variational spaces plus perturbative recovery | exact Rust calibration oracle and shared selected-determinant interface target |

The project-level novelty lies in the recurrence and in the integrated system:
one type-safe Rust engine spans arbitrary-rank wave-function methods, exact
symmetry sectors, deterministic parallel execution, direct integrals, and
machine-verifiable HPC evidence.

Scientific foundations:

- S. Hirata and R. J. Bartlett, “High-order coupled-cluster calculations
  through connected octuple excitations,” *Chemical Physics Letters* 321,
  216-224 (2000), [DOI](https://doi.org/10.1016/S0009-2614(00)00387-0).
- P. J. Knowles and N. C. Handy, determinant-based direct FCI foundations,
  *Chemical Physics Letters* 111, 315 (1984),
  [DOI](https://doi.org/10.1016/0009-2614(84)85513-X).
- A. A. Holmes, N. M. Tubman, and C. J. Umrigar, “Heat-Bath Configuration
  Interaction,” *JCTC* 12, 3674-3680 (2016),
  [DOI](https://doi.org/10.1021/acs.jctc.6b00407).
- N. Zhang, W. Liu, and M. R. Hoffmann, “Iterative Configuration Interaction
  with Selection,” *JCTC* 16, 2296-2316 (2020),
  [DOI](https://doi.org/10.1021/acs.jctc.9b01200).

## Selected-determinant frontier

The exact solver creates a high-value calibration oracle for the next Ranger
release. A common selected-determinant interface can reuse the existing
addresses, signs, sparse source application, Davidson solver, symmetry labels,
and evidence schema. The research sequence is:

1. deterministic HCI/iCI-style determinant selection;
2. variational diagonalization in the selected space;
3. Epstein-Nesbet PT2 with an explicit numerical budget;
4. threshold extrapolation against the exact 245,025-determinant oracle;
5. natural-orbital and orbital-optimized iterations on stretched water;
6. quantum-sampled determinant lists through the same interface.

This **selected-determinant frontier** combines the completed exact engine with
the leading route to FCI-quality accuracy in much larger orbital spaces. The
exact result ladder supplies training targets, convergence references, and
equal-size comparisons for classical and quantum selection strategies.

## Validated scope

The public result package defines four complementary accomplishments:

- **Exact method acceptance:** FCI plus 36/36 CC/CI/MBPT published values on
  the primary Hamiltonian.
- **Exact large sector:** all-electron H2O/cc-pVDZ in the exact C2v/A1 sector,
  451,681,246 determinants, `-76.24321859 Eh`, residual `6.602e-8`.
- **Resource characterization:** the 1.806-billion-determinant symmetry-free
  representation, vector-size model, bounded kernel, and process-packing
  design.
- **Verified HPC campaign:** 18/18 robustness cases, 216/216 repeated solves,
  560 allocated CPUs, 37 manifests, and 960 evidence files.

Each accomplishment has a dedicated machine-readable record. The public
energy precision follows the converged residual, and the evidence snapshot
pins input hashes, solver configuration, logs, hardware context, and
acceptance fields.

## Reproduction and public artifacts

Run the complete local gate:

```bash
uv sync --locked
scripts/verify-submission.sh
```

Run the lightweight final evidence audit:

```bash
python3 scripts/hpc/verify_final_evidence.py
```

Review package:

- [Final technical PDF](../output/pdf/quantum-harness-129-final-technical-report.pdf)
- [Plain-text result card](../output/data/quantum-harness-129-final-results.txt)
- [Submission checksum manifest](../output/quantum-harness-129-submission-manifest.txt)
- [451M C2v/A1 FCI report](h2o-ccpvdz-c2v-fci.md)
- [SCNet campaign report](scnet-hpc-benchmark.md)
- [Machine-readable 451M result](../fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json)
- [Standalone reproduction prompt](../docs/reproduction-prompt.md)
- [Upstream submission PR](https://github.com/QuantumBFS/quantum.harness/pull/217)

Ranger demonstrates a promising path for exact electronic-structure method
development: start from one rigorously tested determinant algebra, introduce
algorithms that compound across scale, and carry every advance from equations
to source, data, HPC evidence, and reproducible public artifacts.
