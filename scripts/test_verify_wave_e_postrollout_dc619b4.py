"""Offline contracts for the release-pinned Wave E state verifier."""
import copy
from contextlib import closing
import hashlib
import importlib.util
import json
import sqlite3
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("verify-wave-e-postrollout-dc619b4.py")
SPEC = importlib.util.spec_from_file_location("wave_e_dc619b4_verifier", SCRIPT)
VERIFIER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFIER)


class WaveEReleaseManifestTests(unittest.TestCase):
    def setUp(self):
        self.manifest = VERIFIER.load_manifest()
        self.slugs = {row["slug"] for row in self.manifest["traders"]}

    def test_manifest_is_exactly_pinned_to_release_and_historical_scope(self):
        self.assertEqual(len(self.slugs), 60)
        self.assertEqual(self.manifest["release_commit"], VERIFIER.RELEASE_COMMIT)
        self.assertEqual(self.manifest["release_tree"], VERIFIER.RELEASE_TREE)
        self.assertEqual(self.manifest["rehearsal_manifest_sha256"],
                         VERIFIER.REHEARSAL_MANIFEST_SHA256)
        self.assertEqual(self.manifest["fields"], list(VERIFIER.FIELDS))

    def test_manifest_content_digest_is_pinned(self):
        with tempfile.TemporaryDirectory(dir="/tmp/opencode") as directory:
            path = Path(directory) / "manifest.json"
            path.write_bytes(VERIFIER.MANIFEST.read_bytes() + b" ")
            with patch.object(VERIFIER, "MANIFEST", path):
                with self.assertRaisesRegex(ValueError, "digest mismatch"):
                    VERIFIER.load_manifest()

    def test_wrong_release_scope_and_row_payload_fail_closed(self):
        base = self.manifest
        mutations = [
            lambda m: m.update(release_commit="0" * 40),
            lambda m: m.update(release_tree="0" * 40),
            lambda m: m.update(rehearsal_manifest_sha256="0" * 64),
            lambda m: m.update(historical_selection_sha256="0" * 64),
            lambda m: m.update(fields=list(reversed(VERIFIER.FIELDS))),
            lambda m: m["traders"].pop(),
            lambda m: m["traders"].append(copy.deepcopy(m["traders"][0])),
            lambda m: m["traders"][0].update(slug="unrelated"),
            lambda m: m["traders"][0]["values"].pop("street"),
            lambda m: m["traders"][0]["values"].update(street=None),
            lambda m: m["traders"][0].update(notes_sha256="invalid"),
            lambda m: m["traders"][0].update(seed_hash="invalid"),
        ]
        for index, mutate in enumerate(mutations):
            with self.subTest(mutation=index):
                manifest = copy.deepcopy(base)
                mutate(manifest)
                with self.assertRaises(ValueError):
                    VERIFIER.validate_manifest(manifest, self.slugs)


class WaveEReleaseStateTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir="/tmp/opencode")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.path = self.directory / "public.db"
        self.db = sqlite3.connect(self.path)
        self.addCleanup(self.db.close)
        self.manifest = copy.deepcopy(VERIFIER.load_manifest())
        self.expected = self.manifest["traders"]
        self.fixture_notes = {}
        columns = ["slug", *VERIFIER.FIELDS, "notes", "extra_json", "updated_at"]
        self.db.execute("CREATE TABLE traders (" + ",".join(f"{column} TEXT" for column in columns) + ")")
        for index, expected in enumerate(self.expected):
            notes = "fixture Timeline for " + expected["slug"]
            self.fixture_notes[expected["slug"]] = notes
            expected["notes_sha256"] = hashlib.sha256(notes.encode()).hexdigest()
            values = {
                "slug": expected["slug"],
                **expected["values"],
                "notes": notes,
                "extra_json": json.dumps({"seed_hash": expected["seed_hash"]}),
                # Unchanged rows legitimately retain pre-rollout timestamps.
                "updated_at": "2026-10-10T12:00:00Z" if index < 20 else "2026-10-27T02:44:00Z",
            }
            self.db.execute(
                "INSERT INTO traders(" + ",".join(values) + ") VALUES ("
                + ",".join("?" for _ in values) + ")",
                list(values.values()),
            )
        self.db.commit()

    def verify(self):
        with closing(VERIFIER.connect(self.path)) as public:
            return VERIFIER.verify(public, self.manifest)

    def test_exact_state_passes_even_with_20_older_updated_at_values(self):
        before = self.path.read_bytes()
        result = self.verify()
        self.assertTrue(result["ok"], result["failures"])
        self.assertEqual(result["checked_count"], 60)
        self.assertEqual(result["scalar_checks"], 660)
        self.assertEqual(result["notes_hash_checks"], 60)
        self.assertEqual(result["seed_hash_checks"], 60)
        self.assertEqual(len(result["rows"]), 60)
        self.assertTrue(all(row["ok"] for row in result["rows"]))
        self.assertEqual(self.path.read_bytes(), before)

    def test_each_of_660_scalar_expectations_is_enforced(self):
        with closing(VERIFIER.connect(self.path)) as public:
            for expected in self.expected:
                for field in VERIFIER.FIELDS:
                    with self.subTest(slug=expected["slug"], field=field):
                        original = self.db.execute(
                            f"SELECT {field} FROM traders WHERE slug=?", (expected["slug"],)
                        ).fetchone()[0]
                        self.db.execute(f"UPDATE traders SET {field}='changed' WHERE slug=?",
                                        (expected["slug"],))
                        self.db.commit()
                        result = VERIFIER.verify(public, self.manifest)
                        self.assertIn(f"{expected['slug']}: {field} target mismatch", result["failures"])
                        self.db.execute(f"UPDATE traders SET {field}=? WHERE slug=?",
                                        (original, expected["slug"]))
                        self.db.commit()

    def test_notes_and_seed_fingerprint_are_enforced(self):
        slug = self.expected[0]["slug"]
        with closing(VERIFIER.connect(self.path)) as public:
            self.db.execute("UPDATE traders SET notes=notes||' changed' WHERE slug=?", (slug,))
            self.db.commit()
            self.assertIn(f"{slug}: seeded Timeline notes mismatch",
                          VERIFIER.verify(public, self.manifest)["failures"])
            self.db.execute("UPDATE traders SET notes=? WHERE slug=?",
                            (self.fixture_notes[slug], slug))
            self.db.execute("UPDATE traders SET extra_json='[]' WHERE slug=?", (slug,))
            self.db.commit()
            self.assertIn(f"{slug}: extra_json is not an object",
                          VERIFIER.verify(public, self.manifest)["failures"])
            self.db.execute("UPDATE traders SET extra_json='{' WHERE slug=?", (slug,))
            self.db.commit()
            self.assertIn(f"{slug}: extra_json is not an object",
                          VERIFIER.verify(public, self.manifest)["failures"])
            self.db.execute("UPDATE traders SET extra_json='{}' WHERE slug=?", (slug,))
            self.db.commit()
            self.assertIn(f"{slug}: release seed hash mismatch",
                          VERIFIER.verify(public, self.manifest)["failures"])

    def test_missing_or_duplicate_scope_rows_fail(self):
        slug = self.expected[0]["slug"]
        self.db.execute("DELETE FROM traders WHERE slug=?", (slug,))
        self.db.commit()
        self.assertIn(f"{slug}: expected one trader, found 0", self.verify()["failures"])
        self.db.execute("INSERT INTO traders SELECT * FROM traders WHERE slug=?",
                         (self.expected[1]["slug"],))
        self.db.commit()
        self.assertIn(f"{self.expected[1]['slug']}: expected one trader, found 2",
                      self.verify()["failures"])

    def test_database_open_is_read_only_and_missing_file_is_not_created(self):
        with closing(VERIFIER.connect(self.path)) as public:
            with self.assertRaises(sqlite3.OperationalError):
                public.execute("DELETE FROM traders")
            public.execute("PRAGMA query_only=OFF")
            with self.assertRaises(sqlite3.OperationalError):
                public.execute("DELETE FROM traders")
        missing = self.directory / "missing.db"
        with self.assertRaises(sqlite3.OperationalError):
            VERIFIER.connect(missing)
        self.assertFalse(missing.exists())


if __name__ == "__main__":
    unittest.main()
