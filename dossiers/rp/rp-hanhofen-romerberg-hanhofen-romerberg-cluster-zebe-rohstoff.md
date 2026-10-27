---
slug: rp-hanhofen-romerberg-hanhofen-romerberg-cluster-zebe-rohstoff
clear_fields: geo
name: Hanhofen/Römerberg-Cluster (ZEBE Rohstoffhandel; Wagner R.+B.; Kreischer J.;
  Lehr T.+M./I.+R.; Wagner P.+S.)
trader_type: metallhaendler
state: RP
city: Hanhofen / Römerberg
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
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# Hanhofen/Römerberg-Cluster (ZEBE Rohstoffhandel; Wagner R.+B.; Kreischer J.; Lehr T.+M./I.+R.; Wagner P.+S.)

## Überblick

Der Cluster bleibt ein Sammel-Klärfall und darf nicht als eine gemeinsame Firma behandelt werden. NorthData führt **ZEBE Rohstoffhandel GmbH Schrott-Metalle Gießerei- und Hüttenbedarf**, HRB 51621 AG Ludwigshafen, Martin-Greif-Str. 31, 67354 Römerberg; der angezeigte Gesellschaftszweck umfasst Handel mit Schrott, Metallen und Nichteisenmetallen sowie Gießerei-/Hüttenbedarf. Sichtbare Jahresabschlüsse reichen bis 2023 (veröffentlicht 24.01.2025); das ist kein Nachweis heutigen Betriebs oder einer öffentlichen Annahmestelle. ZEBE-Adresse und Rechtsträger bleiben nur dieser Cluster-Komponente zugeordnet, nicht in gemeinsame Frontmatter-Felder übertragen. Die übrigen Wagner-, Kreischer- und Lehr-Namen bleiben separate Verzeichnisleads.

**Ankauf:** Registerzweck nennt Handel, aber keine heutigen Annahmebedingungen oder Preise. **Verkauf:** keine Preisliste. **Gebühren:** keine Tarife. **GEO:** Martin-Greif-Str. 31 ist ein Registermirror-Adressbeleg, kein bestätigter öffentlicher Hof; nicht geokodieren, bis Betrieb/Annahmestelle bestätigt sind.

## Timeline

### Owner-Korrektur 27.10.2026: nicht zuordenbaren Clusterpunkt zurücknehmen

- [Owner-Korrektur 27.10.2026: Die read-only Produktionszeile enthält `49.3167, 8.3408`; der Reverse-Lookup liegt an Hauptstraße 9b in Hanhofen. Der einzige identifizierte Rechtsträger im Sammeldatensatz, ZEBE Rohstoffhandel GmbH, hat laut dem dokumentierten Registermirror-Lead die Anschrift Martin-Greif-Straße 31 in Römerberg; die übrigen Personen/Betriebe sind unverbundene Leads. Das bestehende Paar kann deshalb keinem Händler dieses Mehr-Orte-Clusters belastbar zugeordnet werden. `clear_fields: geo` nimmt nur diesen ungeprüften Sammelpunkt beim regulären Seed zurück. Kein Ersatzpunkt, keine Aussage, dass es in Hanhofen keinen Betrieb gibt, und kein Schließungsurteil. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.3167&lon=8.3408&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Martin-Greif-Strasse%2031%2C%2067354%20R%C3%B6merberg%2C%20Germany&format=jsonv2&limit=5&addressdetails=1 ; https://www.northdata.de/ZEBE+Rohstoffhandel+GmbH+Schrott-Metalle+Gie%C3%9Ferei-+und+H%C3%BCttenbedarf,+R%C3%B6merberg/Amtsgericht+Ludwigshafen+a.Rhein+HRB+51621 ; read-only `public.db` (27.10.2026); docs/trader-seed.md.]

### Recherche 27.10.2026

- [Recherche 27.10.2026: NorthData-Unternehmensseite zu ZEBE direkt gelesen: **ZEBE Rohstoffhandel GmbH Schrott-Metalle Gießerei- und Hüttenbedarf**, AG Ludwigshafen HRB 51621, Martin-Greif-Str. 31, 67354 Römerberg; Gegenstand nennt Handel mit Schrott, Metallen und Nichteisenmetallen sowie Gießerei-/Hüttenbedarf. Sichtbarster Jahresabschluss 2023 am 24.01.2025 veröffentlicht. Dies ist ein Registermirror-Identitäts-/Adressbefund, aber weder aktuelle Betriebsbestätigung noch öffentliche Annahme-/Anlieferbestätigung. Keine ZEBE-Felder auf den Sammelcluster übertragen; die übrigen Namen bleiben getrennte Leads. **ANKAUF:** Geschäftszweck belegt keinen tatsächlichen aktuellen Ankauf/Kurs. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife. Kein GEO. Quelle: https://www.northdata.de/ZEBE+Rohstoffhandel+GmbH+Schrott-Metalle+Gie%C3%9Ferei-+und+H%C3%BCttenbedarf,+R%C3%B6merberg/Amtsgericht+Ludwigshafen+a.Rhein+HRB+51621 .]

### Recherche 02.10.2026

- [Recherche 02.10.2026: Klärfall — einziger HR-Beleg im Cluster ist ZEBE Rohstoffhandel GmbH (Martin-Greif-Str. 31, 67354 Römerberg, HR AG Ludwigshafen, Verzeichnis-/Snippet-Hinweis, nicht unabhängig verifiziert); die übrigen Cluster-Kleinhändler (Wagner R.+B., Kreischer J., Lehr T.+M./I.+R., Wagner P.+S.) nur Gelbe-Seiten-Leads ohne Betreiber-Beleg → keine Frontmatter-Fills (Cluster-Adresse nicht mit ZEBE-Adresse überschreiben). Quelle(n): YellowMap-Eintrag + HR-Snippet ZEBE Rohstoffhandel GmbH + Gelbe Seiten]

### Importiert (Seed-Stand 2026-09-30)

- Mehrere Kleinhändler Schrott-/Metallhandel (nur Gelbe Seiten)
