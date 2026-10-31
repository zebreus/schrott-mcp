# Rollout-Bericht: Release `bebfd36` (31.10.2026, 12:06 UTC)

## Ergebnis

**Live seit 12:06:18 UTC.** Release-Commit `bebfd36b` (Tree `0d5a0786…`), Binaries `b226f112…`
(Server) und `047a243f…` (Query-Worker). Vorher live: `1665ccb6` (`eefeb0e4…`/`9d213634…`, seit
31.10. 09:49:17 UTC). Backup: `rollout-bebfd36-20261031T120617Z`.

| Nachweis | Wert |
|---|---|
| Pre-Deploy-Review | **GREEN** (unabhängiger Prüfer, zwei Runden) |
| Workspace-Tests | 804 passed / 0 failed / 11 ignored |
| Baum = Commit | 5775 Dateien, byte-identisch (alle Git-Blob-SHAs eigenständig nachgerechnet, 0 Abweichungen) |
| `unexplained_changes` | leer (60 Zeilen) |
| `full_production_unchanged` | true (Fingerprint `ecc96cc1…` eigenständig nachgerechnet) |
| `post_rollout_check.py` | **exit 0** (`ok: true`) |
| Verifier (live) | release-delta **60**, wave-e **60**, pending-corrections **124** — alle exit 0 |
| Vertragstests | 24 Tests, OK |
| Read-only Zielabgleich | **0 Feld-Diffs** über alle 62 Zielzeilen (18 Felder je Zeile) |
| Zähler | 3945 Händler, 83 Materialien, 37710 Preise, 2938 `current_prices` |

## Inhalt: 06:00-Welle (60) + Geocoding-Runde 2

Scope gegen `1665ccb6`: **60 Dossiers** (SH 18 + MV 2 / TH 18 + HH 2 / NI 17 + BY 3), **279 neue
Timeline-Notizen** und **12 neue GEO-Pins**.

| Feld | geänderte Zeilen |
|---|---|
| `notes` | 60 | `description` | 31 | `email` | 17 | `website_status` | 14 | `dropoff_json` | 12 | `lat`/`lon` | 12/12 | `opening_hours` | 11 | `phone` | 8 | `street` | 6 | `postcode` | 4 | `website` | 3 | `status` | 2 | `name` | 2 |

**Preisfunde der Welle:** Rötgesbüttel (Kupfer Millberry 11,30 €/kg; **„Kabel 75 %“ liefert
abwechselnd 4,80 und 6,50 €/kg** — A/B-Test über 6 Abrufe streng alternierend), Gerdes (vollständige
Gebührentafel: Gasflaschendeckel 500 €/Stück, Fahrtkosten je Gebiet 79–120 € + Maut, Standgebühr
3 €/Tag, Kran 25 €), ReRec Neumünster (~70 Positionen, Mischschrott 70/90/110 €/kg, PayPal-Gebühr 2 %),
Travemünde (Gold 999 108,05 €/g, Silberbarren 1.318,09 €), Böttcher (Pellets 165,17–497,17 €,
Lieferpauschale 10,00 €), EXCHANGE AG (Goldbarren 109.010/114.360 €, Krügerrand 3.400/3.580 €),
Scholz-Einkaufsbedingungen (Vertragsstrafe 0,2 %/Werktag, max. 5 %).

## Prüfrunden und Korrekturen

| Befund | Korrektur |
|---|---|
| **F-01** Struktur: zwei nicht eingerückte Fortsetzungszeilen (Poppenbüttel) | eingerückt — der Bullet fällt damit nicht mehr aus dem Präfix-Check |
| **F-02** `website=https://https://…` (doppeltes Schema, existiert in OSM nicht) | auf den live Wert `website=https://www.schrottplatzroetgesbuettelgifhornlager.de/` korrigiert |
| **F-03** Gerdes-Fahrtkosten: zwei Gebiete vertauscht | „Emsland bis 40 km“ = **109,00 € + 15,00 €** Maut, „Emsland > 40 km“ = **nach Absprache + 15,00 €**, „OS / NRW“ = **120,00 € + 0,30 €/km** (live, 4 identische Abrufe) |
| **F-04** derselbe Korrekturvermerk dreimal eingefügt | Dubletten entfernt |

**Werkzeug-Neuzugänge:** `scripts/release/check_pre_wave_notes.py` — strenger Check, dass **keine**
Vorwellen-Notiz verändert, umsortiert oder entfernt wurde (wortgleich, Reihenfolge, Position); er ist
aus dem `/tmp`-Cleanup hervorgegangen und liegt jetzt dauerhaft im Repo. `check_note_order.py` prüft
zusätzlich Dubletten und inhaltsgleiche Korrekturvermerke.

## Rollout-Ablauf

1. **Backup** `rollout-bebfd36-20261031T120617Z` (SQLite-Backup-API aus einer `mode=ro`-Verbindung).
2. **Binaries installiert:** Server `b226f112bdb377d2b8768d0f311bb8910aed0e7fa82d26937c0e7929dfef9529`,
   Query-Worker `047a243f2c76cabb5ed8b3dc531c05a2a2c61e650f468c2f993e6d9bf2c3b53f`.
3. **Dienst neu gestartet** 31.10.2026 12:06:18 UTC (`ActiveEnterTimestamp` = Manifest, `NRestarts` 0),
   Health `{"ok":true}`, `service_state=active`.
4. **Natürlicher Scheduler-Lauf** `ingestion_run` **3297** (12:06:34 → 12:06:34 UTC), `status: ok`,
   „Katalog aktuell, 0 Preise übernommen, 0 Händler-Fehler“ — kein Force-Ingest.
5. **Read-only Zielabgleich:** alle 62 Zielzeilen gegen den Kandidaten-Seed verglichen → **0 Feld-Diffs**.

## Betriebshinweis: Plattenplatz

Vor diesem Release war die `/tmp`-Tmpfs **zu 100 % belegt** (ein Build brach mit „build failed“ ab —
**kein** Datenfehler, sondern Plattenmangel). Aufgeräumt wurden rund 26 GB: 5 Git-Worktrees (6,9 GB),
`target/debug` + `target/release` (7,3 GB), `/root/verify-scratch` (3,0 GB), `/tmp/opencode` (2,4 GB),
`.cache/mc-verify-target` (2,7 GB), `.cache/mig-verify` (869 MB), 12 alte Rollout-Backups (2,6 GB),
alte Prüfer-Klone und Scratch-Verzeichnisse (180 MB), npm-Cache (348 MB). Ergebnis: `/` von 40 GB auf
21 GB belegt (52 GB frei), `/tmp` von 100 % auf 14 % (3,3 GB frei). **Regel:** vor jedem Release nur das
Bundle des aktuellen Kandidaten behalten; Prüf-Artefakte gehören **nicht** nach `/tmp`; Rollout-Backups
auf die letzten fünf Releases begrenzen.

## Offene Punkte (Owner, kein Release-Blocker)

- **Drei Dossier-Hausnummern existieren in OSM nicht** (Würzburg Lengfelder Str. 38, Stuhr Gutenbergstr. 10,
  Emden Schiffbauerdamm 4) — als Negativbefund geschrieben.
- **Emden**: die TSR-Standortseite liefert nur eine Bot-Challenge → Preise nicht belegbar.
- **Weimar**: Dürrenbacher Hütte 1c oder Rießnerstraße 13–19? · **Blankenhain**: SHP ↔ Hertig nicht
  belegbar · **Pößneck**: „Oberer Bahnhof“ vs. „Am Oberen Bahnhof“? · **HUGO Erfurt**: Ladestraße 1 oder 3?
- **Goslar**: die Adresse trägt in OSM die Kreisverwaltung, REWIMET e.V. ist ein Verein → keine
  Koordinate gesetzt.
- **Sylt**: `roselieb-gmbh.de` gehört der Celler Firma (HRB 100262) — der Sylter Betrieb hat keine eigene
  Website.
