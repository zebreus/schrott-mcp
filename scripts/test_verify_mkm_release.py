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
SINCE = "2026-10-18T18:03:00.000000000+00:00"


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
            CREATE TABLE current_prices(trader_id INTEGER,price_id INTEGER,updated_at TEXT);
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
        # Preserve the historical file. This extra article is independently
        # reviewed source data; bookkeeping below remains synthetic, not a run.
        inventory["articles"].append({
            "article": "MKM-MS-01", "group": "Ankaufpreise für Messing & Rotguss",
            "label": "Messing 58 blank neu", "material": "messing",
            "variant": "Messing 58 blank neu",
            "url": "https://mkm-metals.de/metallankauf/produkt/messing-58-blank-neu",
            "request_only": False, "price_cell": "6,51 € /kg ab 500 kg",
            "price_eur_per_kg": 6.51, "minimum_for_upper_bound_kg": 500.0,
            "product_change_date": "2026-10-18", "product_change_text": "heute",
            "tiers": [{"ab": 0.0, "preis": 6.2}, {"ab": 5.0, "preis": 6.32},
                      {"ab": 500.0, "preis": 6.51}],
        })
        # The synthetic state follows the reviewed partition of 24.11.2026:
        # the fourteen articles on request, the two articles in the reviewed
        # flat price form, the remaining forty-five with the quantity ladder.
        # The values stay the historical ones; only the form is the reviewed
        # one, so the test asks a question about the contract, not about the
        # prices of that day.
        oracle = json.loads(V.ORACLE.read_text())
        self.oracle = oracle
        self.request_ids = set(oracle["partition"]["request"])
        self.flat_ids = set(oracle["flat_quote_articles"])
        for article in inventory["articles"]:
            if article["article"] in self.request_ids:
                article["request_only"] = True
                article["price_eur_per_kg"] = None
                article["minimum_for_upper_bound_kg"] = None
                article["price_cell"] = "auf Anfrage"
                article["tiers"] = []
                article["product_change_date"] = None
                article["product_change_text"] = "ohne Datum"
            elif article["article"] in self.flat_ids:
                article["tiers"] = []
                article["flat_quote"] = True
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
            self.public.execute("INSERT INTO current_prices VALUES(1,?,?)", (ident, SINCE))
        for material, members in grouped.items():
            conditions = json.dumps(sorted(members, key=lambda a: a["article"]))
            self.public.execute("INSERT INTO trader_materials VALUES(1,?,?,1,NULL,NULL,?)", (mids[material], conditions, SINCE))
            self.public.execute("INSERT INTO article_acceptance_owners VALUES(1,?,?,?,?,?)", (mids[material], V.URL, V.OWNER, conditions, SINCE))
        self.internal.execute("INSERT INTO ingestion_steps VALUES(1,1,?,'ok',?,?,?,?)",
                              (V.SLUG, len(self.oracle["partition"]["numeric"]), SINCE, SINCE,
                               "%d Preise übernommen, %d Annahmen" % (len(self.oracle["partition"]["numeric"]),
                                                                    len(self.oracle["partition"]["request"]))))
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
        self.assertEqual((result["articles"], result["numeric"], result["requests"], result["current_count"]),
                         (61, len(self.oracle["partition"]["numeric"]),
                          len(self.oracle["partition"]["request"]), len(self.oracle["partition"]["numeric"])))
        for path in paths:
            db = V.connect(path)
            try:
                with self.assertRaises(sqlite3.OperationalError):
                    db.execute("CREATE TABLE forbidden(id INTEGER)")
            finally:
                db.close()
        self.assertEqual(before, [p.read_bytes() for p in paths])

    def test_the_reviewed_oracle_keeps_the_mapping_and_adds_the_partition(self):
        previous = json.loads((V.ORACLE.parent / "mkm-reviewed-mapping-oracle-20261018.json").read_text())
        self.assertEqual(self.oracle["expected"], previous["expected"])
        self.assertEqual(self.oracle["previous_oracle"],
                         "crates/ingestion/src/traders/handlers/fixtures/mkm-reviewed-mapping-oracle-20261018.json")
        partition = self.oracle["partition"]
        self.assertEqual(len(partition["numeric"]), 47)
        self.assertEqual(len(partition["request"]), 14)
        self.assertEqual(set(partition["numeric"]) | set(partition["request"]), set(self.oracle["expected"]))
        self.assertFalse(set(partition["numeric"]) & set(partition["request"]))
        self.assertEqual(sorted(self.oracle["flat_quote_articles"]), ["MKM-NE-05", "SW10041"])

    def test_the_oracle_partition_matches_the_independent_source_review(self):
        review = json.loads((SCRIPT.parent.parent / "research/mkm-partition-review-20261124.json").read_text())
        self.assertEqual(self.oracle["partition"]["numeric"], review["partition"]["numeric"])
        self.assertEqual(self.oracle["partition"]["request"], review["partition"]["request"])
        self.assertEqual(self.oracle["flat_quote_articles"], review["flat_quote_articles"])
        self.assertEqual(len(review["rows"]), 61)

    def manifest_with(self, ident, request_only=None, price=None):
        changed = json.loads(json.dumps(self.manifest))
        for article in changed["articles"]:
            if article["article"] == ident:
                if request_only is not None:
                    article["request_only"] = request_only
                if price is not None:
                    article["price_eur_per_kg"] = price
                    article["price_cell"] = "%s € /kg ab 500 kg" % price
        return changed

    def test_a_partition_that_leaves_the_review_is_red(self):
        cases = {
            "priced-on-request": (sorted(self.request_ids)[0], "article partition differs from the reviewed partition"),
            "request-with-price": (sorted(self.request_ids)[0], "request article contains numeric contract"),
        }
        for case, (ident, message) in cases.items():
            with self.subTest(case=case):
                self.public.execute("SAVEPOINT mutation")
                if case == "priced-on-request":
                    changed = self.manifest_with(ident, request_only=False)
                else:
                    changed = self.manifest_with(ident, price=1.23)
                self.public.execute("UPDATE source_article_snapshots SET manifest_json=?", (json.dumps(changed),))
                result = V.verify(self.public, self.internal, SINCE)
                self.assertFalse(result["ok"])
                self.assertIn(message, result["failures"])
                self.public.execute("ROLLBACK TO mutation")
                self.public.execute("RELEASE mutation")
        with self.subTest(case="wrong-step-count"):
            self.internal.execute("SAVEPOINT mutation")
            self.internal.execute("UPDATE ingestion_steps SET items_upserted=56")
            self.assertFalse(V.verify(self.public, self.internal, SINCE)["ok"])
            self.internal.execute("ROLLBACK TO mutation")
            self.internal.execute("RELEASE mutation")

    def test_a_flat_form_outside_the_reviewed_set_is_red(self):
        self.public.execute("SAVEPOINT mutation")
        changed = json.loads(json.dumps(self.manifest))
        ident = sorted(set(self.oracle["partition"]["numeric"]) - self.flat_ids)[0]
        for article in changed["articles"]:
            if article["article"] == ident:
                article["tiers"] = []
                article["flat_quote"] = True
        self.public.execute("UPDATE source_article_snapshots SET manifest_json=?", (json.dumps(changed),))
        result = V.verify(self.public, self.internal, SINCE)
        self.assertFalse(result["ok"])
        self.assertIn("invalid numeric metadata/tier contract", result["failures"])
        self.public.execute("ROLLBACK TO mutation")
        self.public.execute("RELEASE mutation")

    def test_historical_60_and_unknown_62_or_same_count_article_are_red(self):
        original = json.dumps(self.manifest)
        for case in ("historical-60", "unknown-62", "same-count-unknown"):
            with self.subTest(case=case):
                changed = json.loads(original)
                if case == "historical-60":
                    changed["articles"] = [a for a in changed["articles"] if a["article"] != "MKM-MS-01"]
                elif case == "unknown-62":
                    extra = dict(changed["articles"][-1], article="MKM-MS-02")
                    changed["articles"].append(extra)
                else:
                    changed["articles"][-1]["article"] = "MKM-MS-02"
                self.public.execute("UPDATE source_article_snapshots SET manifest_json=?", (json.dumps(changed),))
                result = V.verify(self.public, self.internal, SINCE)
                self.assertFalse(result["ok"])
                self.assertIn("reviewed 61-article partition mismatch", result["failures"])

    def test_ms58_missing_quote_reused_variant_or_missing_acceptance_member_are_red(self):
        for case in ("missing-quote", "reuse-old-variant", "missing-acceptance-member"):
            with self.subTest(case=case):
                self.public.execute("SAVEPOINT mutation")
                if case == "missing-quote":
                    self.public.execute("DELETE FROM current_prices WHERE price_id IN (SELECT id FROM prices WHERE variant='Messing 58 blank neu')")
                elif case == "reuse-old-variant":
                    self.public.execute("UPDATE prices SET variant='Messing 58 Abfälle' WHERE variant='Messing 58 blank neu'")
                else:
                    mid = self.public.execute("SELECT id FROM materials WHERE slug='messing'").fetchone()[0]
                    members = [dict(a) for a in self.manifest["articles"] if a["material"] == "messing" and a["article"] != "MKM-MS-01"]
                    for a in members:
                        a.pop("product_change_text", None)
                    conditions = json.dumps(sorted(members, key=lambda a: a["article"]))
                    self.public.execute("UPDATE trader_materials SET conditions=? WHERE material_id=?", (conditions, mid))
                    self.public.execute("UPDATE article_acceptance_owners SET conditions=? WHERE material_id=?", (conditions, mid))
                self.assertFalse(V.verify(self.public, self.internal, SINCE)["ok"])
                self.public.execute("ROLLBACK TO mutation")
                self.public.execute("RELEASE mutation")

    def test_independent_price_and_metadata_mutations_fail(self):
        for assignment in ("price=0", "price_max=0", "unit='EUR/t'", "extra_json='{}'",
                           "published_at='2026-10-11'", "observed_at='2099-01-01'", "source_url='foreign'",
                           "source_type='telefonisch'", "confidence=1", "variant='legacy alias'"):
            with self.subTest(assignment=assignment):
                self.public.execute("SAVEPOINT mutation")
                self.public.execute(f"UPDATE prices SET {assignment} WHERE id=1")
                self.assertFalse(V.verify(self.public, self.internal, SINCE)["ok"])
                self.public.execute("ROLLBACK TO mutation")
                self.public.execute("RELEASE mutation")

    def test_stale_current_pointer_fails(self):
        # Der MKM-Vertrag macht die Preis-Historie unveraenderlich: der
        # Zeitstempel der Zeile ist die ERSTE Beobachtung des Wertes. Die
        # Frische der Beobachtung traegt der Current-Zeiger - ohne ihn
        # waere die Pruefung nicht erfuellbar, auch bei korrekten Daten.
        with self.subTest(pointer="updated_at"):
            self.public.execute("SAVEPOINT mutation")
            self.public.execute("UPDATE current_prices SET updated_at='2020-01-01'")
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
        self.public.execute("INSERT INTO current_prices VALUES(1,1,?)", (SINCE,))
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
        self.internal.execute("UPDATE ingestion_steps SET started_at='2026-10-18T19:00:00Z',finished_at='2026-10-18T19:01:00Z'")
        self.internal.commit()
        self.assertEqual(self.verify()[0], 1)


if __name__ == "__main__":
    unittest.main()
