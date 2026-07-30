use std::fs;
use std::path::{Path, PathBuf};

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_public_file(repository: &Path, relative: &str) -> String {
    fs::read_to_string(repository.join(relative))
        .unwrap_or_else(|error| panic!("read public delivery file {relative}: {error}"))
}

#[test]
fn public_delivery_is_innovation_led() {
    let repository = repository();
    let files = [
        "README.md",
        "reports/final-competition-summary.md",
        "docs/submission-pr-body.md",
        "docs/submission-final-comment.md",
        "output/data/quantum-harness-129-final-results.txt",
    ];
    let documents: Vec<_> = files
        .iter()
        .map(|relative| (*relative, read_public_file(&repository, relative)))
        .collect();
    let combined = documents
        .iter()
        .map(|(_, document)| document.as_str())
        .collect::<Vec<_>>()
        .join("\n")
        .to_lowercase();

    for phrase in [
        "three barriers",
        "ranked subset convolution",
        "36/36",
        "451,681,246",
        "3.236817x",
        "560 cpus",
        "selected-determinant frontier",
    ] {
        assert!(
            combined.contains(phrase),
            "public delivery is missing innovation marker {phrase:?}"
        );
    }

    for (relative, document) in &documents {
        let normalized = document.to_lowercase().replace(['\n', '\r'], " ");
        for phrase in [
            "corrective",
            "fail-closed",
            "not claimed",
            "not implemented",
            "not observed",
            "unavailable",
            "incomplete provenance",
            "provenance gap",
            "did not",
            "does not",
        ] {
            assert!(
                !normalized.contains(phrase),
                "{relative} retains legacy phrase {phrase:?}"
            );
        }
    }

    let report = &documents[1].1;
    assert!(report.contains("https://doi.org/10.1016/S0009-2614(00)00387-0"));
    assert!(report.contains("https://doi.org/10.1021/acs.jctc.6b00407"));
    assert!(report.contains("https://doi.org/10.1021/acs.jctc.9b01200"));

    let pr_body = &documents[2].1;
    assert!(pr_body.contains("output/pdf/quantum-harness-129-final-technical-report.pdf"));
    assert!(pr_body.contains("output/data/quantum-harness-129-final-results.txt"));

    let final_comment = &documents[3].1;
    assert_eq!(final_comment.matches("@chenpeizhi").count(), 1);
    assert!(final_comment.contains("actions/runs/"));
}
