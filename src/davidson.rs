use nalgebra::{DMatrix, linalg::SymmetricEigen};
use thiserror::Error;

use crate::operator::{LinearOperator, OperatorError};

#[derive(Debug, Clone)]
pub struct DavidsonConfig {
    pub residual_tolerance: f64,
    pub energy_tolerance: f64,
    pub max_iterations: usize,
    pub max_subspace: usize,
}

impl Default for DavidsonConfig {
    fn default() -> Self {
        Self {
            residual_tolerance: 1e-10,
            energy_tolerance: 1e-12,
            max_iterations: 100,
            max_subspace: 24,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DavidsonResult {
    pub energy: f64,
    pub eigenvector: Vec<f64>,
    pub residual_norm: f64,
    pub iterations: usize,
    pub converged: bool,
}

#[derive(Debug, Error)]
pub enum DavidsonError {
    #[error("operator dimension is zero")]
    EmptyOperator,
    #[error("initial vector length is {actual}, expected {expected}")]
    InitialLength { actual: usize, expected: usize },
    #[error("initial vector has zero or non-finite norm")]
    InvalidInitialVector,
    #[error("max_subspace must be at least 2")]
    InvalidSubspace,
    #[error(transparent)]
    Operator(#[from] OperatorError),
}

pub fn lowest_eigenpair(
    operator: &impl LinearOperator,
    initial: &[f64],
    config: &DavidsonConfig,
) -> Result<DavidsonResult, DavidsonError> {
    let dimension = operator.dimension();
    if dimension == 0 {
        return Err(DavidsonError::EmptyOperator);
    }
    if initial.len() != dimension {
        return Err(DavidsonError::InitialLength {
            actual: initial.len(),
            expected: dimension,
        });
    }
    if config.max_subspace < 2 {
        return Err(DavidsonError::InvalidSubspace);
    }
    let mut first = initial.to_vec();
    normalize(&mut first)?;
    let mut first_sigma = vec![0.0; dimension];
    operator.apply(&first, &mut first_sigma)?;
    let mut basis = vec![first];
    let mut sigma_basis = vec![first_sigma];
    let mut previous_energy = f64::INFINITY;
    let mut last_result = DavidsonResult {
        energy: f64::NAN,
        eigenvector: vec![0.0; dimension],
        residual_norm: f64::INFINITY,
        iterations: 0,
        converged: false,
    };

    for iteration in 1..=config.max_iterations {
        let subspace = basis.len();
        let mut projected = DMatrix::zeros(subspace, subspace);
        for i in 0..subspace {
            for j in 0..=i {
                let value = dot(&basis[i], &sigma_basis[j]);
                projected[(i, j)] = value;
                projected[(j, i)] = value;
            }
        }
        let eigensystem = SymmetricEigen::new(projected);
        let (root, &energy) = eigensystem
            .eigenvalues
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.total_cmp(b))
            .expect("non-empty Davidson projected spectrum");
        let coefficients = eigensystem.eigenvectors.column(root);
        let mut eigenvector = vec![0.0; dimension];
        let mut sigma = vec![0.0; dimension];
        for k in 0..subspace {
            axpy(coefficients[k], &basis[k], &mut eigenvector);
            axpy(coefficients[k], &sigma_basis[k], &mut sigma);
        }
        let restart_sigma = (basis.len() >= config.max_subspace).then(|| sigma.clone());
        let mut residual = sigma;
        axpy(-energy, &eigenvector, &mut residual);
        let residual_norm = norm(&residual);
        let energy_change = (energy - previous_energy).abs();
        last_result = DavidsonResult {
            energy,
            eigenvector: eigenvector.clone(),
            residual_norm,
            iterations: iteration,
            converged: residual_norm <= config.residual_tolerance
                && energy_change <= config.energy_tolerance,
        };
        if last_result.converged
            || (residual_norm <= config.residual_tolerance && subspace == dimension)
        {
            last_result.converged = true;
            return Ok(last_result);
        }
        previous_energy = energy;
        if iteration == config.max_iterations {
            return Ok(last_result);
        }

        let mut correction = residual;
        for (index, value) in correction.iter_mut().enumerate() {
            let denominator = energy - operator.diagonal()[index];
            if denominator.abs() > 1e-12 {
                *value /= denominator;
            }
        }
        orthogonalize(&mut correction, &basis);
        if norm(&correction) < 1e-12 {
            correction = (0..dimension)
                .map(|index| {
                    let mut candidate = vec![0.0; dimension];
                    candidate[index] = 1.0;
                    orthogonalize(&mut candidate, &basis);
                    candidate
                })
                .max_by(|left, right| norm(left).total_cmp(&norm(right)))
                .expect("non-empty coordinate basis");
        }
        if norm(&correction) < 1e-12 && basis.len() == dimension {
            last_result.converged = last_result.residual_norm <= config.residual_tolerance;
            return Ok(last_result);
        }
        normalize(&mut correction)?;
        let mut correction_sigma = vec![0.0; dimension];
        operator.apply(&correction, &mut correction_sigma)?;

        if basis.len() >= config.max_subspace {
            normalize(&mut eigenvector)?;
            basis = vec![eigenvector, correction];
            sigma_basis = vec![
                restart_sigma.expect("restart sigma was retained"),
                correction_sigma,
            ];
            orthonormalize_last(&mut basis, &mut sigma_basis);
        } else {
            basis.push(correction);
            sigma_basis.push(correction_sigma);
        }
    }
    Ok(last_result)
}

fn dot(left: &[f64], right: &[f64]) -> f64 {
    left.iter().zip(right).map(|(a, b)| a * b).sum()
}

fn norm(vector: &[f64]) -> f64 {
    dot(vector, vector).sqrt()
}

fn axpy(alpha: f64, x: &[f64], y: &mut [f64]) {
    for (y_value, x_value) in y.iter_mut().zip(x) {
        *y_value += alpha * x_value;
    }
}

fn normalize(vector: &mut [f64]) -> Result<(), DavidsonError> {
    let magnitude = norm(vector);
    if !magnitude.is_finite() || magnitude <= 1e-15 {
        return Err(DavidsonError::InvalidInitialVector);
    }
    for value in vector {
        *value /= magnitude;
    }
    Ok(())
}

fn orthogonalize(vector: &mut [f64], basis: &[Vec<f64>]) {
    for _ in 0..2 {
        for basis_vector in basis {
            let overlap = dot(basis_vector, vector);
            axpy(-overlap, basis_vector, vector);
        }
    }
}

fn orthonormalize_last(basis: &mut [Vec<f64>], sigma_basis: &mut [Vec<f64>]) {
    if basis.len() < 2 {
        return;
    }
    let overlap = dot(&basis[0], &basis[1]);
    let first = basis[0].clone();
    let first_sigma = sigma_basis[0].clone();
    axpy(-overlap, &first, &mut basis[1]);
    axpy(-overlap, &first_sigma, &mut sigma_basis[1]);
    let magnitude = norm(&basis[1]);
    if magnitude > 1e-15 {
        for value in &mut basis[1] {
            *value /= magnitude;
        }
        for value in &mut sigma_basis[1] {
            *value /= magnitude;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DVector;

    struct MatrixOperator {
        matrix: DMatrix<f64>,
        diagonal: Vec<f64>,
    }

    impl LinearOperator for MatrixOperator {
        fn dimension(&self) -> usize {
            self.matrix.nrows()
        }

        fn diagonal(&self) -> &[f64] {
            &self.diagonal
        }

        fn apply(&self, input: &[f64], output: &mut [f64]) -> Result<(), OperatorError> {
            let result = &self.matrix * DVector::from_column_slice(input);
            output.copy_from_slice(result.as_slice());
            Ok(())
        }
    }

    #[test]
    fn finds_lowest_eigenvalue_of_symmetric_matrix() {
        let matrix = DMatrix::from_row_slice(3, 3, &[1.0, 0.2, 0.0, 0.2, 2.0, 0.3, 0.0, 0.3, 4.0]);
        let expected = SymmetricEigen::new(matrix.clone()).eigenvalues.min();
        let operator = MatrixOperator {
            diagonal: matrix.diagonal().iter().copied().collect(),
            matrix,
        };
        let result = lowest_eigenpair(
            &operator,
            &[1.0, 0.1, 0.1],
            &DavidsonConfig {
                max_subspace: 3,
                ..Default::default()
            },
        )
        .unwrap();
        assert!(result.converged);
        assert!((result.energy - expected).abs() < 1e-10);
        assert!(result.residual_norm < 1e-10);
    }
}
