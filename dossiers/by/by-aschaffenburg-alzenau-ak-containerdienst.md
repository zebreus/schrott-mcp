---
slug: by-aschaffenburg-alzenau-ak-containerdienst
name: AK Containerdienst
trader_type: containerdienst
state: BY
city: Aschaffenburg
street: ''
postcode: ''
clear_fields: geo
phone: ''
email: ''
opening_hours: ''
website: https://ak-containerdienst.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 'Neu: kept (92, davon Ankauf ja: 9)'
provenance_ankauf_raw: unklar
provenance_origin: table
---

# AK Containerdienst

## Überblick

Die Betreiberwebsite und NorthData identifizieren AK Containerdienst GmbH, HRB 53757, mit Sitz/Adresse Carl-Zeiss-Straße 37, 63322 Rödermark. Die AGB (Stand 15.01.2022) nennen diese Adresse ausdrücklich als Metall-Anlieferplatz für private und gewerbliche Lieferanten. Die Homepage führt Aschaffenburg ausdrücklich als Lieferort im Rhein-Main-Servicegebiet; Alzenau wird nicht ausdrücklich genannt. Der Seed bleibt als BY-Servicegebietszeile erhalten, nicht als bayerischer Betriebs- oder Metallannahmestandort. Der belegte Metall-Anlieferplatz liegt in Hessen und wird nicht auf diesen BY-Datensatz übertragen. Die bisherige GEO-Koordinate führte zu einem Jugendkulturzentrum in Aschaffenburg und wird beim regulären Seed explizit entfernt; kein Ersatzpunkt ist belegt. Die Website zeigt aktuell ein Online-Bestellformular mit Entsorgungsgebühren pro Tonne; deren Gültigkeit ist nicht datiert und die Homepage widerspricht sich beim Mischmüllpreis.

**Ankauf:** Schrottabholung laut Homepage zum jeweiligen Tagespreis; Werkstätten und Metallbetriebe können kostenlos einen Container für Metall/Eisenschrott erhalten, der Inhalt werde vergütet. Die AGB nennen börsenabhängige Tagespreise auf Anfrage, einen Abschlag bei mehr als 1 % Fremdmaterial und keinen Vergütungsanspruch für Container-Schrott unter 1.000 kg. Keine numerischen Ankaufskurse. **Verkauf:** keine eigene Verkaufspreisliste belegt. **Gebühren:** Das Onlineformular zeigte beim Abruf am 22.10.2026 folgende Entsorgungspreise je Tonne (zzgl. 19 % MwSt.): Baumischabfall/Mischmüll 220 €, reiner Bauschutt ab 55 €, Holz A1–A3 75 €, Holz A4 149 €, Erdaushub ohne Wurzeln/Steine/Grasnarbe 55 €, mit diesen Beimengungen ab 85 €, Rigips/Gipsputz 150 €, Grünschnitt ohne Wurzeln/Stammholz 85 €, mit Wurzeln/Stammholz 149 €, Leichtbaustoffe/Ytong 150 €, Porenbeton 150 € und Glas 89 €. Auf demselben Homepageabruf warb ein Banner zugleich mit „Frühjahrsputzpreise Mischmüll“ von 189 €/t; der Widerspruch und die nicht ausgewiesene Gültigkeitsdauer lassen offen, welcher Preis anwendbar ist. Die AGB (15.01.2022) setzen bei Entsorgungscontainern mindestens 1 t an; Anlagen-/Genehmigungskosten können zusätzlich berechnet werden. Die Mietdauer ist im Text widersprüchlich (max. 30 Tage im Geschäftsgegenstand, an anderer Stelle 5 Tage; 5 €/netto/Tag erst jenseits von 30 Tagen). Diese alten Bedingungen sind keine pauschale aktuelle Gebührenzusage; Konditionen vor Auftrag erfragen.

## Timeline

### GEO-Prüfung 22.10.2026

- [GEO-Prüfung 22.10.2026: Die read-only Produktionszeile für diesen exakten Slug enthielt 49.9718667, 9.1479667. Nominatim-Reverse ordnet den Punkt dem Jugendkulturzentrum am Kirchhofweg, Aschaffenburg, zu (OSM-Way 124365324), nicht AK Containerdienst. Die aktuelle Betreiberhomepage nennt Aschaffenburg ausdrücklich im Container-Liefergebiet; Alzenau nicht. Die AGB nennen Rödermark als Metall-Anlieferplatz. Deshalb wird `city` auf das direkt belegte Servicegebiet Aschaffenburg eingeengt, `state: BY` als bisherige Servicegebietszuordnung nicht stillschweigend umklassifiziert und `clear_fields: geo` entfernt das nicht zuordenbare Alt-Paar beim normalen Seed. Weder Rödermark noch das Jugendkulturzentrum werden als Ersatz-GEO übernommen. Quellen: https://ak-containerdienst.de/ ; https://ak-containerdienst.de/agb/ ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=49.9718667&lon=9.1479667&zoom=18&addressdetails=1 ; https://www.openstreetmap.org/way/124365324 .]

### Importiert (Seed-Stand 2026-09-30)

- Containerdienst
- Adresse: Aschaffenburg/Alzenau (nur Facebook)

### Recherche 30.09.2026

- [Recherche 30.09.2026: AK Containerdienst GmbH nur Einzelbeleg Carl-Zeiss-Str. 37, 63322 Rödermark Tel 06074 6989881, Stadt-Mismatch zu Aschaffenburg/Alzenau daher Strasse/PLZ geskippt (Einzelbeleg, unsicher), kein Frontmatter-Fill; Quelle: https://www.gelbeseiten.de/suche/containerdienst/alzenau]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Primärquelle verifiziert (Impressum Name+HRB+Ort): AK Containerdienst GmbH, GF Amalija Krämer, Carl-Zeiss-Straße 37, 63322 Rödermark, HRB 53757 AG Offenbach, Tel. 06074 6989881; Aschaffenburg/Alzenau liegt im eigenen Servicegebiet (Rhein-Main) — Sitz bleibt Rödermark, daher weiter kein Street/PLZ-Fill (City-Mismatch), nur Website-Fill; Schrott-Ankauf zum Tagespreis + Efb bestätigt; Quelle(n): https://ak-containerdienst.de/, https://ak-containerdienst.de/impressum/, Betreiber-Facebook (AKContainerdienst)]
- [Recherche 01.10.2026 (Feedback 2870): unberechtigt für Fill — Sitz Rödermark (Hessen) re-verifiziert, Aschaffenburg/Alzenau bleibt Servicegebiet ohne Beleg für Sitz; Kontaktdaten bereits 30.09./01.10.2026 geklärt, kein Frontmatter-Fill; Quelle(n): siehe Vorbullet]

### Recherche 22.10.2026

- [Recherche 22.10.2026: Impressum nennt AK Containerdienst GmbH, Geschäftsführerin Amalija Krämer, HRB 53757 AG Offenbach und Carl-Zeiss-Straße 37, 63322 Rödermark; NorthData führt dieselbe Registerfirma und Adresse. Die AGB nennen genau diese Adresse als Anlieferplatz für private und gewerbliche Metall-/Schrottlieferanten. Der belegte Annahmeplatz liegt in Hessen, während der bestehende Live-Datensatz/Slug unter BY und Aschaffenburg/Alzenau geführt wird. Wegen dieses Cross-State-Konflikts `state`, `city`, Straße und PLZ nicht geändert; keine stille Re-Klassifizierung oder Geokodierung. Homepage-Schrotthandel nennt Abholung zum aktuellen Tagespreis sowie kostenlosen Container für Werkstätten/Metallbetriebe mit Vergütung des Inhalts; AGB konkretisieren Tagespreise auf Anfrage, >1-%-Fremdmaterialabschlag und fehlende Vergütung bei Container-Schrott unter 1.000 kg. Keine numerischen Schrott-Ankaufskurse. Onlineformular zeigte am Abrufdatum 22.10.2026 Entsorgungsgebühren pro Tonne (zzgl. 19 % MwSt.): Misch-/Baumischabfall 220 €, Bauschutt ab 55 €, Holz A1–A3 75 €, Holz A4 149 €, Erdaushub ohne Wurzeln/Steine/Grasnarbe 55 €, mit Beimengungen ab 85 €, Rigips/Gipsputz 150 €, Grünschnitt ohne Wurzeln/Stammholz 85 €, mit Wurzeln/Stammholz 149 €, Ytong/Leichtbaustoffe 150 €, Porenbeton 150 €, Glas 89 €. Homepagebanner wirbt gleichzeitig mit 189 €/t Mischmüll; Gültigkeit und maßgeblicher Preis bleiben deshalb offen. AGB vom 15.01.2022: Mindestmenge Entsorgungscontainer 1 t, mögliche Anlagen-/Genehmigungszuschläge; Mietregelungen widersprechen sich (max. 30 Tage, andernorts 5 Tage und 5 €/netto/Tag erst nach 30 Tagen), daher keine aktuelle pauschale Gebühr. Quellen: https://ak-containerdienst.de/ ; https://ak-containerdienst.de/impressum/ ; https://ak-containerdienst.de/schrotthandel/ ; https://ak-containerdienst.de/agb/ ; https://www.northdata.de/AK+Containerdienst+GmbH,+R%C3%B6dermark/Amtsgericht+Offenbach+HRB+53757.]
- [Quellenprüfung 22.10.2026, 01:30 UTC: Die Betreiberhomepage nennt Aschaffenburg ausdrücklich als Ort der Containerlieferung und beschreibt das erweiterte Rhein-Main-Gebiet als Servicegebiet. Alzenau wird nicht ausdrücklich genannt; eine bayerische Betriebs- oder Metallannahmestelle ist nicht belegt. Die AGB bestimmen weiterhin Rödermark als Metall-Anlieferplatz. Damit ist Container-Service in Aschaffenburg belegt, aber weder ein Vor-Ort-Betrieb noch Metallannahme in BY. Keine Änderung an `state`, `city`, Straße, PLZ oder Koordinaten; die Bedeutung der regionalen Seed-Zeile und die Zuordnung Alzenau bleiben offen. Quellen: https://ak-containerdienst.de/ ; https://ak-containerdienst.de/agb/.]
