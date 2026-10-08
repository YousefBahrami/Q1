import copy
import tempfile
import unittest
from pathlib import Path

import resource_lab as lab
from rewards import apportion, simulate


class EvidenceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="q1-lab-test-")
        self.addCleanup(self.temp.cleanup)
        self.path = Path(self.temp.name) / "data"
        self.seed, self.participant = bytes(32), bytes([1]) * 32
        self.manifest, self.tree = lab.create_dataset(self.path, 8 * lab.CHUNK_BYTES,
                                                     self.seed, self.participant)
        self.context = dict(profile=lab.PROFILE, chain="q1-resource-lab-no-network",
                            height=1, round=0, producer=self.participant.hex(),
                            nonce=bytes(32).hex(), dataset=lab.commitment(self.manifest))
        self.data = self.path.read_bytes()
        self.proof = lab.prove(self.manifest, self.tree, self.context,
                               lambda i: self.data[i * lab.CHUNK_BYTES:(i + 1) * lab.CHUNK_BYTES], 8)

    def test_deterministic_and_valid(self):
        other, _ = lab.create_dataset(Path(self.temp.name) / "second", len(self.data),
                                      self.seed, self.participant)
        self.assertEqual(other, self.manifest)
        self.assertEqual(lab.verify(lab.canonical(self.proof), other, self.context),
                         self.proof["response_root"])

    def test_forgery_and_corruption_rejected(self):
        for field in ("data", "response", "path", "index"):
            with self.subTest(field=field):
                changed = copy.deepcopy(self.proof)
                row = changed["rows"][0]
                if field == "path":
                    row[field][0] = "00" * 32
                elif field == "index":
                    row[field] = (row[field] + 1) % 8
                else:
                    row[field] = ("ff" if row[field][:2] != "ff" else "00") + row[field][2:]
                with self.assertRaises(lab.LabError):
                    lab.verify(lab.canonical(changed), self.manifest, self.context)

    def test_replay_wrong_context_and_identity(self):
        for field, value in (("height", 2), ("round", 1), ("nonce", "01" * 32),
                             ("chain", "mainnet"), ("producer", "02" * 32),
                             ("dataset", "00" * 32)):
            context = dict(self.context, **{field: value})
            with self.subTest(field=field), self.assertRaises(lab.LabError):
                lab.verify(lab.canonical(self.proof), self.manifest, context)

    def test_wrong_manifest(self):
        changed = dict(self.manifest, participant="02" * 32)
        with self.assertRaises(lab.LabError):
            lab.verify(lab.canonical(self.proof), changed, self.context)

    def test_bounds_schema_and_duplicate_fields(self):
        malformed = [b"{" + b" " * lab.MAX_EVIDENCE_BYTES,
                     b'{"x":1,"x":2}', b'{', b'[]', b'[' * 1500 + b']' * 1500]
        for raw in malformed:
            with self.subTest(raw_size=len(raw)), self.assertRaises(lab.LabError):
                lab.verify(raw, self.manifest, self.context)
        for field, value in (("samples", True), ("samples", 128), ("rows", []),
                             ("response_root", "00" * 32), ("profile", "MAINNET")):
            proof = dict(self.proof, **{field: value})
            with self.subTest(field=field), self.assertRaises(lab.LabError):
                lab.verify(lab.canonical(proof), self.manifest, self.context)
        proof = dict(self.proof, unexpected=0)
        with self.assertRaises(lab.LabError):
            lab.verify(lab.canonical(proof), self.manifest, self.context)

    def test_path_lengths_and_truncation(self):
        for path in ([], ["00" * 32] * 64, "not-a-list"):
            proof = copy.deepcopy(self.proof)
            proof["rows"][0]["path"] = path
            with self.assertRaises(lab.LabError):
                lab.verify(lab.canonical(proof), self.manifest, self.context)
        with self.assertRaises(lab.LabError):
            lab.prove(self.manifest, self.tree, self.context, lambda i: b"bad", 8)

    def test_no_overwrite_and_size_limit(self):
        with self.assertRaises(FileExistsError):
            lab.create_dataset(self.path, len(self.data), self.seed, self.participant)
        with self.assertRaises(lab.LabError):
            lab.create_dataset(Path(self.temp.name) / "too-big", lab.MAX_DATASET_BYTES * 2,
                               self.seed, self.participant)
        self.assertEqual(self.path.read_bytes(), self.data)

    def test_regeneration_without_dataset_is_indistinguishable(self):
        self.path.unlink()
        regenerated = lab.prove(self.manifest, self.tree, self.context,
                                lambda i: lab.chunk(self.seed, self.participant, i), 8)
        self.assertEqual(regenerated, self.proof)
        lab.verify(lab.canonical(regenerated), self.manifest, self.context)

    def test_corrupt_local_file_detected(self):
        with self.assertRaisesRegex(lab.LabError, "DATA_CORRUPTION"):
            lab.prove(self.manifest, self.tree, self.context, lambda i: bytes(lab.CHUNK_BYTES), 8)

    def test_one_leaf_and_duplicate_samples(self):
        manifest, tree = lab.create_dataset(Path(self.temp.name) / "one", lab.CHUNK_BYTES,
                                            self.seed, self.participant)
        context = dict(self.context, dataset=lab.commitment(manifest))
        proof = lab.prove(manifest, tree, context,
                          lambda i: lab.chunk(self.seed, self.participant, i), 8)
        self.assertEqual({r["index"] for r in proof["rows"]}, {0})
        lab.verify(lab.canonical(proof), manifest, context)


class SimulationTests(unittest.TestCase):
    def test_budget_and_supply_conservation(self):
        result = simulate()
        self.assertEqual(result, simulate())
        self.assertEqual(len(result["scenarios"]), 32)
        for row in result["scenarios"]:
            issue = row["issued_per_epoch"]
            self.assertEqual(sum(row["operator_rewards"].values()), issue)
            self.assertEqual(row["supply_by_epoch"]["100"], 100000 + 100 * issue)
            if row["schedule"] == "fixed_epoch_budget":
                self.assertEqual(issue, 7200)
            self.assertTrue(0 <= row["gini"] < 1)

    def test_sybil_defeats_per_identity_caps(self):
        rows = {(r["scenario"], r["model"]): r for r in simulate()["scenarios"]
                if r["schedule"] == "fixed_epoch_budget"}
        for model in ("equal", "capped", "diminishing"):
            self.assertGreater(rows["hardware_64_split", model]["hardware_operator_share"],
                               rows["hardware_64", model]["hardware_operator_share"])
        self.assertEqual(rows["hardware_64_split", "proportional"]["hardware_operator_share"],
                         rows["hardware_64", "proportional"]["hardware_operator_share"])

    def test_rounding_never_mints_extra(self):
        for n in (1, 3, 8, 72):
            for budget in (0, 1, 7, 7200):
                self.assertEqual(sum(apportion(list(range(1, n + 1)), budget)), budget)


if __name__ == "__main__":
    unittest.main()
