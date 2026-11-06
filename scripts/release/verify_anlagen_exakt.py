#!/usr/bin/env python3
"""Exakter Abgleich je **Anlage** gegen das Zertifikats-PDF.

Je Dossier aus dem Reviewer-Artefakt:
  * der Eintrag wird in seine **Zertifikatseintraege** zerlegt (jeder traegt
    seine eigene Vorgangsnummer);
  * je Eintrag wird das **passende** PDF ueber die Vorgangsnummer gesucht -
    ein Dossier kann mehrere Zertifikate fuehren, dann muss je Zertifikat
    sein eigenes PDF geprueft werden;
  * die Anlagenliste des Eintrags wird anlagenweise zerlegt. Es gibt zwei
    Schreibweisen:
      A) ``Sammeln und Befoerdern (Anlage 1, Kennummer X)`` - die Taetigkeit
         steht **vor** der Klammer, die Anlagennummer **in** der Klammer;
      B) ``Anlage 1 Sammeln und Befoerdern (Kennummer X)`` - die Anlagennummer
         steht **vor** der Taetigkeit;
  * verglichen wird: welche **angekreuzte** Taetigkeit des PDF-Bogens fehlt
    im Eintrag, welche dort steht ohne Ankreuzbeleg.

Der Ankreuzzustand steht nur im PDF-Contentstream
(``scripts/release/pdf_checkboxes.py``).

Aufruf: python3 scripts/release/verify_anlagen_exakt.py <strict_agg.json> [dossier ...]
"""
import importlib.util
import json
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("pc", ROOT / "scripts/release/pdf_checkboxes.py")
pc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(pc)

TAET = ["Sammeln", "Befördern", "Lagern", "Behandeln", "Verwerten", "Beseitigen", "Handeln", "Makeln"]
VG_RE = re.compile(r"(ZZ[A-Z]{2}\d{8,14})")


def pdf_boegen(pdf):
    """Je Anlage: nr, angekreuzte Taetigkeiten (2.1-2.8)."""
    data = open(pdf, "rb").read()
    out = []
    for stream in pc.raw_streams(data):
        texts, boxes = pc.parse_stream(stream)
        if not texts or not boxes:
            continue
        zeilen = {}
        for t in texts:
            zeilen.setdefault(round(t["y"], 1), []).append(t)
        nr = None
        felder = {}
        for y in sorted(zeilen, reverse=True):
            items = sorted(zeilen[y], key=lambda t: t["x"])
            line = " ".join(i["text"] for i in items).strip()
            m_head = re.match(r"^Anlage\s+(\d+)\s+zum Zertifikat mit der Nummer\b", line)
            if m_head:
                nr = int(m_head.group(1))
            m = pc.FELD_RE.match(line)
            if not m:
                continue
            cands = [b for b in boxes
                     if abs((b["y0"] + b["y1"]) / 2 - y) < 4 and b["x0"] > items[0]["x"]]
            cands.sort(key=lambda b: b["x0"])
            felder[m.group(1)] = (m.group(2), bool(cands and cands[0]["checked"]))
        if nr is None:
            continue
        out.append({"nr": nr, "taet": sorted({t for t, an in felder.values() if an})})
    return out


def anlagen_der_eintraege(el):
    """Je Anlage: (nr, taetigkeitstext) - beide Schreibweisen."""
    treffer = list(re.finditer(r"Anlage\s+(\d+)", el))
    out = []
    for i, m in enumerate(treffer):
        nr = int(m.group(1))
        vorher = el[:m.start()]
        # Schreibweise A: die Anlagennummer steht in einer Klammer
        if vorher.rstrip().endswith("("):
            ende = m.end()
            k = el.find(")", ende)
            if k >= 0:
                ende = k + 1
            # der Taetigkeitstext steht vor der Klammer
            start = vorher.rstrip().rfind("(")
            start = start + 1 if start >= 0 else 0
            # bis zum Trennzeichen der vorigen Klammer zurueck
            vor = el[:start].rstrip()
            trenner = max(vor.rfind("),"), vor.rfind(";"), vor.rfind(","))
            text = el[trenner + 1:start].strip(" ,;")
        else:
            # Schreibweise B: die Taetigkeit steht hinter der Anlagennummer
            rest = el[m.end():]
            ende = m.end()
            if i + 1 < len(treffer):
                ende = treffer[i + 1].start()
            s = rest.find(";")
            if s >= 0 and m.end() + s < ende:
                ende = m.end() + s
            text = el[m.end():ende]
        out.append((nr, text))
    return out


def taetigkeiten(text):
    return [t for t in TAET if re.search(rf"\b{t}\b", text)]


def pdf_vorgang(pdf):
    out = subprocess.run(["pdftotext", "-layout", pdf, "-"], capture_output=True, text=True).stdout
    return set(VG_RE.findall(out))


def main() -> int:
    pfad = sys.argv[1]
    nur = set(sys.argv[2:])
    agg = json.load(open(pfad))
    geprueft, abweichungen, ohne_pdf = 0, 0, 0
    for e in agg:
        if nur and e["dossier"].split("/")[-1] not in nur:
            continue
        pdf = os.path.join("/tmp/opencode", e["pdf"].lstrip("./"))
        text = (ROOT / e["dossier"]).read_text(encoding="utf-8")
        m = re.search(r"^certifications: (.*)$", text, re.M)
        raw = m.group(1)
        if raw.startswith("'") and raw.endswith("'"):
            raw = raw[1:-1]
        eintrag = json.loads(raw)
        # Kandidaten-PDFs: das des Reviewers + alle PDFs, die die Vorgangsnummer
        # des Zertifikatseintrags tragen (ein Dossier kann mehrere Zertifikate
        # fuehren - je Zertifikat muss sein eigenes PDF geprueft werden)
        kandidaten = [pdf]
        for datei in e.get("pdfs", []) or []:
            kandidaten.append(os.path.join("/tmp/opencode", str(datei).lstrip("./")))
        if os.environ.get("ALLE_PDFS"):
            for wurzel, _, dateien in os.walk("/tmp/opencode"):
                if "/.git" in wurzel:
                    continue
                for d in dateien:
                    if d.lower().endswith(".pdf"):
                        kandidaten.append(os.path.join(wurzel, d))
        pdf_vg = {p: pdf_vorgang(p) for p in kandidaten if os.path.exists(p)}
        for el in eintrag:
            vgs = set(VG_RE.findall(el))
            if not vgs:
                continue
            passend = [p for p, vg in pdf_vg.items() if vg & vgs]
            if not passend:
                ohne_pdf += 1
                print(f"  -- {e['dossier'].split('/')[-1]}: Zertifikat {sorted(vgs)} - kein PDF im Artefakt")
                continue
            zboegen = {b["nr"]: b["taet"] for b in pdf_boegen(passend[0])}
            for nr, text_taet in anlagen_der_eintraege(el):
                genannt = taetigkeiten(text_taet)
                soll = zboegen.get(nr)
                if soll is None or not genannt:
                    continue
                fehlt = [t for t in soll if t not in genannt]
                zuviel = [t for t in genannt if t not in soll]
                geprueft += 1
                if fehlt or zuviel:
                    abweichungen += 1
                    print(f"  !! {e['dossier'].split('/')[-1]} Anlage {nr} "
                          f"({passend[0].split('/')[-1]}): fehlt={fehlt or '-'} zuviel={zuviel or '-'}")
                    print(f"       PDF  : {', '.join(soll)}")
                    print(f"       Eintr: {', '.join(genannt)}  |  {text_taet.strip()[:130]}")
    print(f"\nAnlagen geprueft: {geprueft} | Abweichungen: {abweichungen} "
          f"| Zertifikate ohne PDF: {ohne_pdf}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
