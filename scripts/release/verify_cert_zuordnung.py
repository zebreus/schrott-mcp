#!/usr/bin/env python3
"""Prueft die beanstandeten Dossiers **zertifikatsgenau** gegen die PDFs.

Ein Dossier kann mehrere Zertifikate fuehren, und zu einem Zertifikat kann
es mehrere PDF-Dateien geben (je Register-Download eine Datei). Deshalb
wird hier **je Zertifikatseintrag** gesucht:

  * alle PDFs, deren Vorgangsnummer zum Eintrag passt;
  * daraus je Anlagennummer die **vereinheitlichte** Menge der angekreuzten
    Felder 2.1-2.8 (das ist die Schnittmenge - eine Anlage, die in einem
    PDF angekreuzt und in einem anderen nicht angekreuzt ist, wird als
    "nicht eindeutig" gemeldet und nicht als Fehler gewertet);
  * Vergleich mit dem Eintrag: welche angekreuzte Taetigkeit fehlt, welche
    dort ohne Ankreuzbeleg steht.

Der Ankreuzzustand steht nur im PDF-Contentstream
(``scripts/release/pdf_checkboxes.py``).

Aufruf: python3 scripts/release/verify_cert_zuordnung.py <strict_agg.json> [dossier ...]
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


def alle_pdfs():
    """Alle PDFs unter /tmp/opencode."""
    out = []
    for wurzel, _, dateien in os.walk("/tmp/opencode"):
        if "/.git" in wurzel:
            continue
        for d in dateien:
            if d.lower().endswith(".pdf"):
                out.append(os.path.join(wurzel, d))
    return sorted(out)


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
        if vorher.rstrip().endswith("("):
            ende = el.find(")", m.end())
            ende = ende + 1 if ende >= 0 else m.end()
            start = vorher.rstrip().rfind("(") + 1
            vor = el[:start].rstrip()
            trenner = max(vor.rfind("),"), vor.rfind(";"), vor.rfind(","))
            text = el[trenner + 1:start].strip(" ,;")
        else:
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


def main() -> int:
    pfad = sys.argv[1]
    nur = set(sys.argv[2:])
    agg = json.load(open(pfad))
    alle = alle_pdfs()
    vg_index = {}
    for p in alle:
        try:
            txt = subprocess.run(["pdftotext", "-layout", p, "-"],
                                 capture_output=True, text=True, timeout=60).stdout
        except Exception:
            continue
        vgs = set(VG_RE.findall(txt))
        if vgs:
            vg_index[p] = vgs
    geprueft, abweichungen, ohne = 0, 0, 0
    for e in agg:
        if nur and e["dossier"].split("/")[-1] not in nur:
            continue
        text = (ROOT / e["dossier"]).read_text(encoding="utf-8")
        m = re.search(r"^certifications: (.*)$", text, re.M)
        raw = m.group(1)
        if raw.startswith("'") and raw.endswith("'"):
            raw = raw[1:-1]
        eintrag = json.loads(raw)
        for el in eintrag:
            vgs = set(VG_RE.findall(el))
            if not vgs:
                continue
            passend = [p for p, v in vg_index.items() if v & vgs]
            if not passend:
                ohne += 1
                print(f"  -- {e['dossier'].split('/')[-1]}: Zertifikat {sorted(vgs)} - kein PDF")
                continue
            # je Anlagennummer: die angekreuzten Felder aller passenden PDFs
            bogen = {}
            uneindeutig = set()
            for p in passend:
                for b in pdf_boegen(p):
                    alt = bogen.get(b["nr"])
                    if alt is None:
                        bogen[b["nr"]] = set(b["taet"])
                    elif alt != set(b["taet"]):
                        uneindeutig.add(b["nr"])
            for nr, text_taet in anlagen_der_eintraege(el):
                genannt = [t for t in TAET if re.search(rf"\b{t}\b", text_taet)]
                soll = bogen.get(nr)
                if soll is None or not genannt:
                    continue
                if nr in uneindeutig:
                    print(f"  ?? {e['dossier'].split('/')[-1]} Anlage {nr}: nicht eindeutig "
                          f"({len(passend)} PDFs) - nicht gewertet")
                    continue
                fehlt = [t for t in sorted(soll) if t not in genannt]
                zuviel = [t for t in genannt if t not in soll]
                geprueft += 1
                if fehlt or zuviel:
                    abweichungen += 1
                    print(f"  !! {e['dossier'].split('/')[-1]} Anlage {nr} "
                          f"({len(passend)} PDF(s)): fehlt={fehlt or '-'} zuviel={zuviel or '-'}")
                    print(f"       PDF  : {', '.join(sorted(soll))}")
                    print(f"       Eintr: {', '.join(genannt)}  |  {text_taet.strip()[:120]}")
    print(f"\nAnlagen geprueft: {geprueft} | Abweichungen: {abweichungen} "
          f"| Zertifikate ohne PDF: {ohne}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
