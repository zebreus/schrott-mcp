#!/usr/bin/env python3
"""Zieht die Wellen-Slugs und legt sie in **bundesland-disjunkte** Shards.

Die Welle besteht aus 60 frischen Händler-Slugs aus dem Pool ✓. Die
Produktionsdatenbank wird dabei **nur gelesen** ✓ - mit
``file:...?mode=ro`` ✓ und ``PRAGMA query_only=ON`` ✓.

Pool (wie in `prompts/tiefenrecherche-welle.md` ✓):

    SELECT slug, state FROM traders WHERE website='' OR street='';

Die Slugs werden in **3 Shards** zu je 20 verteilt ✓. Die Shards sind
**bundesland-disjunkt** ✓: jeder Bundesstaat kommt mit seinen bearbeiteten
Slugs in **genau einem** Shard ✓. Damit bearbeiten drei Agenten nie denselben
Bundesstaat ✓ und die Aussagen je Bundesstaat sind eindeutig ✓.

Ein Bundesstaat, der mehr Slugs im Pool hat als ein Shard fasst, wird
teilweise genommen ✓ - aber **nur in einem** Shard ✓.

Aufruf:
    python3 scripts/release/draw_wave_slugs.py [--anzahl 60] [--shards 3]
                                        [--pro-shard 20] [--seed 42]
                                        [--ausschluss <datei> ...]
                                        [--ausgabe /tmp/opencode/welle.json]

``--ausschluss`` nimmt Dateien mit einer Slug je Zeile ✓ - damit kann der
Owner Dossiers ausschliessen, die gerade in einer anderen Welle bearbeitet
werden ✓.

Exitcode 0 = die Aufteilung ist gefunden ✓, 1 = nicht ✓.
"""
import argparse
import json
import random
import sqlite3
import sys
from collections import defaultdict
from pathlib import Path

PROD = "/var/lib/schrott-mcp/public.db"


def pool_lesen(db_path, mit_null):
    """Die Pool-Slugs mit ihrem Bundesstaat - **nur lesend** ✓."""
    uri = f"file:{db_path}?mode=ro"
    con = sqlite3.connect(uri, uri=True)
    try:
        con.execute("PRAGMA query_only=ON")
        if mit_null:
            where = "website='' OR street IS NULL OR street=''"
        else:
            where = "website='' OR street=''"
        rows = con.execute(
            f"SELECT slug, state FROM traders WHERE {where} "
            "ORDER BY slug").fetchall()
    finally:
        con.close()
    return rows


def aufteilen(by_state, anzahl_shards, pro_shard, rng):
    """Sucht eine Aufteilung: je Shard genau ``pro_shard`` Slugs ✓.

    Jeder Bundesstaat kommt mit seinen bearbeiteten Slugs in genau einen
    Shard ✓. Die Suche ist eine Tiefensuche über die Bundesstaaten ✓ mit
    Memoisierung über die Restgroessen der Shards ✓.
    """
    staaten = sorted(by_state, key=lambda s: (-len(by_state[s]), s))
    # die Auswahl je Bundesstaat ist zufaellig gemischt, damit die Ziehung
    # frisch ist ✓ - die Reihenfolge der Staeten bleibt aber stabil ✓
    auswahl = {s: rng.sample(by_state[s], len(by_state[s])) for s in staaten}
    memo = {}

    def suchen(i, rest):
        """Liefert die Zuordnung [(shard, staat, anzahl), ...] ✓."""
        if all(r == 0 for r in rest):
            return []
        if i == len(staaten):
            return None
        key = (i, rest)
        if key in memo:
            return memo[key]
        verfuegbar = len(auswahl[staaten[i]])
        for k in range(anzahl_shards):
            if rest[k] == 0:
                continue
            max_n = min(rest[k], verfuegbar)
            for n in range(max_n, 0, -1):
                neu = list(rest)
                neu[k] -= n
                rest_erg = suchen(i + 1, tuple(neu))
                if rest_erg is not None:
                    memo[key] = [(k, staaten[i], n)] + rest_erg
                    return memo[key]
        memo[key] = None
        return None

    return suchen(0, tuple([pro_shard] * anzahl_shards))


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--anzahl", type=int, default=60, help="Slugs gesamt")
    ap.add_argument("--shards", type=int, default=3, help="Anzahl Shards")
    ap.add_argument("--pro-shard", type=int, default=20, help="Slugs je Shard")
    ap.add_argument("--seed", type=int, default=42, help="Zufalls-Seed")
    ap.add_argument("--db", default=PROD, help="Produktionsdatenbank")
    ap.add_argument("--mit-null-street", action="store_true",
                    help="auch Slugs mit leerem/NULL street nehmen")
    ap.add_argument("--ausschluss-scope", default=None,
                    help="Basis-Commit: Dossiers, die seit diesem Commit im "
                         "Release-Scope geaendert wurden, werden ausgeschlossen")
    ap.add_argument("--ausschluss", action="append", default=[],
                    help="Datei mit auszuschliessenden Slugs")
    ap.add_argument("--auswahl", type=Path,
                    help="Datei mit der Slug-Liste (eine Slug je Zeile); "
                         "ueberschreibt den Pool")
    ap.add_argument("--ausgabe", default="/tmp/opencode/welle.json")
    args = ap.parse_args()

    if args.shards * args.pro_shard != args.anzahl:
        print(f"shards ({args.shards}) * pro_shard ({args.pro_shard}) "
              f"!= anzahl ({args.anzahl}) ✗")
        return 1

    if args.auswahl:
        slugs = []
        for line in args.auswahl.read_text(encoding="utf-8").splitlines():
            s = line.strip()
            if s and not s.startswith("#"):
                slugs.append(s)
        if len(slugs) != args.anzahl:
            print(f"Auswahl hat {len(slugs)} Slugs, erwartet {args.anzahl} ✗")
            return 1
        print(f"Auswahl aus {args.auswahl}: {len(slugs)} Slugs ✓")
        by_state = defaultdict(list)
        for slug in slugs:
            by_state[slug.split("-", 1)[0].upper()].append(slug)
        print(f"Bundesstaaten in der Auswahl: {len(by_state)}")
        # dieselbe Aufteilung wie im Pool-Fall: jeder Bundesstaat mit allen
        # seinen Slugs in genau einen Shard ✓
        rng = random.Random(args.seed)
        shards = None
        for _ in range(200):
            plan = aufteilen(by_state, args.shards, args.pro_shard, rng)
            if plan:
                shards = [[] for _ in range(args.shards)]
                zustand = {s: rng.sample(by_state[s], len(by_state[s])) for s in by_state}
                for k, staat, n in plan:
                    shards[k].extend(zustand[staat][:n])
                break
        if not shards:
            print("Keine bundesland-disjunkte Aufteilung gefunden ✗")
            return 1
        for i, sh in enumerate(shards, 1):
            staaten = sorted({s.split("-", 1)[0].upper() for s in sh})
            print(f"  Shard {i}: {len(sh)} Slugs | {', '.join(staaten)}")
        payload = {
            "seed": args.seed,
            "pool_groesse": len(slugs),
            "pro_shard": args.pro_shard,
            "shards": [
                {"nr": i, "slugs": sh,
                 "bundesstaaten": sorted({s.split("-", 1)[0].upper() for s in sh})}
                for i, sh in enumerate(shards, 1)],
        }
        Path(args.ausgabe).write_text(
            json.dumps(payload, ensure_ascii=False, indent=1), encoding="utf-8")
        print(f"Gespeichert: {args.ausgabe} ✓")
        return 0

    ausgeschlossen = set()
    for pfad in args.ausschluss:
        p = Path(pfad)
        if not p.exists():
            print(f"  (Ausschlussdatei fehlt: {pfad} ✓ - wird uebersprungen ✓)")
            continue
        for line in p.read_text(encoding="utf-8").splitlines():
            s = line.strip()
            if s and not s.startswith("#"):
                ausgeschlossen.add(s)

    if args.ausschluss_scope:
        # Die Dossiers, die eine fruehere Welle **in diesem Release-Scope**
        # schon bearbeitet hat, werden ausgeschlossen. Sonst zieht die Welle
        # Dossiers, die eine andere Welle desselben Releases schon
        # durchgearbeitet hat - der Pool enthaelt jedes Dossier ohne
        # Website oder Adresse, und eine fruehere Welle hat viele davon so
        # gelassen.
        import subprocess
        geaendert = subprocess.run(
            ["git", "diff", "--name-only", args.ausschluss_scope, "--", "dossiers/"],
            capture_output=True, text=True, check=True).stdout.split()
        for rel in geaendert:
            ausgeschlossen.add(Path(rel).stem)
        print(f"Ausgeschlossen (Scope seit {args.ausschluss_scope}): "
              f"{len(geaendert)} Dossiers ✓")

    zeilen = pool_lesen(args.db, args.mit_null_street)
    pool = [(s, st or "?") for s, st in zeilen if s not in ausgeschlossen]
    print(f"Pool (website oder street leer): {len(zeilen)} | "
          f"nach Ausschluss: {len(pool)}")

    by_state = defaultdict(list)
    for slug, state in pool:
        by_state[state].append(slug)
    print(f"Bundesstaaten im Pool: {len(by_state)}")

    rng = random.Random(args.seed)
    for _ in range(50):
        plan = aufteilen(by_state, args.shards, args.pro_shard, rng)
        if plan:
            shards = [[] for _ in range(args.shards)]
            zustand = {s: rng.sample(by_state[s], len(by_state[s]))
                       for s in by_state}
            for k, staat, n in plan:
                shards[k].extend(zustand[staat][:n])
            for i, sh in enumerate(shards, 1):
                staaten = sorted({st for s, st in pool if s in sh})
                print(f"  Shard {i}: {len(sh)} Slugs | {len(set(sh))} "
                      f"verschiedene | Bundesstaaten: {', '.join(staaten)}")
            payload = {
                "seed": args.seed,
                "pool_groesse": len(pool),
                "pro_shard": args.pro_shard,
                "shards": [
                    {"nr": i, "slugs": sh,
                     "bundesstaaten": sorted({st for s, st in pool if s in sh})}
                    for i, sh in enumerate(shards, 1)],
            }
            Path(args.ausgabe).write_text(
                json.dumps(payload, ensure_ascii=False, indent=1),
                encoding="utf-8")
            print(f"Gespeichert: {args.ausgabe} ✓")
            return 0
    print("Keine Aufteilung gefunden - Pool vergroessern oder Seed aendern ✗")
    return 1


if __name__ == "__main__":
    sys.exit(main())
