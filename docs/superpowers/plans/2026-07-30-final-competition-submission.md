# Final Competition Submission Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Produce one public, tested, evidence-complete final Quantum Harness #129 competition artifact from the solver-optimization and SCNet benchmark branches, then update PR #217 and the final technical article without overstating any result.

**Architecture:** Create a dedicated integration branch from `origin/codex/challenge-129-optimizations`, merge the SCNet evidence branch and the now-public immutable v0.5.0 history, normalize production-provenance disclosures, and drive all public claims from tested machine-readable artifacts. Publish the branch as a corrective evidence supplement without moving the v0.5.0 tag.

**Tech Stack:** Rust 1.89, Cargo, Rayon, nalgebra, serde/serde_json, Python 3.12 with uv 0.11.32/PySCF 2.14.0, Slurm `sacct`, Git/GitHub Actions, Markdown.

## Final Audit Deviation

The available local SSH identities can reach XH5 but cannot read the
`qbics2622` production directory or obtain jobs 23008083/23015354 from
`sacct`. Therefore the two planned raw accounting files are intentionally not
created. Task 2 is resolved by explicit unavailability fields and a
fail-closed verifier: the transcribed MaxRSS is unverified and the 1,008-CPU
request remains unobserved. This follows the approved design's third
provenance option and avoids fabricating evidence.

## Global Constraints

- Competition sprint duration is approximately five hours on 2026-07-30.
- Do not implement HCI/iCI, EN-PT2, natural orbitals, or orbital optimization in this sprint.
- Do not claim converged H2O/cc-pVDZ symmetry-free full FCI.
- Do not call task-parallel ensemble concurrency MPI or single-solve scaling.
- Do not claim 1,008 observed CPUs until the 72-worker/216-result fail-closed gate passes.
- Publicly report the C2v/A1 energy as `-76.24321859 Eh` with residual `6.602e-8`.
- Preserve all v0.1.x-v0.4.0 tags and the v0.4.0 numerical acceptance.
- Preserve the already published immutable v0.5.0 tag; do not retag it.
- Keep every persistent HPC file inside the authorized project directories.
- Never delete or overwrite an existing immutable HPC run directory.

---

## File Map

### Create

- `reports/final-competition-summary.md`: canonical competition article and claim table.
- `scripts/hpc/verify_final_evidence.py`: raw accounting, hashes, job state, and claim-boundary validator.
- `tests/final_competition_evidence.rs`: Rust regression over the final machine-readable evidence and human-facing precision.
- `docs/plans/2026-07-30-final-competition-submission-design.md`: approved final-sprint design.
- `docs/superpowers/plans/2026-07-30-final-competition-submission.md`: this implementation plan.

### Modify

- `fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json`: link raw accounting and distinguish stored precision from reported precision.
- `reports/h2o-ccpvdz-c2v-fci.md`: correct provenance, precision, independent-check language, and no-symmetry boundary.
- `reports/scnet-hpc-benchmark.md`: replace pending/completed gang status from raw evidence only.
- `tests/ccpvdz_fci_result.rs`: use numerical and provenance assertions rather than long-string equality as scientific validation.
- `scripts/verify-submission.sh`: invoke final evidence verification.
- `README.md`: add one concise final competition results entry and canonical report link.
- `docs/reproduction-prompt.md`: include final branch/commit, C2v reproduction scope, and raw-evidence verification.
- `docs/submission-pr-body.md`: add the final competition update for upstream PR #217.

---

### Task 1: Freeze the Approved Design and Integration Branch

**Files:**
- Create: `docs/plans/2026-07-30-final-competition-submission-design.md`
- Create: `docs/superpowers/plans/2026-07-30-final-competition-submission.md`

**Interfaces:**
- Consumes: `origin/codex/challenge-129-optimizations`, `origin/codex/scnet-hpc-benchmark`.
- Produces: branch `codex/final-competition-submission` containing both histories.

- [ ] **Step 1: Confirm the current worktree is clean**

Run:

```bash
git status --porcelain=v1 -b
```

Expected: branch header only, with no modified or untracked files other than the two approved plan documents before their commit.

- [ ] **Step 2: Commit the approved design and plan on the SCNet branch**

Run:

```bash
git add docs/plans/2026-07-30-final-competition-submission-design.md docs/superpowers/plans/2026-07-30-final-competition-submission.md
git commit -m "docs: plan final competition submission"
```

Expected: one commit containing only the two planning documents.

- [ ] **Step 3: Create the integration branch from the optimization branch**

Run:

```bash
git switch -c codex/final-competition-submission origin/codex/challenge-129-optimizations
```

Expected: `HEAD` points to `codex/final-competition-submission` at optimization commit `1b31b50` or its current successor.

- [ ] **Step 4: Merge the SCNet evidence branch**

Run:

```bash
git merge --no-ff codex/scnet-hpc-benchmark -m "merge: integrate SCNet competition evidence"
```

Expected: both solver-extension and SCNet files exist; any conflict is resolved by preserving optimization code and the newest evidence/report wording.

- [ ] **Step 5: Record the integration identities**

Run:

```bash
git log -1 --format='%H'
git merge-base --is-ancestor origin/codex/challenge-129-optimizations HEAD
git merge-base --is-ancestor origin/codex/scnet-hpc-benchmark HEAD
```

Expected: both ancestry checks exit zero.

### Task 2: Retrieve and Preserve HPC Accounting

**Files:**
- Create: `fixtures/h2o-ccpvdz-ae/xh5/production-23008083.sacct`
- Create: `fixtures/hpc/scnet-23015354.sacct`

**Interfaces:**
- Consumes: Slurm jobs `23008083` and `23015354` from their authorized clusters.
- Produces: unchanged pipe-delimited accounting files used by the evidence verifier.

- [ ] **Step 1: Query the completed C2v/A1 production job**

Run on the XH5 scheduler:

```bash
sacct -P -j 23008083 --starttime 2026-07-29 --format=JobIDRaw,JobName,State,ExitCode,NodeList,AllocCPUS,Elapsed,TotalCPU,MaxRSS,AveRSS,Start,End
```

Expected: a completed batch/step record with exit `0:0`, node `a02r03n08`, elapsed near `03:55:43`, and a non-empty MaxRSS record for the actual compute step.

- [ ] **Step 2: Save the exact command output locally**

Write the exact stdout bytes to `fixtures/h2o-ccpvdz-ae/xh5/production-23008083.sacct` without reformatting columns or units.

- [ ] **Step 3: Query the 1,008-CPU gang job**

Run on SCNet:

```bash
sacct -P -j 23015354 --starttime 2026-07-30 --format=JobIDRaw,JobName,State,ExitCode,NodeList,AllocCPUS,Elapsed,TotalCPU,MaxRSS,AveRSS,Start,End
```

Expected: raw accounting that proves either pending/no-run state or a completed allocation. Do not infer observation from the submission script.

- [ ] **Step 4: Save the gang accounting locally**

Write the exact stdout bytes to `fixtures/hpc/scnet-23015354.sacct`.

- [ ] **Step 5: Hash both raw files**

Run:

```bash
shasum -a 256 fixtures/h2o-ccpvdz-ae/xh5/production-23008083.sacct fixtures/hpc/scnet-23015354.sacct
```

Expected: two non-empty hashes suitable for inclusion in machine-readable result artifacts.

### Task 3: Make Production Provenance Fail Closed

**Files:**
- Modify: `fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json`
- Modify: `reports/h2o-ccpvdz-c2v-fci.md`
- Create: `scripts/hpc/verify_final_evidence.py`
- Modify: `scripts/verify-submission.sh`

**Interfaces:**
- Consumes: production source hashes, binary hash, input hash, raw stdout/stderr, and raw `sacct`.
- Produces: a zero-exit verifier only when provenance and public claims agree.

- [ ] **Step 1: Write a failing verifier fixture check**

The verifier must reject evidence when any of these conditions holds:

```python
required = {
    "job_state": "COMPLETED",
    "exit_code": "0:0",
    "converged": True,
    "max_residual": 1.0e-7,
    "point_group": "C2v",
    "target_irrep": "A1",
    "determinants": 451_681_246,
}
```

It must also reject a claimed `symmetry_free_full_fci=true`, an observed 1,008-CPU claim without complete worker evidence, a missing raw accounting file, or a hash mismatch.

- [ ] **Step 2: Run the verifier and confirm it fails on the current incomplete evidence**

Run:

```bash
python3 scripts/hpc/verify_final_evidence.py
```

Expected: nonzero exit naming the missing raw accounting or source-provenance mismatch.

- [ ] **Step 3: Add explicit reporting fields to the C2v result JSON**

Add values equivalent to:

```json
{
  "result": {
    "total_energy_hartree_text": "-76.243218589558566",
    "reported_total_energy_hartree_text": "-76.24321859",
    "residual_norm": 6.602e-8,
    "converged": true
  },
  "claim_boundary": {
    "symmetry_adapted_full_fci": true,
    "symmetry_free_full_fci": false,
    "independent_same_hamiltonian_fci_oracle": false
  }
}
```

Also add the raw accounting path/hash and make the source status one of `exact-production-source-archived`, `canonical-public-rerun`, or `archived-run-with-disclosed-source-gap`.

- [ ] **Step 4: Implement the verifier**

Parse the JSON and pipe-delimited accounting, recompute file hashes, check the declared job row, and print a compact verified summary. Exit nonzero on missing fields, unsupported claim scope, failed convergence, residual above tolerance, or inconsistent CPU observation.

- [ ] **Step 5: Correct the C2v report language**

Use `-76.24321859 Eh` in the title/abstract/table, describe PySCF CCSD(T) and literature values as hierarchy/scale checks, and state that they are not a same-Hamiltonian independent FCI oracle. Include the no-symmetry 1,806,590,016 determinant boundary beside the C2v/A1 451,681,246 determinant result.

- [ ] **Step 6: Add the verifier to submission validation**

Append a direct invocation of `python3 scripts/hpc/verify_final_evidence.py` to `scripts/verify-submission.sh` after JSON syntax checks.

- [ ] **Step 7: Run the verifier**

Run:

```bash
python3 scripts/hpc/verify_final_evidence.py
```

Expected: zero exit with the job id, determinant count, reported energy, residual, accounting hash, and claim-boundary summary.

- [ ] **Step 8: Commit provenance hardening**

Run:

```bash
git add fixtures/h2o-ccpvdz-ae reports/h2o-ccpvdz-c2v-fci.md scripts/hpc/verify_final_evidence.py scripts/verify-submission.sh
git commit -m "test: make final HPC evidence fail closed"
```

### Task 4: Replace Long-String Validation with Scientific Assertions

**Files:**
- Modify: `tests/ccpvdz_fci_result.rs`
- Create: `tests/final_competition_evidence.rs`

**Interfaces:**
- Consumes: final C2v JSON and SCNet fixture.
- Produces: numerical, provenance, and scope regression gates.

- [ ] **Step 1: Write assertions for supported precision and scope**

Assert:

```rust
assert!(result.converged);
assert!(result.residual_norm <= 1.0e-7);
assert!((result.total_energy_hartree + 76.243_218_589_558_57).abs() < 1.0e-10);
assert_eq!(reported_energy, "-76.24321859");
assert!(claim.symmetry_adapted_full_fci);
assert!(!claim.symmetry_free_full_fci);
assert!(!claim.independent_same_hamiltonian_fci_oracle);
```

Also assert input, stdout, stderr, binary, source-status, and accounting fields are present and their referenced files exist.

- [ ] **Step 2: Run the focused tests**

Run:

```bash
cargo test --locked --test ccpvdz_fci_result --test final_competition_evidence
```

Expected: all tests pass without running the 451-million determinant calculation.

- [ ] **Step 3: Commit the regression gates**

Run:

```bash
git add tests/ccpvdz_fci_result.rs tests/final_competition_evidence.rs
git commit -m "test: enforce final competition claim boundaries"
```

### Task 5: Write the Canonical Final Article

**Files:**
- Create: `reports/final-competition-summary.md`
- Modify: `README.md`
- Modify: `docs/reproduction-prompt.md`

**Interfaces:**
- Consumes: validated Level 0-4 reports, incremental solver report, SCNet report, C2v report, and final evidence JSON.
- Produces: one public narrative and one reproduction entry point.

- [ ] **Step 1: Create the result ladder table**

Include exact scope and status for 245,025, approximately 28 million,
451,681,246, and 1,806,590,016 determinants. The final row must say
`bounded benchmark; converged full FCI not run`.

- [ ] **Step 2: Add the validation section**

Summarize the independent H2/H4/H2O small-system anchors, 36 Hirata Table 2 entries, v0.4.0 live acceptance, symmetry-projected dense/direct tests, and the absence of a same-Hamiltonian independent 451M FCI oracle.

- [ ] **Step 3: Add the HPC and flexibility-efficiency section**

Report 560 observed allocated CPUs as ten simultaneous 56-CPU tasks, about 20.6 effective busy cores per solve, and the final raw-evidence-derived status of job 23015354. Explicitly separate task packing from single-eigenproblem distributed scaling.

- [ ] **Step 4: Add the classical/quantum comparison framework**

Compare methods by full versus selected space, determinant/configuration dimension, variational status, PT2 availability, classical diagonalization role, and reproducibility. Do not compare capability by qubit count alone.

- [ ] **Step 5: Add limitations and future work**

List symmetry-free billion-dimensional FCI, MPI single-solve scaling, HCI/iCI, EN-PT2, natural orbitals, orbital optimization, and quantum-sampled determinant import as uncompleted follow-ups.

- [ ] **Step 6: Add the canonical article link to README**

Place one concise final-results entry near the validation/release section. Preserve existing badges, commands, units, and stable release links.

- [ ] **Step 7: Update the reproduction prompt**

Add commands for `verify_final_evidence.py`, focused Rust evidence tests, and reading the raw Slurm records. State that reproducing the 451M solve requires an HPC node and is not part of normal local acceptance.

- [ ] **Step 8: Commit the article and reproduction update**

Run:

```bash
git add reports/final-competition-summary.md README.md docs/reproduction-prompt.md
git commit -m "docs: publish final competition report"
```

### Task 6: Run the Full Local Acceptance Gate

**Files:**
- Verify only.

**Interfaces:**
- Consumes: integrated branch.
- Produces: local evidence for publication readiness.

- [ ] **Step 1: Check formatting**

Run:

```bash
cargo fmt --check
```

Expected: zero exit.

- [ ] **Step 2: Run Clippy**

Run:

```bash
cargo clippy --locked --all-targets --all-features -- -D warnings
```

Expected: zero exit and no warning.

- [ ] **Step 3: Run all non-ignored tests**

Run:

```bash
cargo test --locked --all-targets
```

Expected: all non-ignored tests pass.

- [ ] **Step 4: Run the submission verifier**

Run:

```bash
scripts/verify-submission.sh
```

Expected: zero exit, including Python unit/geometry tests, JSON checks, FCIDUMP hashes, and final HPC evidence verification.

- [ ] **Step 5: Confirm clean tree and commit identities**

Run:

```bash
git status --porcelain=v1 -b
git log --oneline --decorate -8
```

Expected: clean integration branch with reviewable task-sized commits.

### Task 7: Publish the Workbench Artifact

**Files:**
- Remote branch and optional release.

**Interfaces:**
- Consumes: locally accepted integration commit.
- Produces: public immutable commit and GitHub CI evidence.

- [ ] **Step 1: Push the integration branch**

Run:

```bash
git push -u origin codex/final-competition-submission
```

Expected: public branch URL resolves.

- [ ] **Step 2: Wait for GitHub CI and inspect every required job**

Expected: formatting, Clippy, tests, minimum Rust, and repository-specific verification are green.

- [ ] **Step 3: Apply the release rule**

If provenance is complete and CI is green, merge to `main`, create annotated tag `v0.5.0`, push it, and publish release notes linking the final report. Otherwise leave v0.4.0 as Latest and link the immutable experimental branch commit.

- [ ] **Step 4: Verify remote identities**

Confirm branch/tag commit SHA, repository visibility, public raw report URL, and GitHub Actions result.

### Task 8: Update Quantum Harness PR #217

**Files:**
- Modify in the upstream solution branch: `tracks/ed/solutions/WangTheoPhys/README.md`
- Modify in the upstream solution branch: `tracks/ed/solutions/WangTheoPhys/reproduction-prompt.md`
- Modify local source: `docs/submission-pr-body.md`

**Interfaces:**
- Consumes: public final commit and green CI URL.
- Produces: one current PR body and one final concise comment.

- [ ] **Step 1: Add `Final competition update — July 30` to the PR body source**

Include stable v0.4.0 acceptance, final integration commit, 560/1008 raw-evidence status, 451,681,246 determinant C2v/A1 result at `-76.24321859 Eh`, and the 1,806,590,016 determinant symmetry-free non-claim.

- [ ] **Step 2: Update the upstream solution README and reproduction prompt**

Preserve existing team identity, Level 0-4 results, poetry/banner, and reviewer benchmark. Replace stale primary release/navigation links with the final stable or experimental artifact selected by Task 7.

- [ ] **Step 3: Validate upstream diff scope**

Expected: only the solution README and reproduction prompt change; no unrelated challenge files are modified.

- [ ] **Step 4: Push the upstream solution branch**

Expected: PR #217 updates in place and remains open, mergeable, and non-draft.

- [ ] **Step 5: Update the PR body**

Apply the versioned `docs/submission-pr-body.md` text to PR #217 only after every public link resolves.

- [ ] **Step 6: Post one final concise comment**

The comment must link the immutable workbench commit, final report, raw evidence, and CI. It must state that C2v/A1 is a symmetry-adapted follow-up and that no symmetry-free converged cc-pVDZ FCI or selected-CI implementation is claimed.

- [ ] **Step 7: Read back PR state**

Confirm body, final comment, open/mergeable status, no broken links, and no duplicated or contradictory result claims.

### Task 9: Completion Audit

**Files:**
- Verify all authoritative artifacts.

**Interfaces:**
- Consumes: local tree, public workbench, GitHub CI, PR #217, and HPC evidence.
- Produces: a defensible completion decision.

- [ ] **Step 1: Audit every design completion criterion**

For each criterion, record the exact file, command output, public URL, or raw scheduler record that proves it.

- [ ] **Step 2: Search for contradictory claims**

Run searches for:

```text
1008 CPUs observed
thousand-core FCI
no-symmetry full FCI completed
independent FCI oracle
selected CI implemented
```

Expected: each occurrence is either supported by final evidence or explicitly negated/qualified.

- [ ] **Step 3: Confirm release and branch safety**

Verify no existing tag moved, `main` points to the intended commit only if release gates passed, and the local worktree is clean.

- [ ] **Step 4: Mark the goal complete only after all required evidence is present**

If any production provenance, raw accounting, CI, public-link, article, or PR requirement remains missing, keep the goal active and continue from the failed criterion.
