use std::collections::HashMap;

use thiserror::Error;

use crate::combinadic::{CombinadicError, combination_count, unrank_occupation};

#[derive(Debug, Error)]
pub enum DeterminantError {
    #[error("the Level 0 u64 representation supports at most 32 spatial orbitals")]
    TooManyOrbitals,
    #[error("electron count is inconsistent with NELEC={nelec} and MS2={ms2}")]
    InvalidElectronCount { nelec: usize, ms2: isize },
    #[error("requested {electrons} electrons in only {orbitals} orbitals")]
    TooManyElectrons { electrons: usize, orbitals: usize },
    #[error("determinant space contains {count} entries and does not fit this platform")]
    SpaceTooLarge { count: u128 },
    #[error("failed to allocate determinant space with {count} entries")]
    AllocationFailed { count: usize },
    #[error(transparent)]
    Combinadic(#[from] CombinadicError),
}

#[derive(Debug, Clone)]
pub struct DeterminantBasis {
    pub norb: usize,
    pub nalpha: usize,
    pub nbeta: usize,
    pub alpha_strings: Vec<u64>,
    pub beta_strings: Vec<u64>,
    pub determinants: Vec<u64>,
    addresses: HashMap<u64, usize>,
}

impl DeterminantBasis {
    pub fn new(norb: usize, nelec: usize, ms2: isize) -> Result<Self, DeterminantError> {
        if norb > 32 {
            return Err(DeterminantError::TooManyOrbitals);
        }
        let nalpha_twice = nelec as isize + ms2;
        let nbeta_twice = nelec as isize - ms2;
        if nalpha_twice < 0 || nbeta_twice < 0 || nalpha_twice % 2 != 0 || nbeta_twice % 2 != 0 {
            return Err(DeterminantError::InvalidElectronCount { nelec, ms2 });
        }
        let nalpha = (nalpha_twice / 2) as usize;
        let nbeta = (nbeta_twice / 2) as usize;
        let alpha_strings = occupation_strings(norb, nalpha)?;
        let beta_strings = occupation_strings(norb, nbeta)?;
        let determinant_count = alpha_strings
            .len()
            .checked_mul(beta_strings.len())
            .ok_or(DeterminantError::SpaceTooLarge {
                count: alpha_strings.len() as u128 * beta_strings.len() as u128,
            })?;
        let mut determinants = Vec::new();
        determinants
            .try_reserve_exact(determinant_count)
            .map_err(|_| DeterminantError::AllocationFailed {
                count: determinant_count,
            })?;
        for &alpha in &alpha_strings {
            for &beta in &beta_strings {
                determinants.push(alpha | (beta << norb));
            }
        }
        let addresses = determinants
            .iter()
            .enumerate()
            .map(|(index, &det)| (det, index))
            .collect();
        Ok(Self {
            norb,
            nalpha,
            nbeta,
            alpha_strings,
            beta_strings,
            determinants,
            addresses,
        })
    }

    pub fn address(&self, determinant: u64) -> Option<usize> {
        self.addresses.get(&determinant).copied()
    }

    pub fn len(&self) -> usize {
        self.determinants.len()
    }

    pub fn is_empty(&self) -> bool {
        self.determinants.is_empty()
    }
}

pub fn occupation_strings(orbitals: usize, electrons: usize) -> Result<Vec<u64>, DeterminantError> {
    if electrons > orbitals {
        return Err(DeterminantError::TooManyElectrons {
            electrons,
            orbitals,
        });
    }
    if orbitals > 64 {
        return Err(DeterminantError::TooManyOrbitals);
    }
    let count_u128 = combination_count(orbitals, electrons)?;
    let count =
        usize::try_from(count_u128).map_err(|_| DeterminantError::SpaceTooLarge {
            count: count_u128,
        })?;
    let mut strings = Vec::new();
    strings
        .try_reserve_exact(count)
        .map_err(|_| DeterminantError::AllocationFailed { count })?;
    for rank in 0..count {
        strings.push(unrank_occupation(rank as u128, orbitals, electrons)?);
    }
    Ok(strings)
}

pub fn apply_annihilation(determinant: u64, orbital: usize) -> Option<(u64, f64)> {
    let mask = 1_u64 << orbital;
    if determinant & mask == 0 {
        return None;
    }
    let occupied_below = (determinant & (mask - 1)).count_ones();
    let sign = if occupied_below.is_multiple_of(2) {
        1.0
    } else {
        -1.0
    };
    Some((determinant ^ mask, sign))
}

pub fn apply_creation(determinant: u64, orbital: usize) -> Option<(u64, f64)> {
    let mask = 1_u64 << orbital;
    if determinant & mask != 0 {
        return None;
    }
    let occupied_below = (determinant & (mask - 1)).count_ones();
    let sign = if occupied_below.is_multiple_of(2) {
        1.0
    } else {
        -1.0
    };
    Some((determinant | mask, sign))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enumerates_fixed_population_in_numeric_lexical_order() {
        assert_eq!(
            occupation_strings(4, 2).unwrap(),
            vec![0b0011, 0b0101, 0b0110, 0b1001, 0b1010, 0b1100]
        );
    }

    #[test]
    fn builds_alpha_beta_product_basis() {
        let basis = DeterminantBasis::new(2, 2, 0).unwrap();
        assert_eq!(basis.nalpha, 1);
        assert_eq!(basis.nbeta, 1);
        assert_eq!(basis.len(), 4);
        for &det in &basis.determinants {
            assert_eq!((det & 0b11).count_ones(), 1);
            assert_eq!((det >> 2).count_ones(), 1);
            assert!(basis.address(det).is_some());
        }
    }

    #[test]
    fn fermionic_operators_return_expected_sign() {
        let det = 0b1011;
        assert_eq!(apply_annihilation(det, 3), Some((0b0011, 1.0)));
        assert_eq!(apply_annihilation(det, 2), None);
        assert_eq!(apply_creation(det, 2), Some((0b1111, 1.0)));
        assert_eq!(apply_annihilation(0b1010, 3), Some((0b0010, -1.0)));
    }
}
