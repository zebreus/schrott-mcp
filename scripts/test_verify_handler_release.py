"""Local fixture tests for the read-only release verifier, not prod evidence."""
import importlib.util
import json
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("verify-handler-release.py")
SPEC = importlib.util.spec_from_file_location("verifier", SCRIPT)
VERIFIER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFIER)
SINCE = "2026-10-09T20:00:00Z"


class VerifyReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir="/tmp/opencode")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.public = sqlite3.connect(self.directory / "public.db")
        self.internal = sqlite3.connect(self.directory / "internal.db")
        self.addCleanup(self.public.close)
        self.addCleanup(self.internal.close)
        self.public.executescript("""
            CREATE TABLE traders (id INTEGER PRIMARY KEY, slug TEXT);
            CREATE TABLE materials (id INTEGER PRIMARY KEY, slug TEXT, unit TEXT);
            CREATE TABLE prices (id INTEGER PRIMARY KEY, price REAL, currency TEXT,
                unit TEXT, price_kind TEXT, confidence REAL, source_type TEXT,
                published INTEGER, source_url TEXT, observed_at TEXT,
                published_at TEXT, extra_json TEXT, material_id INTEGER DEFAULT 1);
            CREATE TABLE current_prices (trader_id INTEGER, material_id INTEGER,
                price_id INTEGER);
            INSERT INTO materials VALUES (1, 'fixture-material', 'EUR/g');
        """)
        self.internal.executescript("""
            CREATE TABLE ingestion_steps (id INTEGER PRIMARY KEY, run_id INTEGER,
                scraper TEXT, status TEXT, items_upserted INTEGER, message TEXT,
                started_at TEXT, finished_at TEXT);
            CREATE TABLE raw_fetches (id INTEGER PRIMARY KEY, run_id INTEGER,
                scraper TEXT, url TEXT, status_code INTEGER, byte_len INTEGER);
        """)
        price_id = 0
        for trader, (slug, (count, kind, dated, skips)) in enumerate(VERIFIER.HANDLERS.items(), 1):
            url = f"https://fixture.invalid/{slug}"
            self.public.execute("INSERT INTO traders VALUES (?, ?)", (trader, slug))
            for _ in range(count):
                price_id += 1
                self.public.execute("INSERT INTO prices (id,price,currency,unit,price_kind,confidence,source_type,published,source_url,observed_at,published_at,extra_json) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)", (
                    price_id, 1.0, "EUR", "EUR/g", kind, 0.8 if kind == "approx" else 1.0,
                    "haendler_angabe", 1, url, SINCE, SINCE if dated else None,
                    json.dumps({"published_at_basis": "page_stated"} if dated else {}),
                ))
                self.public.execute("INSERT INTO current_prices VALUES (?,1,?)", (trader, price_id))
            message = f"{count} Preise übernommen" + (f", {skips} übersprungen" if skips else "")
            self.internal.execute("INSERT INTO ingestion_steps VALUES (?,?,?,?,?,?,?,?)",
                                  (trader, 1, slug, "ok", count, message, SINCE, SINCE))
            self.internal.execute("INSERT INTO raw_fetches VALUES (?,?,?,?,?,?)",
                                  (trader, 1, slug, url, 200, 100))
        self.public.commit()
        self.internal.commit()

    def verify(self):
        result = subprocess.run([sys.executable, str(SCRIPT), "--since", SINCE,
                                 "--data-dir", str(self.directory)], capture_output=True, text=True)
        self.assertTrue(result.stdout, result.stderr)
        return result.returncode, json.loads(result.stdout)

    def test_complete_fixture_passes_without_database_changes(self):
        before = [(path.read_bytes()) for path in (self.directory / "public.db", self.directory / "internal.db")]
        code, results = self.verify()
        self.assertEqual(code, 0)
        self.assertTrue(all(result["ok"] for result in results))
        after = [(path.read_bytes()) for path in (self.directory / "public.db", self.directory / "internal.db")]
        self.assertEqual(before, after)

    def test_missing_recorded_step_fails(self):
        self.internal.execute("DELETE FROM ingestion_steps WHERE id=1")
        self.internal.commit()
        code, results = self.verify()
        self.assertEqual(code, 1)
        self.assertIn("no post-rollout step", results[0]["failures"])

    def test_wrong_semantics_provenance_units_and_freshness_fail(self):
        self.public.execute("UPDATE prices SET price_kind='approx', unit='EUR/t', "
                            "source_url='wrong', observed_at='2020-01-01' WHERE id=1")
        self.public.commit()
        code, results = self.verify()
        self.assertEqual(code, 1)
        self.assertEqual(set(results[0]["failures"]), {
            "quote uncertainty mismatch", "invalid normalized quote",
            "price/fetch provenance mismatch", "stale pre-rollout observation",
        })

    def test_inferred_date_allowed_only_with_central_policy_marker(self):
        self.public.execute("UPDATE prices SET published_at=?, extra_json=? WHERE id=1",
                            (SINCE, json.dumps({"published_at_basis": "observed_price_change"})))
        self.public.commit()
        self.assertEqual(self.verify()[0], 0)
        self.public.execute("UPDATE prices SET extra_json='{}' WHERE id=1")
        self.public.commit()
        self.assertEqual(self.verify()[0], 1)


if __name__ == "__main__":
    unittest.main()
