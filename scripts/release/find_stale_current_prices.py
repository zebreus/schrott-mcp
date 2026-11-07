#!/usr/bin/env python3
"""Listet die Current-Preise, die der Handler heute nicht mehr aufnimmt.

Ein Handler nimmt je Lauf die Preise auf, die seine Seite **heute** anbietet.
Bietet die Seite eine Sorte (oder eine Preisstufe) nicht mehr an, bleibt der
frühere Preis als "aktueller Preis" stehen - die Pipeline zieht Current-
Pointer für normale Handler **nicht** zurück (das wäre bei einem
vorübergehend kaputten Seitenbild gefährlich). Das Projekt hat dafür
Einzelfälle über Migrationen gelöst (Migration 7 `goldtrans`, Migration 8
`heraeus`: "Retire only their current pointers; keep every historical field
intact").

Dieses Skript liefert die Kandidatenliste: je Händler die Current-Preise,
deren `observed_at` älter ist als der letzte erfolgreiche Handler-Lauf -
also genau die Zeilen, die der Handler **nicht** mehr auffrischt.

Aufruf:
    python3 scripts/release/find_stale_current_prices.py [--json <datei>]

Exitcode 0 = die Liste ist erzeugt ✓, 1 = ein Fehler ✓.
"""
import argparse
import datetime
import json
import sqlite3
import sys
from pathlib import Path

PROD = "/var/lib/schrott-mcp/public.db"
INTERNAL = "/var/lib/schrott-mcp/internal.db"
ALTER = datetime.timedelta(days=7)


def read_only(path):
    con = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
    con.row_factory = sqlite3.Row
    con.execute("PRAGMA query_only=ON")
    return con


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--json", type=Path, help="Zieldatei für die Liste")
    args = ap.parse_args()

    public = read_only(PROD)
    internal = read_only(INTERNAL)
    jetzt = datetime.datetime.now(datetime.timezone.utc)

    def alter(iso):
        try:
            stand = datetime.datetime.fromisoformat(iso.replace("+00:00", ""))
        except (TypeError, ValueError):
            return None
        return jetzt - stand.replace(tzinfo=datetime.timezone.utc)

    haendler = {}
    for row in internal.execute(
        "SELECT scraper, items_upserted, status, started_at FROM ("
        "  SELECT *, ROW_NUMBER() OVER (PARTITION BY scraper ORDER BY id DESC) AS rn"
        "  FROM ingestion_steps WHERE status='ok') WHERE rn=1"
    ):
        haendler[row["scraper"]] = dict(row)

    kandidaten = {}
    for row in public.execute(
        "SELECT t.slug, m.slug AS material, p.variant, p.price, p.unit, p.currency, "
        "       p.observed_at, p.source_url "
        "FROM current_prices c "
        "JOIN traders t ON t.id=c.trader_id "
        "JOIN materials m ON m.id=c.material_id "
        "JOIN prices p ON p.id=c.price_id "
        "ORDER BY t.slug, m.slug, p.variant"
    ):
        alt = alter(row["observed_at"])
        if alt is None or alt < ALTER:
            continue
        kandidaten.setdefault(row["slug"], []).append({
            "material": row["material"],
            "variant": row["variant"],
            "price": row["price"],
            "unit": row["unit"],
            "currency": row["currency"],
            "observed_at": row["observed_at"],
            "age_days": alt.days,
            "source_url": row["source_url"],
        })

    # je Händler: der Umfang des letzten erfolgreichen Laufs danebenstellen -
    # die Differenz zur Zahl der Current-Preise zeigt den Umfang.
    bericht = []
    for slug, preise in sorted(kandidaten.items()):
        gesamt = public.execute(
            "SELECT COUNT(*) FROM current_prices c JOIN traders t ON t.id=c.trader_id "
            "WHERE t.slug=?", (slug,)).fetchone()[0]
        lauf = haendler.get(slug)
        bericht.append({
            "slug": slug,
            "stale": len(preise),
            "current_total": gesamt,
            "last_step": lauf,
            "prices": preise,
        })
        print(f"{slug}  stale={len(preise)}  current={gesamt}  "
              f"last_step={lauf['items_upserted'] if lauf else '-'} "
              f"({lauf['started_at'][:19] if lauf else 'kein Lauf'})")

    print(f"\nHändler: {len(bericht)} | veraltete Preispaare: "
          f"{sum(b['stale'] for b in bericht)}")
    if args.json:
        args.json.write_text(json.dumps(bericht, ensure_ascii=False, indent=1),
                             encoding="utf-8")
        print(f"Gespeichert: {args.json} ✓")
    public.close()
    internal.close()
    return 0


if __name__ == "__main__":
    sys.exit(main())
