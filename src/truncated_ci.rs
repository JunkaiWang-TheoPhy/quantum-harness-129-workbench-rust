use thiserror::Error;

use crate::davidson::{DavidsonConfig, DavidsonError, DavidsonResult, lowest_eigenpair};
use crate::determinant::{DeterminantBasis, DeterminantError};
use crate::direct_fci::DirectFciOperator;
use crate::excitation::hartree_fock_reference;
use crate::operator::{LinearOperator, OperatorError};

pub struct ProjectedOperator<'a> {
    full: &'a DirectFciOperator,
    selected: Vec<usize>,
    diagonal: Vec<f64>,
}

impl<'a> ProjectedOperator<'a> {
    pub fn through_rank(
        full: &'a DirectFciOperator,
        rank: usize,
    ) -> Result<Self, DeterminantError> {
        let problem = full.problem();
        let basis = DeterminantBasis::new(problem.norb, problem.nelec, problem.ms2)?;
        let reference = hartree_fock_reference(problem.norb, basis.nalpha, basis.nbeta);
        let selected: Vec<_> = basis
            .determinants
            .iter()
            .enumerate()
            .filter(|(_, determinant)| (reference & !**determinant).count_ones() as usize <= rank)
            .map(|(index, _)| index)
            .collect();
        let diagonal = selected
            .iter()
            .map(|&index| full.diagonal()[index])
            .collect();
        Ok(Self {
            full,
            selected,
            diagonal,
        })
    }

    pub fn selected_indices(&self) -> &[usize] {
        &self.selected
    }
}

impl LinearOperator for ProjectedOperator<'_> {
    fn dimension(&self) -> usize {
        self.selected.len()
    }

    fn diagonal(&self) -> &[f64] {
        &self.diagonal
    }

    fn apply(&self, input: &[f64], output: &mut [f64]) -> Result<(), OperatorError> {
        if input.len() != self.dimension() {
            return Err(OperatorError::InputLength {
                actual: input.len(),
                expected: self.dimension(),
            });
        }
        if output.len() != self.dimension() {
            return Err(OperatorError::OutputLength {
                actual: output.len(),
                expected: self.dimension(),
            });
        }
        let mut full_input = vec![0.0; self.full.dimension()];
        for (&index, &value) in self.selected.iter().zip(input) {
            full_input[index] = value;
        }
        let mut full_output = vec![0.0; self.full.dimension()];
        self.full.apply(&full_input, &mut full_output)?;
        for (value, &index) in output.iter_mut().zip(&self.selected) {
            *value = full_output[index];
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum TruncatedCiError {
    #[error(transparent)]
    Determinant(#[from] DeterminantError),
    #[error(transparent)]
    Davidson(#[from] DavidsonError),
}

pub fn solve_ci(
    full: &DirectFciOperator,
    rank: usize,
    config: &DavidsonConfig,
) -> Result<DavidsonResult, TruncatedCiError> {
    let projected = ProjectedOperator::through_rank(full, rank)?;
    let mut initial = vec![0.0; projected.dimension()];
    let index = projected
        .diagonal()
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| left.total_cmp(right))
        .expect("non-empty CI space")
        .0;
    initial[index] = 1.0;
    Ok(lowest_eigenpair(&projected, &initial, config)?)
}
