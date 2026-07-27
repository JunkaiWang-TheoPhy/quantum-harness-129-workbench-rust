use std::path::Path;
use std::process::Command;

#[test]
fn level3_series_cli_reports_ci_and_mbpt_orders() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/h4-sto3g");
    let output = Command::new(env!("CARGO_BIN_EXE_ed_workbench_rs"))
        .arg("level3-series")
        .arg(root.join("FCIDUMP"))
        .arg(root.join("reference.json"))
        .arg("--max-ci-rank")
        .arg("4")
        .arg("--max-mbpt-order")
        .arg("4")
        .arg("--ci-residual-tolerance")
        .arg("1e-9")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CI series"));
    assert!(stdout.contains("MBPT series"));
    assert!(stdout.lines().any(|line| line.starts_with("CI\t4\t")));
    assert!(stdout.lines().any(|line| line.starts_with("MBPT\t4\t")));
    assert!(stdout.contains("CI series converged: true"));
}
