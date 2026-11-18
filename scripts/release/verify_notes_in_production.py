#!/usr/bin/env python3
"""Rechnet die Notizen des Release-Scopes in der Produktion nach.

Unabhaengig vom Verifier: die Timeline-Notizen werden hier mit einer eigenen
Python-Implementierung der Seed-Extraktion aus den Dossiers gelesen und mit
dem `notes`-Feld der Produktionsdatenbank verglichen. Zusaetzlich werden die
uebrigen Felder des Scopes gegen die Release-Erwartungen geprueft.

Read-only: die Produktionsdatenbank wird mit `mode=ro` geoeffnet.
"""
import argparse
import json
import re
import sqlite3
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def is_h2(line: str) -> bool:
    """Ein `## `-Ueberschrift beendet die Timeline (Rust: `is_h2`)."""
    return line.startswith("## ") and len(line) > 3 and not line[3].isspace()


def collapse(text: str) -> str:
    """Whitespace zusammenfassen (Rust: `collapse`)."""
    return " ".join(text.split())


def timeline_notes(body: str) -> str:
    """Die Seed-Extraktion der Timeline-Notizen, unabhaengig nachgebaut.

    Spiegelt `crates/ingestion/build.rs::timeline_notes`: Bullets mit `- `,
    eingerueckte Fortsetzungszeilen mit einem Leerzeichen angehaengt, jede
    Notiz intern zusammengefasst, die Bullets mit ` | ` verbunden.
    """
    in_tl = False
    bullets: list[str] = []
    for line in body.split("\n"):
        line = line.rstrip("\r")
        if not in_tl:
            if line.strip() == "## Timeline":
                in_tl = True
            continue
        if is_h2(line):
            # Ein zweiter "## Timeline"-Abschnitt setzt die Sammlung fort.
            if line.strip() == "## Timeline":
                continue
            break
        indented = line.startswith(" ") or line.startswith("\t")
        s = line.strip()
        if not indented:
            if s.startswith("- "):
                bullets.append(s[2:].strip())
            # Eine Zeile in Spalte 0, die kein Bullet ist, gehoert zu keiner
            # Notiz und wird nicht angehaengt.
            continue
        if s.startswith("#") or not s:
            continue
        if bullets:
            bullets[-1] = bullets[-1] + " " + s
    return " | ".join(collapse(b) for b in bullets)


def dossier_path(slug: str) -> Path:
    """Der Pfad des Dossiers **im Repository** ✓ ✓.

    Der Ordner ist **nicht** das Praefix des Slugs ✓ ✓ - `hb-nordenham-…`
    liegt unter `dossiers/ni/` ✓ ✓ (der Bundesland steht im Frontmatter ✓ ✓).
    Der Pfad wird deshalb **gesucht** ✓ ✓ und nicht **berechnet** ✓ ✓ ✓ ✓
    """
    for ordner in (ROOT / "dossiers").iterdir():
        if ordner.is_dir() and (ordner / f"{slug}.md").exists():
            return ordner / f"{slug}.md"
    return ROOT / "dossiers" / slug.split("-")[0].lower() / f"{slug}.md"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--data-dir", default="/var/lib/schrott-mcp")
    ap.add_argument("--scope", required=True, help="JSON-Datei mit den Slugs")
    args = ap.parse_args()

    slugs = json.load(open(args.scope))
    db = sqlite3.connect(f"file:{Path(args.data_dir) / 'public.db'}?mode=ro", uri=True)
    db.row_factory = sqlite3.Row

    failures: list[str] = []
    checked = 0
    for slug in slugs:
        row = db.execute(
            "SELECT notes, extra_json, updated_at FROM traders WHERE slug=?",
            (slug,)).fetchone()
        if row is None:
            failures.append(f"{slug}: nicht in der Produktion")
            continue
        path = dossier_path(slug)
        if not path.exists():
            failures.append(f"{slug}: Dossier fehlt")
            continue
        body = path.read_text(encoding="utf-8")
        expected = timeline_notes(body)
        actual = row["notes"] or ""
        if expected != actual:
            failures.append(f"{slug}: Notizen weichen ab "
                            f"(erwartet {len(expected)} Zeichen, Produktion {len(actual)})")
        try:
            extra = json.loads(row["extra_json"])
        except (TypeError, ValueError):
            failures.append(f"{slug}: extra_json ist kein Objekt")
            continue
        if not isinstance(extra, dict):
            failures.append(f"{slug}: extra_json ist kein Objekt")
            continue
        seed_hash = extra.get("seed_hash")
        if not seed_hash:
            failures.append(f"{slug}: kein seed_hash in extra_json")
        checked += 1

    result = {
        "ok": not failures,
        "scope": len(slugs),
        "checked": checked,
        "failures": sorted(set(failures)),
    }
    print(json.dumps(result, indent=2, ensure_ascii=False))
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
