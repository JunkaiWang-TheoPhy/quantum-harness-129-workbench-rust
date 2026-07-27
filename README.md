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

## License

This repository is licensed under the GNU Affero General Public License v3.0.
