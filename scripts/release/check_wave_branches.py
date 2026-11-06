#!/usr/bin/env python3
"""Prueft die **nicht gemergten** Wellen-Zweige gegen die Release-Basis.

Die Zweige `wave1000-t3c` (SN-A) und `wave1000-t3d` (SN-B) sind **nicht** in
`rel12` enthalten: sie sind aus einem Merge-Konflikt entstanden und haben
dabei in 91 Dossiers je eine Notiz verloren. Ein solcher Zweig darf nicht
gemergt werden, solange die Notiz nicht wiederhergestellt ist.

Dieses Werkzeug weist das nach - je Zweig:
  * wie viele Dossiers eine **Vorwellen-Notiz verloren** haben (Block-Verlust),
  * wie viele neue Notizen entstanden sind,
  * ob der `certifications`-Eintrag aus der Basis unveraendert blieb.

Aufruf: python3 scripts/release/check_wave_branches.py <basis> <zweig> [<zweig> ...]
Exitcode 0 = kein Verlust, 1 = Notizverlust (nicht mergbar).
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def timeline_blocks(text):
    """Die Timeline-Notizen als Bloecke (Bullet + Fortsetzungszeilen)."""
    blocks = []
    current = None
    in_timeline = False
    for line in text.split("\n"):
        if line.strip() == "## Timeline":
            in_timeline = True
            continue
        if not in_timeline:
            continue
        if line.startswith("- "):
            if current is not None:
                blocks.append(current)
            current = [line]
        elif current is not None and line.strip() and line[:1] in (" ", "\t"):
            current.append(line)
        elif current is not None:
            blocks.append(current)
            current = None
    if current is not None:
        blocks.append(current)
    return ["\n".join(b) for b in blocks]


def cert_value(text):
    """Der Wert des Feldes `certifications` (oder None)."""
    m = re.search(r"^certifications:\s*(.*)$", text, re.M)
    return m.group(1).strip() if m else None


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__)
        return 2
    basis = sys.argv[1]
    zweige = sys.argv[2:]
    problems = []
    for zweig in zweige:
        dateien = subprocess.run(
            ["git", "diff", "--name-only", basis, zweig, "--", "dossiers/"],
            capture_output=True, text=True, check=True).stdout.split()
        verlust, neu, cert_geaendert = [], 0, 0
        for rel in dateien:
            alt = subprocess.run(["git", "show", f"{basis}:{rel}"],
                                 capture_output=True, text=True).stdout
            cur = subprocess.run(["git", "show", f"{zweig}:{rel}"],
                                 capture_output=True, text=True).stdout
            alt_b, cur_b = timeline_blocks(alt), timeline_blocks(cur)
            fehlend = [b for b in alt_b if b not in cur_b]
            if fehlend:
                verlust.append((rel, len(fehlend)))
            neu += len([b for b in cur_b if b not in alt_b])
            if cert_value(alt) != cert_value(cur):
                cert_geaendert += 1
        print(f"== {zweig}")
        print(f"   Dossiers: {len(dateien)} | geaenderte certifications: {cert_geaendert}")
        print(f"   neue Notizen: {neu} | Dossiers mit **Notizverlust**: {len(verlust)}")
        if verlust:
            print(f"   VERLUST in {sum(n for _, n in verlust)} Notizen "
                  f"ueber {len(verlust)} Dossiers - der Zweig ist NICHT mergbar")
            for rel, n in verlust[:5]:
                print(f"     - {rel}: {n} Notiz(en) verloren")
            problems.append(f"{zweig}: {len(verlust)} Dossiers mit Notizverlust")
    if problems:
        print("\nPROBLEME:")
        for p in problems:
            print("  -", p)
        return 1
    print("\nKein Notizverlust - die Zweige sind mergbar")
    return 0


if __name__ == "__main__":
    sys.exit(main())
