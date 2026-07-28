> Do not go gentle into that good night,<br>
> Old age should burn and rave at close of day;<br>
> Rage, rage against the dying of the light.<br>
> —— [Dylan Thomas](https://www.poetryfoundation.org/poets/dylan-thomas), 「**Do Not Go Gentle into That Good Night**」

![Rager — determinant states around a gravitationally lensed accretion disk](https://raw.githubusercontent.com/JunkaiWang-TheoPhy/quantum.harness/refs/heads/media/rager-pr-banners/assets/rager/pr-217-ed-fci-accretion-states.png)

# Rager — completed #129 submission

## Team

| Field | Value |
|---|---|
| Team | Rager |
| Members | Chenxi Wan, Yedi Shen, Junkai Wang |
| Challenge | [#129 — Exact diagonalization workbench in Rust for electronic structure method development](https://github.com/QuantumBFS/quantum.harness/issues/129) |
| Public workbench | [`JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust`](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust) |
| Release | [`v0.1.0`](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/releases/tag/v0.1.0) |
| License | AGPL-3.0 |

This PR contains the completed public submission, not a development
registration. Python/PySCF is used to construct and audit independent oracle
fixtures; the checked FCI, CC, CI, MBPT, UCC, RHF, and direct-integral
production paths are Rust.

## Design delivered

- **Level 0:** PySCF oracle generation, FCIDUMP parser, alpha/beta determinant
  basis, fermionic signs, tiny dense Hamiltonians, and dense FCI.
- **Level 1:** signed string links, matrix-free spin-free sigma contraction,
  independent diagonal, and restarted Davidson.
- **Level 2:** arbitrary-order determinant CC(n), exact ranked
  subset-convolution `exp(T)|HF>`, Taylor-oracle coefficient checks,
  denominator updates, DIIS, and rank warm starts.
- **Level 3:** warm-started CI(n), recursive MBPT(n), and variational UCC(n).
- **Level 4:** direct libcint AO integrals, Rust RHF/DIIS, staged AO-to-MO
  transformation, and shared direct FCI.

## Primary acceptance

The primary H2O/6-31G Hamiltonian freezes the oxygen 1s orbital and uses
`R(O-H)=0.967 Å`, `angle(H-O-H)=107.6°`, 12 active spatial orbitals, 8 active
electrons, and 245,025 determinants.

- Matrix-free FCI: `-76.121174204141980 E_h`, residual `5.044e-8`.
- CC(1)-CC(8): all 8 equilibrium CC differences match Hirata and Bartlett
  2000 Table 2 at its six printed decimal places.
- CC(2), meaning CCSD here: `-76.119629519205702 E_h`, only `3.025e-10 E_h`
  from the independent PySCF CCSD oracle.
- CC(8): `-76.121174196144139 E_h`, within `7.998e-9 E_h` of FCI.
- CI(1)-CI(8): all 8 Table 2 entries match; CI(8) is
  `-76.121174204143969 E_h`, within `2.004e-12 E_h` of FCI.
- MBPT(1)-MBPT(20): all 20 Table 2 partial-sum entries match.

Together, the submission matches all 36 equilibrium CI, MBPT, and CC entries
printed in Hirata 2000 Table 2. Comparison respects the paper's six-decimal
precision rather than inventing unprinted digits.

## Evidence and reproduction

- [Standalone reproduction prompt](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/v0.1.0/docs/reproduction-prompt.md)
- [FCI report](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/v0.1.0/reports/level1-direct-fci.md)
- [CC(1)-CC(8) report](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/v0.1.0/reports/level2-cc-accuracy.md)
- [CI/MBPT/UCC report](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/v0.1.0/reports/level3-methods.md)
- [Direct-integral report](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/v0.1.0/reports/level4-integrals.md)
- [Machine-readable CC results](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/v0.1.0/fixtures/h2o-631g-fc/cc_series_results.json)
- [Machine-readable CI/MBPT results](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/v0.1.0/fixtures/h2o-631g-fc/level3_series_results.json)
- [Continuous verification](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/actions/workflows/ci.yml)

Normal local acceptance is one command after installing the pinned oracle
environment:

```bash
scripts/verify-submission.sh
```

The public repository also provides a manual `Primary live acceptance`
workflow for complete CC(1)-CC(8) and CI(1)-CI(8)/MBPT(1)-MBPT(20)
recalculations.

## tenferro-rs findings

The audited tenferro-rs 0.2.0 surface already covers dense tensors, strided
views, gather/scatter, element-wise division, reductions, and contractions.
The primary determinant-workload gap is collision-reducing indexed
scatter-add with explicit deterministic semantics. The complete gap and
reproducer list is in the
[tenferro report](https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust/blob/v0.1.0/reports/tenferro-gap-list.md).

## Scope boundary

The mandatory H2O/6-31G frozen-core Level 0-2 path is complete; all three
Level 3 families and Level 4 were also implemented. Kállay 2001 DZ/DZP
calculations are extended targets and are not claimed in this release.

## Reviewer checklist

- [ ] Inspect the Level 0-4 architecture and design decisions in this solution
  README.
- [ ] Confirm the standalone reproduction prompt is present.
- [ ] Confirm the public `v0.1.0` source and FCIDUMP checksums resolve.
- [ ] Confirm the normal CI workflow is green.
- [ ] Review the primary FCI/CC/CI/MBPT tables and tenferro-rs gap list.
