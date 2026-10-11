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


def dossier_notes(slug):
    # Fixture extraction follows documented build.rs Timeline formatting only;
    # actual rehearsal evidence was independently taken from the seeded DB.
    path = SCRIPT.parent.parent / "dossiers" / slug[:2] / (slug + ".md")
    bullets = []
    timeline = False
    for line in path.read_text().splitlines():
        if not timeline:
            timeline = line.rstrip() == "## Timeline"
            continue
        if line.startswith("## ") and line[3:4] and not line[3].isspace():
            break
        text = line.strip()
        if text.startswith("- "):
            bullets.append(text[2:].strip())
        elif text and not text.startswith("#") and bullets:
            bullets[-1] += " " + text
    return " | ".join(" ".join(bullet.split()) for bullet in bullets)


class WaveEPostrolloutTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir="/tmp/opencode")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.db = sqlite3.connect(self.directory / "public.db")
        self.addCleanup(self.db.close)
        self.expected = VERIFIER.load_manifest()["traders"]
        columns = sorted(VERIFIER.FIELDS | {"slug", "notes", "updated_at", "lat", "lon"})
        self.db.execute("CREATE TABLE traders (" + ",".join(f"{c} TEXT" for c in columns) + ")")
        for expected in self.expected:
            notes = dossier_notes(expected["slug"])
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
        for notes in (None, "", "Recherche 11.10.2026", dossier_notes(self.expected[0]["slug"]) + " changed"):
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
