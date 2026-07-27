use std::fs;
use std::path::Path;

use ed_workbench_rs::davidson::{DavidsonConfig, lowest_eigenpair};
use ed_workbench_rs::direct_fci::DirectFciOperator;
use ed_workbench_rs::fcidump::Fcidump;
use ed_workbench_rs::operator::LinearOperator;
use ed_workbench_rs::problem::ElectronicProblem;
use ed_workbench_rs::reference::Reference;

fn davidson_fixture(slug: &str, tolerance: f64) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(slug);
    let dump = Fcidump::parse(&fs::read_to_string(root.join("FCIDUMP")).unwrap()).unwrap();
    let reference = Reference::load(&root.join("reference.json")).unwrap();
    let operator = DirectFciOperator::new(ElectronicProblem::from_fcidump(&dump).unwrap()).unwrap();
    let mut initial = vec![0.0; operator.dimension()];
    let index = operator
        .diagonal()
        .iter()
        .enumerate()
        .min_by(|(_, left), (_, right)| left.total_cmp(right))
        .unwrap()
        .0;
    initial[index] = 1.0;
    let result = lowest_eigenpair(
        &operator,
        &initial,
        &DavidsonConfig {
            residual_tolerance: tolerance,
            energy_tolerance: tolerance * 0.01,
            max_iterations: 100,
            max_subspace: operator.dimension().clamp(2, 24),
        },
    )
    .unwrap();
    assert!(result.converged, "{slug} residual {}", result.residual_norm);
    assert!(
        (result.energy - reference.fci_energy).abs() < tolerance * 10.0,
        "{slug}: Rust {}, PySCF {}",
        result.energy,
        reference.fci_energy
    );
}

#[test]
fn h2_davidson_matches_pyscf() {
    davidson_fixture("h2-sto3g", 1e-11);
}

#[test]
fn h4_davidson_matches_pyscf() {
    davidson_fixture("h4-sto3g", 1e-10);
}

#[test]
fn h2o_sto3g_davidson_matches_pyscf() {
    davidson_fixture("h2o-sto3g", 1e-9);
}
