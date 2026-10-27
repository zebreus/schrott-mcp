"""Offline SQLite contract tests; these fixtures are not rollout evidence."""
import importlib.util
import json
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("verify-pending-corrections.py")
SPEC = importlib.util.spec_from_file_location("pending_verifier", SCRIPT)
VERIFIER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFIER)
SINCE = "2026-10-11T00:00:00Z"


class PendingCorrectionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir="/tmp/opencode")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.public = sqlite3.connect(self.directory / "public.db")
        self.internal = sqlite3.connect(self.directory / "internal.db")
        self.addCleanup(self.public.close)
        self.addCleanup(self.internal.close)
        self.public.executescript("""
            CREATE TABLE traders (id INTEGER PRIMARY KEY, slug TEXT, phone TEXT DEFAULT '',
                website TEXT DEFAULT '', updated_at TEXT, dropoff_json TEXT DEFAULT '{}',
                pickup_json TEXT DEFAULT '{}', street TEXT, postcode TEXT, city TEXT);
            CREATE TABLE materials (id INTEGER PRIMARY KEY, slug TEXT, unit TEXT);
            CREATE TABLE prices (id INTEGER PRIMARY KEY, material_id INTEGER, variant TEXT,
                extra_json TEXT, price_kind TEXT, confidence REAL, price REAL, unit TEXT,
                currency TEXT, published INTEGER, source_type TEXT, observed_at TEXT,
                published_at TEXT, source_url TEXT);
            CREATE TABLE current_prices (trader_id INTEGER, price_id INTEGER);
            INSERT INTO materials VALUES (1, 'fixture', 'EUR/kg');
        """)
        self.internal.executescript("""
            CREATE TABLE ingestion_steps (id INTEGER PRIMARY KEY, run_id INTEGER,
                scraper TEXT, status TEXT, items_upserted INTEGER, message TEXT,
                started_at TEXT, finished_at TEXT);
            CREATE TABLE raw_fetches (run_id INTEGER, scraper TEXT, url TEXT,
                status_code INTEGER, byte_len INTEGER);
        """)
        for slug in (*VERIFIER.CLEARS, VERIFIER.PORTAL, VERIFIER.GOTHA, VERIFIER.MSG):
            self.public.execute("INSERT INTO traders(slug,updated_at) VALUES (?,?)", (slug, SINCE))
        self.public.execute("UPDATE traders SET street='An der Ostbahn 4',postcode='99867',city='Gotha' WHERE slug=?", (VERIFIER.GOTHA,))
        trader_id = self.public.execute("SELECT id FROM traders WHERE slug=?", (VERIFIER.MSG,)).fetchone()[0]
        for ident in range(1, 125):
            extra = {"snapshot_owner": "msg-visible-tiers-v1", "source_title": f"Grade {ident}",
                     "quantity_min_text": "100", "quantity_min": 100, "quantity_unit": "kg",
                     "source_price_unit": "EUR/kg"}
            self.public.execute("INSERT INTO prices VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?)", (
                ident, 1, f"Grade {ident} / ab 100 kg", json.dumps(extra), "exact", 1.0,
                1.2, "EUR/kg", "EUR", 1, "haendler_angabe", SINCE, None, VERIFIER.MSG_URL))
            self.public.execute("INSERT INTO current_prices VALUES (?,?)", (trader_id, ident))
        for ident, (slug, count) in enumerate(((VERIFIER.GOTHA, 19), (VERIFIER.MSG, 124)), 1):
            self.internal.execute("INSERT INTO ingestion_steps VALUES (?,?,?,?,?,?,?,?)",
                                  (ident, ident, slug, "ok", count, "ok", SINCE, SINCE))
            self.internal.execute("INSERT INTO raw_fetches VALUES (?,?,?,?,?)",
                                  (ident, slug, VERIFIER.MSG_URL if slug == VERIFIER.MSG else VERIFIER.GOTHA_URL, 200, 100))
        self.public.commit()
        self.internal.commit()

    def verify(self):
        result = subprocess.run([sys.executable, str(SCRIPT), "--since", SINCE,
                                 "--data-dir", str(self.directory)], capture_output=True, text=True)
        self.assertTrue(result.stdout, result.stderr)
        return result.returncode, json.loads(result.stdout)

    def test_green_fixture_preserves_database_bytes_and_readonly_enforced(self):
        paths = [self.directory / name for name in ("public.db", "internal.db")]
        before = [path.read_bytes() for path in paths]
        self.assertEqual(self.verify()[0], 0)
        for path in paths:
            db = VERIFIER.connect(path)
            try:
                with self.assertRaises(sqlite3.OperationalError):
                    db.execute("CREATE TABLE forbidden (id INTEGER)")
            finally:
                db.close()
        self.assertEqual(before, [path.read_bytes() for path in paths])

    def test_legacy_contacts_portal_and_hybrid_address_fail(self):
        self.public.execute("UPDATE traders SET phone='legacy',website='legacy',pickup_json='[]',street='Hauptstraße 46',postcode='99947'")
        self.public.commit()
        code, result = self.verify()
        self.assertEqual(code, 1)
        self.assertTrue(any("clear values" in failure for failure in result["failures"]))
        self.assertIn("Gutzmann: branch address incorrect", result["failures"])
        self.assertIn("Schrott24: pickup_json is not an explicit empty object", result["failures"])

    def test_stale_seed_quotes_and_step_fail(self):
        self.public.execute("UPDATE traders SET updated_at='2020-01-01'")
        self.public.execute("UPDATE prices SET observed_at='2020-01-01'")
        self.internal.execute("UPDATE ingestion_steps SET started_at='2020-01-01'")
        self.public.commit()
        self.internal.commit()
        code, result = self.verify()
        self.assertEqual(code, 1)
        self.assertIn("Gutzmann: post-rollout seed evidence missing", result["failures"])
        self.assertNotIn("Gutzmann: branch address incorrect", result["failures"])
        self.assertIn(
            "hh-iserbrook-22589-hamburger-schrotthandel-pruffall-hamburg: post-rollout seed evidence missing",
            result["failures"],
        )

    def test_fetch_error_wrong_provenance_and_latest_failed_step_fail(self):
        for update in ("UPDATE raw_fetches SET status_code=500", "UPDATE raw_fetches SET status_code=200,url='wrong'", "UPDATE ingestion_steps SET status='error'"):
            with self.subTest(update=update):
                self.internal.execute("UPDATE raw_fetches SET status_code=200,url=? WHERE scraper=?", (VERIFIER.MSG_URL, VERIFIER.MSG))
                self.internal.execute("UPDATE raw_fetches SET status_code=200,url=? WHERE scraper=?", (VERIFIER.GOTHA_URL, VERIFIER.GOTHA))
                self.internal.execute("UPDATE ingestion_steps SET status='ok'")
                self.internal.execute(update)
                self.internal.commit()
                self.assertEqual(self.verify()[0], 1)

    def test_canary_wrong_count_and_unfinished_step_fail(self):
        for assignment in ("message='CANARY'", "items_upserted=123", "finished_at=NULL"):
            with self.subTest(assignment=assignment):
                self.internal.execute("UPDATE ingestion_steps SET message='ok',items_upserted=124,finished_at=? WHERE scraper=?", (SINCE, VERIFIER.MSG))
                self.internal.execute(f"UPDATE ingestion_steps SET {assignment} WHERE scraper=?", (VERIFIER.MSG,))
                self.internal.commit()
                self.assertEqual(self.verify()[0], 1)

    def test_inferred_date_requires_central_marker(self):
        extra = json.loads(self.public.execute("SELECT extra_json FROM prices WHERE id=1").fetchone()[0])
        extra["published_at_basis"] = "observed_price_change"
        self.public.execute("UPDATE prices SET published_at=?,extra_json=? WHERE id=1", (SINCE, json.dumps(extra)))
        self.public.commit()
        self.assertEqual(self.verify()[0], 0)
        extra["published_at_basis"] = "page_stated"
        self.public.execute("UPDATE prices SET extra_json=? WHERE id=1", (json.dumps(extra),))
        self.public.commit()
        self.assertEqual(self.verify()[0], 1)

    def test_metadata_legacy_invalid_quantity_and_publication_fail(self):
        for extra in ("{}", "[]", "not json", json.dumps({"snapshot_owner": "msg-visible-tiers-v1", "source_title": "Grade 1", "quantity_min_text": "100", "quantity_min": 99, "quantity_unit": "kg", "source_price_unit": "EUR/kg"})):
            with self.subTest(extra=extra):
                self.public.execute("UPDATE prices SET extra_json=? WHERE id=1", (extra,))
                self.public.commit()
                self.assertEqual(self.verify()[0], 1)

    def test_semantics_and_missing_current_fail(self):
        self.public.execute("UPDATE prices SET price_kind='approx',confidence=0.8,price=-1,unit='EUR/t',published_at=? WHERE id=1", (SINCE,))
        self.public.execute("DELETE FROM current_prices WHERE price_id=2")
        self.public.commit()
        code, result = self.verify()
        self.assertEqual(code, 1)
        self.assertIn("MSG: unsupported publication date", result["failures"])
        self.assertIn("MSG: 123 managed quotes instead of reviewed 124", result["failures"])


if __name__ == "__main__":
    unittest.main()
