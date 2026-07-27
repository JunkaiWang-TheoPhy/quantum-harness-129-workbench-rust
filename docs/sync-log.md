# Sync Log

## 2026-07-27

Created private repository:

- Repository: `JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust`
- Visibility: private
- License: AGPL-3.0
- Local path: `/Users/thomasjwang/Documents/GitHub/quantum-harness-129-ed-workbench-rust`

Initialized Rust scaffold:

- `Cargo.toml`
- `Cargo.lock`
- `src/main.rs`
- `README.md`

Created public Quantum Harness registration:

- PR: https://github.com/QuantumBFS/quantum.harness/pull/210
- Branch: `challenge/ed-wangtheophys-rust-workbench`
- Team folder: `tracks/ed/solutions/WangTheoPhys/`
- Registered team: Rewrite It In Rust! (RIIR 2607 Hefei)
- Members: Chenxi Wan, Yedi Shen, Junkai Wang
- Solution directory identifier: WangTheoPhys
- Catalog issue: https://github.com/QuantumBFS/quantum.harness/issues/129

Synced useful upstream context into this repository:

- Challenge metadata and deliverables in `docs/challenge-129-brief.md`.
- Implementation milestones in `docs/implementation-roadmap.md`.
- GitHub, tool, and literature resource index in `docs/resources.md`.

Second-pass web and GitHub sync:

- Re-read the complete upstream issue through the GitHub API.
- Captured PR #210 state, commits, changed file, branch, registered team, and
  mergeability.
- Corrected the registered team to `Rewrite It In Rust! (RIIR 2607 Hefei)` with
  members Chenxi Wan, Yedi Shen, and Junkai Wang. `WangTheoPhys` remains the
  solution-directory identifier.
- Added #114/#115 only as narrow context for #129's required tenferro-rs gap
  reporting.
- Added current repository metadata and release discovery for PySCF, libcint,
  Psi4NumPy, tenferro-rs, Quantum Package, faer, and argmin.
- Added official PySCF, libcint-crate, and tenferro documentation entry points.
- Added `docs/web-and-github-snapshot.md` and
  `docs/reproducibility-notes.md`.
- Expanded `docs/upstream-metadata.json` with source timestamps and states.

Completed the Level 0 tiny-system oracle loop:

- Added a pinned PySCF 2.14.0 generator under `scripts/oracle/`.
- Committed deterministic H2 and linear H4/STO-3G FCIDUMP/reference fixtures.
- Added Rust FCIDUMP, determinant, dense Hamiltonian, eigensolver, reference,
  and CLI modules.
- Verified Rust dense FCI against PySCF to `0.0` hartree for H2 and
  `6.217e-15` hartree for H4.
- Added checksum verification, CLI regression tests, and
  `reports/level0-accuracy.md`.

Completed Levels 1-4:

- Added matrix-free spin-free direct FCI and Davidson, including the
  245,025-determinant frozen-core H2O/6-31G target.
- Added arbitrary-rank determinant CC(n), CI(n), MBPT(n), and unitary CC(n).
- Added the direct Rust `libcint -> RHF/DIIS -> AO-to-MO -> FCI` pipeline for
  H2 and H2O/STO-3G.
- Added complete AO/MO integral fixtures and element-by-element PySCF
  comparisons.
- Re-checked the current tenferro-rs 0.2.0 repository, tensor API, indexing
  trait, and memory-order documentation.
- Recorded the final indexed scatter-add, mutable/output-buffer, BLAS-1,
  layout, numerical, performance, and API findings in
  `reports/tenferro-gap-list.md`.
