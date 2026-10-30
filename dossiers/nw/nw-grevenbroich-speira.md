---
slug: nw-grevenbroich-speira
name: Speira GmbH
trader_type: sonstige
state: NW
city: Grevenbroich
street: Aluminiumstraße 1
postcode: 41515
phone: +49 2181 66 01
email: ''
opening_hours: ''
website: https://www.speira.com/
website_status: aktiv
status: aktiv
description: 'Speira GmbH, Aluminiumstraße 1, 41515 Grevenbroich (HRB 14011 AG Mönchengladbach) — europaeischer Aluminium-Walz- und Recyclingkonzern mit elf Standorten in Deutschland und Norwegen, bis 970.000 t Recycling pro Jahr; die Recycling-Sparte (Speira Recycling Services Germany GmbH) verarbeitet Alu-Schaett, Krusten und Sekundaerschrott zu Guss- und Knetlegierungen. Reines B2B-Geschaeft ohne Privatanlieferung, Angebote nur auf Anfrage, keine Preisliste.'
dropoff_json: '{}'
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 3b (Spezialisten)
provenance_ankauf_raw: ja (B2B)
provenance_origin: table
---

# Speira GmbH

## Überblick

Die Speira GmbH (Aluminiumstraße 1, 41515 Grevenbroich, HRB 14011 AG Mönchengladbach, USt-IdNr DE 812932051) ist ein europäischer **Aluminium-Walz- und Recyclingkonzern** mit elf Standorten in Deutschland und Norwegen (u. a. der Alunorf-JV, laut Unternehmensangabe „the world’s largest aluminium rolling mill“); Grevenbroich ist Hauptsitz und „the world’s largest rolled aluminium finishing mill“. Die Recycling-Sparte trägt die **eigene Rechtsträgerin Speira Recycling Services Germany GmbH** und verarbeitet „a wide range of scrap and dross“ — Anteil aus Produktionsschrott, wachsender Anteil aus Post-Consumer- und End-of-Life-Schrott — zu Guss- und Knetlegierungen (Schmelzverfahren je nach Ausgangsmaterial: Ein- und Doppelkammeröfen, Kipp-Drehrohröfen). Die **Ankaufsbedingungen** gelten ausdrücklich nur gegenüber Unternehmern („only applicable vis-à-vis undertakings in the meaning of § 14 para. 1 BGB“); **Preise** werden ohne Verpackung und ohne Umsatzsteuer ausgewiesen und dürfen bei geänderten „price-relevant factors, in particular the endowment of aluminium scrap“ für Lieferungen angepasst werden, die mehr als **vier Monate** nach Vertragsschluss fällig sind. Eine **Preisliste** gibt es nicht — Angebote laufen über „Contact us“. Eine Anlieferung Privater am Standort Grevenbroich ist nicht belegt; `dropoff_json` bleibt `{}` (unbekannt).

## Timeline

### Recherche 03.10.2026 (Feedback 4518)

- [Recherche 03.10.2026: Feedback 4518 teilweise berechtigt. Speiras Betreiberseiten belegen Recyclingaktivitäten und eine Einkaufsabteilung für Angebote, aber keine Annahme bzw. Selbstanlieferung am Werk Grevenbroich. Der bisherige DB-Wert `{"allowed":true}` und der B2B-Kontext belegen keinen Drop-off. Frontmatter `dropoff_json` explizit auf `{}` gesetzt (= unbekannt; weder Annahme noch Ablehnung behauptet); Kundenarten nicht geraten. Quelle(n): https://www.speira.com/, https://www.speira.com/contact/, https://www.speira.com/imprint/ (Abruf 03.10.2026)]

### Recherche 01.10.2026 (Feedback 2526)

- [Recherche 01.10.2026: Feedback 2526 teils berechtigt/teils verworfen: Straße/PLZ/Telefon bereits verifiziert (Aluminiumstr. 1, 41515 Grevenbroich, +49 2181 66-01) — keine Frontmatter-Änderung (kein Overwrite pro Direktive). E-Mail info.gv@speira.com nur Impressum-Einzelbeleg, Hersteller-B2B ohne Privatanlieferungs-Beleg (DB-dropoff allowed=true unbelegt) → nicht gefüllt (kein Phantom-Fill). Speira ist Alu-Walzwerk/Recycler, kein klassischer Händler. Namensvetter-Check: Speira Grevenbroich eindeutig (HRB 14011 AG Mönchengladbach).; Quelle(n): https://www.speira.com/imprint/ (Abruf 01.10.2026)]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Vollbeleg via Betreiber-Impressum (Owner-Primaerquelle, Name+HRB+Ort): Speira GmbH, Aluminiumstr. 1, 41515 Grevenbroich, HRB 14011 AG Moenchengladbach, Tel. +49 2181 66 01 — Frontmatter gefuellt, website_status aktiv. Hinweis: Alu-Walzproduzent + Recycling-Services (B2B), kein klassischer Privatkunden-Schrottankauf; E-Mail + Oeffnungszeiten nicht ausgewiesen (Anfrageformular) → leer.; Quelle(n): https://www.speira.com/imprint/ (Abruf 01.10.2026)]

### Importiert (Seed-Stand 2026-09-30)

- Alu-Walzprodukte + Recycling (bis ~970 kt/a), eigene Recycling-Services-Sparte, Alunorf-JV

### Recherche 30.10.2026

- [Recherche 30.10.2026: **Rechtsträger der Recycling-Sparte geklärt und Ankaufsbedingungen ausgewertet — B2B-only, Preisvorbehalt, keine Zahlenpreise.** Die auf der Download-Seite verlinkte Lieferbedingung trägt den Titel „**General Delivery and Payment Conditions — Speira Recycling Services Germany GmbH**“; die Recycling-Sparte wird also von einer **eigenen Gesellschaft** geführt, während das Impressum die **Speira GmbH** (HRB 14011) nennt — eine Gleichsetzung der beiden Rechtsträger wird hier **nicht** behauptet. Die Bedingungen (PDF, 4 Seiten) regeln: Ziff. 1 „These GDPC are only applicable vis-à-vis **undertakings in the meaning of § 14 para. 1 BGB** (German Commercial Code)“ — der Ankauf richtet sich damit **ausschließlich an Unternehmen**; Ziff. 2 „Our prices are quoted **excluding packaging and sales tax**. Should the price-relevant factors change after the conclusion of the contract, **in particular the endowment of aluminium scrap** we are entitled to amend the prices for such deliveries, that are due later than **four months** after the conclusion of the contract.“; Ziff. 3 „The risk passes to the customer when the goods leave the delivery plant of Speira Recycling Services“; Ziff. 4 Gewährleistung für Gewichts- und Stückzahlmängel mit Rügefrist von zwei Wochen und Verjährung der Ansprüche nach einem Jahr. **Ankaufpreise:** keine Zahlenwerte und keine Preisliste veröffentlicht — „Our prices are quoted …“ beschreibt nur die Preisbasis; **Gebühren:** keine Tarife; **Verkauf:** Legierungen werden „solid in ingots or liquid in crucibles“ geliefert, Verkaufskonditionen nur über „Contact us“/Anfrage. Quelle(n): https://www.speira.com/media/ve4hkpa5/general-delivery-and-payment-conditions-speira-recycling-services.pdf ; https://www.speira.com/markets-products/recycling-services/ (Abruf 30.10.2026)]

- [Recherche 30.10.2026: **Recycling-Leistungsbeschreibung präzisiert, kein Privatkundenangebot.** Die Seite „Recycling services: future-proof & sustainable“ beschreibt: „We process a wide range of scrap and dross at our sites. The majority of our raw materials comes from production processes, but the share of post-consumer and end-of-life scrap is growing steadily in the context of sustainability“, dazu je nach Ausgangsmaterial eingesetzte Schmelzverfahren („single-chamber furnaces are mainly used for melting and refining pure scrap. Double-chamber furnaces can also process scrap contaminated with paint or varnishes. Tilting rotary furnaces, on the other hand, are particularly suitable for heavily contaminated scrap and dross“) sowie „Production is **ISO and ASI certified**“. Der Kontaktbereich lautet „Do you want to explore how to sustain the flow of materials? Our aluminium recycling experts can provide technical guidance and innovative solutions. Get in touch with us to find out more about the potential and sustainability benefits of aluminium recycling.“ → **kein Privatkunden-Ankauf, keine Annahme am Werk Grevenbroich, keine Preisliste**; die Zertifizierung ist eine Betreiberangabe („Production is ISO and ASI certified“) ohne abgerufenes Zertifikat, ein `certifications`-Eintrag wird deshalb **nicht** erzeugt. Das Impressum (Stand October 01, 2025) nennt zusätzlich die **USt-IdNr DE 812932051** und die Organe (Management: Reinhild Schmidt (Speaker), Volker Backs; Aufsichtsratsvorsitzender: Einar Glomnes); die bereits durch die Vorwelle belegte Hersteller-Mail bleibt als reine Impressum-Angabe ohne Privatanlieferungs-Beleg **nicht** gefüllt. Quelle(n): https://www.speira.com/markets-products/recycling-services/ ; https://www.speira.com/imprint/ (Abruf 30.10.2026)]

- [Recherche 30.10.2026: **OSM-Abgleich der Betriebsadresse — der Betrieb ist als Objekt erfasst.** Im Rasterfenster **6.6120855/51.0731584 bis 6.6244104/51.0819234** liegen auf der **Aluminiumstraße** **8 Adressobjekte / 4 verschiedene Hausnummern**. Die **Hausnummer 1 existiert** als **way/228665831** mit `addr:housenumber=1`, `addr:street=Aluminiumstraße`, `addr:city=Grevenbroich`, `addr:postcode=41515`, `addr:country=DE`, `name=Speira GmbH`, `official_name=Speira Headquarters`, `old_name=Hydro Aluminium Rolled Products`, `landuse=industrial`, `man_made=works`, `email=info.gv@speira.com`, `website=https://www.speira.com`; ein zweites Objekt **way/267561306** trägt dieselbe Hausnummer mit `building=yes`. Der frühere Firmenname „Hydro Aluminium Rolled Products“ ist im `old_name`-Tag erhalten — die Namenshistorie der Speira GmbH ist damit auch in OSM nachvollziehbar. **Ankauf- oder Annahmebedingungen** sind in den Objekten nicht hinterlegt; ein Objekt mit einem Ankaufs- oder Annahmehinweis ist im Raster **nicht** vorhanden — das ist eine Aussage zum OSM-Bestand, kein Negativbeleg über den Betrieb. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, Abruf 30.10.2026); https://api.openstreetmap.org/api/0.6/way/228665831.json]
- [Recherche 30.10.2026: **Geokodierungsbedarf:** die in der Produktion gespeicherte Position (51.100966666666665, 6.6048333333333344) löst per Nominatim-Reverse (Adressstring des Reverse-Geocoders, **kein** OSM-Tag) als „K 10, Wevelinghoven, Grevenbroich, Rhein-Kreis Neuss, Nordrhein-Westfalen, 41516, Deutschland“ auf und liegt damit **nicht** an der Betriebsadresse Aluminiumstraße 1, 41515 Grevenbroich; die genannte Adresse selbst ist durch das oben dokumentierte OSM-Raster belegt. Eine belegte Geokodierung steht aus — es wurde **keine** Koordinate selbst gesetzt und **kein** Ortsmittelpunkt-Ersatz vorgenommen. Quelle(n): https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.100966666666665&lon=6.6048333333333344&zoom=18&addressdetails=1 (Abruf 30.10.2026); read-only `/var/lib/schrott-mcp/public.db` (Abruf 30.10.2026)]
