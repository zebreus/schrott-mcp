#!/usr/bin/env python3
"""Baut die conflicted Dossiers aus drei Quellen korrekt zusammen.

Beide Wellen haben je eine datierte Sektion am Timeline-Ende ergaenzt, die
Release-Linie bringt eine dritte (die Welle vom 04.00). Die Hausregel erlaubt
**genau eine** neue Sektion je Dossier - also werden alle neuen Notizen in
**eine** Sektion zusammengefuehrt; die vorhandenen Sektionen der Basis bleiben
unveraendert.

Aufruf: python3 scripts/release/merge_two_wave_notes.py <basis> <zweig1> <zweig2> <datei> ...
"""
import json
import re
import subprocess
import sys
from pathlib import Path

CERT_RE = re.compile(r"^certifications: (.*)$", re.M)
SEC_RE = re.compile(r"^### .*$", re.M)


def show(rev, path):
    return subprocess.run(["git", "show", f"{rev}:{path}"], capture_output=True,
                          text=True, check=True).stdout


def parts(text):
    """(frontmatter, ueberblick, timeline, kopf) - der Timeline ab '## Timeline'."""
    m = re.search(r"^## Timeline\s*$", text, re.M)
    if not m:
        return text, "", "", ""
    head, timeline = text[:m.start()], text[m.start():]
    fm = head.split("---", 2)[1] if head.startswith("---") else ""
    ub = re.search(r"^## Überblick\s*\n(.*)", head, re.M | re.S)
    name = re.search(r"^# (.+)$", head, re.M)
    return fm, (ub.group(1) if ub else ""), timeline, (name.group(1) if name else "")


def sections(timeline):
    out = []
    idx = [m.start() for m in SEC_RE.finditer(timeline)]
    for i, s in enumerate(idx):
        e = idx[i + 1] if i + 1 < len(idx) else len(timeline)
        out.append((SEC_RE.search(timeline[s:]).group(0), timeline[s:e].rstrip("\n")))
    return out


def notes_of(block):
    """(bullet, fortsetzung) je Notiz eines Blocks."""
    out, cur = [], None
    for line in block.splitlines():
        if line.startswith("- ["):
            cur = [line]
            out.append(cur)
        elif cur is not None and line.startswith("  "):
            cur.append(line)
        elif cur is not None and line.strip() == "":
            continue
        else:
            cur = None
    return ["\n".join(b) for b in out]


def main() -> int:
    base, b1, b2 = sys.argv[1], sys.argv[2], sys.argv[3]
    for path in sys.argv[4:]:
        t_base, t_b1, t_b2 = show(base, path), show(b1, path), show(b2, path)
        fm_base, ub_base, tl_base, name = parts(t_base)
        fm_b1, ub_b1, tl_b1, _ = parts(t_b1)
        fm_b2, ub_b2, tl_b2, _ = parts(t_b2)

        base_secs = sections(tl_base)
        base_titel = [t for t, _ in base_secs]
        base_notizen = {t: notes_of(b) for t, b in base_secs}

        # neue Notizen je Zweig (Titel, die es in der Basis nicht gibt)
        neue = []
        for tl in (tl_b1, tl_b2):
            for titel, block in sections(tl):
                if titel not in base_titel:
                    neue.append((titel, notes_of(block)))
                else:
                    for n in notes_of(block):
                        if n not in base_notizen[titel]:
                            neue.append((titel, [n]))
        if not neue:
            print(f"  {path}: nichts Neues")
            continue
        merged_titel = neue[0][0]
        merged_notizen = []
        for _, notes in neue:
            for n in notes:
                if n not in merged_notizen:
                    merged_notizen.append(n)

        # Timeline: Basis-Sektionen + eine neue Sektion am Ende
        out = []
        for titel, block in base_secs:
            out.append(block)
        if merged_notizen:
            out.append(merged_titel + "\n\n" + "\n".join(merged_notizen))
        timeline = "## Timeline\n\n" + "\n\n".join(out) + "\n"

        # Frontmatter: die juengste Fassung; die Klammerbalance bleibt erhalten
        fm = fm_b2
        m2 = CERT_RE.search(fm)
        if m2:
            raw = m2.group(1)
            inner = raw[1:-1]
            try:
                items = json.loads(inner)
            except json.JSONDecodeError:
                items = None
            if items is not None:
                # Elementgrenzen im Text suchen und die fehlende Klammer dort ergaenzen
                spans, pos, end_all = [], 1, len(inner) - 1
                while pos < end_all:
                    ch = inner[pos]
                    if ch in " \t\n,":
                        pos += 1
                        continue
                    if ch != '"':
                        break
                    j = pos + 1
                    while j < end_all:
                        if inner[j] == "\\":
                            j += 2
                            continue
                        if inner[j] == '"':
                            break
                        j += 1
                    spans.append((pos, j + 1))
                    pos = j + 1
                for i, (a, b) in enumerate(spans):
                    if i < len(items) and items[i].count("(") - items[i].count(")") == 1:
                        inner = inner[:b] + ")" + inner[b:]
                        break
                fm = fm[:m2.start()] + "certifications: " + raw[0] + inner + raw[-1] + fm[m2.end():]

        # Ueberblick: die kuratierte Fassung bevorzugen
        ub = ub_b2 if ub_b2.strip() and "_Noch kein Überblick" not in ub_b2 else ub_base
        if not ub.strip():
            ub = ub_base

        doc = ("---\n" + fm.rstrip("\n") + "\n---\n\n# " + name + "\n\n## Überblick\n"
               + ub.rstrip("\n") + "\n\n" + timeline)
        Path(path).write_text(doc, encoding="utf-8")
        print(f"  {path}: {len(base_secs)} Basis-Sektionen + 1 neue Sektion "
              f"'{merged_titel}' mit {len(merged_notizen)} Notiz(en)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
