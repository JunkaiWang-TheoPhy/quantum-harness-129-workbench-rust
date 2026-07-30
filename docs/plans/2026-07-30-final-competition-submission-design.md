# Final Competition Submission Design

## Status

Approved for immediate implementation on 2026-07-30 with approximately five
competition hours remaining.

## Objective

Turn the already validated v0.4.0 workbench, the SCNet repeatability evidence,
and the post-submission solver extensions into one public, auditable final
competition artifact without overstating symmetry-free FCI, thousand-core
scaling, or selected-CI capabilities.

## Fixed Scientific Claims

The final submission may claim the following only when the linked artifacts
and tests remain present:

- H2O/6-31G frozen core has 245,025 determinants and preserves the published
  FCI, CC(1)-CC(8), CI(1)-CI(8), and MBPT(1)-MBPT(20 acceptance results.
- The pinned v0.4.0 executable completed 18 Davidson robustness cases and 216
  repeated SCNet solves. The completed run observed at most 560 allocated
  CPUs across ten simultaneous tasks.
- The 1,008-CPU gang experiment is a task-parallel ensemble experiment, not
  MPI scaling of one Davidson eigenproblem. It may be described as observed
  only after the fail-closed 72-worker/216-result evidence gate passes.
- H2O/cc-pVDZ without point-group symmetry has 1,806,590,016 determinants.
  Its bounded executable stages were measured, but converged symmetry-free
  full FCI was not executed.
- H2O/cc-pVDZ all electron in the C2v/A1 ground-state block has 451,681,246
  determinants. The archived production calculation converged with residual
  6.602e-8 and stored energy -76.243218589558566 Eh. Public prose reports the
  energy as -76.24321859 Eh, consistent with the residual and absence of an
  independent same-Hamiltonian FCI oracle.
- HCI/iCI, EN-PT2, natural orbitals, and orbital optimization remain future
  work. They are not implemented by this final-submission effort.

## Architecture

The final artifact is assembled on a dedicated branch created from
`origin/codex/challenge-129-optimizations`. The independently developed SCNet
benchmark branch is merged into it so that scientific extensions, HPC
orchestration, raw evidence, tests, reports, and reproduction instructions are
addressable by one immutable commit.

During implementation, v0.5.0 was published from the upstream integration
work. Its tag is immutable and is not moved. The final branch therefore acts
as a corrective evidence supplement: it merges v0.5.0, passes formatting,
Clippy, all non-ignored tests, the submission verifier, evidence hash checks,
and GitHub CI, and tightens claims without rewriting the release tag.

## Evidence Model

### Production source

The final result must not rely on a prose assertion that two differently
hashed `direct_fci.rs` files differ only by formatting. The authoritative path
is, in preference order:

1. archive the exact source used to build production job 23008083 and a patch
   to the public source; or
2. rerun from one public immutable commit and make the rerun canonical; or
3. keep the result explicitly labeled as an archived production result with a
   disclosed source-provenance limitation.

### Slurm accounting

The final audit could log in to XH5 as the available `cfys01` and `acamtw70yu`
identities, but jobs 23008083 and 23015354 belong to another account and were
not returned by `sacct`; `/work/home/qbics2622` is also inaccessible. No raw
accounting file is fabricated. The machine-readable result marks raw
accounting and MaxRSS verification false, and public prose labels the memory
number as a transcribed scheduler summary.

### Numerical presentation

Machine-readable artifacts retain full floating-point output. Human-facing
titles, abstracts, PR text, and tables use precision supported by residuals
and independent checks. Tests compare numerical values with explicit
tolerances and verify convergence, residual, input hashes, and claim scope;
they do not treat a long decimal string as independent validation.

## Local Workstream

The local machine is the control plane. It integrates branches, validates
small-system correctness, audits raw HPC data, writes the final report and
article, updates the reproduction prompt, and prepares the public PR update.
No new solver family is developed during the final sprint.

## HPC Workstream

HPC is used only to retrieve immutable accounting evidence, observe the
already submitted gang job, and conditionally run one source-pinned C2v/A1
reproduction if it can start early enough to finish before the deadline. No
symmetry-free billion-determinant Davidson job or new selected-CI sweep is
submitted.

## Final Report and Article

`reports/final-competition-summary.md` is the canonical narrative. It contains
the validated method ladder, exact numerical anchors, determinant scale
ladder, local and HPC performance evidence, the flexibility-efficiency
tradeoff, a fair exact-FCI/selected-subspace/NISQ comparison framework, claim
boundaries, and future work.

The article emphasizes a reproducible Rust determinant workbench rather than
an SOTA performance claim. Its principal scale table distinguishes:

| Calculation | Determinants | Status |
|---|---:|---|
| H2O/6-31G frozen core | 245,025 | validated exact FCI and CC/CI/MBPT |
| H2O/DZP frozen core | about 28 million | converged FCI extension |
| H2O/cc-pVDZ C2v/A1 | 451,681,246 | converged symmetry-block FCI |
| H2O/cc-pVDZ no symmetry | 1,806,590,016 | bounded benchmark; full FCI not run |

## Pull Request Strategy

Quantum Harness PR #217 remains the sole upstream submission. Its solution
README and reproduction prompt are updated in place; no second challenge PR is
opened. The PR body receives a concise `Final competition update` section that
links one immutable workbench commit and the canonical final report.

Only one final issue comment is posted after the public commit and CI are
available. It states the final 560/1008 CPU status, reports the C2v/A1 result,
and repeats that symmetry-free converged cc-pVDZ FCI and selected CI are not
claimed.

## Release Rule

- Preserve the already published immutable v0.5.0 tag.
- Publish this branch as a clearly labeled corrective evidence supplement;
  merge it later only through normal review.
- Do not create a replacement tag while production provenance is incomplete.
- Never move or overwrite v0.1.x-v0.4.0 tags.

## Completion Criteria

The sprint is complete only when one public commit contains the integrated
code and evidence; all required local gates pass; raw Slurm accounting and
source provenance are either complete or explicitly disclosed; the final
report and article are committed; PR #217 links the final artifacts; and no
public statement confuses symmetry-adapted FCI, symmetry-free bounded
benchmarking, task-parallel concurrency, or selected-CI future work.
