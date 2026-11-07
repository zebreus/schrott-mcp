#!/usr/bin/env python3
"""Extrahiert den echten Strassenanteil aus den verschmutzten Seed-Zeilen.

Beim Import hat das Verzeichnis-Portal (11880) den Namen und den Ort mit in
die Adresszeile geschrieben. Im Dossier steht deshalb der Name und die Stadt
noch einmal vor der eigentlichen Strasse - der Wert ist als Anschrift
unbrauchbar.

Dieses Skript rechnet nur: es schneidet den wiederholten Praefix ab und nennt
den Kandidaten. **Bevor** der Wert in ein Dossier kommt, braucht er eine
zweite unabhaengige Quelle (Auftrag:
`research/auftrag-verschmutzte-seed-strassen.md`).

Aufruf:
    python3 scripts/release/extract_polluted_streets.py [--json <datei>]

Exitcode 0 = die Liste ist erzeugt ✓, 1 = ein Fehler ✓.
"""
import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def felder(text):
    """Die Frontmatter-Felder eines Dossiers."""
    out = {}
    for line in text.split("\n"):
        if line.startswith("## "):
            break
        m = re.match(r"^([a-z_]+):\s*(.*)$", line)
        if m:
            out[m.group(1)] = m.group(2).strip().strip("'\"")
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--json", type=Path, help="Zieldatei für die Liste")
    args = ap.parse_args()

    dateien = subprocess.run(
        ["git", "-C", str(ROOT), "ls-tree", "-r", "--name-only", "HEAD", "dossiers/"],
        capture_output=True, text=True, check=True).stdout.split()
    treffer = []
    for rel in dateien:
        text = subprocess.run(["git", "-C", str(ROOT), "show", f"HEAD:{rel}"],
                              capture_output=True, text=True).stdout
        f = felder(text)
        strasse, name, stadt = f.get("street", ""), f.get("name", ""), f.get("city", "")
        if not (strasse and name and stadt):
            continue
        # Das Muster: der Wert beginnt mit dem Namen und nennt die Stadt
        # ein zweites Mal - die Strasse steht dahinter.
        kandidat = strasse
        if kandidat.startswith(name):
            kandidat = kandidat[len(name):].strip()
        if stadt and kandidat.startswith(stadt):
            kandidat = kandidat[len(stadt):].strip()
        # Nach dem Ortsnamen muss ein Strassenname folgen - ein GROSSBUCHSTABE
        # oder eine Ziffer. Sonst schneidet der Ortsname auch den Anfang des
        # Strassennamens ab ("Witten" aus "Wittener Straße").
        if not kandidat or not re.match(r"[A-ZÄÖÜ0-9]", kandidat):
            continue
        if kandidat == strasse or len(kandidat) < 5:
            continue
        if not re.search(r"\d", kandidat):
            # ohne Hausnummer ist der Rest kein Strassenname
            continue
        slug = f.get("slug", "")
        treffer.append({
            "slug": slug,
            "datei": rel,
            "ist": strasse,
            "kandidat": kandidat,
            "name": name,
            "stadt": stadt,
            "postcode": f.get("postcode", ""),
        })

    for t in treffer:
        print(f"{t['slug'][:46]:48s} {t['ist'][:52]:54s} → {t['kandidat']}")
    print(f"\nDossiers mit verschmutzter Seed-Zeile: {len(treffer)}")
    if args.json:
        args.json.write_text(json.dumps(treffer, ensure_ascii=False, indent=1),
                             encoding="utf-8")
        print(f"Gespeichert: {args.json} ✓")
    return 0


if __name__ == "__main__":
    sys.exit(main())
