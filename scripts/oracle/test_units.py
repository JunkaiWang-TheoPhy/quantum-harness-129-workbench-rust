"""Regression tests for geometry and unit metadata in oracle systems."""

from __future__ import annotations

import json
import unittest
from pathlib import Path

import numpy
from pyscf import gto

from scripts.oracle.generate import SYSTEMS, System


class GeometryUnitTests(unittest.TestCase):
    def molecule(self, slug: str) -> tuple[System, numpy.ndarray]:
        system = SYSTEMS[slug]
        self.assertEqual(system.coordinate_unit, "Angstrom")
        molecule = gto.M(
            atom=system.atom,
            basis=system.basis,
            charge=system.charge,
            spin=system.spin,
            unit=system.coordinate_unit,
            verbose=0,
        )
        return system, molecule.atom_coords(unit="Angstrom")

    def test_h2_coordinates_mean_a_1_4_angstrom_bond(self) -> None:
        system, coordinates = self.molecule("h2-sto3g")
        distance = numpy.linalg.norm(coordinates[1] - coordinates[0])
        self.assertAlmostEqual(distance, 1.4, places=12)
        self.assertEqual(
            system.geometry_parameters,
            (("R(H-H)", 1.4, "angstrom"),),
        )

    def test_equilibrium_h2_has_a_0_7414_angstrom_bond(self) -> None:
        system, coordinates = self.molecule("h2-equilibrium-sto3g")
        distance = numpy.linalg.norm(coordinates[1] - coordinates[0])
        self.assertAlmostEqual(distance, 0.7414, places=12)
        self.assertEqual(
            system.geometry_parameters,
            (("R(H-H)", 0.7414, "angstrom"),),
        )

    def test_h4_has_one_angstrom_adjacent_spacing(self) -> None:
        system, coordinates = self.molecule("h4-sto3g")
        distances = [
            numpy.linalg.norm(coordinates[index + 1] - coordinates[index])
            for index in range(3)
        ]
        for distance in distances:
            self.assertAlmostEqual(distance, 1.0, places=12)
        self.assertEqual(
            system.geometry_parameters,
            (("adjacent R(H-H)", 1.0, "angstrom"),),
        )

    def test_water_geometry_is_0_967_angstrom_and_107_6_degrees(self) -> None:
        for slug in ("h2o-sto3g", "h2o-631g-fc"):
            system, coordinates = self.molecule(slug)
            first_bond = coordinates[1] - coordinates[0]
            second_bond = coordinates[2] - coordinates[0]
            first_length = numpy.linalg.norm(first_bond)
            second_length = numpy.linalg.norm(second_bond)
            cosine = numpy.dot(first_bond, second_bond) / (first_length * second_length)
            angle_degrees = numpy.degrees(numpy.arccos(cosine))
            self.assertAlmostEqual(first_length, 0.967, places=12)
            self.assertAlmostEqual(second_length, 0.967, places=12)
            self.assertAlmostEqual(angle_degrees, 107.6, places=12)
            self.assertEqual(
                system.geometry_parameters,
                (
                    ("R(O-H)", 0.967, "angstrom"),
                    ("angle(H-O-H)", 107.6, "degree"),
                ),
            )

    def test_committed_references_expose_units_and_geometry_parameters(self) -> None:
        fixtures_root = Path(__file__).resolve().parents[2] / "fixtures"
        for slug, system in SYSTEMS.items():
            reference_path = fixtures_root / slug / "reference.json"
            reference = json.loads(reference_path.read_text(encoding="utf-8"))
            self.assertEqual(reference["coordinate_unit"], "angstrom")
            self.assertEqual(reference["energy_unit"], "hartree")
            expected_parameters = [
                {"name": name, "value": value, "unit": unit}
                for name, value, unit in system.geometry_parameters
            ]
            self.assertEqual(reference["geometry_parameters"], expected_parameters)


if __name__ == "__main__":
    unittest.main()
