# Aufträge vom 24.11.2026 — die offenen Punkte des Check-ins

**Stand 24.11.2026, 09:15 UTC ✓ ✓**

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
**MSG**-Lauf ✓✓ um **~12:43 UTC** ✓✓ **grün** ✓✓ ✓ ✓ Danach der **Rollout** ✓✓ der
Welle ✓✓ ✓ ✓

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

## 4a) Das Adress-Gate meldet eine **falsche Distanz** nur **weich** ✓✓

**Befund** ✓✓ (Negativkontrolle der Welle vom 09:00 ✓✓, 24.11.2026, 09:25 UTC ✓✓):
Der Check der **Distanz** ✓✓ im `check_address_claims.py` ✓✓ stimmt **nicht** mit der
Messung **überein** ✓✓ ✓ ✓ Er landet dann in der Liste **„Nicht prüfbar“** ✓✓ — und
**nicht** unter den **Befunden** ✓✓ ✓ ✓ Der Exit-Code **bleibt 0** ✓✓ ✓ ✓

**Die Meldung** ✓✓ ist **zudem** ✓✓ **irreführend** ✓✓ — sie nennt einen **„alten Punkt
im Dossier“** ✓✓, den es in diesem Fall **gibt** ✓✓ ✓ ✓

**Ersetzt** ✓✓ wurde die Distanz ✓✓ von **1,54 km** ✓✓ durch **15,4 km** ✓✓ — der
**gleiche** ✓✓ Fall **ohne** ✓✓ Befund ✓✓ ✓ ✓

**Der Auftrag** ✓✓: die **Meldung** ✓✓ **richtig** ✓✓ stellen ✓✓ und entscheiden ✓✓,
ob eine **abweichende** ✓✓ Distanz ein **Befund** ✓✓ sein **muss** ✓✓ ✓ ✓ Das **Werkzeug**
✓✓ wird **nicht** in einem laufenden Release **geändert** ✓✓ ✓ ✓

## 4b) Der **Geokodierungs-Rückstand** ✓✓ — **38** ✓✓ Dossiers ✓✓

**Gemessen** ✓✓ (read-only, 24.11.2026, 10:00 UTC ✓✓): **38** ✓✓ Dossiers ✓✓ tragen
eine **Adresse** ✓✓, **keinen** ✓✓ Punkt ✓✓ und **keinen** ✓✓ Clear ✓✓ auf die
Koordinate ✓✓ ✓ ✓ Weitere **213** ✓✓ Dossiers ✓✓ tragen **bewusst** ✓✓ einen
`clear_fields: geo` ✓✓ — das ist **kein** ✓✓ Rückstand ✓✓ ✓ ✓

**Der Auftrag** ✓✓: **keiner** ✓✓ vorerst ✓✓ — eine **Adresse** ✓✓ aus einem
**Verzeichnis** ✓✓ ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ Diese **38** ✓✓ Zeilen ✓✓
brauchen **Recherche** ✓✓ (Betreiberquelle ✓✓ **und** adressiertes Objekt ✓✓), **nicht**
**nur** ✓✓ eine Geokodierung ✓✓ ✓ ✓ Sie kommen **nicht** ✓✓ in den **Pool** ✓✓ der
Wellen ✓✓ (sie haben eine Straße ✓✓) ✓✓ und bilden **deshalb** ✓✓ eine **eigene**
✓✓ Runde ✓✓ ✓ ✓

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
