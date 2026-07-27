use std::fs;
use std::path::Path;

use ed_workbench_rs::coupled_cluster::{CcConfig, solve_cc};
use ed_workbench_rs::direct_fci::DirectFciOperator;
use ed_workbench_rs::fcidump::Fcidump;
use ed_workbench_rs::problem::ElectronicProblem;
use ed_workbench_rs::reference::Reference;

fn cc_fixture(slug: &str, rank: usize, tolerance: f64) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(slug);
    let dump = Fcidump::parse(&fs::read_to_string(root.join("FCIDUMP")).unwrap()).unwrap();
    let reference = Reference::load(&root.join("reference.json")).unwrap();
    let operator = DirectFciOperator::new(ElectronicProblem::from_fcidump(&dump).unwrap()).unwrap();
    let result = solve_cc(
        &operator,
        rank,
        &reference.active_orbital_energies,
        &CcConfig {
            residual_tolerance: tolerance,
            energy_tolerance: tolerance * 0.01,
            max_iterations: 100,
            diis_history: 6,
            exponential_threshold: 1e-14,
        },
    )
    .unwrap();
    assert!(result.converged, "{slug} residual {}", result.residual_norm);
    let expected = if rank >= dump.nelec {
        reference.fci_energy
    } else {
        reference.ccsd_total_energy.unwrap()
    };
    assert!(
        (result.energy - expected).abs() < tolerance * 10.0,
        "{slug} CC({rank}): Rust {}, expected {}",
        result.energy,
        expected
    );
}

#[test]
fn h2_cc2_matches_fci_and_ccsd() {
    cc_fixture("h2-sto3g", 2, 1e-9);
}

#[test]
fn equilibrium_h2_cc2_matches_fci_and_ccsd() {
    cc_fixture("h2-equilibrium-sto3g", 2, 1e-9);
}

#[test]
fn h4_cc2_matches_pyscf_ccsd() {
    cc_fixture("h4-sto3g", 2, 1e-8);
}

#[test]
fn h4_full_rank_cc_matches_fci() {
    cc_fixture("h4-sto3g", 4, 1e-8);
}

#[test]
fn h2o_sto3g_cc2_matches_pyscf_ccsd() {
    cc_fixture("h2o-sto3g", 2, 1e-7);
}
