#!/usr/bin/env python3
"""Convert the per-state research markdown (recherche/*.md) into structured
seed JSON (seed/traders/<state>.json) for the Schrott MCP public database.

Sources per file:
  1. Markdown tables (header-mapped: Name/Ort/Website/Spezialitaet/Ankauf/...)
  2. Prose register clusters ("Name Ort PLZ" sequences in Nachtrag sections,
     e.g. BY Runde 4) — flagged with prose:true, status pruefung.

Skipped: sections titled Anhang/Ausgeschlossen/Verworfen/Spillover/...,
dedup/exclusion notes, entries explicitly marked as not-in-state.

Slug rule (STABLE — do not hand-edit committed slugs):
    <state>-<city-slug>-<name-slug>  (+ -2/-3 on collision, file order)
New research appends rows; existing slugs never change, so re-imports
update instead of duplicating. Run: python3 tools/md2seed.py
"""
import json
import re
import sys
import unicodedata
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RECH = ROOT / "recherche"
OUT = ROOT / "seed" / "traders"

STATES = ["bw", "by", "be", "bb", "hb", "hh", "he", "mv",
          "ni", "nw", "rp", "sl", "sn", "st", "sh", "th"]

SKIP_SECTION = re.compile(
    r"anhang|ausgeschlossen|verworfen|spillover|außerhalb| můžete|nicht aufnehmen"
    r"| Corrections|Korrektur|Dedup|Dubletten|Exhaustion|erschöpft|Negativ"
    r"|Quellen|Hinweise|Lücken|Verwendung|Umfang|Umland \(nicht",
    re.IGNORECASE,
)
CHUNK_EXCLUDE = re.compile(
    r"alias|duplikat|dublett|ausgeschlossen|nicht aufnehmen|negativ|redirect|leitet "
    r"|kein |keine |nicht |warnung|zusammenhang|identisch|gleiche adresse"
    r"|beide \d|nicht separat|zusammengefasst|bereits .*drin|fehlzuordnung|fälschlich"
    r"|gegenbeweis|kein neueintrag|nicht importieren",
    re.IGNORECASE,
)


def slugify(s: str, maxlen: int) -> str:
    s = unicodedata.normalize("NFKD", s).encode("ascii", "ignore").decode()
    s = s.lower()
    s = re.sub(r"[^a-z0-9]+", "-", s).strip("-")
    s = re.sub(r"-{2,}", "-", s)
    return s[:maxlen].strip("-") or "x"


LEGAL = re.compile(
    r"\b(gmbh\s*(&\s*co\.?)?\s*(kg)?|gbr|ug\s*\(haftungsbeschränkt\)|ug|kg|ohg|eg|e\.?\s*k\.?|e\.?\s*v\.?|gsku|gsk|co\.?\s*kg)\b\.?",
    re.IGNORECASE,
)


def clean_name(raw: str) -> tuple[str, str]:
    """Return (name, extra_note). Pulls (FLAG ...) markers into the note."""
    s = raw.replace("**", "").strip()
    notes = []
    for m in re.finditer(r"\(([^)]*(?:flag|autoverwertung|kfz|zweitstandort|filiale)[^)]*)\)", s, re.I):
        notes.append(m.group(0))
    s = re.sub(r"\(([^)]*(?:flag|autoverwertung|kfz)[^)]*)\)", "", s, flags=re.I).strip()
    s = re.sub(r"\s{2,}", " ", s)
    return s, " ".join(notes)


def clean_website(raw: str) -> tuple[str, str]:
    s = raw.replace("**", "").strip()
    s = re.sub(r"\s*\(.*$", "", s).strip()  # trailing "(... belegt)" notes
    if not s or s in ("—", "-", "?", "/"):
        return "", ""
    if re.match(r"(?i)^(keine?\s+(website|webseite|webauftritt|domain|url)|kein\s+web|n\.?\s*/?\s*a\.?|unbekannt)", s):
        return "", ""
    if re.match(r"(?i)^https?://", s):
        url = s.split()[0].rstrip(").,;")
        return url, ""
    if re.match(r"(?i)^(www\.|[a-z0-9äöü-]+\.[a-z]{2,})", s):
        return "https://" + s.split()[0].rstrip(").,;"), ""
    return "", f"urspr. Website-Angabe: {raw.strip()}"


def split_sites(ort: str) -> list[str]:
    """Split multi-site Ort cells on '+'. Returns city strings."""
    if "+" in ort:
        parts = [p.strip(" ,;") for p in ort.split("+")]
        parts = [p for p in parts if p]
        if 1 < len(parts) <= 12:
            return parts
    return [ort]


def city_of(site: str) -> str:
    s = site.strip()
    s = re.sub(r"\s*\(.*$", "", s)  # "(Lkr. ...)" / "(Str. ...)" / district detail kept? no—strip
    s = s.split(",")[0]
    return s.strip(" -")


def map_status(ankauf: str) -> tuple[str, bool]:
    """Return (status, autoverwertung_hint)."""
    a = ankauf.lower()
    auto = bool(re.search(r"auto ?verwertung|\bkfz\b|fahrzeug|altauto|abschlepp", a))
    if re.search(r"unklar|unsicher|unbestätigt|ungeklärt|prüfen|offen\b|vergleich", a):
        return "pruefung", auto
    if re.search(r"kein\w* ankauf|entsorgung only|kein schrottankauf|reine .*annahme", a):
        return "unbekannt", auto
    if re.search(r"nur (alt ?autos?|kfz|fahrzeuge?|auto ?recycling)", a) or (auto and "ja" not in a):
        return ("aktiv" if re.search(r"\bja\b|annahme", a) else "pruefung"), True
    if re.search(r"\bja\b|annahme|ankauf\b", a):
        return "aktiv", auto
    if not a.strip() or a.strip() in ("—", "-", "?", "/"):
        return "unbekannt", auto
    return "pruefung", auto


def map_type(name: str, spec: str, auto_hint: bool) -> str:
    t = f"{name} {spec}".lower()
    if auto_hint or re.search(r"autoverwertung|auto-?verwertung|\bkfz\b|fahrzeug-?verwertung", t):
        return "autoverwertung"
    if "containerdienst" in t:
        return "containerdienst"
    if "schrottplatz" in t:
        return "schrottplatz"
    if "wertstoff" in t:
        return "wertstoffhaendler"
    if re.search(r"metall ?h?handel|metall ?recycling|buntmetall|ne-?metall", t):
        return "metallhaendler"
    if re.search(r"schrott|altmetall", t):
        return "schrotthaendler"
    if "mobil" in t and len(t) < 120:
        return "mobil"
    return "sonstige"


HEADER_NUM = re.compile(r"^(nr\.?|#|lfd\.?|no\.?)$", re.I)
HEADER_MAP = [
    ("name", re.compile(r"^(name|firma|betrieb|unternehmen)$", re.I)),
    ("city", None),  # contains-match below
    ("website", re.compile(r"^(website|webseite|webauftritt|url|domain|web|homepage|internetseite)$", re.I)),
    ("spec", re.compile(r"^(spezialität|spezialisierung|schwerpunkt|profil|leistung|tätigkeit|branche)$", re.I)),
    ("ankauf", re.compile(r"^ankauf", re.I)),
    ("address", None),  # contains-match below
    ("note", re.compile(r"^(notiz|anmerkung|beleg|quelle|kommentar|quellen?$|anmerkungen|notizen)$", re.I)),
    ("size", re.compile(r"^(größe|groesse|size|groeße)$", re.I)),
]
CITY_HINT = re.compile(r"\b(ort|stadt|stadtteil|bezirk|lage|standort|gemeinde|kreis|region)\b", re.I)
ADDR_HINT = re.compile(r"\b(adresse|anschrift)\b", re.I)


def parse_table_header(cells: list[str]) -> dict | None:
    # Drop pure numbering columns ("#", "Nr") first — they must never win
    # the name mapping.
    kept = [(i, re.sub(r"\s*\(.*$", "", c.strip()).strip(" :")) for i, c in enumerate(cells)]
    kept = [(i, h) for i, h in kept if not HEADER_NUM.match(h)]
    idx: dict[str, int] = {}
    for i, h in kept:
        for key, rx in HEADER_MAP:
            if key in idx or rx is None:
                continue
            if rx.match(h):
                idx[key] = i
    for i, h in kept:
        if "city" not in idx and CITY_HINT.search(h):
            idx["city"] = i
        if "address" not in idx and ADDR_HINT.search(h):
            idx["address"] = i
    if "name" not in idx or ("city" not in idx and "address" not in idx):
        return None
    return idx


def parse_address(addr: str) -> tuple[str, str, str]:
    """Try (street, postcode, cityhint) from 'Street 1, 12345 Town'."""
    m = re.search(r"(\d{5})\s+([A-ZÄÖÜ][\wäöüß.-]+(?:[ -][\wäöüß.-]+)?)", addr)
    postcode = m.group(1) if m else ""
    street = addr
    if m:
        street = (addr[: m.start()] + addr[m.end():]).strip(" ,;")
    return street, postcode, (m.group(2) if m else "")


CITY_WORD = r"[A-ZÄÖÜ][\wäöüß.-]+(?:[ -][A-ZÄÖÜa-zäöüß\d][\wäöüß.-]*){0,1}"
PROSE_PATTERNS = [
    # A: "Name [Ort] PLZ" e.g. "Kollmann Lappersdorf 93138"
    re.compile(
        r"(?P<names>[A-ZÄÖÜ][\wäöüß. &+/-]*)\s+(?P<city>" + CITY_WORD + r")\s+(?P<plz>\d{5})(?=[\s,;)]|$)"
    ),
    # B: "Name, PLZ Ort" e.g. "Katja Hartl, 94209 Regen", "(Huber 82538, Bad Tölz)"
    re.compile(
        r"(?P<names>[A-ZÄÖÜ][^,;()]*)\s*,\s*(?P<plz>\d{5})\s+(?P<city>" + CITY_WORD + r")(?=[\s,;)]|$)"
    ),
]


def parse_prose_chunk(chunk: str) -> list[tuple[str, str, str]]:
    """Return [(name, city, plz)] from one ';'-separated chunk.

    Region labels ride along ("...), **Region** (..."), so split off the
    label part first: entries live before '),' and after the last '('.
    """
    out = []
    chunk = re.sub(r"\*\*", "", chunk).strip()
    if not chunk:
        return out
    pieces = [chunk]
    if ")," in chunk:
        head, tail = chunk.split("),", 1)
        pieces = [head, tail.split("(")[-1]]
    for piece in pieces:
        piece = piece.strip()
        if not piece or CHUNK_EXCLUDE.search(piece):
            continue
        for pat in PROSE_PATTERNS:
            for m in pat.finditer(piece):
                names = m.group("names")
                names = re.split(r"[(:–—]\s*", names)[-1]
                city = m.group("city")
                plz = m.group("plz")
                if len(city) < 3 or len(names.strip()) < 3:
                    continue
                if re.search(r"\d", names):
                    continue
                for n in re.split(r"\s*\+\s*", names.strip()):
                    n = n.strip(" .")
                    if len(n) >= 3 and not re.search(r"\d", n):
                        out.append((n, city.strip(), plz))
            if out:
                break
    return out


def convert_file(stem: str) -> tuple[list[dict], dict]:
    text = (RECH / f"{stem}.md").read_text(encoding="utf-8")
    entries: list[dict] = []
    stats = {"table_rows": 0, "prose_rows": 0, "skipped_sections": 0, "skipped_chunks": 0}
    seen_slugs: set[str] = set()
    section = "top"
    section_skip = False
    header: dict | None = None

    def emit(name_raw, city_raw, website_raw, spec_raw, ankauf_raw, extra_notes,
            postcode="", street="", origin="table"):
        nonlocal entries
        name, flag_note = clean_name(name_raw)
        if not name or len(name) < 2:
            return
        cities = split_sites(city_raw) if origin == "table" else [city_raw]
        website, web_note = clean_website(website_raw)
        status, auto_hint = map_status(ankauf_raw)
        ttype = map_type(name_raw + " " + (flag_note or ""), spec_raw or "", auto_hint)
        if re.search(r"geschlossen|ehemalig|insolvent|abgemeldet", f"{name} {spec_raw} {extra_notes}", re.I):
            status = "geschlossen"
        notes = " | ".join(p for p in [
            (spec_raw or "").strip(),
            flag_note, web_note, extra_notes.strip(),
        ] if p)
        for site in cities:
            city = city_of(site)
            base = f"{stem}-{slugify(city, 28)}-{slugify(LEGAL.sub('', name), 40)}"
            slug = base
            i = 2
            while slug in seen_slugs:
                slug = f"{base}-{i}"
                i += 1
            seen_slugs.add(slug)
            entries.append({
                "slug": slug,
                "name": name,
                "trader_type": ttype,
                "description": "",
                "street": street,
                "postcode": postcode,
                "city": city or site.strip(),
                "state": stem.upper(),
                "website": website,
                "dropoff_json": "",
                "pickup_json": "",
                "status": status,
                "notes": notes[:2000],
                "provenance": {
                    "seed_file": stem,
                    "section": section[:120],
                    "ankauf_raw": ankauf_raw.strip()[:200],
                    "origin": origin,
                },
            })

    for line in text.splitlines():
        s = line.strip()
        m = re.match(r"^#{1,3}\s+(.*)", s)
        if m:
            section = m.group(1).strip()
            section_skip = bool(SKIP_SECTION.search(section))
            if section_skip:
                stats["skipped_sections"] += 1
            header = None
            continue
        if section_skip or not s:
            continue
        if s.startswith("|"):
            cells = [c.strip() for c in s.strip("|").split("|")]
            if any(re.match(r"^:?-{2,}:?$", c) for c in cells):
                continue
            if header is None:
                header = parse_table_header(cells)
                continue
            if header is None:
                continue
            get = lambda k: cells[header[k]] if k in header and header[k] < len(cells) else ""
            name_raw = get("name")
            if re.match(r"^(name|nr\.?|#)$", name_raw, re.I):
                header = parse_table_header(cells)
                continue
            if re.search(r"kein Neueintrag|kein neuer Eintrag", name_raw, re.I):
                # Explicit non-entry: upgrade/merge note for another row.
                # Its payload is patched onto the base entry in JSON review.
                stats["table_rows"] += 1
                continue
            city_raw = get("city") or get("address")
            addr_raw = get("address")
            street, postcode, addr_city = parse_address(addr_raw) if addr_raw else ("", "", "")
            if not city_raw and addr_city:
                city_raw = addr_city
            extra = " ".join(p for p in [get("note"), get("size") and f"Größe: {get('size')}",
                                        addr_raw and f"Adresse: {addr_raw}"] if p)
            emit(name_raw, city_raw, get("website"), get("spec"), get("ankauf"), extra,
                 postcode=postcode, street=street if street != addr_raw else "", origin="table")
            stats["table_rows"] += 1
            continue
        header = None
        # prose clusters: lines with ';' and PLZ outside tables
        if ";" in s and re.search(r"\d{5}", s) and not s.startswith(("-", "*", ">")):
            for chunk in s.split(";"):
                found = parse_prose_chunk(chunk)
                if not found and CHUNK_EXCLUDE.search(chunk):
                    stats["skipped_chunks"] += 1
                for n, c, plz in found:
                    emit(n, c, "", "", "unklar (Register-Prosa)",
                         f"Registerfund ohne geprüfte Website (PLZ {plz})",
                         postcode=plz, origin="prose")
                    stats["prose_rows"] += 1
    return entries, stats


PRESERVE_KEYS = ("description", "dropoff_json", "pickup_json")


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    total = 0
    for stem in STATES:
        entries, stats = convert_file(stem)
        # Never wipe enrichment stored in the committed JSON: carry the
        # enrichment-owned keys forward by stable slug.
        old_rows = {}
        src = OUT / f"{stem}.json"
        if src.exists():
            try:
                old_rows = {r["slug"]: r for r in json.loads(src.read_text(encoding="utf-8"))}
            except (json.JSONDecodeError, KeyError):
                old_rows = {}
        for e in entries:
            old = old_rows.get(e["slug"], {})
            for k in PRESERVE_KEYS:
                if not e[k] and old.get(k):
                    e[k] = old[k]
        (OUT / f"{stem}.json").write_text(
            json.dumps(entries, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
        total += len(entries)
        print(f"{stem}: {len(entries)} entries "
              f"(table={stats['table_rows']} prose={stats['prose_rows']} "
              f"skipsec={stats['skipped_sections']})")
    print(f"TOTAL: {total}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
