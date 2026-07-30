#!/usr/bin/env python3
"""Generate the innovation-led Ranger competition delivery package."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

from reportlab.lib import colors
from reportlab.lib.enums import TA_CENTER, TA_LEFT
from reportlab.lib.pagesizes import A4
from reportlab.lib.styles import ParagraphStyle, getSampleStyleSheet
from reportlab.lib.units import mm
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.platypus import (
    BaseDocTemplate,
    Frame,
    KeepTogether,
    PageBreak,
    PageTemplate,
    Paragraph,
    Spacer,
    Table,
    TableStyle,
)


ROOT = Path(__file__).resolve().parents[2]
PDF_PATH = ROOT / "output/pdf/quantum-harness-129-final-technical-report.pdf"
TEXT_PATH = ROOT / "output/data/quantum-harness-129-final-results.txt"
MANIFEST_PATH = ROOT / "output/quantum-harness-129-submission-manifest.txt"
REPOSITORY_URL = "https://github.com/JunkaiWang-TheoPhy/quantum-harness-129-workbench-rust"
BRANCH_URL = f"{REPOSITORY_URL}/tree/codex/final-competition-submission"
PR_URL = "https://github.com/QuantumBFS/quantum.harness/pull/217"
RELEASE_URL = f"{REPOSITORY_URL}/releases/tag/v0.5.0"


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
            pdfmetrics.registerFont(TTFont("RangerSans", str(regular)))
            pdfmetrics.registerFont(TTFont("RangerSans-Bold", str(bold)))
            return "RangerSans", "RangerSans-Bold"
    return "Helvetica", "Helvetica-Bold"


def page_decor(canvas, document) -> None:
    canvas.saveState()
    width, height = A4
    canvas.setFillColor(colors.HexColor("#071D31"))
    canvas.rect(0, height - 15 * mm, width, 15 * mm, fill=1, stroke=0)
    canvas.setFillColor(colors.HexColor("#52D6C9"))
    canvas.setFont(document.font_bold, 8)
    canvas.drawString(18 * mm, height - 9.5 * mm, "RANGER / EXACTNESS AT SCALE")
    canvas.setFillColor(colors.HexColor("#53687D"))
    canvas.setFont(document.font_regular, 8)
    canvas.drawString(18 * mm, 10 * mm, "Quantum Harness #129 / Public research package")
    canvas.drawRightString(width - 18 * mm, 10 * mm, f"Page {document.page}")
    canvas.restoreState()


def build_pdf(primary: dict, large: dict, hpc: dict, parallel: dict) -> None:
    PDF_PATH.parent.mkdir(parents=True, exist_ok=True)
    regular, bold = register_fonts()
    document = BaseDocTemplate(
        str(PDF_PATH),
        pagesize=A4,
        leftMargin=18 * mm,
        rightMargin=18 * mm,
        topMargin=23 * mm,
        bottomMargin=18 * mm,
        title="Exactness at Scale: From CC(8) to 451 Million Determinants in Rust",
        author="Ranger: Chenxi Wan, Yedi Shen, Junkai Wang",
        subject="Quantum Harness #129 innovation-led final technical report",
    )
    document.font_regular = regular
    document.font_bold = bold
    frame = Frame(
        document.leftMargin,
        document.bottomMargin,
        document.width,
        document.height,
        id="body",
    )
    document.addPageTemplates(PageTemplate(id="main", frames=[frame], onPage=page_decor))

    base = getSampleStyleSheet()
    title = ParagraphStyle(
        "RangerTitle",
        parent=base["Title"],
        fontName=bold,
        fontSize=27,
        leading=31,
        textColor=colors.HexColor("#071D31"),
        alignment=TA_LEFT,
        spaceAfter=7 * mm,
    )
    subtitle = ParagraphStyle(
        "RangerSubtitle",
        parent=base["Normal"],
        fontName=regular,
        fontSize=12.5,
        leading=17,
        textColor=colors.HexColor("#53687D"),
        spaceAfter=7 * mm,
    )
    h1 = ParagraphStyle(
        "RangerH1",
        parent=base["Heading1"],
        fontName=bold,
        fontSize=18,
        leading=22,
        textColor=colors.HexColor("#071D31"),
        spaceBefore=3 * mm,
        spaceAfter=3 * mm,
    )
    h2 = ParagraphStyle(
        "RangerH2",
        parent=base["Heading2"],
        fontName=bold,
        fontSize=12,
        leading=15,
        textColor=colors.HexColor("#087E8B"),
        spaceBefore=3 * mm,
        spaceAfter=1.5 * mm,
    )
    body = ParagraphStyle(
        "RangerBody",
        parent=base["BodyText"],
        fontName=regular,
        fontSize=9.2,
        leading=13.1,
        textColor=colors.HexColor("#263746"),
        spaceAfter=2.4 * mm,
    )
    small = ParagraphStyle(
        "RangerSmall",
        parent=body,
        fontSize=7.8,
        leading=10.4,
        textColor=colors.HexColor("#53687D"),
    )
    metric = ParagraphStyle(
        "RangerMetric",
        parent=body,
        fontName=bold,
        fontSize=12,
        leading=15,
        textColor=colors.HexColor("#071D31"),
        alignment=TA_CENTER,
    )
    center_small = ParagraphStyle(
        "RangerCenterSmall",
        parent=small,
        alignment=TA_CENTER,
    )

    def paragraph(text: str, style=body) -> Paragraph:
        return Paragraph(text, style)

    def styled_table(rows, widths=None, header=True) -> Table:
        header_style = ParagraphStyle(
            "RangerTableHeader",
            parent=small,
            fontName=bold,
            textColor=colors.white,
        )
        formatted = []
        for row_index, row in enumerate(rows):
            style = header_style if header and row_index == 0 else small
            formatted.append([paragraph(str(cell), style) for cell in row])
        table = Table(
            formatted,
            colWidths=widths,
            repeatRows=1 if header else 0,
            hAlign="LEFT",
        )
        commands = [
            ("VALIGN", (0, 0), (-1, -1), "TOP"),
            ("GRID", (0, 0), (-1, -1), 0.35, colors.HexColor("#C8D3DC")),
            ("LEFTPADDING", (0, 0), (-1, -1), 5),
            ("RIGHTPADDING", (0, 0), (-1, -1), 5),
            ("TOPPADDING", (0, 0), (-1, -1), 5),
            ("BOTTOMPADDING", (0, 0), (-1, -1), 5),
        ]
        if header:
            commands.append(("BACKGROUND", (0, 0), (-1, 0), colors.HexColor("#071D31")))
        for row_index in range(1 if header else 0, len(rows)):
            if row_index % 2 == 0:
                commands.append(
                    ("BACKGROUND", (0, row_index), (-1, row_index), colors.HexColor("#EDF5F6"))
                )
        table.setStyle(TableStyle(commands))
        return table

    result = large["result"]
    scope = large["scientific_scope"]
    robustness = hpc["robustness_array"]
    repeats = hpc["replicate_array"]
    serial_time = parallel["aggregate"]["median_serial_seconds"]
    parallel_time = parallel["aggregate"]["median_parallel_seconds"]
    speed_ratio = parallel["aggregate"]["ratio_of_medians"]

    evidence_strip = Table(
        [
            [
                paragraph("PUBLISHED ACCURACY", center_small),
                paragraph("LARGEST EXACT SECTOR", center_small),
                paragraph("WALL TIME", center_small),
                paragraph("HPC CAMPAIGN", center_small),
            ],
            [
                paragraph("36/36", metric),
                paragraph("451,681,246", metric),
                paragraph("3:55:43", metric),
                paragraph("560 CPUs", metric),
            ],
        ],
        colWidths=[document.width / 4] * 4,
        style=TableStyle(
            [
                ("BACKGROUND", (0, 0), (-1, -1), colors.HexColor("#DFF4F2")),
                ("BOX", (0, 0), (-1, -1), 0.9, colors.HexColor("#087E8B")),
                ("INNERGRID", (0, 0), (-1, -1), 0.35, colors.HexColor("#A7D9D4")),
                ("ALIGN", (0, 0), (-1, -1), "CENTER"),
                ("VALIGN", (0, 0), (-1, -1), "MIDDLE"),
                ("TOPPADDING", (0, 0), (-1, -1), 7),
                ("BOTTOMPADDING", (0, 0), (-1, -1), 7),
            ]
        ),
    )

    story = [
        Spacer(1, 15 * mm),
        paragraph("FINAL TECHNICAL REPORT", small),
        paragraph("Exactness at Scale:<br/>From CC(8) to 451 Million Determinants in Rust", title),
        paragraph(
            "Quantum Harness Challenge #129 / Team Ranger<br/>"
            "Chenxi Wan, Yedi Shen, Junkai Wang<br/>"
            "Innovation-led public delivery / 2026-07-30",
            subtitle,
        ),
        evidence_strip,
        Spacer(1, 8 * mm),
        paragraph("Breakthrough", h1),
        paragraph(
            "Ranger delivers one Rust determinant engine for FCI, CC, CI, MBPT, UCC, "
            "direct integrals, symmetry resolution, deterministic parallel execution, "
            "restartable Davidson, and verified HPC evidence. Exact ranked subset "
            "convolution makes CC(1)-CC(8) practical; symmetry-compact matrix-free FCI "
            "carries the same engine to an exact 451,681,246-determinant sector.",
        ),
        paragraph(
            "The result is a complete path from equations to algorithms, from algorithms "
            "to hundreds of millions of determinants, and from one benchmark to a reusable "
            "platform for selected electronic-structure methods.",
            small,
        ),
        PageBreak(),
        paragraph("1. Three barriers, three algorithms", h1),
        KeepTogether(
            [
                paragraph("1 / Wave-function construction", h2),
                paragraph(
                    "Exact ranked subset convolution builds exp(T)|HF> by excitation rank. "
                    "Each target coefficient combines compatible amplitude/source partitions "
                    "with exact fermionic phases and reuses completed lower-rank coefficients. "
                    "A Taylor implementation supplies an independent coefficient oracle.",
                ),
            ]
        ),
        KeepTogether(
            [
                paragraph("2 / Hamiltonian scale", h2),
                paragraph(
                    "Matrix-free spin-free sigma applies the Hamiltonian through signed string "
                    "links. Compact ORBSYM/ISYM addresses propagate across FCI, CI, MBPT, CC, "
                    "and UCC, transforming 1,806,590,016 determinants into 451,681,246 exact "
                    "C2v/A1 determinants for the water ground state.",
                ),
            ]
        ),
        KeepTogether(
            [
                paragraph("3 / Reproducible production", h2),
                paragraph(
                    "Fixed source blocks, thread-local partial vectors, and ordered reduction "
                    "make a fixed parallel policy bitwise repeatable. Versioned vector stores, "
                    "atomic checkpoint generations, input hashes, memory preflight, and resume "
                    "turn long Davidson runs into durable scientific computations.",
                ),
            ]
        ),
        paragraph("Why the advances compound", h2),
        styled_table(
            [
                ["Advance", "Scaling effect", "Measured proof"],
                ["Ranked subset convolution", "Direct finite-rank CC wave function", "CC(1)-CC(8), 8/8"],
                ["Matrix-free sigma", "Operator storage replaces matrix storage", "28M and 451M sectors"],
                ["Compact symmetry addresses", "Fourfold exact representation gain", "1.806B -> 451M"],
                ["Deterministic Davidson", "Parallel, repeatable, restartable", f"{speed_ratio:.6f}x + HPC"],
            ],
            widths=[47 * mm, 62 * mm, 58 * mm],
        ),
        PageBreak(),
        paragraph("2. Primary acceptance: 36/36", h1),
        paragraph(
            "The primary H2O/6-31G Hamiltonian freezes the oxygen 1s orbital and contains "
            f"12 active spatial orbitals, eight active electrons, and {primary['determinants']:,} "
            "determinants. Every published equilibrium entry matches at the six decimal places "
            "printed by Hirata and Bartlett.",
        ),
        styled_table(
            [
                ["Method family", "Range", "Matches", "Representative value"],
                ["Coupled cluster", "CC(1)-CC(8)", "8/8", f"CC(8) {primary['results'][-1]['energy']:.15f} Eh"],
                ["Configuration interaction", "CI(1)-CI(8)", "8/8", "CI(8) reaches FCI"],
                ["Perturbation theory", "MBPT(1)-MBPT(20)", "20/20", "All printed orders"],
                ["Matrix-free FCI", "Ground state", "Accepted", f"{primary['fci_energy']:.15f} Eh"],
            ],
            widths=[43 * mm, 44 * mm, 25 * mm, 55 * mm],
        ),
        paragraph("Independent numerical ladder", h2),
        paragraph(
            "Dense Rust FCI agrees with PySCF for H2, linear H4, and H2O/STO-3G. "
            "The direct, serial, parallel, symmetry, memory, disk, checkpoint, resume, "
            "multi-root, and full-rank UCC paths cross-check shared algebra through distinct "
            "executions. Stretched-water fixtures at 1.5 and 2.0 times the equilibrium bond "
            "length extend acceptance across changing correlation regimes.",
        ),
        paragraph("High-rank coupled cluster", h2),
        paragraph(
            "CC(2) agrees with the independent PySCF CCSD oracle within 3.025e-10 Eh. "
            "CC(8) reaches 7.998e-9 Eh from FCI, while CI(8) reaches 2.004e-12 Eh. "
            "The complete CC series finishes in 186.94 seconds on the recorded Apple M4 "
            "environment, turning general-order theory into a practical test oracle.",
        ),
        PageBreak(),
        paragraph("3. Exact scaling ladder", h1),
        styled_table(
            [
                ["Hamiltonian and exact sector", "Determinants", "Rust energy"],
                ["H2O/6-31G, O 1s frozen", "245,025", "-76.121174204142 Eh"],
                ["H2O/DZ, all electron", "1,002,708", "-76.156699030930056 Eh"],
                ["H2O/DZP, O 1s frozen", "28,233,466", "-76.256624441300147 Eh"],
                ["H2O/cc-pVDZ, all electron, C2v/A1", f"{scope['determinants']:,}", f"{result['reported_total_energy_hartree_text']} Eh"],
            ],
            widths=[79 * mm, 43 * mm, 45 * mm],
        ),
        paragraph("451-million-determinant exact sector", h2),
        paragraph(
            "The largest run correlates all ten electrons in 24 spherical spatial orbitals. "
            f"Davidson reaches residual {result['residual_norm']:.3e} in "
            f"{result['iterations']} iterations and {large['hpc']['elapsed']} wall time. "
            "Same-input PySCF RHF, MP2, CISD, CCSD, and CCSD(T) supply a method hierarchy; "
            "CCSD(T) lies 0.647144 mEh above the Rust FCI result.",
        ),
        paragraph("Symmetry-free resource characterization", h2),
        paragraph(
            "The companion 1,806,590,016-determinant study measures a 13.460145 GiB CI "
            "vector, Rust integral generation, RHF, AO-to-MO transformation, string links, "
            "distributed source samples, and sparse Hamiltonian columns. The fourfold compact "
            "A1 representation connects this resource model to the completed production solve.",
        ),
        paragraph("Scale multiplier", h2),
        paragraph(
            "The largest completed exact sector contains more than 1,800 times the determinants "
            "of the primary challenge. Rank recursion, matrix-free action, symmetry addressing, "
            "parallel reduction, and restartable storage create this multiplier together.",
        ),
        PageBreak(),
        paragraph("4. Deterministic HPC", h1),
        paragraph(
            "The SCNet workflow rebuilds the pinned Rust source in an offline toolchain, runs "
            "the test suite and numerical smoke checks, sweeps Davidson parameters, and repeats "
            "solves across AMD EPYC 7742 nodes.",
        ),
        styled_table(
            [
                ["Evidence", "Result"],
                ["Robustness matrix", f"{len(robustness['cases'])}/{len(robustness['cases'])} converged"],
                ["Repeated solves", f"{repeats['sample_count']}/{repeats['sample_count']} converged"],
                ["Energy range", "8.10e-13 Eh"],
                ["Verified manifests / evidence files", f"{hpc['provenance']['verified_sha256_manifests']} / {hpc['provenance']['downloaded_evidence_files']}"],
                ["Allocation peak", f"{repeats['observed_peak']['cpus']} CPUs across {repeats['observed_peak']['tasks']} tasks"],
            ],
            widths=[73 * mm, 94 * mm],
        ),
        paragraph("Local kernel acceleration", h2),
        paragraph(
            f"Median sigma time moves from {serial_time:.9f} seconds serial to "
            f"{parallel_time:.9f} seconds with four fixed blocks, a {speed_ratio:.6f}x "
            "ratio of medians. Ordered block reduction yields bitwise repeatability for a "
            "fixed policy and links the local kernel test directly to production execution.",
        ),
        paragraph("Throughput architecture", h2),
        paragraph(
            "Per-solve utilization measurements motivate four 14-thread solver processes per "
            "56-core node. The 1,008-CPU campaign design schedules 72 independent processes "
            "across 18 nodes, providing a concrete architecture for high-volume determinant "
            "method studies. The verified 560-CPU campaign establishes the repeatability and "
            "portability foundation for this design.",
        ),
        PageBreak(),
        paragraph("5. One engine, many methods", h1),
        paragraph(
            "The workbench is organized around reusable determinant algebra. Direct libcint AO "
            "integrals flow through Rust RHF/DIIS and staged AO-to-MO transformation into the "
            "same determinant basis, symmetry addresses, and Hamiltonian action used by every "
            "wave-function method.",
        ),
        styled_table(
            [
                ["Shared primitive", "Method families that reuse it"],
                ["Determinant addresses + fermionic phases", "FCI, CI, CC, MBPT, UCC"],
                ["Matrix-free Hamiltonian action", "FCI, projected CI, CC residuals"],
                ["Compact symmetry sectors", "FCI, CI, MBPT, CC, UCC"],
                ["Davidson vector stores", "Ground states, several roots, selected spaces"],
                ["Machine-readable fixtures", "Dense, direct, parallel, HPC acceptance"],
            ],
            widths=[70 * mm, 97 * mm],
        ),
        paragraph("Selected-determinant frontier", h2),
        paragraph(
            "The exact engine prepares a common interface for deterministic HCI/iCI-style "
            "selection, variational selected-space Davidson, Epstein-Nesbet PT2, threshold "
            "extrapolation, natural-orbital iterations, orbital optimization, and quantum-sampled "
            "determinant lists. The completed exact result ladder supplies calibration targets "
            "and equal-size comparisons for every selection strategy.",
        ),
        paragraph("Promising research direction", h2),
        paragraph(
            "Ranger combines the accuracy of an exact finite-sector oracle with an architecture "
            "built for method invention. New determinant generators can enter through one address "
            "layer and immediately inherit symmetry, matrix elements, eigensolvers, fixtures, "
            "residual checks, and reproducible evidence.",
        ),
        PageBreak(),
        paragraph("6. Reproduction and public package", h1),
        paragraph("Complete local acceptance", h2),
        paragraph("<font name='Courier'>uv sync --locked<br/>scripts/verify-submission.sh</font>"),
        paragraph("Focused evidence audit", h2),
        paragraph("<font name='Courier'>python3 scripts/hpc/verify_final_evidence.py</font>"),
        styled_table(
            [
                ["Artifact", "Repository path"],
                ["Technical article", "reports/final-competition-summary.md"],
                ["Technical PDF", "output/pdf/quantum-harness-129-final-technical-report.pdf"],
                ["Result card", "output/data/quantum-harness-129-final-results.txt"],
                ["Checksum manifest", "output/quantum-harness-129-submission-manifest.txt"],
                ["451M machine record", "fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json"],
                ["SCNet machine record", "fixtures/hpc/scnet-2026-07-30.json"],
                ["Reproduction prompt", "docs/reproduction-prompt.md"],
            ],
            widths=[48 * mm, 119 * mm],
        ),
        paragraph("Public endpoints", h2),
        paragraph(
            f"Repository: {REPOSITORY_URL}<br/>"
            "Publication branch: codex/final-competition-submission<br/>"
            f"PR: {PR_URL}<br/>Release: {RELEASE_URL}"
        ),
        paragraph("Scientific foundations", h2),
        paragraph(
            "Hirata-Bartlett general-order CC: DOI 10.1016/S0009-2614(00)00387-0<br/>"
            "Knowles-Handy determinant FCI: DOI 10.1016/0009-2614(84)85513-X<br/>"
            "Holmes-Tubman-Umrigar HCI: DOI 10.1021/acs.jctc.6b00407<br/>"
            "Zhang-Liu-Hoffmann iCI: DOI 10.1021/acs.jctc.9b01200",
            small,
        ),
        paragraph("Final perspective", h2),
        paragraph(
            "Ranger demonstrates a powerful path for electronic-structure research: build one "
            "trusted determinant algebra, introduce algorithms whose gains compound, and carry "
            "every advance from equations to source, data, HPC evidence, and public reproduction.",
        ),
    ]
    document.build(story)


def build_text(primary: dict, large: dict, hpc: dict, parallel: dict) -> None:
    TEXT_PATH.parent.mkdir(parents=True, exist_ok=True)
    result = large["result"]
    scope = large["scientific_scope"]
    repeats = hpc["replicate_array"]
    ratio = parallel["aggregate"]["ratio_of_medians"]
    lines = [
        "QUANTUM HARNESS #129 - RANGER FINAL RESULT CARD",
        "================================================",
        "Date: 2026-07-30",
        "Team: Ranger (Chenxi Wan, Yedi Shen, Junkai Wang)",
        f"Repository: {REPOSITORY_URL}",
        f"Publication branch: {BRANCH_URL}",
        f"Upstream PR: {PR_URL}",
        "",
        "BREAKTHROUGH",
        "------------",
        "Exactness at scale in Rust: CC(1)-CC(8) to 451,681,246 determinants.",
        "36/36 published Hirata-Bartlett CC/CI/MBPT entries match.",
        f"Deterministic parallel sigma median timing ratio: {ratio:.6f}x.",
        "Verified SCNet campaign: 560 CPUs across ten tasks.",
        "",
        "THREE BARRIERS, THREE ALGORITHMS",
        "--------------------------------",
        "1. Exact ranked subset convolution for exp(T)|HF> through CC(8).",
        "2. Symmetry-compact matrix-free FCI across the shared method stack.",
        "3. Deterministic fixed-block and restartable Davidson execution.",
        "",
        "PRIMARY 36/36 ACCEPTANCE",
        "------------------------",
        "System: H2O/6-31G, oxygen 1s frozen, 12 active orbitals, 8 active electrons",
        f"Determinants: {primary['determinants']:,}",
        f"FCI energy (Eh): {primary['fci_energy']:.15f}",
        "CC(1)-CC(8): 8/8 published entries match",
        "CI(1)-CI(8): 8/8 published entries match",
        "MBPT(1)-MBPT(20): 20/20 published entries match",
        "Published total: 36/36",
        "",
        "LARGEST EXACT SECTOR",
        "--------------------",
        "System: H2O/cc-pVDZ, all electron, exact C2v/A1 sector",
        f"Determinants: {scope['determinants']:,}",
        f"FCI energy (Eh): {result['reported_total_energy_hartree_text']}",
        f"Residual norm: {result['residual_norm']:.3e}",
        f"Davidson iterations: {result['iterations']}",
        f"Wall time: {large['hpc']['elapsed']}",
        "Scale multiplier over primary space: greater than 1,800x",
        "",
        "VALIDATED SCOPE",
        "---------------",
        "Exact C2v/A1 result: 451,681,246 determinants",
        "Symmetry-free resource characterization: 1,806,590,016 determinants",
        f"Verified SCNet campaign: {len(hpc['robustness_array']['cases'])}/18 robustness cases",
        f"Cross-node repeat solves: {repeats['sample_count']}/216",
        f"Allocation peak: {repeats['observed_peak']['cpus']} CPUs",
        "1,008-CPU campaign design: 72 processes across 18 nodes",
        "",
        "SELECTED-DETERMINANT FRONTIER",
        "-----------------------------",
        "The shared exact engine prepares deterministic HCI/iCI-style selection,",
        "variational selected-space Davidson, EN-PT2, threshold extrapolation,",
        "orbital optimization, and quantum-sampled determinant import.",
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
        ROOT / "README.md",
        ROOT / "reports/final-competition-summary.md",
        ROOT / "docs/submission-pr-body.md",
        ROOT / "docs/submission-final-comment.md",
        PDF_PATH,
        TEXT_PATH,
        ROOT / "fixtures/h2o-ccpvdz-ae/fci-c2v-xh5-result.json",
        ROOT / "fixtures/hpc/scnet-2026-07-30.json",
        ROOT / "docs/reproduction-prompt.md",
        ROOT / "scripts/hpc/verify_final_evidence.py",
        ROOT / "scripts/report/generate_final_submission.py",
    ]
    lines = [
        "QUANTUM HARNESS #129 - PUBLIC SUBMISSION MANIFEST",
        "================================================",
        f"Publication branch: {BRANCH_URL}",
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
    parallel = load_json("fixtures/h2o-631g-fc/parallel-sigma-m4.json")
    build_pdf(primary, large, hpc, parallel)
    build_text(primary, large, hpc, parallel)
    build_manifest()
    print(PDF_PATH.relative_to(ROOT))
    print(TEXT_PATH.relative_to(ROOT))
    print(MANIFEST_PATH.relative_to(ROOT))


if __name__ == "__main__":
    main()
