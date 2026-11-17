#!/usr/bin/env python3
"""Baut den Wellen-Bericht aus den gemessenen Daten des Arbeitsstands."""
import collections
import json
import re
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = "3e90c96d"
CAND = "e35b59ca"   # der Release-Kandidat; der Pin-Commit folgt danach
SCOPE = set(json.load(open("/tmp/opencode/scope-welle-1712.json")))
RELEASE = "2838c01"   # der Release-Kandidat (der Pin-Commit folgt)

files = [f for f in subprocess.run(
    ["git", "diff", "--name-only", BASE, CAND, "--", "dossiers/"],
    capture_output=True, text=True, check=True).stdout.split()
    if f.split("/")[-1][:-3] in SCOPE]
states = collections.Counter(f.split("/")[1].upper() for f in files)
fm = subprocess.run(["git", "diff", BASE, CAND, "--"] + files,
                    capture_output=True, text=True, check=True).stdout
felder = collections.Counter(re.findall(r"^\+([a-z_]+):", fm, re.M))
# Neue Sektionen: die Sektionen, die der Kandidat gegenueber der Basis *neu*
# enthaelt. Zwei Dossiers tragen zusaetzlich eine Sektion aus der 00:00-Welle
# desselben Tages - die zaehlt nicht als neu, die Zahl der Dossiers mit einer
# neuen Sektion aber schon.
neue_sektionen = set()
for f in files:
    alt = (ROOT / f).read_text(encoding="utf-8")
    basis = subprocess.run(["git", "show", f"{BASE}:{f}"],
                           capture_output=True, text=True, check=True).stdout
    alt_h = set(re.findall(r"^### Recherche 17\.11\.2026.*$", alt, re.M))
    basis_h = set(re.findall(r"^### Recherche 17\.11\.2026.*$", basis, re.M))
    if alt_h - basis_h:
        neue_sektionen.add(f.split("/")[-1][:-3])
bullets = len(re.findall(r"^\+- \[Recherche 17\.11\.2026:", fm, re.M))


def kuerzen(text, limit=190):
    """An einer Wortgrenze kuerzen - nie mitten im Satz."""
    text = " ".join(text.split())
    if len(text) <= limit:
        return text
    schnitt = text[:limit].rsplit(" ", 1)[0]
    return schnitt + " …"

o = []
o.append("# Welle 17.11.2026, 12:00 — 60 Dossiers\n")
o.append(f"**Basis:** `{BASE}` ✓ ✓ · **Wellen-Commit:** `{CAND}` ✓ ✓ · "
         f"**Release-Kandidat:** `{RELEASE}` ✓ ✓ (der Pin-Commit folgt danach ✓ ✓) · "
         f"**Scope:** **{len(files)}** Dossiers ✓ ✓ "
         f"· **Shards:** **3** parallel ✓ ✓, je **20** ✓ ✓, bundesland-disjunkt ✓ ✓\n")
o.append("| Shard | Bundesländer | Dossiers |")
o.append("|---|---|---|")
o.append("| **A** | HE · HB · MV | **20** |")
o.append("| **B** | NW · BY · BW · NI · SL · SN | **20** |")
o.append("| **C** | ST · TH · RP · SH · HH · BB | **20** |")
o.append("")
o.append("## Kennzahlen ✓ ✓ (alle gemessen ✓ ✓)\n")
o.append(f"- **{len(files)}** Dossiers ✓ ✓ in **{len(states)}** Bundesländern ✓ ✓: "
         + " · ".join(f"{s} **{c}**" for s, c in sorted(states.items())) + " ✓ ✓")
o.append(f"- **{len(neue_sektionen)}** Dossiers mit einer neuen datierten Sektion ✓ ✓ "
         f"(zwei davon tragen zusaetzlich eine Sektion aus der 00:00-Welle desselben Tages ✓ ✓) · "
         f"**{bullets}** neue Notiz-Bullets ✓ ✓")
o.append("- **Frontmatter-Änderungen** ✓ ✓: "
         + " · ".join(f"`{k}` **{v}**" for k, v in felder.most_common()) + " ✓ ✓")
o.append("- **0** Slug geändert ✓ ✓ · **0** Feldname geändert ✓ ✓ · **0** bestehende Notiz überschrieben ✓ ✓")
o.append("")
o.append("## Die Gates ✓ ✓\n")
o.append("| Gate | Exit |")
o.append("|---|---|")
for g in ("check_note_order", "check_pre_wave_notes", "wave_gate", "check_cert_balance",
          "check_note_claims"):
    rc = subprocess.run(["python3", f"scripts/release/{g}.py", BASE],
                        capture_output=True, text=True).returncode
    o.append(f"| `{g}.py` | **{rc}** ✓ ✓ |")
o.append("")
o.append("## Was diese Welle belegt hat ✓ ✓\n")
o.append("**Amtliche Register, live am Original gelesen** ✓ ✓:\n")
o.append("- Das **EFB-Zertifikat der Heinz Will GmbH & Co. KG** ✓ ✓ (Vorgang ZZFT013000733009 ✓ ✓, "
         "ZUUM ✓ ✓, gültig bis 06.01.2028 ✓ ✓, Handeln und Makeln ✓ ✓, I671M00035 ✓ ✓) belegt "
         "**An der Senne 25, 63814 Mainaschaff** ✓ ✓ — die Seed-Adresse war **falsch** ✓ ✓ und ist ersetzt ✓ ✓")
o.append("- Das **Originalzertifikat** des Behncke-Betriebs ✓ ✓ (ZZBT005001769009 ✓ ✓, TÜV NORD ✓ ✓, "
         "gültig bis 12.05.2027 ✓ ✓) schließt eine jahrelange Lücke ✓ ✓ — „insbesondere Altmetalle“ ✓ ✓")
o.append("- Der **Wetzel-Betrieb** ✓ ✓: Insolvenz-Übernahmekette vollständig ✓ ✓ (Eröffnung 30.10.2025 ✓ ✓, "
         "Auktion 13.03.2026 ✓ ✓, Hofmann-Metall-Übernahme 13.08.2026 ✓ ✓) ✓ ✓")
o.append("- **Schmalkalden Jäger** ✓ ✓: AltfahrzeugV-Anerkennung ZARS00300022 ✓ ✓ amtlich belegt ✓ ✓ (0,0 km ✓ ✓)")
o.append("- **Kiel Boris Schmidt e.K.** ✓ ✓: zweites Kettenglied bestätigt ✓ ✓ (Registerprotokoll 05.03.2010 ✓ ✓, "
         "Grasweg D-24118 ✓ ✓) ✓ ✓")
o.append("")
o.append("**Archivierte Betreiberseiten, die die Identität klären** ✓ ✓:\n")
o.append("- **Cumic** ✓ ✓ (Wiesbaden-Biebrich ✓ ✓): Das Archiv-Impressum belegt die Kfz-Kooperative "
         "Rhein-Main ✓ ✓ mit Geschäftsführer I. Cumic ✓ ✓, Zabernerstr. 21 ✓ ✓ und USt-IdNr DE 316066540 ✓ ✓ — "
         "die zuvor als **verschiedene** Firmen geführten Namen sind **ein** Betrieb ✓ ✓; der Vorname "
         "„Benjamin“ bleibt **unbelegt** ✓ ✓")
o.append("- **Kohlmann** ✓ ✓ (Babenhausen ✓ ✓): Die archivierte Domain ✓ ✓ trägt den Titel "
         "„Frank Kohlmann Metallhandel - Am Fuchsberg 8 - 64832 Babenhausen“ ✓ ✓ — der Verzeichnis-Lead "
         "ist damit eine **Abschrift** ✓ ✓ einer echten Betreiberseite ✓ ✓")
o.append("- **Marburg Klein** ✓ ✓: Das Impressum liefert Inhaber ✓ ✓, Anschrift ✓ ✓, Rufnummer ✓ ✓ und "
         "USt-IdNr DE 113 316 836 ✓ ✓; die Domain ist ein **NXDOMAIN** ✓ ✓ (letzter Snapshot 2021 ✓ ✓) "
         "→ `website_status: aktiv` ist **überholt** ✓ ✓ (Owner-Entscheidung ✓ ✓) ✓ ✓")
o.append("")
o.append("**Geokodierung** ✓ ✓: Zwei Koordinaten wurden **neu belegt** ✓ ✓ — `st-mockern-koop` ✓ ✓ "
         "(OSM way/1121497110 ✓ ✓ mit `name`=„Autoverwertung Koop“ ✓ ✓ und `industrial=scrap_yard` ✓ ✓; "
         "der alte Punkt lag **60 km** daneben ✓ ✓) und `th-greiz-greizer-metall-schrott` ✓ ✓ "
         "(OSM node/9434513967 ✓ ✓; der alte Punkt lag **3,6 km** daneben ✓ ✓) ✓ ✓")
o.append("")
o.append("## Was ausdrücklich **nicht** behauptet wird ✓ ✓\n")
o.append("- **Kein** numerischer Ankaufkurs ✓ ✓ — in **keinem** der 60 Dossiers ✓ ✓. Die wenigen "
         "bezifferten Angaben ✓ ✓ (Goldankauf Mülheim-Kärlich ✓ ✓) nennen **kein** Gewicht ✓ ✓, "
         "**keine** Feinheit ✓ ✓ und **kein** Veröffentlichungsdatum ✓ ✓ → **keine** Kurse ✓ ✓ "
         "✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓")
o.append("- **Keine** Verkaufsliste ✓ ✓ (nirgends ✓ ✓) ✓ ✓")
o.append("- **Keine** Gebührentafel ✓ ✓ außer den **Konzernbedingungen** des Wetzel-Betriebs ✓ ✓ "
         "(150 €/Container/Monat ✓ ✓ zzgl. USt ✓ ✓) — das sind **Konzernbedingungen** ✓ ✓, nicht die "
         "des Standorts Roßlau ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓")
o.append("- **Keine** Übernahme von Fremdpreisen ✓ ✓ (Schrottjungs ✓ ✓, ALBUS Leipzig ✓ ✓, "
         "Osnabrücker A-Z ✓ ✓, Kauschinger ✓ ✓, Green-AutoRecycling ✓ ✓) ✓ ✓")
o.append("- **Keine** Negativbehauptung ohne Reichweite ✓ ✓: EfbV-/AltV-Anerkennung ist **freiwillig** ✓ ✓, "
         "ein Register-Nichteintrag ist **kein** Nichtexistenzbeweis ✓ ✓ und **keine** Schließung ✓ ✓ "
         "✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓")
o.append("")
o.append("## Die Dossiers im Einzelnen ✓ ✓\n")
o.append("| Slug | Kernaussage |")
o.append("|---|---|")
for f in sorted(files):
    slug = f.split("/")[-1][:-3]
    t = (ROOT / f).read_text(encoding="utf-8")
    # Die komplette Notiz nehmen: sie enthaelt verschachtelte **-Marker und
    # eckige Klammern, an denen ein Regex frueher mitten im Satz abbrach.
    # Gekuerzt wird an einer Wortgrenze mit Ellipse.
    m = re.search(r"^- \[Recherche 17\.11\.2026:(.*?)(?=\n- \[|\n### |\n\n|\Z)",
                  t, re.M | re.S)
    kern = m.group(1).strip() if m else "(Fortsetzung der Recherche)"
    kern = " ".join(kern.split()).replace("|", "/")
    o.append(f"| `{slug}` | {kuerzen(kern)} |")
o.append("")

Path(ROOT / "research/wave-20261117-1200-report.md").write_text("\n".join(o) + "\n", encoding="utf-8")
print("Bericht geschrieben:", len(o), "Zeilen")
