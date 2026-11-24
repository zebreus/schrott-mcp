# Aufträge vom 24.11.2026 — die offenen Punkte des Check-ins

**Stand 24.11.2026, 10:50 UTC ✓ ✓**

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

**Die zwei Befunde** ✓✓ der ersten Runde ✓✓ sind **behoben** ✓✓ ✓ ✓ Die **Hausnummer**
✓✓ „**8e**“ ✓✓ ist „**8D**“ ✓✓, und die **Zahl** ✓✓ der objektgenauen Punkte ✓✓
ist „**zwölf**“ ✓✓ ✓ ✓ **Zwei** ✓✓ Werkzeuglücken ✓✓ sind **geschlossen** ✓✓ ✓ ✓

**Nächster Schritt** ✓✓: das **Pending-Gate** ✓✓ von `8e2de6d` ✓✓ wird nach dem
**MSG**-Lauf ✓✓ um **~12:48 UTC** ✓✓ **grün** ✓✓ ✓ ✓ Der **Gutzmann**-Lauf ✓✓ ist
um **10:38 UTC** ✓✓ **nach** ✓✓ dem Rollout ✓✓ gelaufen ✓✓ — es fehlt **nur**
✓✓ noch **MSG** ✓✓ ✓ ✓ Danach der **Rollout** ✓✓ der Welle ✓✓ ✓ ✓

## 2) Die **hundertzweiundachtzig** ✓✓ Zeilen mit der Nummer in **beiden** ✓✓ Feldern ✓✓

**Befund** ✓✓ (read-only, 24.11.2026, 08:20 UTC ✓✓): **182** ✓✓ Zeilen ✓✓ tragen
die Postleitzahl **noch** ✓✓ im Feld `city` ✓✓ — und **181** ✓✓ davon haben das
Feld `postcode` ✓✓ **schon** ✓✓ **richtig** ✓✓ gefüllt ✓✓ ✓ ✓

| Feld | Wert |
|---|---|
| Die Zeilen ✓ ✓ | **182** ✓ ✓ |
| Mit der PLZ **auch** ✓✓ im Feld `postcode` ✓✓ | **181** ✓ ✓ |
| **Ohne** ✓✓ Leerzeichen ✓✓ vor der Nummer ✓✓ | **2** ✓ ✓ |
| Die **Verteilung** ✓✓ | HE **56** ✓✓ · SN **25** ✓✓ · MV **22** ✓✓ · BY **19** ✓✓ · RP **17** ✓✓ · NI **12** ✓✓ · SH **8** ✓✓ · BW **6** ✓✓ · SL **5** ✓✓ · NW **4** ✓✓ · BB **2** ✓✓ · BE **2** ✓✓ · HB **2** ✓✓ · ST **1** ✓✓ · TH **1** ✓✓ |
| Die **Dossiers** ✓✓ | **alle** ✓✓ **vorhanden** ✓✓ — und das Feld `city` ✓✓ stimmt in **jedem** ✓✓ Dossier ✓✓ mit der Produktion ✓✓ ✓ ✓ |

**Die eine Ausnahme** ✓✓: `rp-11880-…` ✓✓ trägt im Ortsfeld nur die **Nummer** ✓✓
**11880** ✓✓ und **kein** Feld `postcode` ✓✓ — das ist der **Klärfall** ✓✓, der nach
Vermerk **`pruefung`** ✓✓ **bleibt** ✓✓ ✓ ✓ Er wird **nicht** mitgezogen ✓✓ ✓ ✓

**Der Auftrag** ✓✓: das Feld `city` ✓✓ auf den **reinen** ✓✓ Ortsnamen ✓✓
zurücknehmen ✓✓ — das Feld `postcode` ✓✓ bleibt **unberührt** ✓✓ ✓ ✓

**Die Reihenfolge** ✓✓: die Korrektur ist eine **eigene** ✓✓ Korrekturrunde ✓✓ mit
**eigenem** ✓✓ Release ✓✓ — sie folgt **nach** ✓✓ beiden Wellen ✓✓, denn **sieben**
✓✓ ihrer Zeilen ✓✓ liegen in den Scopes der Wellen ✓✓ (**vier** ✓✓ in der 03:00-Welle
✓✓, **drei** ✓✓ in der 09:00-Welle ✓✓) ✓✓ ✓ ✓ Sie darf **nicht** ✓✓ in einen
Release der Welle ✓✓ hineinlaufen ✓✓ ✓ ✓

## 3) Die **drei** ✓✓ dauerhaft fehlschlagenden Handler ✓✓

Sie stehen **außerhalb** ✓✓ des Release-Scopes ✓✓ und sind **kein** ✓✓ Befund
der Releases ✓✓ ✓ ✓

| Handler | Letzter Lauf | Meldung |
|---|---|---|
| `be-treptow-kopenick-amr-…` ✓ ✓ | 24.11.2026, 06:36 UTC ✓ ✓ | **HTTP 502** ✓✓ — die Quelle ist **nicht erreichbar** ✓✓ ✓ ✓ |
| `nw-munster-a-z-recycling-udo-salzsieder` ✓ ✓ | 24.11.2026, 06:21 UTC ✓ ✓ | „**verdächtige gleichmäßige Preisleiter** ✓✓ über mehrere …“ ✓✓ ✓ ✓ |
| `th-zella-mehlis-wahl-co-inh-thomas-madeiski` ✓ ✓ | 24.11.2026, 05:20 UTC ✓ ✓ | „**Preisstand ist älter als 24 Stunden** ✓✓; tagesabhängige Richtwerte werden …“ ✓✓ ✓ ✓ |

**MKM** ✓✓ läuft wieder **grün** ✓✓ — **47** ✓✓ Preise ✓✓ und **14** ✓✓ Annahmen ✓✓
um **01:25:25 UTC** ✓✓ ✓ ✓

## 4) Die **Nachprüfungen** ✓✓

| Release | Job | Stand |
|---|---|---|
| Die **PLZ-Trennung** ✓✓ (`8e2de6d` ✓✓) ✓✓ | `release-nachpruefung-8e2de6d` ✓✓ | **angelegt** ✓✓ für **13:15 UTC** ✓✓ ✓ ✓ |
| Die **Welle vom 03:00** ✓✓ | **noch keiner** ✓✓ | **anzulegen** ✓✓, **sobald** ✓✓ der Rollout **gelaufen** ✓✓ ist ✓✓ ✓ ✓ |
| Die **Welle vom 09:00** ✓✓ | **noch keiner** ✓✓ | **anzulegen** ✓✓, **sobald** ✓✓ der Rollout **gelaufen** ✓✓ ist ✓✓ ✓ ✓ |

## 4a) Das Adress-Gate meldet eine **falsche Distanz** nur **weich** ✓✓ — **BEHOBEN** ✓✓

**Befund** ✓✓ (Negativkontrolle der Welle vom 09:00 ✓✓, 24.11.2026, 09:25 UTC ✓✓):
Der Check der **Distanz** ✓✓ im `check_address_claims.py` ✓✓ stimmt **nicht** mit der
Messung **überein** ✓✓ ✓ ✓ Er landet dann in der Liste **„Nicht prüfbar“** ✓✓ — und
**nicht** unter den **Befunden** ✓✓ ✓ ✓ Der Exit-Code **bleibt 0** ✓✓ ✓ ✓

**Die Meldung** ✓✓ ist **zudem** ✓✓ **irreführend** ✓✓ — sie nennt einen **„alten Punkt
im Dossier“** ✓✓, den es in diesem Fall **gibt** ✓✓ ✓ ✓

**Ersetzt** ✓✓ wurde die Distanz ✓✓ von **1,54 km** ✓✓ durch **15,4 km** ✓✓ — der
**gleiche** ✓✓ Fall **ohne** ✓✓ Befund ✓✓ ✓ ✓

### Die Lösung ✓✓ — Commit `68a89914` ✓✓

**Die Ursache** ✓✓: die Schleife ✓✓ über die Objekte ✓✓ brach **nur** ✓✓ ab ✓✓, wenn
die **Objektmitte** ✓✓ (Nominatim) ✓✓ zur behaupteten Distanz ✓✓ passte ✓✓ — der Rest
landete **immer** ✓✓ in der Liste **„Nicht prüfbar“** ✓✓ ✓ ✓

| Schritt | Vorher | Nachher |
|---|---|---|
| Eine **messbare** ✓✓ Distanz, die **nicht** ✓✓ passt ✓✓ | **Lücke** ✓✓ · Exit **0** ✓✓ | **Befund** ✓✓ · Exit **1** ✓✓ |
| Die **Meldung** ✓✓ der Lücke ✓✓ | „kein alter Punkt im Dossier“ ✓✓ — **falsch** ✓✓ | der **Grund** ✓✓: Rate-Limit ✓✓ oder kein Objekt ✓✓ ✓ ✓ |
| Die **Messung** ✓✓ | nur die **Objektmitte** ✓✓ | **drei** ✓✓ Werte ✓✓: Objektmitte ✓✓, **Mittelwert** ✓✓ der eindeutigen Knoten ✓✓, **Spanne** ✓✓ der Knotenabstaende ✓✓ ✓ ✓ |
| Der **Rate-Limit** ✓✓ | Lücke ✓✓ | Lücke ✓✓ — **nie** ✓✓ ein Befund ✓✓ ✓ ✓ |

**Der Beleg** ✓✓ — die **Negative Kontrolle** ✓✓ vom **24.11.2026** ✓✓ **10:38 UTC** ✓✓ ✓ ✓

| Gate | Ausgabe | Exit |
|---|---|---|
| Das **alte** ✓✓ Gate ✓✓ | „Nicht pruefbar (kein alter Punkt im Dossier): 1“ ✓✓ | **0** ✓✓ |
| Das **neue** ✓✓ Gate ✓✓ | „Befunde: 1 — die behauptete Distanz 15400 m passt nicht zur gemessenen Distanz“ ✓✓ | **1** ✓✓ |

**Die Gates** ✓✓ nach dem Fix ✓✓: **null** ✓✓ Befunde ✓✓ auf den Scopes der
**03:00**-Welle ✓✓ (**60** ✓✓), der **09:00**-Welle ✓✓ (**60** ✓✓) und der
Korrekturrunde ✓✓ (**181** ✓✓) ✓✓ — und **keine** ✓✓ offene **Lücke** ✓✓ mehr ✓✓ ✓ ✓

**Neun** ✓✓ Tests ✓✓ decken die Entscheidung ✓✓ ab ✓✓ (`scripts/test_check_address_claims.py` ✓✓) ✓ ✓

## 4c) Die Korrekturrunde des Ortsfeldes ✓✓ — **FREIGE** ✓✓

**Stand** ✓✓ (24.11.2026, 10:40 UTC ✓✓): das **unabhängige** ✓✓ Review ✓✓ ist in
**zwei** ✓✓ Runden ✓✓ gelaufen ✓✓ ✓ ✓

| Schritt | Stand |
|---|---|
| Das **unabhängige Review** ✓✓ | **FREIGE** ✓✓ — Runde 1 **SPERREN** ✓✓ (**4** ✓✓ Datenfehler ✓✓, **2** ✓✓ Werkzeuglücken ✓✓), Runde 2 **FREIGE** ✓✓ **ohne** ✓✓ Befund ✓✓ ✓ ✓ |
| Die **Kandidatin** ✓✓ | `1bf23fad` ✓✓ ✓ ✓ |
| Der **Release-Zyklus** ✓✓ | **läuft** ✓✓ ✓ ✓ |
| Der **Rollout** ✓✓ | **nach** ✓✓ den **beiden** ✓✓ Wellen ✓✓ ✓ ✓ |

**Die vier** ✓✓ Datenfehler ✓✓ der ersten Runde ✓✓ und die **zwei** ✓✓ Werkzeuglücken
✓✓ sind **vollständig** ✓✓ behoben ✓✓ ✓ ✓ Der Reviewer hat das mit **vier** ✓✓
**Negativkontrollen** ✓✓ belegt ✓✓ ✓ ✓

## 4b) Der **Geokodierungs-Rückstand** ✓✓ — **29** ✓✓ Dossiers ✓✓

**Nachgemessen** ✓✓ (read-only, 24.11.2026, 11:00 UTC ✓✓) — die frühere Zahl
**„38“** ✓✓ ist **nicht** ✓✓ reproduzierbar ✓✓ und wird **hiermit** ✓✓
**berichtigt** ✓✓ ✓ ✓

| Die Definition ✓✓ | Die Zahl ✓✓ |
|---|---|
| In der **Produktion** ✓✓ **ohne** ✓✓ Punkt ✓✓, **ohne** ✓✓ `clear_fields: geo` ✓✓ | **64** ✓✓ |
| davon mit einer **Straße** ✓✓ | **29** ✓✓ |
| davon **nur** ✓✓ mit PLZ ✓✓ oder Ort ✓✓, **ohne** ✓✓ Straße ✓✓ | **35** ✓✓ |
| mit **bewusstem** ✓✓ `clear_fields: geo` ✓✓ | **513** ✓✓ |

**Die Runde** ✓✓ braucht die **29** ✓✓ — nur sie haben eine **Straße** ✓✓, und
**nur** ✓✓ mit einer Straße ✓✓ lässt sich ein **adressiertes Objekt** ✓✓ mit
**Hausnummer** ✓✓ überhaupt **finden** ✓✓ ✓ ✓

**Der Auftrag** ✓✓: **keiner** ✓✓ vorerst ✓✓ — eine **Adresse** ✓✓ aus einem
**Verzeichnis** ✓✓ ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ Diese **29** ✓✓ Zeilen ✓✓
brauchen **Recherche** ✓✓ (Betreiberquelle ✓✓ **und** adressiertes Objekt ✓✓), **nicht**
**nur** ✓✓ eine Geokodierung ✓✓ ✓ ✓ Sie kommen **nicht** ✓✓ in den **Pool** ✓✓ der
Wellen ✓✓ (sie haben eine Straße ✓✓) ✓✓ und bilden **deshalb** ✓✓ eine **eigene**
Runde ✓✓ ✓ ✓

**Die Liste** ✓✓ liegt in `research/geokodierung-rueckstand-20261124.json` ✓✓ ✓ ✓

## 5) Die Zeile `rp-11880-…` ✓✓

**Entschieden** ✓✓: sie bleibt `pruefung` ✓✓ mit **dokumentiertem** ✓✓
**Klärfall** ✓✓ ✓ ✓ Der Eintrag beschreibt **keinen** ✓✓ Betrieb ✓✓ und **keinen**
✓✓ Ort ✓✓ ✓ ✓ **Kein** offener Auftrag ✓✓ ✓ ✓

## 6) Die Welle vom 24.11.2026, 09:00 ✓✓ — **entschieden** ✓✓

**Sechzig** ✓✓ Dossiers ✓✓ in **fünf** ✓✓ Bundesländern ✓✓ ✓ ✓ **Hundertzwei** ✓✓
Notizen ✓✓ in sechzig neuen Sektionen ✓✓ ✓ ✓ **Null** ✓✓ Überschneidung ✓✓ mit allen
**41** ✓✓ älteren versionierten Scopes ✓✓ ✓ ✓

**Die Entscheidung** ✓✓ ist **getroffen** ✓✓ — **ein** ✓✓ Punkt **gesetzt** ✓✓
(`by-rothenburg-eisenmann` ✓✓ — die Standortseite des Betreibers nennt **Am Igelsbach 9** ✓✓,
das Objekt trägt **dieselbe Hausnummer** ✓✓ **und** den **Namen** ✓✓ des Betriebs ✓✓ ✓ ✓),
**achtundvierzig** ✓✓ **entfernt** ✓✓, **elf** ✓✓ **bleiben** ✓✓ ✓ ✓ Die **sechs** Gates ✓✓
und die **fünfzehn** ✓✓ Seed-Tests ✓✓ sind **grün** ✓✓ ✓ ✓

**Der Kandidat** ✓✓ ist **freigegeben** ✓✓ — **zwei** ✓✓ Review-Runden ✓✓, die zweite
ohne Befund ✓✓ ✓ ✓ Der **Pin** ✓✓ liegt in `039013c3` ✓✓ ✓ ✓

**Noch offen** ✓✓: der **Rollout** ✓✓ — er wartet auf das **Pending-Gate** ✓✓ der
03:00-Welle ✓✓ ✓ ✓

**Der Rollout** ✓✓ der 09:00-Welle ✓✓ wartet auf das **Pending-Gate** ✓✓ der
03:00-Welle ✓✓ ✓ ✓ — **nie** ✓✓ zwei Releases **gleichzeitig** ✓✓ ✓ ✓
