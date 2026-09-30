#!/usr/bin/env python3
"""Merge verified trader enrichment (address/website/hours/services) into
`seed/traders/*.json`, idempotently.

Input: append-only JSONL log, one object per trader:
  {slug, website, street, postcode, city, phone, email, source_url,
   source_type, opening_hours, services, certifications, customer_types,
   min_quantity_kg, notes}
Missing keys = not belegt (never guessed upstream).

Rules (durable by design):
- Scalar fields (website, phone, email, opening_hours, street, postcode):
  fill only when the seed value is empty. Never overwrite research.
- website filled  => website_status "aktiv" (when empty).
- street/postcode only when the agent city matches the seed city
  (normalized); otherwise warn + skip (avoids mismatched addresses).
- services/certifications/customer_types/agent notes are appended ONCE to
  seed `notes` with a "[Website-Recherche ...]" marker (dedupe by marker).
- Run order after regeneration: `md2seed.py && merge_enrichment.py`
  (`md2seed` PRESERVE_KEYS additionally carries scalar enrichment forward).

Usage: python3 tools/merge_enrichment.py [pending.jsonl]
"""
import json
import re
import sys
import unicodedata
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SEED = ROOT / "seed" / "traders"
STATES = ["bw", "by", "be", "bb", "hb", "hh", "he", "mv",
          "ni", "nw", "rp", "sl", "sn", "st", "sh", "th"]

SCALARS = ("website", "phone", "email", "opening_hours", "street", "postcode")
MARKER = "[Website-Recherche"


def norm(s: str) -> str:
    s = unicodedata.normalize("NFKD", s or "").encode("ascii", "ignore").decode().lower()
    return " ".join(s.split())


def norm_city(s: str) -> str:
    # Seed cities often carry a PLZ suffix ("Alsbach-Hähnlein 64665",
    # "Mannheim-Rheinau"); strip it before comparing.
    n = norm(s)
    n = re.sub(r"\s+\d{5}$", "", n)
    return n


def main() -> int:
    pending = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "recherche" / "enrichment_pending.jsonl"
    if not pending.exists():
        print(f"no pending file: {pending}")
        return 1
    # Load seed files once.
    corpus = {}
    for st in STATES:
        p = SEED / f"{st}.json"
        corpus[st] = (p, json.loads(p.read_text(encoding="utf-8")))
    by_slug = {}
    for st, (p, rows) in corpus.items():
        for r in rows:
            by_slug[r["slug"]] = (st, r)

    n_scalar = n_notes = n_skip = 0
    warnings = []
    for line in pending.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        o = json.loads(line)
        slug = o.get("slug", "")
        hit = by_slug.get(slug)
        if hit is None:
            warnings.append(f"{slug}: unknown slug, skipped")
            n_skip += 1
            continue
        st, row = hit
        # City guard for street/postcode.
        city_ok = (not (o.get("city") or "").strip()
                   or norm_city(o.get("city")) == norm_city(row.get("city", "")))
        for k in SCALARS:
            v = (o.get(k) or "").strip()
            if not v or row.get(k):
                continue
            if k in ("street", "postcode") and not city_ok:
                warnings.append(
                    f"{slug}: city mismatch agent={o.get('city')!r} seed={row.get('city')!r} — street/postcode skipped")
                continue
            if k == "website" and not (v.startswith("http://") or v.startswith("https://")):
                warnings.append(f"{slug}: bad website {v!r}, skipped")
                continue
            row[k] = v
            n_scalar += 1
        if row.get("website") and not row.get("website_status"):
            row["website_status"] = "aktiv"
            n_scalar += 1
        # Notes append (once).
        bits = []
        for k in ("services", "certifications", "customer_types", "notes"):
            v = (o.get(k) or "").strip()
            if v:
                bits.append(f"{k}: {v}")
        src = (o.get("source_type") or "").strip() or "website"
        if bits and MARKER not in (row.get("notes") or ""):
            suffix = f" {MARKER} {src}: " + "; ".join(bits) + "]"
            row["notes"] = ((row.get("notes") or "") + suffix).strip()
            n_notes += 1
    for st, (p, rows) in corpus.items():
        p.write_text(json.dumps(rows, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")
    print(f"merged: {n_scalar} scalar fills, {n_notes} notes appends, {n_skip} skipped")
    for w in warnings[:20]:
        print(f"  WARN {w}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
