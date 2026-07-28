use rayon::prelude::*;
use thiserror::Error;

use crate::determinant::{DeterminantBasis, DeterminantError};
use crate::operator::{LinearOperator, OperatorError};
use crate::problem::{ElectronicProblem, index4};
use crate::strings::{StringSpace, StringSpaceError};

#[derive(Debug)]
pub struct DirectFciOperator {
    problem: ElectronicProblem,
    basis: DeterminantBasis,
    pub alpha: StringSpace,
    pub beta: StringSpace,
    effective_eri: Vec<f64>,
    alpha_same_spin: Vec<Vec<SameSpinTransition>>,
    beta_same_spin: Option<Vec<Vec<SameSpinTransition>>>,
    diagonal: Vec<f64>,
}

#[derive(Debug, Clone, Copy)]
struct SameSpinTransition {
    target: usize,
    coefficient: f64,
}

#[derive(Debug, Error)]
pub enum DirectFciError {
    #[error("invalid electron/spin counts")]
    InvalidSpin,
    #[error(transparent)]
    Strings(#[from] StringSpaceError),
    #[error(transparent)]
    Determinants(#[from] DeterminantError),
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
        let basis = DeterminantBasis::from_problem(&problem)?;
        let effective_eri = absorb_one_body(&problem);
        let alpha_same_spin = same_spin_transitions(problem.norb, &effective_eri, &alpha);
        let beta_same_spin = (alpha.nelec != beta.nelec)
            .then(|| same_spin_transitions(problem.norb, &effective_eri, &beta));
        let diagonal = hamiltonian_diagonal(&problem, &alpha, &beta, &basis);
        Ok(Self {
            problem,
            basis,
            alpha,
            beta,
            effective_eri,
            alpha_same_spin,
            beta_same_spin,
            diagonal,
        })
    }

    pub fn problem(&self) -> &ElectronicProblem {
        &self.problem
    }

    pub fn basis(&self) -> &DeterminantBasis {
        &self.basis
    }

    fn g(&self, p: usize, q: usize, r: usize, s: usize) -> f64 {
        self.effective_eri[index4(self.problem.norb, p, q, r, s)]
    }

    fn apply_source(
        &self,
        source: usize,
        alpha_source: usize,
        beta_source: usize,
        coefficient: f64,
        output: &mut [f64],
    ) {
        if coefficient == 0.0 {
            return;
        }
        output[source] += self.problem.ecore * coefficient;

        for transition in &self.alpha_same_spin[alpha_source] {
            if let Some(destination) = self.basis.pair_address(transition.target, beta_source) {
                output[destination] += coefficient * transition.coefficient;
            }
        }
        let beta_same_spin = self
            .beta_same_spin
            .as_ref()
            .unwrap_or(&self.alpha_same_spin);
        for transition in &beta_same_spin[beta_source] {
            if let Some(destination) = self.basis.pair_address(alpha_source, transition.target) {
                output[destination] += coefficient * transition.coefficient;
            }
        }

        for first in self.alpha.outgoing(alpha_source) {
            for second in self.beta.outgoing(beta_source) {
                if let Some(destination) = self.basis.pair_address(first.target, second.target) {
                    let integral = self.g(
                        second.created,
                        second.annihilated,
                        first.created,
                        first.annihilated,
                    ) + self.g(
                        first.created,
                        first.annihilated,
                        second.created,
                        second.annihilated,
                    );
                    output[destination] +=
                        coefficient * integral * f64::from(first.sign) * f64::from(second.sign);
                }
            }
        }
    }
}

impl LinearOperator for DirectFciOperator {
    fn dimension(&self) -> usize {
        self.basis.len()
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
        let nonzero = input
            .iter()
            .filter(|&&coefficient| coefficient != 0.0)
            .count();
        let workers = rayon::current_num_threads().min(dimension);
        if dimension < 4_096 || nonzero < workers * 4 || nonzero < dimension / 64 || workers == 1 {
            for (source, (alpha_source, beta_source)) in self.basis.string_pairs().enumerate() {
                self.apply_source(source, alpha_source, beta_source, input[source], output);
            }
            return Ok(());
        }

        let chunk = dimension.div_ceil(workers);
        let partials: Vec<Vec<f64>> = (0..workers)
            .into_par_iter()
            .map(|worker| {
                let start = worker * chunk;
                let end = (start + chunk).min(dimension);
                let mut partial = vec![0.0; dimension];
                for (source, &coefficient) in input.iter().enumerate().take(end).skip(start) {
                    let (alpha_source, beta_source) = self
                        .basis
                        .string_pair(source)
                        .expect("source address belongs to determinant basis");
                    self.apply_source(source, alpha_source, beta_source, coefficient, &mut partial);
                }
                partial
            })
            .collect();
        for partial in partials {
            for (destination, contribution) in output.iter_mut().zip(partial) {
                *destination += contribution;
            }
        }
        Ok(())
    }
}

fn same_spin_transitions(
    norb: usize,
    effective_eri: &[f64],
    strings: &StringSpace,
) -> Vec<Vec<SameSpinTransition>> {
    let mut result = Vec::with_capacity(strings.len());
    let mut coefficients = vec![0.0; strings.len()];
    let mut marks = vec![usize::MAX; strings.len()];
    let mut touched = Vec::new();
    for source in 0..strings.len() {
        touched.clear();
        for first in strings.outgoing(source) {
            for second in strings.outgoing(first.target) {
                let target = second.target;
                if marks[target] != source {
                    marks[target] = source;
                    coefficients[target] = 0.0;
                    touched.push(target);
                }
                let integral = effective_eri[index4(
                    norb,
                    second.created,
                    second.annihilated,
                    first.created,
                    first.annihilated,
                )];
                coefficients[target] += integral * f64::from(first.sign) * f64::from(second.sign);
            }
        }
        touched.sort_unstable();
        result.push(
            touched
                .iter()
                .map(|&target| SameSpinTransition {
                    target,
                    coefficient: coefficients[target],
                })
                .collect(),
        );
    }
    result
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
    basis: &DeterminantBasis,
) -> Vec<f64> {
    let mut result = Vec::with_capacity(basis.len());
    for (alpha_index, beta_index) in basis.string_pairs() {
        let alpha_bits = alpha.strings[alpha_index];
        let beta_bits = beta.strings[beta_index];
        let mut energy = problem.ecore;
        for mut occupied in [alpha_bits, beta_bits] {
            while occupied != 0 {
                let i = occupied.trailing_zeros() as usize;
                energy += problem.h1(i, i);
                occupied &= occupied - 1;
            }
        }
        for (spin_i, bits_i) in [alpha_bits, beta_bits].into_iter().enumerate() {
            let mut occupied_i = bits_i;
            while occupied_i != 0 {
                let i = occupied_i.trailing_zeros() as usize;
                occupied_i &= occupied_i - 1;
                for (spin_j, bits_j) in [alpha_bits, beta_bits].into_iter().enumerate() {
                    let mut occupied_j = bits_j;
                    while occupied_j != 0 {
                        let j = occupied_j.trailing_zeros() as usize;
                        occupied_j &= occupied_j - 1;
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
    result
}

pub fn determinant_basis(operator: &DirectFciOperator) -> DeterminantBasis {
    operator.basis.clone()
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
    fn symmetry_block_sigma_matches_the_projected_dense_hamiltonian() {
        let dump = Fcidump::parse(
            "&FCI NORB=2,NELEC=2,MS2=0,\n\
             ORBSYM=1,2,\n\
             ISYM=1,\n\
             &END\n\
             0.7 1 1 1 1\n\
             0.2 2 2 2 2\n\
             0.1 1 1 2 2\n\
             -1.0 1 1 0 0\n\
             -0.5 2 2 0 0\n",
        )
        .unwrap();
        let operator =
            DirectFciOperator::new(ElectronicProblem::from_fcidump(&dump).unwrap()).unwrap();
        assert_eq!(operator.alpha.len() * operator.beta.len(), 4);
        assert_eq!(operator.dimension(), 2);

        let input = vec![0.3, -0.4];
        let mut direct = vec![0.0; 2];
        operator.apply(&input, &mut direct).unwrap();
        let dense = build_dense_hamiltonian(&dump, operator.basis());
        let expected = dense * DVector::from_vec(input);
        assert!(
            direct
                .iter()
                .zip(expected.iter())
                .all(|(actual, expected)| (actual - expected).abs() < 1e-12)
        );
    }
}
