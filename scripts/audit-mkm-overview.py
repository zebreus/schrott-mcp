#!/usr/bin/env python3
"""Read-only diagnostic inventory, not an ingestion parser or mapping verifier.

Read a saved operator HTML snapshot and optionally compare the *count* of live
handler quotes. A matching count alone cannot approve material mappings.
"""

import argparse
import hashlib
import html
import json
import re
from pathlib import Path


def text(fragment):
    return " ".join(html.unescape(re.sub(r"<[^>]*>", " ", fragment)).split())


def inventory(document):
    rows = []
    for table in re.findall(r"<table\b[^>]*>(.*?)</table>", document, re.S):
        caption = re.search(r"<caption\b[^>]*>(.*?)</caption>", table, re.S)
        if not caption or not text(caption[1]).startswith("Ankaufpreise für "):
            continue
        group = text(caption[1])
        for row in re.findall(r"<tr\b[^>]*>(.*?)</tr>", table, re.S):
            product = re.search(
                r'<a\b[^>]*href="(/metallankauf/produkt/[^\"]+)"[^>]*>(.*?)</a>',
                row,
                re.S,
            )
            if not product:
                continue
            cells = re.findall(r"<td\b[^>]*>(.*?)</td>", row, re.S)
            if len(cells) != 4:
                raise ValueError(f"Changed product row layout: {text(product[2])}")
            price_text = text(cells[1])
            quote = re.search(r"([\d.,]+)\s*€\s*/kg\b", price_text)
            request = "auf Anfrage" in price_text
            if bool(quote) == request:
                raise ValueError(f"Ambiguous quote/request: {price_text}")
            tiers = re.search(r'\bdata-staffel="([^\"]*)"', cells[3])
            date = re.search(r'<time\b[^>]*datetime="([^\"]+)"', cells[2])
            rows.append(
                {
                    "group": group,
                    "article": text(cells[0]),
                    "title": text(product[2]),
                    "url": "https://mkm-metals.de" + html.unescape(product[1]),
                    "quote_kind": "request" if request else "upper_bound",
                    "price_eur_per_kg": (
                        float(quote[1].replace(".", "").replace(",", "."))
                        if quote
                        else None
                    ),
                    "price_cell": price_text,
                    "product_change_date": date[1] if date else None,
                    "embedded_tiers": json.loads(html.unescape(tiers[1])) if tiers else None,
                }
            )
    if not rows or len({r["article"] for r in rows}) != len(rows):
        raise ValueError("Missing products or duplicate article identities")
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("snapshot", type=Path)
    parser.add_argument("--handler-output", type=Path)
    args = parser.parse_args()
    raw = args.snapshot.read_bytes()
    rows = inventory(raw.decode("utf-8"))
    numeric = sum(r["quote_kind"] == "upper_bound" for r in rows)
    report = {
        "snapshot_sha256": hashlib.sha256(raw).hexdigest(),
        "groups": len({r["group"] for r in rows}),
        "products": len(rows),
        "numeric": numeric,
        "request_only": len(rows) - numeric,
        "rows": rows,
    }
    if args.handler_output:
        count = sum("conf=" in line for line in args.handler_output.read_text().splitlines())
        report["handler_numeric_count"] = count
        report["count_gap"] = numeric - count
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 1 if report.get("count_gap", 0) != 0 else 0


if __name__ == "__main__":
    raise SystemExit(main())
