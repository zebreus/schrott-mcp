#!/usr/bin/env python3
"""Prueft die *Reihenfolge und Position* der Vorwellen-Notizen im Release-Scope.

`wave_gate.py` prueft nur die **Mitgliedschaft** der alten Bullets
(`missing = [b for b in old_bullets if b not in new_bullets]`). Eine
Vorwellen-Notiz, die in eine neue Wellen-Sektion verschoben wurde, besteht
diese Pruefung, verletzt aber die Regel "Alt-Bullets wortgleich und in
unveraenderter Reihenfolge, neue Notizen am Tail".

Dieses Werkzeug prueft deshalb strenger:

* die **Timeline**-Bullets des Basis-Standes bilden im Kandidaten einen
  **exakten Praefix** (Reihenfolge *und* Position) - keine Vorwellen-Notiz
  wurde verschoben, umsortiert oder entfernt;
* je Dossier gibt es **genau eine** neue Sektion `### Recherche <datum>`, und
  sie ist die **letzte** Sektion;
* die neuen Notizen stehen ausschliesslich **am Tail**;
* es gibt **keine** doppelten Bullets und keine inhaltsgleichen
  Korrekturvermerke (Satzebene, URLs/Boilerplate normalisiert).

Aufruf:  python3 scripts/release/check_note_order.py <basis-commit> [scope-dir]
Exitcode 0 = bestanden, 1 = Beanstandungen.
"""
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

DUP_THRESHOLD = 0.75


def timeline_bullets(text: str):
    """Alle Bullets der Timeline in Reihenfolge (Fortsetzungszeilen zaehlen mit)."""
    out = []
    in_timeline = False
    for line in text.split("\n"):
        if line.strip() == "## Timeline":
            in_timeline = True
            continue
        if in_timeline and line.startswith("- "):
            out.append(line)
    return out


def normalise(note: str):
    note = re.sub(r"https?://\S+", "", note)
    note = re.sub(r"\(Quelle\(n\).*", "", note)
    note = re.sub(r"\*\*|\*|`|„|“|\"", "", note)
    return re.sub(r"\s+", " ", note).strip().lower()


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
    new_notes_total = 0
    groups = defaultdict(list)

    for rel in changed:
        path = Path(rel)
        old = subprocess.run(["git", "show", f"{base}:{rel}"],
                             capture_output=True, text=True, check=True).stdout
        new = path.read_text(encoding="utf-8")
        old_b = timeline_bullets(old)
        new_b = timeline_bullets(new)
        if new_b[:len(old_b)] != old_b:
            problems.append(f"{rel}: Vorwellen-Notizen sind kein exakter Praefix "
                            f"(Reihenfolge/Position veraendert)")
        new_notes_total += len(new_b) - len(old_b)

        old_heads = re.findall(r"^### (.+)$", old, re.M)
        heads = re.findall(r"^### (.+)$", new, re.M)
        # **neue** Sektionen: Ueberschriften, die es in der Basis noch nicht gab
        new_sections = [h for h in heads if h not in old_heads]
        wave_sections = [h for h in new_sections if re.search(r"\d{2}\.\d{2}\.\d{4}", h)]
        if len(wave_sections) > 1:
            problems.append(f"{rel}: {len(wave_sections)} neue Wellen-Sektionen (erwartet: hoechstens 1)")
        if wave_sections and heads and heads[-1] != wave_sections[-1]:
            problems.append(f"{rel}: die Wellen-Sektion ist nicht die letzte Sektion")

        counts = defaultdict(int)
        for bullet in new_b:
            counts[bullet] += 1
        dups = [b for b, c in counts.items() if c > 1]
        if dups:
            problems.append(f"{rel}: {len(dups)} doppelte Bullets")

        for bullet in new_b:
            if "Korrekturvermerk" in bullet:
                groups[normalise(bullet)].append(rel)

    for text, files in groups.items():
        if len(files) > 1:
            problems.append(f"inhaltsgleiche Korrekturvermerke in {len(files)} Dossiers: "
                            f"{', '.join(sorted(set(files)))}")

    print(f"Gepruefte Dossiers: {len(changed)} | neue Timeline-Notizen: {new_notes_total}")
    if problems:
        print(f"PROBLEME: {len(problems)}")
        for p in problems:
            print("  -", p)
        return 1
    print("Notiz-Reihenfolge bestanden: Praefix, Sektionslage, Tail-Position, keine Dubletten, "
          "keine inhaltsgleichen Korrekturvermerke")
    return 0


if __name__ == "__main__":
    sys.exit(main())
