use thiserror::Error;

use crate::amplitudes::Amplitudes;
use crate::determinant::DeterminantBasis;
use crate::excitation::ExcitationSpace;

#[derive(Debug, Error)]
pub enum ClusterError {
    #[error("amplitude length is {actual}, expected {expected}")]
    AmplitudeLength { actual: usize, expected: usize },
    #[error("vector length is {actual}, expected {expected}")]
    VectorLength { actual: usize, expected: usize },
}

pub struct ClusterOperator<'a> {
    basis: &'a DeterminantBasis,
    space: &'a ExcitationSpace,
    amplitudes: &'a Amplitudes,
}

impl<'a> ClusterOperator<'a> {
    pub fn new(
        basis: &'a DeterminantBasis,
        space: &'a ExcitationSpace,
        amplitudes: &'a Amplitudes,
    ) -> Result<Self, ClusterError> {
        if amplitudes.values.len() != space.excitations.len() {
            return Err(ClusterError::AmplitudeLength {
                actual: amplitudes.values.len(),
                expected: space.excitations.len(),
            });
        }
        Ok(Self {
            basis,
            space,
            amplitudes,
        })
    }

    pub fn apply(&self, input: &[f64], output: &mut [f64]) -> Result<(), ClusterError> {
        if input.len() != self.basis.len() || output.len() != self.basis.len() {
            return Err(ClusterError::VectorLength {
                actual: input.len().max(output.len()),
                expected: self.basis.len(),
            });
        }
        output.fill(0.0);
        for (source_index, &coefficient) in input.iter().enumerate() {
            if coefficient == 0.0 {
                continue;
            }
            let source = self.basis.determinants[source_index];
            for (excitation, &amplitude) in
                self.space.excitations.iter().zip(&self.amplitudes.values)
            {
                if amplitude == 0.0 {
                    continue;
                }
                if let Some((target, phase)) = excitation.apply(source)
                    && let Some(target_index) = self.basis.address(target)
                {
                    output[target_index] += coefficient * amplitude * phase;
                }
            }
        }
        Ok(())
    }

    pub fn apply_adjoint(&self, input: &[f64], output: &mut [f64]) -> Result<(), ClusterError> {
        if input.len() != self.basis.len() || output.len() != self.basis.len() {
            return Err(ClusterError::VectorLength {
                actual: input.len().max(output.len()),
                expected: self.basis.len(),
            });
        }
        output.fill(0.0);
        let mut unit = vec![0.0; self.basis.len()];
        let mut column = vec![0.0; self.basis.len()];
        for source in 0..self.basis.len() {
            unit[source] = 1.0;
            self.apply(&unit, &mut column)?;
            for target in 0..self.basis.len() {
                output[source] += column[target] * input[target];
            }
            unit[source] = 0.0;
        }
        Ok(())
    }

    pub fn exponential_on_reference(&self, threshold: f64) -> Result<Vec<f64>, ClusterError> {
        let mut result = vec![0.0; self.basis.len()];
        result[self.space.reference_index] = 1.0;
        let mut term = result.clone();
        for order in 1..=self.basis.nalpha + self.basis.nbeta {
            let mut next = vec![0.0; self.basis.len()];
            self.apply(&term, &mut next)?;
            for value in &mut next {
                *value /= order as f64;
            }
            let next_norm = next.iter().map(|value| value * value).sum::<f64>().sqrt();
            for (result_value, next_value) in result.iter_mut().zip(&next) {
                *result_value += next_value;
            }
            term = next;
            if next_norm < threshold {
                break;
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::excitation::hartree_fock_reference;

    #[test]
    fn cluster_maps_reference_amplitudes_to_target_coefficients() {
        let basis = DeterminantBasis::new(2, 2, 0).unwrap();
        let reference = hartree_fock_reference(2, 1, 1);
        let space = ExcitationSpace::new(&basis, reference, 2).unwrap();
        let amplitudes = Amplitudes {
            values: (1..=space.excitations.len())
                .map(|index| index as f64 * 0.1)
                .collect(),
        };
        let cluster = ClusterOperator::new(&basis, &space, &amplitudes).unwrap();
        let mut input = vec![0.0; basis.len()];
        input[space.reference_index] = 1.0;
        let mut output = vec![0.0; basis.len()];
        cluster.apply(&input, &mut output).unwrap();
        for (excitation, amplitude) in space.excitations.iter().zip(&amplitudes.values) {
            assert!((output[excitation.determinant_index] - amplitude).abs() < 1e-12);
        }
    }
}
