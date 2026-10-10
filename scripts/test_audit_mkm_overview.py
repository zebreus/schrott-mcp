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
