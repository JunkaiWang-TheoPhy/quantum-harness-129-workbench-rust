# Level 3 CI, MBPT, and Unitary CC Report

Date: 2026-07-27

## CI(n)

Linear H4/STO-3G:

| Method | Energy (hartree) | Davidson iterations | Residual |
|---|---:|---:|---:|
| CI(1) | -2.098545936998035 | 3 | 4.441e-16 |
| CI(2) | -2.165031841780534 | 10 | 2.008e-15 |
| CI(4) | -2.166387448634764 | 11 | 4.306e-10 |

The sequence is variationally non-increasing. CI(4), the full excitation rank
for four electrons, agrees with PySCF FCI to better than `1e-14` hartree.

## MBPT(n)

H2/STO-3G, using the canonical RHF Fock diagonal as `H0`:

| Order | Energy correction | Partial sum |
|---:|---:|---:|
| 1 | 1.110223024625157e-16 | -0.941480654707799 |
| 2 | -3.908905485819417e-2 | -0.980569709565993 |
| 3 | -2.071042716569801e-2 | -1.001280136731691 |
| 4 | -9.772141028901954e-3 | -1.011052277760593 |
| 5 | -3.905112572958080e-3 | -1.014957390333551 |
| 6 | -1.131558429877173e-3 | -1.016088948763428 |

The second-order partial sum matches the committed PySCF MP2 oracle. Higher
orders are reported individually so convergence or divergence is visible.

## Unitary CC(n)

H2/STO-3G UCC(2):

```text
energy: -1.015468249288246 hartree
PySCF FCI: -1.015468249288245 hartree
gradient norm: 1.543e-9
iterations: 4
parameters: 3
```

The implementation applies the anti-Hermitian generator `T-T†`, evaluates its
Taylor exponential action, and minimizes the normalized variational energy
using deterministic BFGS with a line search and finite-difference gradient.

## Commands

```bash
cargo run --release -- ci fixtures/h4-sto3g/FCIDUMP --rank 4
cargo run --release -- mbpt \
  fixtures/h2-sto3g/FCIDUMP fixtures/h2-sto3g/reference.json --order 6
cargo run --release -- ucc fixtures/h2-sto3g/FCIDUMP --rank 2
```

CI and MBPT are practical on the direct-FCI spaces supported by Level 1.
The present UCC implementation forms `T†` transparently and uses numerical
gradients, so it is intentionally a small-system reference rather than a
large-system production optimizer.

