#!/usr/bin/env python3
"""Haengt eine neue Recherche-Sektion ans Timeline-Ende eines Dossiers.

Nur lokale Dateibearbeitung im Arbeitsbaum, keine Datenbank- oder
Produktionsaenderung. Format:

    ### Recherche 09.11.2026

    - [Recherche 09.11.2026: ...]
      Quelle(n): ... (Abruf 09.11.2026).]

Aufruf:
    python3 tools/append_note.py <slug> <date> <notiz-datei>
"""
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main():
    slug, date, note_file = sys.argv[1], sys.argv[2], sys.argv[3]
    path = ROOT / "dossiers" / "he" / f"{slug}.md"
    if not path.exists():
        matches = list((ROOT / "dossiers").rglob(f"{slug}.md"))
        if not matches:
            raise SystemExit(f"Dossier {slug} nicht gefunden")
        path = matches[0]
    text = path.read_text(encoding="utf-8")
    body = Path(note_file).read_text(encoding="utf-8").rstrip("\n")
    section = f"\n\n### Recherche {date}\n\n{body}\n"
    if f"### Recherche {date}" in text:
        raise SystemExit(f"Sektion 'Recherche {date}' existiert bereits in {path}")
    path.write_text(text.rstrip("\n") + section, encoding="utf-8")
    print(f"{path}: Sektion 'Recherche {date}' angehaengt ({len(body)} Zeichen)")


if __name__ == "__main__":
    main()
