---
slug: nw-hagen-erb
name: ERB
trader_type: mobil
state: NW
city: Hagen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.erbhandel.de/
website_status: aktiv
status: pruefung
clear_fields: 'geo'
description: 'ERB Handel GmbH, HRB 11726 AG Hagen, aktiv, gegründet 13.11.2020; Firmenadresse Döinghauser Straße 31, 58332 Schwelm (Sitzverlegung aus Hagen), der 11880-Eintrag führt noch die frühere Hagen-Adresse Unterm Hagen 10, 58119 Hagen mit der Betreiber-E-Mail info@erbhandel.de. Betreiberportal erbhandel.de: Handel mit NE-Schrott (Kupfer, Messing, Aluminium, Zinn, Bronze), keine Preise veröffentlicht. Der Seed-Ort Hagen entspricht dem Registergericht, nicht der heutigen Betriebsadresse.'
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# ERB

## Überblick

Ein möglicher Kandidat zur vagen Seed-Zeile „ERB“ ist die ERB Handel GmbH, HRB 11726 (AG Hagen): North Data führt sie aktuell mit Sitz Döinghauser Str. 31, 58332 Schwelm und dokumentierter Anschriftenänderung 2022 aus Hagen. Die Betreiberwebsite bestätigt den NE-Metallhandel und nennt E-Mail sowie Mobiltelefon, enthält im gelesenen Auftritt aber keine eindeutige Hagen-Annahmestelle bzw. keinen vollständigen Abgleich der Seed-Identität. Daher keine Frontmatter-Übernahme und kein Schluss, dass der frühere Hagen-Bezug die aktuelle Seed-Firma ist.

**ANKAUF:** Betreiberseite nennt Kauf von NE-Schrott, aber nur flexible/tagesmarktbezogene Angebote ohne Zahlenliste. **VERKAUF:** Rohstoffversorgung wird als Großhandelsleistung beschrieben; keine Kunden- oder Verkaufspreisliste. **GEBÜHREN:** keine Tarifangaben gefunden. **GEO:** nicht geokodieren, bis die Seed-Zuordnung und ein aktueller Kunden-/Annahmestandort belegt sind; der Register-Sitz in Schwelm ist kein Hagen-Pin.

## Timeline

### Korrektur 24.10.2026: Gemeinsamen falschen Hagen-Punkt zurückziehen

- [Korrektur 24.10.2026: Die Produktionszeile `nw-hagen-erb` enthielt den mit mehreren anderen Hagen-Datensätzen geteilten Punkt `51.90933, 7.88825`. Die read-only-Rückwärtssuche löst ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351 auf, nicht in Hagen. `clear_fields: 'geo'` zieht den falschen Punkt zurück; mangels belegter Händleradresse wird keine Ersatzkoordinate gesetzt. Die Produktionsdatenbank blieb unverändert. Quelle: https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/way/81516036.]

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Hagen

### Recherche 02.10.2026

- [Recherche 02.10.2026: kein belastbarer Treffer — gezielte Suche nach ERB + Schrott/Entsorgung/Container/Altauto in Hagen ohne zuordenbares Ergebnis (nur HEB Hagener Entsorgungsbetrieb, HP Metalle, Bötzel, generische Klüngelskerl-Portale); keine Betreiber-Website, kein Register-, kein Verzeichnisbeleg; keine Frontmatter-Fills (außer website_status unbekannt), Status pruefung bleibt, Klärfall für Folgewelle; Quelle(n): keine Belegquelle]

### Recherche 20.10.2026

- [Recherche 20.10.2026: Neue Kandidatenspur: Die Betreiberseite bezeichnet ERB Handel GmbH als Händler/Käufer und Lieferant von NE-Metallschrott, nennt info@erbhandel.de, edgaras.kancaitis@erbhandel.de und +49 176 63788764; die Seite führt ©2021 und liefert keine konkret ausgewiesene Hagen-Annahme. North Data führt ERB Handel GmbH, AG Hagen HRB 11726, Gegenstand NE-Metallschrotthandel, aktuell Döinghauser Str. 31, 58332 Schwelm; die Registerhistorie vermerkt den Adresswechsel am 16.12.2022. Name plus früherer Hagen-Sitz sind ein plausibler Kandidat, aber mangels Verknüpfung zum Seed „ERB“ kein Identitätsbeweis; Felder/Status bleiben deshalb unverändert. ANKAUF: NE-Schrottkauf behauptet, Angebote flexibel/marktorientiert, keine numerische oder datierte Preisliste. VERKAUF: Belieferung von Rohstoffindustrien beschrieben, keine Endkunden-Verkaufspreisliste. GEBÜHREN: keine Liste/kein pauschal kostenloses Angebot gefunden. GEO offen: aktuelle Anschrift ist Schwelm, nicht nachgewiesener Hagen-Kundenpunkt. Quellen: https://www.erbhandel.de/ ; https://www.northdata.de/ERB%20Handel%20GmbH,%20Schwelm/Amtsgericht%20Hagen%20HRB%2011726]

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Für **ERB** (ERB Handel GmbH) ist **keine**
  Anschrift in Hagen dokumentiert; der Sitz wurde nach Schwelm verlegt. **Was live geprüft wurde (03.11.2026):**
  (a) Das Kachelraster **7.4200/51.3400 – 7.5600/51.4200** (112 Kacheln à 0,01 Grad, live abgerufen über
  `https://api.openstreetmap.org/api/0.6/map.json?bbox=…`, **0 Kachel-Fehler**) wurde vollständig ausgewertet: eine Suche
  über `name`, `operator`, `brand`, `website`, `contact:website`, `description`, `note`, `short_name` und
  `official_name` nach **„ERB“**, **„ERB Handel“** und **„erbhandel“** liefert **0** Treffer — Aussage zum
  OSM-Bestand, **kein** Negativbeleg über den Betrieb. (b) Ein Objekt mit `addr:housename` oder Gewerbebezug zu ERB
  existiert im Raster nicht. Quelle 1 (Betreiber/Register): die Betreiberseite **erbhandel.de** bezeichnet die
  **ERB Handel GmbH** als Händler/Käufer und Lieferant von NE-Metallschrott und nennt **info@erbhandel.de** und
  **+49 176 63788764**; **NorthData** führt **ERB Handel GmbH, AG Hagen HRA 11726** mit aktuellem Sitz
  **Döinghauser Str. 31, 58332 Schwelm** und vermerkt den **Adresswechsel zum 16.12.2022**. Das Registergericht
  **AG Hagen** ist damit **kein** Sitz- oder Betriebsort in Hagen, und eine frühere Hagen-Straßenanschrift ist im
  Dossier **nicht** benannt; der Name plus der frühere Hagen-Bezug sind ein plausibler Kandidat, aber **kein**
  Identitätsbeweis für den Seed „ERB“. Quelle 2 (OSM): kein Objekt mit Betriebsbezug zu ERB im live abgerufenen
  Hagen-Raster; die **aktuelle** Schwelm-Adresse wurde in dieser Welle **nicht** übernommen (anderer Ort, andere Zeile).
  **Was fehlt:** eine belegte Anschrift in Hagen **oder** eine Entscheidung, die Zeile auf den Schwelm-Standort
  umzuschreiben; ein auf Schwelm gesetzter Punkt wäre hier geraten. `clear_fields: 'geo'` bleibt bestehen,
  `status: pruefung` bleibt unverändert, es wird weder Fortbestand noch Annahme behauptet.
  Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.4200,51.3400,7.5600,51.4200 ;
  https://www.erbhandel.de/ ;
  https://www.northdata.de/ERB%20Handel%20GmbH,%20Schwelm/Amtsgericht%20Hagen%20HRB%2011726 (Abruf 03.11.2026).]

### Recherche 11.11.2026

- [Recherche 11.11.2026: **Die Seed-Zeile „ERB“ in Hagen ist die ERB Handel GmbH (HRB 11726, AG Hagen), deren Betriebsadresse inzwischen in Schwelm liegt, und der Betreiberauftritt gehört zu dieser Gesellschaft.** (1) Das amtliche Handelsregisterportal führt „ERB Handel GmbH“, Amtsgericht **Hagen**, **HRB 11726**, Status **aktiv**, Firmenadresse **Döinghauser Straße 31, 58332 Schwelm**, Gründung 13.11.2020, eine Registerbekanntmachung mit letzter Änderung vom 13.11.2020: das Registergericht passt zum Seed-Ort Hagen, der Sitz ist jedoch verlegt. (2) Der 11880-Eintrag „ERB Handel GmbH“ führt die **frühere** Hagen-Adresse **Unterm Hagen 10, 58119 Hagen** mit der E-Mail **info@erbhandel.de**; das Betreiberportal erbhandel.de (HTTP 200) nennt in der Fußzeile „All rights reserved 2021 © ERB Handel GmbH | info@erbhandel.de | edgaras.kancaitis@erbhandel.de | tel. +4917663788764“ und damit **dieselbe** E-Mail und **denselben** Firmennamen, die Seite beschreibt den „Professional Trading of Non-Ferrous metal scrap“.
  `website` und `website_status` wurden entsprechend gesetzt; `street` und `postcode` bleiben leer, weil das Portal **kein** Impressum und **keine** Anschrift nennt und die aktuelle Betriebsadresse in Schwelm liegt, `clear_fields: 'geo'` bleibt bestehen, `description` wurde um den Registerbefund ergänzt. (3) Inhalt des Portals: /services/ und /services-wholesale-non-ferrous-scrap/ nennen als Handelssortiment **Kupfer, Messing, Aluminium, Zinn (Sn), Bronze**, /about-erb-handel-gmbh/ beschreibt „purchase to transport organization to payment“ und individuelle Angebote zu „fair and competitive prices“ auf Basis des nationalen und globalen Marktes; der Stand ist inhaltlich von 2021 und damit kein Aktualitätsnachweis.
  (4) Im amtlichen Fachbetrieberegister (Filter „Art des Betriebs = Egal“) liefern „ERB Handel“ **7** und „ERB Handel GmbH“ **3 Treffer** (H.L.R. Herbert Lagerin Rohstoffhandel GmbH, Currenta GmbH & Co. OHG), **kein** zertifizierter Betrieb mit diesem Namen; die Standortsuche um **Döinghauser Straße 31, 58332 Schwelm** mit 3 km liefert **17 Efb-Treffer** (Günter Allermann GmbH, Hans-Dieter Ahrens GmbH, Michael Quel Altkleidersammlungen) und **1 AfV-Treffer** (Heinz Lusch GmbH & Co. KG, Wuppertal), um **Hagen** mit 3 km **8 Efb-Treffer** (Fass-Braun GmbH, HEB GmbH Hagener Entsorgungsbetrieb, HUI GmbH), sämtlich Fremdbetriebe.
  **Ankauf:** Ankauf von NE-Schrott mit individuellen Angeboten nach Markt, **ohne** veröffentlichte €/kg-Kurse; **Verkauf:** das Handelssortiment Kupfer, Messing, Aluminium, Zinn, Bronze, **ohne** Preise; **Gebühren:** keine Tarife. **Offen** bleibt, ob der Seed-Eintrag den Betrieb in Hagen oder den in Schwelm meint. Quelle(n): https://www.online-handelsregister.de/handelsregisterauszug/nw/Hagen/HRB/11726/ERB-Handel-GmbH ; https://www.11880.com/branchenbuch/hagen-westfalen/012070452B112479349/erb-handel-gmbh.html ; https://www.erbhandel.de/ ; https://www.erbhandel.de/services/ ; https://www.erbhandel.de/about-erb-handel-gmbh/ ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=D%C3%B6inghauser%20Stra%C3%9Fe%2031%2C%2058332%20Schwelm…&dst=3&is=1&e=1 (Abruf 11.11.2026).]
