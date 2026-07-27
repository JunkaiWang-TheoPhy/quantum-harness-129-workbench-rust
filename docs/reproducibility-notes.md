# Reproducibility Notes

This document converts the upstream web material into implementation checks.
It does not replace the cited papers or official API documentation.

## Primary Molecular Target

- Molecule: H2O.
- Geometry: `r_OH = 0.967 Angstrom`, `HOH = 107.6 degrees`.
- Basis: 6-31G.
- Orbitals: restricted Hartree-Fock canonical molecular orbitals.
- Frozen core: oxygen 1s.
- Expected FCI energy: `-76.121174 hartree`.

Extended targets:

- Water/DZ, all electrons: `-76.156699 hartree`.
- Water/DZP, frozen oxygen 1s: `-76.256624 hartree`.
- Reproduce the geometry and basis definitions from Bauschlicher 1986 rather
  than substituting similarly named modern basis-library entries without
  comparison.

## Oracle Generation

The upstream recommended path is:

1. Run restricted Hartree-Fock in PySCF.
2. Export molecular-orbital integrals with
   `pyscf.tools.fcidump.from_scf(mf, "FCIDUMP")`.
3. Compute and store HF, FCI, and CCSD reference energies as JSON.
4. Record geometry, basis text/name, charge, spin, frozen orbitals, PySCF
   version, convergence thresholds, and integral checksum alongside every
   fixture.
5. Keep a comparison script that fails when Rust results exceed stated
   tolerances.

Official FCIDUMP documentation says `from_scf` transforms the one- and
two-electron integrals using the supplied SCF orbitals. `read` returns at least
`H1`, `H2`, `ECORE`, `NORB`, `NELEC`, `MS`, `ORBSYM`, and `ISYM`.

## FCIDUMP Parser Contract

Tests should cover these facts explicitly:

- Input orbital indices are one-based; internal indices should be zero-based.
- `k != 0` denotes a two-electron-integral record.
- `k == 0 && j != 0` denotes a one-electron-integral record.
- `k == 0 && j == 0` denotes the core/nuclear energy.
- Two-electron integrals use spatial-orbital Mulliken ordering with eightfold
  permutation symmetry.
- The header includes electron/orbital/spin and optional symmetry metadata.
- PySCF's writer defaults visible in its source are a `1e-15` write threshold
  and `%.16g`-style numeric formatting; fixtures should record any override.

## Determinant and Hamiltonian Verification

- Represent alpha and beta strings independently.
- Lexically rank occupied-orbital combinations without gaps and implement the
  inverse unranking operation.
- Exhaustively compare every H2/H4 matrix element and fermionic sign against a
  dense reference before moving to water.
- Confirm Hermiticity and diagonal elements independently.
- Compare dense `H C` and direct sigma-vector results for random vectors.

## Davidson Checks

- Start with the ground state and diagonal preconditioning.
- Track Ritz energy, residual norm, subspace size, restart count, and
  orthogonality.
- Verify the iterative result against the tiny dense eigensolver.
- Do not claim a published-energy match unless the residual and energy
  tolerances are both reported.

## CC(n) Checks

- Keep amplitudes through excitation level `n`.
- Apply `T` using the same determinant/string machinery.
- Accumulate `exp(T)|HF>` by Taylor series and stop only after the next term's
  norm is below the documented threshold; also exploit the exact finite
  excitation ceiling.
- Evaluate the energy and projected residuals on determinants through level
  `n`.
- Use orbital-energy denominators for Jacobi updates and DIIS acceleration.
- First verify CC(2) against PySCF CCSD, then compare the full CC(n) series to
  Hirata 2000 Table 2.
- Use residual norm `< 1e-6` as the published baseline convergence criterion;
  tighter internal tests are encouraged.

## tenferro-rs Evaluation

The dense GEMM-like integral-contraction block may fit tenferro. The
determinant-driven kernels also require operations that deserve explicit
evaluation:

- indexed gather/scatter-add with sign changes;
- mutable sliced views;
- element-wise denominator division;
- in-place axpy, dot, and norm operations;
- predictable column/row-major conversion;
- small-operation dispatch overhead;
- CPU parallelism and reproducible reductions.

Check the current
[supported-operation inventory](https://tensor4all.org/tenferro-rs/design/supported-ops.html)
before calling something missing. Every reported gap should include a minimal
reproducer, sizes, hardware/software versions, expected behavior, observed
behavior, and—when performance-related—a comparison with a reference backend.

## Minimum Provenance Record

Every published result should carry:

- git commit;
- Rust compiler and Cargo.lock;
- Python and PySCF versions;
- OS, CPU, thread count, and relevant BLAS/backend;
- complete molecular input and frozen-orbital definition;
- fixture/integral checksum;
- algorithm thresholds and iteration limits;
- final energy, residual norm, iteration count, and wall time;
- source table/DOI when comparing with literature.
