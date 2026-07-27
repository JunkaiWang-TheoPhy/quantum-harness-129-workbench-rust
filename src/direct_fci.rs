use thiserror::Error;

use crate::determinant::DeterminantBasis;
use crate::operator::{LinearOperator, OperatorError};
use crate::problem::{ElectronicProblem, index4};
use crate::strings::{OneBodyLink, StringSpace, StringSpaceError};

#[derive(Debug)]
pub struct DirectFciOperator {
    problem: ElectronicProblem,
    pub alpha: StringSpace,
    pub beta: StringSpace,
    effective_eri: Vec<f64>,
    diagonal: Vec<f64>,
}

#[derive(Debug, Error)]
pub enum DirectFciError {
    #[error("invalid electron/spin counts")]
    InvalidSpin,
    #[error(transparent)]
    Strings(#[from] StringSpaceError),
}

impl DirectFciOperator {
    pub fn new(problem: ElectronicProblem) -> Result<Self, DirectFciError> {
        let nalpha_twice = problem.nelec as isize + problem.ms2;
        let nbeta_twice = problem.nelec as isize - problem.ms2;
        if nalpha_twice < 0 || nbeta_twice < 0 || nalpha_twice % 2 != 0 || nbeta_twice % 2 != 0 {
            return Err(DirectFciError::InvalidSpin);
        }
        let alpha = StringSpace::new(problem.norb, (nalpha_twice / 2) as usize)?;
        let beta = StringSpace::new(problem.norb, (nbeta_twice / 2) as usize)?;
        let effective_eri = absorb_one_body(&problem);
        let diagonal = hamiltonian_diagonal(&problem, &alpha, &beta);
        Ok(Self {
            problem,
            alpha,
            beta,
            effective_eri,
            diagonal,
        })
    }

    pub fn problem(&self) -> &ElectronicProblem {
        &self.problem
    }

    fn g(&self, p: usize, q: usize, r: usize, s: usize) -> f64 {
        self.effective_eri[index4(self.problem.norb, p, q, r, s)]
    }

    fn add_link_pair(
        &self,
        coefficient: f64,
        first: &OneBodyLink,
        second: &OneBodyLink,
        destination: usize,
        output: &mut [f64],
    ) {
        let integral = self.g(
            second.created,
            second.annihilated,
            first.created,
            first.annihilated,
        );
        output[destination] +=
            coefficient * integral * f64::from(first.sign) * f64::from(second.sign);
    }
}

impl LinearOperator for DirectFciOperator {
    fn dimension(&self) -> usize {
        self.alpha.len() * self.beta.len()
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
        let nb = self.beta.len();
        for alpha_source in 0..self.alpha.len() {
            for beta_source in 0..nb {
                let source = alpha_source * nb + beta_source;
                let coefficient = input[source];
                if coefficient == 0.0 {
                    continue;
                }
                output[source] += self.problem.ecore * coefficient;

                for first in self.alpha.outgoing(alpha_source) {
                    for second in self.alpha.outgoing(first.target) {
                        let destination = second.target * nb + beta_source;
                        self.add_link_pair(coefficient, first, second, destination, output);
                    }
                    for second in self.beta.outgoing(beta_source) {
                        let destination = first.target * nb + second.target;
                        self.add_link_pair(coefficient, first, second, destination, output);
                    }
                }
                for first in self.beta.outgoing(beta_source) {
                    for second in self.alpha.outgoing(alpha_source) {
                        let destination = second.target * nb + first.target;
                        self.add_link_pair(coefficient, first, second, destination, output);
                    }
                    for second in self.beta.outgoing(first.target) {
                        let destination = alpha_source * nb + second.target;
                        self.add_link_pair(coefficient, first, second, destination, output);
                    }
                }
            }
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
        operator.problem.norb,
        operator.problem.nelec,
        operator.problem.ms2,
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
}
