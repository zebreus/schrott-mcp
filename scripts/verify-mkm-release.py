#!/usr/bin/env python3
"""Read-only persisted MKM rollout gate, not a fresh source or trigger oracle.

Pair with source audit/deployment journal. Fixed 56/5 needs review on source drift.
Databases are independently sampled. No ingestion or production DB mutations.
"""
import argparse
import json
import math
import sqlite3
from pathlib import Path

SLUG = "rp-ludwigshafen-mkm-metals"
URL = "https://mkm-metals.de/metallpreise"
OWNER = "mkm-article-v1"
ORACLE = Path(__file__).resolve().parent.parent / "crates/ingestion/src/traders/handlers/fixtures/mkm-reviewed-mapping-oracle-20261018.json"


def connect(path):
    db = sqlite3.connect(path.resolve().as_uri() + "?mode=ro", uri=True)
    db.row_factory = sqlite3.Row
    db.execute("PRAGMA query_only=ON")
    return db


def object_json(raw):
    value = json.loads(raw)
    if not isinstance(value, dict):
        raise ValueError("expected JSON object")
    return value


def verify(public, internal, since):
    if public.execute("SELECT julianday(?)", (since,)).fetchone()[0] is None:
        raise ValueError("invalid rollout timestamp")
    failures = []

    def fresh(at):
        return public.execute("SELECT julianday(?)>=julianday(?)", (at, since)).fetchone()[0] == 1

    tables = {r[0] for r in public.execute("SELECT name FROM sqlite_master WHERE type='table'")}
    if not {"source_article_snapshots", "article_acceptance_owners"} <= tables:
        return {"ok": False, "failures": ["typed MKM schema missing; deployment not proven"]}
    step = internal.execute("SELECT * FROM ingestion_steps WHERE scraper=? AND julianday(started_at)>=julianday(?) ORDER BY id DESC LIMIT 1", (SLUG, since)).fetchone()
    if step is None or step["status"] != "ok" or step["items_upserted"] != 56 or not fresh(step["finished_at"]) or "CANARY" in (step["message"] or ""):
        failures.append("successful completed post-rollout 56-price step missing")
    if step is not None:
        fetch = internal.execute("SELECT * FROM raw_fetches WHERE run_id=? AND scraper=? AND url=? ORDER BY id DESC LIMIT 1", (step["run_id"], SLUG, URL)).fetchone()
        if fetch is None or fetch["status_code"] != 200 or fetch["byte_len"] <= 0:
            failures.append("canonical HTTP-200 source fetch missing")
    ledger = public.execute("SELECT s.* FROM source_article_snapshots s JOIN traders t ON t.id=s.trader_id WHERE t.slug=? AND s.source_url=? AND s.owner=? ORDER BY s.observed_at DESC LIMIT 1", (SLUG, URL, OWNER)).fetchone()
    if ledger is None:
        return {"ok": False, "failures": sorted(set(failures + ["complete article ledger missing"]))}
    try:
        manifest = object_json(ledger["manifest_json"])
        articles = manifest["articles"]
        if not isinstance(articles, list) or any(not isinstance(a, dict) for a in articles):
            raise ValueError("invalid articles")
        by_id = {a["article"]: a for a in articles}
    except (TypeError, ValueError, KeyError):
        return {"ok": False, "failures": ["invalid article manifest"]}
    if (not fresh(ledger["observed_at"]) or manifest.get("observed_at") != ledger["observed_at"]
            or manifest.get("trader_id") != ledger["trader_id"] or manifest.get("owner") != OWNER
            or manifest.get("source_url") != URL):
        failures.append("manifest scope/freshness mismatch")
    if step is not None and public.execute("SELECT julianday(?) BETWEEN julianday(?) AND julianday(?)", (ledger["observed_at"], step["started_at"], step["finished_at"])).fetchone()[0] != 1:
        failures.append("manifest is not from latest completed step window")
    expected = json.loads(ORACLE.read_text())["expected"]
    if len(articles) != 61 or len(by_id) != 61 or set(by_id) != set(expected):
        failures.append("reviewed 61-article partition mismatch")
    prices = public.execute("SELECT p.*,m.slug AS material,m.unit AS catalog_unit FROM current_prices c JOIN prices p ON p.id=c.price_id JOIN materials m ON m.id=p.material_id WHERE c.trader_id=? AND p.source_url=?", (ledger["trader_id"], URL)).fetchall()
    indexed = {(p["material"], p["variant"]): p for p in prices}
    wanted_keys = set()
    grouped = {}
    numeric = requests = 0
    for ident, article in by_id.items():
        mapping = expected.get(ident)
        if mapping is None or (article.get("material"), article.get("variant")) != tuple(mapping):
            failures.append("article mapping differs from reviewed oracle")
            continue
        material, variant = mapping
        semantic = dict(article)
        semantic.pop("product_change_text", None)
        grouped.setdefault(material, []).append(semantic)
        is_request = article.get("request_only")
        if is_request is True:
            requests += 1
            if article.get("price_eur_per_kg") is not None or article.get("tiers") != [] or article.get("minimum_for_upper_bound_kg") is not None or article.get("price_cell") != "auf Anfrage":
                failures.append("request article contains numeric contract")
            if (material, variant) in indexed:
                failures.append("request article still has Current quote")
            continue
        if is_request is not False:
            failures.append("invalid request/numeric partition")
            continue
        numeric += 1
        wanted_keys.add((material, variant))
        row = indexed.get((material, variant))
        if row is None:
            failures.append("numeric article Current missing")
            continue
        try:
            extra = object_json(row["extra_json"])
            source_price = article["price_eur_per_kg"]
            minimum = article["minimum_for_upper_bound_kg"]
            tiers = article["tiers"]
            valid_source = (isinstance(source_price, (int, float)) and not isinstance(source_price, bool)
                            and math.isfinite(source_price) and source_price > 0
                            and isinstance(minimum, (int, float)) and not isinstance(minimum, bool)
                            and math.isfinite(minimum) and minimum >= 0 and isinstance(tiers, list) and bool(tiers)
                            and tiers[0]["ab"] == 0 and tiers[-1]["ab"] == minimum
                            and tiers[-1]["preis"] == source_price)
            if valid_source:
                valid_source = all(
                    isinstance(t, dict) and set(t) == {"ab", "preis"}
                    and isinstance(t["ab"], (int, float)) and not isinstance(t["ab"], bool)
                    and math.isfinite(t["ab"]) and t["ab"] >= 0
                    and isinstance(t["preis"], (int, float)) and not isinstance(t["preis"], bool)
                    and math.isfinite(t["preis"]) and t["preis"] > 0 for t in tiers)
            if valid_source:
                valid_source = all(a["ab"] < b["ab"] and a["preis"] <= b["preis"] for a, b in zip(tiers, tiers[1:]))
            if not valid_source:
                raise ValueError("bad source terms")
            unit = row["catalog_unit"]
            if unit not in ("EUR/kg", "EUR/t"):
                raise ValueError("unproven catalog unit")
            amount = source_price * (1000 if unit == "EUR/t" else 1)
            if (not math.isfinite(amount) or row["price"] != amount or row["price_max"] != amount
                    or row["price_min"] is not None or row["unit"] != unit
                    or row["currency"] != "EUR" or row["price_kind"] != "upto" or row["confidence"] != 0.5):
                failures.append("normalized quote contract mismatch")
            if extra.get("snapshot_owner") != OWNER or extra.get("article") != semantic or extra.get("page_change_date") != manifest.get("page_change_date"):
                failures.append("Current article metadata differs from latest manifest")
        except (TypeError, ValueError, KeyError, IndexError):
            failures.append("invalid numeric metadata/tier contract")
        if row["published_at"] is not None or row["valid_from"] is not None or row["valid_to"] is not None or row["source_type"] != "haendler_angabe" or row["published"] != 1 or not fresh(row["observed_at"]) or public.execute("SELECT julianday(?)<=julianday(?)", (row["observed_at"], ledger["observed_at"])).fetchone()[0] != 1:
            failures.append("quote provenance/date/freshness mismatch")
    if (numeric, requests) != (56, 5):
        failures.append(f"article quote/request partition {numeric}/{requests} != 56/5")
    if set(indexed) != wanted_keys or len(prices) != numeric:
        failures.append("unexpected same-source Current keys; legacy alias or unproven leftovers require review")
    acceptances = public.execute("SELECT tm.*,m.slug AS material,o.source_url AS owner_source,o.owner,o.conditions AS owner_conditions,o.observed_at AS owner_at FROM trader_materials tm JOIN materials m ON m.id=tm.material_id LEFT JOIN article_acceptance_owners o ON o.trader_id=tm.trader_id AND o.material_id=tm.material_id WHERE tm.trader_id=?", (ledger["trader_id"],)).fetchall()
    acc_by_material = {a["material"]: a for a in acceptances}
    for material, members in grouped.items():
        row = acc_by_material.get(material)
        if row is None:
            failures.append("article-aware material acceptance missing")
            continue
        try:
            conditions = json.loads(row["conditions"])
        except (TypeError, ValueError):
            conditions = None
        if (conditions != sorted(members, key=lambda a: a["article"]) or row["accepts"] != 1
                or row["valid_from"] is not None or row["valid_to"] is not None
                or row["owner"] != OWNER or row["owner_source"] != URL
                or row["owner_conditions"] != row["conditions"]
                or row["observed_at"] != ledger["observed_at"] or row["owner_at"] != ledger["observed_at"]):
            failures.append("acceptance contract/ownership differs from latest manifest")
    return {"ok": not failures, "articles": len(articles), "numeric": numeric, "requests": requests,
            "current_count": len(prices), "failures": sorted(set(failures))}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--since", required=True)
    parser.add_argument("--data-dir", type=Path, default=Path("/var/lib/schrott-mcp"))
    args = parser.parse_args()
    with connect(args.data_dir / "public.db") as public, connect(args.data_dir / "internal.db") as internal:
        result = verify(public, internal, args.since)
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
