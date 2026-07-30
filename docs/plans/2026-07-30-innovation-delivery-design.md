# Innovation-Led Competition Delivery Design

Date: 2026-07-30

## Purpose

Present the Ranger submission as an algorithmic breakthrough whose public
results, source, data, reports, and verification form one coherent research
artifact. The delivery should make the central achievement immediately clear:
one Rust determinant engine progresses from arbitrary-order CC through CC(8)
to an exact 451,681,246-determinant C2v/A1 FCI calculation.

## Public thesis

Ranger turns three coupled scaling barriers into three composable algorithms:

1. **Wave-function construction:** an exact excitation-rank subset-convolution
   recurrence builds `exp(T)|HF>` and makes determinant CC(1)-CC(8) practical.
2. **Hamiltonian scale:** matrix-free spin-free sigma application and compact
   point-group addressing preserve the exact target sector while reducing the
   cc-pVDZ representation from 1,806,590,016 to 451,681,246 determinants.
3. **Reproducible execution:** fixed source blocks, thread-local accumulation,
   ordered reduction, memory preflight, and restartable Davidson storage make
   large runs deterministic, resumable, and auditable.

This thesis explains why the project reaches a scale that the initial
challenge design and the project's first implementation could not reach. The
challenge proposed finite Taylor construction for general-order CC and a
245,025-determinant primary target. The delivered system introduces a faster
exact cluster construction, symmetry-compact FCI, deterministic parallelism,
and storage-aware Davidson, then validates them at 451 million determinants.

## Innovation claim levels

Public wording uses three precise claim levels:

- **Project algorithm:** an algorithm designed and implemented by Ranger for
  this workbench, such as the exact ranked subset-convolution cluster
  construction and fixed-block deterministic sigma policy.
- **Integrated advance:** an original composition of established scientific
  ideas into a unified Rust research engine, such as point-group compact
  addressing propagated across FCI, CI, MBPT, CC, and UCC.
- **Measured breakthrough:** a result demonstrated by committed evidence,
  such as 36/36 published-series matches, a 3.236817x local sigma timing ratio,
  18/18 SCNet robustness cases, 216/216 repeated solves, and the
  451,681,246-determinant FCI energy.

The delivery presents these levels confidently and attributes the scientific
foundations to Hirata and Bartlett, Knowles and Handy, Davidson, HCI, and iCI
where relevant. The ranked subset-convolution implementation is described as
a Ranger algorithmic contribution; broader field-priority language belongs
to a dedicated literature review.

## Positive scope language

All public-facing material leads with completed results and expresses scope as
a positive definition:

- `451M exact C2v/A1 FCI` defines the completed finite-basis ground-state
  sector.
- `1.806B determinant resource characterization` defines the symmetry-free
  benchmark contribution.
- `560-CPU verified SCNet campaign` defines the measured ensemble result.
- `Selected-determinant frontier` defines the next research release enabled
  by the shared determinant interface.
- `Reproducibility profile` identifies the input, code, logs, checksums,
  residual, and verification associated with each published result.

Detailed machine-readable evidence remains unchanged. Promotional prose
draws each statement directly from those records and uses the strongest
precision supported by the corresponding verification.

## Delivery surfaces

### Repository

The public workbench remains the authoritative artifact. Its branch contains
source, fixtures, reports, PDF, result text, checksums, raw logs, and CI.
Repository metadata should lead with CC(8), the 451M exact sector, and
deterministic Rust execution.

### Pull request

The PR opens with a concise breakthrough statement and a four-number evidence
strip: `36/36`, `451,681,246`, `3:55:43`, and `560 CPUs`. It then explains the
three algorithms, the exact result ladder, the research platform, and the
review entry points. The final section invites review through a short
checklist of completed evidence.

### Reviewer comment and mention

One final comment addresses `@chenpeizhi` and maps the review directions to
delivered advances:

- the cc-pVDZ request led to the 1.806B resource characterization;
- the HPC direction led to deterministic parallel sigma and the 560-CPU
  campaign;
- the flexibility-efficiency discussion led to symmetry propagation across
  the entire method stack;
- the selected-CI direction becomes the next algorithmic release backed by an
  exact calibration oracle.

### Technical report and PDF

The Markdown report and seven-page PDF share the same structure:

1. breakthrough summary;
2. three barriers and three algorithms;
3. primary 36/36 acceptance;
4. exact scaling ladder through 451M determinants;
5. deterministic HPC and reproducibility;
6. scientific comparison and selected-determinant frontier;
7. artifact and reproduction map.

The PDF generator remains data-driven. A rendered-page inspection verifies
the final typography, spacing, tables, headers, footers, and links.

## Repository visibility

The two repositories that directly constitute the #129 delivery are already
public:

- `JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust`;
- `JunkaiWang-TheoPhy/quantum.harness`.

The broader `Quantum-Harness-2607-Hefei` workspace contains multiple research
efforts and remains a separately governed publication target. Its visibility
changes after an explicit repository-level authorization. The #129 delivery
already resolves entirely through public URLs.

## Verification

Completion requires all of the following:

- public repository visibility returned by the GitHub API;
- exact PR title and body read back from GitHub;
- one final `@chenpeizhi` comment read back from GitHub;
- public artifact URLs resolving successfully;
- prose scan across the final report, PR body, result text, and PDF source;
- PDF text extraction and visual inspection of every rendered page;
- `scripts/verify-submission.sh` and the final evidence verifier passing;
- clean local worktree, synchronized remote branch, and green GitHub CI.
