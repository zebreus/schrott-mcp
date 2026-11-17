#!/usr/bin/env python3
"""Extrahiert pro Dossier die neue Wellen-Sektion als Kurzfassung.

Aufruf:  python3 scripts/release/wave_sections.py <basis-commit> [--dossier SLUG ...]

Liest ausschliesslich das Repo (git show + Dateien). Produktionsdatenbanken
werden nicht angefasst. Die Ausgabe ist Markdown-tauglich: je Dossier eine
Zeile mit Slug, Bundesland, Frontmatter-Aenderung und dem ersten Satz der
neuen Notiz.
"""
import argparse
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FM = re.compile(r"^---\n(.*?)\n---\n", re.S)


def frontmatter(text):
    m = FM.match(text)
    out = {}
    if not m:
        return out
    for line in m.group(1).split("\n"):
        mm = re.match(r"^([a-z_]+):\s*(.*)$", line)
        if mm:
            out[mm.group(1)] = mm.group(2).strip().strip("'\"")
    return out


def last_section(text):
    """Die letzte datierte Sektion und ihre Bullets."""
    parts = re.split(r"^### ", text, flags=re.M)
    best = None
    for part in parts:
        head = part.split("\n", 1)[0].strip()
        if re.search(r"\d{2}\.\d{2}\.\d{4}", head) and "Importiert" not in head:
            best = part
    if best is None:
        return None, []
    bullets = [l for l in best.split("\n")[1:] if l.startswith("- ")]
    return best.split("\n", 1)[0].strip(), bullets


def first_sentence(text, limit=300):
    """Der erste aussagekräftige Satz aus einer Notiz (ohne Quellenliste)."""
    body = re.sub(r"Quelle\(n\):.*$", "", text, flags=re.S)
    body = re.sub(r"\s+", " ", body).strip()
    body = body.lstrip("- [").split("]: ", 1)
    body = body[-1]
    # den Notiz-Doppelpunkt am Anfang entfernen ("Recherche 17.11.2026: ...")
    body = re.sub(r"^(Recherche|Geokodierung|Korrektur)\s+\d{2}\.\d{2}\.\d{4}\s*:?\s*", "", body)
    # der erste fett markierte Kernsatz ist meist die Kernaussage
    bold = re.findall(r"\*\*([^*]{20,400}?)\*\*", body)
    if bold:
        return bold[0].strip()[:limit]
    return body[:limit]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("base")
    ap.add_argument("--dossier", action="append", default=[])
    ap.add_argument("--limit", type=int, default=300)
    args = ap.parse_args()

    files = subprocess.run(
        ["git", "diff", "--name-only", args.base, "--", "dossiers/"],
        capture_output=True, text=True, check=True).stdout.split()
    if args.dossier:
        wanted = set(args.dossier)
        files = [f for f in files if f.split("/")[-1][:-3] in wanted]

    for rel in files:
        slug = rel.split("/")[-1][:-3]
        new = (ROOT / rel).read_text(encoding="utf-8")
        old = subprocess.run(["git", "show", f"{args.base}:{rel}"],
                             capture_output=True, text=True, check=True).stdout
        ofm, nfm = frontmatter(old), frontmatter(new)
        changed = [k for k in set(ofm) | set(nfm) if ofm.get(k) != nfm.get(k)]
        head, bullets = last_section(new)
        note = first_sentence(bullets[0], args.limit) if bullets else ""
        print(f"### `{slug}`")
        print(f"- **Bundesland** {nfm.get('state', '?')} · **Abschnitt** {head or '-'}")
        print(f"- **Frontmatter**: {', '.join(sorted(changed)) if changed else 'unveraendert'}")
        print(f"- **Kernaussage**: {note}")
        print()


if __name__ == "__main__":
    main()
