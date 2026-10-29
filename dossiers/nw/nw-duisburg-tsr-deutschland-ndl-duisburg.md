---
slug: nw-duisburg-tsr-deutschland-ndl-duisburg
name: TSR Deutschland GmbH & Co. KG (Ndl. Duisburg)
trader_type: schrotthaendler
state: NW
city: Duisburg
street: Rohstoffinsel 2-10
postcode: "47138"
phone: 02034 5007-0
email: ""
opening_hours: 'Mo-Fr 06:00-17:30'
website: https://www.tsr-recycling.de/standorte/schrotthaendler-duisburg/
website_status: blockiert
status: aktiv
certifications: '["Vom Betreiber auf der Standortseite veroeffentlichte Zertifikatsdokumente (PDF-Links, Abruf 29.10.2026): Entsorgungsfachbetrieb (EfB; Zertifikatsdatei laut Dateiname ‚EfB_Zertifikat_RheinRuhrII_Duisburg_Mülheim‚), Qualitätsmanagement DIN EN ISO 9001:2015, Qualitätsmanagementsystem Verordnung (EU) Nr. 333/2011 und Nr. 715/2013, Shredderanlage gemäß Altautoverordnung, Autorisierter Sammelpartner im A I U I F e.V., Energiemanagementsystem DIN EN ISO 50001:2018, Managementsystem für Sicherheit und Gesundheit bei der Arbeit (ISO 45001), Umweltmanagementsystem DIN EN ISO 14001:2015. Die verlinkten PDF-Dateien lieferten am 29.10.2026 beim Direktabruf HTTP 403 (Bot-Schutz der Seite), deshalb sind Zertifikatsnummern und Gueltigkeitsdauer hier NICHT belegt - die Liste ist die Zertifikatsbekanntgabe des Betreibers fuer diesen Standort."]'
description: 'TSR-Niederlassung Duisburg auf der Rohstoffinsel: Anlaufstelle fuer Gewerbe- und Industriekunden, Shredder- und TSR40-Anlage, Containerservice fuer Duisburg und Umgebung; keine Annahme von Privatkunden.'
dropoff_json: '{"allowed": true, "customer_types": ["gewerbe"], "days": ["Mo", "Di", "Mi", "Do", "Fr"], "conditions": "Keine Annahme von Privatkunden; Anlaufstelle fuer Gewerbe-/Industriekunden (TSR-Standortseite, 30.09.2026)"}'
pickup_json: ""
provenance_section: Ruhrgebiet – Großrecycler / Stahlwerksnah (Duisburg, Dortmund,
  Gelsenkirchen, Lünen, Essen)
provenance_ankauf_raw: ja (nur Gewerbe, keine Privatkunden)
provenance_origin: table
---

# TSR Deutschland GmbH & Co. KG (Ndl. Duisburg)

## Überblick

Großrecycler auf der Rohstoffinsel Duisburg (Rohstoffinsel 2-10, 47138 Duisburg);
Stahl-/NE-Schrott mit Shredder und TSR40-Anlage. Nur Gewerbe, keine Privatkunden.
Zentrale: 02306 1063800 (Lünen).

## Timeline

### Importiert (Seed-Stand 30.09.2026)

- Stahl-/NE-Schrott, Shredder, TSR40-Anlage (Rohstoffinsel 2-10) [Quelle: seed/nw.json,
  Section "Ruhrgebiet – Großrecycler / Stahlwerksnah"]
- Adressbeleg: tsr-recycling.de/standorte + Bibliothek-Zertifikat [Quelle: seed-Notes]

### Korrektur 30.09.2026 (Feedback-Triage)

- [Korrektur Feedback fb1845: Telefon war Zentrale Lünen (02306 1063800 = TSR-HQ Brunnenstr. 138, bestätigt via wer-zu-wem.de + TSR-Impressum) → Standort-Nummer 02034 5007-0 (TSR-Standortseite via Jina-Reader: T +49 2034 5007-0, F ...5007-68); Verzeichnis-Angabe „(0203) 45 00 70" (golocal, lokaleschrottplatz.de) weicht ab und ist gegen die Primärquelle unterlegen; Quelle: tsr-recycling.de/standorte/schrotthaendler-duisburg]
- [Korrektur Feedback fb1845: keine Privatannahme mit Website-Beleg — TSR-Seite: „Anmerkung: Keine Annahme von Privatkunden", „Anlaufstelle für Gewerbe- und Industriekunden" → dropoff_json (gewerbe only); Quelle: TSR-Standortseite (30.09.2026)]
- [Korrektur Feedback fb1845: website_status blockiert (Human-Challenge auf tsr-recycling.de, verifiziert 30.09.2026); Quelle: Direktabruf]
- (Klärfall): Öffnungszeiten widersprüchlich — TSR-Seite Mo–Fr 06:00–17:30 vs. lokaleschrottplatz.de Mo–Fr 06:00–17:00 vs. schrottplaetze.org Mo–Do 06:00–16:30/Fr 15:00 (dort zudem alte Straßenschreibung „Schrottinsel") → opening_hours leer gelassen.

### Recherche 29.10.2026

- [Recherche 29.10.2026: Standortseite Duisburg am 29.10.2026 ueber den Jina-Reader erneut live gelesen (Direktabruf = Bot-Challenge, `website_status: blockiert` bestaetigt). Woertlich bestaetigt: „TSR Deutschland GmbH & Co. KG, Rohstoffinsel 2-10, 47138 Duisburg", „T +49 2034 5007-0", „F +49 2034 5007-68", „Oeffnungszeiten: Mo-Fr: 06:00 - 17:30", „Anmerkung: Keine Annahme von Privatkunden", „Die Duisburger Rohstoffinsel ist Anlaufstelle fuer **Gewerbe- und Industriekunden** aller Art", „Fe-Metalle koennen auf der Rohstoffinsel Duisburg ausser ueber den Land- auch ueber den Seeweg angeliefert werden", „TSR bietet fuer Duisburg und Umgebung einen umfassenden Containerservice an", „In der innovativen Anlage auf der Rohstoffinsel Duisburg wird das nach patentiertem Verfahren hergestellte Recyclingprodukt TSR40 produziert". **Oeffnungszeiten** jetzt aus der Betreiber-Primärquelle gefuellt (Mo-Fr 06:00-17:30); die frueher notierte Aggregator-Abweichung (schrottplaetze.org Mo-Do 06:00-16:30/Fr 15:00, lokaleschrottplatz.de 17:00) ist nach der Quellenhierarchie ein Aggregator-Lead und unterliegt dem Betreiberwert - die fruehere Klaerfall-Notiz bleibt unveraendert stehen. **Anmerkung** der Seite: keine Privatannahme (im `dropoff_json` bereits so abgebildet). **ANKAUF**: Abnahme „jeder Art von Fe- und NE-Metallen in jeder Form" (Gruppen-Seite Schrottlieferanten), „Marktgerechte Preise ... das, was der jeweilige Schrott wert ist", sortiertes Material wird „hoeher verguetet"; **keine Zahlenpreise** - „Die Schrottpreise unterliegen taeglichen Schwankungen ... bieten wir Ihnen die besten tagesaktuellen Preise", Preisinformation nur auf Anfrage (Hotline Schrottlieferanten 0800 8772667). **VERKAUF**: Recyclingprodukt TSR40 und aufbereitete Recyclingrohstoffe an Industrie; keine Verkaufspreisliste. **GEBUEHREN**: keine Tarife; Containerdienst als Angebot. **Offen**: ob das nun gesetzte `opening_hours` die Waagen- oder die Hallenzeiten meint (Seite nennt nur eine Zeitangabe). Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-duisburg/ (Abruf 29.10.2026); https://www.tsr-recycling.de/kunden/schrottlieferanten/ (Abruf 29.10.2026).]

- [Recherche 29.10.2026 (Zertifikats-Nachtrag): Die Standortseite Duisburg veroeffentlicht **acht Zertifikatsdokumente** mit PDF-Link: „Entsorgungsfachbetrieb", „Qualitaetsmanagement gemaess DIN EN ISO9001:2015", „Qualitaetsmanagementsystem gemaess Verordnung (EU) Nr. 333/2011 und Nr. 715/2013", „Shredderanlage gemaess Altfahrzeug-Verordnung", „Autorisierter Sammelpartner im A I U I F e.V.", „Energiemanagementsystem gemaess DIN EN ISO 50001:2018", „Managementsystem fuer Sicherheit und Gesundheit bei der Arbeit", „Umweltmanagementsystem gemaess DIN EN ISO 14001:2015". **Praezise Einschraenkung**: die verlinkten PDF-Dateien lieferten am 29.10.2026 beim Direktabruf **HTTP 403** (Bot-Schutz der Seite) — **Zertifikatsnummern und Gueltigkeitsdauer sind daher nicht belegt** und werden nicht behauptet; `certifications` fuehrt ausschliesslich die Zertifikatsbekanntgabe des Betreibers fuer diesen Standort. Der EfB-Dateiname lautet „EfB_Zertifikat_RheinRuhrII_Duisburg_Mülheim_11052027.pdf" — er deckt also **Duisburg und Mülheim** gemeinsam ab (die TSR-Zertifikatslisten der beiden Standortseiten unterscheiden sich dementsprechend). Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-duisburg/ (Abruf 29.10.2026); Direktabruf der Zertifikats-PDFs am 29.10.2026 (HTTP 403).]
