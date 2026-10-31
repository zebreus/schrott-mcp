#!/usr/bin/env python3
"""Prueft die **Hausnummern-Aufzaehlungen** der Wellen-Notizen live gegen OSM.

Die haeufigste Sachfehlerklasse der Wellen ist eine Aufzaehlung, die
Hausnummern nennt, die es auf der genannten Strasse nicht gibt (z. B. die
Dossier-Hausnummer selbst, obwohl sie in OSM auf einer *anderen* Strasse
liegt). Ebenso stoert eine Gesamtzahl, die nur zufaellig stimmt, weil
Phantomnummern echte Nummern ersetzen.

Fuer jede Wellen-Notiz mit
  * einer BBox (``bbox=lon,lat,lon,lat`` oder "Strassenbbox a/b bis c/d"),
  * einem Strassennamen (``addr:street=X`` oder ``Straße "X"``),
  * einer Aufzaehlung (``(1, 2, 3, ...)`` im Umfeld von "verschiedene
    Hausnummern")
wird die Aufzaehlung live nachgerechnet. Bereichs- und Sammelnotationen
werden **aufgeloest**: "35-37" = 35/36/37, "6;8" = 6/8, "2a-b" = 2a/2b,
"1, 1A" = 1/1A.

Aufruf:  python3 scripts/release/check_house_number_lists.py <basis-commit> [scope-dir]
Exitcode 0 = bestanden, 1 = Beanstandungen.
"""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from osm_street_scan import api  # noqa: E402

CACHE = {}


def elements(bbox):
    if bbox not in CACHE:
        try:
            data = json.loads(api(
                f"https://api.openstreetmap.org/api/0.6/map.json?bbox={bbox}"))
            CACHE[bbox] = data.get("elements", [])
        except Exception:
            CACHE[bbox] = None
    return CACHE[bbox]


def expand(token: str):
    """'35-37' -> {'35','36','37'}; '6;8' -> {'6','8'}; '2a-b' -> {'2a','2b'}."""
    token = token.strip()
    if not token:
        return set()
    m = re.fullmatch(r"([0-9]+[a-zA-Z]?)\s*[-–]\s*([0-9]+[a-zA-Z]?)", token)
    if m:
        a, b = m.group(1), m.group(2)
        na = re.match(r"([0-9]+)", a).group(1)
        nb = re.match(r"([0-9]+)", b).group(1)
        if len(na) == len(nb) and na.isdigit() and nb.isdigit():
            return {f"{n}{a[len(na):]}" for n in range(int(na), int(nb) + 1)}
        return {a, b}
    if ";" in token:
        return set().union(*(expand(p) for p in token.split(";")))
    if "/" in token:
        return set().union(*(expand(p) for p in token.split("/")))
    return {token}


def parse_list(raw: str):
    """Eine Aufzaehlung in atomare Nummern aufloesen."""
    raw = raw.replace(" und ", ", ").replace(" sowie ", ", ")
    out = set()
    for part in raw.split(","):
        part = part.strip().strip("„”\"'")
        if not part:
            continue
        if re.fullmatch(r"[0-9]+[a-zA-Z]?", part):
            out.add(part)
        else:
            out |= expand(part)
    return out


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    base = sys.argv[1]
    scope = Path(sys.argv[2]) if len(sys.argv) > 2 else Path("dossiers")

    import subprocess
    changed = subprocess.run(
        ["git", "diff", "--name-only", base, "--", str(scope)],
        capture_output=True, text=True, check=True).stdout.split()

    problems = []
    checked = 0
    for rel in changed:
        path = Path(rel)
        text = path.read_text(encoding="utf-8")
        if "## Timeline" not in text:
            continue
        tail = text.split("## Timeline", 1)[1]
        # nur die neuen Abschnitte
        secs = re.split(r"(?m)^(### .*)$", tail)
        wave = "".join(secs[-2:]) if len(secs) >= 3 else tail
        for note in wave.split("\n- [Recherche"):
            bbox = None
            m = re.search(r"bbox=([0-9.]+),([0-9.]+),([0-9.]+),([0-9.]+)", note)
            if m:
                bbox = ",".join(m.groups())
            else:
                m = re.search(r"Strassenbbox \*\*([0-9.]+)/([0-9.]+) bis ([0-9.]+)/([0-9.]+)\*\*", note)
                if m:
                    bbox = ",".join(m.groups())
            if not bbox:
                continue
            street = None
            for pat in (r"`addr:street`\s*(?:=|gleich)\s*„?([^`„”\n]+?)“?(?=[`\s,.;]|$)",
                        r"Straße\s*[„\"]([^„\"]+)[”\"]"):
                sm = re.search(pat, note)
                if sm:
                    street = sm.group(1).strip().rstrip(",.;")
                    break
            if not street:
                continue
            lm = re.search(r"verschiedene Hausnummern(?:\*\*)?\s*\(([^)]*)\)", note)
            if not lm:
                continue
            claimed = parse_list(lm.group(1))
            if not claimed:
                continue
            els = elements(bbox)
            if els is None:
                problems.append(f"{rel}: BBox {bbox} nicht abrufbar")
                continue
            live = set()
            for e in els:
                tg = e.get("tags") or {}
                if tg.get("addr:street") == street and tg.get("addr:housenumber"):
                    live |= expand(tg["addr:housenumber"])   # Rohwerte koennen Bereiche tragen
            if not live:
                continue
            checked += 1
            extra = sorted(claimed - live)
            missing = sorted(live - claimed)
            if extra:
                problems.append(f"{rel}: Straße {street!r} — nicht existierende Nummern in der "
                                f"Aufzählung: {', '.join(extra)}")
            if missing:
                problems.append(f"{rel}: Straße {street!r} — live vorhandene, nicht genannte "
                                f"Nummern: {', '.join(missing)}")

    print(f"Gepruefte Aufzaehlungen: {checked}")
    if problems:
        print(f"PROBLEME: {len(problems)}")
        for p in problems:
            print("  -", p)
        return 1
    print("Hausnummern-Aufzaehlungen bestanden: alle genannten Nummern existieren live, "
          "alle live vorhandenen sind genannt")
    return 0


if __name__ == "__main__":
    sys.exit(main())
