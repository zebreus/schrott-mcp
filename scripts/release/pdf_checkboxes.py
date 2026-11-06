#!/usr/bin/env python3
"""Liest die **Ankreuzfelder** der EfbV-Zertifikats-PDFs aus dem Content-Stream.

Warum das nötig ist: in den Anlagenbögen der amtlichen Zertifikate stehen die
zertifizierten Tätigkeiten in den Feldern 2.1 bis 2.8. Der Text dieser Felder
ist **immer** da - auch dann, wenn das Kästchen **nicht** angekreuzt ist. Wer
nur mit `pdftotext -layout` arbeitet, hält eine leere Feldzeile für eine
zertifizierte Tätigkeit. Zwei Wellen-Einträge sind genau so falsch geworden
(06.11.2026, `by-buchloe-schmidt-recycling` und `nw-dortmund-horde-alcunimet`).

Die PDFs werden von Aspose erzeugt:
  * leeres Kaestchen   = Rechteck-Pfad ``x0 y0 m x1 y0 l x1 y1 l x0 y1 l h s``
  * angekreuztes Feld  = derselbe Pfad, danach zwei diagonale ``m ... l S``-
    Strecken (die beiden "X"-Striche)

Dieses Werkzeug liest die Kästchen und ihre Striche aus dem Content-Stream,
rekonstruiert die Geraetepositionen (CTM + Tm) und ordnet jedes Kaestchen der
Textzeile auf gleicher Hoehe zu.

Aufruf:
    python3 scripts/release/pdf_checkboxes.py <pdf> [<pdf> ...]
    python3 scripts/release/pdf_checkboxes.py --anlagen <pdf>

Ausgabe je Anlagenbogen: die Felder 2.1 bis 2.8 mit `gekreuzt` und Kennummer.
"""
import re
import sys
import zlib

# --- minimale PDF-Objekt-Ebene -------------------------------------------------

OBJ_RE = re.compile(rb"(\d+)\s+(\d+)\s+obj\b(.*?)\bendobj", re.S)


def raw_streams(data: bytes):
    """Alle Content-Streams der Datei (dekomprimiert, falls FlateDecode)."""
    out = []
    for m in re.finditer(rb"stream\r?\n", data):
        start = m.end()
        end = data.find(b"endstream", start)
        if end < 0:
            continue
        chunk = data[start:end]
        # der Dict vor dem Stream verrät die Filter
        head = data[max(0, m.start() - 400):m.start()]
        if b"FlateDecode" in head:
            try:
                chunk = zlib.decompress(chunk)
            except zlib.error:
                try:
                    chunk = zlib.decompressobj().decompress(chunk)
                except zlib.error:
                    continue
        elif b"Filter" in head and b"FlateDecode" not in head:
            continue  # anderer Filter (JPEG o. Ae.) - nicht auswertbar
        out.append(chunk)
    return out


# --- Content-Stream-Tokenizer --------------------------------------------------

NUM = re.compile(rb"[-+]?(?:[0-9]+\.?[0-9]*|\.[0-9]+)")
OP = re.compile(rb"[A-Za-z'\"*][A-Za-z0-9'\"*]*")


def tokens(s: bytes):
    i, n = 0, len(s)
    out = []
    while i < n:
        c = s[i:i + 1]
        if c in b" \t\r\n":
            i += 1
            continue
        if c == b"(":
            depth, j, buf = 1, i + 1, bytearray()
            while j < n and depth:
                ch = s[j:j + 1]
                if ch == b"\\":
                    buf += s[j:j + 2]
                    j += 2
                    continue
                if ch == b"(":
                    depth += 1
                elif ch == b")":
                    depth -= 1
                    if depth == 0:
                        j += 1
                        break
                buf += ch
                j += 1
            out.append(("str", bytes(buf)))
            i = j
            continue
        if s[i:i + 2] == b"<<":
            out.append(("op", "<<"))
            i += 2
            continue
        if s[i:i + 2] == b">>":
            out.append(("op", ">>"))
            i += 2
            continue
        if c == b"/":
            j = i + 1
            while j < n and s[j:j + 1] not in b" \t\r\n/[]<>()":
                j += 1
            out.append(("name", s[i + 1:j].decode("latin-1")))
            i = j
            continue
        m = NUM.match(s, i)
        if m and (chr(s[i]).isdigit() or chr(s[i]) in "+-."):
            out.append(("num", float(m.group(0))))
            i = m.end()
            continue
        m = OP.match(s, i)
        if m:
            out.append(("op", m.group(0).decode("latin-1")))
            i = m.end()
            continue
        i += 1
    return out


def mat_mul(a, b):
    return (a[0] * b[0] + a[1] * b[2], a[0] * b[1] + a[1] * b[3],
            a[2] * b[0] + a[3] * b[2], a[2] * b[1] + a[3] * b[3],
            a[4] * b[0] + a[5] * b[2] + b[4], a[4] * b[1] + a[5] * b[3] + b[5])


def apply(m, x, y):
    return (m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5])


def unescape(s: bytes) -> str:
    s = re.sub(rb"\\([()\\])", rb"\1", s)
    s = re.sub(rb"\\([0-7]{1,3})", lambda m: bytes([int(m.group(1), 8)]), s)
    return s.decode("latin-1", "replace")


def arr_text(raw: bytes) -> str:
    return "".join(unescape(m.group(0)[1:-1])
                   for m in re.finditer(rb"\((?:\\.|[^\\()])*\)", raw))


# --- Seitenanalyse -------------------------------------------------------------

def parse_stream(content: bytes):
    """Textpositionen, Kaestchen und Diagonalen eines Content-Streams."""
    toks = tokens(content)
    ctm = (1, 0, 0, 1, 0, 0)
    stack = []
    tm = tlm = (1, 0, 0, 1, 0, 0)
    texts, boxes, diags = [], [], []
    closed, cur, path = [], None, []
    operands = []

    for kind, val in toks:
        if kind != "op":
            operands.append((kind, val))
            continue
        op = val
        nums = [v for k, v in operands if k == "num"]
        if op == "q":
            stack.append(ctm)
        elif op == "Q":
            if stack:
                ctm = stack.pop()
        elif op == "cm" and len(nums) >= 6:
            ctm = mat_mul(tuple(nums[-6:]), ctm)
        elif op == "BT":
            tm = tlm = (1, 0, 0, 1, 0, 0)
        elif op == "Tm" and len(nums) >= 6:
            tm = tlm = tuple(nums[-6:])
        elif op == "Td" and len(nums) >= 2:
            tlm = mat_mul((1, 0, 0, 1, nums[-2], nums[-1]), tlm)
            tm = tlm
        elif op in ("Tj", "TJ", "'", '"'):
            text = ""
            for k, v in operands:
                if k == "str":
                    text += unescape(v)
                elif k == "arr":
                    text += arr_text(v)
            if op == "TJ":
                for k, v in operands:
                    if k == "arr":
                        text = arr_text(v)
                        break
            if text.strip():
                p = apply(mat_mul(tm, ctm), 0, 0)
                texts.append({"x": p[0], "y": p[1], "text": text})
        elif op == "m" and len(nums) >= 2:
            cur = [(nums[-2], nums[-1])]
            path = [cur]
        elif op == "l" and len(nums) >= 2:
            if path:
                path[-1].append((nums[-2], nums[-1]))
        elif op == "h":
            if cur and len(cur) >= 3:
                closed.append(cur)
            cur = None
        elif op == "s":                      # Rechteck gestrichen -> Kaestchen
            for pts in closed:
                dp = [apply(ctm, px, py) for px, py in pts]
                xs = [p[0] for p in dp]
                ys = [p[1] for p in dp]
                boxes.append({"x0": min(xs), "y0": min(ys), "x1": max(xs), "y1": max(ys),
                              "checked": False})
            closed, path, cur = [], [], None
        elif op == "S":                      # offene Strecke -> moeglicher "X"-Strich
            if cur and len(cur) >= 2:
                diags.append([apply(ctm, px, py) for px, py in cur])
            closed, path, cur = [], [], None
        elif op in ("f", "f*", "B", "b", "n"):
            closed, path, cur = [], [], None
        operands = []

    for b in boxes:
        for d in diags:
            if len(d) < 2:
                continue
            xs = [p[0] for p in d[:2]]
            ys = [p[1] for p in d[:2]]
            if (min(xs) >= b["x0"] - 2 and max(xs) <= b["x1"] + 2 and
                    min(ys) >= b["y0"] - 2 and max(ys) <= b["y1"] + 2):
                b["checked"] = True
                break
    return texts, boxes


TAETIGKEITEN = {
    "2.1": "Sammeln", "2.2": "Befördern", "2.3": "Lagern", "2.4": "Behandeln",
    "2.5": "Verwerten", "2.6": "Beseitigen", "2.7": "Handeln", "2.8": "Makeln",
}
FELD_RE = re.compile(r"^\s*(2\.\d)\s*(Sammeln|Befördern|Lagern|Behandeln|Verwerten|"
                     r"Beseitigen|Handeln|Makeln)\b(.*)$", re.M)


def analyse(path: str):
    """Je Anlagenbogen: die Felder 2.1 bis 2.8 mit Ankreuzzustand und Kennnummer."""
    data = open(path, "rb").read()
    out = []
    for stream in raw_streams(data):
        texts, boxes = parse_stream(stream)
        if not texts or not boxes:
            continue
        # Textzeilen (gleiche y) bilden
        zeilen = {}
        for t in texts:
            zeilen.setdefault(round(t["y"], 1), []).append(t)
        kopf = None
        for y in sorted(zeilen, reverse=True):
            line = " ".join(i["text"] for i in sorted(zeilen[y], key=lambda t: t["x"]))
            if "zum Zertifikat mit der Nummer" in line:
                kopf = line.strip()
                break
        if kopf is None:
            continue
        felder = {}
        for y2, items2 in zeilen.items():
            items2 = sorted(items2, key=lambda t: t["x"])
            text = " ".join(i["text"] for i in items2)
            m = FELD_RE.match(text)
            if not m:
                continue
            feld, tat, rest = m.group(1), m.group(2), m.group(3)
            kenn = ""
            km = re.search(r"Kennnummer nach \u00a7 28 NachwV:\s*([A-Z0-9]*)", rest)
            if km:
                kenn = km.group(1)
            # Kaestchen auf derselben Hoehe, rechts vom Text
            x_text = items2[0]["x"]
            cands = [b for b in boxes
                     if abs((b["y0"] + b["y1"]) / 2 - y2) < 4 and b["x0"] > x_text]
            cands.sort(key=lambda b: b["x0"])
            gekreuzt = bool(cands) and cands[0]["checked"]
            felder[feld] = {"taetigkeit": tat, "gekreuzt": gekreuzt, "kennung": kenn}
        out.append({"anlage": kopf, "felder": felder})
    return out


def main() -> int:
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    if not args:
        print(__doc__)
        return 2
    for path in args:
        print("=" * 100)
        print(path)
        for a in analyse(path):
            print("  ", a["anlage"][:110])
            for feld in sorted(a["felder"]):
                f = a["felder"][feld]
                print(f"      {feld} {f['taetigkeit']:<12} gekreuzt={str(f['gekreuzt']):<5} "
                      f"kennung={f['kennung'] or '-'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
