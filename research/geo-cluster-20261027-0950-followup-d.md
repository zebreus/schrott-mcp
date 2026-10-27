# Essener GEO-Cluster: Nachprüfung D, 27.10.2026 (Rechercheagent, read-only)

Geprüfte Fälle: `nw-essen-mre-metall-recycling-essen`, `nw-essen-schrott-busters`,
`nw-essen-schrott-nowak`, `nw-essen-schrotthandel-missal-metzler`. Der Ausgangspunkt
`51.450284375, 7.015053125` wurde als Fixwert übernommen, nicht erneut aus der
Produktionsdatenbank gelesen. Es gab keine Dateiänderung, keinen Commit, keinen Push,
keinen Deploy und keinen Produktionsdatenbankzugriff.

## Kernbefund zum gemeinsamen Punkt

Die Nominatim-Rückwärtssuche vom 27.10.2026 löst den Punkt auf die Straße **„Freiheit",
Sternviertel/Südviertel, 45128 Essen** auf (OSM way 46961889, `highway=service`,
`bus=designated`, `vehicle=no`, Beschreibung „Fernbus Linien und Reisebusse"). Der Punkt
liegt damit auf der **Fernbus-Spur am Essener Hauptbahnhof** (nächster benannter Punkt:
UBahn-Knoten Essen Hauptbahnhof 51.4501992/7.0126609) und **nicht** auf einem
Gewerbestandort. Für alle vier Dossierzeilen ist der gespeicherte Punkt damit nicht nur
„nicht zuordenbar“, sondern **affirmativ falsch**. Keine Quelle nennt „Freiheit" als
Betreiber- oder Annahmeort.

Die Umweltinspektionsberichte der Bezirksregierung Düsseldorf für die Stadt Essen
(alle am 27.10.2026 einzeln abgerufen) enthalten keinen Eintrag auf der Straße „Freiheit"
und keinen zu Bonifaciusring 5, Fischweiher 26, Bonnekampstraße 14, Marktstraße 14,
Kalthofweg 4 oder Zum Oberhof 32.

## Einzelfälle

### `nw-essen-mre-metall-recycling-essen`: `clear_fields: geo` empfohlen

- **Belegter Standort:** Bonifaciusring 5, 45309 Essen (Kray), Betriebsstandort mit
  Waagen und Anlieferung. Betreiberseite nennt „Das Unternehmen befindet sich auf dem
  Bonifaciusring 5 45309 Essen", „Entladehilfen auf unserem Betriebsgelände",
  „LKW Waage bis 40 t", Gründung 2011 durch Ibrahim EL Osman, zweite Generation Zakaria
  El Osman; Impressum ohne HRB, Steuernummer 111/5062/5152.
  Quellen: https://mre-essen.de.tl/ , https://mre-essen.de.tl/Unsere-Leistungen.htm ,
  https://mre-essen.de.tl/Impressum.htm , https://mre-essen.de.tl/Anfahrt.htm
- **Unabhängige Bestätigung:** Das Verlagsverzeichnis Das Örtliche führt „Metall
  Recycling Essen, Bonifaciusring 5, 45309 Essen", Branche „Schrottplatz", identische
  Telefonnummer 0201 47 61 64 18.
  Quelle: https://www.dasoertliche.de/Themen/Metall-Recycling-Essen-Essen-Kray-Bonifaciusring
- **Abstand zum gespeicherten Punkt:** 5,81 km. Nominatim-Forward „Bonifaciusring 5,
  45309 Essen" → OSM way 93518934, `building=yes`, 51.4761186/7.0879341 (Gewerbe-/Industriegebiet
  Kray, u. a. FW Metallservice mbH, Suether & Schoen GmbH, Bonifaciusring Grundbesitz GmbH & Co. KG).
- **Auflösung des Marken-Mixes:** „Häde"/„fast 70 Jahre" gehört zu Häde Recycling GmbH,
  Alte Bottroper Straße 120, 45356 Essen (anderes Unternehmen, anderer Standort, HRB-Feld
  dort nur Platzhalter). MRE selbst ist die 2011 gegründete El-Osman-Familie.
  Quelle: https://www.haede-recycling.de/impressum
- **Restlücken:** kein Registerbeleg (Einzelunternehmer ohne HRB), Verhältnis MRE zu Häde
  nicht belegt, kein Behördenbeleg einer Annahmestelle, keine Preisliste, keine
  Einfahrt-/Gate-Präzision (Nominatim liefert nur den Gebäudezentroid).
- **Disposition:** `clear_fields: geo`, **keine** Ersatzkoordinate. Der Punkt ist affirmativ
  falsch; die Ersetzung durch den Bonifaciusring-Zentroid würde einen Kundenstandort
  behaupten, den die Quellen nicht belegen.

### `nw-essen-schrott-busters`: `clear_fields: geo` empfohlen

- **Belegte Geschäftsanschrift:** Fischweiher 26, D-45307 Essen (Kray). Betreiber-Impressum
  nennt „Schrott Busters / Oliver Wojke / Fischweiher 26 / D – 45307 Essen",
  kontakt@schrott-busters.de, Tel. 0177 610 50 96, Steuernummer 111/5329/3605, kein HRB.
  Quelle: https://schrott-busters.de/kontakt/impressum/
- **Unabhängige Bestätigung:** 11880 führt „Schrott Busters Oliver Wojke, Fischweiher 26,
  45307 Essen (Kray)" mit identischer Telefonnummer, Kategorie Altmetallrecycling,
  Eintrag aktualisiert 15.10.2026.
  Quelle: https://www.11880.com/branchenbuch/essen-ruhr/060440092B112411338/schrott-busters-oliver-wojke.html
- **Abstand zum gespeicherten Punkt:** 4,38 km. Nominatim-Forward „Fischweiher 26,
  45307 Essen" → OSM way 100180825, `building=residential`, 51.4660509/7.0729867; kein
  erkennbarer Gewerbehof, keine belegte Hofannahme (nur mobile Abholung).
- **Restlücken:** kein Register-/Behördenbeleg, keine öffentlichen Abholgrenzen oder
  Vergütung, Mobilnummer ggf. nicht ortsfest.
- **Disposition:** `clear_fields: geo`, **keine** Ersatzkoordinate und keine
  Annahmestellen-Behauptung.

### `nw-essen-schrott-nowak`: `clear_fields: geo` empfohlen, zusätzlich Adressfehler

- **Belegte Adresse:** **Bonnekampstraße 14, 45327 Essen (Katernberg)**. Das
  Betreiber-Impressum im Webarchiv (06.02.2016) nennt „Inhaber und Verantwortlich für den
  Inhalt: Miroslaw Nowak, Bonnekampstrasse 14, 45327 Essen", Amtsgericht Essen,
  Steuernummer DE260095030. Quellen:
  http://web.archive.org/web/20160206233754/http://schrott-nowak.de/Impressum.htm ,
  http://web.archive.org/web/20160207002041/http://schrott-nowak.de/Startseite.htm
- **Unabhängige Bestätigung:** 11880 führt „Schrott-Nowak.de Miroslaw Nowak,
  Bonnekampstr. 14, 45327 Essen (Katernberg)", Tel. (0176) 65336367, Kategorien
  Altmaterialhandel/Schrotthandel/Altmetallrecycling, Eintrag 30.12.2022.
  Quelle: https://www.11880.com/branchenbuch/essen-ruhr/060440090B102234372/schrott-nowak-de-miroslaw-nowak.html
- **Behördliche Bestätigung der USt-IdNr:** Die vom Betreiber veröffentlichte USt-IdNr
  DE260095030 ist über den EU-VIES-Dienst am 27.10.2026 gültig (`isValid: true`); Name und
  Anschrift werden vom Mitgliedstaat nicht offengelegt, die VIES prüft also keine Adresse.
  Quelle: https://ec.europa.eu/taxation_customs/vies/rest-api/ms/DE/vat/260095030
- **Der Seed-Lead „Marktstr. 14" ist widerlegt:** In OSM existiert in Essen nur eine
  „Marktstraße", in **45355** (Borbeck, way 161727939, 51.4724271/6.9502561); „Marktstr. 14,
  45327" existiert als Kombination nicht.
- **Domain nicht mehr auflösbar:** `schrott-nowak.de` und `www.` scheitern an der
  DNS-Auflösung (HTTP 000); letzte Archivfassung 2017, 11880-Daten von 2022.
- **Abstand zum gespeicherten Punkt:** Bonnekampstr. 14 → 5,84 km; Marktstr. 14 → 5,12 km.
  Nominatim-Forward „Bonnekampstraße 14, 45327 Essen" → way 276495449, `building=apartments`,
  51.4914352/7.0675323 (Wohnobjekt, kein Gewerbehof).
- **Disposition:** `clear_fields: geo`, **keine** Ersatzkoordinate (kein belegter aktueller
  Betrieb, Domain nicht auflösbar). Der aktuelle Betriebsstatus bleibt unbekannt; das
  Statusfeld wird nicht angefasst. **Zusätzlich offener Adressfehler:** Die
  Frontmatter-`street` lautet aktuell „Schrott Nowak Essen Marktstr. 14" und ist damit
  sowohl unvollständig (Name und Stadt im Adressfeld) als auch sachlich falsch. Das ist
  eine Adress- und keine GEO-Entscheidung und braucht eine eigene Begründung
  (Umstellung auf „Bonnekampstraße 14" mit PLZ 45327 oder Leerwert).

### `nw-essen-schrotthandel-missal-metzler`: `clear_fields: geo` empfohlen

- **Zwei belegte Adressen, beide in einer Betreiber-Sphäre:**
  - **Kalthofweg 4, 45277 Essen (Überruhr-Hinsel)** aus dem Betreiber-Impressum
    („Schrotthandel Schrottabholung, Jürgen Missal, Kalthofweg 4, 45277 Essen,
    Vertretungsberechtigt: Markus Metzler", Tel. 0170 3547895, USt-ID derzeit nicht erteilt)
    und dem unabhängigen 11880-Eintrag „Schrotthandel Jürgen Missal, Kalthofweg 4,
    45277 Essen", Kategorie Altmaterialhandel, Eintrag 24.02.2026.
    Quellen: https://schrotthandel-metzler-essen.de/impressum/ ,
    https://www.schrotthandel-missal-essen.de/impressum/ ,
    https://www.11880.com/branchenbuch/essen-ruhr/060440090B101936723/schrotthandel-juergen-missal.html
  - **Zum Oberhof 32, 45307 Essen (Kray)** aus dem 11880-Eintrag „Markus Metzler
    Schrotthandel, Zum Oberhof 32, 45307 Essen", der die im Dossier gespeicherte Domain
    trägt; die Betreiberseiten erwähnen „Zum Oberhof" nirgends.
    Quelle: https://www.11880.com/branchenbuch/essen-ruhr/120672194B37573047/markus-metzler-schrotthandel.html
- **Abstand zum gespeicherten Punkt:** Kalthofweg 4 → 5,17 km; Zum Oberhof 32 → 4,07 km.
  Beide Forward-Treffer sind Wohnobjekte (way 318494685 `building=yes`, 51.4257353/7.0783745;
  way 88583555 `building=apartments`, 51.4568962/7.0728579), kein belegter Annahmehof.
- **Restlücken:** Rollen- und Standortkonflikt Missal (Kalthofweg) ↔ Metzler (Zum Oberhof)
  ungelöst, kein Registerbeleg (kein HRB/HRA; das 11880-Label „Handelsgesellschaft" ist
  eine Verzeichniskategorie), keine öffentliche Hofannahme, keine Preisliste in EUR/kg.
- **Disposition:** `clear_fields: geo`, **keine** Ersatzkoordinate. Der Adress- und
  Rollenkonflikt bleibt offen und separat zu führen.

## Geprüfte Adress- und Geodaten (Nominatim, Abruf 27.10.2026)

| Adresse | Treffer | OSM | lat/lon | Abstand zum gespeicherten Punkt |
|---|---|---|---|---|
| Bonifaciusring 5, 45309 Essen | building | way 93518934 | 51.4761186 / 7.0879341 | 5,81 km |
| Fischweiher 26, 45307 Essen | building=residential | way 100180825 | 51.4660509 / 7.0729867 | 4,38 km |
| Bonnekampstraße 14, 45327 Essen | building=apartments | way 276495449 | 51.4914352 / 7.0675323 | 5,84 km |
| Marktstraße 14, 45355 Essen | building | way 161727939 | 51.4724271 / 6.9502561 | 5,12 km |
| Kalthofweg 4, 45277 Essen | building=yes | way 318494685 | 51.4257353 / 7.0783745 | 5,17 km |
| Zum Oberhof 32, 45307 Essen | building=apartments | way 88583555 | 51.4568962 / 7.0728579 | 4,07 km |
| Freiheit, 45128 Essen (gespeicherter Punkt) | Straßenspur, Fernbus | way 46961889 | 51.450284375 / 7.015053125 | - |

## Nicht abrufbare oder nicht verifizierbare Quellen

- https://media.essen.de/media/wwwessende/aemter/59/umweltinspektionen/ → HTTP 403;
  nur per Namensmuster auffindbar, `2021-04-06_InspBericht.pdf` ohne Textebene (betrifft
  nur Rainer Ludwig GmbH & Co. KG, Eisenbahnstr. 1, also keinen der vier Fälle).
- https://www.haede-recycling.de/ → HTTP 403; Angaben nur über Suchmaschinen-Snippet.
- https://schrott-nowak.de/ → DNS nicht auflösbar; Ersatz nur das Webarchiv.
- https://www.handelsregister.de/ → nicht automatisierbar (Session-Fehler); alle vier
  Betreiber sind Einzelunternehmer ohne veröffentlichtes HRB/HRA, daher kein Registertreffer
  zitierbar.
- https://www.companyhouse.de/ , https://openregister.de/ → HTTP 403.
- Overpass-API (overpass-api.de / .ch / private.coffee) → 502 bzw. Timeout; Umfeldprüfung
  ersatzweise über Nominatim und die OSM-Way-Tags.
- https://essen.city-map.de/de/metall-recycling-essen → Direktabruf Timeout, nur Snippet.
- VIES → erreichbar, `VALID`, aber ohne offengelegten Namen und ohne Anschrift.

## Empfehlung an den Owner

Viermal `clear_fields: 'geo'` ohne Ersatzkoordinate in den Dossiers
`nw-essen-mre-metall-recycling-essen`, `nw-essen-schrott-busters`,
`nw-essen-schrott-nowak` und `nw-essen-schrotthandel-missal-metzler`, jeweils mit
datiertem Timeline-Eintrag; zusätzlich bei `nw-essen-schrott-nowak` die
Frontmatter-`street` als eigene Adressentscheidung bereinigen. Die Rücknahme ist keine
Aussage über Schließung, Annahme oder Materialien.
