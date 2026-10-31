---
slug: sl-merzig-harlingen-66663-abschleppdienst-autoverwertung-1000-serv
name: Abschleppdienst-Autoverwertung 1000 Service GmbH
trader_type: sonstige
state: SL
city: Merzig
street: Im Wäldchen 13-17
postcode: '66663'
phone: 06861-93 97 040
email: ''
opening_hours: ''
website: http://www.1000-service.de
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026) — Kandidatenaudit R1+R2 (21)
  + Seed-Website-Sweep
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Abschleppdienst-Autoverwertung 1000 Service GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026 (Feedback-Triage ID 3005)

- [Recherche 01.10.2026 (Feedback-Triage ID 3005): Feedback berechtigt, eingearbeitet — city 'Merzig-Harlingen 66663' → 'Merzig' (Format, Website nennt Merzig), trader_type autoverwertung → sonstige (Negativbefund: gesamte Betreiber-Site geprüft — nur Autovermietung, Baumaschinen-Mietpark, 24h Abschleppdienst, SB-Waschpark; keine Autoverwertung/Schrottannahme/Altautoankauf belegt; Verwertungs-Seiten 404). Adresse Im Wäldchen 13-17, 66663 Merzig + Abschlepp-Tel. 06861-9397040 rückbestätigt. Öffnungszeiten nur Timeline: Abschleppdienst Mo-Fr 08:00-12:00 + Termine n.V. (Kontaktseite, Einzelbeleg) bzw. Vermietung Mo-Fr 08-17/Sa 08-12 (andere Entity Saarbrücker Allee 80) → kein Frontmatter-Fill, status pruefung bleibt. Quelle(n): 1000-service.de/ + /kontakt/ + /impressum/ (Betreiber-Primärquelle, abgerufen 01.10.2026).]

### Importiert (Seed-Stand 2026-09-30)

- Autoverwertung (Firmenname + GS-Kategorie + schrottplatz-info; Website zeigt nur Mietpark/Abschlepp, Verwertungs-Seiten 404 → realer Auto-Intake plausibel, unbelegt)
- Adresse: Merzig-Harlingen 66663, Im Wäldchen 13-17
- Adressbeleg: http://www.1000-service.de

### Recherche 31.10.2026

- [Recherche 31.10.2026: **Zwei Rechtsträger, nicht einer — das Impressum trennt die Gesellschaften.** Das Impressum (http://www.1000-service.de/impressum/) nennt unter der Überschrift „Abschleppdienst & SB-Waschpark an der B51" die **1000 Service GmbH**, GF **Berthold Endres**, Im Wäldchen 13-17, 66663 Merzig, Tel. 06861-93 97 00, **Handelsregister Saarbrücken HRB 63537**, USt-IdNr. DE137970428 — das ist die Entity der Dossier-Adresse. Davon getrennt führt das Impressum unter „Auto- und Baumaschinenvermietung" die **1000 Service Vermietung GmbH**, GF **Daniel Hussinger**, Saarbrücker Allee 80, 66663 Merzig, Tel. 06861-1000, USt-IdNr. DE357740462 (dort ist zusätzlich „Handelsregister Saarbrücken HRB 109035" genannt). Die Dossier-Adresse gehört also zur **1000 Service GmbH** (HRB 63537), **nicht** zur Vermietungsgesellschaft; die im Dossier genannte **Abschlepp-Telefonnummer 06861-93 97 040** ist wiederum eine dritte Nummer (die Impressumszeile nennt 06861-93 97 **00**, die Kontaktseite und die Seite „24h-Abschleppdienst" nennen 06861-93 97 **040** — die Endung 040 ist die **24h-Bereitschaft**). Die Seed-Adresse „Merzig-Harlingen" ist damit zu **„Merzig"** zu normalisieren (die Impressumsanschrift lautet „66663 Merzig"), die **PLZ 66663** bleibt. Quelle(n): http://www.1000-service.de/impressum/ ; http://www.1000-service.de/24h-abschleppdienst/ ; http://www.1000-service.de/kontakt/ (Abruf 31.10.2026)]
- [Recherche 31.10.2026: OSM-Adressprüfung live am Objekt, Raster bbox=6.6262228,49.4126909,6.6467732,49.4339401 (OSM-Way-BBox des gesamten Straßenverlaufs "Im Wäldchen", way/228980103, plus 0,010 Grad Rand; in Kacheln abgetastet), Filter addr:street=Im Wäldchen mit addr:housenumber: **11 Objekte** | **11 verschiedene Hausnummern** (roh 1, 2, 6, 8, 9, 11, 12, 13, 14, 15, 16 — keine Bereichsnotation, daher unverändert aufgelöst). Die Dossier-Hausnummer **13 ist erfasst**: way/386171404 mit `addr:housenumber=13`, `addr:street=Im Wäldchen`, `addr:city=Merzig`, `addr:postcode=66663`, `addr:country=DE`, `building=yes` — das Objekt trägt **kein** `name=` und **kein** `operator=`. **Die Bereichsangabe „13-17" der Dossier-Adresse ist in OSM nicht als eine Adresse erfasst**; auf der Straße existiert die Nummer 13, die Nummern 14, 15 und 16 sind als eigene Objekte erfasst, eine Nummer 17 kommt **nicht** vor (die Nummernfolge endet bei 16). Ob die Firma den Bereich 13-17 oder nur 13 nutzt, ist aus OSM **nicht** entscheidbar — die Dossier-Adresse bleibt als Bereichsangabe stehen. Von den 11 Adressobjekten der Straße tragen 2 ein `name=`/`operator=`-Tag, **beide ohne** Bezug zur Dossier-Firma: way/153159908 mit `addr:housenumber=1`, `addr:street=Im Wäldchen`, `name=Reimsbach`, `shop=car_repair` und way/386171193 mit `addr:housenumber=9`, `addr:street=Im Wäldchen`, `name=Nema`, `craft=hvac`. Im gesamten Raster tragen 221 Objekte `name=` oder `operator=`; Objekte mit Bezug zu **Schrott/Metall/Recycling/Edelmetall** sind dort **0** (Kriterium: `shop=gold_buyer` | `amenity=recycling` | `industrial=` | `landuse=industrial` | `recycling:` oder ein Name/Operator mit Schrott-/Recycling-Bezug, zusätzlich ein Name mit 1000-Service-Bezug). **Positiver Betriebsbeleg, aber ohne Adressbezug:** way/386171280 trägt `name=1000 Service` und `shop=car_parts` (Position 6.6371144/49.423716) — das Objekt nennt die **Firma** und liegt **34 m** vom Adressobjekt way/386171404 („Im Wäldchen 13", 6.6369679/49.4240049) entfernt (Haversine, R=6371 km), trägt aber selbst **kein** `addr:housenumber` und **kein** `addr:street`. Der Betrieb ist damit in OSM an der Dossier-Adresse **kartiert** (Firmenname am Standort), die Adresszuordnung „Im Wäldchen 13-17" bleibt davon unberührt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/386171404.json ; https://api.openstreetmap.org/api/0.6/way/386171280.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=6.6262228,49.4126909,6.6467732,49.4339401 (Abruf 31.10.2026)]
- [Recherche 31.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. **Ankauf:** Für Altautoankauf, Schrottannahme oder Fahrzeugverwertung ist **keine** Ankaufvergütung und **kein** Euro-Betrag veröffentlicht; die Negativaussage der Vorwelle („nur Autovermietung, Baumaschinen-Mietpark, 24h Abschleppdienst, SB-Waschpark") ist am 31.10.2026 **unverändert gültig** — die Seiten „Autovermietung", „Baumaschinen-Mietpark", „24h-Abschleppdienst", „Waschpark", „Transportkosten", „Kautionen" und „AGB" enthalten keine Verwertungsleistung, und eine eigene Verwertungsseite existiert weiterhin nicht. **Verkauf:** Der Betrieb **verkauft** im Kern Miete von Fahrzeugen und Baumaschinen — das ist keine Schrott-Verkaufssparte, die für dieses Dossier einschlägig wäre; erkauft werden (als Vermieter) keine Rohstoffe. **Gebühren:** Hier ist die **Preisliste** des Betreibers belegt: Die **Gewerbe-Mietpreisliste 2026** (PDF, auf der Startseite als „Preisliste herunterladen" verlinkt, live gelesen) enthält bezifferte Tarife, ausgewiesen mit „alle Preise zzgl. MwSt." — für den **Abschlepp-/Bergungsdienst** (24 h, PKW und LKW bis 7,5 t) ist **kein** Tarif enthalten; die Preisliste deckt Autovermietung (PKW/Transporter/9-Sitzer/LKW bis 7,5 t/Kipper), Baumaschinen-Mietpark (Minibagger 0,8–3,5 t ab **109,00 €/Tag** … Radlader **159,00–189,00 €/Tag**, Verdichtungstechnik, Rüttelplatten **39,00–69,00 €/Tag** usw.), Hub-/Scheren-/LKW-Bühnen, Arbeitsbühnen, Werkzeuge, Baustrom, Reinigung und Gartengeräte ab. Beispiele: PKW-Gruppe Opel Corsa/Ford Fiesta ab **69,00 €/Tag** (inkl. 100 km), Opel Astra ab 79,00 €, Mini Cooper/BMW 1er ab 85,00 €, VW Passat Kombi ab 99,00 €; 9-Sitzer-Tarife Tag/Wochenende/Woche **139,00/299,00/599,00 €**; Verkehrspauschalen **Mehrkilometer 0,30 €/km**, Kilometerpakete 250 km **69,- €**, 500 km **129,- €**, 1000 km **259,- €**; Versicherungspakete **29,00 €/Tag** (Tagestarif), **79,00 €** (Wochenende), **159,00 €** (Woche), jeweils mit 1000 € SB; LKW-Kipper 7,5 t inkl. 100 km **159,00 €/Tag**, mehr km **0,45 €**; PKW-Vollkasko **15,00–25,00 €/Tag** je Gruppe. Die Seite „Transportkosten" nennt „Bei Transport mit LKW zzgl. **Mautkosten**" — eine Gebühr **ohne** Betrag. Für die Dossier-Leistung (Abschleppen/Bergung, Waschpark) ist damit **kein** Tarif öffentlich; die Vermietpreise sind als **Gebühren der Vermietung** zu werten und **nicht** als Ankauf- oder Verwertungspreis. Quelle(n): http://www.1000-service.de/ (Preisliste „Preisliste 1000 2026", PDF) ; http://www.1000-service.de/transportkosten/ ; http://www.1000-service.de/24h-abschleppdienst/ (Abruf 31.10.2026)]
