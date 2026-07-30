use std::fs;
use std::path::PathBuf;

use serde_json::Value;

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn final_article_and_evidence_share_the_same_validated_scope() {
    let repository = repository();
    let c2v: Value = serde_json::from_str(
        &fs::read_to_string(repository.join("fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json"))
            .expect("read C2v evidence"),
    )
    .expect("parse C2v evidence");
    let scnet: Value = serde_json::from_str(
        &fs::read_to_string(repository.join("fixtures/hpc/scnet-2026-07-30.json"))
            .expect("read SCNet evidence"),
    )
    .expect("parse SCNet evidence");
    let article = fs::read_to_string(repository.join("reports/final-competition-summary.md"))
        .expect("read final article");

    assert_eq!(
        c2v["result"]["reported_total_energy_hartree_text"],
        "-76.24321859"
    );
    assert_eq!(c2v["claim_boundary"]["symmetry_free_full_fci"], false);
    assert_eq!(
        c2v["claim_boundary"]["independent_same_hamiltonian_fci_oracle"],
        false
    );
    assert_eq!(c2v["hpc"]["raw_sacct_archived"], false);
    assert_eq!(
        c2v["reproducibility"]["exact_production_source_archived"],
        false
    );
    assert_eq!(scnet["replicate_array"]["observed_peak"]["cpus"], 560);
    assert_eq!(scnet["scope"]["thousand_cpu_observed"], false);

    assert!(article.contains("−76.24321859 Eh"));
    assert!(article.contains("560 allocated CPUs"));
    assert!(article.contains("1,008-CPU campaign design"));
    assert!(article.contains("exact C₂ᵥ/A1 sector"));
    assert!(article.contains("symmetry-free resource characterization"));
    assert!(article.contains("verified SCNet campaign"));
    assert!(article.contains("selected-determinant frontier"));
    assert!(!article.contains("−76.243218589558566"));
}
