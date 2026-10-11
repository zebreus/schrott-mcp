#!/usr/bin/env python3
"""Read-only diagnostic inventory, not an ingestion parser or mapping verifier.

Read a saved operator HTML snapshot and optionally compare the *count* of live
handler quotes. A matching count alone cannot approve material mappings.
With --candidate-json, compare every article contract against the independent
HTML inventory and material/variant against a fixed, separately reviewed oracle.
Not browser visibility, new-source semantic validation or production approval.
"""

import argparse
import hashlib
import html
import json
import re
from pathlib import Path

DEFAULT_MAPPING_ORACLE = (Path(__file__).resolve().parent.parent /
    "crates/ingestion/src/traders/handlers/fixtures/mkm-reviewed-mapping-oracle-20261011.json")


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
    parser.add_argument("--candidate-json", type=Path)
    parser.add_argument("--mapping-oracle", type=Path, default=DEFAULT_MAPPING_ORACLE)
    parser.add_argument("--detail-dir", type=Path,
                        help="saved mkm-detail-<ARTICLE>.html operator responses")
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
    if args.candidate_json:
        candidate = json.loads(args.candidate_json.read_text())
        oracle = json.loads(args.mapping_oracle.read_text())
        report["candidate_article_contract_gaps"] = compare_candidate(rows, candidate, oracle)
        report["mapping_semantics_scope"] = oracle["scope"]
        report["mapping_oracle"] = str(args.mapping_oracle)
        report["visibility_scope"] = "HTML inventory only; no browser/CSS visibility or register verification. Separate real-parser mutation tests required."
        report["production_approval"] = False
    if args.detail_dir:
        report["detail_pages"], report["detail_contract_gaps"] = compare_details(rows, args.detail_dir)
    print(json.dumps(report, ensure_ascii=False, indent=2))
    return 1 if (report.get("count_gap", 0) != 0
                 or report.get("candidate_article_contract_gaps")
                 or report.get("detail_contract_gaps")) else 0


def compare_candidate(rows, candidate, oracle=None):
    """Contract + fixed reviewed mapping comparison; never production approval."""
    if oracle is None:
        oracle = json.loads(DEFAULT_MAPPING_ORACLE.read_text())
    expected_mappings = oracle["expected"]
    source = {r["article"]: r for r in rows}
    actual = {}
    gaps = list(candidate.get("diagnostics", []))
    for row in candidate["articles"]:
        article = row["article"]
        if article in actual:
            gaps.append(f"{article}: duplicate candidate article")
        actual[article] = row
    for article in sorted(source.keys() - actual.keys()):
        gaps.append(f"{article}: missing candidate article")
    for article in sorted(actual.keys() - source.keys()):
        gaps.append(f"{article}: phantom candidate article")
    for article in sorted(source.keys() & actual.keys()):
        raw, parsed = source[article], actual[article]
        expected = {
            "group": raw["group"], "label": raw["title"], "url": raw["url"],
            "price_cell": raw["price_cell"],
            "price_eur_per_kg": raw["price_eur_per_kg"],
            "product_change_date": raw["product_change_date"],
            "tiers": raw["embedded_tiers"] or [],
            "request_only": raw["quote_kind"] == "request",
        }
        quantity = re.search(r"\bab ([\d.,]+) kg$", raw["price_cell"])
        expected["minimum_for_upper_bound_kg"] = (
            float(quantity[1].replace(".", "").replace(",", ".")) if quantity else None
        )
        for key, value in expected.items():
            if parsed.get(key) != value:
                gaps.append(f"{article}: {key} differs: {parsed.get(key)!r} != {value!r}")
        if not parsed.get("material"):
            gaps.append(f"{article}: explicit material mapping missing")
        mapping = expected_mappings.get(article)
        if mapping is None:
            gaps.append(f"{article}: mapping semantics NOT CHECKED: no reviewed oracle entry")
        else:
            for key, value in zip(["material", "variant"], mapping):
                if parsed.get(key) != value:
                    gaps.append(f"{article}: reviewed mapping {key} differs: {parsed.get(key)!r} != {value!r}")
    return gaps


def compare_details(rows, directory):
    """Check saved primary detail identities and visible per-product tiers."""
    pages, gaps = [], []
    for row in rows:
        article = row["article"]
        path = directory / f"mkm-detail-{article}.html"
        if not path.exists():
            gaps.append(f"{article}: detail response missing")
            continue
        raw = path.read_bytes()
        document = raw.decode("utf-8")
        identity = re.search(r"\bArtikelnummer\s+(\S+)", text(document))
        actual_article = identity[1] if identity else None
        if actual_article != article:
            gaps.append(f"{article}: detail identity differs: {actual_article}")
        tiers = []
        for table in re.findall(r"<table\b[^>]*>(.*?)</table>", document, re.S):
            if "Preis pro kg" not in text(table):
                continue
            for tr in re.findall(r"<tr\b[^>]*>(.*?)</tr>", table, re.S):
                cells = re.findall(r"<td\b[^>]*>(.*?)</td>", tr, re.S)
                if len(cells) != 2:
                    continue
                quantity = re.fullmatch(r"(bis|ab) ([\d.]+) kg", text(cells[0]))
                price = re.fullmatch(r"([\d.,]+) €", text(cells[1]))
                if not quantity or not price:
                    gaps.append(f"{article}: changed detail tier row")
                    continue
                tiers.append({
                    "ab": 0 if quantity[1] == "bis" else int(quantity[2].replace(".", "")),
                    "preis": float(price[1].replace(".", "").replace(",", ".")),
                })
        if tiers != (row["embedded_tiers"] or []):
            gaps.append(f"{article}: visible detail tiers disagree with overview")
        pages.append({"article": article, "url": row["url"],
                      "sha256": hashlib.sha256(raw).hexdigest(),
                      "detail_article": actual_article, "visible_tiers": tiers})
    return pages, gaps


if __name__ == "__main__":
    raise SystemExit(main())
