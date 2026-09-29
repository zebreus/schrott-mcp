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


def clean_website(raw: str) -> tuple[str, str, str]:
    """Return (url, website_status, note). Trailing markers like
    '— TOT' / '(offline)' set the status instead of killing the URL:
    a dead address is still their address."""
    s = raw.replace("**", "").strip()
    status = ""
    m = re.search(r"\s+[—–-]\s*([^—–()]*)$|\(([^()]*)\)$", s)
    if m:
        marker = (m.group(1) or m.group(2) or "").lower()
        if re.search(r"\btot\b|dead|offline|erloschen|dns|timeout|geparkt|kommt bald|coming soon", marker):
            status = "tot"
        elif re.search(r"blockiert|bot|403|429|captcha", marker):
            status = "blockiert"
        s = s[: m.start()].strip()
    s = re.sub(r"\s*\(.*$", "", s).strip()  # other trailing "(...)" notes
    if not s or s in ("—", "-", "?", "/"):
        return "", status, ""
    if re.match(r"(?i)^(keine?\s+(website|webseite|webauftritt|domain|url)|kein\s+web|n\.?\s*/?\s*a\.?|unbekannt)", s):
        return "", status, ""
    if re.match(r"(?i)^https?://", s):
        url = s.split()[0].rstrip(").,;")
        return url, status, ""
    if re.match(r"(?i)^(www\.|[a-z0-9äöü-]+\.[a-z]{2,})", s):
        return "https://" + s.split()[0].rstrip(").,;"), status, ""
    return "", status, f"urspr. Website-Angabe: {raw.strip()}"


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
    s = re.sub(r"\s*\(.*$", "", s)  # "(Lkr. ...)" / street detail stripped
    s = s.split(",")[0]
    return s.strip(" -")


def extract_contact(text: str) -> tuple[str, str, str]:
    """Conservatively pull street/postcode/phone out of free text.

    - street+postcode only as an adjacent pair ("Kruppstr. 81, 47229")
      or with an explicit PLZ marker — a bare 5-digit number could be a
      phone area code, never a postcode on its own.
    - phone only with Tel/Fax marker or a full 0-prefixed number with
      separators (never bare digit runs).
    Returns (street, postcode, phone), each possibly "".
    """
    street, postcode, phone = "", "", ""
    m = re.search(
        r"([A-ZÄÖÜ][\wäöüß. -]*?(?:str\.|straße|weg|allee|gasse|platz|damm|ufer|ring|chaussee|zeile)\s*\d[\w/-]*)(?:,?\s*(\d{5})\b)?",
        text,
    )
    if m:
        street = m.group(1).strip()
        postcode = m.group(2) or ""
    if not postcode:
        m = re.search(r"\bPLZ\s*(\d{5})\b", text)
        if m:
            postcode = m.group(1)
    m = re.search(
        r"(?:Tel\.?|Telefon|Mobil|Handy|Fax)[:\s]*(\+?[\d\s/()\-]{6,}\d)"
        r"|(?<![\w(])((?:\+49|0)\d{2,5}[\s/\-()]*\d[\d\s/\-()]{4,})(?![\w)])",
        text,
    )
    if m:
        phone = re.sub(r"\s+", " ", (m.group(1) or m.group(2)).strip())
    return street, postcode, phone


def norm_identity(name: str, city: str) -> tuple[str, str]:
    """Dedupe key: legal forms and case/accents stripped."""
    n = unicodedata.normalize("NFKD", LEGAL.sub("", name)).encode("ascii", "ignore").decode()
    n = re.sub(r"[^a-z0-9 ]", " ", n.lower())
    c = unicodedata.normalize("NFKD", city).encode("ascii", "ignore").decode()
    c = re.sub(r"[^a-z0-9 ]", " ", c.lower())
    return re.sub(r"\s+", " ", n).strip(), re.sub(r"\s+", " ", c).strip()


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
    stats = {"table_rows": 0, "prose_rows": 0, "skipped_sections": 0, "skipped_chunks": 0,
             "dupe_skips": []}
    seen_slugs: set[str] = set()
    seen_identity: set[tuple[str, str]] = set()
    section = "top"
    section_skip = False
    header: dict | None = None
    upgrades: list[dict] = []
    upgrade_mode = False

    def emit(name_raw, city_raw, website_raw, spec_raw, ankauf_raw, extra_notes,
            postcode="", street="", origin="table"):
        nonlocal entries
        name, flag_note = clean_name(name_raw)
        if not name or len(name) < 2:
            return
        cities = split_sites(city_raw) if origin == "table" else [city_raw]
        website, website_status, web_note = clean_website(website_raw)
        status, auto_hint = map_status(ankauf_raw)
        ttype = map_type(name_raw + " " + (flag_note or ""), spec_raw or "", auto_hint)
        if re.search(r"geschlossen|ehemalig|insolvent|abgemeldet", f"{name} {spec_raw} {extra_notes}", re.I):
            status = "geschlossen"
        notes = " | ".join(p for p in [
            (spec_raw or "").strip(),
            flag_note, web_note, extra_notes.strip(),
        ] if p)
        cs, cp, cph = extract_contact(f"{name_raw} {city_raw} {notes}")
        street = street or cs
        postcode = postcode or cp
        first_city = city_of(cities[0]) if cities else ""
        ident = norm_identity(name, first_city)
        if ident in seen_identity and ident[0]:
            stats["dupe_skips"].append(f"{name}ᴉ{first_city}")
            return
        seen_identity.add(ident)
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
                "phone": cph,
                "city": city or site.strip(),
                "state": stem.upper(),
                "website": website,
                "website_status": website_status,
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

    upgrade_cols: dict = {}

    def collect_upgrade(cells: list[str]):
        # Column-mapped upgrade rows. The table header decides the layout:
        # Name | Ort | Straße | PLZ | Telefon | Quelle (A) or
        # Name | Bezirk | Straße | PLZ | Telefon | Quelle (B, Berlin).
        nonlocal upgrade_cols
        low = [c.lower() for c in cells]
        if any("stra" in c or c in ("plz", "telefon", "quelle", "bezirk", "ort") for c in low) \
                and not any(re.search(r"\d", c) for c in cells):
            upgrade_cols = {}
            for i, h in enumerate(low):
                if re.match(r"^(name|firma|betrieb)$", h):
                    upgrade_cols["name"] = i
                elif h in ("ort", "stadt", "stadtteil", "lage", "standort"):
                    upgrade_cols["city"] = i
                    upgrade_cols["is_bezirk"] = -1
                elif h == "bezirk":
                    upgrade_cols["city"] = i
                    upgrade_cols["is_bezirk"] = i
                elif "stra" in h:
                    upgrade_cols["street"] = i
                elif h == "plz":
                    upgrade_cols["postcode"] = i
                elif h == "telefon":
                    upgrade_cols["phone"] = i
                elif h in ("quelle", "quellen", "notiz"):
                    upgrade_cols["note"] = i
            return
        cols = upgrade_cols or {"name": 0, "city": 1, "street": 2,
                               "postcode": 3, "phone": 4, "note": 5}
        def col(key: str) -> str:
            i = cols.get(key, -1)
            return cells[i] if 0 <= i < len(cells) else ""
        name = col("name")
        if not name or len(name) < 2:
            return
        slug_override = ""
        m = re.match(r"^slug:(\S+)\s*(.*)$", name)
        if m:
            slug_override, name = m.group(1), m.group(2).strip()
        street = col("street")
        if not re.search(r"\d", street):
            return
        city = col("city")
        bezirk = city if cols.get("is_bezirk", -1) == cols.get("city") else ""
        if bezirk:
            city = ""
        upgrades.append({
            "name": name, "slug": slug_override, "city": city, "bezirk": bezirk, "street": street,
            "postcode": col("postcode"), "phone": col("phone"),
            "note": col("note"),
            "korrektur": bool(re.search(r"korrektur", name, re.I)),
        })

    for line in text.splitlines():
        s = line.strip()
        m = re.match(r"^#{1,3}\s+(.*)", s)
        if m:
            section = m.group(1).strip()
            section_skip = bool(SKIP_SECTION.search(section))
            if section_skip:
                stats["skipped_sections"] += 1
            # "Nachtrag Adressen" sections never emit new traders: their rows
            # patch empty address/phone fields of existing entries (matched
            # by name+city, website domain, or single-candidate fuzzy).
            # Slugs stay stable; unmatched rows are reported, never appended.
            upgrade_mode = bool(re.search(r"nachtrag adressen", section, re.I))
            header = None
            upgrade_cols = {}
            continue
        if section_skip or not s:
            continue
        if s.startswith("|"):
            cells = [c.strip() for c in s.strip("|").split("|")]
            if any(re.match(r"^:?-{2,}:?$", c) for c in cells):
                continue
            if upgrade_mode:
                collect_upgrade(cells)
                stats["table_rows"] += 1
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
    apply_upgrades(entries, upgrades, stats)
    return entries, stats


def apply_upgrades(entries: list[dict], upgrades: list[dict], stats: dict):
    """Patch empty address/phone fields of existing entries (slug-stable).

    Resolution per row: exact name+city identity, then website-domain
    overlap with the Quelle cell, then single-candidate fuzzy on the city.
    Anything ambiguous or unmatched is reported in stats and never appended.
    KORREKTUR rows overwrite street/postcode/phone; all others fill empties
    only. Bezirk notes never touch city (slug stability).
    """
    by_ident: dict = {}
    by_dom: dict = {}
    for e in entries:
        by_ident.setdefault(norm_identity(e["name"], e["city"]), []).append(e)
        m = re.search(r"([\w-]+\.[\w.-]+)", e.get("website") or "")
        if m:
            by_dom.setdefault(m.group(1).lower(), []).append(e)
    stats["upgrades_applied"] = 0
    stats["upgrade_misses"] = []
    for u in upgrades:
        name, _ = clean_name(u["name"])
        match_name = re.sub(r"[—–-]\s*korrektur\s*$", "", name, flags=re.I).strip()
        cands = []
        if u.get("slug"):
            cands = [e for e in entries if e["slug"] == u["slug"]]
            if len(cands) != 1:
                stats["upgrade_misses"].append(f"{u['name']} (slug {u['slug']})")
                continue
            e = cands[0]
        else:
            match_city = u["city"] or u["bezirk"]
            ident = norm_identity(match_name, match_city.split("(")[0].strip())
            cands = list(by_ident.get(ident, []))
            if not cands:
                m = re.search(r"([\w-]+\.[\w.-]+)", u.get("note") or "")
                if m:
                    cands = list(by_dom.get(m.group(1).lower(), []))
            if not cands and ident[0]:
                def same_place(a: str, b: str) -> bool:
                    return a == b or a.startswith(b) or b.startswith(a)
                cands = [e for key, lst in by_ident.items()
                         if same_place(key[1], ident[1])
                         and (ident[0] in key[0] or key[0] in ident[0])
                         for e in lst]
                # keep only unambiguous single-trader hits
                slugs = {e["slug"] for e in cands}
                if len(slugs) != 1:
                    cands = []
            if len(cands) != 1:
                stats["upgrade_misses"].append(
                    f"{u['name']} / {u['city'] or u['bezirk']}")
                continue
            e = cands[0]
        if u["korrektur"]:
            e["street"], e["postcode"], e["phone"] = \
                u["street"], u["postcode"], u["phone"]
        else:
            if not e["street"]:
                e["street"] = u["street"]
            if not e["postcode"]:
                e["postcode"] = u["postcode"]
            if not e["phone"]:
                e["phone"] = u["phone"]
        extra = " ".join(p for p in [
            u["bezirk"] and f"Bezirk: {u['bezirk']}",
            u["note"] and f"Adressbeleg: {u['note']}",
        ] if p)
        if extra and extra not in e["notes"]:
            e["notes"] = (e["notes"] + " | " + extra)[:2000]
        stats["upgrades_applied"] += 1


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
              f"skipsec={stats['skipped_sections']} dupes={len(stats['dupe_skips'])} "
              f"upgrades={stats.get('upgrades_applied', 0)} "
              f"misses={len(stats.get('upgrade_misses', []))})")
        for d in stats["dupe_skips"][:20]:
            print(f"    dupe-skip: {d}")
        for m in stats.get("upgrade_misses", [])[:20]:
            print(f"    upgrade-miss: {m}")
    print(f"TOTAL: {total}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
