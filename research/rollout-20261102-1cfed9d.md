# Rollout `1cfed9d` — 02.11.2026, 22:29 UTC

## Kandidat

| Feld | Wert |
|---|---|
| Commit | `1cfed9d4aff5ef42b11acffe4bd89834603016ff` |
| Tree | `693467b99d210c86230f7d3f6a6bf03cf8ea3169` |
| Vorgänger (Live vor dem Rollout) | `06a5bb8` (02.11. 13:40:59 UTC) |
| Scope | **90** Dossiers, 90 neue Notizen, **19** mit `certifications` |
| Bundle | `target/predeploy-1cfed9d/` (6008 Dateien) |
| Zielliste | `/tmp/opencode/release-targets-1200b.json` (90 Slugs) |

Die **vierte Zertifikats-Welle** vom 02.11. 12:00.

## Unabhängige Pre-Deploy-Freigabe (GREEN)

Prüfer: eigener Prüf-Agent, read-only, Arbeitsverzeichnis `/root/Documents/vclone-mech`,
Kandidat `1cfed9d4`. Belege in `/tmp/opencode/mech57/`.

| Prüfpunkt | Ergebnis |
|---|---|
| Zertifikate live | **18** live am PDF geprüft (15 amtliche Efb-Register-PDFs + 3 ISO-Betreiber-PDFs), **368** Feldvergleiche: **335** exakt, **33** eingeordnet, **0** ungeklärt; 7 AfV-Bescheinigungen live am Dialog |
| Abgrenzungen | 5 Spezialfälle + 5 Fremdstandort-Dossiers geprüft — **0** unzulässige Übernahmen (u. a. Hoppecke: Registerzertifikat ZZET01600069 gehört der *Metallhütte* Hoppecke, HRA 4169, nicht HOPPECKE Batterien, HRB 3989; Philoro: Konzernsitz Leipzig ≠ Filiale Frankfurt; Remhof: Bescheinigung ZAHS01600036 **ohne** Anlagenart → keine Demontagebehauptung; AIPEX: Firmenadresse Minden ≠ Standort Bassum) |
| Negativaussagen | **18** live, **0** Selbsttreffer, **0** Dossier-Adressen; **71/71** mit Reichweitenklausel, **0** ohne |
| Struktur | 90 Dossiers, 0 Konfliktmarker (auch im Gesamttree), 0 Dubletten, JSON valide, Tree 6008/6008 byte-identisch, 3 Release-Gates exit 0 |
| Bundle | **39** Einzelprüfungen, **0** Fehler; Delta 90 Zeilen eigenständig nachgerechnet = Manifest |
| Live unverändert | Fingerprint identisch (3945 Zeilen), Binaries identisch, NRestarts 0, 5 Backup-Dirs unverändert, 90 Scope-Zeilen unberührt |
| Verifier | `pending-corrections` exit 0 (124/0); `release-delta` und `wave-e` **exit 1** gegen die Bundle-Daten — die Abweichung ist **genau der neue 02.11.-Notizblock** (ohne den Block stimmt der Hash) und der **vorbestehende ESH-Live-Drift** (Zeile außerhalb des 90er-Scope, in source/data/live identisch); Vertragstests 2× exit 0 |

### Drei offengelegte Hinweise (keine Beanstandung)

* **B-1:** `certifications` verkürzt den ISO-Scope eines Eintrags (Dossier „Energiespeichersystemen“
  vs. PDF „Energiespeichersysteme zentral und dezentral an Kundenstandorten“). Nummer, Norm,
  Daten, Organisation, Standort und Anlage stimmen exakt; der volle Scope steht wörtlich in
  der Notiz.
* **B-2:** ESH-Live-Drift — vorbestehend, nicht von dieser Welle.
* **B-3:** planmäßige wave-e-Verifier-Semantik (der historische Scope enthält die neue Notiz
  noch nicht).

## Rollout-Ablauf

| Schritt | Wert |
|---|---|
| Backup | `rollout-1cfed9d-20261102T222954Z`, Quelle: SQLite-Backup-API aus einer `mode=ro`-Verbindung — **kein** Produktions-Write |
| Binaries | Server `74503356103c2213589c7789851a75d97c614132c8a1a0c20f0b04da1d276be8`, Query-Worker `8630c446b7d1a8bb661b4c7c9659fc5ed1f7d1c6f36fae79c6dfb8a21d687a91` |
| Neustart | 02.11. 22:29:55 UTC |
| Health | `{"ok":true,"service":"schrott-mcp"}`, HTTP 200, `ActiveState=active` |
| Natürlicher Scheduler-Lauf | `ingestion_runs` id 3534, 22:30:11 UTC, `status=ok`, „Katalog aktuell, 0 Preise übernommen, **0 Händler-Fehler**“ |
| `NRestarts` | 0 |

## Live-Nachprüfung (read-only, `mode=…?mode=ro` + `PRAGMA query_only=ON`)

* `verify-release-delta-1cfed9d.py` → **ok**, 90 geprüft, 0 Fehler
* `verify-wave-e-postrollout-1cfed9d.py` → **ok**, 60 geprüft, 0 Fehler
* `verify-pending-corrections-1cfed9d.py --since 2026-10-27T02:43:15Z` → **ok**, 0 Fehler
* **182** Händler tragen in Live jetzt `certifications` (101 → 118 → 137 → 163 → **182**)
* **0** Live-Zeilen mit der Auslands-Null `(0)`
* Zähler: 3945 Händler · 83 Materialien · 39 406 Preise · 2938 `current_prices`

## Wirkung

* **19** Dossiers mit einem amtlichen Nachweis (Efb §56 KrWG in neun PrüberVarianten,
  AltfahrzeugV §2 Abs. 2, ISO 9001/14001/45001), **71** mit einer belegten Negativaussage
  samt Reichweitenklausel.

## Backups

Nach dem Rollout liegen 6 Backup-Verzeichnisse unter
`/root/Documents/schrott-mcp-rollout-backups/`; die Aufbewahrung wird im nächsten Wellenlauf
wieder auf die letzten fünf begrenzt (E-06/E-07).
