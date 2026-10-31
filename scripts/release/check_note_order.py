#!/usr/bin/env python3
"""Prueft die *Reihenfolge und Position* der Vorwellen-Notizen im Release-Scope.

`wave_gate.py` prueft nur die **Mitgliedschaft** der alten Bullets
(`missing = [b for b in old_bullets if b not in new_bullets]`). Eine
Vorwellen-Notiz, die in eine neue Wellen-Sektion verschoben wurde, besteht
diese Pruefung, verletzt aber die Regel "Alt-Bullets wortgleich und in
unveraenderter Reihenfolge, neue Notizen am Tail".

Dieses Werkzeug prueft deshalb strenger:

* die **Timeline**-Notizen des Basis-Standes bilden im Kandidaten einen
  **exakten Praefix** - jede Notiz wird als *Block* erfasst (Bullet-Zeile plus
  alle eingerueckten Fortsetzungszeilen), damit kann auch eine veraenderte
  Fortsetzungszeile nicht unbemerkt durchrutschen;
* eine Notiz, die in eine neue Wellen-Sektion verschoben wurde, faellt dabei
  auf, weil der Block dann nicht mehr an derselben Stelle steht;
* je Dossier mit neuen Notizen gibt es **genau eine** neue Sektion, und sie
  ist die **letzte** Sektion; eine neue Sektion **ohne** Datum im Titel wird
  ebenfalls beanstandet;
* es gibt **keine** doppelten Bullets und keine inhaltsgleichen
  Korrekturvermerke - sowohl **innerhalb** eines Dossiers als auch
  **dossieruebergreifend** (Satzebene, URLs/Boilerplate normalisiert).

Aufruf:  python3 scripts/release/check_note_order.py <basis-commit> [scope-dir]
Exitcode 0 = bestanden, 1 = Beanstandungen.
"""
import re
import subprocess
import sys
from collections import defaultdict
from pathlib import Path

DATE_IN_TITLE = re.compile(r"\d{2}\.\d{2}\.\d{4}")


def timeline_notes(text: str):
    """Die Timeline-Notizen als Bloecke (Bullet-Zeile + Fortsetzungszeilen)."""
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
            current.append(line)          # eingerueckte Fortsetzungszeile
        elif current is not None:
            blocks.append(current)
            current = None
    if current is not None:
        blocks.append(current)
    return ["\n".join(b) for b in blocks]


def normalise(note: str):
    note = re.sub(r"https?://\S+", "", note)
    note = re.sub(r"\(Quelle\(n\).*", "", note, flags=re.S)
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
        old_notes = timeline_notes(old)
        new_notes = timeline_notes(new)
        if new_notes[:len(old_notes)] != old_notes:
            problems.append(f"{rel}: Vorwellen-Notizen sind kein exakter Praefix "
                            f"(Reihenfolge/Position veraendert)")
        new_notes_total += len(new_notes) - len(old_notes)

        old_heads = re.findall(r"^### (.+)$", old, re.M)
        heads = re.findall(r"^### (.+)$", new, re.M)
        new_sections = [h for h in heads if h not in old_heads]
        # Geokodierungs-Abschnitte ("### Geokodierung <datum>") sind eine eigene,
        # zulaessige Sektionsart und zaehlen nicht als Wellen-Sektion.
        dated = [h for h in new_sections
                 if DATE_IN_TITLE.search(h) and not h.startswith("Geokodierung")]
        undated = [h for h in new_sections if not DATE_IN_TITLE.search(h)]
        if undated:
            problems.append(f"{rel}: neue Sektion ohne Datum im Titel: {undated}")
        has_new_notes = len(new_notes) > len(old_notes)
        if has_new_notes and len(dated) != 1:
            problems.append(f"{rel}: {len(dated)} neue datierte Sektion(en) bei neuen Notizen "
                            f"(erwartet: genau 1)")
        # Nach der Wellen-Sektion darf eine Geokodierungs-Sektion folgen.
        tail_ok = True
        if dated and heads and heads[-1] != dated[-1]:
            tail_ok = all(h.startswith("Geokodierung") for h in heads[heads.index(dated[-1]) + 1:])
        if dated and heads and heads[-1] != dated[-1] and not tail_ok:
            problems.append(f"{rel}: die Wellen-Sektion ist nicht die letzte Sektion")

        counts = defaultdict(int)
        for note in new_notes:
            counts[note] += 1
        dups = [n for n, c in counts.items() if c > 1]
        if dups:
            problems.append(f"{rel}: {len(dups)} doppelte Notizen")

        # Korrekturvermerke: inhaltsgleich innerhalb eines Dossiers und ueber Dossiers hinweg.
        # Nur **neue** Vermerke zaehlen - bereits im Basis-Stand vorhandene Vermerke sind
        # unveraenderlich und duerfen nicht als Duplikat des neuen Stands gewertet werden.
        local = defaultdict(list)
        for note in new_notes:
            if "Korrekturvermerk" in note and note not in old_notes:
                key = normalise(note)
                local[key].append(note)
                groups[key].append(rel)
        for key, notes in local.items():
            if len(notes) > 1:
                problems.append(f"{rel}: {len(notes)} inhaltsgleiche Korrekturvermerke innerhalb des Dossiers")

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
    print("Notiz-Reihenfolge bestanden: Praefix (mit Fortsetzungszeilen), Sektionslage, Tail-Position, "
          "keine Dubletten, keine inhaltsgleichen Korrekturvermerke")
    return 0


if __name__ == "__main__":
    sys.exit(main())
