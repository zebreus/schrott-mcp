---
slug: sn-jahnsdorf-ot-pfaffenhain-schrotti
name: Schrotti
trader_type: schrotthaendler
state: SN
city: Jahnsdorf OT Pfaffenhain
street: 'Wilhermsdorfer Str. 11'
postcode: '09387'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Schrotti

## Überblick

Für „Schrotti" existiert ein 11880-Verzeichnislead an der Wilhermsdorfer Straße 11, 09387 Jahnsdorf (Tel. 0162 8770984). Öffnungszeiten weichen zwischen Verzeichnissen ab; kein Betreiberauftritt, Register- oder kommunaler Gewerbenachweis verbindet den Eintrag bisher mit einem aktuellen Schrottannahmepunkt in Pfaffenhain. Der amtliche Fachbetrieberegister führt für Jahnsdorf keinen Betrieb und im gesamten Bestand keinen Eintrag „Schrotti". Nicht mit Pellmann Olaf an Stollberger Straße 11 zusammenführen. Der Seed-Status `aktiv` ist nicht neu bestätigt und wurde deshalb auf `pruefung` gesetzt — das ist keine Schließungsaussage. **Ankauf:** keine händlerspezifischen Kurse; **Verkauf:** keine Liste; **Gebühren:** keine Angaben. Adresse bleibt GEO-offen.

## Timeline

### Recherche 27.10.2026: amtlicher Registerabgleich und Statuskorrektur

- [Recherche 27.10.2026: Amtlicher Fachbetrieberegister als neue Gegenquelle (GESA/eEFBV-Auszug, Stand 11.08.2026): Für Jahnsdorf ist keine Ortsseite vorhanden, im gesamten Register (4.507 zertifizierte Betriebe) existiert kein Eintrieb mit dem Namen „Schrotti" und keine zertifizierte Betriebsstätte in Jahnsdorf. Das Register enthält nur zertifizierte Metall- und Fahrzeugbetriebe; das Fehlen belegt weder Nichtexistenz noch Geschäftsaufgabe, es existiert aber kein EFB-/Demontage-Nachweis für den Lead; Quelle(n): https://schrottregister.pages.dev/llms.txt ; https://schrottregister.pages.dev/was-fehlt]
- [Recherche 27.10.2026: Statuskorrektur aktiv → pruefung: Der einzige Beleg bleibt der 11880-Verzeichniseintrag (Wilhermsdorfer Str. 11, 09387 Jahnsdorf, Tel. 0162 8770984, Datenstand 20.11.2025) mit widersprüchlichen Öffnungszeiten gegenüber Mapcarta; eine Betreiber-Website, ein Registereintrag, ein kommunaler Gewerbenachweis oder eine Ankaufspreisliste existieren nicht. Analog zur Behandlung der rein verzeichnisgestützten Kandidaten in diesem Shard (bw-balingen-schlaich-kurt-schrotthdl, hh-barmbek-sud-hamburg-schrott) ist `aktiv` nicht haltbar und wurde auf `pruefung` gesetzt; das ist ausdrücklich keine Schließungsbehauptung. Adresse, Telefon und Öffnungszeiten bleiben unverändert im Frontmatter, da es nur eine Belegklasse gibt; Quelle(n): https://www.11880.com/branchenbuch/jahnsdorf-erzgebirge/060440092B112753071/schrotti.html ; https://mapcarta.com/de/W207064499]
- [Recherche 27.10.2026: GEO-Abgleich: Die read-only Produktionskoordinate 50.75/12.85 löst nach OSM/Nominatim auf „Pfaffenhain, Leukersdorf, Jahnsdorf/Erzgebirge, Erzgebirgskreis, Sachsen, 09387" auf, also genau den im Dossier genannten Ortsteil Pfaffenhain; sie ist damit kein Fehlpunkt wie in anderen Dossiers dieser Welle. Sie bleibt jedoch ein Orts-/Ortsteilpunkt ohne Hausnummer, da die Adresse Wilhermsdorfer Straße 11 nur als Verzeichnislead vorliegt; eine Koordinate wurde deshalb nicht geändert; Quelle(n): https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.75&lon=12.85&zoom=14&addressdetails=1 ; read-only `public.db` (27.10.2026).]

### Recherche 13.10.2026

- [Recherche 13.10.2026: 11880 listet „Schrotti“, Wilhermsdorfer Str. 11, 09387 Jahnsdorf, Tel. 0162 8770984; die Seite nennt als Datenstand 20.11.2025 und zeigt Öffnungszeiten Do/Fr 12–17 sowie Sa 09–12. Mapcarta gibt andere Stunden (Mo–Fr 12–18, Sa 09–12). Diese Verzeichnisangaben klären nicht die aktuelle Annahme oder den genauen OT-Pfaffenhain-Bezug; daher keine Kontaktdaten/Stunden in Frontmatter ergänzt. Pellmann Olaf, Stollberger Str. 11, ist ein getrennter Listeneintrag und kein Ersatzbeleg. ANKAUF/VERKAUF/GEBÜHREN: keine belastbare Preisliste. Keine Koordinaten aus Verzeichnisdaten übernommen. Quellen: https://www.11880.com/branchenbuch/jahnsdorf-erzgebirge/060440092B112753071/schrotti.html ; https://mapcarta.com/de/W207064499 (beide Leads; Stundenabweichung).]

### Importiert (Seed-Stand 2026-09-30)

- Fe/Alu/Cu/Edel/Blei/Messing
- Annahme lt. lokaleschrottplatz

### Recherche 04.10.2026

- [Recherche 04.10.2026: Klärfall bleibt offen. Schrottplatz-Info listet für Jahnsdorf aktuell nur „Pellmann Olaf“ an der Stollberger Str. 11, 09387 Jahnsdorf; „Schrotti“ bzw. OT Pfaffenhain erscheint dort nicht. Die Ortsliste ist ein Aggregator und ihr fehlender Eintrag belegt weder Nichtexistenz noch Schließung; der andere Händler wird nicht mit diesem Dossier zusammengeführt. Die heutige Abfrage liefert keinen zusätzlichen Betreiber-, Register-, kommunalen Gewerbe- oder Betreiber-Social-Beleg, daher keine Frontmatter-Änderung. Status aktiv bleibt Seed-Stand, nicht neu bestätigt; Adresse, Kontakte, Öffnungszeiten sowie Annahme/Abholung bleiben ungeklärt; keine Geocodierung; Quelle(n): https://www.schrottplatz-info.de/schrottplatz/Jahnsdorf (Abruf 04.10.2026; Aggregator, kein Beweis).]

### Recherche 19.11.2026 (Welle 0600 B)

- [Recherche 19.11.2026: **Der Betrieb ist jetzt durch ein betriebstragendes OSM-Objekt am Seed-Namen belegt, der Ortsbezug Pfaffenhain steht dadurch fest.** Das Objekt `way/207064499` trägt `name`=SCHROTTI Schrott und Buntmetalle, `addr:street`=Wilhermsdorfer Straße, `addr:housenumber`=11, `addr:postcode`=09387, `addr:city`=Jahnsdorf/Erzgeb., `opening_hours`=Mo-Fr 12:00-18:00; Sa 09:00-12:00, `source`=survey und liegt bei 50.7524675/12.8538833; die Rückauflösung weist den Ortsteil `Pfaffenhain` aus, der damit nicht nur aus dem Seed-Ortsteil, sondern auch aus der Adresse folgt. Der Verzeichniseintrag 11880 nennt am 19.11.2026 unverändert `Schrotti`, Wilhermsdorfer Str. 11, 09387 Jahnsdorf, Telefon `(0162) 8770984`, den Rubrik-Wert `Altmetallrecycling` und die Öffnungszeiten `Do 12:00-17:00, Fr 12:00-17:00, Sa 09:00-12:00` bei Eintragsdaten vom 20.11.2025 und Aktualisierung am 10.11.2026; Cylex führt denselben Betrieb als `Schrotti, Wilhermsdorfer Str. 11, 09387 Jahnsdorf/Erzgebirge, Pfaffenhain, Sachsen`. Die Öffnungszeiten der beiden Quellen widersprechen sich in der Wochentagsabdeckung, Mo bis Fr nach OSM gegenüber Do und Fr nach 11880; `street` und `postcode` wurden deshalb aus der belegten Adresse übernommen, die Telefonnummer bleibt nur Verzeichnisbeleg und wurde nicht in das Frontmatter übernommen. Quellen: https://api.openstreetmap.org/api/0.6/way/207064499/full.json ; https://www.11880.com/branchenbuch/jahnsdorf-erzgebirge/060440092B112753071/schrotti.html ; https://web2.cylex.de/firma-home/schrotti-14237490.html .]
- [Recherche 19.11.2026: **Amtliche Gegenprobe an der exakten Anschrift negativ, der Betrieb ist kein zertifizierter Entsorgungs- oder Altfahrzeugbetrieb.** Die Standortabfrage mit 3 km um Wilhermsdorfer Str. 11, 09387 Jahnsdorf liefert in der Route `Entsorgungsfachbetriebe` 0 Treffer und in der Route `Altfahrzeugverwertung` 0 Treffer, die Namenssuche `Schrotti` liefert in beiden Routen 0 Treffer. Reichweite: die EfbV-Anerkennung nach § 56 KrWG und die AltfahrzeugV-Anerkennung nach § 2 Abs. 2 AltfahrzeugV sind freiwillig, die Nichttreffer sind kein Nichtexistenzbeweis; sie belegen nur, dass für diese Anschrift keine amtlich zertifizierte Betriebsstätte geführt wird. Am selben Hausnummer-Paket weist OSM zusätzlich `way/207056925` mit `name`=KD Handel Service GmbH, `way/197707566` mit `name`=INTEGRA Fachgroßhandel und `way/207064487` mit `website`=`metallwaren-online.de` aus, es handelt sich also um ein Haus mit mehreren Gewerbetreibenden. **Ankauf:** keine händlerspezifischen Kurse; **Verkauf:** keine Preisliste; **Gebühren:** keine Angaben. Der in der Produktion gespeicherte Punkt 50.75/12.85 löst weiterhin auf die Straße Wilhermsdorfer Straße im Ortsteil Pfaffenhain auf und ist damit kein Fehlpunkt, er wurde nicht geändert. Quellen: https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Wilhermsdorfer%20Str.%2011%2C%2009387%20Jahnsdorf%2C%20Deutschland%3B12.8538833%3B50.7524675%3B12.8964794%3B50.7794168%3B12.8112872%3B50.7255182&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrotti .]
