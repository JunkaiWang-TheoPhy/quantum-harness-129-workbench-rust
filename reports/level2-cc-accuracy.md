# Level 2 Arbitrary-Order CC Report

Date: 2026-07-27

## Results

| System | Method | Rust energy (Hartree) | Reference (Hartree) | Absolute error (Hartree) | Iterations | Final residual |
|---|---|---:|---:|---:|---:|---:|
| Equilibrium H2/STO-3G, H-H = 0.7414 Å | CC(2) | -1.137270174665275 | PySCF CCSD -1.137270174666663 | 1.388 × 10⁻¹² | 12 | 3.899 × 10⁻¹¹ |
| Stretched H2/STO-3G, H-H = 1.4 Å | CC(2) | -1.0154682493 | PySCF CCSD / FCI | < 1e-9 | tested | < 1e-9 |
| Linear H4/STO-3G | CC(2) | -2.166379520346392 | PySCF CCSD -2.166379520332999 | 1.339e-11 | 12 | 6.389e-10 |
| Linear H4/STO-3G | CC(4) | -2.166387448640237 | PySCF FCI -2.166387448634763 | 5.474e-12 | 16 | 6.440e-11 |
| H2O/STO-3G | CC(2) | -75.012790405014059 | PySCF CCSD -75.012790405040 | < 3e-11 | 13 | 2.499e-9 |

CC(4) on four-electron H4 is full-rank coupled cluster and reaches the FCI
limit. The same compiled solver accepts any rank supported by the finite
electron/basis space; there are no separately hard-coded CCSD/CCSDT equations.

## Implementation

Each amplitude corresponds to one determinant connected to the Hartree–Fock
reference. Its excitation operator is phase-normalized so:

```text
tau_mu |HF> = |mu>
```

The cluster operator applies these substitutions to any full-FCI vector.
`exp(T)|HF>` is accumulated by `T^k/k!` and terminates at the electron-count
ceiling. The solver evaluates:

```text
E = <HF|H exp(T)|HF>
R_mu = <mu|(H-E)exp(T)|HF>
```

Amplitudes receive orbital-denominator Jacobi updates followed by DIIS.

## Reproduction

```bash
cargo test --test level2

cargo run --release -- cc \
  fixtures/h2-equilibrium-sto3g/FCIDUMP \
  fixtures/h2-equilibrium-sto3g/reference.json \
  --rank 2 --residual-tolerance 1e-9

cargo run --release -- cc \
  fixtures/h4-sto3g/FCIDUMP fixtures/h4-sto3g/reference.json \
  --rank 2 --residual-tolerance 1e-8

cargo run --release -- cc \
  fixtures/h4-sto3g/FCIDUMP fixtures/h4-sto3g/reference.json \
  --rank 4 --residual-tolerance 1e-8
```
