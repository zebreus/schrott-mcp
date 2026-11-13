#!/usr/bin/env python3
"""Sammelt die Kennzahlen einer Welle fuer den Wellen-Bericht.

Aufruf:  python3 scripts/release/wave_stats.py <basis-commit>
Liest ausschliesslich das Repo (git show + Dateien). Produktionsdatenbanken
werden nicht angefasst.
"""
import json
import re
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

BASE = sys.argv[1] if len(sys.argv) > 1 else "HEAD"
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


def main():
    files = subprocess.run(
        ["git", "diff", "--name-only", BASE, "--", "dossiers/"],
        capture_output=True, text=True, check=True).stdout.split()
    print(f"Basis: {BASE}")
    print(f"Geaenderte Dossiers: {len(files)}")

    states = Counter()
    field_changes = Counter()
    with_fm = []
    without_fm = []
    sections = Counter()
    new_notes = 0
    text_added = 0
    text_removed = 0
    fundstellen = Counter()
    cert_counts = Counter()
    geo_set = 0
    geo_cleared = 0
    bullets = 0

    for rel in files:
        old_text = subprocess.run(["git", "show", f"{BASE}:{rel}"],
                                  capture_output=True, text=True, check=True).stdout
        new_text = (ROOT / rel).read_text(encoding="utf-8")
        old, new = frontmatter(old_text), frontmatter(new_text)
        states[new.get("state", "?")] += 1
        changed = [k for k in set(old) | set(new) if old.get(k) != new.get(k)]
        if changed:
            with_fm.append(rel)
            for k in changed:
                field_changes[k] += 1
                if k in ("lat", "lon"):
                    if new.get(k):
                        geo_set += 1
                    else:
                        geo_cleared += 1
        else:
            without_fm.append(rel)
        for h in re.findall(r"^### (.+)$", new_text, re.M):
            sections[h.strip()] += 1
        # neue Notiz-Bloecke
        old_bullets = len(re.findall(r"^- ", old_text, re.M))
        new_bullets = len(re.findall(r"^- ", new_text, re.M))
        bullets += new_bullets - old_bullets
        diff = subprocess.run(
            ["git", "diff", "--unified=0", BASE, "--", rel],
            capture_output=True, text=True, check=True).stdout.splitlines()
        added = [l[1:] for l in diff if l.startswith("+") and not l.startswith("+++")]
        removed = [l[1:] for l in diff if l.startswith("-") and not l.startswith("---")]
        text_added += sum(len(l) for l in added)
        text_removed += sum(len(l) for l in removed)
        for l in added:
            for d in re.findall(r"\d{2}\.\d{2}\.\d{4}", l):
                fundstellen[d] += 1
        # certifications-Zaehlung
        certs = new.get("certifications", "")
        try:
            arr = json.loads(certs) if certs else []
        except json.JSONDecodeError:
            arr = []
        for c in arr:
            if "kein Eintrag" in c or "kein" in c.lower() and "register" in c.lower():
                cert_counts["Negativbefund"] += 1
            else:
                cert_counts["Zertifikat"] += 1

    print("\nBundeslaender:", dict(states.most_common()))
    print(f"Mit Frontmatter-Aenderung: {len(with_fm)} | nur Notiz: {len(without_fm)}")
    print("\nGeaenderte Frontmatter-Felder:")
    for k, v in field_changes.most_common():
        print(f"  {k:16s} {v}")
    print(f"\nlat/lon gesetzt: {geo_set} | zurueckgenommen (clear): {geo_cleared}")
    print(f"Neue Notiz-Bullets: {bullets}")
    print(f"Text: +{text_added} / -{text_removed} Zeichen")
    print("\nDatums-Fundstellen in neuen Zeilen:")
    for d, c in fundstellen.most_common():
        print(f"  {d}: {c}")
    print("\ncertifications-Eintraege:", dict(cert_counts))
    print("\nNeue Sektionstitel:")
    for h, c in sections.most_common(12):
        print(f"  {c:3d}  {h}")


if __name__ == "__main__":
    main()
