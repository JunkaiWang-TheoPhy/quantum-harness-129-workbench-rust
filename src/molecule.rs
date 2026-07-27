use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Molecule {
    pub atom: String,
    pub basis: String,
    pub charge: isize,
}

#[derive(Debug, Error)]
pub enum MoleculeError {
    #[error("atom specification must not be empty")]
    EmptyAtoms,
    #[error("basis must not be empty")]
    EmptyBasis,
    #[error("atom or basis strings must not contain double quotes")]
    InvalidText,
}

impl Molecule {
    pub fn new(
        atom: impl Into<String>,
        basis: impl Into<String>,
        charge: isize,
    ) -> Result<Self, MoleculeError> {
        let atom = atom.into();
        let basis = basis.into();
        if atom.trim().is_empty() {
            return Err(MoleculeError::EmptyAtoms);
        }
        if basis.trim().is_empty() {
            return Err(MoleculeError::EmptyBasis);
        }
        if atom.contains('"') || basis.contains('"') {
            return Err(MoleculeError::InvalidText);
        }
        Ok(Self {
            atom,
            basis,
            charge,
        })
    }

    pub fn h2_sto3g() -> Self {
        Self::new("H 0 0 -0.7; H 0 0 0.7", "STO-3G", 0).expect("built-in molecule is valid")
    }

    pub fn h2o_sto3g() -> Self {
        Self::new(
            "O 0 0 0; H 0.967 0 0; H -0.2923916843556798 0.9217353757557798 0",
            "STO-3G",
            0,
        )
        .expect("built-in molecule is valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_or_unsafe_text_fields() {
        assert!(matches!(
            Molecule::new("", "STO-3G", 0),
            Err(MoleculeError::EmptyAtoms)
        ));
        assert!(matches!(
            Molecule::new("H 0 0 0", "", 0),
            Err(MoleculeError::EmptyBasis)
        ));
        assert!(matches!(
            Molecule::new("H \"0\" 0 0", "STO-3G", 0),
            Err(MoleculeError::InvalidText)
        ));
    }

    #[test]
    fn built_in_systems_have_expected_charge_and_basis() {
        for molecule in [Molecule::h2_sto3g(), Molecule::h2o_sto3g()] {
            assert_eq!(molecule.charge, 0);
            assert_eq!(molecule.basis, "STO-3G");
            assert!(!molecule.atom.is_empty());
        }
    }
}
