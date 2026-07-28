use std::fs;
use std::process::Command;

use ed_workbench_rs::benchmark::BoundedBenchmarkResult;

#[test]
fn cc_pvdz_benchmark_rejects_budget_before_large_allocations() {
    let output = Command::new(env!("CARGO_BIN_EXE_ed_workbench_rs"))
        .args([
            "benchmark",
            "h2o-cc-pvdz",
            "--sources",
            "1",
            "--max-memory-gib",
            "0.5",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("exceeds budget"), "{stderr}");
}

#[test]
#[ignore = "live cc-pVDZ integral/link benchmark; run explicitly in release mode"]
fn live_cc_pvdz_benchmark_is_bounded_and_matches_pyscf_rhf() {
    let output_path = std::env::temp_dir().join(format!(
        "ed-workbench-h2o-ccpvdz-{}.json",
        std::process::id()
    ));
    let output = Command::new(env!("CARGO_BIN_EXE_ed_workbench_rs"))
        .args([
            "benchmark",
            "h2o-cc-pvdz",
            "--sources",
            "1",
            "--max-memory-gib",
            "2",
            "--json-output",
        ])
        .arg(&output_path)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: BoundedBenchmarkResult =
        serde_json::from_slice(&fs::read(&output_path).unwrap()).unwrap();
    fs::remove_file(output_path).unwrap();

    assert_eq!(result.norb, 24);
    assert_eq!(result.nelec, 10);
    assert_eq!(result.nalpha, 5);
    assert_eq!(result.nbeta, 5);
    assert_eq!(result.space.determinants, 1_806_590_016);
    assert!(result.rhf_converged);
    assert!(result.rhf_absolute_error < 1e-8);
    assert!(!result.point_group_symmetry);
    assert!(!result.full_fci_executed);
    assert!(result.bounded_memory.conservative_peak_bytes < result.memory_budget_bytes);
}
