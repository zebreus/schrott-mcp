---
slug: nw-essen-mre-metall-recycling-essen
clear_fields: geo
name: MRE Metall Recycling Essen
trader_type: metallhaendler
state: NW
city: Essen
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Neu, Ankauf unklar (nur Verzeichnis-/Profilbeleg)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# MRE Metall Recycling Essen

## Überblick

Die live Betreiberseite beansprucht den Namen „MRE Metall Recycling Essen“ und nennt Bonifaciusring 5, 45309 Essen sowie Container von 5–40 m³ und An-/Verkauf von Schrott und NE-Metallen. Der Auftritt widerspricht sich intern: MRE/Familienbetrieb und Gründung 2011 stehen neben „Häde Metall Recycling Essen“ und fast 70 Jahren Unternehmensgeschichte; Startseite und Impressum nennen verschiedene Telefonnummern. Das Impressum enthält keine Handelsregisternummer. Daher bleibt die rechtliche Betreiberidentität offen; Bonifaciusring 5 ist nur ein Eigenangaben-Kandidat. Die frühere Frontmatter-`street`-Zeile vermischte Firmenname, Stadt und Straße und wurde geleert statt als Anschrift weiterzugeben.

**Preise:** Ankauf: aktuelle Tagespreise laut Betreiber nur telefonisch; keine numerische Liste. Verkauf: keine separate Verkaufspreisliste gefunden. Gebühren: Containergrößen 5-40 m³ sind publiziert, Container-/Transportgebühren nicht. **Geocoding:** Bonifaciusring 5 bleibt ein Betreiberseiten-Kandidat und ist erst nach Klärung der Identitätswidersprüche als Pin zu übernehmen.

## Timeline

### Owner-Korrektur 27.10.2026: falsches GEO zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `51.450284375, 7.015053125`. Der Reverse-Lookup löst diesen Punkt auf die Straße „Freiheit", 45128 Essen auf, genauer auf die Fernbus-Spur am Essener Hauptbahnhof (OSM way 46961889, `highway=service`, `vehicle=no`); das ist kein Gewerbestandort und keine Quelle nennt diese Straße als Betreiber- oder Annahmeort. Die aktuelle Betreiberseite belegt MRE Metall Recycling Essen (Geschäftsführer Zakaria El Osman) am Bonifaciusring 5, 45309 Essen mit Waagen und Anlieferung; das unabhängige Verlagsverzeichnis Das Örtliche nennt dieselbe Adresse und Telefonnummer, während „Häde“ zur anderen Firma Häde Recycling GmbH, Alte Bottroper Straße 120, 45356 Essen gehört. Bonifaciusring 5 liegt rund 5,8 km vom gespeicherten Punkt entfernt, ist aber nur ein Gebäudezentroid und kein belegter Kundenstandort. Deshalb wird ausschließlich das falsche GEO-Paar mit `clear_fields: geo` zurückgenommen: keine Ersatzkoordinate, keine Adressübernahme in die Frontmatter, keine Annahme- oder Schließungsbehauptung, `status: pruefung` bleibt. Offen bleiben Register- und Behördenbeleg (Einzelunternehmer ohne HRB), das Verhältnis MRE zu Häde und eine belegte Annahmestelle. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1 ; https://mre-essen.de.tl/ ; https://mre-essen.de.tl/Impressum.htm ; https://mre-essen.de.tl/Unsere-Leistungen.htm ; https://www.dasoertliche.de/Themen/Metall-Recycling-Essen-Essen-Kray-Bonifaciusring ; https://www.haede-recycling.de/impressum ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 26.10.2026

- [Recherche 26.10.2026: MRE-Startseite, Impressum und Preisliste live erneut direkt gelesen. Die Seite /Preisliste.htm existiert, enthält aber weiterhin ausschließlich „Ankaufspreise frei Lager Essen. Preisänderungen vorbehalten“ und keine Materialien oder Beträge; das ist ein Preislistenfund ohne veröffentlichte Zahlen. Startseite nennt 0201 47616418, Impressum 0176 61438885; interne Namens-/Historienwidersprüche (MRE vs. Häde; Gründung 2011 vs. fast 70 Jahre) bleiben unaufgelöst. Ankauf: Tagespreise telefonisch, kein Kursblatt; Verkauf: keine separate Preisliste; Gebühren: Container 5–40 m³, keine Kosten-/Transporttarife. Frontmatter unverändert; Bonifaciusring 5 bleibt ungeklärter Kandidat und GEO-gesperrt. Quellen: https://mre-essen.de.tl/ ; https://mre-essen.de.tl/Impressum.htm ; https://mre-essen.de.tl/Preisliste.htm ; https://mre-essen.de.tl/Unsere-Leistungen.htm.]

### Recherche 24.10.2026

- [Recherche 24.10.2026: Betreiber-Startseite, Impressum, Leistungs- und Preisliste direkt geprüft. Der Auftritt nennt MRE/„Häde Metall Recycling Essen“, Zakaria El Osman und Bonifaciusring 5, 45309 Essen, jedoch widersprüchliche Unternehmenshistorie (2011 gegenüber fast 70 Jahren) und Telefonnummern 0201 47616418 gegenüber 0176 61438885; kein HRB im Impressum. Die Leistungsseite bewirbt Ankauf von Eisen, Edelstahl, Kupfer, Messing, Kabel und Aluminium sowie Abbruch/Abholung/Container und Waagen, aber ohne numerische Ankaufspreise. Die „Preisliste“ trägt nur die Überschrift „Ankaufspreise frei Lager Essen“ und enthält keine Materialwerte. **Verkauf:** keine separate belastbare Preisliste. **Gebühren:** Containerangabe 5–40 m³ ist Volumen, kein Tarif; keine Pickup-/Entsorgungspreise. Identität/Adresse bleiben ungeklärt; die frühere fehlerhafte `street`-Zeile (Firmenname + „Essen“ + Bonifaciusring 5) wurde aus dem strukturierten Adressfeld entfernt; Betreiberadresse bleibt als nicht bestätigter Kandidat im Überblick. Status pruefung, kein GEO. Quellen: https://mre-essen.de.tl/ ; https://mre-essen.de.tl/Impressum.htm ; https://mre-essen.de.tl/Unsere-Leistungen.htm ; https://mre-essen.de.tl/Preisliste.htm.]

### Recherche 06.10.2026

- [Recherche 06.10.2026: Betreiberwebsite direkt geöffnet: Startseite nennt MRE Metall Recycling Essen, Bonifaciusring 5, 45309 Essen, 0201 47616418, zmre@outlook.de, Mo–Fr 07:00–20:00, Sa 07:30–20:00, Schrott-/NE-Metallankauf und -verkauf sowie Container 5–40 m³. Das Impressum nennt Zakaria El Osman als Geschäftsführer, dieselbe Büroadresse, aber Tel. 0176 61438885 und keine Handelsregisterkennung; die Website erklärt zugleich, es handle sich um „Häde Metall Recycling Essen“ mit fast 70-jähriger Geschichte, im Konflikt zur MRE-/Gründungsangabe 2011. Angaben daher als widersprüchliche Betreiber-Selbstaussagen dokumentiert, keine Kontaktdaten/Adresse ins Frontmatter übernommen. Preise getrennt: Ankauf nur aktuelle Tagespreise auf Anfrage per Telefon, kein Zahlenblatt; Verkaufspreisliste fehlt; Containergrößen publiziert, Tarife/Gebühren nicht. Geocoding erst nach Betreiber-/Adressklärung. Quelle(n): https://mre-essen.de.tl/ ; https://mre-essen.de.tl/Impressum.htm ; https://mre-essen.de.tl/-Oe-ffnungszeiten.htm]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Betreiberauftritt samt Impressum erneut direkt abgerufen; die Seiten sind erreichbar und wiederholen Bonifaciusring 5, 45309 Essen, MRE Metall Recycling Essen, Ankauf/Verkauf, 5-40-m3-Container und Öffnungszeiten. Der Marketingtext vermischt weiterhin MRE mit „Häde Metall Recycling Essen“ und fast 70 Jahren Geschichte, obwohl er MRE 2011 als gegründet bezeichnet; Impressum nennt keine HRB und zwei verschiedene Telefonnummern bleiben im Auftritt. Keine Identitätsauflösung durch neue unabhängige Primärquelle; daher keine Kontaktdaten oder Betreiberbeziehung in Frontmatter übernommen. Ankauf: Zahlenpreise nicht veröffentlicht, telefonisch anzufragen; Verkauf: keine separate Preisliste; Gebühren: keine Container-/Transporttarife. Bonifaciusring 5 nur ungeklärter Geocoding-Kandidat; Quelle: https://mre-essen.de.tl/ ; https://mre-essen.de.tl/Impressum.htm ; https://mre-essen.de.tl/-Oe-ffnungszeiten.htm]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Keine Betreiber-Website und kein Registerbeleg gefunden. Nur Aggregator-Leads (ENF/city-map: Bonifaciusring 5, 45309 Essen, Tel. 017661438885) — nach Quellenhierarchie keine Belege. Alle Kontaktfelder leer gelassen, Klärfall für Folgewelle (Betreiber-Impressum oder HR-Auszug nötig). Quelle(n): keine belegfähige Quelle; Leads: enfmetal.com, city-map.com.]

### Importiert (Seed-Stand 2026-09-30)

- Bonifaciusring 5, nur schrottradar-Profil
- urspr. Website-Angabe: keine gefunden
