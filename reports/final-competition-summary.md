# Final Competition Report: A Reproducible Rust Ladder from CC(8) to 451 Million Determinants

Date: 2026-07-30

## Abstract

This submission delivers an electronic-structure exact-diagonalization
workbench whose checked production algorithms are implemented in Rust. The
mandatory H₂O/6-31G frozen-core Hamiltonian is solved by matrix-free Davidson
FCI and determinant-based CC(1) through CC(8); the same implementation also
provides CI, MBPT, UCC, direct `libcint` integrals, Rust RHF, point-group
symmetry, multi-root Davidson, full-rank UCC, deterministic parallel sigma,
and checkpointed Davidson storage.

The central result is not one isolated energy but an auditable scale ladder.
On the primary 245,025-determinant problem, all 36 CC, CI, and MBPT entries
used from Hirata and Bartlett's Table 2 match at the paper's printed
precision. On SCNet, 18 numerical-robustness cases and 216 repeated solves all
converged, with an observed peak of 560 allocated CPUs across ten independent
tasks. At the largest completed scale, an all-electron H₂O/cc-pVDZ Davidson
solve in the C₂ᵥ/A1 ground-state block included 451,681,246 determinants and
converged to **−76.24321859 Eh** with residual `6.602e-8`.

The evidence is deliberately fail-closed. The 1,008-CPU ensemble was
submitted but not observed at that concurrency; the 1,806,590,016-determinant
symmetry-free cc-pVDZ space was bounded but not solved; selected CI and
orbital optimization remain proposed follow-ups. Two final-audit provenance
gaps—the unavailable raw Slurm accounting row for the 451-million run and an
unarchived exact production `direct_fci.rs`—are recorded rather than hidden.

## What was built

The workbench follows the method rather than outsourcing it to Python:

- FCIDUMP parsing, determinant enumeration, signs, symmetry sectors, and
  matrix-free spin-free sigma application;
- restarted single- and multi-root Davidson with memory preflight,
  deterministic threaded reduction, checkpointing, and resume validation;
- determinant CC(n) with an exact ranked subset-convolution exponential,
  together with CI(n), MBPT(n), and variational UCC(n);
- direct `libcint` AO integrals, Rust RHF/DIIS, AO-to-MO transformation, and
  the same FCI engine used by the FCIDUMP path.

PySCF is used to generate checksum-identified fixtures and small or
lower-level cross-checks. It does not execute the checked Rust production
paths. The distinction between published rounded values, independent
oracles, Rust calculations, and performance observations is maintained in
the [provenance register](data-provenance.md).

## Result ladder

| Hamiltonian and sector | Determinants | Result | Evidence status |
|---|---:|---|---|
| H₂O/6-31G, O 1s frozen | 245,025 | FCI, CC(1)-CC(8), CI(1)-CI(8), MBPT(1)-MBPT(20) | primary acceptance complete |
| H₂O/DZP, O 1s frozen | 28,233,466 | Davidson FCI `−76.256624441300147 Eh` | converged extension; six-decimal literature anchor |
| H₂O/cc-pVDZ all electron, C₂ᵥ/A1 | 451,681,246 | Davidson FCI **`−76.24321859 Eh`**, residual `6.602e-8` | converged symmetry-block result |
| H₂O/cc-pVDZ all electron, no point-group reduction | 1,806,590,016 | vector/storage and sparse-kernel benchmark | bounded benchmark; converged full FCI not run |

“Full CI” in the third row means all determinants with the recorded electron
number, spin projection, and A1 spatial symmetry. For a symmetry-preserving
Hamiltonian this is the exact A1 block, not an active-space truncation. It is
also not evidence that the four-times-larger symmetry-free representation was
executed.

## Numerical validation

The validation ladder is intentionally stronger at small scale, where truly
independent references are affordable:

1. Dense Rust FCI matches PySCF for H₂/STO-3G, linear H₄/STO-3G, and
   H₂O/STO-3G to between `2.2e-16` and `2.3e-13 Eh` on the SCNet binary.
2. The primary matrix-free H₂O/6-31G solve converges near
   `−76.121174204142 Eh`; CC(1)-CC(8), CI(1)-CI(8), and MBPT(1)-MBPT(20)
   match all 36 values used from Hirata and Bartlett 2000 at the six decimals
   printed by the paper.
3. Stretched-water fixtures at 1.5 and 2.0 times the equilibrium O–H distance
   prevent a single equilibrium point from serving as the only test.
4. Dense/direct, serial/parallel, symmetry-projected, checkpoint/resume, and
   multi-root comparisons exercise implementation paths independently.
5. For the 451-million A1 solve, the immutable FCIDUMP and logs, convergence
   residual, method hierarchy through same-input PySCF CCSD(T), and a
   literature-scale gap provide consistency checks. There is no independent
   same-Hamiltonian FCI oracle at that dimension, so the public energy is
   reported to eight decimal places rather than treating the solver's full
   printed decimal string as independently validated.

## HPC limit, resource estimate, and efficiency

The local Apple M4 benchmark measured deterministic four-block parallel sigma
at a `3.236817x` ratio of median serial to parallel time on the primary
245,025-determinant problem. SCNet then tested the same pinned v0.4.0 binary on
AMD EPYC 7742 nodes:

- 18/18 combinations of residual tolerance and Davidson subspace converged;
- 216/216 repeated solves converged, and every set of 12 repeats stored an
  identical energy for its parameter case;
- ten simultaneous 56-CPU tasks produced an observed allocation peak of 560
  CPUs on ten nodes;
- each small solve used about 20.63 effective busy cores on average despite a
  56-CPU allocation.

That last observation is the important efficiency result. For a small
245,025-dimensional solve, assigning more threads to one process saturates
before it consumes the whole node efficiently. The proposed 1,008-CPU job
therefore packed four 14-thread solver processes per node across 18 nodes. It
was an ensemble-throughput experiment, not MPI scaling of one eigenproblem.
The job was submitted, but the completed evidence observed only 560 CPUs; no
thousand-CPU performance claim is made.

The 451-million C₂ᵥ/A1 solve used one 64-thread task with 64 deterministic
sigma blocks and finished in 3:55:43. It requested 384 GiB. A scheduler
summary reported 222.257 GiB MaxRSS, but the raw `sacct` row could not be
retrieved with the repository credentials during the final audit, so that
memory figure is transcribed rather than independently verified. For the
symmetry-free 1.806-billion space, one `f64` CI vector is 13.460 GiB and the
current minimum vector storage is about 67.301 GiB before a practical
Davidson subspace and parallel workspace are included. This explains why the
bounded kernel is runnable while a converged in-memory solve was not claimed.

## Flexibility versus efficiency

The determinant representation is transparent and general: arbitrary
excitation ranks, symmetry sectors, several roots, direct integrals, and
multiple wave-function models share the same auditable basis and Hamiltonian
action. Its cost is combinatorial growth. Deterministic thread-local sigma
buffers also trade memory for parallel throughput, while ordered reduction
trades some peak speed for reproducibility.

Point-group symmetry was the decisive exact optimization at the largest
scale: it reduced the A1 representation by four without discarding an orbital
or electron. For future problems, however, exact symmetry alone is not enough.
The next algorithmic gain should come from reducing the variational subspace,
not merely requesting more cores for an exponentially growing full space.

## Fair comparison with selected CI and quantum approaches

These method families answer different questions and should be compared on
the same geometry, basis, electron treatment, target state, and error metric:

| Approach | What is diagonalized | Main strength | Required disclosure for a fair result |
|---|---|---|---|
| This work's FCI | every determinant in the declared finite symmetry sector | deterministic finite-basis reference and complete eigenvector | determinant count, symmetry, residual, memory, wall time, input hash |
| HCI/SHCI or iCIPT2 | an iteratively selected variational space, often plus EN-PT2 | reaches much larger orbital spaces with far fewer retained configurations | selection threshold, variational dimension, PT2 and extrapolation policy, stochastic error if any |
| Truncated CC | excitation-rank parameterization | size-extensive low-rank accuracy near a single reference | rank, convergence, reference dependence, behavior in strong correlation |
| NISQ/VQE/QSCI | a circuit expectation or a classically diagonalized subspace sampled from hardware | explores hybrid state preparation and determinant selection | qubits, gates, shots, noise/error mitigation, classical post-processing, total uncertainty and resource cost |

Heat-bath CI provides an efficient determinant-selection rule
([Holmes, Tubman, and Umrigar, JCTC 2016](https://doi.org/10.1021/acs.jctc.6b00407));
iCI with selection organizes screened spin-adapted spaces and estimates the
remaining dynamic correlation with EN-PT2
([Zhang, Liu, and Hoffmann, JCTC 2020](https://doi.org/10.1021/acs.jctc.9b01200)).
Quantum-selected CI samples determinants from a prepared quantum state and
then diagonalizes the selected Hamiltonian classically
([Kanno et al., QSCI](https://arxiv.org/abs/2302.11320)). These are natural
comparators because the Rust determinant engine could consume the same kind
of selected index set. They are not implemented results in this submission.

A claim of “chemical accuracy” alone would not make the comparison fair. A
selected-CI energy needs its variational and perturbative components; a
quantum result needs sampling and hardware uncertainty plus classical cost;
this work's FCI needs the declared symmetry sector and finite-basis boundary.
Wall time should be compared only with matched hardware or accompanied by
complete resource accounting.

## Response to reviewer directions and next work

The reviewer requested an HPC limit study, a fair comparison to leading
quantum-chemistry and NISQ strategies, and consideration of iCI/selected CI
and orbital optimization. The implemented response is:

- **HPC limit:** repeatability and robustness are demonstrated at an observed
  560-CPU ensemble peak; the per-solve utilization measurement motivates
  process packing. One large C₂ᵥ/A1 FCI is complete. Multi-node strong scaling
  of one Davidson solve is not yet implemented.
- **Fair comparison:** the table above separates exact symmetry-block FCI,
  selected-subspace methods, truncated CC, and hybrid quantum selection by
  algorithm, error source, and resource disclosure instead of comparing only
  final energy digits.
- **Selected CI and orbital optimization:** the next milestone is a common
  selected-determinant interface followed by deterministic HCI/iCI-style
  selection, a variational solve, EN-PT2 with an explicit error budget, and
  threshold extrapolation. Natural-orbital and orbital-optimized iterations
  should then be benchmarked on stretched water, where reference dependence
  is visible. Quantum-sampled determinant lists can enter through the same
  interface and be judged against equal-size classical selections.

This sequence is more scientifically informative than starting a new
billion-dimensional full-space run during the final hours. It converts the
current exact solver into an oracle for calibrating future approximations.

## Reproduction and claim boundaries

Run the complete local gate with:

```bash
scripts/verify-submission.sh
```

The lightweight final evidence audit alone is:

```bash
python3 scripts/hpc/verify_final_evidence.py
```

Detailed artifacts are in the [C₂ᵥ/A1 report](h2o-ccpvdz-c2v-fci.md), the
[SCNet report](scnet-hpc-benchmark.md), and their machine-readable JSON
fixtures. The final audit accepts the numerical C₂ᵥ/A1 result while marking
provenance as incomplete: the exact production `direct_fci.rs` and raw Slurm
accounting are unavailable. Consequently this is published as a
post-submission experimental extension on top of the stable v0.4.0 baseline,
not as a silently rewritten release.

The submission does not claim a completed symmetry-free cc-pVDZ full FCI,
1,008 observed CPUs, single-solve MPI scaling, selected CI, EN-PT2, natural
orbitals, orbital optimization, or quantum advantage.
