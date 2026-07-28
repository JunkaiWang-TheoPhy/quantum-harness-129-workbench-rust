use std::collections::HashMap;

use thiserror::Error;

use crate::problem::ElectronicProblem;

#[derive(Debug, Error)]
pub enum DeterminantError {
    #[error("the Level 0 u64 representation supports at most 32 spatial orbitals")]
    TooManyOrbitals,
    #[error("electron count is inconsistent with NELEC={nelec} and MS2={ms2}")]
    InvalidElectronCount { nelec: usize, ms2: isize },
    #[error("requested {electrons} electrons in only {orbitals} orbitals")]
    TooManyElectrons { electrons: usize, orbitals: usize },
    #[error("ORBSYM contains {actual} labels, expected NORB={expected}")]
    OrbitalSymmetryLength { actual: usize, expected: usize },
    #[error("ORBSYM[{orbital}]={irrep} is outside the Molpro range 1..=8")]
    InvalidOrbitalSymmetry { orbital: usize, irrep: usize },
    #[error("ISYM={0} is outside the Molpro range 1..=8")]
    InvalidWavefunctionSymmetry(usize),
    #[error("the requested ISYM={0} determinant sector is empty")]
    EmptySymmetrySector(usize),
}

#[derive(Debug, Clone)]
pub struct DeterminantBasis {
    pub norb: usize,
    pub nalpha: usize,
    pub nbeta: usize,
    pub alpha_strings: Vec<u64>,
    pub beta_strings: Vec<u64>,
    pub determinants: Vec<u64>,
    string_pairs: Vec<(usize, usize)>,
    pair_addresses: Vec<usize>,
    addresses: HashMap<u64, usize>,
}

impl DeterminantBasis {
    pub fn new(norb: usize, nelec: usize, ms2: isize) -> Result<Self, DeterminantError> {
        Self::build(norb, nelec, ms2, None)
    }

    pub fn with_symmetry(
        norb: usize,
        nelec: usize,
        ms2: isize,
        orbsym: &[usize],
        isym: usize,
    ) -> Result<Self, DeterminantError> {
        if orbsym.len() != norb {
            return Err(DeterminantError::OrbitalSymmetryLength {
                actual: orbsym.len(),
                expected: norb,
            });
        }
        if let Some((orbital, &irrep)) = orbsym
            .iter()
            .enumerate()
            .find(|(_, irrep)| !(1..=8).contains(*irrep))
        {
            return Err(DeterminantError::InvalidOrbitalSymmetry { orbital, irrep });
        }
        if !(1..=8).contains(&isym) {
            return Err(DeterminantError::InvalidWavefunctionSymmetry(isym));
        }
        Self::build(norb, nelec, ms2, Some((orbsym, isym)))
    }

    pub fn from_problem(problem: &ElectronicProblem) -> Result<Self, DeterminantError> {
        Self::with_symmetry(
            problem.norb,
            problem.nelec,
            problem.ms2,
            &problem.orbsym,
            problem.isym,
        )
    }

    fn build(
        norb: usize,
        nelec: usize,
        ms2: isize,
        symmetry: Option<(&[usize], usize)>,
    ) -> Result<Self, DeterminantError> {
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
        let alpha_irreps = symmetry.map(|(orbsym, _)| {
            alpha_strings
                .iter()
                .map(|&bits| string_irrep(bits, orbsym))
                .collect::<Vec<_>>()
        });
        let beta_irreps = symmetry.map(|(orbsym, _)| {
            beta_strings
                .iter()
                .map(|&bits| string_irrep(bits, orbsym))
                .collect::<Vec<_>>()
        });
        let mut determinants = Vec::with_capacity(alpha_strings.len() * beta_strings.len());
        let mut string_pairs = Vec::with_capacity(alpha_strings.len() * beta_strings.len());
        let mut pair_addresses = vec![usize::MAX; alpha_strings.len() * beta_strings.len()];
        for (alpha_index, &alpha) in alpha_strings.iter().enumerate() {
            for (beta_index, &beta) in beta_strings.iter().enumerate() {
                if let Some((_, isym)) = symmetry
                    && molpro_irrep_product(
                        alpha_irreps.as_ref().expect("symmetry irreps")[alpha_index],
                        beta_irreps.as_ref().expect("symmetry irreps")[beta_index],
                    ) != isym
                {
                    continue;
                }
                let address = determinants.len();
                determinants.push(alpha | (beta << norb));
                string_pairs.push((alpha_index, beta_index));
                pair_addresses[alpha_index * beta_strings.len() + beta_index] = address;
            }
        }
        if let Some((_, isym)) = symmetry
            && determinants.is_empty()
        {
            return Err(DeterminantError::EmptySymmetrySector(isym));
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
            string_pairs,
            pair_addresses,
            addresses,
        })
    }

    pub fn address(&self, determinant: u64) -> Option<usize> {
        self.addresses.get(&determinant).copied()
    }

    pub fn string_pair(&self, address: usize) -> Option<(usize, usize)> {
        self.string_pairs.get(address).copied()
    }

    pub fn pair_address(&self, alpha: usize, beta: usize) -> Option<usize> {
        if alpha >= self.alpha_strings.len() || beta >= self.beta_strings.len() {
            return None;
        }
        let address = self.pair_addresses[alpha * self.beta_strings.len() + beta];
        (address != usize::MAX).then_some(address)
    }

    pub fn string_pairs(&self) -> &[(usize, usize)] {
        &self.string_pairs
    }

    pub fn len(&self) -> usize {
        self.determinants.len()
    }

    pub fn is_empty(&self) -> bool {
        self.determinants.is_empty()
    }
}

/// Direct product for one-based Molpro irreps in D2h and its Abelian subgroups.
fn molpro_irrep_product(left: usize, right: usize) -> usize {
    ((left - 1) ^ (right - 1)) + 1
}

fn string_irrep(bits: u64, orbsym: &[usize]) -> usize {
    let mut product = 1;
    for (orbital, &irrep) in orbsym.iter().enumerate() {
        if bits & (1_u64 << orbital) != 0 {
            product = molpro_irrep_product(product, irrep);
        }
    }
    product
}

pub fn occupation_strings(orbitals: usize, electrons: usize) -> Result<Vec<u64>, DeterminantError> {
    if electrons > orbitals {
        return Err(DeterminantError::TooManyElectrons {
            electrons,
            orbitals,
        });
    }
    let limit = if orbitals == 64 {
        u64::MAX
    } else {
        1_u64 << orbitals
    };
    Ok((0..limit)
        .filter(|bits| bits.count_ones() as usize == electrons)
        .collect())
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
    fn filters_to_the_requested_molpro_symmetry_sector() {
        let basis = DeterminantBasis::with_symmetry(2, 2, 0, &[1, 2], 1).unwrap();
        assert_eq!(basis.len(), 2);
        assert_eq!(basis.string_pairs(), &[(0, 0), (1, 1)]);
        assert_eq!(basis.pair_address(0, 0), Some(0));
        assert_eq!(basis.pair_address(0, 1), None);
        assert_eq!(basis.pair_address(1, 1), Some(1));
        assert_eq!(basis.address(0b0101), Some(0));
        assert_eq!(basis.address(0b1010), Some(1));
    }

    #[test]
    fn molpro_irrep_labels_form_the_expected_xor_product_table() {
        assert_eq!(molpro_irrep_product(1, 7), 7);
        assert_eq!(molpro_irrep_product(4, 6), 7);
        assert_eq!(molpro_irrep_product(8, 8), 1);
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
