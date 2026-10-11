"""Offline checks for the diagnostic inventory, not handler mapping approval."""

import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("audit-mkm-overview.py")
SPEC = importlib.util.spec_from_file_location("inventory", SCRIPT)
AUDIT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AUDIT)

ROW = '''<tr><th><a href="/metallankauf/produkt/example">Example</a></th>
<td>SKU-1</td><td>11,13 € /kg ab 250 kg</td>
<td><time datetime="2026-09-28">old</time></td>
<td><input data-staffel="[{&quot;ab&quot;:0,&quot;preis&quot;:10.6},
{&quot;ab&quot;:250,&quot;preis&quot;:11.13}]"></td></tr>'''
TABLE = '<table><caption>Ankaufpreise für Kupfer</caption>' + ROW + '</table>'


class InventoryTests(unittest.TestCase):
    def test_preserved_pre_fix_artifact_exposes_tin_and_kerze_mappings(self):
        root = SCRIPT.parent.parent
        rows = AUDIT.inventory((root / "crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261010.html").read_text())
        old_candidate = json.loads((root / "research/mkm-full-live-inventory-20261010.json").read_text())
        gaps = AUDIT.compare_candidate(rows, old_candidate)
        self.assertEqual({gap.split(":", 1)[0] for gap in gaps}, {"SW10040", "MKM-CU-01"})
        self.assertTrue(all("reviewed mapping material differs" in gap for gap in gaps))

    def test_real_snapshot_all_battery_wrong_is_not_semantically_green(self):
        root = SCRIPT.parent.parent
        document = (root / "crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261010.html").read_text()
        candidate = json.loads((root / "research/mkm-full-live-inventory-20261011.json").read_text())
        rows = AUDIT.inventory(document)
        self.assertEqual(AUDIT.compare_candidate(rows, candidate), [], "corrected actual candidate must be baseline green")
        for row in candidate["articles"]:
            row["material"] = "batterien-blei"
            row["variant"] = "WRONG"
        gaps = AUDIT.compare_candidate(rows, candidate)
        self.assertEqual(len(gaps), 120, "each of 60 material AND variant mutations must be diagnosed")
        self.assertEqual({gap.split(":", 1)[0] for gap in gaps}, {row["article"] for row in rows})
        self.assertTrue(all("reviewed mapping" in gap for gap in gaps))

    def test_detail_identity_and_visible_tiers_are_independently_checked(self):
        source = AUDIT.inventory(TABLE)
        detail = "<p>Artikelnummer SKU-1</p><table><thead><tr><th>Preis pro kg</th></tr></thead>" + \
                 "<tbody><tr><td>bis 250 kg</td><td>10,60 €</td></tr>" + \
                 "<tr><td>ab 250 kg</td><td>11,13 €</td></tr></tbody></table>"
        with tempfile.TemporaryDirectory(dir="/tmp/opencode") as directory:
            path = Path(directory) / "mkm-detail-SKU-1.html"
            path.write_text(detail)
            pages, gaps = AUDIT.compare_details(source, Path(directory))
            self.assertEqual(gaps, [])
            self.assertEqual(pages[0]["detail_article"], "SKU-1")
            for changed in [detail.replace("SKU-1", "OTHER"), detail.replace("11,13", "99,00")]:
                path.write_text(changed)
                self.assertTrue(AUDIT.compare_details(source, Path(directory))[1])

    def test_article_contract_not_count_can_detect_phantom_or_changed_candidate(self):
        source = AUDIT.inventory(TABLE)
        row = source[0]
        parsed = {"article": row["article"], "group": row["group"],
                  "label": row["title"], "url": row["url"],
                  "price_cell": row["price_cell"],
                  "price_eur_per_kg": row["price_eur_per_kg"],
                  "product_change_date": row["product_change_date"],
                  "tiers": row["embedded_tiers"], "request_only": False,
                  "minimum_for_upper_bound_kg": 250, "material": "kupfer-millberry", "variant": ""}
        candidate = {"articles": [parsed], "diagnostics": []}
        oracle = {"expected": {"SKU-1": ["kupfer-millberry", ""]}}
        self.assertEqual(AUDIT.compare_candidate(source, candidate, oracle), [])
        for field, changed in [("article", "PHANTOM"), ("label", "Wrong"),
                               ("price_eur_per_kg", 0), ("minimum_for_upper_bound_kg", 0),
                               ("tiers", []), ("product_change_date", "2026-10-10"),
                               ("material", None)]:
            with self.subTest(field=field):
                bad = {"articles": [dict(parsed, **{field: changed})], "diagnostics": []}
                self.assertTrue(AUDIT.compare_candidate(source, bad, oracle))
        self.assertTrue(AUDIT.compare_candidate(source, {"articles": [parsed, parsed]}, oracle))
        self.assertTrue(AUDIT.compare_candidate(source, dict(candidate, diagnostics=["missing known article"]), oracle))
        self.assertTrue(any("NOT CHECKED" in gap for gap in AUDIT.compare_candidate(source, candidate, {"expected": {}})))

    def test_price_conditions_dates_and_embedded_tiers_are_distinct(self):
        row = AUDIT.inventory(TABLE)[0]
        self.assertEqual(row["price_eur_per_kg"], 11.13)
        self.assertEqual(row["product_change_date"], "2026-09-28")
        self.assertEqual(row["embedded_tiers"][0], {"ab": 0, "preis": 10.6})
        self.assertIn("ab 250 kg", row["price_cell"])

    def test_request_never_becomes_zero_quote(self):
        row = AUDIT.inventory(TABLE.replace("11,13 € /kg ab 250 kg", "auf Anfrage"))[0]
        self.assertIsNone(row["price_eur_per_kg"])
        self.assertEqual(row["quote_kind"], "request")

    def test_duplicate_empty_or_ambiguous_inputs_fail(self):
        for document in [TABLE + TABLE, "<p>no table</p>",
                         TABLE.replace("11,13 € /kg", "11,13 € /kg auf Anfrage")]:
            with self.subTest(document=document), self.assertRaises(ValueError):
                AUDIT.inventory(document)

    def test_sales_table_not_counted(self):
        self.assertEqual(len(AUDIT.inventory(
            TABLE + TABLE.replace("Ankaufpreise für Kupfer", "Verkaufspreise"))), 1)

    def test_cli_gap_is_red_and_equal_count_not_mapping_verification(self):
        with tempfile.TemporaryDirectory(dir="/tmp/opencode") as directory:
            snapshot = Path(directory) / "source.html"
            output = Path(directory) / "handler.txt"
            snapshot.write_text(TABLE)
            for quotes, status in [("", 1), ("price conf=Some(0.5)\n", 0)]:
                output.write_text(quotes)
                result = subprocess.run(
                    [sys.executable, str(SCRIPT), str(snapshot),
                     "--handler-output", str(output)], capture_output=True, text=True)
                self.assertEqual(result.returncode, status, result.stderr)
                self.assertEqual(json.loads(result.stdout)["count_gap"], 1 - bool(quotes))


if __name__ == "__main__":
    unittest.main()
