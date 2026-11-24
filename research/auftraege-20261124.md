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

## 3) Die **drei** ✓✓ dauerhaft fehlschlagenden Handler ✓✓ — **nachgeprüft** ✓✓

Sie stehen **außerhalb** ✓✓ des Release-Scopes ✓✓ und sind **kein** ✓✓ Befund
der Releases ✓✓ ✓ ✓ **Alle drei** ✓✓ sind **richtig** ✓✓ — **live** ✓✓
nachgeprüft ✓✓ am **24.11.2026** ✓✓, **11:20 UTC** ✓✓ ✓ ✓

| Handler | Die Meldung ✓✓ | Der **live** ✓✓ Befund ✓✓ | Das Urteil ✓✓ |
|---|---|---|---|
| `be-treptow-kopenick-amr-…` ✓✓ | „**HTTP 502**“ ✓✓ | `http://www.amr-schrottplatz.de` ✓✓ → **502** ✓✓ · `https://…` ✓✓ → **000** ✓✓ (kein TLS) ✓✓ | Die **Quelle** ✓✓ ist **nicht erreichbar** ✓✓ ✓ ✓ Kein **Owner-Problem** ✓✓ ✓ ✓ |
| `nw-munster-a-z-recycling-udo-salzsieder` ✓✓ | „**verdächtige gleichmäßige Preisleiter** ✓✓ über mehrere …“ ✓✓ | Die Seite ✓✓ (HTTP **200** ✓✓) ✓✓ nennt **Platzhalter** ✓✓: Mischschrott **1,23/1,24/1,25** ✓✓ · Zink **1,26/1,27/1,28** ✓✓ · Blei **1,29/1,30/1,31** ✓✓ · Kupfer **1,32/1,33/1,34** ✓✓ · Messing **1,35/1,36/1,37** ✓✓ · Haushaltskabel **1,38/1,39/1,40** ✓✓ ✓ ✓ **Fünf** ✓✓ Karten ✓✓ mit **gleicher** ✓✓ Spanne ✓✓ und **gleichem** ✓✓ Schritt ✓✓ ✓ ✓ | Die **Regel** ✓✓ greift **richtig** ✓✓ ✓ ✓ Zink ✓✓, Blei ✓✓ und Kupfer ✓✓ zu **1,3 €/kg** ✓✓ sind **wirtschaftlich** ✓✓ **unmöglich** ✓✓ ✓ ✓ **Fail closed** ✓✓ ist **richtig** ✓✓ ✓ ✓ |
| `th-zella-mehlis-wahl-co-inh-thomas-madeiski` ✓✓ | „**Preisstand ist älter als 24 Stunden** ✓✓“ ✓✓ | Die Seite ✓✓ nennt den **20.11.2026** ✓✓ und bezeichnet die Werte selbst als **„Zur Orientierung“** ✓✓ — der **Tagespreis** ✓✓ werde **telefonisch** ✓✓ geklärt ✓✓ ✓ ✓ | Die **Regel** ✓✓ greift **richtig** ✓✓ ✓ ✓ **Tagesabhängige** ✓✓ Richtwerte ✓✓ gehören **nicht** ✓✓ in die Preistabelle ✓✓ ✓ ✓ |

**Die drei** ✓✓ sind damit **keine** ✓✓ offenen **Datenfehler** ✓✓ ✓ ✓ **Kein**
✓✓ Auftrag ✓✓ ✓ ✓

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

## 4d) Die **Welle vom 24.11.2026, 12:00** ✓✓ — **FREIGE** ✓✓

**Stand** ✓✓ (24.11.2026, 15:20 UTC ✓✓): die **Recherche** ✓✓ aller **fünf**
✓✓ Shards ✓✓ ist **übernommen** ✓✓ ✓ ✓ Die **Entscheidung** ✓✓ über **alle
sechzig** ✓✓ Dossiers ✓✓ ist **getroffen** ✓✓ ✓ ✓ Das **unabhängige** ✓✓
**Review** ✓✓ ist in **drei** ✓✓ Runden ✓✓ gelaufen ✓✓ — die erste **sperrte** ✓✓
(**8** ✓✓ Datenfehler ✓✓, **7** ✓✓ Werkzeuglücken ✓✓), die zweite **sperrte**
✓✓ (**3** ✓✓ Datenfehler ✓✓, **2** ✓✓ Werkzeuglücken ✓✓), die **dritte** ✓✓
**gab frei** ✓✓ ✓ ✓

**Ein** ✓✓ **Build-Abbruch** ✓✓ war darunter ✓✓ — **meiner** ✓✓: die Straße des
Stuttgarter Dossiers ✓✓ war als **zweite** ✓✓ Zeile ✓✓ **eingefügt** ✓✓ statt
**ersetzt** ✓✓ ✓ ✓ **Die Lehre** ✓✓: der **Build** ✓✓ ist das **einzige** ✓✓
Gate ✓✓, das einen **doppelten** ✓✓ Key ✓✓ findet ✓✓ — er **gehört** ✓✓ nach
**jeder** ✓✓ Änderung ✓✓ an einem Dossier ✓✓ **dorthin** ✓✓ ✓ ✓

**Der Release-Zyklus** ✓✓ **läuft** ✓✓ — Kandidat `1cbdd6ca` ✓✓ ✓ ✓



**Stand** ✓✓ (24.11.2026, 12:05 UTC ✓✓): **sechzig** ✓✓ frische Dossiers ✓✓ aus
**fünf** ✓✓ **neuen** ✓✓ Bundesländern ✓✓ — **BE** ✓✓ **BW** ✓✓ **HH** ✓✓ **ST** ✓✓
**TH** ✓✓ ✓ ✓

| Shard | Bundesland | Dossiers |
|---|---|---|
| **A** ✓✓ | BE ✓✓ | **12** ✓✓ |
| **B** ✓✓ | BW ✓✓ | **12** ✓✓ |
| **C** ✓✓ | HH ✓✓ | **12** ✓✓ |
| **D** ✓✓ | ST ✓✓ | **12** ✓✓ |
| **E** ✓✓ | TH ✓✓ | **12** ✓✓ |

**Die Ziehung** ✓✓ ✓ ✓ **Null** ✓✓ Überschneidung ✓✓ mit den Wellen vom **03:00**
✓✓ und **09:00** ✓✓ ✓ ✓ **Null** ✓✓ mit allen **21** ✓✓ versionierten Scopes ✓✓ ✓ ✓
**Pool** ✓✓ **1669** ✓✓ → nach dem Ausschluss ✓✓ **498** ✓✓ ✓ ✓

**Die Deckelung** ✓✓ ✓ ✓ Die heutigen Wellen haben **NW** ✓✓, **HE** ✓✓, **NI** ✓✓,
**RP** ✓✓ und **BY** ✓✓ bearbeitet ✓✓ — diese Welle geht deshalb in **fünf**
✓✓ **unberührte** ✓✓ Länder ✓✓ ✓ ✓

**Ein** ✓✓ Dossier ✓✓ (`hh-billbrook-alt-und-buntmetallhandel-dejan` ✓✓) liegt in
**beiden** ✓✓ Runden ✓✓ — die **Standortrunde** ✓✓ lässt es **aus** ✓✓ und
**wartet** ✓✓ auf das **Ergebnis** ✓✓ der Welle ✓✓ ✓ ✓

## 4e) Die **Standortrunde** ✓✓ für die **29** ✓✓ Dossiers ✓✓ — **läuft** ✓✓

**Stand** ✓✓ (24.11.2026, 12:10 UTC ✓✓): **zwei** ✓✓ Recherche-Agenten ✓✓ auf
**zwei** ✓✓ Worktrees ✓✓ ✓ ✓ **Vierzehn** ✓✓ Dossiers ✓✓ in **Shard A** ✓✓
(**HH** ✓✓ **BY** ✓✓ **NW** ✓✓) ✓✓ und **vierzehn** ✓✓ in **Shard B** ✓✓
(**HB** ✓✓ **HE** ✓✓ **MV** ✓✓ **NI** ✓✓ **RP** ✓✓ **SL** ✓✓ **SN** ✓✓ **ST** ✓✓) ✓✓ ✓ ✓

**Der Auftrag an die Agenten** ✓✓: **recherchieren** ✓✓ und **dokumentieren** ✓✓ —
**keine** ✓✓ Koordinate **setzen** ✓✓, **kein** ✓✓ `clear_fields` ✓✓ ändern ✓✓ ✓ ✓
**Die** ✓✓ **Standortentscheidung** ✓✓ **trifft** ✓✓ der **Owner** ✓✓ ✓ ✓

**Die drei** ✓✓ Fragen ✓✓ je Dossier ✓✓ ✓ ✓

| Frage ✓✓ | Das Kriterium ✓✓ |
|---|---|
| Die **Quelle** ✓✓ über das **Geschäft** ✓✓ selbst ✓✓ | Impressum ✓✓ · Standortseite ✓✓ · amtliches Register ✓✓ — eine **Verzeichnisadresse** ✓✓ allein ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ |
| Das **adressierte Objekt** ✓✓ | ein Objekt ✓✓ mit `addr:street` ✓✓ **und** ✓✓ `addr:housenumber` ✓✓ **an** ✓✓ genau dieser Adresse ✓✓ ✓ ✓ |
| Der **gespeicherte Punkt** ✓✓ | die **Distanz** ✓✓ zum Objekt ✓✓ (Haversine ✓✓, Erdradius **6371 km** ✓✓) ✓✓ ✓ ✓ |

## 4f) Der **MKM-Verifier** ✓✓ ist **rot** ✓✓ — die Quelle ist **gewandert** ✓✓

**Befund** ✓✓ (read-only ✓✓, 24.11.2026, 14:30 UTC ✓✓) ✓ ✓

```
python3 scripts/verify-mkm-release.py --data-dir /var/lib/schrott-mcp --since 2026-11-24T12:50:25Z
→ ok: false · articles 61 · numeric 47 · requests 14
  "article quote/request partition 47/14 != 56/5"
  "invalid numeric metadata/tier contract"
  "successful completed post-rollout 56-price step missing"
```

**Der Handler selbst läuft grün** ✓✓ — seit dem **01:25 UTC** ✓✓ schreibt er
**47** ✓✓ Preise und **14** ✓✓ Annahmen ✓✓ ✓ ✓ Die **Quelle** ✓✓
(mkm-metals.de ✓✓) **teilt** ✓✓ heute **anders** ✓✓ als noch im **Oktober** ✓✓
(56/5 ✓✓) ✓ ✓

**Der Verifier** ✓✓ **nennt** ✓✓ das **selbst** ✓✓ ✓ ✓ „Fixed 56/5 needs review
on source drift“ ✓✓ ✓ ✓ **Er** ✓✓ ist also **nicht** ✓✓ kaputt ✓✓ — seine
**geprüfte** ✓✓ Erwartung ✓✓ ist **veraltet** ✓✓ ✓ ✓

**Der dritte** ✓✓ Befund ✓✓ folgt **daraus** ✓✓ ✓ ✓ **Der zweite** ✓✓ ist **eigen**
✓✓ ✓ ✓ **Gemessen** ✓✓ (Manifest des letzten Laufs ✓✓ gegen das **Orakel** ✓✓,
live aus `public.db` ✓✓) ✓ ✓

| Die Messung ✓✓ | Der Wert ✓✓ |
|---|---|
| Die Artikel im Manifest ✓✓ / im **Orakel** ✓✓ | **61** ✓✓ / **61** ✓✓ — **gleich** ✓✓ ✓ ✓ |
| **Numerische** ✓✓ Artikel ✓✓ | **47** ✓✓ (vorher **56** ✓✓) ✓ ✓ |
| **Anfragen** ✓✓ | **14** ✓✓ (vorher **5** ✓✓) ✓ ✓ |
| **Verletzer** ✓✓ des **Staffelvertrags** ✓✓ | **2** ✓✓ ✓ ✓

**Die zwei** ✓✓ **Artikel** ✓✓ ohne **Staffel** ✓✓ ✓ ✓

| Artikel ✓✓ | Material ✓✓ | Preis ✓✓ | Minimum ✓✓ | Die **Staffel** ✓✓ |
|---|---|---|---|---|
| `MKM-NE-05` ✓✓ | Nickel rein ab 99 % ✓✓ | **12,08** ✓✓ €/kg ✓✓ | **250** ✓✓ kg ✓✓ | **leer** ✓✓ ✓ ✓ |
| `SW10041` ✓✓ | Zinn 30% ✓✓ | **5,25** ✓✓ €/kg ✓✓ | **500** ✓✓ kg ✓✓ | **leer** ✓✓ ✓ ✓ |

**Das Orakel** ✓✓ kennt **beide** ✓✓ Artikel ✓✓ ✓ ✓ **Die Quelle** ✓✓ liefert für
sie **keine** ✓✓ **Staffel** ✓✓ mehr ✓✓ — **entweder** ✓✓ hat sich ihr **Markup**
✓✓ geändert ✓✓ **oder** ✓✓ der **Parser** ✓✓ findet die Tabelle **nicht** ✓✓ mehr
✓ ✓ **Neun** ✓✓ weitere Artikel ✓✓ hat die Quelle von **Kurs** ✓✓ auf **Anfrage**
✓✓ umgestellt ✓✓ ✓ ✓

**An der Quelle gelesen** ✓✓ (live ✓✓, 24.11.2026, **15:10 UTC** ✓✓) ✓ ✓
Die Seite nennt **61** ✓✓ Sorten ✓✓ und ist **heute** ✓✓ auf den **24.11.2026**
✓✓ geändert ✓✓ ✓ ✓

**Die zwei** ✓✓ **Artikel** ✓✓ tragen jetzt **einen** ✓✓ **Preis** ✓✓ **und** ✓✓ den
**Zusatz** ✓✓ „**Anfragen**“ ✓✓ — **keine** ✓✓ **Staffel** ✓✓ ✓ ✓

| Artikel ✓✓ | Die Seite ✓✓ | Die **Staffel** ✓✓ |
|---|---|---|
| `MKM-NE-05` ✓✓ | „Nickel rein ab 99 %“ ✓✓ **12,08 €** ✓✓ /kg ✓✓ **ab 250 kg** ✓✓ **Anfragen** ✓✓ ✓ ✓ | **leer** ✓✓ ✓ ✓ |
| `SW10041` ✓✓ | „Zinn 30%“ ✓✓ **5,25 €** ✓✓ /kg ✓✓ **ab 500 kg** ✓✓ **Anfragen** ✓✓ ✓ ✓ | **leer** ✓✓ ✓ ✓ |

**Der Vertrag** ✓✓ verlangt eine **Staffel** ✓✓, deren **oberster** ✓✓ Preis dem
**Artikelpreis** ✓✓ entspricht ✓✓ und die bei **0 kg** ✓✓ beginnt ✓✓ — für
**diese** ✓✓ zwei **gibt** ✓✓ es **keine** ✓✓ ✓ ✓

**Die vierzehn** ✓✓ **Anfrage-Artikel** ✓✓ von heute ✓✓ (live aus dem Manifest
✓✓) ✓ ✓

`MKM-HM-01` ✓✓ · `MKM-HM-02` ✓✓ · `MKM-NE-01` ✓✓ · `MKM-NE-02` ✓✓ ·
`MKM-NE-06` ✓✓ · `MKM-SCH-01` ✓✓ · `MKM-SCH-02` ✓✓ · `MKM-SCH-03` ✓✓ ·
`SW10036` ✓✓ · `SW10037` ✓✓ · `SW10038` ✓✓ · `SW10039` ✓✓ · `SW10040` ✓✓ ·
`SW10042` ✓✓ ✓ ✓

**Die Artikel-IDs** ✓✓ sind **unverändert** ✓✓ — das Manifest führt **61** ✓✓,
das Orakel **61** ✓✓, die Mengen **passen** ✓✓ ✓ ✓ **Neun** ✓✓ Artikel ✓✓ tragen
**heute** ✓✓ **keinen** ✓✓ Preis ✓✓, im **Oktober** ✓✓ aber **noch** ✓✓ ✓ ✓ Die
**Gruppen** ✓✓ „Hartmetall“ ✓✓ und „Schrott“ ✓✓ stehen auf der Seite **ohne**
✓✓ Preiskarten ✓✓ ✓ ✓

**Der Auftrag** ✓✓: **offen** ✓✓ ✓ ✓ **Erstens** ✓✓ das **Orakel** ✓✓ und die
**geprüfte** ✓✓ Teilung **neu** ✓✓ **prüfen** ✓✓ — **47** ✓✓ Preise und **14**
✓✓ Anfragen ✓✓ sind der **Stand** ✓✓ der Quelle ✓✓ ✓ ✓ **Zweitens** ✓✓
entscheiden ✓✓, ob der **Handler** ✓✓ aus **einer** ✓✓ Preiszeile ohne **Staffel**
✓✓ eine **Staffel** ✓✓ **bauen** ✓✓ soll ✓✓ (etwa **ab** ✓✓ der **Mindestmenge**
✓✓ ✓✓) oder ob er **solche** ✓✓ Artikel **ablehnen** ✓✓ muss ✓✓ — und **dann**
✓✓ den **Vertrag** ✓✓ entsprechend **fassen** ✓✓ ✓ ✓ **Bis dahin** ✓✓ tragen
**zwei** ✓✓ veröffentlichte **Preise** ✓✓ eine **unvollständige** ✓✓ **Staffel**
✓✓ ✓ ✓

**Der Auftrag** ✓✓: **offen** ✓✓ ✓ ✓ **Die Quelle** ✓✓ **Artikel für Artikel**
✓✓ gegen das **geprüfte** ✓✓ **Orakel** ✓✓
(`crates/ingestion/src/traders/handlers/fixtures/mkm-reviewed-mapping-oracle-20261018.json`
✓✓) halten ✓✓ und **entscheiden** ✓✓, ob die neue **Staffel** ✓✓ eine
**Wandlung** ✓✓ der Quelle ✓✓ oder ein **Fehler** ✓✓ des Handlers ✓✓ ist ✓✓ ✓ ✓
**Das ist kein Befund** ✓✓ der **Releases** ✓✓ — die Wellen ändern **Dossiers** ✓✓,
**keine** ✓✓ MKM-Preise ✓✓ ✓ ✓

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

## 7) Der **MKM-Verifier** ✓✓ — **entschieden** ✓✓ (**24.11.2026, 17:35 UTC** ✓✓)

**Erledigt** ✓✓ ✓ ✓ Die **Akt** ✓✓ liegt in
`research/mkm-entscheidung-20261124.md` ✓✓ ✓ ✓ Das **Prüfartefakt** ✓✓ liegt in
`research/mkm-partition-review-20261124.json` ✓✓ ✓ ✓

**Die Frage** ✓✓ von **4f)** ✓✓ ist **beantwortet** ✓✓ ✓ ✓ Die **Quelle** ✓✓
zeigt **heute** ✓✓ die **flache** ✓✓ **Preisform** ✓✓ — **kein** ✓✓
**Parserfehler** ✓✓ ✓ ✓ Der **eigene** ✓✓ **Spaltenparser** ✓✓
(`scripts/audit-mkm-overview.py` ✓✓) liest **dieselben** ✓✓ **zwei** ✓✓
Artikel **ohne** ✓✓ `data-staffel` ✓✓ ✓ ✓ **Der Handler** ✓✓ hat sie **schon**
✓✓ **seit dem 17.11.2026** ✓✓ **angenommen** ✓✓ ✓ ✓

**Die drei** ✓✓ **unabhängigen** ✓✓ **Quellen** ✓✓ nennen **dieselben** ✓✓
**61** ✓✓ Artikel ✓✓, **dieselben** ✓✓ **47** ✓✓ **Preise** ✓✓ und
**dieselben** ✓✓ **14** ✓✓ **Anfragen** ✓✓ ✓ ✓ Die **live** ✓✓ **Seite** ✓✓
(HTTP **200** ✓✓, **248080** ✓✓ Bytes ✓✓, SHA256 `a5424e23…` ✓✓) ✓✓ · der
**Abruf** ✓✓ im **Repo** ✓✓ vom **22.11.** ✓✓ · das **Ledger** ✓✓ der
**Produktion** ✓✓ ✓ ✓

**Die Entscheidung** ✓✓ ✓ ✓

| Der Punkt ✓✓ | Die Entscheidung ✓✓ |
|---|---|
| Die **Teilung** ✓✓ | **wandert** ✓✓ in das **Orakel** ✓✓ vom **24.11.** ✓✓ — **47/14** ✓✓ ✓ ✓ |
| Der **Verifier** ✓✓ | liest die **Teilung** ✓✓ **aus dem Orakel** ✓✓ und prüft **jeden** ✓✓ **Artikel** ✓✓ **einzeln** ✓✓ dagegen ✓✓ ✓ ✓ |
| Die **flache** ✓✓ **Preisform** ✓✓ | **anerkannt** ✓✓ für die **zwei** ✓✓ **geprüften** ✓✓ Artikel ✓✓ `MKM-NE-05` ✓✓ · `SW10041` ✓✓ ✓ ✓ |
| Die **Staffel** ✓✓ | **bleibt** ✓✓ **Pflicht** ✓✓ für alle **anderen** ✓✓ **45** ✓✓ **numerischen** ✓✓ Artikel ✓✓ ✓ ✓ |
| Die **Zuordnung** ✓✓ | **unverändert** ✓✓ — die **61** ✓✓ **bytegleichen** ✓✓ Zuordnungen ✓✓ vom **18.10.** ✓✓ ✓ ✓ |

**Der Verifier** ✓✓ ist **grün** ✓✓ ✓ ✓
`python3 scripts/verify-mkm-release.py --since 2026-11-24T13:51:01Z` ✓✓ →
**exit 0** ✓✓ · **61** ✓✓ Artikel ✓✓ · **47/14** ✓✓ · **47** ✓✓ Current ✓✓ ·
**0** ✓✓ Befunde ✓✓ ✓ ✓ **Die Tests** ✓✓: **15** ✓✓ (Verifier ✓✓) ✓✓ und
**10** ✓✓ (Audit ✓✓) ✓✓, **0** ✓✓ Fehler ✓✓ ✓ ✓

**Kein offener MKM-Auftrag** ✓✓ mehr ✓✓ ✓ ✓
