"""Isolated SQLite fixtures, not production evidence or a material oracle."""
import importlib.util
import json
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("verify-kernschrott-release.py")
SPEC = importlib.util.spec_from_file_location("kern_verifier", SCRIPT)
V = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(V)
SINCE = "2026-10-11T00:00:00Z"


class KernReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir="/tmp/opencode")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.public = sqlite3.connect(self.directory / "public.db")
        self.internal = sqlite3.connect(self.directory / "internal.db")
        self.addCleanup(self.public.close)
        self.addCleanup(self.internal.close)
        self.public.executescript("""
            CREATE TABLE traders(id INTEGER PRIMARY KEY,slug TEXT);
            CREATE TABLE materials(id INTEGER PRIMARY KEY,slug TEXT,unit TEXT);
            CREATE TABLE prices(id INTEGER PRIMARY KEY,material_id INTEGER,variant TEXT,price REAL,
                price_kind TEXT,confidence REAL,currency TEXT,unit TEXT,source_url TEXT,
                source_type TEXT,published INTEGER,observed_at TEXT,published_at TEXT,
                extra_json TEXT,notes TEXT);
            CREATE TABLE current_prices(trader_id INTEGER,price_id INTEGER);
            CREATE TABLE trader_materials(trader_id INTEGER,material_id INTEGER,
                accepts INTEGER,conditions TEXT,observed_at TEXT);
        """)
        self.public.execute("INSERT INTO traders VALUES(1,?)", (V.SLUG,))
        names = ["luefter", "platinen", *sorted(V.REQUESTS), *sorted(V.MATERIALS - V.REQUESTS - {"luefter", "platinen"})]
        conditions = V.RESERVATION + " " + V.EXCLUSION
        for ident, name in enumerate(names, 1):
            self.public.execute("INSERT INTO materials VALUES(?,?, 'EUR/kg')", (ident, name))
            aggregate = "Nur die aufgeführten Varianten mit ihren jeweiligen Bedingungen: " + conditions
            if name in V.REQUESTS:
                aggregate += " Preis auf Anfrage"
            if name == "platinen":
                aggregate += "; Leiterplatten Kl. 3 quarantänisiert, keine bestätigte Annahme dieser Variante"
            self.public.execute("INSERT INTO trader_materials VALUES(1,?,1,?,?)", (ident, aggregate, SINCE))
        for ident in range(1, 39):
            self.public.execute("INSERT INTO prices VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)", (
                ident, 1 if ident == 1 else 2, "Lüfter" if ident == 1 else f"fixture-{ident}",
                0 if ident == 1 else 1, "exact", 1, "EUR", "EUR/kg", V.URL,
                "haendler_angabe", 1, SINCE, "2026-10-06T00:00:00Z",
                json.dumps({"published_at_basis": "page_stated"}), conditions))
            self.public.execute("INSERT INTO current_prices VALUES(1,?)", (ident,))
        self.internal.executescript("""
            CREATE TABLE ingestion_steps(id INTEGER PRIMARY KEY,run_id INTEGER,scraper TEXT,
                status TEXT,items_upserted INTEGER,message TEXT,started_at TEXT,finished_at TEXT);
            CREATE TABLE raw_fetches(id INTEGER PRIMARY KEY,run_id INTEGER,scraper TEXT,
                url TEXT,status_code INTEGER,byte_len INTEGER);
        """)
        self.internal.execute("INSERT INTO ingestion_steps VALUES(1,1,?,'ok',38,?,?,?)",
                              (V.SLUG, "38 Preise übernommen, 1 übersprungen (Leiterplatten Kl. 3 quarantänisiert)", SINCE, SINCE))
        self.internal.execute("INSERT INTO raw_fetches VALUES(1,1,?,?,200,100)", (V.SLUG, V.URL))
        self.public.commit()
        self.internal.commit()

    def verify(self):
        run = subprocess.run([sys.executable, str(SCRIPT), "--since", SINCE,
                              "--data-dir", str(self.directory)], capture_output=True, text=True)
        self.assertTrue(run.stdout, run.stderr)
        return run.returncode, json.loads(run.stdout)

    def test_green_zero_quote_and_byte_preservation(self):
        paths = [self.directory / name for name in ("public.db", "internal.db")]
        before = [path.read_bytes() for path in paths]
        self.assertEqual(self.verify()[0], 0)
        for path in paths:
            db = V.connect(path)
            try:
                with self.assertRaises(sqlite3.OperationalError):
                    db.execute("CREATE TABLE forbidden(id INTEGER)")
            finally:
                db.close()
        self.assertEqual(before, [path.read_bytes() for path in paths])

    def test_independent_bad_quote_contracts_fail(self):
        for assignment in ("price=0", "notes=''", "observed_at='2020-01-01'", "source_url='wrong'",
                           "published_at=NULL", "extra_json='[]'", "variant='Leiterplatten Kl. 3'",
                           "material_id=3", "confidence=0.8", "unit='EUR/t'"):
            with self.subTest(assignment=assignment):
                self.public.execute("SAVEPOINT mutation")
                self.public.execute(f"UPDATE prices SET {assignment} WHERE id=2")
                result = V.verify(self.public_as_rows(), self.internal_as_rows(), SINCE)
                self.assertFalse(result["ok"])
                self.public.execute("ROLLBACK TO mutation")
                self.public.execute("RELEASE mutation")

    def public_as_rows(self):
        self.public.row_factory = sqlite3.Row
        return self.public

    def internal_as_rows(self):
        self.internal.row_factory = sqlite3.Row
        return self.internal

    def test_lost_acceptance_conditions_fail(self):
        self.public.execute("UPDATE trader_materials SET conditions='',accepts=0")
        self.public.commit()
        self.assertEqual(self.verify()[0], 1)

    def test_missing_step_bad_fetch_and_canary_fail(self):
        for table, assignment in (("raw_fetches", "status_code=500"),
                                  ("ingestion_steps", "message='CANARY'"),
                                  ("ingestion_steps", "started_at='2020-01-01'")):
            with self.subTest(assignment=assignment):
                self.internal.execute("SAVEPOINT mutation")
                self.internal.execute(f"UPDATE {table} SET {assignment}")
                self.assertFalse(V.verify(self.public_as_rows(), self.internal_as_rows(), SINCE)["ok"])
                self.internal.execute("ROLLBACK TO mutation")
                self.internal.execute("RELEASE mutation")


if __name__ == "__main__":
    unittest.main()
