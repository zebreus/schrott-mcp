#!/usr/bin/env python3
"""Prueft die im Repository belegten Kennzahlen der Wellenberichte nach.

Jede Zahl, die ein Wellenbericht nennt, muss gegen ihre **genannte Basis**
nachrechenbar sein. Das Skript rechnet die Kennzahlen der Berichte der
Wellen 10:00, 12:00, 07:00, 08:30, 13:00 und der SN-Zweige neu und meldet
jede Abweichung.

Aufruf: python3 scripts/release/check_report_figures.py [ref]
Exitcode 0 = alle Kennzahlen stimmen ✓, 1 = mindestens eine Abweichung ✓.
"""
import re
import subprocess
import sys
from collections import Counter

ROOT = "."


def git(*args):
    return subprocess.run(["git", "-C", ROOT, *args],
                          capture_output=True, text=True, check=True).stdout


def dossier_text(ref, rel):
    return git("show", f"{ref}:{rel}")


def notes(text):
    """Die Timeline-Notizbloecke (Bullet + Fortsetzungszeilen)."""
    start = text.find("## Timeline")
    if start < 0:
        return []
    out, block = [], None
    for line in text[start:].split("\n"):
        if line.startswith("- "):
            if block:
                out.append(block)
            block = [line]
        elif block is not None and line.strip() and line[:1] in (" ", "\t"):
            block.append(line)
        elif block is not None:
            out.append(block)
            block = None
    if block:
        out.append(block)
    return ["\n".join(b) for b in out]


def sections(text):
    return [l for l in text.split("\n") if l.startswith("### ")]


def frontmatter(text):
    out = {}
    for line in text.split("\n"):
        if line.startswith("## "):
            break
        m = re.match(r"^([a-z_]+):\s*(.*)$", line)
        if m:
            out[m.group(1)] = m.group(2)
    return out


def scope(base, cand):
    return git("diff", "--name-only", base, cand, "--", "dossiers/").split()


def feldzaehlungen(base, cand):
    """Je Feld die Zahl der Dossiers mit geändertem Frontmatter."""
    zaehler = Counter()
    for rel in scope(base, cand):
        a = frontmatter(dossier_text(base, rel))
        b = frontmatter(dossier_text(cand, rel))
        for feld in set(a) | set(b):
            if a.get(feld) != b.get(feld):
                zaehler[feld] += 1
    return zaehler


def neue_notizen(base, cand):
    """Die neuen Notizbloecke je Dossier."""
    pro_dossier = {}
    for rel in scope(base, cand):
        alt = set(notes(dossier_text(base, rel)))
        neu = [n for n in notes(dossier_text(cand, rel)) if n not in alt]
        if neu:
            pro_dossier[rel] = neu
    return pro_dossier


def reichweite(base, cand):
    """Die Reichweiten-Tabelle: je Dossier Ankreuz / PDF / kein.

    Ein Dossier ohne neue Notiz zaehlt zu **kein** ✓ - so steht es in der
    Regel der Berichte ✓ ("kein PDF-Pruefvermerk" heisst: fuer dieses Dossier
    liegt kein geprueftes Zertifikats-PDF vor ✓).
    """
    ankreuz = pdf = kein = 0
    pro_dossier = neue_notizen(base, cand)
    for rel, neu in pro_dossier.items():
        if any("pdf_checkboxes" in n for n in neu):
            ankreuz += 1
        elif any("PDF" in n.split("\n")[0] for n in neu):
            pdf += 1
        else:
            kein += 1
    kein += len(scope(base, cand)) - len(pro_dossier)
    return ankreuz, pdf, kein


def sektionsverteilung(base, cand):
    zaehler = Counter()
    for rel in scope(base, cand):
        alt = set(sections(dossier_text(base, rel)))
        neu = [s for s in sections(dossier_text(cand, rel)) if s not in alt]
        zaehler[len(neu)] += 1
    return zaehler


def unausgeglichen(ref):
    """Die Dossiers mit einer unausgeglichenen Klammer im `certifications`-Feld."""
    import json
    n = 0
    for rel in git("ls-tree", "-r", "--name-only", ref, "dossiers/").split():
        text = dossier_text(ref, rel)
        m = re.search(r"^certifications:\s*(.*)$", text, re.M)
        if not m:
            continue
        raw = m.group(1).strip()
        if len(raw) >= 2 and raw[0] == "'" and raw[-1] == "'":
            raw = raw[1:-1]
        if not raw:
            continue
        try:
            els = json.loads(raw)
        except json.JSONDecodeError:
            continue
        if any(e.count("(") != e.count(")") for e in els):
            n += 1
    return n


def pruefe(bezeichnung, ist, soll):
    ok = ist == soll
    print(f"  {'OK  ' if ok else 'FEHL'} {bezeichnung}: {ist} (Bericht: {soll})")
    return ok


def main() -> int:
    print("=== 10:00-Bericht ===")
    ok = True
    ok &= pruefe("Wellen-Scope der Welle 10:00 (2e5059b0..74a36d7f)",
                 len(scope("2e5059b0", "74a36d7f")), 115)
    z10 = feldzaehlungen("2e5059b0", "74a36d7f")
    ok &= pruefe("10:00 geänderte certifications", z10["certifications"], 99)
    ok &= pruefe("Release-Scope 27ef26f8..20cf7fe",
                 len(scope("27ef26f8", "20cf7fe")), 256)
    a, p, k = reichweite("27ef26f8", "31178f3")
    ok &= pruefe("Reichweite 27ef26f8..31178f3", f"{a}/{p}/{k}", "66/41/90")
    a, p, k = reichweite("27ef26f8", "20cf7fe")
    ok &= pruefe("Reichweite 27ef26f8..20cf7fe", f"{a}/{p}/{k}", "68/45/143")
    v = sektionsverteilung("27ef26f8", "20cf7fe")
    ok &= pruefe("Sektionen 1/2/3/0 im Release-Scope",
                 f"{v[1]}/{v[2]}/{v[3]}/{v[0]}", "226/14/6/10")
    ok &= pruefe("unausgeglichene Klammern in 20cf7fe", unausgeglichen("20cf7fe"), 7)

    print("=== 12:00-Bericht ===")
    ok &= pruefe("Dossiers der 12:00-Welle (54d2983d^1..54d2983d)",
                 len(scope("54d2983d^1", "54d2983d")), 60)
    z12 = feldzaehlungen("54d2983d^1", "54d2983d")
    for feld, soll in (("description", 16), ("email", 9), ("certifications", 9),
                        ("phone", 8), ("street", 6), ("postcode", 6), ("name", 5),
                        ("website", 5), ("website_status", 5),
                        ("opening_hours", 4), ("status", 2), ("clear_fields", 1)):
        ok &= pruefe(f"12:00 {feld}", z12[feld], soll)
    ok &= pruefe("Wellen 10:00 und 12:00 (27ef26f8..31178f3)",
                 len(scope("27ef26f8", "31178f3")), 197)

    print("=== 07:00- und 08:30-Bericht ===")
    zaehler = feldzaehlungen("a44ee5c1", "2bc04406")
    ok &= pruefe("08:30 Dossiers", len(scope("a44ee5c1", "2bc04406")), 20)
    ok &= pruefe("08:30 phone", zaehler["phone"], 6)
    ok &= pruefe("08:30 description", zaehler["description"], 7)
    ok &= pruefe("08:30 certifications", zaehler["certifications"], 4)
    ok &= pruefe("08:30 lat", zaehler["lat"], 2)
    ok &= pruefe("08:30 lon", zaehler["lon"], 2)
    ok &= pruefe("08:30 name", zaehler["name"], 1)
    ok &= pruefe("08:30 status", zaehler["status"], 2)
    ok &= pruefe("08:30 street", zaehler["street"], 4)
    ok &= pruefe("08:30 postcode", zaehler["postcode"], 4)
    ok &= pruefe("08:30 email", zaehler["email"], 3)
    ok &= pruefe("08:30 opening_hours", zaehler["opening_hours"], 2)
    ok &= pruefe("08:30 website", zaehler["website"], 1)
    ok &= pruefe("08:30 website_status", zaehler["website_status"], 3)

    print("=== 13:00-Bericht ===")
    ok &= pruefe("Dossiers 20cf7fec..544da648", len(scope("20cf7fec", "544da648")), 60)

    print("=== SN-Zweige ===")
    ok &= pruefe("unausgeglichene Dossiers in der Basis 2e5059b0",
                 unausgeglichen("2e5059b0"), 43)

    print()
    print("Alle Kennzahlen stimmen ✓" if ok else "Es gibt Abweichungen ✗")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
