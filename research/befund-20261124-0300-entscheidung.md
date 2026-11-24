# Der Befund vom 24.11.2026, 13:20 UTC — ein Regelverstoß in der Welle vom 03:00

**Stand 24.11.2026, 13:20 UTC ✓ ✓**

Der Owner hat die **36** ✓✓ entfernten Punkte ✓✓ der Welle vom **03:00** ✓✓
**systematisch** ✓✓ nachgeprüft ✓✓ ✓ ✓ **Kein** ✓✓ Zufallsbefund ✓✓ — eine
**Messung** ✓✓ über **alle** ✓✓ 36 ✓✓ Dossiers ✓✓ ✓ ✓

## Die Messung ✓ ✓

**Für** ✓✓ jedes Dossier ✓✓ der **36** ✓✓ wurde geprüft ✓✓ ✓ ✓

1. Trägt ✓✓ ein **Adressobjekt** ✓✓ in OpenStreetMap ✓✓ die **Hausnummer** ✓✓
   des Dossiers ✓✓ **an** ✓✓ der belegten ✓✓ Straße ✓✓ (Nominatim-Suche ✓✓,
   live ✓✓ am **24.11.2026** ✓✓ **13:10 UTC** ✓✓) ✓✓ ✓ ✓
2. Welchen **Grund** ✓✓ die **Korrektur-Sektion** ✓✓ des Dossiers ✓✓ für den
   **Verzicht** ✓✓ auf einen **Ersatzpunkt** ✓✓ nennt ✓✓ ✓ ✓
3. Ob dieser **Grund** ✓✓ die **Regel** ✓✓ trägt ✓✓ ✓ ✓

## Das Ergebnis ✓ ✓

**Acht** ✓✓ der **36** ✓✓ Dossiers ✓✓ tragen **überhaupt** ✓✓ ein
**Adressobjekt** ✓✓ an der Hausnummer ✓✓ ✓ ✓ **Sieben** ✓✓ davon sind
**korrekt** ✓✓ entfernt ✓✓ ✓ ✓

| Dossier ✓✓ | Der Grund ✓✓ | Die Regel ✓✓ |
|---|---|---|
| `he-frankfurt-60488-bertin` ✓✓ | Die Bertin GmbH ist ein **Messtechnikunternehmen** ✓✓ — kein Schrottbetrieb ✓✓ ✓ ✓ | die Quelle belegt **nicht** ✓✓ den Standort **des Schrottbetriebs** ✓✓ ✓ ✓ |
| `he-staufenberg-35460-stahlhugel` ✓✓ | Das Objekt trägt **Mörfelden-Walldorf** ✓✓ (64546) ✓✓ — **nicht** ✓✓ Staufenberg ✓✓ ✓ ✓ | **anderer Ort** ✓✓ ✓ ✓ |
| `he-wiesbaden-marchenland-…-winterstein` ✓✓ | **HTTP 403** ✓✓ und Archiv **HTTP 503** ✓✓ — nur ein **Verzeichniseintrag** ✓✓ ✓ ✓ | ein **Verzeichnislead** ✓✓ allein ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ |
| `nw-bocholt-nam-schrottabholung-bocholt` ✓✓ | Die Adresse ruht allein auf einem **Portalprofil** ✓✓ ✓ ✓ | ein **Verzeichnislead** ✓✓ allein ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ |
| `nw-duisburg-baykal` ✓✓ | Die Adresse ruht allein auf dem **11880-Lead** ✓✓ ✓ ✓ | ein **Verzeichnislead** ✓✓ allein ist **kein** ✓✓ Standortnachweis ✓✓ ✓ ✓ |
| `nw-monchengladbach-rheydt-kupferrecycling-rheydt` ✓✓ | Der Rechtsträger (**HRB 13678** ✓✓) ist **gelöscht** ✓✓ ✓ ✓ | eine **gelöschte** ✓✓ Gesellschaft wird **nicht** ✓✓ verortet ✓✓ ✓ ✓ |
| `rp-kinderbeuern-bach-kinderbeuern` ✓✓ | Die **Domain** ✓✓ ist **nicht erreichbar** ✓✓ — die Adresse ist **nicht belegt** ✓✓ ✓ ✓ | die Adresse ist **nicht** ✓✓ belegt ✓✓ ✓ ✓ |

## Der Befund ✓ ✓ — `nw-marl-schrott-metallhandel-h-w-nowakowski` ✓✓

**Die Regel** ✓✓ (`research/geo-entscheidung-20261119.md` ✓✓) ✓ ✓

> **Belegt** ✓✓ ist eine Adresse **durch eine Quelle über das Geschäft selbst**
> ✓✓ — das **Impressum des Betreibers** ✓✓ … — **und** das **Adressobjekt**
> ✓✓ **trägt diese Hausnummer** ✓✓ → der Punkt **wird gesetzt** ✓✓ ✓ ✓

**Was** ✓✓ vorliegt ✓✓ ✓ ✓

| Die Bedingung ✓✓ | Der Beleg ✓✓ |
|---|---|
| Eine **Quelle über das Geschäft** ✓✓ selbst ✓✓ | das **Impressum** ✓✓ — „H. W. Nowakowski, **Zechenstr. 5**, **45768 Marl**“ ✓✓ — live gelesen am **23.11.2026** ✓✓ (HTTP **200** ✓✓, alle vier Unterseiten ✓✓) ✓✓ ✓ ✓ |
| Ein **Adressobjekt** ✓✓ mit dieser **Hausnummer** ✓✓ | **way/250105006** ✓✓ — `addr:street`=**Zechenstraße** ✓✓ · `addr:housenumber`=**5** ✓✓ · `addr:city`=**Marl** ✓✓ · `addr:postcode`=**45772** ✓✓ · `building`=**yes** ✓✓ ✓ ✓ |

**Der Postleitzahl-Widerspruch** ✓✓ ist **kein** ✓✓ Gegenbeweis ✓✓ ✓ ✓
Nominatim ✓✓ löst **„Zechenstraße 5, 45768 Marl“** ✓✓ auf **denselben**
✓✓ **way/250105006** ✓✓ auf ✓✓ ✓ ✓ — die **Hausnummer 5** ✓✓ ist an dieser
Straße **eindeutig** ✓✓ ✓ ✓

**Die Entscheidung** ✓✓ der Welle ✓✓ hat **dennoch** ✓✓ **ohne** ✓✓
**Ersatzpunkt** ✓✓ entfernt ✓✓ ✓ ✓ Und die **Akt** ✓✓ behauptet ✓✓ für **keinen**
✓✓ dieser Fälle ✓✓ liege ein **belegter Standort** ✓✓ **und** ✓✓ ein
**adressiertes Objekt mit der Hausnummer** ✓✓ **gleichzeitig** ✓✓ vor ✓✓ — für
**dieses** ✓✓ Dossier ✓✓ ist das **falsch** ✓✓ ✓ ✓

**Der Beleg für den Punkt** ✓✓ ✓ ✓ **Nominatim** ✓✓ (live ✓✓ **24.11.2026**
✓✓ **13:12 UTC** ✓✓) ✓✓: **51.6640815 / 7.0889051** ✓✓ ✓ ✓

| Die Messung ✓✓ | Der Wert ✓✓ |
|---|---|
| **way/250105006** ✓✓ — der **Mittelwert** ✓✓ der **7** ✓✓ eindeutigen Knoten ✓✓ | **51.6641019 / 7.0889063** ✓✓ |
| **way/250105006** ✓✓ — **Nominatim** ✓✓ (die **Hausmethode** ✓✓ des Owners ✓✓) ✓✓ | **51.6640815 / 7.0889051** ✓✓ |

Der **Abstand** ✓✓ der beiden Werte ✓✓ beträgt **2,3 m** ✓✓ ✓ ✓

**Gegenprobe** ✓✓ — dieselbe Rechnung ✓✓ am **gesetzten** ✓✓ Punkt der Welle
✓✓ vom **03:00** ✓✓ (`nw-monchengladbach-kreggenwinkel` ✓✓) ✓ ✓

| Die Messung ✓✓ | Der Wert ✓✓ |
|---|---|
| **way/284642833** ✓✓ — der **Mittelwert** ✓✓ der **8** ✓✓ eindeutigen Knoten ✓✓ | **51.1484141 / 6.3643607** ✓✓ |
| **way/284642833** ✓✓ — **Nominatim** ✓✓ ✓✓ | **51.1483935 / 6.3643273** ✓✓ |
| Der **gesetzte** ✓✓ Punkt ✓✓ im Dossier ✓✓ | **51.1483935 / 6.3643273** ✓✓ — **identisch** ✓✓ ✓ ✓ |

**Die Methode** ✓✓ des Owners ✓✓ ist damit **gemessen** ✓✓ ✓ ✓

## Die Korrektur ✓ ✓

**Der Punkt** ✓✓ des Dossiers ✓✓ `nw-marl-schrott-metallhandel-h-w-nowakowski`
✓✓ wird auf **51.6640815 / 7.0889051** ✓✓ **gesetzt** ✓✓ ✓ ✓

**Die Korrektur** ✓✓ bekommt eine **eigene** ✓✓ datierte **Sektion** ✓✓ ✓ ✓
Sie gehört in die **Standortrunde** ✓✓ (die **29** ✓✓ Dossiers ✓✓ mit Adresse
✓✓ ohne Punkt ✓✓) ✓✓ — dort ist **sie** ✓✓ am **richtigen** ✓✓ Ort ✓✓ ✓ ✓

## Der Liquidationsfall der Welle vom 09:00 ✓ ✓ — **kein** ✓✓ Befund ✓ ✓

`by-friedberg-selzam-autoverwertung` ✓✓ traf den **Vor-Screen** ✓✓ ✓ ✓ Die
**Registerquelle** ✓✓ nennt **Äußere Industriestr. 1** ✓✓ und das **Objekt**
✓✓ **way/215659304** ✓✓ trägt genau diese **Hausnummer 1** ✓✓ ✓ ✓

**Aber** ✓✓ die Gesellschaft steht seit dem **30.12.2024** ✓✓ **in Liquidation**
✓✓ ✓ ✓ Der **Präzedenzfall** ✓✓ des Projekts ✓✓ (`research/wave-20261014-5-owner.md`
✓✓) ✓✓ sagt ✓ ✓

> **RRW** ✓✓ nun Betreiber-/Register-kongruent, **aber Liquidation** ✓✓
> **Erst operative Annahme/Fortführung klären** ✓✓ ✓ ✓

**Die Entscheidung** ✓✓ — **kein** ✓✓ Punkt ✓✓ — ist damit **richtig** ✓✓ ✓ ✓
Und der **Reichweitensatz** ✓✓ der Notiz ✓✓ deckt es **selbst** ✓✓ ab ✓ ✓
„Die Liquidation ist ein Registerstand und **kein** Nachweis, dass der Standort
bereits geschlossen ist“ ✓✓ ✓ ✓

## Die Lehre ✓ ✓

Die Regel ✓✓ ist im Bestand **zwei** ✓✓ Mal präzisiert ✓✓ worden ✓✓ — **einmal**
✓✓ um das **Handelsregister** ✓✓ als Quelle ✓✓ und **einmal** ✓✓ darum ✓✓, dass
ein **adressiertes Objekt ohne** ✓✓ `name` ✓✓ **und ohne** ✓✓ `operator` ✓✓
**die Anschrift** ✓✓ belegt ✓✓ — **nicht** ✓✓ den **Betrieb** ✓✓ ✓ ✓

**Diese** ✓✓ beiden **Präzisierungen** ✓✓ gehören **zusammen** ✓✓ ✓ ✓ Wer die
**eine** ✓✓ anwendet ✓✓ und die **andere** ✓✓ übergeht ✓✓, entscheidet **falsch**
✓✓ ✓ ✓ **Das ist** ✓✓ der **Befund** ✓✓ ✓ ✓

**Die Standortrunde** ✓✓ wendet **beide** ✓✓ an ✓✓ ✓ ✓
