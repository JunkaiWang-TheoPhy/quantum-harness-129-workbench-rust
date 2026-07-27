use std::path::Path;
use std::process::Command;

#[test]
fn cc_series_cli_reports_every_requested_rank() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/h4-sto3g");
    let output = Command::new(env!("CARGO_BIN_EXE_ed_workbench_rs"))
        .arg("cc-series")
        .arg(root.join("FCIDUMP"))
        .arg(root.join("reference.json"))
        .arg("--max-rank")
        .arg("2")
        .arg("--residual-tolerance")
        .arg("1e-8")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(
        "rank\tenergy_hartree\tmethod_minus_fci_hartree\titerations\tresidual\telapsed_seconds\tconverged"
    ));
    assert!(stdout.lines().any(|line| line.starts_with("1\t")));
    assert!(stdout.lines().any(|line| line.starts_with("2\t")));
    assert!(stdout.contains("series converged: true"));
}
