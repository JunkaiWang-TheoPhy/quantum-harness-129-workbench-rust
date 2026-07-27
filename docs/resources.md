# Resources

This file collects the useful upstream GitHub and web information currently
known for Challenge #129.

## Quantum Harness

- Challenge issue: https://github.com/QuantumBFS/quantum.harness/issues/129
- Registration PR: https://github.com/QuantumBFS/quantum.harness/pull/210
- Official repository: https://github.com/QuantumBFS/quantum.harness
- Private working repository: https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust

## Software Repositories

| Resource | URL | Relevance |
|---|---|---|
| PySCF | https://github.com/pyscf/pyscf | Oracle generation: RHF, MO integrals, FCIDUMP export, FCI and CCSD references. |
| libcint | https://github.com/sunqm/libcint | Gaussian integral engine underneath PySCF; direct Rust integration is the Level 4 stretch target. |
| Psi4NumPy | https://github.com/psi4/psi4numpy | Readable reference implementations for quantum chemistry methods and verification logic. |
| tenferro-rs | https://github.com/tensor4all/tenferro-rs | Rust-native tensor/autodiff stack to evaluate for dense contractions and gap-list reporting. |
| faer-rs | https://github.com/sarah-quinones/faer-rs | Candidate Rust linear algebra backend when operations do not map to tenferro-rs. |
| argmin | https://github.com/argmin-rs/argmin | Candidate Rust optimizer for unitary CC(n) stretch work. |
| pounce | https://github.com/jkitchin/pounce | Candidate Rust optimization package mentioned by the challenge. |
| Quantum Package | https://github.com/QuantumPackage/qp2 | Modern determinant-driven electronic-structure package for conceptual comparison. |
| MRCC | https://www.mrcc.hu | Production general-order coupled-cluster reference named in the challenge. |

## Primary Literature Cited by the Challenge

| Short name | DOI / URL | Use |
|---|---|---|
| Hirata 2000 | https://doi.org/10.1016/S0009-2614(00)00387-0 | Main CC(n), CI(n), MBPT(n) grading reference; Sec. 2 contains the determinant-based CC recipe. |
| Kallay 2000 | https://doi.org/10.1063/1.481925 | Independent determinant-based general-order CC. |
| Olsen 2000 | https://doi.org/10.1063/1.1290005 | General active-space coupled-cluster implementation. |
| Kallay 2001 | https://doi.org/10.1063/1.1383290 | Extended DZ/DZP grading tables. |
| Knowles 1984 | https://doi.org/10.1016/0009-2614(84)85513-X | Determinant-based FCI and lexical string addressing. |
| Olsen 1988 | https://doi.org/10.1063/1.455063 | Alpha/beta string factorization and determinant CI algorithms. |
| Handy 1980 | https://doi.org/10.1016/0009-2614(80)85158-X | Origin of alpha/beta string factorization. |
| Knowles 1989 | https://doi.org/10.1016/0010-4655(89)90033-7 | Detailed determinant-based FCI program reference. |
| Walter 1963 | https://doi.org/10.1145/366246.366260 | Combination ranking for string-to-index addressing. |
| Buckles 1977 | https://doi.org/10.1145/355732.355739 | Combination unranking for index-to-string addressing. |
| Davidson 1975 | https://doi.org/10.1016/0021-9991(75)90065-0 | Davidson eigensolver. |
| Crouzeix 1994 | https://doi.org/10.1137/0915004 | Davidson method numerical analysis. |
| Pulay 1980 | https://doi.org/10.1016/0009-2614(80)80396-4 | DIIS convergence acceleration. |
| Sherrill 1999 | https://doi.org/10.1016/S0065-3276(08)60532-8 | CI/FCI entry-point review. |
| Sun 2015 | https://doi.org/10.1002/jcc.23981 | libcint reference. |
| Sun 2018 | https://doi.org/10.1002/wcms.1340 | PySCF reference. |
| Smith 2018 | https://doi.org/10.1021/acs.jctc.8b00286 | Psi4NumPy reference-implementation philosophy. |
| Hirata 2003 | https://doi.org/10.1021/jp034596z | Tensor Contraction Engine and symbolic code-generation route. |

## Notes From Current Web/GitHub Inspection

- The upstream issue is open and accepted.
- The registration PR #210 is open and targets `QuantumBFS/quantum.harness:main`.
- The private working repo is intentionally separate from the public registration
  PR because challenge development may include intermediate experiments,
  generated fixtures, and AGPL-licensed code before final submission.

