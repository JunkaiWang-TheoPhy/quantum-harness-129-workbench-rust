use std::collections::HashSet;

use thiserror::Error;

use crate::problem::{ElectronicProblem, ProblemError};

#[derive(Debug, Error)]
pub enum ActiveSpaceError {
    #[error("frozen orbital {0} is out of range")]
    InvalidOrbital(usize),
    #[error("frozen orbitals contain duplicates")]
    DuplicateOrbital,
    #[error("cannot freeze {frozen} doubly occupied orbitals with only {nelec} electrons")]
    TooManyFrozen { frozen: usize, nelec: usize },
    #[error(transparent)]
    Problem(#[from] ProblemError),
}

pub fn freeze_core(
    problem: &ElectronicProblem,
    frozen: &[usize],
) -> Result<ElectronicProblem, ActiveSpaceError> {
    if frozen.is_empty() {
        return Ok(problem.clone());
    }
    let unique: HashSet<_> = frozen.iter().copied().collect();
    if unique.len() != frozen.len() {
        return Err(ActiveSpaceError::DuplicateOrbital);
    }
    if let Some(&orbital) = frozen.iter().find(|&&orbital| orbital >= problem.norb) {
        return Err(ActiveSpaceError::InvalidOrbital(orbital));
    }
    if 2 * frozen.len() > problem.nelec {
        return Err(ActiveSpaceError::TooManyFrozen {
            frozen: frozen.len(),
            nelec: problem.nelec,
        });
    }
    let active: Vec<_> = (0..problem.norb)
        .filter(|orbital| !unique.contains(orbital))
        .collect();
    let mut ecore = problem.ecore;
    for &i in frozen {
        ecore += 2.0 * problem.h1(i, i);
        for &j in frozen {
            ecore += 2.0 * problem.eri(i, i, j, j) - problem.eri(i, j, j, i);
        }
    }
    let nactive = active.len();
    let mut h1 = vec![0.0; nactive * nactive];
    let mut eri = vec![0.0; nactive.pow(4)];
    for (p_new, &p) in active.iter().enumerate() {
        for (q_new, &q) in active.iter().enumerate() {
            let mut value = problem.h1(p, q);
            for &i in frozen {
                value += 2.0 * problem.eri(p, q, i, i) - problem.eri(p, i, i, q);
            }
            h1[p_new * nactive + q_new] = value;
            for (r_new, &r) in active.iter().enumerate() {
                for (s_new, &s) in active.iter().enumerate() {
                    eri[((p_new * nactive + q_new) * nactive + r_new) * nactive + s_new] =
                        problem.eri(p, q, r, s);
                }
            }
        }
    }
    let active_orbsym = active.iter().map(|&index| problem.orbsym[index]).collect();
    let mut active_problem = ElectronicProblem::new(
        nactive,
        problem.nelec - 2 * frozen.len(),
        problem.ms2,
        ecore,
        h1,
        eri,
    )?
    .with_symmetry(active_orbsym, problem.isym)?;
    if let Some(energies) = &problem.orbital_energies {
        active_problem.orbital_energies =
            Some(active.iter().map(|&index| energies[index]).collect());
    }
    Ok(active_problem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_frozen_orbitals_is_identity() {
        let problem = ElectronicProblem::new(1, 2, 0, 0.5, vec![-1.0], vec![0.7]).unwrap();
        let active = freeze_core(&problem, &[]).unwrap();
        assert_eq!(active.ecore, problem.ecore);
        assert_eq!(active.h1(0, 0), problem.h1(0, 0));
    }

    #[test]
    fn folds_a_doubly_occupied_core_into_ecore() {
        let mut eri = vec![0.0; 16];
        eri[0] = 0.7;
        let problem =
            ElectronicProblem::new(2, 4, 0, 0.5, vec![-1.0, 0.0, 0.0, -0.5], eri).unwrap();
        let active = freeze_core(&problem, &[0]).unwrap();
        assert_eq!(active.norb, 1);
        assert_eq!(active.nelec, 2);
        assert!((active.ecore - (-0.8)).abs() < 1e-12);
    }

    #[test]
    fn freezing_a_doubly_occupied_orbital_preserves_total_symmetry() {
        let problem = ElectronicProblem::new(2, 4, 0, 0.0, vec![0.0; 4], vec![0.0; 16])
            .unwrap()
            .with_symmetry(vec![4, 1], 1)
            .unwrap();
        let active = freeze_core(&problem, &[0]).unwrap();
        assert_eq!(active.orbsym, vec![1]);
        assert_eq!(active.isym, 1);
    }
}
