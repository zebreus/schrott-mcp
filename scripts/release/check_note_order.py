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


def is_correction_note(note: str) -> bool:
    """Ein Vermerk, der eine Korrektur dokumentiert.

    Erkannt werden der bisherige Literalmarker und der Formatpraefix der
    Dossiers ("- [Korrektur <datum>: ...]"). Der Marker allein war ein
    Schlupfloch: zwei Dossiers mit demselben neuen Korrekturvermerk wurden
    nicht erkannt, weil ihr Text den Marker nicht woertlich enthielt.
    """
    return "Korrekturvermerk" in note or note.startswith("- [Korrektur ")


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


def notes_by_section(text: str):
    """Die Timeline-Notizen je Sektion ("### ..."), Reihenfolge wie im Dokument."""
    result = defaultdict(list)
    current = None
    in_timeline = False
    for line in text.split("\n"):
        if line.strip() == "## Timeline":
            in_timeline = True
            continue
        if not in_timeline:
            continue
        if line.startswith("### "):
            current = line[4:].strip()
            continue
        if line.startswith("- "):
            result[current].append(line)
    return result


def datum(text: str):
    """Das Datum einer Sektionsueberschrift als sortierbares Tupel (jjjj, mm, tt)."""
    m = DATE_IN_TITLE.search(text)
    if not m:
        return None
    tag, monat, jahr = m.group(0).split(".")
    return (int(jahr), int(monat), int(tag))


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
        geo = [h for h in new_sections if h.startswith("Geokodierung")]
        undated = [h for h in new_sections if not DATE_IN_TITLE.search(h)]
        if undated:
            problems.append(f"{rel}: neue Sektion ohne Datum im Titel: {undated}")
        has_new_notes = len(new_notes) > len(old_notes)
        if has_new_notes and not dated:
            # Reine Geokodierungs-Welle: die neuen Notizen liegen in genau einem
            # neuen Geokodierungs-Abschnitt, der die letzte Sektion sein muss.
            # Steckt eine neue Notiz in einer bereits bestehenden Sektion, gibt
            # es weiterhin keine neue Wellen-Sektion - das bleibt ein Fehler.
            if len(geo) != 1:
                problems.append(f"{rel}: {len(dated)} neue datierte Sektion(en) bei neuen Notizen "
                                f"(erwartet: genau 1)")
            elif heads and heads[-1] != geo[-1]:
                problems.append(f"{rel}: die Geokodierungs-Sektion ist nicht die letzte Sektion")
            else:
                # Die neue Sektion muss die neuen Notizen auch wirklich
                # enthalten - eine leere Sektion neben einer neuen Notiz in
                # einer bestehenden Sektion ist keine gueltige Welle.
                by_section = notes_by_section(new)
                in_geo = set(by_section.get(geo[-1], []))
                outside = [n for n in new_notes[len(old_notes):]
                           if n.split("\n")[0] not in in_geo]
                if outside:
                    problems.append(f"{rel}: {len(outside)} neue Notiz(en) liegen nicht in der "
                                    f"Geokodierungs-Sektion")
        elif has_new_notes:
            # Mehrere **Wellen-Sektionen** sind erlaubt: ein Dossier kann in
            # zwei Wellen desselben Release-Scopes gelegen haben (der Pool
            # enthaelt auch Dossiers, die eine fruehere Welle nicht mit
            # Adresse und Website versehen hat). Jede Welle traegt ihre eigene
            # datierte Sektion am Timeline-Ende, in aufsteigender Reihenfolge -
            # ein Veroeffentlichter Vermerk wird **nicht** in place geaendert
            # (Hausregel). Das prueft der Prefix-Check oben: die Vorwellen-
            # Notizen muessen ein exakter Praefix bleiben.
            #
            # Danach duerfen beliebig viele **Korrektur-Sektionen** folgen: eine
            # spaetere Welle darf einen veroeffentlichten Vermerk nicht in
            # place aendern (Hausregel), sondern traegt ihre Korrektur als
            # eigene datierte Sektion am Timeline-Ende nach.
            #
            # Es gibt auch den Fall, dass ein Dossier **nur** Korrektur-
            # Sektionen erhaelt (die Welle hat dort nichts Neues recherchiert,
            # sondern nur einen bestehenden Vermerk berichtigt). Dann gibt es
            # keine Wellen-Sektion - erlaubt sind dann ausschliesslich
            # Korrektur-Sektionen, alle am Timeline-Ende.
            wellen = [h for h in dated if not h.startswith("Korrektur")]
            korrekturen = [h for h in dated if h.startswith("Korrektur")]
            # Die datierten Sektionen bilden **Bloecke**: eine Wellen-Sektion und
            # die Korrektur-Sektionen, die zu ihr gehoeren. Eine Korrektur muss
            # nach einer Wellen-Sektion stehen - die kann auch aus einem
            # frueheren Release stammen, wenn die Welle dort nichts Neues
            # recherchiert, sondern nur einen bestehenden Vermerk berichtigt
            # hat - und ihr Datum darf nicht vor dem Datum dieser Welle liegen.
            bloecke_ok = True
            for i, h in enumerate(dated):
                if not h.startswith("Korrektur"):
                    continue
                vor = [x for x in heads[:heads.index(h)]
                       if not x.startswith("Korrektur")
                       and not x.startswith("Geokodierung")
                       and DATE_IN_TITLE.search(x)]
                if not vor:
                    # Keine Wellen-Sektion im Dossier: die Korrektur berichtigt
                    # einen Vermerk, der mit dem Seed-Stand eingezogen wurde.
                    continue
                if datum(h) < datum(vor[-1]):
                    bloecke_ok = False
                    break
            # Die Wellen-Sektionen stehen in aufsteigender Reihenfolge.
            aufsteigend = [datum(h) for h in wellen] == sorted(datum(h) for h in wellen)
            # Die letzte Sektion der Datei ist die letzte datierte Sektion -
            # oder eine Geokodierungs-Sektion: die gehoert zur Welle und
            # steht am Ende (sie dokumentiert die Geokodierung der Welle).
            letzter = heads[-1] if heads else ""
            abschluss_ok = letzter == dated[-1] or letzter.startswith("Geokodierung")
            if not bloecke_ok or not abschluss_ok or not aufsteigend:
                problems.append(f"{rel}: {len(dated)} neue datierte Sektion(en) bei neuen Notizen "
                                f"(erwartet: Wellen-Sektionen in aufsteigender Reihenfolge, "
                                f"Korrektur-Sektionen danach)")
        # Nach der Wellen-Sektion darf eine Geokodierungs-Sektion folgen.
        tail_ok = True
        if dated and heads and heads[-1] != dated[-1]:
            tail_ok = all(h.startswith("Geokodierung") for h in heads[heads.index(dated[-1]) + 1:])
        if dated and heads and heads[-1] != dated[-1] and not tail_ok:
            problems.append(f"{rel}: die Wellen-Sektion ist nicht die letzte Sektion")
        # Eine Geokodierungs-Sektion gehoert **nach** der Wellen-Sektion: sie
        # dokumentiert das, was die Wellen-Recherche geokoordiniert hat.
        if dated and geo and heads:
            # Eine Geokodierungs-Sektion gehoert **nach** der Wellen-Sektion:
            # sie dokumentiert das, was die Wellen-Recherche geokoordiniert
            # hat. Massgeblich ist die **erste** Wellen-Sektion des Releases -
            # ein Dossier kann in zwei Wellen gelegen haben, und die
            # Geokodierung der frueheren Welle steht dann zwischen den beiden
            # Wellen-Sektionen, nicht vor der Wellen-Sektion des Releases.
            wave_at = heads.index(dated[0])
            for g in geo:
                if g in heads and heads.index(g) < wave_at:
                    problems.append(f"{rel}: die Geokodierungs-Sektion steht vor der "
                                    f"Wellen-Sektion")
                    break

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
            if is_correction_note(note) and note not in old_notes:
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
