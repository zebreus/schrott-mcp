---
slug: ni-alfeld-i-d-schrotthandel
name: I&D Schrotthandel
trader_type: schrotthaendler
state: NI
city: Alfeld
street: 'Föhrster Str. 31'
postcode: '31061'
phone: '+49 176 62769357'
email: 'irmakirvelaitiene@gmail.com'
opening_hours: ''
website: https://id-schrotthandel.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: '{"allowed": true, "conditions": "Kostenlose Abholung (auch grosse Mengen, privat und gewerblich), Karossen-Annahme, Demontage, Containerstellung; Bar oder Ueberweisung zu Tagespreisen (Betreiber-Website, Feedback-Triage 01.10.2026)"}'
certifications: ''
provenance_section: Weitere Verzeichnis-Funde (Nienburg/Diepholz/Verden, Osnabrück-Nord,
  Vechta/Cloppenburg, Peine, Stade, Gifhorn, Lüneburg
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# I&D Schrotthandel

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website live und tiefgecrawlt (Start, Über-uns, Preise, Kontakt, Impressum einzeln abgerufen). Angebot: Ankauf/Abholung Altmetall, Alu/Kupfer/Kabel/Messing, kostenlose Abholung, Karossen-Annahme, Demontage, Containerstellung; Schrottplatz Föhrster Str. 31, 31061 Alfeld; Tagespreise-Seite vorhanden (Preise tagesaktuell, daher nicht übernommen). Website_status aktiv (eigener Live-Abruf). Impressum (Einzelbeleg, unsicher — Einzelunternehmen I&D Metallhandel, Inh. Irma Kirvelaitiené, keine HRB, Owner-Direktive nicht anwendbar): Föhrster Straße 31, 31061 Alfeld, Tel. +49 176 62769357. Kontaktfelder mangels Zweitbeleg leer gelassen — Klärfall für Folgewelle. Quelle(n): https://www.id-schrotthandel.de, https://www.id-schrotthandel.de/impressum, https://www.id-schrotthandel.de/ueber-uns.]

- [Audit-2 2430 01.10.2026: Audit-2 Feedback 2430 (lat/lon Bayern statt Alfeld + Kontaktdaten fehlten): Betreiber-Adresse belegt Alfeld (Geokoordinaten damit ueberholt, Neu-Geocodierung adressbasiert automatisch) - I&D Metallhandel, Inh. Irma Kirvelaitiene, Schrottplatz Foehrster Str. 31, 31061 Alfeld; Abdeckung LK Hannover/Holzminden/Hameln/Bad Pyrmont (Home + Impressum/Kontakt doppelbestaetigt als starke Quelle per Beleg-Leitlinie; frueherer Folgewellen-Vorbehalt damit geheilt); Quelle(n): https://id-schrotthandel.de Home + Impressum, 01.10.2026]
- [Audit-2 2430 01.10.2026: Eingearbeitet: street/postcode/phone/email/pickup_json]

### Importiert (Seed-Stand 2026-09-30)

- Alu/Kupfer/Kabel/Messing, Demontage

### Recherche 02.11.2026

- [Recherche 02.11.2026: **Zertifikats-Recherche ohne Treffer.** Für die `I&D Schrotthandel` (Föhrster Str. 31, 31061 Alfeld, Tel. +49 176 62769357; laut Dossier mit eigener Website id-schrotthandel.de, aktuelle öffentliche Annahme im Register nicht bestätigt) wurde im amtlichen Fachbetrieberegister weder ein Efb-Zertifikat noch ein AltfahrzeugV-Eintrag gefunden; `certifications` bleibt leer. Suchen und Befunde (live am 02.11.2026, clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt): (1) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=I&D` -> **0 Treffer**, ebenso `…/Altfahrzeugverwertung/dst=10&is=1&e=1&n=I&D` -> **0 Treffer** (Filter „Art des Betriebs“ = Demontagebetrieb); (2) Kontrollsuche mit dem generischen Suchbegriff `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrotthandel` -> **111 Treffer** und `…/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrotthandel` -> **13 Treffer** (u. a. `LSH Lübecker Schrotthandel GmbH | Zentrale`, Dampfpfeife 12-20, 23569 Lübeck; `KSH Kieler Schrotthandel GmbH | Betrieb Kiel`, Ostuferhafen 9, 24149 Kiel; `Johann Helfert Schrotthandel GmbH & Co. KG (Annahmestelle und Demontagebetrieb für Altfahrzeuge)`, Daimlerstraße 4, 91161 Hilpoltstein) - **Namensvetter, kein Treffer in Alfeld**; (3) Radiusabfrage mit **3 km** um die Dossier-Adresse `…/Entsorgungsfachbetriebe/adr=Föhrster Str. 31, 31061 Alfeld, Deutschland;…&dst=3&is=1&e=1` -> **0 Treffer**, dieselbe Abfrage in der Route `…/Altfahrzeugverwertung/adr=…&dst=3&is=1&e=1` -> **0 Treffer**; (4) **Positivkontrolle** mit **10 km** Radius -> `…/Entsorgungsfachbetriebe/adr=…&dst=10&is=1&e=1` -> **4 Treffer** (`Zimmermann Recycling Alfeld, Inh. Giuliano Mario Zimmermann`, Industriestraße 11, 31061 Alfeld, 3,2 km, `ZZBT00500354 / 1` bis `/ 3` - **anderer Rechtsträger am selben Ort**; `HeiTec ROHSTOFFE GmbH`, Untere Hilsstraße 23b, 31073 Delligsen OT Grünenplan, 5,9 km, `ZZEE00100131 / 1`) und `…/Altfahrzeugverwertung/adr=…&dst=10&is=1&e=1` -> **0 Treffer** - die Abfragetechnik funktioniert also, an der Föhrster Str. 31 und im Umkreis 3 km gibt es jedoch **keinen** Registereintrag.
    Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt.
    Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=I%26D ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=I%26D ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrotthandel ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrotthandel ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=F%C3%B6hrster%20Str.%2031,%2031061%20Alfeld,%20Deutschland;9.8230804;51.9766119;9.893080595028785;52.01973083639957;9.753080204971216;51.933492963600436&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=F%C3%B6hrster%20Str.%2031,%2031061%20Alfeld,%20Deutschland;9.8230804;51.9766119;9.893080595028785;52.01973083639957;9.753080204971216;51.933492963600436&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=F%C3%B6hrster%20Str.%2031,%2031061%20Alfeld,%20Deutschland;9.8230804;51.9766119;10.056414383429281;52.12034168799856;9.58974641657072;51.83288211200144&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=F%C3%B6hrster%20Str.%2031,%2031061%20Alfeld,%20Deutschland;9.8230804;51.9766119;10.056414383429281;52.12034168799856;9.58974641657072;51.83288211200144&dst=3&is=1&e=1 (Abruf 02.11.2026).]
