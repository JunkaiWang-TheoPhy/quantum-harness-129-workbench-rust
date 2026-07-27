use std::time::{Duration, Instant};

use thiserror::Error;

use crate::amplitudes::{Amplitudes, orbital_denominators};
use crate::cluster::{ClusterError, ClusterExpansionPlan};
use crate::determinant::{DeterminantBasis, DeterminantError};
use crate::diis::{Diis, DiisError};
use crate::direct_fci::DirectFciOperator;
use crate::excitation::{ExcitationError, ExcitationSpace, hartree_fock_reference};
use crate::operator::{LinearOperator, OperatorError};

#[derive(Debug, Clone)]
pub struct CcConfig {
    pub residual_tolerance: f64,
    pub energy_tolerance: f64,
    pub max_iterations: usize,
    pub diis_history: usize,
    pub exponential_threshold: f64,
}

impl Default for CcConfig {
    fn default() -> Self {
        Self {
            residual_tolerance: 1e-8,
            energy_tolerance: 1e-10,
            max_iterations: 100,
            diis_history: 6,
            exponential_threshold: 1e-14,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CcIteration {
    pub iteration: usize,
    pub energy: f64,
    pub energy_change: f64,
    pub residual_norm: f64,
}

#[derive(Debug, Clone)]
pub struct CcResult {
    pub energy: f64,
    pub amplitudes: Amplitudes,
    pub residual_norm: f64,
    pub iterations: Vec<CcIteration>,
    pub converged: bool,
}

#[derive(Debug, Clone)]
pub struct CcSeriesEntry {
    pub rank: usize,
    pub result: CcResult,
    pub elapsed: Duration,
}

#[derive(Debug, Error)]
pub enum CcError {
    #[error("CC rank {requested} is outside 1..={maximum}")]
    InvalidRank { requested: usize, maximum: usize },
    #[error("orbital energy length is {actual}, expected {expected}")]
    OrbitalEnergyLength { actual: usize, expected: usize },
    #[error("warm-start length is {actual}, expected {expected}")]
    WarmStartLength { actual: usize, expected: usize },
    #[error("orbital denominator {index} is zero or non-finite")]
    InvalidDenominator { index: usize },
    #[error(transparent)]
    Determinant(#[from] DeterminantError),
    #[error(transparent)]
    Excitation(#[from] ExcitationError),
    #[error(transparent)]
    Cluster(#[from] ClusterError),
    #[error(transparent)]
    Operator(#[from] OperatorError),
    #[error(transparent)]
    Diis(#[from] DiisError),
}

pub fn solve_cc(
    operator: &DirectFciOperator,
    rank: usize,
    orbital_energies: &[f64],
    config: &CcConfig,
) -> Result<CcResult, CcError> {
    let problem = operator.problem();
    let basis = DeterminantBasis::new(problem.norb, problem.nelec, problem.ms2)?;
    validate_rank(rank, &basis)?;
    let reference = hartree_fock_reference(problem.norb, basis.nalpha, basis.nbeta);
    let space = ExcitationSpace::new(&basis, reference, rank)?;
    solve_cc_in_space(operator, &basis, &space, orbital_energies, config, None)
}

pub fn solve_cc_series(
    operator: &DirectFciOperator,
    max_rank: usize,
    orbital_energies: &[f64],
    config: &CcConfig,
) -> Result<Vec<CcSeriesEntry>, CcError> {
    let problem = operator.problem();
    let basis = DeterminantBasis::new(problem.norb, problem.nelec, problem.ms2)?;
    validate_rank(max_rank, &basis)?;
    if orbital_energies.len() != problem.norb {
        return Err(CcError::OrbitalEnergyLength {
            actual: orbital_energies.len(),
            expected: problem.norb,
        });
    }

    let reference = hartree_fock_reference(problem.norb, basis.nalpha, basis.nbeta);
    let mut warm_start = vec![0.0; basis.len()];
    let mut series = Vec::with_capacity(max_rank);
    for rank in 1..=max_rank {
        let space = ExcitationSpace::new(&basis, reference, rank)?;
        let started = Instant::now();
        let result = solve_cc_in_space(
            operator,
            &basis,
            &space,
            orbital_energies,
            config,
            Some(&warm_start),
        )?;
        let elapsed = started.elapsed();
        for (excitation, &amplitude) in space.excitations.iter().zip(&result.amplitudes.values) {
            warm_start[excitation.determinant_index] = amplitude;
        }
        let converged = result.converged;
        series.push(CcSeriesEntry {
            rank,
            result,
            elapsed,
        });
        if !converged {
            break;
        }
    }
    Ok(series)
}

fn validate_rank(rank: usize, basis: &DeterminantBasis) -> Result<(), CcError> {
    let maximum = basis.nalpha + basis.nbeta;
    if rank == 0 || rank > maximum {
        return Err(CcError::InvalidRank {
            requested: rank,
            maximum,
        });
    }
    Ok(())
}

fn solve_cc_in_space(
    operator: &DirectFciOperator,
    basis: &DeterminantBasis,
    space: &ExcitationSpace,
    orbital_energies: &[f64],
    config: &CcConfig,
    warm_start: Option<&[f64]>,
) -> Result<CcResult, CcError> {
    let problem = operator.problem();
    if orbital_energies.len() != problem.norb {
        return Err(CcError::OrbitalEnergyLength {
            actual: orbital_energies.len(),
            expected: problem.norb,
        });
    }
    let denominators = orbital_denominators(space, orbital_energies, problem.norb);
    for (index, &denominator) in denominators.iter().enumerate() {
        if !denominator.is_finite() || denominator.abs() < 1e-12 {
            return Err(CcError::InvalidDenominator { index });
        }
    }
    let mut amplitudes = Amplitudes::zeros(space);
    if let Some(warm_start) = warm_start {
        if warm_start.len() != basis.len() {
            return Err(CcError::WarmStartLength {
                actual: warm_start.len(),
                expected: basis.len(),
            });
        }
        for (value, excitation) in amplitudes.values.iter_mut().zip(&space.excitations) {
            *value = warm_start[excitation.determinant_index];
        }
    }
    let expansion = ClusterExpansionPlan::new(basis, space)?;
    let mut history = Diis::new(config.diis_history);
    let mut iterations = Vec::new();
    let mut previous_energy = f64::INFINITY;

    for iteration in 1..=config.max_iterations {
        let (energy, residual) = energy_and_residual(operator, space, &expansion, &amplitudes)?;
        let residual_norm = residual
            .iter()
            .map(|value| value * value)
            .sum::<f64>()
            .sqrt();
        let energy_change = (energy - previous_energy).abs();
        iterations.push(CcIteration {
            iteration,
            energy,
            energy_change,
            residual_norm,
        });
        if residual_norm <= config.residual_tolerance && energy_change <= config.energy_tolerance {
            return Ok(CcResult {
                energy,
                amplitudes,
                residual_norm,
                iterations,
                converged: true,
            });
        }
        for ((amplitude, residual_value), denominator) in amplitudes
            .values
            .iter_mut()
            .zip(&residual)
            .zip(&denominators)
        {
            *amplitude += residual_value / denominator;
        }
        history.push(&amplitudes.values, &residual)?;
        if let Some(extrapolated) = history.extrapolate() {
            amplitudes.values = extrapolated;
        }
        previous_energy = energy;
    }

    let (energy, residual) = energy_and_residual(operator, space, &expansion, &amplitudes)?;
    let residual_norm = residual
        .iter()
        .map(|value| value * value)
        .sum::<f64>()
        .sqrt();
    Ok(CcResult {
        energy,
        amplitudes,
        residual_norm,
        iterations,
        converged: false,
    })
}

fn energy_and_residual(
    operator: &DirectFciOperator,
    space: &ExcitationSpace,
    expansion: &ClusterExpansionPlan<'_>,
    amplitudes: &Amplitudes,
) -> Result<(f64, Vec<f64>), CcError> {
    let wavefunction = expansion.exponential_on_reference(amplitudes)?;
    let mut h_wavefunction = vec![0.0; operator.dimension()];
    operator.apply(&wavefunction, &mut h_wavefunction)?;
    let energy = h_wavefunction[space.reference_index];
    let residual = space
        .excitations
        .iter()
        .map(|excitation| {
            h_wavefunction[excitation.determinant_index]
                - energy * wavefunction[excitation.determinant_index]
        })
        .collect();
    Ok((energy, residual))
}
