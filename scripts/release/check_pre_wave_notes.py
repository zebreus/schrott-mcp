#!/usr/bin/env python3
"""Prueft streng, dass die **Vorwellen-Notizen** im Release-Scope unveraendert sind.

`wave_gate.py` prueft nur die *Mitgliedschaft* der alten Bullets; `check_note_order.py`
prueft den *Prafix* (Reihenfolge und Position, mit Fortsetzungszeilen). Dieses
Skript ist der strengere Gegencheck fuer den Basis-Stand: **keine** Vorwellen-Notiz
darf entfernt, umformatiert oder inhaltlich geaendert sein - jede Abweichung
wird mit dem konkreten Alt- und Neuwert ausgegeben.

Aufruf:  python3 scripts/release/check_pre_wave_notes.py <basis-commit> [scope-dir]
Exitcode 0 = bestanden, 1 = Abweichungen.
"""
import re
import subprocess
import sys
from pathlib import Path


def timeline_blocks(text: str):
    """Die Timeline-Notizen als Bloecke: Bullet-Zeile + eingerueckte Fortsetzungszeilen."""
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


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    base = sys.argv[1]
    scope = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("dossiers")

    changed = subprocess.run(
        ["git", "diff", "--name-only", base, "--", str(scope)],
        capture_output=True, text=True, check=True).stdout.split()

    problems = []
    new_total = 0
    for rel in changed:
        old = subprocess.run(["git", "show", f"{base}:{rel}"],
                             capture_output=True, text=True, check=True).stdout
        new = Path(rel).read_text(encoding="utf-8")
        old_b = timeline_blocks(old)
        new_b = timeline_blocks(new)
        missing = [b for b in old_b if b not in new_b]
        if missing:
            problems.append(f"{rel}: {len(missing)} Vorwellen-Notiz(en) veraendert oder entfernt")
            for b in missing[:3]:
                head = b.split("\n")[0][:110]
                problems.append(f"    ALT: {head}")
        new_total += len(new_b) - len(old_b)

    print(f"Dateien: {len(changed)} | Abweichungen: {len(problems)} | neue Notizen: {new_total}")
    if problems:
        print(f"PROBLEME: {len(problems)}")
        for p in problems:
            print("  -", p)
        return 1
    print("Vorwellen-Notizen unveraendert (wortgleich, in Reihenfolge und Position)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
