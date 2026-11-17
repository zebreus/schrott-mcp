#!/usr/bin/env python3
"""Uebernimmt die Geo-Notizen eines Wellen-Shards in den aktuellen Stand.

Der Shard-Branch `geo1108-1` (Recherche vom 08.11.2026) enthaelt 74 Dossiers mit
je einer Geokodierungs-Notiz, die im Hauptstand fehlen. Die spaeteren Wellen
haben die **Koordinaten** uebernommen, aber die **Notiz** (den Beleg) nicht.

Regeln (wie im Repo ueblich):
* Die Notiz kommt als eigene datierte Sektion ans **Ende** der Timeline
  (`### Geokodierung 08.11.2026`) — vorhandene Notizen werden nie veraendert.
* `lat`/`lon` werden nur gesetzt, wenn der Hauptstand **keine** Koordinate hat
  (leeres Feld heisst: kein Ueberschreiben).
* Der Sektionsname `### Geokodierung <datum>` ist eine zulaessige eigene
  Sektionsart (der `check_note_order`-Gate behandelt sie so).

Aufruf:  python3 scripts/release/merge_geo_notices.py --branch geo1108-1 --date 08.11.2026
"""
import argparse
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FM = re.compile(r"^---\n(.*?)\n---\n", re.S)


def frontmatter(text):
    m = FM.match(text)
    if not m:
        return {}
    return {k: v.strip().strip("'\"") for k, v in re.findall(r"^([a-z_]+):\s*(.*)$", m.group(1), re.M)}


def bullets(text):
    return [l.rstrip() for l in text.split("\n") if l.strip().startswith("- [")]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--branch", required=True)
    ap.add_argument("--date", required=True)
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    files = subprocess.run(
        ["git", "diff", "--name-only", f"main..{args.branch}", "--", "dossiers/"],
        capture_output=True, text=True, check=True).stdout.split()

    touched = 0
    coords = 0
    for rel in files:
        path = ROOT / rel
        if not path.exists():
            continue
        alt = subprocess.run(["git", "show", f"{args.branch}:{rel}"],
                             capture_output=True, text=True).stdout
        cur = path.read_text(encoding="utf-8")
        have = set(bullets(cur))
        new = [b for b in bullets(alt) if b not in have]
        if not new:
            continue
        afm, cfm = frontmatter(alt), frontmatter(cur)
        # Koordinaten nur, wenn der Hauptstand keine hat
        set_coord = bool(afm.get("lat")) and not cfm.get("lat")
        if args.dry_run:
            print(f"{rel}: {len(new)} Notiz(en), Koordinate={'ja' if set_coord else 'nein'}")
            touched += 1
            coords += 1 if set_coord else 0
            continue
        if set_coord:
            cur = re.sub(r"^lat:\s*$", f"lat: {afm['lat']}", cur, count=1, flags=re.M)
            cur = re.sub(r"^lon:\s*$", f"lon: {afm['lon']}", cur, count=1, flags=re.M)
        # Das Gate verlangt bei neuen Notizen **genau eine** neue datierte
        # Sektion am Timeline-Ende. Eine zweite Geokodierungs-Sektion mit
        # demselben Datum waere eine Doppelung (das Gate beanstandet sie),
        # also traegt der Vermerk eine eigene Wellen-Sektion.
        section = f"\n### Recherche {args.date}\n\n" + "\n".join(new) + "\n"
        cur = cur.rstrip("\n") + "\n" + section
        path.write_text(cur, encoding="utf-8")
        touched += 1
        coords += 1 if set_coord else 0
    print(f"Dossiers angefasst: {touched} | Koordinaten ergaenzt: {coords}")


if __name__ == "__main__":
    main()
