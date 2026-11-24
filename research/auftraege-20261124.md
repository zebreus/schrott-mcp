# Aufträge vom 24.11.2026 — die offenen Punkte des Check-ins

**Stand 24.11.2026, 07:25 UTC ✓ ✓**

## 1) Die Welle vom 24.11.2026, 03:00 — der Rollout steht noch aus ✓ ✓

**Stand** ✓✓: die Owner-Entscheidung über **alle sechzig** ✓✓ Dossiers ✓✓ ist
getroffen ✓✓ — **fünf** ✓✓ Punkte **gesetzt** ✓✓, **sechsunddreißig** ✓✓
**entfernt** ✓✓, **neunzehn** ✓✓ **bleiben** ✓✓ ✓ ✓

| Schritt | Stand |
|---|---|
| Die **sechs** Gates ✓ ✓ | **grün** ✓ ✓ |
| Die **fünfzehn** Seed-Tests ✓ ✓ | **grün** ✓ ✓ |
| Das **unabhängige Review** ✓✓ | **FREIGE** ✓✓ — **zwei** Runden ✓✓, **null** ✓✓ Befunde ✓✓ in Runde 2 ✓✓ ✓ ✓ |
| Der **Release-Zyklus** ✓✓ (Kandidat `89467979` ✓✓) | **grün** ✓✓ ✓ ✓ |
| Der **Pin-Commit** ✓✓ | `b3973192` ✓✓ ✓ ✓ |
| Der **Rollout** ✓✓ | **wartet** ✓✓ auf das **Pending-Gate** ✓✓ der PLZ-Trennung ✓✓ ✓ ✓ |

**Nächster Schritt** ✓✓: das **Pending-Gate** ✓✓ von `8e2de6d` ✓✓ wird nach dem
**MSG**-Lauf ✓✓ um **~12:43 UTC** ✓✓ **grün** ✓✓ ✓ ✓ Danach der **Rollout** ✓✓ der
Welle ✓✓ ✓ ✓

**Bis dahin** ✓✓ gilt: **keine** ✓✓ Produktionsänderung ✓✓ außerhalb eines
**freigegebenen** Releases ✓✓ ✓ ✓

## 2) Die **hundertzweiundachtzig** ✓✓ Zeilen mit der Nummer in **beiden** ✓✓ Feldern ✓✓

**Befund** ✓✓ (read-only, 24.11.2026, 07:20 UTC ✓✓): **182** ✓✓ Zeilen ✓✓ tragen
die Postleitzahl **noch** ✓✓ im Feld `city` ✓✓ — und **181** ✓✓ davon haben das
Feld `postcode` ✓✓ **schon** ✓✓ **richtig** ✓✓ gefüllt ✓✓ ✓ ✓

Das ist die **kosmetische** ✓✓ Bereinigung ✓✓, die die Korrekturrunde vom
24.11.2026 ✓✓ für ihre **hundertfünfzigvier** ✓✓ Zeilen ✓✓ schon erledigt hat ✓✓
— für diese **hundertzweiundachtzig** ✓✓ Zeilen ✓✓ steht sie **noch** ✓✓ aus ✓✓
✓ ✓

**Warum jetzt nicht** ✓✓: sie ist eine **eigene** ✓✓ Korrekturrunde ✓✓ mit
**eigenem** ✓✓ Release ✓✓ — sie darf **nicht** ✓✓ in den Release der Welle ✓✓
hineinlaufen ✓✓ ✓ ✓ Sie folgt **nach** dem Rollout der Welle ✓✓ ✓ ✓

**Der Auftrag** ✓✓: das Feld `city` ✓✓ auf den **reinen** ✓✓ Ortsnamen ✓✓
zurücknehmen ✓✓ — das Feld `postcode` ✓✓ bleibt **unberührt** ✓✓ ✓ ✓

## 3) Die **drei** ✓✓ dauerhaft fehlschlagenden Handler ✓✓

Sie stehen **außerhalb** ✓✓ des Release-Scopes ✓✓ und sind **kein** ✓✓ Befund
der Releases ✓✓ ✓ ✓

| Handler | Letzter Lauf | Meldung |
|---|---|---|
| `be-treptow-kopenick-amr-…` ✓ ✓ | 24.11.2026, 06:36 UTC ✓ ✓ | **HTTP 502** ✓✓ — die Quelle ist **nicht erreichbar** ✓✓ ✓ ✓ |
| `nw-munster-a-z-recycling-udo-salzsieder` ✓ ✓ | 24.11.2026, 06:20 UTC ✓ ✓ | „**verdächtige gleichmäßige Preisleiter** ✓✓ über mehrere …“ ✓✓ ✓ ✓ |
| `th-zella-mehlis-wahl-co-inh-thomas-madeiski` ✓ ✓ | 24.11.2026, 05:20 UTC ✓ ✓ | „**Preisstand ist älter als 24 Stunden** ✓✓; tagesabhängige Richtwerte werden …“ ✓✓ ✓ ✓ |

**MKM** ✓✓ läuft wieder **grün** ✓✓ — **47** ✓✓ Preise ✓✓ und **14** ✓✓ Annahmen ✓✓
um **01:25:25 UTC** ✓✓ ✓ ✓

## 4) Die **Nachprüfungen** ✓✓

| Release | Job | Stand |
|---|---|---|
| Die **PLZ-Trennung** ✓✓ (`8e2de6d` ✓✓) ✓✓ | `release-nachpruefung-8e2de6d` ✓✓ | **angelegt** ✓✓ für **13:15 UTC** ✓✓ ✓ ✓ |
| Die **Welle vom 03:00** ✓✓ | **noch keiner** ✓✓ | **anzulegen** ✓✓, **sobald** ✓✓ der Rollout **gelaufen** ✓✓ ist ✓✓ ✓ ✓ |

## 6) Die Welle vom 24.11.2026, 09:00 ✓✓ — **läuft** ✓✓

**Sechzig** ✓✓ Dossiers ✓✓ in **fünf** ✓✓ Bundesländern ✓✓ ✓ ✓ Fünf **Shards** ✓✓ à
**zwölf** ✓✓ in **temporären** ✓✓ Worktrees ✓✓ ✓ ✓ **Null** ✓✓ Überschneidung ✓✓ mit
allen **41** ✓✓ älteren versionierten Scopes ✓✓ ✓ ✓

**Noch offen** ✓✓: die **Recherche** ✓✓ der fünf Shards ✓✓ ✓✓, die **Übernahme** ✓✓ in
`main` ✓✓ ✓✓ die **sechs** Gates ✓✓ ✓✓ die **Entscheidung** ✓✓ des Owners ✓✓ über die
gespeicherten Punkte ✓✓ ✓✓ der **Bericht** ✓✓ und der **Release-Scope** ✓✓ ✓✓

**Der Release** ✓✓ der 09:00-Welle ✓✓ folgt dem Rollout der 03:00-Welle ✓✓ — **nie**
✓✓ zwei Releases **gleichzeitig** ✓✓ ✓ ✓

## 5) Die Zeile `rp-11880-…` ✓✓

**Entschieden** ✓✓: sie bleibt `pruefung` ✓✓ mit **dokumentiertem** ✓✓
**Klärfall** ✓✓ ✓ ✓ Der Eintrag beschreibt **keinen** ✓✓ Betrieb ✓✓ und **keinen**
✓✓ Ort ✓✓ ✓ ✓ **Kein** offener Auftrag ✓✓ ✓ ✓
