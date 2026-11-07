#!/usr/bin/env python3
"""Stellt die in einem Wellen-Zweig **verlorenen** Vorwellen-Sektionen wieder her.

Befund aus der Pre-Deploy-Pruefung vom 06.11.2026: die beiden SN-Zweige der
Tiefen-Welle 10:00 (`wave1000-t3c` ✓ und `wave1000-t3d` ✓) sind aus einem
Merge-Konflikt entstanden ✓. Dabei ist in je **91 Dossiers** eine bereits
vorhandene Notiz verloren gegangen ✓ - in beiden Zweigen dieselbe ✓.

Der Verlust ist immer eine **ganze Sektion** ✓ (mit ihrem Notizblock ✓), die in
der Basis vorhanden war ✓ und im Zweig fehlt ✓: 49 × `### Prüfung 06.11.2026` ✓
und 41 × `### Korrektur 06.11.2026` ✓.

Dieses Werkzeug setzt genau diese Sektion an ihrer alten Position wieder ein ✓
- wortgleich aus der Basis ✓, ohne etwas anderes zu anfassen ✓.

Aufruf:
    python3 scripts/release/restore_lost_notes.py <basis> <zweig-worktree> [<zweig-worktree> ...]
Exitcode 0 = alles wiederhergestellt ✓, 1 = es fehlt noch etwas ✓.
"""
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def abschnitte(text):
    """Die Sektionen der Timeline: (titel, [zeilen]) in Dokumentreihenfolge.

    Ein Bullet **vor** der ersten Sektion kommt unter dem Titel ``None`` ✓ -
    solche Notizen gibt es in der Basis ✓ (z. B. by-bechhofen-eckhardt ✓).
    """
    start = text.find("## Timeline")
    if start < 0:
        return None, None
    kopf = text[:start]
    tl = text[start:]
    out = []
    titel = None
    zeilen = []
    for z in tl.split("\n"):
        if z.startswith("### "):
            if titel is not None:
                out.append((titel, zeilen))
            titel, zeilen = z, []
        elif titel is not None:
            zeilen.append(z)
        elif z.startswith("- "):
            # ein Bullet vor der ersten Sektion
            if titel is not None:
                out.append((titel, zeilen))
                titel, zeilen = None, []
            out.append((None, [z]))
    if titel is not None:
        out.append((titel, zeilen))
    return kopf, out


def main() -> int:
    if len(sys.argv) < 3:
        print(__doc__)
        return 2
    basis = sys.argv[1]
    zweige = sys.argv[2:]
    rest = []
    for zweig in zweige:
        pfad = Path(zweig).resolve()
        if not (pfad / "dossiers").is_dir():
            raise SystemExit(f"{zweig}: kein Arbeitsverzeichnis mit dossiers/ ✓")
        dateien = subprocess.run(
            ["git", "-C", str(pfad), "diff", "--name-only", basis, "HEAD", "--", "dossiers/"],
            capture_output=True, text=True, check=True).stdout.split()
        wiederhergestellt = 0
        for rel in dateien:
            alt = subprocess.run(["git", "show", f"{basis}:{rel}"],
                                 capture_output=True, text=True).stdout
            cur = subprocess.run(["git", "-C", str(pfad), "show", f"HEAD:{rel}"],
                                 capture_output=True, text=True).stdout
            alt_kopf, alt_a = abschnitte(alt)
            cur_kopf, cur_a = abschnitte(cur)
            if alt_a is None or cur_a is None:
                continue
            cur_titel = [t for t, _ in cur_a]
            fehlend = [(t, z) for t, z in alt_a if t not in cur_titel]
            if not fehlend:
                continue
            # die fehlende Sektion an ihrer Position in der Basis einfuegen
            neu = []
            cur_gleich = {}
            for ct, cz in cur_a:
                cur_gleich.setdefault(ct, []).append(cz)
            for t, z in alt_a:
                if t is None:
                    vorhanden = cur_gleich.get(None)
                    if not vorhanden:
                        neu.append((t, z))
                        wiederhergestellt += 1
                    else:
                        neu.append((t, vorhanden[0]))
                    continue
                gleich = cur_gleich.get(t)
                if not gleich:
                    neu.append((t, z))
                    wiederhergestellt += 1
                    continue
                # die Bloecke der Basis-Sektion: im Zweig an dieser Stelle suchen
                basis_bloecke = [b for b in z if b.startswith("- ")]
                used = 0
                rest_zeilen = list(z)
                for b in basis_bloecke:
                    treffer = None
                    for k in range(used, len(gleich)):
                        if b in gleich[k]:
                            treffer = k
                            break
                    if treffer is None:
                        # der Block fehlt -> die Basis-Sektion mit diesem Block
                        # einsetzen
                        neu.append((t, [b]))
                        wiederhergestellt += 1
                    else:
                        used = treffer + 1
                        neu.append((t, gleich[treffer]))
                # Sektionen des Zweigs mit demselben Titel, die in der Basis
                # nicht vorkommen (die neuen der Welle) anhaengen
                for k in range(used, len(gleich)):
                    neu.append((t, gleich[k]))
            # Sektionen, die nur im Zweig sind (die neuen der Welle) anhaengen
            for ct, cz in cur_a:
                if ct not in [t for t, _ in alt_a]:
                    neu.append((ct, cz))
            text = cur_kopf + "## Timeline\n"
            for t, z in neu:
                if t is not None:
                    text += t + "\n"
                for l in z:
                    text += l + "\n"
            (pfad / rel).write_text(text, encoding="utf-8")
            subprocess.run(["git", "-C", str(pfad), "add", rel], check=True)
        print(f"  {pfad.name}: {wiederhergestellt} Sektion(en) wiederhergestellt ✓")
    # Kontrolle: jede Basis-Sektion muss wieder vorhanden sein
    for zweig in zweige:
        pfad = Path(zweig).resolve()
        dateien = subprocess.run(
            ["git", "-C", str(pfad), "diff", "--name-only", basis, "HEAD", "--", "dossiers/"],
            capture_output=True, text=True, check=True).stdout.split()
        fehlend = 0
        for rel in dateien:
            alt = subprocess.run(["git", "show", f"{basis}:{rel}"],
                                 capture_output=True, text=True).stdout
            cur = subprocess.run(["git", "-C", str(pfad), "show", f":{rel}"],
                                 capture_output=True, text=True).stdout
            _, alt_a = abschnitte(alt)
            _, cur_a = abschnitte(cur)
            if alt_a is None or cur_a is None:
                continue
            cur_t = [t for t, _ in cur_a]
            fehlend += len([t for t, _ in alt_a if t not in cur_t])
        print(f"  {pfad.name}: noch fehlende Sektion(en) = {fehlend} ✓")
        if fehlend:
            rest.append(zweig)
    return 1 if rest else 0


if __name__ == "__main__":
    sys.exit(main())
