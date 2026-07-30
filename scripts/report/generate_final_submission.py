#!/usr/bin/env python3
"""Generate the final competition PDF and plain-text evidence summary."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER, TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import mm
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.pdfbase import pdfmetrics
from reportlab.platypus import (
    BaseDocTemplate,
    Frame,
    KeepTogether,
    PageTemplate,
    Paragraph,
    PageBreak,
    Spacer,
    Table,
    TableStyle,
)


ROOT = Path(__file__).resolve().parents[2]
PDF_PATH = ROOT / "output/pdf/quantum-harness-129-final-technical-report.pdf"
TEXT_PATH = ROOT / "output/data/quantum-harness-129-final-results.txt"
MANIFEST_PATH = ROOT / "output/quantum-harness-129-submission-manifest.txt"
EVIDENCE_COMMIT = "720307ad6ecef3a39b2135ccf7ad53c2962f12ab"
REPO_URL = "https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust"
PR_URL = "https://github.com/QuantumBFS/quantum.harness/pull/217"


def load_json(relative: str) -> dict:
    with (ROOT / relative).open(encoding="utf-8") as handle:
        return json.load(handle)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def register_fonts() -> tuple[str, str]:
    candidates = [
        (
            Path("/System/Library/Fonts/Supplemental/Arial.ttf"),
            Path("/System/Library/Fonts/Supplemental/Arial Bold.ttf"),
        ),
        (
            Path("/System/Library/Fonts/Supplemental/Helvetica.ttf"),
            Path("/System/Library/Fonts/Supplemental/Helvetica Bold.ttf"),
        ),
    ]
    for regular, bold in candidates:
        if regular.exists() and bold.exists():
            pdfmetrics.registerFont(TTFont("SubmissionSans", str(regular)))
            pdfmetrics.registerFont(TTFont("SubmissionSans-Bold", str(bold)))
            return "SubmissionSans", "SubmissionSans-Bold"
    return "Helvetica", "Helvetica-Bold"


def page_decor(canvas, doc) -> None:
    canvas.saveState()
    width, height = A4
    canvas.setFillColor(colors.HexColor("#0B1F33"))
    canvas.rect(0, height - 15 * mm, width, 15 * mm, fill=1, stroke=0)
    canvas.setFillColor(colors.white)
    canvas.setFont(doc.font_bold, 8)
    canvas.drawString(18 * mm, height - 9.5 * mm, "RANGER / QUANTUM HARNESS #129")
    canvas.setFillColor(colors.HexColor("#53687D"))
    canvas.setFont(doc.font_regular, 8)
    canvas.drawString(18 * mm, 10 * mm, "Evidence snapshot: 720307ad6ec")
    canvas.drawRightString(width - 18 * mm, 10 * mm, f"Page {doc.page}")
    canvas.restoreState()


def build_pdf(primary: dict, large: dict, hpc: dict) -> None:
    PDF_PATH.parent.mkdir(parents=True, exist_ok=True)
    regular, bold = register_fonts()
    doc = BaseDocTemplate(
        str(PDF_PATH),
        pagesize=A4,
        leftMargin=18 * mm,
        rightMargin=18 * mm,
        topMargin=23 * mm,
        bottomMargin=18 * mm,
        title="Quantum Harness #129 Final Technical Report",
        author="Ranger: Chenxi Wan, Yedi Shen, Junkai Wang",
        subject="Reproducible Rust exact-diagonalization workbench",
    )
    doc.font_regular = regular
    doc.font_bold = bold
    frame = Frame(doc.leftMargin, doc.bottomMargin, doc.width, doc.height, id="body")
    doc.addPageTemplates(PageTemplate(id="main", frames=[frame], onPage=page_decor))

    styles = getSampleStyleSheet()
    title = ParagraphStyle(
        "TitleCustom", parent=styles["Title"], fontName=bold, fontSize=28,
        leading=32, textColor=colors.HexColor("#0B1F33"), alignment=TA_LEFT,
        spaceAfter=8 * mm,
    )
    subtitle = ParagraphStyle(
        "Subtitle", parent=styles["Normal"], fontName=regular, fontSize=13,
        leading=18, textColor=colors.HexColor("#53687D"), spaceAfter=8 * mm,
    )
    h1 = ParagraphStyle(
        "H1Custom", parent=styles["Heading1"], fontName=bold, fontSize=18,
        leading=22, textColor=colors.HexColor("#0B1F33"), spaceBefore=4 * mm,
        spaceAfter=3 * mm,
    )
    h2 = ParagraphStyle(
        "H2Custom", parent=styles["Heading2"], fontName=bold, fontSize=12,
        leading=15, textColor=colors.HexColor("#126E82"), spaceBefore=3 * mm,
        spaceAfter=2 * mm,
    )
    body = ParagraphStyle(
        "BodyCustom", parent=styles["BodyText"], fontName=regular, fontSize=9.3,
        leading=13.2, textColor=colors.HexColor("#263746"), spaceAfter=2.5 * mm,
    )
    small = ParagraphStyle(
        "Small", parent=body, fontSize=7.8, leading=10.5,
        textColor=colors.HexColor("#53687D"),
    )
    metric = ParagraphStyle(
        "Metric", parent=body, fontName=bold, fontSize=12, leading=15,
        textColor=colors.HexColor("#0B1F33"), alignment=TA_CENTER,
    )

    def p(text: str, style=body) -> Paragraph:
        return Paragraph(text, style)

    def bullet(text: str) -> Paragraph:
        return Paragraph(f"- {text}", body)

    def table(rows, widths=None, header=True) -> Table:
        formatted = [[p(str(cell), small) for cell in row] for row in rows]
        result = Table(formatted, colWidths=widths, repeatRows=1 if header else 0, hAlign="LEFT")
        commands = [
            ("VALIGN", (0, 0), (-1, -1), "TOP"),
            ("GRID", (0, 0), (-1, -1), 0.35, colors.HexColor("#C8D3DC")),
            ("LEFTPADDING", (0, 0), (-1, -1), 5),
            ("RIGHTPADDING", (0, 0), (-1, -1), 5),
            ("TOPPADDING", (0, 0), (-1, -1), 5),
            ("BOTTOMPADDING", (0, 0), (-1, -1), 5),
        ]
        if header:
            commands.extend([
                ("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#0B1F33")),
                ("TEXTCOLOR", (0, 0), (-1, 0), colors.white),
            ])
            for item in formatted[0]:
                item.style = ParagraphStyle("TableHeader", parent=small, fontName=bold, textColor=colors.white)
        for row in range(1 if header else 0, len(rows)):
            if row % 2 == 0:
                commands.append(("BACKGROUND", (0, row), (-1, row), colors.HexColor("#F1F5F7")))
        result.setStyle(TableStyle(commands))
        return result

    fci = large["result"]
    scope = large["scientific_scope"]
    replicate = hpc["replicate_array"]
    robustness = hpc["robustness_array"]

    story = [
        Spacer(1, 17 * mm),
        p("FINAL TECHNICAL REPORT", small),
        p("A Reproducible Rust Ladder from CC(8) to 451 Million Determinants", title),
        p(
            "Quantum Harness Challenge #129 / Team Ranger<br/>"
            "Chenxi Wan, Yedi Shen, Junkai Wang<br/>"
            "Submission date: 2026-07-30",
            subtitle,
        ),
        Table(
            [[p("PRIMARY", small), p("LARGEST CONVERGED", small), p("HPC EVIDENCE", small)],
             [p("245,025 determinants", metric), p("451,681,246 determinants", metric), p("560 CPUs observed", metric)]],
            colWidths=[doc.width / 3] * 3,
            style=TableStyle([
                ("BACKGROUND", (0, 0), (-1, -1), colors.HexColor("#E8F1F3")),
                ("BOX", (0, 0), (-1, -1), 0.8, colors.HexColor("#126E82")),
                ("INNERGRID", (0, 0), (-1, -1), 0.4, colors.HexColor("#B7CDD2")),
                ("ALIGN", (0, 0), (-1, -1), "CENTER"),
                ("VALIGN", (0, 0), (-1, -1), "MIDDLE"),
                ("TOPPADDING", (0, 0), (-1, -1), 8),
                ("BOTTOMPADDING", (0, 0), (-1, -1), 8),
            ]),
        ),
        Spacer(1, 9 * mm),
        p("Outcome", h1),
        p(
            "The mandatory Level 0-4 challenge is complete. The checked Rust implementation "
            "covers determinant construction, matrix-free FCI, arbitrary-rank CC through CC(8), "
            "CI, MBPT, UCC, direct libcint integrals, Rust RHF/DIIS, symmetry sectors, multi-root "
            "Davidson, deterministic parallel sigma, and restartable disk checkpoints. All 36 "
            "published primary entries match at the source paper's printed precision.",
        ),
        p(
            "This PDF is a compact submission artifact. The repository's Markdown reports, JSON "
            "fixtures, raw logs, checksums, source, tests, and verifier remain authoritative.",
            small,
        ),
        PageBreak(),
        p("1. Acceptance result", h1),
        p(
            "The primary Hamiltonian is H2O/6-31G with the oxygen 1s orbital frozen, 12 active "
            "spatial orbitals, eight active electrons, and 245,025 determinants. Matrix-free "
            f"Davidson FCI converges to {primary['fci_energy']:.15f} Eh.",
        ),
        table([
            ["Required family", "Delivered", "Validation"],
            ["CC(n)", "CC(1)-CC(8)", "8/8 published entries match"],
            ["CI(n)", "CI(1)-CI(8)", "8/8 published entries match"],
            ["MBPT(n)", "MBPT(1)-MBPT(20)", "20/20 published entries match"],
            ["FCI", "Dense and matrix-free Davidson", "Small independent oracles + primary residual"],
            ["Direct integrals", "libcint -> Rust RHF -> AO-to-MO -> FCI", "Shared FCI engine and fixtures"],
        ], widths=[38 * mm, 54 * mm, 75 * mm]),
        p("Acceptance interpretation", h2),
        p(
            "Basic requirement completion is 100%, not a forecast. Relative to only the mandatory "
            "acceptance ladder, the project also delivers substantial extensions. A numeric "
            "'over-completion percentage' is not an official score, so this report lists extra "
            "capabilities instead of inventing grading points.",
        ),
        p("Representative primary values", h2),
        table([
            ["Quantity", "Value"],
            ["FCI", f"{primary['fci_energy']:.15f} Eh"],
            ["CC(2) / CCSD", f"{primary['results'][1]['energy']:.15f} Eh"],
            ["CC(8)", f"{primary['results'][-1]['energy']:.15f} Eh"],
            ["CC(8) - FCI", f"{primary['results'][-1]['method_minus_fci']:.3e} Eh"],
        ], widths=[70 * mm, 90 * mm]),
        PageBreak(),
        p("2. Largest completed exact sector", h1),
        p(
            "The largest converged run is all-electron H2O/cc-pVDZ in the exact C2v/A1 "
            "ground-state block. Point-group block diagonalization changes representation size, "
            "not the Hamiltonian or determinant completeness inside A1.",
        ),
        table([
            ["Quantity", "Recorded value"],
            ["Spatial orbitals / electrons", "24 spherical / 10 all electron"],
            ["Determinants in C2v/A1", f"{scope['determinants']:,}"],
            ["Reported FCI energy", f"{fci['reported_total_energy_hartree_text']} Eh"],
            ["Residual norm", f"{fci['residual_norm']:.3e}"],
            ["Davidson iterations", str(fci['iterations'])],
            ["Wall time", large['hpc']['elapsed']],
            ["Requested memory", f"{large['hpc']['memory_request_gib']} GiB"],
            ["Scheduler MaxRSS", "222.257 GiB, transcribed; raw accounting unavailable"],
        ], widths=[62 * mm, 105 * mm]),
        p("Exact scope boundary", h2),
        p(
            "The symmetry-free representation contains 1,806,590,016 determinants and was "
            "bounded and kernel-benchmarked, but it was not solved to convergence. The public "
            "eight-decimal energy is scientifically accepted within the recorded residual. The "
            "exact production direct_fci.rs and raw Slurm accounting row are not archived, so "
            "production provenance is explicitly incomplete.",
        ),
        PageBreak(),
        p("3. Algorithmic and engineering contributions", h1),
        p(
            "These are project contributions and implementation advances. They should not be "
            "described as field-wide novel inventions without a separate literature novelty review.",
            small,
        ),
        *[
            KeepTogether([p(title_text, h2), p(detail)])
            for title_text, detail in [
                ("Exact ranked subset-convolution CC exponential",
                 "Computes exp(T)|HF> by ranked subset convolution and supports arbitrary determinant CC(n) through CC(8). A finite Taylor implementation remains a small-system oracle. This removes the previous all-amplitude exponential bottleneck and makes full-rank validation practical."),
                ("Matrix-free spin-free sigma FCI",
                 "Applies the Hamiltonian without materializing the determinant-space matrix. Restarted Davidson, independent diagonals, memory preflight, and checked index arithmetic allow the same core to span tiny dense oracles and very large sectors."),
                ("Deterministic bounded-memory parallel sigma",
                 "Uses fixed source blocks, thread-local partial vectors, and an ordered reduction. It trades some memory and peak speed for reproducibility. The Apple M4 primary benchmark measured a 3.236817x median serial-to-parallel time ratio."),
                ("Symmetry propagated through the method stack",
                 "Compact Abelian point-group sector enumeration is carried through FCI, CI, MBPT, CC, and UCC. For cc-pVDZ it reduces the exact target representation from 1.806 billion to 451.681 million determinants, a fourfold reduction without discarding an A1 determinant."),
                ("Robust Davidson infrastructure",
                 "Versioned disk checkpoints use atomic state updates, input/configuration hashes, corruption rejection, and safe resume. Multi-root block Davidson and general active-space combinadic rank/unrank extend the solver beyond one ground-state fixture."),
                ("Fail-closed evidence design",
                 "Machine-readable evidence records accepted quantities and rejected claim boundaries. The final verifier recomputes hashes and refuses to turn requested resources, unverified accounting, or future methods into completed results."),
            ]
        ],
        PageBreak(),
        p("4. HPC evidence and efficiency", h1),
        p(
            "SCNet validates portability, robustness, deterministic repeated solves, and ensemble "
            "throughput. It does not demonstrate MPI scaling of one eigenproblem.",
        ),
        table([
            ["Evidence", "Result"],
            ["Robustness grid", f"{len(robustness['cases'])}/{len(robustness['cases'])} converged"],
            ["Repeated solves", f"{replicate['sample_count']}/{replicate['sample_count']} converged"],
            ["Repeated case determinism", str(replicate['all_case_energies_deterministic']).lower()],
            ["Observed allocation peak", f"{replicate['observed_peak']['cpus']} CPUs across {replicate['observed_peak']['tasks']} tasks"],
            ["Requested but not observed", "1,008 CPUs"],
            ["Evidence files / manifests", f"{hpc['provenance']['downloaded_evidence_files']} / {hpc['provenance']['verified_sha256_manifests']}"],
        ], widths=[70 * mm, 97 * mm]),
        p("Optimization conclusion", h2),
        p(
            "For the 245,025-dimensional case, effective busy cores saturated well below a "
            "56-CPU allocation. Packing independent 14-thread processes is therefore the correct "
            "throughput optimization for the submitted ensemble. For one much larger FCI solve, "
            "exact point-group reduction and memory-aware Davidson storage provide more value "
            "than simply requesting more cores.",
        ),
        PageBreak(),
        p("5. Reproduction and artifact map", h1),
        p("Run the complete local acceptance gate:", h2),
        p("<font name='Courier'>uv sync --locked<br/>scripts/verify-submission.sh</font>"),
        p("Run the lightweight evidence audit:", h2),
        p("<font name='Courier'>python3 scripts/hpc/verify_final_evidence.py</font>"),
        table([
            ["Artifact", "Path"],
            ["Narrative report", "reports/final-competition-summary.md"],
            ["This PDF", "output/pdf/quantum-harness-129-final-technical-report.pdf"],
            ["Plain-text results", "output/data/quantum-harness-129-final-results.txt"],
            ["Large FCI JSON", "fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json"],
            ["SCNet JSON", "fixtures/hpc/scnet-2026-07-30.json"],
            ["Reproduction prompt", "docs/reproduction-prompt.md"],
            ["Evidence verifier", "scripts/hpc/verify_final_evidence.py"],
        ], widths=[46 * mm, 121 * mm]),
        p("Public endpoints", h2),
        p(f"Repository: {REPO_URL}<br/>Upstream PR: {PR_URL}<br/>Evidence snapshot: {EVIDENCE_COMMIT}"),
        p("5-hour-to-finish lesson", h2),
        p(
            "Freeze numerical evidence first; run only missing bounded checks locally; use HPC only "
            "for already-prepared, auditable workloads; write claims directly from machine-readable "
            "artifacts; then generate the PDF, manifest, and PR index from the same frozen evidence. "
            "Do not start a new billion-dimensional calculation during submission packaging.",
        ),
        PageBreak(),
        p("6. Claim ledger and next research step", h1),
        table([
            ["Claim", "Status"],
            ["Mandatory Level 0-4 challenge", "Complete"],
            ["36 primary published entries", "36/36 match at printed precision"],
            ["C2v/A1 451M determinant FCI", "Converged and accepted"],
            ["Symmetry-free 1.806B determinant FCI", "Not converged; bounded benchmark only"],
            ["Observed thousand-CPU run", "Not observed; 560-CPU peak is public"],
            ["Single-solve MPI scaling", "Not implemented"],
            ["HCI/iCI, EN-PT2, orbital optimization", "Future work; not implemented"],
            ["Quantum advantage", "Not claimed"],
        ], widths=[76 * mm, 91 * mm]),
        p("Recommended next algorithm", h2),
        p(
            "Add a common selected-determinant interface, deterministic heat-bath or iCI-style "
            "selection, a variational solve, EN-PT2 with an explicit error budget, and threshold "
            "extrapolation. Then compare natural and orbital-optimized bases on stretched water. "
            "The current exact solver supplies the calibration oracle and makes those future "
            "approximations scientifically testable.",
        ),
        p("Submission decision", h2),
        p(
            "Submit the existing corrective branch and PR. The basic challenge is already complete; "
            "the remaining work is packaging, CI confirmation, and reviewer communication. Avoid "
            "adding unverified claims or moving the immutable v0.5.0 tag.",
        ),
    ]
    doc.build(story)


def build_text(primary: dict, large: dict, hpc: dict) -> None:
    TEXT_PATH.parent.mkdir(parents=True, exist_ok=True)
    result = large["result"]
    scope = large["scientific_scope"]
    repeat = hpc["replicate_array"]
    lines = [
        "QUANTUM HARNESS #129 - FINAL RESULTS",
        "====================================",
        "Date: 2026-07-30",
        "Team: Ranger (Chenxi Wan, Yedi Shen, Junkai Wang)",
        f"Repository: {REPO_URL}",
        f"Upstream PR: {PR_URL}",
        f"Validated evidence commit: {EVIDENCE_COMMIT}",
        "",
        "MANDATORY ACCEPTANCE",
        "--------------------",
        "Status: COMPLETE (100%)",
        "System: H2O/6-31G, oxygen 1s frozen, 12 active orbitals, 8 active electrons",
        f"Determinants: {primary['determinants']:,}",
        f"FCI energy (Eh): {primary['fci_energy']:.15f}",
        "CC: 8/8 published entries match (CC(1)-CC(8))",
        "CI: 8/8 published entries match (CI(1)-CI(8))",
        "MBPT: 20/20 published entries match (MBPT(1)-MBPT(20))",
        "Published entries matched in total: 36/36",
        "",
        "LARGEST CONVERGED EXACT SECTOR",
        "------------------------------",
        "System: H2O/cc-pVDZ, all electron, C2v/A1",
        f"Determinants: {scope['determinants']:,}",
        f"Reported FCI energy (Eh): {result['reported_total_energy_hartree_text']}",
        f"Residual norm: {result['residual_norm']:.3e}",
        f"Davidson iterations: {result['iterations']}",
        f"Wall time: {large['hpc']['elapsed']}",
        f"Job state / exit: {large['hpc']['state']} / {large['hpc']['exit_code']}",
        "Provenance status: scientific result accepted; production provenance incomplete",
        "",
        "HPC EVIDENCE",
        "------------",
        f"Robustness cases: {len(hpc['robustness_array']['cases'])}/{len(hpc['robustness_array']['cases'])}",
        f"Repeated solves: {repeat['sample_count']}/{repeat['sample_count']}",
        f"Observed peak CPUs: {repeat['observed_peak']['cpus']}",
        "Requested but not observed: 1,008 CPUs",
        "Interpretation: ensemble throughput, not MPI scaling of one solve",
        "",
        "IMPLEMENTED CONTRIBUTIONS",
        "-------------------------",
        "1. Exact ranked subset-convolution recurrence for determinant CC(n) through CC(8).",
        "2. Matrix-free spin-free sigma FCI with restarted Davidson.",
        "3. Deterministic fixed-block parallel sigma with ordered reduction.",
        "4. Compact point-group symmetry sectors propagated through FCI/CI/MBPT/CC/UCC.",
        "5. Versioned disk-backed checkpoint/resume with atomic state and hash guards.",
        "6. Multi-root block Davidson and general active-space combinadic indexing.",
        "7. Direct libcint -> Rust RHF/DIIS -> AO-to-MO -> FCI path.",
        "8. Fail-closed, machine-readable evidence and claim boundaries.",
        "",
        "NOT CLAIMED",
        "-----------",
        "- Converged symmetry-free 1,806,590,016-determinant FCI",
        "- Observed 1,008-CPU execution or single-solve MPI scaling",
        "- HCI/iCI, EN-PT2, natural orbitals, orbital optimization, or quantum advantage",
        "",
        "REPRODUCTION",
        "------------",
        "uv sync --locked",
        "scripts/verify-submission.sh",
        "python3 scripts/hpc/verify_final_evidence.py",
        "",
    ]
    TEXT_PATH.write_text("\n".join(lines), encoding="utf-8")


def build_manifest() -> None:
    entries = [
        ROOT / "reports/final-competition-summary.md",
        PDF_PATH,
        TEXT_PATH,
        ROOT / "fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json",
        ROOT / "fixtures/hpc/scnet-2026-07-30.json",
        ROOT / "docs/reproduction-prompt.md",
        ROOT / "scripts/hpc/verify_final_evidence.py",
    ]
    lines = [
        "QUANTUM HARNESS #129 - SUBMISSION MANIFEST",
        "==========================================",
        f"Validated evidence commit: {EVIDENCE_COMMIT}",
        "Format: SHA256  BYTES  PATH",
        "",
    ]
    for path in entries:
        lines.append(f"{sha256(path)}  {path.stat().st_size}  {path.relative_to(ROOT)}")
    lines.append("")
    MANIFEST_PATH.write_text("\n".join(lines), encoding="utf-8")


def main() -> None:
    primary = load_json("fixtures/h2o-631g-fc/cc_series_results.json")
    large = load_json("fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json")
    hpc = load_json("fixtures/hpc/scnet-2026-07-30.json")
    build_pdf(primary, large, hpc)
    build_text(primary, large, hpc)
    build_manifest()
    print(PDF_PATH.relative_to(ROOT))
    print(TEXT_PATH.relative_to(ROOT))
    print(MANIFEST_PATH.relative_to(ROOT))


if __name__ == "__main__":
    main()
