# ED Workbench RS

Private workspace for Quantum Harness challenge
[#129](https://github.com/QuantumBFS/quantum.harness/issues/129):
**Exact diagonalization workbench in Rust for electronic structure method
development**.

The goal is to build a Rust reference implementation for determinant-based
FCI/ED machinery, with arbitrary-order coupled cluster as the mandatory
showcase.

## Scope

- Parse FCIDUMP files generated from PySCF.
- Enumerate alpha/beta determinant strings.
- Build tiny-system dense Hamiltonians for oracle checks.
- Implement string-based direct FCI and Davidson iteration.
- Implement determinant-based arbitrary-order CC(n).
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
- [docs/sync-log.md](docs/sync-log.md) records what was pulled from GitHub and
  how this private repo relates to the public Quantum Harness registration PR.
- [docs/upstream-metadata.json](docs/upstream-metadata.json) is the
  machine-readable counterpart of the dated snapshot.

## Upstream Registration

- Challenge issue: https://github.com/QuantumBFS/quantum.harness/issues/129
- Registration PR: https://github.com/QuantumBFS/quantum.harness/pull/210
- Track folder: `tracks/ed/solutions/WangTheoPhys/`
- Registered team: Rewrite It In Rust! (RIIR 2607 Hefei)
- Members: Chenxi Wan, Yedi Shen, Junkai Wang
- Solution directory identifier: WangTheoPhys

## License

This repository is licensed under the GNU Affero General Public License v3.0.
