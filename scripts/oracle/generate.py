#!/usr/bin/env python3
"""Generate deterministic Level 0 FCIDUMP and reference fixtures with PySCF."""

from __future__ import annotations

import argparse
import hashlib
import json
from dataclasses import dataclass
from pathlib import Path
import platform
import sys

import pyscf
import numpy
from pyscf import ao2mo, cc, fci, gto, scf
from pyscf.tools import fcidump


@dataclass(frozen=True)
class System:
    slug: str
    name: str
    atom: str
    basis: str = "sto-3g"
    charge: int = 0
    spin: int = 0
    frozen_orbitals: tuple[int, ...] = ()


SYSTEMS = {
    "h2-sto3g": System(
        slug="h2-sto3g",
        name="H2",
        atom="H 0 0 -0.7; H 0 0 0.7",
    ),
    "h4-sto3g": System(
        slug="h4-sto3g",
        name="linear H4",
        atom="H 0 0 -1.5; H 0 0 -0.5; H 0 0 0.5; H 0 0 1.5",
    ),
    "h2o-sto3g": System(
        slug="h2o-sto3g",
        name="H2O",
        atom="O 0 0 0; H 0.967 0 0; H -0.2923916843556798 0.9217353757557798 0",
    ),
    "h2o-631g-fc": System(
        slug="h2o-631g-fc",
        name="H2O frozen core",
        atom="O 0 0 0; H 0.967 0 0; H -0.2923916843556798 0.9217353757557798 0",
        basis="6-31g",
        frozen_orbitals=(0,),
    ),
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def generate(system: System, fixtures_root: Path) -> dict[str, object]:
    output_dir = fixtures_root / system.slug
    output_dir.mkdir(parents=True, exist_ok=True)
    dump_path = output_dir / "FCIDUMP"

    mol = gto.M(
        atom=system.atom,
        basis=system.basis,
        charge=system.charge,
        spin=system.spin,
        unit="Angstrom",
        symmetry=False,
        verbose=0,
    )
    mf = scf.RHF(mol)
    mf.conv_tol = 1e-12
    hf_energy = float(mf.kernel())
    if not mf.converged:
        raise RuntimeError(f"RHF did not converge for {system.slug}")

    nmo = mf.mo_coeff.shape[1]
    mo_h1 = mf.mo_coeff.T @ mf.get_hcore() @ mf.mo_coeff
    mo_eri = ao2mo.restore(1, ao2mo.kernel(mol, mf.mo_coeff), nmo)
    frozen = tuple(system.frozen_orbitals)
    active = tuple(index for index in range(nmo) if index not in frozen)
    if frozen:
        ecore = float(mol.energy_nuc())
        for i in frozen:
            ecore += 2.0 * mo_h1[i, i]
            for j in frozen:
                ecore += 2.0 * mo_eri[i, i, j, j] - mo_eri[i, j, j, i]
        active_h1 = mo_h1[numpy.ix_(active, active)].copy()
        for p_new, p in enumerate(active):
            for q_new, q in enumerate(active):
                for i in frozen:
                    active_h1[p_new, q_new] += (
                        2.0 * mo_eri[p, q, i, i] - mo_eri[p, i, i, q]
                    )
        active_eri = mo_eri[numpy.ix_(active, active, active, active)]
        active_nelec = mol.nelectron - 2 * len(frozen)
        fcidump.from_integrals(
            str(dump_path),
            active_h1,
            active_eri,
            len(active),
            active_nelec,
            nuc=ecore,
            ms=system.spin,
            tol=1e-15,
            float_format=" %.16g",
        )
        cisolver = fci.direct_spin1.FCI()
        cisolver.conv_tol = 1e-12
        fci_energy, _ = cisolver.kernel(
            active_h1,
            active_eri,
            len(active),
            active_nelec,
            ecore=ecore,
        )
    else:
        fcidump.from_scf(mf, str(dump_path), tol=1e-15, float_format=" %.16g")
        cisolver = fci.FCI(mf)
        cisolver.conv_tol = 1e-12
        fci_energy, _ = cisolver.kernel()
    fci_converged = bool(getattr(cisolver, "converged", True))

    ccsd = cc.CCSD(mf, frozen=list(frozen) or None)
    ccsd.conv_tol = 1e-12
    correlation_energy, _, _ = ccsd.kernel()
    ccsd_total_energy = hf_energy + float(correlation_energy)

    reference: dict[str, object] = {
        "schema_version": 1,
        "system": system.name,
        "slug": system.slug,
        "geometry_angstrom": system.atom,
        "basis": system.basis,
        "charge": system.charge,
        "spin": system.spin,
        "frozen_orbitals": list(frozen),
        "nuclear_repulsion_energy": float(mol.energy_nuc()),
        "number_of_atomic_orbitals": int(mol.nao_nr()),
        "number_of_molecular_orbitals": int(mf.mo_coeff.shape[1]),
        "number_of_active_molecular_orbitals": len(active),
        "number_of_electrons": int(mol.nelectron),
        "number_of_active_electrons": int(mol.nelectron - 2 * len(frozen)),
        "active_orbital_energies": [float(mf.mo_energy[index]) for index in active],
        "pyscf_version": pyscf.__version__,
        "python_version": platform.python_version(),
        "hf_converged": bool(mf.converged),
        "fci_converged": fci_converged,
        "ccsd_converged": bool(ccsd.converged),
        "hf_energy": hf_energy,
        "fci_energy": float(fci_energy),
        "ccsd_correlation_energy": float(correlation_energy),
        "ccsd_total_energy": ccsd_total_energy,
        "fcidump_sha256": sha256(dump_path),
        "generator": "scripts/oracle/generate.py",
        "energy_unit": "hartree",
    }
    reference_path = output_dir / "reference.json"
    reference_path.write_text(
        json.dumps(reference, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    return reference


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "systems",
        nargs="*",
        choices=sorted(SYSTEMS),
    )
    parser.add_argument(
        "--fixtures-root",
        type=Path,
        default=Path(__file__).resolve().parents[2] / "fixtures",
    )
    args = parser.parse_args()

    selected_systems = args.systems or sorted(SYSTEMS)
    for slug in selected_systems:
        reference = generate(SYSTEMS[slug], args.fixtures_root)
        print(
            f"{slug}: HF={reference['hf_energy']:.12f} "
            f"FCI={reference['fci_energy']:.12f} "
            f"CCSD={reference['ccsd_total_energy']:.12f}"
        )
    return 0


if __name__ == "__main__":
    sys.exit(main())
