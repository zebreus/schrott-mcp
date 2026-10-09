#!/usr/bin/env python3
"""Read-only post-rollout evidence; run after regular scheduler phases.

Usage: python3 scripts/verify-handler-release.py --since <rollout RFC3339>
This proves recorded steps and prices, not the trigger type: ingestion_runs
does not store it. Pair this with the rollout journal/no-force-run evidence.
"""
import argparse
import json
import sqlite3
from pathlib import Path


HANDLERS = {
    "be-spandau-elno-container-und-dienstleistungs": (36, "exact", False, 3),
    "by-kulmbach-trapper": (21, "approx", True, 0),
    "he-erlensee-agh-altgoldhandel": (8, "exact", True, 0),
    "sn-halsbrucke-09633-saxonia-edelmetalle": (4, "exact", True, 6),
    "bw-rastatt-hofmann": (23, "exact", True, 0),
    "be-spandau-regold-edelmetallhandel": (20, "approx", False, 0),
}


def connect(path):
    db = sqlite3.connect(path.resolve().as_uri() + "?mode=ro")
    db.row_factory = sqlite3.Row
    return db


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--since", required=True)
    parser.add_argument("--data-dir", type=Path, default=Path("/var/lib/schrott-mcp"))
    args = parser.parse_args()
    public = connect(args.data_dir / "public.db")
    internal = connect(args.data_dir / "internal.db")
    if public.execute("SELECT julianday(?)", (args.since,)).fetchone()[0] is None:
        parser.error("--since must be a valid rollout timestamp")
    results = []
    for slug, (count, kind, dated, skips) in HANDLERS.items():
        failures = []
        step = internal.execute(
            "SELECT * FROM ingestion_steps WHERE scraper=? "
            "AND julianday(started_at)>=julianday(?) ORDER BY id DESC LIMIT 1",
            (slug, args.since),
        ).fetchone()
        if step is None:
            results.append({"slug": slug, "ok": False, "failures": ["no post-rollout step"]})
            continue
        if step["status"] != "ok" or step["items_upserted"] != count:
            failures.append("step status/count mismatch")
        if "CANARY" in step["message"]:
            failures.append("step contains canary")
        if skips and f"{skips} übersprungen" not in step["message"]:
            failures.append("expected explicit exclusions missing")
        if not skips and "übersprungen" in step["message"]:
            failures.append("unexpected skips")
        fetch = internal.execute(
            "SELECT * FROM raw_fetches WHERE run_id=? AND scraper=? ORDER BY id DESC LIMIT 1",
            (step["run_id"], slug),
        ).fetchone()
        if fetch is None or fetch["status_code"] != 200 or fetch["byte_len"] <= 0:
            failures.append("missing successful source fetch")
        rows = public.execute(
            "SELECT p.*, m.slug AS material, m.unit AS catalog_unit FROM traders t "
            "JOIN current_prices c ON c.trader_id=t.id JOIN prices p ON p.id=c.price_id "
            "JOIN materials m ON m.id=p.material_id WHERE t.slug=?",
            (slug,),
        ).fetchall()
        if len(rows) != count:
            failures.append(f"current price count {len(rows)} != {count}")
        for row in rows:
            if row["price_kind"] != kind or row["confidence"] != (0.8 if kind == "approx" else 1.0):
                failures.append("quote uncertainty mismatch")
            if row["unit"] != row["catalog_unit"] or row["currency"] != "EUR" or row["price"] <= 0:
                failures.append("invalid normalized quote")
            if row["source_type"] != "haendler_angabe" or not row["published"]:
                failures.append("source classification mismatch")
            if fetch and row["source_url"] != fetch["url"]:
                failures.append("price/fetch provenance mismatch")
            if dated:
                if not row["published_at"] or json.loads(row["extra_json"]).get("published_at_basis") != "page_stated":
                    failures.append("page-stated publication date missing")
            elif row["published_at"] is not None:
                # Later comparable price changes may legitimately infer a date.
                if json.loads(row["extra_json"]).get("published_at_basis") != "observed_price_change":
                    failures.append("undated source has unsupported publication date")
            if public.execute("SELECT julianday(?)>=julianday(?)", (row["observed_at"], args.since)).fetchone()[0] != 1:
                failures.append("stale pre-rollout observation")
        results.append({"slug": slug, "ok": not failures, "step": dict(step),
                        "fetch": dict(fetch) if fetch else None, "current_count": len(rows),
                        "failures": sorted(set(failures))})
    print(json.dumps(results, ensure_ascii=False, indent=2))
    return 0 if all(item["ok"] for item in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
