---
slug: sl-merzig-66663-auto-bohr
name: Auto Bohr GmbH
trader_type: autoverwertung
state: SL
city: Merzig
street: 'In der Pfingstweide 18'
postcode: '66663'
phone: '06861 4999'
email: 'info@auto-bohr.de'
opening_hours: 'Mo-Fr 8-12 und 13-17 Uhr, Sa 10-12 Uhr'
website: https://auto-bohr.de
website_status: aktiv
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4 (27.09.2026) — Kandidatenaudit R1+R2 (21)
  + Seed-Website-Sweep
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Auto Bohr GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche Adresse: Creditreform-Firmeneintrag plus Betreiber-Social (beide: In der Pfingstweide 18, 66663 Merzig); Quelle(n): firmeneintrag.creditreform.de, facebook.com/AutoBohrMerzig]
- Bewiesene Korrektur: Strasse lautet In der Pfingstweide 18 (2 Belege), nicht 18a wie im Seed; PLZ 66663.
- Telefon (06861 4999) und E-Mail (info@auto-bohr.de) nur je Einzelbeleg (Facebook bzw. Such-Snippet), daher Felder leer.
- Betreiber-Website http://www.auto-bohr.de liefert live HTTP 500 (01.10.2026), daher website_status unbekannt; Kontaktseiten-Inhalte nur via Such-Cache einsehbar (Einzelbeleg, unsicher): Oeffnungszeiten Mo-Fr 8-12/13-17 u.a.

### Importiert (Seed-Stand 2026-09-30)

- Autoverwertung/-verschrottung, Abschlepp, Ersatzteile (Creditreform + eBay-Shop „seit 1959, Verwertung"; Website derzeit WP-Fehler/HTTP 500, Domain lebt)
- Adresse: Merzig 66663, In der Pfingstweide 18a

### Recherche 31.10.2026

- [Recherche 31.10.2026: **Die Betreiber-Website ist wieder erreichbar** — der HTTP-500-Befund der Vorwelle vom 01.10.2026 ist **nicht mehr gültig**: `http://www.auto-bohr.de` liefert am 31.10.2026 HTTP 301 auf die kanonische Domain `https://auto-bohr.de` (dort HTTP 200, Impressum-/Kontakt-/Leistungsseiten erreichbar); die Inhalte werden jetzt als **„Auto Bohr GmbH"** bzw. „Wir für Euch seit **1959**" geführt. Das Feld `website` wird deshalb von `http://www.auto-bohr.de` auf **`https://auto-bohr.de`** geändert und `website_status` von `unbekannt` auf **`aktiv`** gesetzt. **Neu belegte Kontaktdaten aus der eigenen Kontaktseite** (https://auto-bohr.de/contact/): „Tel: **06861/4999**", „Email: **info@auto-bohr.de**", „Adresse: **In der Pfingstweide 18, 66663 Merzig**"; Öffnungszeiten **Montag bis Freitag 8–12 Uhr und 13–17 Uhr**, **Samstag 10–12 Uhr**, Sonntag geschlossen. Damit sind Telefon und E-Mail jetzt **zweifach** belegt (Kontaktseite **und** Footer der Startseite, die denselben Wert nennt) und können in die Frontmatter übernommen werden. Die Korrektur „18a" → „18" der Vorwelle wird durch die Betreiberseite bestätigt. Quelle(n): https://auto-bohr.de/contact/ ; https://auto-bohr.de/ (Abruf 31.10.2026)]
- [Recherche 31.10.2026: OSM-Adressprüfung live am Objekt, Raster bbox=6.6212997,49.4405590,6.6432747,49.4633492 (OSM-Way-BBox des gesamten Straßenverlaufs "In der Pfingstweide", way/35883665, plus 0,010 Grad Rand; in Kacheln abgetastet), Filter addr:street=In der Pfingstweide mit addr:housenumber: **13 Objekte** | **13 verschiedene Hausnummern** (roh 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16, 18 — keine Bereichsnotation, daher unverändert aufgelöst). Die Dossier-Hausnummer **18 ist erfasst**: way/346528926 mit `addr:housenumber=18`, `addr:street=In der Pfingstweide`, `addr:city=Merzig`, `addr:postcode=66663`, `addr:country=DE`, `building=yes` — das Objekt trägt **kein** `name=` und **kein** `operator=`. Von den 13 Adressobjekten der Straße trägt **0** ein `name=`- oder `operator=`-Tag. Im gesamten Raster tragen 878 Objekte `name=` oder `operator=`; Objekte mit Bezug zu **Schrott/Metall/Recycling/Edelmetall** sind dort **1** (Kriterium: `shop=gold_buyer` | `amenity=recycling` | `industrial=` | `landuse=industrial` | `recycling:` oder ein Name/Operator mit Schrott-/Recycling-Bezug, zusätzlich ein Name mit Bohr-Bezug): way/242465556 mit `amenity=recycling`, `name=Wertstoffzentrum Merzig`, `recycling_type=centre`, `landuse=commercial`, `note=nur für Merziger Bürger` — ein **Wertstoffzentrum der Stadt Merzig**, nicht der Dossier-Betrieb. **Kein Objekt im Raster trägt `name=Auto Bohr` oder `operator=Auto Bohr GmbH`:** der Betrieb ist in OSM an der Dossier-Adresse **nicht** kartiert. Quelle(n): https://api.openstreetmap.org/api/0.6/way/346528926.json ; https://api.openstreetmap.org/api/0.6/map.json?bbox=6.6212997,49.4405590,6.6432747,49.4633492 (Abruf 31.10.2026)]
- [Recherche 31.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. **Ankauf:** Die eigene Leistungsseite „Verwertung" (https://auto-bohr.de/services/item/verwertung/) ist jetzt abrufbar und belegt die Tätigkeit: „Als **zertifizierte Autoverwertung** kümmern wir uns zuverlässig um die fachgerechte Entsorgung und Verwertung von Altfahrzeugen umweltschonend, transparent und gesetzeskonform. Verwertbare Teile werden geprüft und wieder in den Kreislauf gebracht, der Rest wird umweltgerecht recycelt." — **ein** Euro-Betrag je Fahrzeug wird **nicht** genannt; es gibt eine „unverbindliches Angebot" per Kontaktformular, keinen öffentlichen Kurs. Auch die Startseite nennt nur „faire Preise" ohne Zahl. **Verkauf:** Der Betrieb verkauft über den verlinkten **eBay-Shop** (`ebay.de/str/autobohr`) gebrauchte Ersatzteile („geprüfte Neu und Gebrauchtteile"); die Bewertungen der Startseite nennen „Sehr seriöses Unternehmen" und „Ankauf eines Touareg 7P Unfallwagens … als Ersatzteilspender", „Austausch Motor via Spedition" — Belege für einen **Teileverkauf** mit Lieferung, aber **keine** öffentliche Preisliste. Die Startseite wirbt zudem mit „**Autokran Vermietung** im Raum Saar-Lor-Lux 30 bis 90 Tonnen" (Dienstleistung mit Miete, ohne Tarif). **Gebühren:** Keine Gebühren- oder Pauschalangabe auffindbar; ob die Verwertung für Privatkunden kostenlos ist, wird **nicht** gesagt (anders als bei vergleichbaren Betrieben, die die Kostenfreiheit ausdrücklich nennen). Offen bleibt außerdem, ob „zertifizierte Autoverwertung" eine Altautoverwertung nach Anlage 1 Nummer 2.2. AVO (also AVO-gelistet) meint — ein Nachweis ist auf der Website **nicht** verlinkt. Quelle(n): https://auto-bohr.de/services/item/verwertung/ ; https://auto-bohr.de/services/ ; https://auto-bohr.de/ (Abruf 31.10.2026)]
