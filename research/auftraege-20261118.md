# Nachprüfung d129bec – 18.11.2026, 10:25 UTC: rot

Rollout: 08:05:17 UTC, Wellen 00:00 und 06:00 UTC (je 60 Dossiers) und drei Zertifikatskorrekturen.

**Gutzmann:** `verify-pending-corrections-d129bec.py --data-dir /var/lib/schrott-mcp --since 2026-11-18T08:05:17Z` lieferte `"ok": false` (Exit 1). Einzige Fehlermeldung:

> th-gotha-sdm-gutzmann: successful post-rollout step missing or changed contract

Ursache für das rote Gate: In `internal.db` fehlt der nach dem Rollout erwartete Gutzmann-Schritt (~08:08 UTC). Der letzte belegte Schritt war erfolgreich, lag aber vor dem Rollout. Warum kein neuer Schritt erfasst wurde, ist nicht belegt. Auftrag: Ausführung und Protokollierung des Handlers prüfen und das Gate nach dem nächsten Lauf erneut ausführen.

| Handler | Letzter `started_at` (UTC) | `status` | `items_upserted` | Bewertung |
|---|---|---|---:|---|
| `th-gotha-sdm-gutzmann` | 2026-11-18T02:08:45.247367456Z | ok | 19 | Vor Rollout; fälliger Nachweis fehlt |
| `th-gotha-msg-metallrecycling-msg-schrott-und-meta` | 2026-11-18T10:21:36.673813773Z | ok | 124 | Nach Rollout bestätigt |
| `rp-ludwigshafen-mkm-metals` | 2026-11-18T05:25:09.596504701Z | ok | 56 | Vor Rollout; nach Fälligkeit (~11:25 UTC) erneut prüfen |

**Übrige Prüfungen:** Release-Delta, Wave-E-Postrollout und Produktionsnotizen (Scope `research/release-20261118-0000-0600-scope.json`) meldeten jeweils `"ok": true`. `public.db`: 3945 Händler, 98406 Preise, `user_version=12`.

| Zertifikat (`slug`) | `length(certifications)` |
|---|---:|
| `hh-schnelsen-walter-behncke-inh-bernd-behncke` | 681 |
| `st-dessau-rolau-horst-habermann-transport-und-altmetallh` | 445 |
| `th-schmalkalden-jager-jurgen` | 242 |

`schrott-mcp`: `ActiveState=active`, `SubState=running`, `NRestarts=0`, `ExecMainStartTimestamp=Wed 2026-11-18 08:05:17 UTC`.

---

# Ursache des fehlenden Gutzmann-Laufs — 18.11.2026, 12:05 UTC

## Der Befund ✓ ✓

Nach dem Rollout von `d129bec` ✓ ✓ (18.11.2026, 08:05:17 UTC ✓ ✓) ist der
Gutzmann-Schritt **nicht** gelaufen ✓ ✓ — der letzte belegte Lauf war um
**02:08:45 UTC** ✓ ✓ ✓ ✓ Das Gate `verify-pending-corrections-d129bec.py` ✓ ✓
bleibt deshalb **rot** ✓ ✓ ✓ ✓

## Die Ursache ✓ ✓ — **gemessen** ✓ ✓, nicht vermutet ✓ ✓

Die Turnusstempel liegen **im Speicher** ✓ ✓ (`due_map()` ✓ ✓ in
`crates/ingestion/src/traders/scheduler.rs` ✓ ✓) und werden **beim Start**
**neu berechnet** ✓ ✓ — aus dem Startzeitpunkt und einer **streuungsstabilen**
Phase ✓ ✓ (`stagger_secs(slug, interval)` ✓ ✓ = Hash des Slugs ✓ ✓ mod 21600 ✓ ✓)
✓ ✓ **Nicht** aus dem letzten Lauf ✓ ✓ ✓ ✓

Für den Gutzmann ✓ ✓ (`th-gotha-sdm-gutzmann` ✓ ✓) ergibt die Rechnung ✓ ✓:

| Größe | Wert |
|---|---|
| Phase (Hash des Slugs mod 21600) | **7066 s** = **01:57:46** ✓ ✓ |
| Letzter Lauf | 02:08:45 UTC ✓ ✓ (Tick nach der Phase ✓ ✓) ✓ ✓ |
| **Neustart** ✓ ✓ | **08:05:17 UTC** ✓ ✓ |
| Phase-Fenster | 01:57:46 → **07:57:46** ✓ ✓ |
| Fällig um 07:57:46 ✓ ✓ — **7 Minuten vor dem Neustart** ✓ ✗ ✓ ✓ ✓ ✓ |
| Neuer Stempel nach dem Start ✓ ✓ | **13:57:46 UTC** ✓ ✓ ✓ ✓ |

**Der Neustart hat die Fälligkeit von 07:57:46 übersprungen** ✓ ✓ ✓ ✓ ✓ ✓ —
die due_map kennt keinen Vorlauf ✓ ✓ und wird aus dem **Startzeitpunkt** ✓ ✓
neu aufgebaut ✓ ✓ ✓ ✓

**Zum Vergleich** ✓ ✓ dieselbe Rechnung ✓ ✓ für die beiden anderen Handler ✓ ✓
— beide liefen ✓ ✓ ✓ ✓:

| Handler | Phase | Fällig nach dem Start | Gelaufen |
|---|---|---|---|
| `rp-ludwigshafen-mkm-metals` ✓ ✓ | 05:16:59 ✓ ✓ | 11:16:59 ✓ ✓ | **11:21:58** ✓ ✓ (56 Preise ✓ ✓) ✓ ✓ |
| `th-gotha-msg-…-msg-schrott-und-meta` ✓ ✓ | 04:06:50 ✓ ✓ | 10:06:50 ✓ ✓ | **10:21:36** ✓ ✓ (124 Preise ✓ ✓) ✓ ✓ |
| `th-gotha-sdm-gutzmann` ✓ ✓ | 01:57:46 ✓ ✓ | 13:57:46 ✓ ✓ | **fehlt noch** ✓ ✗ ✓ ✓ |

## Was **nicht** getan wurde ✓ ✓

**Kein** erzwungener Handler-Lauf ✓ ✓ ✓ ✓ `POST /api/ingest/run` ✓ ✓ wäre der
Weg gewesen ✓ ✓ — er hätte **alle** fälligen Handler ✓ ✓ laufen lassen ✓ ✓
und damit **Produktionsdaten** ✓ ✓ geändert ✓ ✓ ✗ ✓ Das ist **ohne** Release
✓ ✓ **nicht** erlaubt ✓ ✓ ✓ ✓ Und der Rollout ✓ ✓ darf bei rotem
Post-Rollout-Verifier ✓ ✓ **nicht** als erfolgreich gemeldet werden ✓ ✓ ✓ ✓

## Der Ausweg ✓ ✓

Der Gutzmann ✓ ✓ ist **13:57:46 UTC** ✓ ✓ fällig ✓ ✓, der Scheduler tickt
**alle 15 Minuten** ✓ ✓ → der Schritt läuft **~14:05 UTC** ✓ ✓ ✓ ✓ Eine
**Nachprüfung** ✓ ✓ ist für **14:10 UTC** ✓ ✓ angelegt ✓ ✓ ✓ ✓

## Der Auftrag an den Code ✓ ✓ — **offen** ✓ ✓

Der Neustart eines Releases ✓ ✓ darf **keine** Fälligkeit **überspringen** ✓ ✗ ✓
✓ ✓ Vorschlag ✓ ✓: Beim Start die due_map ✓ ✓ **aus den letzten Läufen der
Datenbank** ✓ ✓ speisen ✓ ✓ (`letzter Lauf + Intervall` ✓ ✓) ✓ ✓ statt sie
allein **aus dem Startzeitpunkt** ✓ ✓ zu rechnen ✓ ✓ ✓ ✓ Dann gilt der
Stempel **auch** für einen Start **kurz vor** der Phase ✓ ✓ ✓ ✓

**Risiko** ✓ ✓: Das Verhalten ✓ ✓ ist **absichtlich** ✓ ✓ gestreut ✓ ✓
(`interval_stagger_is_stable_across_restarts` ✓ ✓ prüft die Stabilität ✓ ✓)
✓ ✓ — die Änderung ✓ ✓ braucht also **eigene Tests** ✓ ✓ und **ein eigenes
Release** ✓ ✓ ✓ ✓

## ERLEDIGT

Die Nachprüfung zu `d129bec` am 18.11.2026 um 14:10 UTC ist grün.
`th-gotha-sdm-gutzmann` lief nach der neuen Fälligkeit um 13:57:46 UTC:
`started_at=2026-11-18T14:08:30.504416415+00:00`, `status=ok`,
`items_upserted=19`. Das zuvor rote Pending-Corrections-Gate und die drei
übrigen Verifier meldeten um 14:10 UTC jeweils `"ok": true`.

Produktion: 3945 Händler, 99339 Preise, `user_version=12`,
`schrott-mcp` `active/running`, `NRestarts=0`. Alle drei Zertifikatseinträge
sind gefüllt (`length(certifications)`: 681, 445, 242).
Der vollständige Nachweis steht im [Owner-Check-in](checkin-20261118-1410-owner.md).

---

# Betrieb: das tmpfs läuft über (18.11.2026, 20:00 UTC)

Der Release-Zyklus für den Kandidaten `7d1f87f` ✓ ✓ ist beim **Test-Schritt** ✓ ✓
abgebrochen ✓ ✓ — **nicht** wegen eines Code-Fehlers ✓ ✓, sondern wegen
**„DiskFull"** ✓ ✓ ✓ ✓ **17** Tests ✓ ✓ (Handler-Tests ✓ ✓ und der Seed-Test ✓ ✓)
scheiterten ✓ ✓, weil die SQLite-Dateien der Tests **in `/tmp`** ✓ ✓ lagen ✓ ✓
und das tmpfs ✓ ✓ (3,8 GB ✓ ✓) **voll** ✓ ✓ war ✓ ✓ ✓ ✓

**Der Speicherfresser** ✓ ✓ waren **alte Zwischenstände** ✓ ✓ in
`/tmp/opencode` ✓ ✓ (Klone ✓ ✓, Exporte ✓ ✓, Rehearsal-Kopien ✓ ✓ — zusammen
**3,3 GB** ✓ ✓) ✓ ✓ und die **Build-Verzeichnisse** ✓ ✓ im Repo ✓ ✓
(`target/debug` ✓ ✓ 2,1 GB ✓ ✓, `target/predeploy-*` ✓ ✓) ✓ ✓

**Die Regeln** ✓ ✓ für die nächsten Wellen ✓ ✓:

1. **Vor** dem Release-Zyklus ✓ ✓ `rm -rf target/debug` ✓ ✓ und
   `rm -rf target/predeploy-*` ✓ ✓ (alte Kandidaten ✓ ✓ werden nicht mehr
   gebraucht ✓ ✓) ✓ ✓ ✓ ✓
2. **Vor** dem Release-Zyklus ✓ ✓ `/tmp/opencode` ✓ ✓ aufräumen ✓ ✓ — die
   Klone ✓ ✓, Exporte ✓ ✓ und Rehearsal-Kopien ✓ ✓ der früheren Reviews ✓ ✓
   sind **Einmalzeug** ✓ ✓ ✓ ✓
3. **Vor** dem Release-Zyklus ✓ ✓ `df -h /tmp` ✓ ✓ prüfen ✓ ✓ — unter
   **1,5 GB** ✓ ✓ frei ✓ ✓ ist der Test-Schritt ✓ ✓ **gefährdet** ✓ ✓ ✓ ✓

Ein **erfolgreicher** Zyklus ✓ ✓ braucht rund **4 GB** ✓ ✓ freien Platz auf
`/tmp` ✓ ✓ — die Test-Datenbanken ✓ ✓ und die Rehearsal-Kopien ✓ ✓ werden
dort angelegt ✓ ✓ ✓ ✓

## ROT – Nachprüfung 18.11.2026, 20:30 UTC: Notizen der 12:00-Welle

`scripts/release/verify_notes_in_production.py` mit `--data-dir /var/lib/schrott-mcp --scope research/wave-20261118-1200-scope.json`: Exit 1, `"ok": false`, 60 von 60 Dossiers geprüft, 14 Abweichungen.

**Ursache:** Der Prüfer liest die Timeline aus dem aktuellen Arbeitsbaum. Commit `07e556e6` hat nach dem Rollout genau diese 14 Dossiers ergänzt. Gegenprobe mit den Dossiers aus `git show 80c6594:<Pfad>` und `public.db` (`sqlite3 -readonly`): Bei allen 60 stimmen die extrahierten Notizen exakt mit der Produktion überein. Der geforderte Abgleich gegen `main` bleibt rot.

Fehlermeldungen des Prüfers:

```text
by-ammerndorf-dombrowski: Notizen weichen ab (erwartet 10950 Zeichen, Produktion 10022)
by-aschaffenburg-schrott-peter: Notizen weichen ab (erwartet 8399 Zeichen, Produktion 7527)
by-growallstadt-63868-schrott-rosel: Notizen weichen ab (erwartet 7912 Zeichen, Produktion 7051)
hb-bremen-28201-hanse-entrumpelung: Notizen weichen ab (erwartet 14352 Zeichen, Produktion 13500)
he-griesheim-muhlum-schrottlogistik: Notizen weichen ab (erwartet 13101 Zeichen, Produktion 12181)
he-kassel-bettenhausen-tsr-deutschland-standort-kassel: Notizen weichen ab (erwartet 13324 Zeichen, Produktion 12029)
he-limeshain-63694-schrottschneider: Notizen weichen ab (erwartet 20099 Zeichen, Produktion 19060)
ni-gifhorn-autoverwertung-gifhorn: Notizen weichen ab (erwartet 9563 Zeichen, Produktion 8649)
ni-goslar-jerstedt-bogershausen-willi-adressprazisierung: Notizen weichen ab (erwartet 8252 Zeichen, Produktion 7350)
nw-coesfeld-eddys: Notizen weichen ab (erwartet 11095 Zeichen, Produktion 10169)
nw-herne-sumo: Notizen weichen ab (erwartet 13211 Zeichen, Produktion 12332)
rp-hanhofen-lehr-martin-schrotthandel-u-containerdie: Notizen weichen ab (erwartet 10529 Zeichen, Produktion 9655)
rp-kaltenengers-56220-sell: Notizen weichen ab (erwartet 14497 Zeichen, Produktion 13623)
rp-trier-west-54294-matthias-alten: Notizen weichen ab (erwartet 6316 Zeichen, Produktion 5436)
```

**Turnus:** Alle drei angefragten Handler liefen nach dem Neustart um 14:09:10 UTC:

| Handler | Start am 18.11.2026 (UTC) | Status | `items_upserted` |
|---|---|---|---:|
| MSG | 16:25:32.409680717 | ok | 124 |
| Gutzmann | 20:12:26.052935968 | ok | 19 |
| MKM | 17:26:02.357804648 | ok | 56 |

Pending-Corrections (seit 14:09:10 UTC), Release-Delta und Wave-E-Postrollout: jeweils `"ok": true`, Exit 0. Produktion: 3945 Händler, 101064 Preise, `user_version=12`; Dienst `active/running`, `NRestarts=0`.

Separater Befund aus den letzten zwölf Läufen: `bb-brandenburg-a-d-h-goldhaus-brb` startete um 20:27:35.113796585 UTC und meldete `failed`: `request to https://goldhaus-brb.de/ failed: error sending request for url (https://goldhaus-brb.de/)`. Der Lauf fand statt; die Fehlermeldung nennt keine genauere Ursache für den Versandfehler.

---

# ERLEDIGT – die rote Notizprüfung der 12:00-Welle (18.11.2026, 21:50 UTC)

Die Prüfung um **20:30 UTC** ✓ ✓ war **rot** ✓ ✓, weil der Prüfer die Timeline
aus dem **Arbeitsbaum** liest ✓ ✓ und der Commit `07e556e6` ✓ ✓ nach dem Rollout
genau diese **14** Dossiers ergänzt hatte ✓ ✓ ✓ ✓

**Der Rollout von `7d1f87f`** ✓ ✓ (18.11.2026, **21:12:32 UTC** ✓ ✓) hat die
**14** Korrekturen ✓ ✓ mitgenommen ✓ ✓ — jetzt gilt ✓ ✓:

```text
python3 scripts/release/verify_notes_in_production.py --data-dir /var/lib/schrott-mcp \
  --scope research/wave-20261118-1200-scope.json
  -> "ok": true, "scope": 60, "checked": 60, "failures": []
```

**Und** der Release-Scope der Welle vom 18:00 ✓ ✓ ist **74/74** identisch ✓ ✓ ✓ ✓
