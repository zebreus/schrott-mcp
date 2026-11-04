# Rollout `3ffdf8b` — 04.11.2026, 08:59 UTC

## Kandidat

| Feld | Wert |
|---|---|
| Commit | `3ffdf8bc4f87254db754cb1c5bd25e1a37f375b4` |
| Tree | `91e06536a218b3534f6bd035634b019e22f693c5` |
| Vorgänger (Live vor dem Rollout) | `03a0af3` (03.11. 23:20:16 UTC) |
| Scope | **87** Dossiers, 89 geänderte Notizen, **43** neue Koordinaten |
| Bundle | `target/predeploy-3ffdf8b/` (6088 Dateien) |
| Zielliste | `/tmp/opencode/release-targets-0000d.json` (87 Slugs) |

Die **Geokodierungs-Welle** vom 04.11. 00:00 (86 Dossiers + 4 Nachprüfungen) **plus** die
Korrekturen der Vorprüfung **plus** ein **Builder-Fix** im Seed-Codegen.

## Der Builder-Fix (der eigentliche Fund)

`crates/ingestion/build.rs` → `timeline_notes()` **trimmt** jede Zeile, bevor sie auf den
Bullets-Präfix `- ` geprüft wird. Dadurch wurde jede **eingerückte** Aufzählungszeile
(`  - …`) zu einem **eigenen** Bullet, und die Notiz bekam einen zusätzlichen ` | `-Trenner
mitten im Satz. Der Prüfer in `scripts/release/check_note_order.py` hat die eingerückte Zeile
schon immer als **Fortsetzungszeile** behandelt — Builder und Prüfer widersprachen sich.

Der Builder liest jetzt die **Rohzeile**: eine eingerückte Zeile ist eine Fortsetzungszeile,
auch wenn sie mit `- ` beginnt.

**Wirkung:** 2 Dossiers **außerhalb** des Wellen-Scopes (`by-munchen-r-muller`,
`he-gernsheim-64579-ca-recycling`) verlieren den falschen Trenner (` | ` → `-`); die
Notizlänge bleibt unverändert, der Satz bleibt ein Satz.

## Weitere Korrekturen aus der Vorprüfung

| Dossier | Befund | Korrektur |
|---|---|---|
| `nw-koln-wkr` | Der Vermerk schrieb Rodenkirchen die Öffnungszeiten „08:00–16:30“ zu — die gehören **Longerich** | berichtigt: Rodenkirchen 08:00–16:00 + Samstag geschlossen (live bestätigt); `opening_hours` war bereits korrekt |
| `mv-carinerland-18236-friehold-recycling` | Quelle 2 (Register-Spiegel) war nur im Fließtext genannt | `https://order.firmenkontor24.com/search/HRB/9677` jetzt im `Quelle(n)`-Block |
| `nw-koln-rodenkirchen-mullenmeister-schwerfel` | „268 Adressobjekte“ (zählte Randdopplungen) | 256 verschiedene Objekte (268 Nodes im Raster), 234 Hausnummern |
| `st-salzatal-uhlmann` | „43 Adressobjekte“ | 41 Adressobjekte + 2 Straßenobjekte (43 Objekte im Raster) |

## Ein Vorwellen-Defekt behoben

`nw-lippstadt-dorozala` enthielt den Vermerk vom 23.10.2026 **zweimal wortgleich** (Versehen des
Wellenlaufs vom 23.10.). Die identische Kopie wurde als **eigener Commit** mit datiertem
Korrekturvermerk entfernt — die Aussage selbst bleibt **vollständig** erhalten. Grund für den
eigenen Commit: die Präfix-Regel des Gates erlaubt keine Entfernung innerhalb der Welle.
`check_pre_wave_notes.py` (mitgliedschaftsbasiert) ist mit beiden Basen grün.

## Unabhängige Pre-Deploy-Freigabe (GREEN)

Prüfer: eigener Prüf-Agent, read-only, Arbeitsverzeichnis `/root/Documents/vclone-mech`,
Kandidat `3ffdf8b`. Belege in `/tmp/opencode/mech65/`.

| Prüfpunkt | Ergebnis |
|---|---|
| Builder-Fix | (a) eingerückte `- `-Zeile = Fortsetzungszeile (zeichengenauer Diff, 1 Zeichen je Dossier); (b) Spalten-0-Bullet bleibt eigener Bullet — Builder-Erwartung == Seed == Bundle-DB für **alle 3945** Dossiers, **0** Abweichungen; (c) **keine** Regression — die 10 Dossiers mit Builder-/Prüfer-Abweichung sind in Basis und Kandidat **identisch** (Spalten-0-Prosazeilen), alle außerhalb des Scopes |
| Die 43 Koordinaten live | **43/43** live: HTTP 200, Mittel der eindeutigen Knoten == Dossier (2 Rundungen 1e-7), Punkt in der BBox 43/43, behauptete Knotenzahl und BBox 43/43 == live, **zwei** Quellen 43/43; **0** Abweichungen |
| Die redaktionellen Korrekturen | WKR live per DOM-Zuordnung bestätigt (Longerich 08:00–16:30, Rodenkirchen 08:00–16:00 + Sa geschlossen), `opening_hours` und `dropoff_json` deckungsgleich; Register-URL im Quellenblock; Objektzahlen live nachgerechnet (268/256/234 und 41+2/43) — **exakt wie vermerkt** |
| Der Vorwellen-Defekt | Basis 2× wortgleich (je 1091 Zeichen, byte-identisch), Kandidat 1× byte-identisch zum Basisblock, Korrekturvermerk vorhanden, Bundle-DB 1× + Vermerk 1×; `check_note_order` meldet **genau den einen** erwarteten Präfix-Befund, `check_pre_wave_notes` exit 0 |
| Struktur | alle drei Gates mit der Wellen-Basis **exit 0**; mit der Release-Basis nur der eine erwartete Befund; `lat`/`lon` 43/43 (44/44 mit dem belegten Paar) direkt nach `slug:`; 0 Konfliktmarker; **0** doppelte Notizen (Basis und Kandidat) |
| Bundle | head/tree korrekt; Baum **byte-identisch 6088/6088**, 0 Hash-Differenzen; 809/0/11; fmt exit 0; `full_production_unchanged` true (Live-Fingerabdruck `5341b8b8…a806` == before == after); `unexplained_changes` []; `changed_trader_count` **89** mit `{notes: 89, lat: 43, lon: 43, street: 1, postcode: 1, phone: 1}` (0 Abweichungen je Zeile; 87 im Scope + 2 außerhalb); `clear_directive_check` 309/0; deployment/restart/write false; Boot-Idempotenz 89 → 0 |
| Live unverändert | Binaries unverändert `b24cc5ba…`/`c1051b57…`, Dienst-Start 03.11. 23:20:16 UTC, NRestarts 0, Live trägt `03a0af3`, 5 Backup-Dirs (kein neues), Bundle-Binaries ≠ installierte, Kandidaten-Verifier gegen Live **fehlschlagend** (exit 1) |
| Verifier | release-delta (**89**), wave-e (**60**), pending (124 Meldungen) je **exit 0**, 0 Fehler; Vertragstests 7 + 8 + 9 OK |

## Rollout-Ablauf

| Schritt | Wert |
|---|---|
| Backup | `rollout-3ffdf8b-20261104T085928Z`, Quelle: SQLite-Backup-API aus einer `mode=ro`-Verbindung — **kein** Produktions-Write |
| Binaries | Server `ef0a27f862d359336f841dcb103dc8520c76ecae825b08733e9c2360aeb7c682`, Query-Worker `06c059069c640b8463ce270e4b2d52fd5140b01a543369fabc04d786a6491ab9` |
| Neustart | 04.11. 08:59:29 UTC |
| Health | `{"ok":true,"service":"schrott-mcp"}`, HTTP 200, `ActiveState=active` |
| Natürlicher Scheduler-Lauf | `ingestion_runs` id 3673, 08:59:45 UTC, `status=ok`, „Katalog aktuell, 0 Preise übernommen, **0 Händler-Fehler**“ |
| `NRestarts` | 0 |

## Read-only Produktionsabgleich nach dem Rollout

* **44/44** Koordinaten live bestätigt (43 neue + 1 re-bestätigte), **0** Abweichungen.
* **3636 von 3945** Händlern tragen jetzt eine Koordinate (vor der Welle: 3593).
* Der **Builder-Fix** ist live wirksam: `by-munchen-r-muller` und `he-gernsheim-64579-ca-recycling`
  tragen die Fortsetzungszeichen **ohne** den falschen ` | `-Trenner.
* `nw-koln-wkr` steht auf **Kirschbaumweg 8, 50996, 0221 97060622** ✓ (live bestätigt).
* Zähler: 3945 Händler · 83 Materialien · 41 570 Preise · 2938 `current_prices`.
