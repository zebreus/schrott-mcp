#!/usr/bin/env python3
"""Prueft je Händler, ob die veralteten Preisschluessel noch auf der Seite stehen.

Ein Handler nimmt je Lauf die Preise auf, die seine Seite heute anbietet.
Bietet die Seite eine Sorte (oder eine Preisstufe) nicht mehr an - oder nennt
sie die Sorte ohne Preis -, bleibt der fruehere Preis als "aktueller Preis"
stehen. Dieser Lauf holt die Seite des Handlers und prueft, ob die
**Variante** des veralteten Preises dort noch vorkommt ✓.

**Das ist eine erste Einordnung, kein Beweis** ✓: kommt die Variante auf der
Seite vor ✓, kann sie dort trotzdem ohne Preis stehen ✓ - dann ueberspringt
der Handler sie ✓ und der Current-Pointer gehoert trotzdem zurueckgezogen
✓. Umgekehrt kann die Seite eine aehnlich benannte Sorte fuehren ✓, ohne
dass der Schluessel (material, variant) existiert ✓. Der Beweis ist je
Händler die **Preiszelle** der Seite ✓ - die Liste markiert nur die
Kandidaten ✓ ✓.

Die Seite wird **nur gelesen** ✓ ✓; die Produktionsdatenbank bleibt
read-only ✓ ✓.

Aufruf:
    python3 scripts/release/check_stale_price_pages.py [--json <datei>]
                                        [--liste <datei>]

Exitcode 0 = der Lauf ist durch ✓, 1 = ein Fehler ✓.
"""
import argparse
import json
import re
import sqlite3
import subprocess
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
UA = ("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 "
      "(KHTML, like Gecko) Chrome/120 Safari/537.36")


def seite(url, timeout=45):
    req = urllib.request.Request(url, headers={"User-Agent": UA})
    with urllib.request.urlopen(req, timeout=timeout) as resp:
        return resp.read().decode("utf-8", "replace")


def text(html):
    html = re.sub(r"<script.*?</script>", " ", html, flags=re.S)
    html = re.sub(r"<style.*?</style>", " ", html, flags=re.S)
    aus = re.sub(r"<[^>]+>", " ", html)
    aus = aus.replace("&amp;", "&").replace("&nbsp;", " ")
    aus = aus.replace("&quot;", '"').replace("&#39;", "'")
    return re.sub(r"\s+", " ", aus)


def handler_url(slug):
    """Die Preis-URL des Handlers zu einem Slug (aus dem Quelltext)."""
    for datei in (ROOT / "crates/ingestion/src/traders/handlers").glob("*.rs"):
        quelle = datei.read_text(encoding="utf-8", errors="replace")
        if f'"{slug}"' not in quelle:
            continue
        treffer = re.search(r'pub const URL[A-Z_]*:\s*&str\s*=\s*"([^"]+)"', quelle)
        if treffer:
            return treffer.group(1)
    return None


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--json", type=Path, help="Zieldatei für die Liste")
    ap.add_argument("--liste", type=Path,
                    default=ROOT / "scripts/release/stale-current-prices.json",
                    help="Kandidatenliste aus find_stale_current_prices.py")
    args = ap.parse_args()

    liste = json.loads(args.liste.read_text(encoding="utf-8")) \
        if args.liste.exists() else None
    if liste is None:
        print("Keine Liste gefunden - erst find_stale_current_prices.py laufen lassen ✗")
        return 1

    bericht = []
    for eintrag in liste:
        slug = eintrag["slug"]
        url = handler_url(slug)
        if url is None:
            print(f"{slug}: keine Handler-URL gefunden ✗")
            bericht.append({**eintrag, "page": None, "urteil": "keine URL"})
            continue
        try:
            inhalt = text(seite(url))
        except Exception as exc:  # noqa: BLE001 - der Lauf soll weitergehen
            print(f"{slug}: Seite nicht erreichbar ({exc}) ✗")
            bericht.append({**eintrag, "page": url, "urteil": "Seite nicht erreichbar"})
            continue
        verschwunden, noch_da = [], []
        for preis in eintrag["prices"]:
            marke = preis["variant"]
            if not marke:
                continue
            if marke.lower() in inhalt.lower():
                noch_da.append(marke)
            else:
                verschwunden.append(marke)
        urteil = ("alle Schlüssel verschwunden" if not noch_da else
                  "kein Schlüssel verschwunden" if not verschwunden else
                  f"{len(verschwunden)} verschwunden, {len(noch_da)} noch da")
        print(f"{slug}: {urteil}")
        bericht.append({**eintrag, "page": url, "urteil": urteil,
                        "verschwunden": verschwunden, "noch_da": noch_da})

    if args.json:
        args.json.write_text(json.dumps(bericht, ensure_ascii=False, indent=1),
                             encoding="utf-8")
        print(f"Gespeichert: {args.json} ✓")
    return 0


if __name__ == "__main__":
    sys.exit(main())
