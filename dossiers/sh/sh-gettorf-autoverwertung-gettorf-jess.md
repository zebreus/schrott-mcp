---
slug: sh-gettorf-autoverwertung-gettorf-jess
name: Autoverwertung Gettorf (Jess)
trader_type: autoverwertung
state: SH
city: Gettorf
street: Hasselrott 67
postcode: '24214'
phone: 04346/5355
email: 'robert-jess@gmx.de'
opening_hours: 'Mo-Fr 09:00-18:00, Sa 09:00-12:00'
website: https://autoverwertung-jess-gettorf.de/
website_status: aktiv
status: pruefung
description: 'Autoverwertung Gettorf (Robert Jess, USt-IdNr DE 203288353), Hasselrott 67, 24214 Gettorf, Tel. 04346/5355, Fax 04346/298099, E-Mail robert-jess@gmx.de. Seit 1968 in Gettorf tätig; anerkannter Demontagebetrieb nach Altfahrzeug-Verordnung, Zertifizierung jährlich durch den TÜV Nord. Leistungen: umweltgerechte Demontage und Verwertung von Altfahrzeugen, Verkauf demontierter Gebrauchtteile (eigener eBay-Shop „Autoverwertung Jess“), Fahrzeuge zum Selbstausbau, Schrotthandel. ANKAUF: keine öffentliche Ankaufspreisliste. VERKAUF: Gebrauchtteile und Restfahrzeuge als Einzel-Exposé (Beispiel Startseite: Porsche 911T EZ. 07.1970, 13500 Euro; Leistungsseite: VW Scirocco 3 EZ. 2009, 750 €), keine Preisliste. GEBÜHREN: keine Tarife.'
dropoff_json: '{"allowed": true, "customer_types": ["privat", "gewerbe"], "conditions": "Altfahrzeug-Annahme und Demontage am Standort Hasselrott 67, 24214 Gettorf, Mo-Fr 09:00-18:00, Sa 09:00-12:00. Anerkannter Demontagebetrieb nach Altfahrzeug-Verordnung (Zertifizierung jährlich durch den TÜV Nord). Keine öffentliche Ankaufspreisliste."}'
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: nur Fahrzeuge (Flag)
provenance_origin: table
---

# Autoverwertung Gettorf (Jess)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Altfahrzeug-Entsorgung, Gebrauchtteile

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website per Vollcrawl verifiziert (Altfahrzeug-Entsorgung, Gebrauchtteile; Öffnungszeiten Mo–Fr 09:00–18:00/Sa 09:00–12:00), website_status aktiv; Quelle(n): https://autoverwertung-jess-gettorf.de/, https://autoverwertung-jess-gettorf.de/Impressum/, https://autoverwertung-jess-gettorf.de/Kontakt/]
- [Recherche 01.10.2026: Adresse/Telefon (Robert Jess, Hasselrott 67, 24214 Gettorf, 04346/5355) nur Betreiber-Einzelbeleg (Einzelunternehmen ohne HRB) → kein Frontmatter-Fill; Mail widersprüchlich (Impressum/Kontakt: robert-jess@gmx.de vs. Footer: robert-jess@versanet.de) → ebenfalls kein Fill; Quelle(n): https://autoverwertung-jess-gettorf.de/Impressum/]
- [Feedback-2767 (berechtigt, eingearbeitet 01.10.2026): Adresse/Telefon/Zeiten per Betreiber-Einzelbeleg (Impressum/Kontakt: Hasselrott 67, 24214 Gettorf, 04346/5355; Zeiten Mo-Fr 09:00-18:00/Sa 09:00-12:00) in Frontmatter übernommen (Unsicherheit offengelegt: Einzelunternehmen ohne HR-Abgleich); E-Mail weiter leer (Widerspruch gmx.de vs. versanet.de); Quelle(n): https://autoverwertung-jess-gettorf.de/Impressum/, https://autoverwertung-jess-gettorf.de/Kontakt/]

### Recherche 31.10.2026

- [Recherche 31.10.2026: **OSM live geprueft (Hasselrott, Gettorf) - die Dossier-Hausnummer 67 existiert dort nicht.** Kachelraster ueber den **gesamten** Strassenverlauf (Strassenbbox 9.9630515/54.4035858 bis 9.9763288/54.4082695, **1 Kachel, 0 Kachel-Fehler**); die Bbox wurde aus den vier Strassenways mit `name=Hasselrott` gebildet (way/25572512, way/47071741, way/206541119, way/475440752). Zaehlung getrennt: **80 Adressobjekte / 79 verschiedene Hausnummern** auf der Hasselrott (1, 2, 3, 4, 4a, 5, 6, 7, 8, 9, 9a, 10, 11, 12, 13, 14, 15, 15a, 16, 17, 17a, 18, 19, 20, 20a, 21, 22a, 22b, 22c, 22d, 22e, 22f, 23, 24, 25, 26, 27, 27a, 28, 28a, 28b, 28c, 28d, 28e, 29, 29a, 30, 30a, 30b, 30c, 30d, 30e, 31, 32, 33, 34, 35, 35e, 36, 37, 38, 39, 39a, 40, 41, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68); im Raster tragen **2** der 80 Adressobjekte `name=` und **0** `operator=`. **Die Hausnummer 67 existiert in OSM auf der Hasselrott NICHT** - im abgetasteten Raster gibt es kein Objekt mit `addr:street=Hasselrott` und der Hausnummer 67 (auch nicht im weiteren Umfeld); die auf der Strasse erfassten Nummern springen von **66** zu **68**. Das ist **zu schreiben, nicht zu uebergehen**. Ein Objekt mit einem `name`- oder `operator`-Bezug zum Betrieb ist im abgetasteten Raster **nicht** vorhanden; das ist eine Aussage ueber den OSM-Bestand und **kein** Negativbeleg ueber den Betrieb. Die beiden `name=`-Adressobjekte sind **Fremdbetriebe**: node/279472908 (Hausnummer **11**, Tankstelle, `name=team`, `tenant=Thomas u. Stefan Meurer`, `website=https://www.antonwiller.de/index.php/gettorf.html`) und way/370348928 (Hausnummer **35e**, `name=Automobile Lingen`, `brand=Mercedes-Benz`, `shop=car`). Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9630515,54.4035858,9.9763288,54.4082695 ; https://api.openstreetmap.org/api/0.6/node/279472908.json ; https://api.openstreetmap.org/api/0.6/way/370348928.json (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Betreiber-Website live: Zertifizierung, Impressumsdaten und ein Widerspruch in der E-Mail-Adresse geklaert.** Die Seite „Über uns“ (https://autoverwertung-jess-gettorf.de/Ueber-uns/, HTTP 200 am 31.10.2026) belegt: „Wir sind ein **anerkannter Demontagebetrieb nach Altfahrzeug-Verordnung**. Die Zertifizierung erfolgt **jährlich durch den Tüv Nord**. **Seit 1968** sind wir im Bereich Autoverwertung und Schrotthandel in Gettorf tätig.“ Das Impressum (https://autoverwertung-jess-gettorf.de/Impressum/, HTTP 200) nennt **Autoverwertung Gettorf- Hasselrott 67- 24214 Gettorf**, **Robert Jess**, Telefon **04346/ 5355**, Fax **04346/ 298099**, E-Mail **robert-jess@gmx.de**, USt-Identifikationsnummer **DE 203288353**, inhaltlich verantwortlich Robert Jess - Adresse, Telefon und USt-ID sind damit **betreiberbelegt**. Die Startseite (https://autoverwertung-jess-gettorf.de/, HTTP 200) bestaetigt Telefon **04346 / 5355** und Oeffnungszeiten **Montag- Freitag 09:00- 18:00 Uhr, Samstag 09:00- 12:00 Uhr** (deckungsgleich mit dem Dossier). **Der E-Mail-Widerspruch aus der Vorwelle ist aufgeloest:** die **Fusszeile der Startseite** verlinkt `mailto:robert-jess@versanet.de`, waehrend **Impressum und Kontaktseite** durchgaengig **robert-jess@gmx.de** nennen - das Impressum ist die **Impressums-Angabe** im Sinne des § 5 DDG und damit die belastbare Quelle. `email` wird deshalb auf **robert-jess@gmx.de** gesetzt; die Startseiten-Fusszeile mit versanet.de wird als **abweichende Angabe** dokumentiert. Quelle(n): https://autoverwertung-jess-gettorf.de/Ueber-uns/ ; https://autoverwertung-jess-gettorf.de/Impressum/ ; https://autoverwertung-jess-gettorf.de/Kontakt/ ; https://autoverwertung-jess-gettorf.de/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Preise und Gebuehren getrennt.** ANKAUF: **keine oeffentliche Ankaufspreisliste** fuer Schrott oder Metall; die Leistungsseite nennt „Die umweltgerechte Demontage und Verwertung von Altfahrzeugen ist unsere Aufgabe“ und „Sie können bei uns bereits demontierte Gebrauchtteile erwerben und Fahrzeuge zum Selbstausbau stehen auch zur Verfügung“ - **ohne** Preise. VERKAUF: ein **Verkauf von Gebrauchtteilen** ist belegt (eigener eBay-Shop „Autoverwertung Jess“; auf der Leistungsseite eine Beispielanzeige „VW Scirocco 3 EZ. 2009 / Lederausstattung / Berry White / Collectors Edition / **750€**“ und auf der Startseite ein Fahrzeug „Porsche 911T EZ. 07.1970 / Ohne Motor / Viel Rost / Zu Verkaufen / **13500 Euro**“) - das sind **Einzel-Exposé-Preise**, **keine** Preisliste; die Startseite weist ausdruecklich darauf hin, dass das Angebot wechselt. GEBUEHREN: **keine Tarife** fuer Annahme, Demontage oder Lagerung; ein Fahrzeug kann laut Leistungsseite „zum Selbstausbau“ erworben werden, ohne dass Gebuehren genannt werden. Quelle(n): https://autoverwertung-jess-gettorf.de/Leistungen/ ; https://autoverwertung-jess-gettorf.de/ (Abruf 31.10.2026).]
- [Recherche 31.10.2026: **Offene Frage: die Adresslage Hasselrott 67 ist in OSM nicht aufloesbar.** Die Betreiber-Adresse **Hasselrott 67** ist in OSM **nicht** erfasst (kein Objekt mit der Hausnummer 67 auf der Hasselrott oder in deren Umfeld; die Strasse ist in OSM bis 68 durchnummeriert). Ob der Betrieb an der Nummer 67 oder unter einer Nachbarnummer gefuehrt wird, ist **offen**; der Seed-Punkt (54.4/9.9833) ist in jedem Fall **neu zu geokodieren**. Zu geokodieren ist: **Hasselrott 67, 24214 Gettorf**. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=9.9630515,54.4035858,9.9763288,54.4082695 ; https://autoverwertung-jess-gettorf.de/Impressum/ ; read-only `public.db` (31.10.2026).]
