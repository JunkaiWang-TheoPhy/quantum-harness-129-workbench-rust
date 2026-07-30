@chenpeizhi Your review directions became the roadmap for Ranger's strongest
algorithmic advances. The complete innovation-led delivery is now assembled as
one public, reproducible research package.

## From review direction to delivered advance

1. **All-electron H2O/cc-pVDZ benchmark** -> a
   **1,806,590,016-determinant symmetry-free resource characterization** plus a
   completed **451,681,246-determinant exact C2v/A1 FCI** calculation at
   **`-76.24321859 Eh`**, residual `6.602e-8`, 21 iterations, and `3:55:43`.
2. **Push the implementation on HPC** -> deterministic fixed-block sigma,
   restartable Davidson, **18/18** robustness cases, **216/216** repeated
   solves, and a verified **560-CPU** SCNet campaign.
3. **Study the flexibility-efficiency tradeoff** -> one compact symmetry and
   determinant-address layer shared across FCI, CI, MBPT, CC, and UCC.
4. **Pursue FCI accuracy at reduced cost** -> a
   **selected-determinant frontier** that can reuse the exact solver as its
   calibration oracle for HCI/iCI-style selection, EN-PT2, orbital
   optimization, and quantum-sampled determinant lists.

## Ranger's key project algorithm

The coupled-cluster exponential now uses **exact ranked subset convolution**.
Instead of repeated wave-function applications, each excitation-rank layer is
assembled directly from compatible amplitude/source partitions with exact
fermionic phases. The independent Taylor path remains a coefficient oracle.
This recurrence powers CC(1)-CC(8), while the complete primary result matches
**36/36** Hirata-Bartlett CC/CI/MBPT entries.

Combined with matrix-free FCI, fourfold symmetry-compact addressing,
deterministic ordered reduction, and restartable storage, this is how Ranger
progresses from the 245,025-determinant challenge target to an exact sector
with **451,681,246 determinants**.

## Public review package

- [Final technical PDF](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/output/pdf/quantum-harness-129-final-technical-report.pdf)
- [Innovation-led technical article](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/reports/final-competition-summary.md)
- [Plain-text result card](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/output/data/quantum-harness-129-final-results.txt)
- [451M FCI machine record](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json)
- [SCNet campaign](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/codex/final-competition-submission/reports/scnet-hpc-benchmark.md)
- [v0.5.0 release](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/releases/tag/v0.5.0)
- [Green acceptance run](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/actions/runs/30537439032)

Thank you for pushing the project toward scale, efficiency, and a broader
research vision. Ranger now offers both a completed exact-method benchmark and
a promising foundation for the selected-determinant methods that follow.
