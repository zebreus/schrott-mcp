#!/usr/bin/env python3
"""Read-only gate for MSG tiers, northwest seed corrections and Gotha address.

Run AFTER coordinated rollout and regular scheduler phases. This is not proof
of trigger type, all handler coverage, all dossier facts, or source-to-material
mapping. Pair with reviewed source inventory and deployment journal. No ingestion
runs. The databases are sampled independently, not a cross-database snapshot.
"""
import argparse
import json
import math
import re
import sqlite3
from pathlib import Path

MSG = "th-gotha-msg-metallrecycling-msg-schrott-und-meta"
MSG_URL = "https://www.msg-metallrecycling.de/preisliste"
GOTHA = "th-gotha-sdm-gutzmann"
GOTHA_URL = "https://www.sdm-gutzmann.de/preislisten/"
CLEARS = {
    "hh-iserbrook-22589-hamburger-schrotthandel-pruffall-hamburg": ("phone", "website"),
    "ni-buchholz-i-d-n-21244-all-in-schrott-abholung": ("phone",),
    "ni-osnabruck-reinhard-peters-schrottplatz": ("phone",),
    "ni-region-hannover-hildesheim-schrotthandel-f-weber-ort-n-a": ("phone",),
    "nw-leichlingen-muller": ("phone",),
}
PORTAL = "nw-nrw-weit-schrott24-metaloop-online-portal"


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

    for slug, fields in CLEARS.items():
        row = public.execute("SELECT * FROM traders WHERE slug=?", (slug,)).fetchone()
        if row is None:
            failures.append(f"{slug}: missing trader")
        else:
            if any(row[field] != "" for field in fields):
                failures.append(f"{slug}: clear values not cleared")
            if not fresh(row["updated_at"]):
                failures.append(f"{slug}: post-rollout seed evidence missing")
    portal = public.execute("SELECT * FROM traders WHERE slug=?", (PORTAL,)).fetchone()
    if portal is None or not fresh(portal["updated_at"]):
        failures.append("Schrott24: missing post-rollout trader")
    if portal is not None:
        for field in ("dropoff_json", "pickup_json"):
            try:
                empty = json.loads(portal[field]) == {}
            except (TypeError, ValueError):
                empty = False
            if not empty:
                failures.append(f"Schrott24: {field} is not an explicit empty object")
    gotha = public.execute("SELECT * FROM traders WHERE slug=?", (GOTHA,)).fetchone()
    if gotha is None:
        failures.append("Gutzmann: missing trader")
    else:
        if (gotha["street"], gotha["postcode"], gotha["city"]) != ("An der Ostbahn 4", "99867", "Gotha"):
            failures.append("Gutzmann: branch address incorrect")
        if not fresh(gotha["updated_at"]):
            failures.append("Gutzmann: post-rollout seed evidence missing")

    for slug, count in ((GOTHA, 19), (MSG, 124)):
        step = internal.execute(
            "SELECT * FROM ingestion_steps WHERE scraper=? AND julianday(started_at)>=julianday(?) ORDER BY id DESC LIMIT 1",
            (slug, since),
        ).fetchone()
        if step is None or step["status"] != "ok" or step["items_upserted"] != count or "CANARY" in (step["message"] or "") or not fresh(step["finished_at"]):
            failures.append(f"{slug}: successful post-rollout step missing or changed contract")
        if step is not None:
            fetches = internal.execute(
                "SELECT * FROM raw_fetches WHERE run_id=? AND scraper=?",
                (step["run_id"], slug),
            ).fetchall()
            if not any(fetch["status_code"] == 200 and fetch["byte_len"] > 0
                       and fetch["url"] == (MSG_URL if slug == MSG else GOTHA_URL) for fetch in fetches):
                failures.append(f"{slug}: successful source fetch missing")
    rows = public.execute(
        "SELECT p.*,m.slug AS material,m.unit AS catalog_unit FROM traders t "
        "JOIN current_prices c ON c.trader_id=t.id JOIN prices p ON p.id=c.price_id "
        "JOIN materials m ON m.id=p.material_id WHERE t.slug=? AND p.source_url=?",
        (MSG, MSG_URL),
    ).fetchall()
    managed = 0
    for row in rows:
        try:
            extra = json.loads(row["extra_json"])
            if not isinstance(extra, dict):
                raise ValueError("metadata must be object")
        except (TypeError, ValueError):
            failures.append("MSG: invalid source metadata")
            continue
        if extra.get("snapshot_owner") != "msg-visible-tiers-v1":
            # Normal producer leaves unproven same-source keys intact; this
            # rollout gate deliberately rejects unexpected leftovers for review.
            failures.append("MSG: unmanaged same-source Current remains")
            continue
        managed += 1
        title, quantity, unit = (extra.get(k) for k in ("source_title", "quantity_min_text", "quantity_unit"))
        numeric_quantity = extra.get("quantity_min")
        quantity_valid = isinstance(quantity, str) and re.fullmatch(r"(?:0|[1-9][0-9]*)(?:\.[0-9]*[1-9])?", quantity)
        numeric_valid = (isinstance(numeric_quantity, (int, float)) and not isinstance(numeric_quantity, bool)
                         and math.isfinite(numeric_quantity) and numeric_quantity >= 0)
        if not isinstance(title, str) or not title.strip() or not quantity_valid or not numeric_valid or (quantity_valid and numeric_valid and float(quantity) != numeric_quantity) or unit not in ("kg", "t") or row["variant"] != f"{title} / ab {quantity} {unit}":
            failures.append("MSG: grade/quantity identity mismatch")
        if row["price_kind"] != "exact" or row["confidence"] != 1.0 or row["price"] is None or not math.isfinite(row["price"]) or row["price"] <= 0 or row["unit"] != row["catalog_unit"] or row["currency"] != "EUR":
            failures.append("MSG: quote semantics or normalization mismatch")
        if not row["published"] or row["source_type"] != "haendler_angabe" or extra.get("source_price_unit") != "EUR/kg" or not fresh(row["observed_at"]):
            failures.append("MSG: source/freshness mismatch")
        if row["published_at"] is not None and extra.get("published_at_basis") != "observed_price_change":
            failures.append("MSG: unsupported publication date")
    if managed != 124:
        failures.append(f"MSG: {managed} managed quotes instead of reviewed 124")
    return {"ok": not failures, "msg_managed_count": managed, "failures": sorted(set(failures))}


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
