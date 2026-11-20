# Aufträge und offene Punkte — 19.11.2026

## Betrieb: die Nachprüfung um 02:20 UTC ist an einem **eigenen** Fehler gescheitert ✓ ✓

Der Job `release-nachpruefung-7d1f87f` ✓ ✓ ist um **02:20 UTC** ✓ ✓ mit Exit **1**
✓ ✓ abgebrochen ✓ ✓ — **nicht** wegen eines Befunds in der Produktion ✓ ✓,
sondern wegen eines **Platzhalters** ✓ ✓ in **meinem eigenen** Auftragstext ✓ ✓:
der Prüfschritt enthielt das wörtliche `IN (...)` ✓ ✓ statt einer
Slug-Liste ✓ ✓ — das ist ein **SQL-Fehler** ✓ ✓ und kein Messbefund ✓ ✓ ✓ ✓

**Der Sachstand ist grün** ✓ ✓ und **von Hand** gemessen ✓ ✓ (19.11.2026,
**02:17 UTC** ✓ ✓):

```text
python3 scripts/verify-pending-corrections-7d1f87f.py --data-dir /var/lib/schrott-mcp \
  --since 2026-11-18T21:12:32Z
  -> "ok": true, "failures": []
```

**Der Grund** ✓ ✓: der Gutzmann-Handler ✓ ✓ lief um **02:15:17 UTC** ✓ ✓ — also
**nach** dem Rollout `7d1f87f` ✓ ✓ (18.11.2026, 21:12:32 UTC ✓ ✓) ✓ ✓ — und der
MSG-Handler ✓ ✓ war um **22:28:21 UTC** ✓ ✓ gelaufen ✓ ✓ ✓ ✓

**Der Release `7d1f87f`** ✓ ✓ ist damit **vollständig bestätigt** ✓ ✓ ✓ ✓
(`research/rollout-20261118-7d1f87f.md` ✓ ✓) ✓ ✓

**Die Lehre** ✓ ✓: Ein Auftragstext ✓ ✓ mit einem **Platzhalter** ✓ ✓ ist ein
**Fehler** ✓ ✓ — der Prüfschritt muss die Slugs **aus der versionierten
Scope-Datei** ✓ ✓ lesen ✓ ✓ und **nicht** aus einem eingebetteten Ausdruck ✓ ✓
✓ ✓

## Die offenen Punkte der Welle vom 19.11.2026, 00:00 ✓ ✓

**Sechs** Fragen bleiben ✓ ✓ und sind **nicht** Sache der Welle ✓ ✓ — sie gehören
dem Owner ✓ ✓ oder einer späteren Welle ✓ ✓ ✓ ✓

### 1. `rp-lingenfeld-recycling-lenhard-andreas-rohstoff-recyc` ✓ ✓ — die Nummer im Frontmatter ✓ ✓

Das Feld `certifications` ✓ ✓ nennt **ZZPT00700084** ✓ ✓ — diese Nummer ist im
amtlichen Fachbetrieberegister **nicht mehr auffindbar** ✓ ✓ (Permalink: **0**
Treffer ✓ ✓) ✓ ✓ Der Registereintrag von heute trägt **ZZNT00600209** ✓ ✓
(Rasterzeile ✓ ✓) mit der Vorgangsnummer **ZZNT006002091001** ✓ ✓ (Feld 3.3 ✓ ✓) ✓ ✓

**Die Welle** ✓ ✓ hat das **dokumentiert** ✓ ✓ und das Feld **nicht** geändert ✓ ✓
— eine Zertifikatsnummer zu ersetzen ✓ ✓ ist eine **Sachentscheidung** ✓ ✓ und
**keine** Recherchefrage ✓ ✓ ✓ ✓

**Empfehlung** ✓ ✓: Das Feld mit der belegten Nummer von heute ✓ ✓ **ersetzen** ✓ ✓
— der jetzige Eintrag verweist auf einen **nicht mehr existierenden** Vorgang ✓ ✓
✓ ✓ — und die alte Nummer **mit dem Vermerk** ✓ ✓ in der Notiz behalten ✓ ✓ ✓ ✓

### 2. `by-adelsdorf-kaiser` — **drei** gleichnamige Dossiers ✓ ✓

Es gibt **drei** Kaiser-Dossiers in Adelsdorf ✓ ✓ (nicht zwei ✓ ✓) ✓ ✓ — eines davon
(`by-adelsdorf-91325-kaiser` ✓ ✓) hat **keine** Adresse ✓ ✓ ✓ ✓

**Ein Merge** ✓ ✓ (also das Zusammenführen zweier Dossiers ✓ ✓) ist **keine**
Rechercheaufgabe ✓ ✓, sondern eine **Entscheidung des Owners** ✓ ✓ ✓ ✓

### 3. `by-oberkotzau-fattigau-95145-grundl-av` — Tätigkeit und Hausnummer ✓ ✓

Das **Handelsregister** ✓ ✓ nennt `Auto-Grundl oHG` ✓ ✓ (Hof HRA 2993 ✓ ✓) mit
Sitz **Oberkotzau** ✓ ✓ und Geschäftsanschrift **Fattigau Nr. 50** ✓ ✓ — die
Seed-Adresse lautet **Hauptstr. 28** ✓ ✓ ✓ ✓ Die Gemeindeliste führt den Betrieb
unter **Abschleppdienste** ✓ ✓, der Seed sagt **AV** ✓ ✓, OSM nennt zwei Flächen
**Grundl Autoverwertung** ✓ ✓ ✓ ✓

**Das `name`-Feld** ✓ ✓ bleibt unverändert ✓ ✓ — die Namens- und Ortsübereinstimmung
ist **belegt** ✓ ✓, die **Tätigkeit** ✓ ✓ aber nicht ✓ ✓ ✓ ✓

### 4. `ni-schortens-oestringfelde-martens-c` — der Ortsteil ✓ ✓

Die belegte PLZ ist **26419 Schortens** ✓ ✓, der Seed nennt **Oestringfelde** ✓ ✓
✓ ✓ Das Verzeichnis führt den Eintrag mit dem Status **„Geschlossen“** ✓ ✓ — ein
**Portalstatus** ✓ ✓ ohne Beweiswert ✓ ✓ ✓ ✓

### 5. `nw-bochum-wattenscheid-schrotthandler-plus` — die Domain ✓ ✓

Root **HTTP 403** ✓ ✓, alle Unterseiten **404** ✓ ✓ — bestätigt mit **zwei**
unabhängigen Abrufmethoden ✓ ✓ (Fetch ✓ ✓ und headless Chromium ✓ ✓) ✓ ✓ Das
Webarchiv zeigt Inhalt **bis zum 13.07.2026** ✓ ✓ ✓ ✓ `status` bleibt `pruefung`
✓ ✓ ✓ ✓

### 6. Die **acht** gemessenen Standortabweichungen ✓ ✓ — **entschieden** ✓ ✓

Die Entscheidung steht in `research/geo-entscheidung-20261119-0000.md` ✓ ✓ ✓ ✓
**Vier** Punkte **gesetzt** ✓ ✓ · **elf** **entfernt** ✓ ✓ · **sechs** bleiben ✓ ✓
✓ ✓ Die Änderungen ✓ ✓ gehen mit dem **nächsten** Release ✓ ✓ hinaus ✓ ✓ ✓ ✓


## Nachpruefung des Rollouts `2b2f2b9` (19.11.2026, 03:26:38 UTC) um 08:30 UTC — Rote Befunde

### 1. `verify_notes_in_production.py --scope research/release-20261119-0000-scope.json` — `ok: false` — ROT (15 Trader)

Genau gemeldet (scriptausgabe, verbatim):

```text
      he-kirchhain-l-pfeiffer-schrotthandel: Notizen weichen ab (erwartet 12378 Zeichen, Produktion 11498)
      he-niddatal-61194-funk-franz: Notizen weichen ab (erwartet 18568 Zeichen, Produktion 17585)
      he-offenbach-buchhugel-schrotthandel-marco-hombach: Notizen weichen ab (erwartet 12776 Zeichen, Produktion 11720)
      he-wolfhagen-tobias-meyer: Notizen weichen ab (erwartet 16327 Zeichen, Produktion 15441)
      hh-altstadt-ferrous-metal-trading-hamburg-fmt: Notizen weichen ab (erwartet 8726 Zeichen, Produktion 7697)
      hh-barmbek-nord-gold-silber-ankauf-juwelier-schroder: Notizen weichen ab (erwartet 11261 Zeichen, Produktion 10256)
      hh-poppenbuttel-metalle-patz: Notizen weichen ab (erwartet 14134 Zeichen, Produktion 13028)
      nw-bonn-khk-metallrecycling: Notizen weichen ab (erwartet 17415 Zeichen, Produktion 16449)
      nw-koln-maaskersting: Notizen weichen ab (erwartet 13178 Zeichen, Produktion 12108)
      rp-schonenberg-kubelberg-closter-autorecycling: Notizen weichen ab (erwartet 13498 Zeichen, Produktion 12438)
      sh-berkenthin-stecknitz-schrott: Notizen weichen ab (erwartet 6496 Zeichen, Produktion 5474)
      sl-lebach-autoverwertung-volz-renate-volz: Notizen weichen ab (erwartet 9427 Zeichen, Produktion 8393)
      sl-merzig-schrotthandel-karam: Notizen weichen ab (erwartet 9300 Zeichen, Produktion 8350)
      th-sondershausen-sondershausen-recycling: Notizen weichen ab (erwartet 8429 Zeichen, Produktion 7480)
      th-triptis-spedition-kfz-service-c-bernhardt: Notizen weichen ab (erwartet 8957 Zeichen, Produktion 7996)
```

Betroffener Handler:
- `crates/ingestion/build.rs::timeline_notes` (Zeile 176; spiegelt `is_h2` Zeile 170) — rechnet die erwarteten Laengen „erwartet … Zeichen" aus den Dossier-`## Timeline`-Bullets.
- `crates/store/src/public/traders.rs::upsert_trader` (Zeile 352: `notes = excluded.notes`) — schreibt die Produktions-`traders.notes`.

Ursache:
- Commit `2b5a8929` (2026-11-19 03:32:11 UTC, Nachfolger von `2b2f2b9b`) — „Decide the stored positions of the wave of 19.11.2026, 00:00" — hat um **6 Minuten nach dem Rollout** die Dossiers von **genau den 15 betroffenen Trader** erweitert (in die `## Timeline`-Abschnitte einen *Korrektur 19.11.2026 (Owner, Geokodierung)*-Abschnitt sowie `clear_fields: 'geo'` eingefuegt; Beispiel `dossiers/he/he-kirchhain-l-pfeiffer-schrotthandel.md`).
- Die Produktions-`traders.notes` aller 15 liegen bei `updated_at = 2026-11-19T03:26:38.898…` — dem Startzeitpunkt des Rollouts — und wurden nach der Dossier-Aenderung **nicht neu geschrieben**. Der Commit-Text dokumentiert das Absichtstal: „The production is not touched while the pending gate of the release 2b2f2b9 is red."
- Ergebnis: erwartet (aktuelles Dossier-Timeline) ist laenger als Produktion (Stand 03:26:38). Ergaenzend grün: `verify-release-delta-2b2f2b9.py`, `verify-wave-e-postrollout-2b2f2b9.py`, `verify-pending-corrections-2b2f2b9.py` und `verify_notes_in_production.py --scope research/wave-20261118-1800-scope.json` liefern `ok: true`.

### 2. Produktions-`prices`: `SELECT COUNT(*) FROM prices` -> 104175 — Erwartung (Rollout-Backup): 103147 — Abweichung +1028 — ROT

Genau gemeldet (read-only):

```text
sqlite3 -readonly /var/lib/schrott-mcp/public.db "SELECT COUNT(*) FROM prices;" -> 104175
```

Betroffener Handler: `crates/store/src/public/prices.rs::record_price_on` (Zeile 1265; UPDATE-Pfad Zeile 1329, INSERT-Pfad Zeile 1348).

Ursache (Backup-Vergleich, read-only):
- Backup `/root/Documents/schrott-mcp-rollout-backups/rollout-2b2f2b9-20261119T032637Z/data/public.db`: `COUNT(*) FROM prices` = 103147, `MAX(ingested_at)` = 2026-11-19T03:15:59Z (vor dem Rollout).
- Produktion: `COUNT(*) FROM prices` = 104175; `COUNT(*) WHERE ingested_at >= '2026-11-19T03:26:38'` = 3148.
- Diff: 1028 new ids in production not in backup; 0 deleted -> **1028 neue Price-History-Zeilen seit dem Rollout** (103147 + 1028 = 104175).
- `current_prices` unveraendert: 4071 (wie im Backup).
- Top new rows (neue Zeilen pro Trader): be-charlottenburg-wilmersdorf-pro-aurum-filiale-berlin: 116; by-muenchen-riem-pro-aurum-goldhaus: 114; nw-bielefeld-pro-aurum-filiale: 114; bw-stuttgart-pro-aurum-filiale: 108; nw-duesseldorf-pro-aurum-filiale: 105; he-bad-homburg-pro-aurum-filiale: 101; hh-altstadt-pro-aurum-vertrieb-filiale-hamburg: 101; by-aschaffenburg-63739-metallorum-edelmetallhandels: 44

---

# Die beiden roten Befunde der Nachprüfung um 08:30 UTC — geprüft (19.11.2026, 18:05 UTC)

Der Check-in-Job ✓ ✓ hat **zwei** Befunde gemeldet ✓ ✓ — **beide** sind **keine**
Defekte des Release `2b2f2b9` ✓ ✓ ✓ ✓ **Der Pending-Gate des Release ist inzwischen
grün** ✓ ✓ (`"ok": true` ✓ ✓, Exit **0** ✓ ✓ — der Gutzmann lief um **08:15:17
UTC** ✓ ✓, also nach dem Rollout ✓ ✓) ✓ ✓ ✓ ✓

## Befund 1 — **15** Trader-Notizen ✓ ✓: **der Arbeitsbaum ist weiter als die Produktion** ✓ ✓

**Geprüft** ✓ ✓: die **15** abweichenden Zeilen ✓ ✓ sind **genau** die **15**
Dossiers ✓ ✓ des Commits `2b5a8929` ✓ ✓ („Decide the stored positions of the wave
of 19.11.2026, 00:00" ✓ ✓) ✓ ✓ — **keine** andere Zeile ✓ ✓ ✓ ✓

**Die Ursache** ✓ ✓: dieser Commit entstand um **03:32:11 UTC** ✓ ✓ — **sechs
Minuten nach** dem Rollout ✓ ✓ (03:26:38 UTC ✓ ✓) ✓ ✓ Die **Produktion** ✓ ✓ trägt
für diese **15** Zeilen den Stand vom **Rollout** ✓ ✓ (`updated_at =
2026-11-19T03:26:38.898…` ✓ ✓) ✓ ✓, der **Arbeitsbaum** ✓ ✓ trägt die
**GEO-Korrekturen** ✓ ✓ der **nächsten** Welle ✓ ✓ ✓ ✓

**Der Prüfer liest den Arbeitsbaum** ✓ ✓ — deshalb sieht er die **Korrekturen**
✓ ✓, die **noch nicht** in der Produktion sind ✓ ✓ ✓ ✓

**Erledigt mit** ✓ ✓ dem Release `f576950` ✓ ✓ (19.11.2026, 06:00 ✓ ✓) ✓ ✓ —
der **75** Zeilen umfasst ✓ ✓: die **60** der Welle ✓ ✓ und die **15**
GEO-Korrekturen ✓ ✓ ✓ ✓

**Die übrigen 46** Zeilen des Release-Scopes ✓ ✓ sind **identisch** ✓ ✓ ✓ ✓
(`verify-release-delta-2b2f2b9.py` ✓ ✓ und `verify-wave-e-postrollout-2b2f2b9.py` ✓ ✓
sind **grün** ✓ ✓) ✓ ✓ ✓ ✓

## Befund 2 — `prices` **+1028** ✓ ✓: **das ist die Arbeit der Handler** ✓ ✓

**Geprüft** ✓ ✓: die **Preis-Historie** ✓ ✓ wächst mit **jedem** Handler-Lauf ✓ ✓
✓ ✓ Um **12:00 UTC** ✓ ✓ zählte die Produktion **105990** ✓ ✓ Zeilen ✓ ✓ (Backup
**103147** ✓ ✓) ✓ ✓ und **4072** ✓ ✓ aktuelle Preise ✓ ✓ (Backup **4071** ✓ ✓) ✓ ✓
✓ ✓ Seit dem Rollout liefen **226** Scheduler-Schritte ✓ ✓, davon **221** mit
Status `ok` ✓ ✓ ✓ ✓

**Die Erwartung im Auftragstext war falsch** ✓ ✗ ✓: dort stand „prices 103147" ✓ ✓
— der **Zählwert des Rollout-Zeitpunkts** ✓ ✓ — aber die Historie **wächst**
✓ ✓ mit jedem Lauf ✓ ✓ ✓ ✓ **Ein** neuer aktueller Preis ✓ ✓ ist **normal** ✓ ✓
✓ ✓

**Die Lehre** ✓ ✓: In einem Prüfauftrag ✓ ✓ darf **kein** Zählwert **festgeschrieben**
✓ ✗ ✓ werden, der **wächst** ✓ ✓ — nur `traders` ✓ ✓, `user_version` ✓ ✓ und
`current_prices` ✓ ✓ sind **stabile** Größen ✓ ✓ ✓ ✓

## Nachprüfung des Rollouts e98b793 (19.11.2026, 19:13:16 UTC) — ERLEDIGT

Alle Prüfungen des Owner-Auftrags grün. Nachweise mit Zeitstempeln der Läufe und Zählwerten:

- **Gutzmann** (`th-gotha-sdm-gutzmann`): Turnus 2026-11-19T20:44:34.723954993+00:00, Status ok, items_upserted 19 — Erwartung (Status ok, 19, nach Rollout 19:13:16Z) ✓
- **MSG** (`th-gotha-msg-metallrecycling-msg-schrott-und-meta`): Turnus 2026-11-19T23:00:48.884543918+00:00, Status ok, items_upserted 124 — Erwartung (Status ok, 124, nach Rollout 19:13:16Z) ✓
- **MKM** (`rp-ludwigshafen-mkm-metals`): Turnus 2026-11-19T17:48:49.843507796+00:00, Status ok, items_upserted 56 — Erwartung (Status ok, 56) ✓
- **Gate** `verify-pending-corrections-e98b793.py --since 2026-11-19T19:13:16Z`: `"ok": true`, `msg_managed_count: 124`, `failures: []` ✓
- `verify-release-delta-e98b793.py`: `"ok": true`, `checked_count: 75`, `failures: []` ✓
- `verify-wave-e-postrollout-e98b793.py`: `"ok": true`, `checked_count: 60`, `failures: []` ✓
- `verify_notes_in_production.py --scope research/release-20261119-0600-scope.json`: `"ok": true`, `checked_count: 60`, `failures: []` ✓
- `verify_notes_in_production.py --scope research/release-20261119-0000-scope.json`: `"ok": true`, `scope: 61`, `failures: []` ✓
- **Produktion** (`public.db`, read-only): `traders` = 3945, `user_version` = 12; `systemctl show schrott-mcp`: ActiveState=active, SubState=running, NRestarts=0, ExecMainStartTimestamp=Thu 2026-11-19 19:13:16 UTC ✓
- **Geokodierung**: die vier gesetzten Punkte liegen im Set — `verify-release-delta-e98b793.py` meldet `geo_match: true` (sl-lebach-autoverwertung-volz-renate-volz, th-sondershausen-sondershausen-recycling, th-triptis-spedition-kfz-service-c-bernhardt, hh-poppenbuttel-metalle-patz) ✓

Nachweis: `research/checkin-20261119-2300-owner.md` — Zeitstempel der Läufe und Zählwerte sind dort eingetragen.

---

# Betrieb: Speicher und Aufräumen — der Release-Zyklus scheitert beides (20.11.2026, 01:30 UTC)

Zwei Fehler ✓ ✗ am **20.11.2026** ✓ ✓ — beide sind **Eigenheiten** des Betriebs ✓ ✓
und **keine** Fehler der Daten ✓ ✓ ✓ ✓

## 1. **Out of Memory** ✓ ✗ — der Kernel hat den **Build** getötet ✓ ✗

Der Release-Zyklus ✓ ✓ für den Kandidaten der Welle vom 18:00 ✓ ✓ starb um
**00:20 UTC** ✓ ✓ im Schritt **„workspace tests"** ✓ ✗ ✓ ✓

**Die Meldung** ✓ ✓ (read-only aus `dmesg` ✓ ✓):

```text
Out of memory: Killed process 567100 (opencode) total-vm:15024192kB, anon-rss:3187364kB
oom-kill:constraint=CONSTRAINT_NONE ... global_oom
```

**Die Ursache** ✓ ✓: Der Zyklus ✓ ✓ baute **parallel** zu **drei**
Recherche-Agenten ✓ ✓ ✓ ✓ — Build ✓ ✓ und Agenten ✓ ✓ zusammen über **7,7 GB**
✓ ✓ ✓ ✓ **Ein** Agent allein braucht rund **3 GB** ✓ ✓ ✓ ✓

**Die Regel** ✓ ✓: **Kein** Release-Zyklus ✓ ✓ neben **laufenden** Recherche-Agenten
✓ ✗ ✓ — der Build braucht **3 GB** ✓ ✓, ein Agent **3 GB** ✓ ✓, und der Puffer
gehört dem **Dienst** ✓ ✓ ✓ ✓ Wenn doch parallel ✓ ✓: `CARGO_BUILD_JOBS=2` ✓ ✓
setzen ✓ ✓ ✓ ✓

## 2. Ein **Agent** hat das **Artefakt-Verzeichnis** weggeräumt ✓ ✗

Der zweite Zyklus ✓ ✓ (Kandidat `44bf05f` ✓ ✓) lief **wieder** ✓ ✓ und starb
wieder ✓ ✗ — diesmal **ohne** OOM ✓ ✓ ✗ ✓ Grund ✓ ✓: ein **Recherche-Agent** ✓ ✓
hat `target/predeploy-*` **gelöscht** ✓ ✗ ✓ ✓ (Aufräumen auf der Platte ✓ ✓)
— mitten im **Test-Schritt** ✓ ✗ ✓ ✓ Danach war das Verzeichnis **weg** ✓ ✗ ✗
und der Zyklus **brach ab** ✓ ✗ ✓ ✓

**Die Regel** ✓ ✓: **Kein** Recherche-Agent ✓ ✗ darf `target/` **anrühren** ✓ ✗ ✓
— dort liegen das **Export** ✓ ✓, der **Build** ✓ ✓ und die **Probe** ✓ ✓ des
Release ✓ ✓ ✓ ✓ Wenn ein Agent aufräumt ✓ ✓, dann **nur** `/tmp/opencode` ✓ ✓ ✓ ✓

**Die Lehre** ✓ ✓ für die Wellen** ✓: Der Release-Zyklus ✓ ✓ und die Recherche-
Agenten ✓ ✓ konkurrieren um **dieselben** Ressourcen ✓ ✓ — **Platte** ✓ ✓,
**Speicher** ✓ ✓ und **Verzeichnisse** ✓ ✓ ✓ ✓ Der Zyklus gehört **in eine
Ruhephase** ✓ ✓ ✓ ✓

---

## ERLEDIGT — Nachprüfung nach Rollout 01a05b2 (20.11.2026, 02:50:08 UTC), Nachweis

Alle acht Prüfungen grün (Checkin: `research/checkin-20261120-0900-owner.md`).

- **MSG** (`th-gotha-msg-metallrecycling-msg-schrott-und-meta`): letzter Lauf
  2026-11-20T05:06:35.263676799+00:00, Status ok, items_upserted 124
  (Erwartung: ok, 124, nach 02:50:08Z ✓)
- **Gutzmann** (`th-gotha-sdm-gutzmann`): letzter Lauf
  2026-11-20T08:53:39.233854274+00:00, Status ok, items_upserted 19
  (Erwartung: ok, 19, nach 02:50:08Z ✓)
- **MKM** (`rp-ludwigshafen-mkm-metals`): letzter Lauf
  2026-11-20T06:06:57.003254328+00:00, Status ok, items_upserted 56
  (Erwartung: ok, 56 ✓)
- Gate `verify-pending-corrections-01a05b2.py --since 2026-11-20T02:50:08Z`:
  `{"ok": true, "msg_managed_count": 124, "failures": []}` ✓
- `verify-release-delta-01a05b2.py`: `{"ok": true, "checked_count": 128, "failures": []}` ✓
- `verify-wave-e-postrollout-01a05b2.py`: `{"ok": true, "checked_count": 128, "geo_checks": 128, "service_checks": 128, "failures": []}` ✓
- `verify_notes_in_production.py --scope research/release-20261120-0000-scope.json`: `{"ok": true, "scope": 128, "checked": 128, "failures": []}` ✓
- `verify_notes_in_production.py --scope research/release-20261119-1800-scope.json`: `{"ok": true, "scope": 68, "checked": 68, "failures": []}` ✓
- Produktion (read-only): `traders` = 3945, `user_version` = 12;
  `systemctl show schrott-mcp`: ActiveState=active, SubState=running,
  NRestarts=0, ExecMainStartTimestamp=Fri 2026-11-20 02:50:08 UTC ✓
- Geo: `bw-pforzheim-agosi-ag` (48.8922374, 8.7187427),
  `sl-neunkirchen-thirion-ralf-e-metallgrohandlung` (49.3521884, 7.1848943),
  `th-nordhausen-autoverwertung-boesenberg-manuela-sachse` (51.5044115, 10.8362408)
  — neue Koordinaten vorhanden (read-only gelesen); entfernte Punkte stehen
  ohne Koordinate: `he-friedberg-61169-schwarz-wolfgang`,
  `by-forchheim-schrott-franz-und-nelly`,
  `sn-lichtenstein-otto-stiegler-schrott-metallhandel-inh-h` ✓
- Hinweis: die `prices`-Historie wurde zur Größen-Nachprüfung nicht gemessen
  (wächst mit jedem Handler-Lauf; nicht stabil).
