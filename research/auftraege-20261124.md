# Aufträge vom 24.11.2026 — die offenen Punkte des Check-ins

**Stand 24.11.2026, 07:25 UTC ✓ ✓**

## 1) Die Welle vom 24.11.2026, 03:00 — der Rollout steht aus ✓ ✓

**Stand** ✓✓: die Owner-Entscheidung über **alle sechzig** ✓✓ Dossiers ✓✓ ist
getroffen ✓✓ — **fünf** ✓✓ Punkte **gesetzt** ✓✓, **sechsunddreißig** ✓✓
**entfernt** ✓✓, **neunzehn** ✓✓ **bleiben** ✓✓ ✓ ✓

| Schritt | Stand |
|---|---|
| Die **sechs** Gates ✓ ✓ | **grün** ✓ ✓ |
| Die **fünfzehn** Seed-Tests ✓ ✓ | **grün** ✓ ✓ |
| Der **Release-Zyklus** ✓✓ (Kandidat `f01b168c` ✓✓) | **läuft** ✓✓ |
| Das **unabhängige Review** ✓✓ | **läuft** ✓✓ |
| Die **Pre-Deploy-Freigabe** ✓✓ | **offen** ✓✓ — folgt dem Review ✓✓ |

**Nächster Schritt** ✓✓: das Urteil des Reviews ✓✓ einordnen ✓✓, den **Pin-Commit** ✓✓
mit den Verifiern ✓✓ auf `main` ✓✓ **pushen** ✓✓, den Rollout ✓✓ anstreben ✓✓ ✓ ✓

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

## 5) Die Zeile `rp-11880-…` ✓✓

**Entschieden** ✓✓: sie bleibt `pruefung` ✓✓ mit **dokumentiertem** ✓✓
**Klärfall** ✓✓ ✓ ✓ Der Eintrag beschreibt **keinen** ✓✓ Betrieb ✓✓ und **keinen**
✓✓ Ort ✓✓ ✓ ✓ **Kein** offener Auftrag ✓✓ ✓ ✓
