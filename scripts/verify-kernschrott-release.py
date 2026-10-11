#!/usr/bin/env python3
"""Bounded read-only rollout gate; not a source oracle or scheduler-trigger proof.

Pair with reviewed source inventory and deployment journal. No ingestion runs.
The two databases are independently sampled. Fixed counts require source review
if the price list changes. Conditions are read from base rows, not the notes-free
Current view. This does not establish immutable history for this legacy handler.
"""
import argparse
import json
import math
import sqlite3
from pathlib import Path

SLUG = "by-regensburg-kernschrott-recycling"
URL = "https://www.kernschrott-recycling.de/elektro"
RESERVATION = "Wir kaufen alle hier angegebenen Materialien zum Tageshöchstpreis. Die Preise sind unverbindlich – bei Verkauf bitten wir um kurze Rücksprache."
EXCLUSION = "Kein Ankauf von Materialien, die z. B. Kondensatoren ab 25 mm Bauhöhe, PCB oder andere gefährliche Stoffe enthalten. Bei Unsicherheit sprechen Sie uns vorab an."
REQUESTS = {"kabel-ohne-stecker", "kabel-mit-stecker", "alu-cu-kuehler"}
MATERIALS = REQUESTS | {
    "platinen", "integrierte-schaltungen", "handys", "prozessoren", "ram",
    "elektronikstecker", "notebooks", "festplatten", "netzteile", "computer",
    "laufwerke", "optische-datentraeger", "ablenkeinheiten", "panzersicherungen",
    "elektrobauteile", "luefter",
}


def connect(path):
    db = sqlite3.connect(path.resolve().as_uri() + "?mode=ro", uri=True)
    db.row_factory = sqlite3.Row
    db.execute("PRAGMA query_only=ON")
    return db


def verify(public, internal, since):
    if public.execute("SELECT julianday(?)", (since,)).fetchone()[0] is None:
        raise ValueError("invalid rollout timestamp")
    failures = []

    def fresh(timestamp):
        return public.execute("SELECT julianday(?)>=julianday(?)", (timestamp, since)).fetchone()[0] == 1

    step = internal.execute(
        "SELECT * FROM ingestion_steps WHERE scraper=? AND julianday(started_at)>=julianday(?) ORDER BY id DESC LIMIT 1",
        (SLUG, since),
    ).fetchone()
    if step is None:
        failures.append("missing post-rollout step")
    else:
        message = step["message"] or ""
        if step["status"] != "ok" or step["items_upserted"] != 38 or not fresh(step["finished_at"]):
            failures.append("step status/count/completion mismatch")
        if "CANARY" in message or "1 übersprungen" not in message or "Leiterplatten Kl. 3" not in message:
            failures.append("missing explicit Class-3 quarantine or canary")
        fetch = internal.execute(
            "SELECT * FROM raw_fetches WHERE run_id=? AND scraper=? AND url=? ORDER BY id DESC LIMIT 1",
            (step["run_id"], SLUG, URL),
        ).fetchone()
        if fetch is None or fetch["status_code"] != 200 or fetch["byte_len"] <= 0:
            failures.append("missing successful canonical source fetch")
    rows = public.execute(
        "SELECT p.*,m.slug AS material,m.unit AS catalog_unit FROM traders t "
        "JOIN current_prices c ON c.trader_id=t.id JOIN prices p ON p.id=c.price_id "
        "JOIN materials m ON m.id=p.material_id WHERE t.slug=?", (SLUG,),
    ).fetchall()
    if len(rows) != 38:
        failures.append(f"Current count {len(rows)} != 38")
    fans = []
    for row in rows:
        if row["material"] in REQUESTS or row["variant"] == "Leiterplatten Kl. 3":
            failures.append("request/quarantined grade has Current quote")
        if row["material"] == "luefter":
            fans.append(row)
        if (row["price"] is None or not math.isfinite(row["price"]) or row["price"] < 0
                or (row["price"] == 0 and (row["material"], row["variant"]) != ("luefter", "Lüfter"))
                or row["price_kind"] != "exact" or row["confidence"] != 1.0
                or row["currency"] != "EUR" or row["unit"] != "EUR/kg" or row["catalog_unit"] != "EUR/kg"):
            failures.append("invalid quote semantics/unit/zero grade")
        if row["source_url"] != URL or row["source_type"] != "haendler_angabe" or row["published"] != 1 or not fresh(row["observed_at"]):
            failures.append("source classification or freshness mismatch")
        try:
            extra = json.loads(row["extra_json"])
            dated = isinstance(extra, dict) and extra.get("published_at_basis") == "page_stated"
        except (TypeError, ValueError):
            dated = False
        if not dated or public.execute("SELECT julianday(?)=julianday('2026-10-06')", (row["published_at"],)).fetchone()[0] != 1:
            failures.append("reviewed page date missing or changed")
        if not all(text in (row["notes"] or "") for text in (RESERVATION, EXCLUSION)):
            failures.append("quote conditions missing")
    if len(fans) != 1 or fans[0]["variant"] != "Lüfter" or fans[0]["price"] != 0:
        failures.append("reviewed explicit fan zero-price missing or changed")
    acceptances = public.execute(
        "SELECT tm.*,m.slug AS material FROM traders t JOIN trader_materials tm ON tm.trader_id=t.id "
        "JOIN materials m ON m.id=tm.material_id WHERE t.slug=?", (SLUG,),
    ).fetchall()
    # Legacy record() also writes fallback acceptances; don't claim all rows are
    # the 19 explicit aggregates. Identify and verify only those aggregates.
    aggregates = [row for row in acceptances if (row["conditions"] or "").startswith("Nur die aufgeführten Varianten mit ihren jeweiligen Bedingungen:")]
    if len(aggregates) != 19:
        failures.append(f"aggregate count {len(aggregates)} != 19")
    if {row["material"] for row in aggregates} != MATERIALS:
        failures.append("reviewed aggregate material partition mismatch")
    for row in aggregates:
        if row["accepts"] != 1 or not fresh(row["observed_at"]) or not all(text in row["conditions"] for text in (RESERVATION, EXCLUSION)):
            failures.append("aggregate conditions/acceptance/freshness mismatch")
    request_materials = {row["material"] for row in aggregates if "Preis auf Anfrage" in row["conditions"]}
    if request_materials != REQUESTS:
        failures.append("request acceptance partition mismatch")
    boards = [row for row in aggregates if row["material"] == "platinen"]
    if len(boards) != 1 or "Leiterplatten Kl. 3 quarantänisiert, keine bestätigte Annahme dieser Variante" not in boards[0]["conditions"]:
        failures.append("Class-3 acceptance exclusion missing")
    return {"ok": not failures, "current_count": len(rows), "aggregate_count": len(aggregates), "failures": sorted(set(failures))}


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
