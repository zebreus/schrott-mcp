"""Offline temp-DB contracts, not rollout evidence or a seed implementation test."""
import hashlib
import importlib.util
import json
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("verify-wave-e-postrollout.py")
SPEC = importlib.util.spec_from_file_location("wave_e_verifier", SCRIPT)
VERIFIER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFIER)
SINCE = "2026-10-11T09:00:00Z"
NOTES_FIXTURE = SCRIPT.parent / "fixtures" / "wave-e-20261011-notes.json"
DOSSIER_REVISION = "db6c81e048458b12a0d9fd1aefde6fbcb7320a40"


def load_notes_fixture(path=NOTES_FIXTURE):
    # Frozen Timeline output from db6c81e0, independently checked against all
    # 60 historical manifest digests. Never read current dossiers or Git here.
    fixture = json.loads(path.read_text())
    manifest = VERIFIER.load_manifest()
    if (set(fixture) != {"version", "dossier_revision", "traders"}
            or fixture["version"] != 1
            or fixture["dossier_revision"] != DOSSIER_REVISION
            or manifest["dossier_revision"] != DOSSIER_REVISION[:8]):
        raise ValueError("historical notes fixture revision mismatch")
    rows = fixture["traders"]
    expected = {row["slug"]: row["notes_sha256"] for row in manifest["traders"]}
    if (len(rows) != 60
            or any(set(row) != {"slug", "notes"} for row in rows)
            or len({row["slug"] for row in rows}) != 60
            or {row["slug"] for row in rows} != expected.keys()):
        raise ValueError("historical notes fixture scope mismatch")
    for row in rows:
        if (not isinstance(row["notes"], str)
                or hashlib.sha256(row["notes"].encode()).hexdigest() != expected[row["slug"]]):
            raise ValueError("historical notes fixture digest mismatch: " + row["slug"])
    return {row["slug"]: row["notes"] for row in rows}


class WaveENotesFixtureTests(unittest.TestCase):
    def test_loading_requires_only_static_fixture_and_manifest(self):
        read_text = Path.read_text
        allowed = {NOTES_FIXTURE.resolve(), VERIFIER.MANIFEST.resolve()}

        def static_read(path, *args, **kwargs):
            self.assertIn(path.resolve(), allowed)
            return read_text(path, *args, **kwargs)

        with patch.object(Path, "read_text", static_read), \
                patch.object(subprocess, "run", side_effect=AssertionError("no runtime Git")), \
                patch.object(subprocess, "check_output", side_effect=AssertionError("no runtime Git")):
            self.assertEqual(len(load_notes_fixture()), 60)

    def test_historical_revision_scope_and_all_manifest_hashes(self):
        notes = load_notes_fixture()
        expected = VERIFIER.load_manifest()["traders"]
        self.assertEqual(len(notes), 60)
        self.assertEqual(set(notes), {row["slug"] for row in expected})
        for row in expected:
            with self.subTest(slug=row["slug"]):
                self.assertEqual(hashlib.sha256(notes[row["slug"]].encode()).hexdigest(),
                                 row["notes_sha256"])

    def test_fixture_rejects_wrong_revision_scope_or_content(self):
        base = json.loads(NOTES_FIXTURE.read_text())
        mutations = [lambda f: f.update(version=2),
                     lambda f: f.update(dossier_revision="81a64832"),
                     lambda f: f["traders"].pop(),
                     lambda f: f["traders"].append(f["traders"][0]),
                     lambda f: f["traders"].__setitem__(0, f["traders"][1]),
                     lambda f: f["traders"][0].update(slug="unrelated"),
                     lambda f: f["traders"][0].update(extra="unexpected"),
                     lambda f: f["traders"][0].update(notes=None),
                     lambda f: f["traders"][0].update(notes=""),
                     lambda f: f["traders"][0].update(notes=f["traders"][0]["notes"] + " changed")]
        with tempfile.TemporaryDirectory(dir="/tmp/opencode") as directory:
            path = Path(directory) / "notes.json"
            for index, mutate in enumerate(mutations):
                with self.subTest(mutation=index):
                    fixture = json.loads(json.dumps(base))
                    mutate(fixture)
                    path.write_text(json.dumps(fixture))
                    with self.assertRaises(ValueError):
                        load_notes_fixture(path)


class WaveEPostrolloutTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir="/tmp/opencode")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.db = sqlite3.connect(self.directory / "public.db")
        self.addCleanup(self.db.close)
        self.expected = VERIFIER.load_manifest()["traders"]
        self.notes = load_notes_fixture()
        columns = sorted(VERIFIER.FIELDS | {"slug", "notes", "updated_at", "lat", "lon"})
        self.db.execute("CREATE TABLE traders (" + ",".join(f"{c} TEXT" for c in columns) + ")")
        for expected in self.expected:
            notes = self.notes[expected["slug"]]
            self.assertEqual(hashlib.sha256(notes.encode()).hexdigest(), expected["notes_sha256"])
            values = {"slug": expected["slug"], "notes": notes, "updated_at": SINCE,
                      **expected["values"]}
            self.db.execute("INSERT INTO traders(" + ",".join(values) + ") VALUES ("
                            + ",".join("?" for _ in values) + ")", list(values.values()))
        self.db.commit()

    def verify(self, since=SINCE):
        result = subprocess.run([sys.executable, str(SCRIPT), "--since", since,
                                 "--data-dir", str(self.directory)], capture_output=True, text=True)
        self.assertTrue(result.stdout, result.stderr)
        return result.returncode, json.loads(result.stdout)

    def test_green_and_readonly_bytes(self):
        path = self.directory / "public.db"
        before = path.read_bytes()
        code, result = self.verify()
        self.assertEqual(code, 0)
        self.assertEqual(result["checked_count"], 60)
        self.assertEqual(path.read_bytes(), before)
        with VERIFIER.connect(path) as reader:
            with self.assertRaises(sqlite3.OperationalError):
                reader.execute("DELETE FROM traders")
            reader.execute("PRAGMA query_only=OFF")
            with self.assertRaises(sqlite3.OperationalError):
                reader.execute("DELETE FROM traders")

    def test_every_target_and_clear_rejects_wrong_value(self):
        with VERIFIER.connect(self.directory / "public.db") as reader:
            for expected in self.expected:
                for field, value in expected["values"].items():
                    with self.subTest(slug=expected["slug"], field=field):
                        self.db.execute(f"UPDATE traders SET {field}=? WHERE slug=?",
                                        ("wrong", expected["slug"]))
                        self.db.commit()
                        result = VERIFIER.verify(reader, SINCE)
                        self.assertIn(f"{expected['slug']}: {field} target mismatch", result["failures"])
                        self.db.execute(f"UPDATE traders SET {field}=? WHERE slug=?",
                                        (value, expected["slug"]))
                        self.db.commit()

    def test_missing_or_duplicate_manifest_trader(self):
        slug = self.expected[0]["slug"]
        self.db.execute("INSERT INTO traders SELECT * FROM traders WHERE slug=?", (slug,))
        self.db.commit()
        self.assertEqual(self.verify()[0], 1)
        self.db.execute("DELETE FROM traders WHERE slug=?", (slug,))
        self.db.commit()
        code, result = self.verify()
        self.assertEqual(code, 1)
        self.assertEqual(result["checked_count"], 59)

    def test_notes_missing_changed_or_history_removed(self):
        for notes in (None, "", "Recherche 11.10.2026", self.notes[self.expected[0]["slug"]] + " changed"):
            with self.subTest(notes=notes):
                self.db.execute("UPDATE traders SET notes=? WHERE slug=?", (notes, self.expected[0]["slug"]))
                self.db.commit()
                self.assertEqual(self.verify()[0], 1)

    def test_since_required_valid_and_not_prerollout(self):
        self.assertEqual(self.verify("not-a-time")[0], 1)
        self.assertEqual(self.verify("2026-10-11T10:00:00Z")[0], 1)
        for timestamp in (None, "invalid", "2026-10-11T08:59:59Z"):
            self.db.execute("UPDATE traders SET updated_at=? WHERE slug=?", (timestamp, self.expected[0]["slug"]))
            self.db.commit()
            self.assertEqual(self.verify()[0], 1)
        result = subprocess.run([sys.executable, str(SCRIPT)], capture_output=True, text=True)
        self.assertEqual(result.returncode, 2)

    def test_only_manifest_scope_no_geo_or_blank_enrichment_clear_claim(self):
        self.db.execute("INSERT INTO traders(slug) VALUES ('unrelated')")
        self.db.execute("UPDATE traders SET lat='unvalidated',lon='unvalidated'")
        # Pfragner's blank dossier phone is a keep, not a clear expectation.
        self.db.execute("UPDATE traders SET phone='legacy kept',email='legacy kept' WHERE slug=?",
                        (self.expected[0]["slug"],))
        self.db.commit()
        self.assertEqual(self.verify()[0], 0)

    def test_no_missing_database_creation(self):
        missing = self.directory / "absent.db"
        with self.assertRaises(sqlite3.OperationalError):
            VERIFIER.connect(missing)
        self.assertFalse(missing.exists())

    def test_manifest_fails_closed(self):
        base = json.loads(VERIFIER.MANIFEST.read_text())
        mutations = [lambda m: m["traders"].pop(),
                     lambda m: m["traders"].__setitem__(0, m["traders"][1]),
                     lambda m: m["traders"][0]["values"].update(lat="50"),
                     lambda m: m["traders"][0].update(clear_fields=["street"]),
                     lambda m: m["traders"][0].update(clear_fields=["phone"]),
                     lambda m: m["traders"][0].update(notes_sha256="invalid")]
        path = self.directory / "manifest.json"
        for mutate in mutations:
            manifest = json.loads(json.dumps(base))
            mutate(manifest)
            path.write_text(json.dumps(manifest))
            with patch.object(VERIFIER, "MANIFEST", path):
                with self.assertRaises(ValueError):
                    VERIFIER.load_manifest()


if __name__ == "__main__":
    unittest.main()
