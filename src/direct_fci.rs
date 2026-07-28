use std::collections::HashMap;

use thiserror::Error;

use crate::determinant::DeterminantBasis;
use crate::operator::{LinearOperator, OperatorError};
use crate::problem::{ElectronicProblem, index4};
use crate::strings::{OneBodyLink, StringSpace, StringSpaceError};

#[derive(Debug)]
pub struct DirectFciKernel {
    problem: ElectronicProblem,
    pub alpha: StringSpace,
    pub beta: StringSpace,
    effective_eri: Vec<f64>,
}

#[derive(Debug)]
pub struct DirectFciOperator {
    kernel: DirectFciKernel,
    diagonal: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SparseColumn {
    pub source: usize,
    pub entries: Vec<(usize, f64)>,
    pub raw_contributions: usize,
}

#[derive(Debug, Error)]
pub enum DirectFciError {
    #[error("invalid electron/spin counts")]
    InvalidSpin,
    #[error("source determinant index {index} is outside dimension {dimension}")]
    SourceOutOfRange { index: usize, dimension: usize },
    #[error(transparent)]
    Strings(#[from] StringSpaceError),
}

impl DirectFciKernel {
    pub fn new(problem: ElectronicProblem) -> Result<Self, DirectFciError> {
        let nalpha_twice = problem.nelec as isize + problem.ms2;
        let nbeta_twice = problem.nelec as isize - problem.ms2;
        if nalpha_twice < 0 || nbeta_twice < 0 || nalpha_twice % 2 != 0 || nbeta_twice % 2 != 0 {
            return Err(DirectFciError::InvalidSpin);
        }
        let alpha = StringSpace::new(problem.norb, (nalpha_twice / 2) as usize)?;
        let beta = StringSpace::new(problem.norb, (nbeta_twice / 2) as usize)?;
        let effective_eri = absorb_one_body(&problem);
        Ok(Self {
            problem,
            alpha,
            beta,
            effective_eri,
        })
    }

    pub fn problem(&self) -> &ElectronicProblem {
        &self.problem
    }

    fn g(&self, p: usize, q: usize, r: usize, s: usize) -> f64 {
        self.effective_eri[index4(self.problem.norb, p, q, r, s)]
    }

    fn link_pair_value(&self, coefficient: f64, first: &OneBodyLink, second: &OneBodyLink) -> f64 {
        let integral = self.g(
            second.created,
            second.annihilated,
            first.created,
            first.annihilated,
        );
        coefficient * integral * f64::from(first.sign) * f64::from(second.sign)
    }

    fn apply_source(
        &self,
        source: usize,
        coefficient: f64,
        mut accumulate: impl FnMut(usize, f64),
    ) -> Result<(), DirectFciError> {
        let dimension = self.dimension();
        if source >= dimension {
            return Err(DirectFciError::SourceOutOfRange {
                index: source,
                dimension,
            });
        }
        let nb = self.beta.len();
        let alpha_source = source / nb;
        let beta_source = source % nb;
        accumulate(source, self.problem.ecore * coefficient);

        for first in self.alpha.outgoing(alpha_source) {
            for second in self.alpha.outgoing(first.target) {
                let destination = second.target * nb + beta_source;
                accumulate(
                    destination,
                    self.link_pair_value(coefficient, first, second),
                );
            }
            for second in self.beta.outgoing(beta_source) {
                let destination = first.target * nb + second.target;
                accumulate(
                    destination,
                    self.link_pair_value(coefficient, first, second),
                );
            }
        }
        for first in self.beta.outgoing(beta_source) {
            for second in self.alpha.outgoing(alpha_source) {
                let destination = second.target * nb + first.target;
                accumulate(
                    destination,
                    self.link_pair_value(coefficient, first, second),
                );
            }
            for second in self.beta.outgoing(first.target) {
                let destination = alpha_source * nb + second.target;
                accumulate(
                    destination,
                    self.link_pair_value(coefficient, first, second),
                );
            }
        }
        Ok(())
    }

    pub fn dimension(&self) -> usize {
        self.alpha.len() * self.beta.len()
    }

    pub fn apply_source_sparse(&self, source: usize) -> Result<SparseColumn, DirectFciError> {
        let mut entries = HashMap::new();
        let mut raw_contributions = 0;
        self.apply_source(source, 1.0, |destination, value| {
            raw_contributions += 1;
            *entries.entry(destination).or_insert(0.0) += value;
        })?;
        let mut entries: Vec<_> = entries
            .into_iter()
            .filter(|(_, value)| *value != 0.0)
            .collect();
        entries.sort_unstable_by_key(|(destination, _)| *destination);
        Ok(SparseColumn {
            source,
            entries,
            raw_contributions,
        })
    }

    pub fn alpha_link_count(&self) -> usize {
        self.alpha.link_count()
    }

    pub fn beta_link_count(&self) -> usize {
        self.beta.link_count()
    }
}

impl DirectFciOperator {
    pub fn new(problem: ElectronicProblem) -> Result<Self, DirectFciError> {
        let kernel = DirectFciKernel::new(problem)?;
        let diagonal = hamiltonian_diagonal(&kernel.problem, &kernel.alpha, &kernel.beta);
        Ok(Self { kernel, diagonal })
    }

    pub fn problem(&self) -> &ElectronicProblem {
        self.kernel.problem()
    }
}

impl LinearOperator for DirectFciOperator {
    fn dimension(&self) -> usize {
        self.kernel.dimension()
    }

    fn diagonal(&self) -> &[f64] {
        &self.diagonal
    }

    fn apply(&self, input: &[f64], output: &mut [f64]) -> Result<(), OperatorError> {
        let dimension = self.dimension();
        if input.len() != dimension {
            return Err(OperatorError::InputLength {
                actual: input.len(),
                expected: dimension,
            });
        }
        if output.len() != dimension {
            return Err(OperatorError::OutputLength {
                actual: output.len(),
                expected: dimension,
            });
        }
        output.fill(0.0);
        for (source, &coefficient) in input.iter().enumerate() {
            if coefficient == 0.0 {
                continue;
            }
            self.kernel
                .apply_source(source, coefficient, |destination, value| {
                    output[destination] += value;
                })
                .expect("source originates from the validated input dimension");
        }
        Ok(())
    }
}

fn absorb_one_body(problem: &ElectronicProblem) -> Vec<f64> {
    let n = problem.norb;
    let mut result = problem.eri_data().to_vec();
    let mut f1 = vec![0.0; n * n];
    for p in 0..n {
        for q in 0..n {
            let mut contraction = 0.0;
            for i in 0..n {
                contraction += problem.eri(p, i, i, q);
            }
            f1[p * n + q] = (problem.h1(p, q) - 0.5 * contraction) / problem.nelec as f64;
        }
    }
    for k in 0..n {
        for p in 0..n {
            for q in 0..n {
                result[index4(n, k, k, p, q)] += f1[p * n + q];
                result[index4(n, p, q, k, k)] += f1[p * n + q];
            }
        }
    }
    for value in &mut result {
        *value *= 0.5;
    }
    result
}

fn hamiltonian_diagonal(
    problem: &ElectronicProblem,
    alpha: &StringSpace,
    beta: &StringSpace,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(alpha.len() * beta.len());
    for &alpha_bits in &alpha.strings {
        for &beta_bits in &beta.strings {
            let mut energy = problem.ecore;
            for spin in 0..2 {
                let bits = if spin == 0 { alpha_bits } else { beta_bits };
                for i in 0..problem.norb {
                    if bits & (1_u64 << i) != 0 {
                        energy += problem.h1(i, i);
                    }
                }
            }
            for spin_i in 0..2 {
                let bits_i = if spin_i == 0 { alpha_bits } else { beta_bits };
                for i in 0..problem.norb {
                    if bits_i & (1_u64 << i) == 0 {
                        continue;
                    }
                    for spin_j in 0..2 {
                        let bits_j = if spin_j == 0 { alpha_bits } else { beta_bits };
                        for j in 0..problem.norb {
                            if bits_j & (1_u64 << j) == 0 {
                                continue;
                            }
                            energy += 0.5 * problem.eri(i, i, j, j);
                            if spin_i == spin_j {
                                energy -= 0.5 * problem.eri(i, j, j, i);
                            }
                        }
                    }
                }
            }
            result.push(energy);
        }
    }
    result
}

pub fn determinant_basis(operator: &DirectFciOperator) -> DeterminantBasis {
    DeterminantBasis::new(
        operator.problem().norb,
        operator.problem().nelec,
        operator.problem().ms2,
    )
    .expect("validated direct-FCI problem must define a determinant basis")
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use nalgebra::DVector;

    use super::*;
    use crate::fcidump::Fcidump;
    use crate::hamiltonian::build_dense_hamiltonian;

    fn compare_fixture(slug: &str) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join(slug)
            .join("FCIDUMP");
        let dump = Fcidump::parse(&fs::read_to_string(path).unwrap()).unwrap();
        let problem = ElectronicProblem::from_fcidump(&dump).unwrap();
        let operator = DirectFciOperator::new(problem).unwrap();
        let input: Vec<_> = (0..operator.dimension())
            .map(|index| ((index * 17 + 3) as f64).sin())
            .collect();
        let mut direct = vec![0.0; input.len()];
        operator.apply(&input, &mut direct).unwrap();
        let dense = build_dense_hamiltonian(&dump, &determinant_basis(&operator));
        let expected = &dense * DVector::from_vec(input);
        let error = direct
            .iter()
            .zip(expected.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        assert!(error < 1e-11, "{slug} sigma error {error:e}");
        for (actual, expected) in operator.diagonal().iter().zip(dense.diagonal().iter()) {
            assert!((actual - expected).abs() < 1e-11);
        }
    }

    #[test]
    fn h2_direct_sigma_matches_dense() {
        compare_fixture("h2-sto3g");
    }

    #[test]
    fn h4_direct_sigma_matches_dense() {
        compare_fixture("h4-sto3g");
    }

    #[test]
    fn sparse_columns_match_full_operator() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join("h2-sto3g")
            .join("FCIDUMP");
        let dump = Fcidump::parse(&fs::read_to_string(path).unwrap()).unwrap();
        let problem = ElectronicProblem::from_fcidump(&dump).unwrap();
        let operator = DirectFciOperator::new(problem.clone()).unwrap();
        let kernel = DirectFciKernel::new(problem).unwrap();
        for source in 0..operator.dimension() {
            let mut input = vec![0.0; operator.dimension()];
            input[source] = 1.0;
            let mut expected = vec![0.0; operator.dimension()];
            operator.apply(&input, &mut expected).unwrap();
            let sparse = kernel.apply_source_sparse(source).unwrap();
            let mut actual = vec![0.0; operator.dimension()];
            for &(destination, value) in &sparse.entries {
                actual[destination] = value;
            }
            for (actual, expected) in actual.iter().zip(expected) {
                assert!((actual - expected).abs() < 1e-12);
            }
        }
    }
}
