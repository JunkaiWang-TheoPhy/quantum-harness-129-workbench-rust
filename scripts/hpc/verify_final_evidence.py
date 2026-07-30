#!/usr/bin/env python3
"""Fail-closed audit of the final competition evidence and claim boundaries."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path


REPOSITORY = Path(__file__).resolve().parents[2]
C2V_RESULT = REPOSITORY / "fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json"
SCNET_RESULT = REPOSITORY / "fixtures/hpc/scnet-2026-07-30.json"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(f"final evidence verification failed: {message}")


def load_json(path: Path) -> dict:
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def verify_c2v() -> None:
    evidence = load_json(C2V_RESULT)
    result = evidence["result"]
    claims = evidence["claim_boundary"]
    hpc = evidence["hpc"]
    reproducibility = evidence["reproducibility"]
    acceptance = evidence["acceptance"]

    require(evidence["scientific_scope"]["determinants"] == 451_681_246,
            "C2v determinant count changed")
    require(result["reported_total_energy_hartree_text"] == "-76.24321859",
            "public C2v energy must use the accepted precision boundary")
    require(abs(result["total_energy_hartree"] - (-76.24321859)) < 5.0e-9,
            "stored C2v energy is inconsistent with the reported value")
    require(result["converged"], "C2v result is not converged")
    require(result["residual_norm"] <= evidence["solver"]["residual_tolerance"],
            "C2v residual exceeds its requested tolerance")

    require(claims["symmetry_adapted_full_fci"],
            "C2v full-FCI scope is not asserted")
    require(not claims["symmetry_free_full_fci"],
            "symmetry-free full FCI must remain unclaimed")
    require(not claims["independent_same_hamiltonian_fci_oracle"],
            "the lower-level PySCF checks must not be called an FCI oracle")
    require(not claims["thousand_cpu_single_solve"],
            "a thousand-CPU single solve was not demonstrated")

    require(hpc["state"] == "COMPLETED" and hpc["exit_code"] == "0:0",
            "recorded Slurm completion changed")
    require(not hpc["raw_sacct_archived"],
            "raw sacct may only be claimed when the artifact is committed")
    require(not hpc["max_rss_verified_from_raw_accounting"],
            "MaxRSS may not be verified without raw accounting")
    require(not reproducibility["exact_production_source_archived"],
            "production source gap must remain disclosed")
    require(not acceptance["provenance_complete"],
            "incomplete provenance must not be marked complete")

    expected_hashes = {
        "fixtures/h2o-ccpvdz-ae/FCIDUMP.c2v": evidence["input"]["sha256"],
        hpc["stdout"]: reproducibility["stdout_sha256"],
        hpc["stderr"]: reproducibility["stderr_sha256"],
        "hpc/xh5/production.slurm": reproducibility["production_slurm_sha256"],
    }
    for relative, expected in expected_hashes.items():
        path = REPOSITORY / relative
        require(path.is_file(), f"missing C2v artifact: {relative}")
        require(sha256(path) == expected, f"checksum mismatch: {relative}")

    archived_source = sha256(REPOSITORY / "src/direct_fci.rs")
    require(archived_source == reproducibility[
                "archived_src_direct_fci_sha256_after_rustfmt"],
            "archived direct_fci.rs checksum changed")
    require(archived_source != reproducibility["production_source_sha256"]
            ["src/direct_fci.rs"],
            "source-gap declaration is inconsistent with the hashes")


def verify_scnet() -> None:
    evidence = load_json(SCNET_RESULT)
    scope = evidence["scope"]
    robustness = evidence["robustness_array"]
    replicates = evidence["replicate_array"]

    require(not scope["single_solve_mpi"],
            "SCNet workload must remain described as an ensemble")
    require(scope["thousand_cpu_request_submitted"],
            "the recorded 1008-CPU request disappeared")
    require(not scope["thousand_cpu_observed"],
            "1008 CPUs must not be claimed as observed")
    require(robustness["all_completed"] and len(robustness["cases"]) == 18,
            "SCNet robustness matrix is incomplete")
    require(all(case["converged"] for case in robustness["cases"]),
            "a SCNet robustness case did not converge")
    require(replicates["all_completed"] and replicates["all_converged"],
            "SCNet replicate ensemble is incomplete")
    require(replicates["sample_count"] == 216,
            "SCNet replicate count changed")
    require(replicates["observed_peak"]["cpus"] == 560,
            "SCNet observed CPU peak changed")
    require(replicates["requested_max_cpus"] == 1008,
            "SCNet requested CPU ceiling changed")


def verify_public_wording() -> None:
    public_files = [
        REPOSITORY / "README.md",
        REPOSITORY / "reports/h2o-ccpvdz-c2v-fci.md",
        REPOSITORY / "reports/scnet-hpc-benchmark.md",
    ]
    forbidden = {
        "-76.243218589558566": "over-precise C2v headline energy",
        "−76.243218589558566": "over-precise C2v headline energy",
        "1008 CPUs observed": "unobserved 1008-CPU claim",
        "1,008 CPUs observed": "unobserved 1008-CPU claim",
        "thousand-core FCI": "ambiguous thousand-core FCI claim",
        "no-symmetry full FCI completed": "unperformed symmetry-free solve",
    }
    for path in public_files:
        text = path.read_text(encoding="utf-8")
        for phrase, description in forbidden.items():
            require(phrase.lower() not in text.lower(),
                    f"{description} in {path.relative_to(REPOSITORY)}")


def main() -> None:
    verify_c2v()
    verify_scnet()
    verify_public_wording()
    print(
        "final evidence verified: C2v/A1 FCI accepted at -76.24321859 Eh; "
        "560 CPUs observed; 1008 requested but not observed; provenance gaps disclosed"
    )


if __name__ == "__main__":
    main()
