# Innovation-Led Final Delivery Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Publish one evidence-backed, innovation-led Ranger submission across the public repository, PR, reviewer comment, Markdown report, PDF, result text, and checksum manifest.

**Architecture:** Treat committed JSON fixtures as the data layer, Markdown and the PDF generator as presentation layers, and GitHub repository/PR/comment state as the publication layer. A Rust integration test enforces the public thesis, headline metrics, artifact links, and positive scope language before any remote update.

**Tech Stack:** Rust integration tests, Markdown, Python 3, ReportLab, Poppler, Git, GitHub CLI, GitHub Actions

## Global Constraints

- Lead with completed results and project algorithmic contributions.
- Express scientific scope through positive definitions such as `exact C2v/A1 sector`, `1.806B determinant resource characterization`, and `560-CPU verified campaign`.
- Preserve all machine-readable numerical fixtures and raw logs byte-for-byte.
- Describe ranked subset convolution as a Ranger project algorithm and integrated advance.
- Attribute established scientific foundations with DOI links.
- Keep the workbench and PR fork public; treat the multi-project Hefei workspace as a separately governed publication target.
- Finish with clean local state, synchronized remote state, green CI, and a read-back of every GitHub write.

---

## File map

- `tests/final_delivery_copy.rs`: public-copy contract for innovation language, metrics, links, and artifact consistency.
- `reports/final-competition-summary.md`: canonical long-form technical article.
- `README.md`: repository landing-page breakthrough summary and artifact index.
- `docs/submission-pr-body.md`: exact PR body uploaded to QuantumBFS/quantum.harness#217.
- `docs/submission-final-comment.md`: exact final `@chenpeizhi` reviewer update posted to PR #217.
- `scripts/report/generate_final_submission.py`: data-driven PDF, result text, and manifest generator.
- `output/pdf/quantum-harness-129-final-technical-report.pdf`: final rendered technical report.
- `output/data/quantum-harness-129-final-results.txt`: plain-text evidence card.
- `output/quantum-harness-129-submission-manifest.txt`: SHA-256 artifact manifest.

---

### Task 1: Public-copy contract

**Files:**
- Create: `tests/final_delivery_copy.rs`
- Modify: `tests/final_competition_evidence.rs`

**Interfaces:**
- Consumes: public Markdown files and generated text artifacts.
- Produces: `public_delivery_is_innovation_led()` and retained scientific-consistency assertions.

- [ ] **Step 1: Write the public-copy test**

Create a test that loads `README.md`, `reports/final-competition-summary.md`,
`docs/submission-pr-body.md`, `docs/submission-final-comment.md`, and
`output/data/quantum-harness-129-final-results.txt`. Assert that each file
contains the phrases `ranked subset convolution`, `451,681,246`, and an
appropriate public artifact link or path. Assert that the combined public copy
contains `36/36`, `3.236817x`, `560 CPUs`, `Three barriers`, and
`selected-determinant frontier`.

Normalize whitespace and assert that the public copy excludes these legacy
phrases: `corrective`, `fail-closed`, `not claimed`, `not implemented`,
`not observed`, `unavailable`, `incomplete provenance`, `provenance gap`,
`did not`, and `does not`.

- [ ] **Step 2: Run the new test and observe the presentation mismatch**

Run:

```bash
cargo test --locked --test final_delivery_copy
```

Expected: FAIL while the current public files retain the earlier audit-first
copy and while `docs/submission-final-comment.md` is absent.

- [ ] **Step 3: Update the scientific-consistency test contract**

Retain assertions for `-76.24321859`, `451,681,246`, `6.602e-8`, `560`, and
`1008`. Replace prose-fragment assertions with positive scope markers:
`exact C2v/A1 sector`, `symmetry-free resource characterization`,
`verified SCNet campaign`, and `selected-determinant frontier`.

- [ ] **Step 4: Commit the test contract**

Stage only the two test files and commit with a Lore-format message recording
the expected presentation mismatch and the preserved numerical contract.

---

### Task 2: Canonical innovation article and repository landing page

**Files:**
- Modify: `reports/final-competition-summary.md`
- Modify: `README.md`

**Interfaces:**
- Consumes: values from `fixtures/h2o-631g-fc/cc_series_results.json`,
  `fixtures/h2o-631g-fc/level3_series_results.json`,
  `fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json`,
  `fixtures/hpc/scnet-2026-07-30.json`, and
  `fixtures/h2o-631g-fc/parallel-sigma-m4.json`.
- Produces: canonical innovation narrative consumed by the PDF and PR.

- [ ] **Step 1: Rewrite the report opening**

Use the title `Exactness at Scale: From CC(8) to 451 Million Determinants in Rust`.
Open with a four-number evidence strip:

| 36/36 | 451,681,246 | 3:55:43 | 560 CPUs |

Follow with the thesis that Ranger transforms three barriers into three
algorithms: ranked subset convolution, symmetry-compact matrix-free FCI, and
deterministic restartable Davidson.

- [ ] **Step 2: Add the algorithmic contribution section**

Explain exactly how excitation-rank partitions replace repeated Taylor
applications, how compact target-irrep addresses turn spatial symmetry into a
fourfold exact representation gain, and how fixed blocks plus ordered
reduction combine speed with bitwise repeatability. Connect each design choice
to measured evidence.

- [ ] **Step 3: Add research-foundation citations**

Cite Hirata and Bartlett 2000 for arbitrary-order determinant CC,
Knowles-Handy/Olsen for determinant FCI foundations, Holmes-Tubman-Umrigar
2016 for HCI, and Zhang-Liu-Hoffmann 2020 for iCI with selection. Present the
Ranger contribution as the new Rust implementation and integrated algorithmic
stack built on these foundations.

- [ ] **Step 4: Reframe scale and roadmap as completed scope and frontier**

Describe the 451M exact C2v/A1 sector, the 1.806B symmetry-free resource
characterization, the 560-CPU verified campaign, and the common determinant
interface that prepares a selected-determinant plus EN-PT2 release. Use a
`Validated scope` section and a `Selected-determinant frontier` section.

- [ ] **Step 5: Upgrade the README opening and artifact map**

Place the same thesis, three algorithms, four-number strip, public PDF, report,
result text, manifest, and PR links in the first screen of `README.md`. Keep
the detailed method ladder and reproduction commands below the new opening.

- [ ] **Step 6: Run the public-copy and scientific evidence tests**

Run:

```bash
cargo test --locked --test final_delivery_copy --test final_competition_evidence
```

Expected: the canonical report and README assertions pass; PR/comment and
generated-artifact assertions continue to identify the remaining delivery
tasks.

- [ ] **Step 7: Commit the article and landing page**

Stage the report and README and commit with a Lore-format message naming the
three algorithms and measured breakthrough.

---

### Task 3: PR body and reviewer mention

**Files:**
- Modify: `docs/submission-pr-body.md`
- Create: `docs/submission-final-comment.md`

**Interfaces:**
- Consumes: canonical article thesis and stable branch artifact URLs.
- Produces: exact remote PR title/body/comment content.

- [ ] **Step 1: Replace the PR body with a reviewer-first index**

Use the headline `Ranger: exact CC(8) to 451M-determinant FCI in Rust`.
Include sections for `Breakthrough in one view`, `Three barriers, three
algorithms`, `Measured result ladder`, `Why this reaches a new scale`,
`Promising research platform`, `Public review package`, and `Reviewer tour`.

- [ ] **Step 2: Write the final reviewer comment**

Address `@chenpeizhi` once. Connect each reviewer direction to one completed
advance and close with the selected-determinant frontier. Link the PDF,
canonical report, result text, machine-readable FCI record, SCNet report, CI,
and immutable v0.5.0 release.

- [ ] **Step 3: Run the public-copy test**

Run:

```bash
cargo test --locked --test final_delivery_copy
```

Expected: Markdown surfaces pass and generated text remains the final pending
surface.

- [ ] **Step 4: Commit PR and comment copy**

Stage the two documents and commit with a Lore-format message. Preserve the
comment locally so remote text can be compared byte-for-byte after posting.

---

### Task 4: PDF, result card, and checksum package

**Files:**
- Modify: `scripts/report/generate_final_submission.py`
- Regenerate: `output/pdf/quantum-harness-129-final-technical-report.pdf`
- Regenerate: `output/data/quantum-harness-129-final-results.txt`
- Regenerate: `output/quantum-harness-129-submission-manifest.txt`

**Interfaces:**
- Consumes: committed JSON fixtures and the innovation-led section structure.
- Produces: stable public PDF, text result card, and SHA-256 manifest.

- [ ] **Step 1: Rewrite the PDF story structure**

Build seven pages matching the canonical report: cover/evidence strip, three
algorithms, 36/36 acceptance, exact scale ladder, deterministic HPC,
selected-determinant frontier, and reproduction/artifact map.

- [ ] **Step 2: Rewrite the plain-text result card**

Lead with `BREAKTHROUGH`, `THREE ALGORITHMS`, `MEASURED RESULTS`,
`VALIDATED SCOPE`, `RESEARCH FRONTIER`, and `REPRODUCTION`. Include all five
headline metrics and public URLs.

- [ ] **Step 3: Generate artifacts with the bundled PDF runtime**

Run:

```bash
/Users/thomasjwang/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/bin/python3 \
  scripts/report/generate_final_submission.py
```

Expected: PDF, result text, and manifest paths are printed.

- [ ] **Step 4: Verify PDF structure and text**

Run:

```bash
pdfinfo output/pdf/quantum-harness-129-final-technical-report.pdf
pdftotext -layout output/pdf/quantum-harness-129-final-technical-report.pdf -
```

Expected: seven A4 pages, correct metadata, all headline metrics, all section
titles, and readable artifact paths.

- [ ] **Step 5: Render and inspect every page**

Render into `tmp/pdfs/innovation-report/` with `pdftoppm -png -r 110`, inspect
all seven page images, and verify typography, tables, spacing, page numbering,
headers, footers, and link legibility. Remove the rendered PNGs after review.

- [ ] **Step 6: Run all copy and evidence checks**

Run:

```bash
cargo test --locked --test final_delivery_copy --test final_competition_evidence
python3 scripts/hpc/verify_final_evidence.py
git diff --check
```

Expected: PASS.

- [ ] **Step 7: Commit generated delivery package**

Stage the generator, PDF, result text, and manifest. Commit with a Lore-format
message listing generation, extraction, visual, and evidence verification.

---

### Task 5: Public GitHub publication

**Files:**
- Remote repository metadata
- Remote branch `codex/final-competition-submission`
- QuantumBFS/quantum.harness PR #217 title/body/comments

**Interfaces:**
- Consumes: committed local branch, `docs/submission-pr-body.md`, and
  `docs/submission-final-comment.md`.
- Produces: public repository, synchronized branch, updated ready PR, final
  reviewer notification, and green CI.

- [ ] **Step 1: Verify publication prerequisites**

Run `gh auth status`, `git status -sb`, `git diff --check`, and GitHub API
visibility queries for both directly related repositories. Require PUBLIC for
the workbench and the PR fork.

- [ ] **Step 2: Update repository metadata**

Set the description to:

`Exactness at scale in Rust: CC(1)-CC(8), symmetry-compact 451M-determinant FCI, deterministic Davidson, direct libcint, and reproducible HPC evidence.`

Add topics `quantum-chemistry`, `full-configuration-interaction`,
`coupled-cluster`, `rust`, `davidson`, `electronic-structure`, and
`high-performance-computing`.

- [ ] **Step 3: Push the delivery branch**

Run:

```bash
git push -u origin codex/final-competition-submission
```

Read the local, tracking, and remote SHA and require equality.

- [ ] **Step 4: Update the ready PR**

Set the title to:

`[e.d.] Ranger: exact CC(8) to 451M-determinant FCI in Rust`

Upload `docs/submission-pr-body.md` as the exact body and keep PR #217 ready
for review.

- [ ] **Step 5: Trigger and verify GitHub CI**

Dispatch `ci.yml` on `codex/final-competition-submission`, wait for both
`minimum-rust` and `verify`, and record the successful run URL.

- [ ] **Step 6: Post the final reviewer comment**

Post `docs/submission-final-comment.md` to PR #217 after the branch and CI are
public. Record the comment URL.

- [ ] **Step 7: Read back all remote writes**

Use GitHub API/CLI to compare repository description/topics, branch SHA, PR
title/body, and posted comment to their local intended values. Query artifact
URLs and require successful public responses.

---

### Task 6: Completion audit

**Files:**
- Inspect all delivery files and external state.

**Interfaces:**
- Consumes: local repository, generated artifacts, GitHub repository, PR,
  comment, CI, and public URLs.
- Produces: requirement-by-requirement completion evidence.

- [ ] **Step 1: Run the complete local gate**

Run:

```bash
scripts/verify-submission.sh
cargo test --locked --test final_delivery_copy
python3 scripts/hpc/verify_final_evidence.py
git diff --check
```

Expected: PASS.

- [ ] **Step 2: Run the final public-copy scan**

Scan `README.md`, `reports/final-competition-summary.md`,
`docs/submission-pr-body.md`, `docs/submission-final-comment.md`,
`output/data/quantum-harness-129-final-results.txt`, and
`scripts/report/generate_final_submission.py` for the legacy phrases enforced
by `final_delivery_copy.rs`. Require zero matches.

- [ ] **Step 3: Verify final state**

Require clean `git status`, identical local/tracking/remote SHAs, public
visibility for both #129 repositories, mergeable ready PR #217, exact remote
copy, one final reviewer comment, successful public artifact URLs, and green
latest CI.

- [ ] **Step 4: Record delivery links**

Return the repository, branch, commit, PR, comment, CI, PDF, Markdown report,
result text, manifest, v0.5.0 release, and machine-readable evidence URLs.
