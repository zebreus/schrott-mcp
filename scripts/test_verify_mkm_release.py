"""Offline verifier tests; actual reviewed article fixture, synthetic SQLite state."""
import importlib.util
import json
import sqlite3
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("verify-mkm-release.py")
SPEC = importlib.util.spec_from_file_location("mkm_verifier", SCRIPT)
V = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(V)
SINCE = "2026-10-11T03:00:00.000000000+00:00"


class MkmReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(dir="/tmp/opencode")
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)
        self.public = sqlite3.connect(self.directory / "public.db")
        self.internal = sqlite3.connect(self.directory / "internal.db")
        self.public.row_factory = self.internal.row_factory = sqlite3.Row
        self.addCleanup(self.public.close)
        self.addCleanup(self.internal.close)
        self.public.executescript("""
            CREATE TABLE traders(id INTEGER PRIMARY KEY,slug TEXT);
            CREATE TABLE materials(id INTEGER PRIMARY KEY,slug TEXT,unit TEXT);
            CREATE TABLE prices(id INTEGER PRIMARY KEY,material_id INTEGER,variant TEXT,
                price REAL,price_max REAL,price_min REAL,unit TEXT,currency TEXT,price_kind TEXT,
                confidence REAL,extra_json TEXT,published_at TEXT,valid_from TEXT,valid_to TEXT,
                source_type TEXT,published INTEGER,observed_at TEXT,source_url TEXT);
            CREATE TABLE current_prices(trader_id INTEGER,price_id INTEGER);
            CREATE TABLE trader_materials(trader_id INTEGER,material_id INTEGER,conditions TEXT,
                accepts INTEGER,valid_from TEXT,valid_to TEXT,observed_at TEXT);
            CREATE TABLE source_article_snapshots(id INTEGER PRIMARY KEY,trader_id INTEGER,
                source_url TEXT,owner TEXT,observed_at TEXT,manifest_json TEXT);
            CREATE TABLE article_acceptance_owners(trader_id INTEGER,material_id INTEGER,
                source_url TEXT,owner TEXT,conditions TEXT,observed_at TEXT);
        """)
        self.internal.executescript("""
            CREATE TABLE ingestion_steps(id INTEGER PRIMARY KEY,run_id INTEGER,scraper TEXT,
                status TEXT,items_upserted INTEGER,started_at TEXT,finished_at TEXT,message TEXT);
            CREATE TABLE raw_fetches(id INTEGER PRIMARY KEY,run_id INTEGER,scraper TEXT,
                url TEXT,status_code INTEGER,byte_len INTEGER);
        """)
        self.public.execute("INSERT INTO traders VALUES(1,?)", (V.SLUG,))
        inventory = json.loads((SCRIPT.parent.parent / "research/mkm-f6-registered-live-inventory-20261011.json").read_text())
        self.manifest = {"owner": V.OWNER, "trader_id": 1, "source_url": V.URL,
                         "observed_at": SINCE, "page_change_date": inventory["page_change_date"],
                         "articles": inventory["articles"]}
        self.public.execute("INSERT INTO source_article_snapshots VALUES(1,1,?,?,?,?)",
                            (V.URL, V.OWNER, SINCE, json.dumps(self.manifest)))
        grouped = {}
        mids = {}
        ident = 0
        for article in self.manifest["articles"]:
            material = article["material"]
            if material not in mids:
                mids[material] = len(mids) + 1
                self.public.execute("INSERT INTO materials VALUES(?,?,?)", (mids[material], material,
                                    "EUR/t" if material in ("mischschrott", "schrott-eisen-ne-gemischt") else "EUR/kg"))
            semantic = dict(article)
            semantic.pop("product_change_text", None)
            grouped.setdefault(material, []).append(semantic)
            if article["request_only"]:
                continue
            ident += 1
            extra = {"snapshot_owner": V.OWNER, "article": semantic,
                     "page_change_date": self.manifest["page_change_date"]}
            self.public.execute("INSERT INTO prices VALUES(?,?,?,?,?,NULL,'EUR/kg','EUR','upto',0.5,?,NULL,NULL,NULL,'haendler_angabe',1,?,?)",
                                (ident, mids[material], article["variant"], article["price_eur_per_kg"],
                                 article["price_eur_per_kg"], json.dumps(extra), SINCE, V.URL))
            self.public.execute("INSERT INTO current_prices VALUES(1,?)", (ident,))
        for material, members in grouped.items():
            conditions = json.dumps(sorted(members, key=lambda a: a["article"]))
            self.public.execute("INSERT INTO trader_materials VALUES(1,?,?,1,NULL,NULL,?)", (mids[material], conditions, SINCE))
            self.public.execute("INSERT INTO article_acceptance_owners VALUES(1,?,?,?,?,?)", (mids[material], V.URL, V.OWNER, conditions, SINCE))
        self.internal.execute("INSERT INTO ingestion_steps VALUES(1,1,?,'ok',55,?,?,?)", (V.SLUG, SINCE, SINCE, "55 Preise übernommen, 5 Annahmen"))
        self.internal.execute("INSERT INTO raw_fetches VALUES(1,1,?,?,200,100)", (V.SLUG, V.URL))
        self.public.commit()
        self.internal.commit()

    def verify(self):
        run = subprocess.run([sys.executable, str(SCRIPT), "--since", SINCE,
                              "--data-dir", str(self.directory)], capture_output=True, text=True)
        self.assertTrue(run.stdout, run.stderr)
        return run.returncode, json.loads(run.stdout)

    def test_complete_fixture_passes_and_is_readonly(self):
        paths = [self.directory / name for name in ("public.db", "internal.db")]
        before = [p.read_bytes() for p in paths]
        code, result = self.verify()
        self.assertEqual(code, 0, result)
        self.assertEqual(result["articles"], 60)
        for path in paths:
            db = V.connect(path)
            try:
                with self.assertRaises(sqlite3.OperationalError):
                    db.execute("CREATE TABLE forbidden(id INTEGER)")
            finally:
                db.close()
        self.assertEqual(before, [p.read_bytes() for p in paths])

    def test_independent_price_and_metadata_mutations_fail(self):
        for assignment in ("price=0", "price_max=0", "unit='EUR/t'", "extra_json='{}'",
                           "published_at='2026-10-11'", "observed_at='2020-01-01'", "source_url='foreign'",
                           "source_type='telefonisch'", "confidence=1", "variant='legacy alias'"):
            with self.subTest(assignment=assignment):
                self.public.execute("SAVEPOINT mutation")
                self.public.execute(f"UPDATE prices SET {assignment} WHERE id=1")
                self.assertFalse(V.verify(self.public, self.internal, SINCE)["ok"])
                self.public.execute("ROLLBACK TO mutation")
                self.public.execute("RELEASE mutation")

    def test_acceptance_owner_mutations_fail(self):
        for table, assignment in (("trader_materials", "conditions='[]'"), ("trader_materials", "accepts=0"),
                                  ("article_acceptance_owners", "owner='foreign'"),
                                  ("article_acceptance_owners", "observed_at='2020-01-01'")):
            with self.subTest(table=table, assignment=assignment):
                self.public.execute("SAVEPOINT mutation")
                self.public.execute(f"UPDATE {table} SET {assignment}")
                self.assertFalse(V.verify(self.public, self.internal, SINCE)["ok"])
                self.public.execute("ROLLBACK TO mutation")
                self.public.execute("RELEASE mutation")

    def test_catalog_normalized_money_not_source_metadata(self):
        mid = self.public.execute("SELECT material_id FROM prices WHERE id=1").fetchone()[0]
        self.public.execute("UPDATE materials SET unit='EUR/t' WHERE id=?", (mid,))
        self.public.execute("UPDATE prices SET price=price*1000,price_max=price_max*1000,unit='EUR/t' WHERE material_id=?", (mid,))
        self.public.commit()
        code, result = self.verify()
        self.assertEqual(code, 0, result)
        self.public.execute("UPDATE prices SET price_max=price_max/1000 WHERE id=1")
        self.public.commit()
        self.assertEqual(self.verify()[0], 1)

    def test_unknown_same_source_current_is_not_silently_ignored(self):
        self.public.execute("INSERT INTO current_prices VALUES(1,1)")
        self.public.commit()
        code, result = self.verify()
        self.assertEqual(code, 1)
        self.assertTrue(any("unexpected same-source" in f for f in result["failures"]))

    def test_schema_and_manifest_missing_fail(self):
        self.public.execute("DROP TABLE source_article_snapshots")
        self.public.commit()
        self.assertEqual(self.verify()[0], 1)

    def test_duplicate_manifest_identity_fails(self):
        self.manifest["articles"][1] = self.manifest["articles"][0]
        self.public.execute("UPDATE source_article_snapshots SET manifest_json=?", (json.dumps(self.manifest),))
        self.public.commit()
        self.assertEqual(self.verify()[0], 1)

    def test_bad_fetch_and_late_ledger_binding_fail(self):
        self.internal.execute("UPDATE raw_fetches SET status_code=500")
        self.internal.commit()
        self.assertEqual(self.verify()[0], 1)
        self.internal.execute("UPDATE raw_fetches SET status_code=200")
        self.internal.execute("UPDATE ingestion_steps SET started_at='2026-10-12T00:00:00Z',finished_at='2026-10-12T01:00:00Z'")
        self.internal.commit()
        self.assertEqual(self.verify()[0], 1)


if __name__ == "__main__":
    unittest.main()
