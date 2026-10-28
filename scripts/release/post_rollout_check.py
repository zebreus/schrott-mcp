#!/usr/bin/env python3
"""Read-only post-rollout verification for release f40d1dbb.

Runs the release-pinned gates and the target-row checks against the live
production database. Read-only: every connection is opened with mode=ro and
PRAGMA query_only=ON.
"""
import json
import os
import sqlite3
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]  # Repo-Wurzel
PROD = Path("/var/lib/schrott-mcp")

SHORT = os.environ["RELEASE_SHORT"]  # Pflicht: sonst greift ein fremder Pin
GEO_CLEARED_EXPECTED = json.loads(
    Path(os.environ.get("RELEASE_TARGET_SLUGS", "{}")).read_text() or "[]")
ESSEN_GEO_SLUGS = json.loads(
    Path(os.environ.get("RELEASE_GEO_SLUGS", "{}")).read_text() or "[]")


def connect(path):
    conn = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA query_only=ON")
    return conn


def main() -> int:
    result = {"checked_utc": datetime.now(timezone.utc).isoformat()}
    conn = connect(PROD / "public.db")
    literals = ",".join(f"'{s}'" for s in GEO_CLEARED_EXPECTED + ESSEN_GEO_SLUGS)
    rows = {r["slug"]: dict(r) for r in conn.execute(
        "SELECT slug, name, street, postcode, phone, website, website_status, status, "
        "lat, lon, dropoff_json, updated_at FROM traders WHERE slug IN (" + literals + ")")}
    counts = {
        t: conn.execute(f"SELECT COUNT(*) FROM {t}").fetchone()[0]
        for t in ("traders", "materials", "current_prices", "prices")
    }
    conn.close()
    result["counts"] = counts
    result["target_rows"] = {s: rows.get(s) for s in GEO_CLEARED_EXPECTED}
    result["essen_rows"] = {s: rows.get(s) for s in ESSEN_GEO_SLUGS}
    result["geo_cleared_ok"] = all(
        rows.get(s) and rows[s]["lat"] is None and rows[s]["lon"] is None
        for s in GEO_CLEARED_EXPECTED + ESSEN_GEO_SLUGS)
    result["essen_cluster_remaining_with_point"] = [
        s for s in ESSEN_GEO_SLUGS
        if rows.get(s) and rows[s]["lat"] is not None]
    def scoped(slug):
        return slug in GEO_CLEARED_EXPECTED + ESSEN_GEO_SLUGS

    result["wismar_ok"] = None if not scoped("mv-wismar-alba-metall-nord") else bool(
        rows.get("mv-wismar-alba-metall-nord")
        and rows["mv-wismar-alba-metall-nord"]["name"] == "ALBA Metall Nord GmbH"
        and rows["mv-wismar-alba-metall-nord"]["status"] == "pruefung"
        and rows["mv-wismar-alba-metall-nord"]["dropoff_json"] == "{}"
        and rows["mv-wismar-alba-metall-nord"]["street"] == ""
        and rows["mv-wismar-alba-metall-nord"]["postcode"] == ""
        and rows["mv-wismar-alba-metall-nord"]["phone"] == ""
        and rows["mv-wismar-alba-metall-nord"]["website"] == ""
        and rows["mv-wismar-alba-metall-nord"]["website_status"] == "unbekannt")
    result["db_recycling_ok"] = None if not scoped("sn-rochlitz-db-recycling-denis-blum") else bool(
        rows.get("sn-rochlitz-db-recycling-denis-blum")
        and rows["sn-rochlitz-db-recycling-denis-blum"]["dropoff_json"] == "{}"
        and rows["sn-rochlitz-db-recycling-denis-blum"]["lat"] is None
        and rows["sn-rochlitz-db-recycling-denis-blum"]["status"] == "aktiv"
        # the DB Recycling handler must not re-publish the single-source
        # Impressum address after a natural scrape run
        and rows["sn-rochlitz-db-recycling-denis-blum"]["street"] == ""
        and rows["sn-rochlitz-db-recycling-denis-blum"]["postcode"] == "")
    result["rilei_ok"] = None if not scoped("st-kalbe-rilei") else bool(
        rows.get("st-kalbe-rilei")
        and rows["st-kalbe-rilei"]["lat"] is None
        and rows["st-kalbe-rilei"]["status"] == "pruefung")

    # Das Pending-Korrektur-Gate wird **release-gepinnt** mitgeführt: ein
    # generischer Aufruf prueft den Seed-Stand eines alten Releases und ist
    # nach jedem Dossier-Wechsel rot. Ist fuer dieses Release keine
    # gepinnte Variante vorhanden, bleibt das generische Gate als Hinweis.
    pinned_pending = ROOT / f"scripts/verify-pending-corrections-{SHORT}.py"
    pending_cmd = ([sys.executable, str(pinned_pending), "--since", "2026-10-27T02:43:15Z"]
                   if pinned_pending.exists()
                   else [sys.executable, str(ROOT / "scripts/verify-pending-corrections.py"),
                         "--since", "2026-10-27T02:43:15Z"])
    gates = {}
    for label, cmd in (
        (f"wave_e_{SHORT}", [sys.executable, str(ROOT / f"scripts/verify-wave-e-postrollout-{SHORT}.py")]),
        ("wave_e_6b3da663", [sys.executable, str(ROOT / "scripts/verify-wave-e-postrollout-6b3da663.py")]),
        ("pending_corrections", pending_cmd),
    ):
        proc = subprocess.run(cmd, capture_output=True, text=True, cwd=str(ROOT))
        try:
            payload = json.loads(proc.stdout)
            gates[label] = {"exit_code": proc.returncode, "ok": payload.get("ok"),
                            "failures": payload.get("failures", [])[:10],
                            "checked_count": payload.get("checked_count"),
                            "scalar_checks": payload.get("scalar_checks")}
        except json.JSONDecodeError:
            gates[label] = {"exit_code": proc.returncode, "ok": False,
                            "stdout": proc.stdout[-500:], "stderr": proc.stderr[-500:]}
    result["gates"] = gates
    result["ok"] = (result["geo_cleared_ok"]
                    and result["wismar_ok"] in (True, None)
                    and result["db_recycling_ok"] in (True, None)
                    and result["rilei_ok"] in (True, None)
                    and gates[f"wave_e_{SHORT}"].get("ok") is True
                    and gates["pending_corrections"].get("ok") is True)
    print(json.dumps(result, indent=2, ensure_ascii=False))
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
