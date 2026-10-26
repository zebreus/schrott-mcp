---
slug: bw-offenburg-intlekofer-schrott
name: Intlekofer (Schrott)
trader_type: schrotthaendler
state: BW
city: Offenburg
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
clear_fields: 'dropoff_json,geo,phone'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 2
provenance_ankauf_raw: ja
provenance_origin: table
---

# Intlekofer (Schrott)

## Überblick

Die Verzeichnisse nennen für „Intlekofer (Schrott)“ widersprüchlich Griesheimer Straße 27 bzw. 66 und die Telefonnummer 0781 9708589. Das sind Leads, keine zulässigen Betreiber-/Registerbelege; PLZ und Telefon wurden deshalb aus dem Frontmatter entfernt und bleiben in der Timeline als ungeprüfte Spur. Eine Betreiberwebsite oder eindeutige Registerzuordnung fehlt; `status: pruefung` ist keine Schließungsbehauptung.

**Preise:** Ankauf — keine verifizierte Preisliste; Verkauf — keine Preise belegt; Gebühren — keine Gebührenliste belegt. Die widersprüchliche Hausnummer muss vor jeder Geokodierung geklärt werden.

## Timeline

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Read-only Produktion enthält weiterhin 0781 9708589, `dropoff_json={"allowed":true}` und GEO 48.4685/7.9436667. Der Punkt löst auf Spitalstraße 5 in der Offenburger Altstadt auf; die zwei widersprüchlichen Griesheimer Verzeichnis-Leads (Hausnummer 27 bzw. 66) belegen weder diese Position noch einen aktuellen Annahmeort. `clear_fields: 'dropoff_json,geo,phone'` nimmt die nicht belegte Annahme, Telefonnummer und den Fehlpunkt zurück. Widersprüchliche Leads bleiben historisch dokumentiert, Status `pruefung`; keine Ersatzadresse oder -koordinate geraten. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.4685&lon=7.94366666666667&zoom=18&addressdetails=1 ; https://www.openstreetmap.org/node/9793352852 ; https://www.gelbeseiten.de/gsbiz/a6ce9769-8a71-43f0-b21d-bca4b96bfd50 (Lead) ; https://www.schrottplatz-info.de/schrottplatz/Offenburg/Intlekofer (Lead) ; README.md, Quellenhierarchie.]

### Owner-GEO-Prüfung 21.10.2026

- [Owner-GEO-Prüfung 21.10.2026: Der gespeicherte Produktionspunkt löst auf das OSM-Kartenobjekt Spitalstraße 5 in der Offenburger Altstadt auf. Die Verzeichnis-Leads Griesheimer Str. 27 und 66 sind getrennte Gebäude in Griesheim; die Karten belegen keine davon als Intlekofer-Annahmeort und lösen den Adresskonflikt nicht. Kein bestätigter GEO-Kandidat; offen. Quellen: [Reverse-Abfrage](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=48.4685&lon=7.94366666666667&zoom=18&addressdetails=1), [OSM-Objekt am Produktionspunkt](https://www.openstreetmap.org/node/9793352852), [Owner-Bericht](../../research/wave-20261021-0900-geo-bw.md).]

### Recherche 21.10.2026

- [Recherche 21.10.2026: Neue Suche bestätigt nur die bekannten Verzeichnis-Leads: Das Telefonbuch nennt Griesheimer Str. 27, 77652 und 0781 9708589; Schrottplatz-Info führt Nr. 66 mit derselben Nummer. Keine Quelle aus der zulässigen Betreiber-/Register-/Kommunalkategorie löst den Hausnummernkonflikt oder die Telefonnummer auf. PLZ und Telefon daher aus strukturierten Feldern entfernt; Quellenwerte und frühere Recherche bleiben hier erhalten. Ankauf: keine verifizierten Preise; Verkauf: keine Preise; Gebühren: keine Tarife. GEO erst nach bestätigter aktueller Annahmeadresse. Quellen (Leads): https://www.dastelefonbuch.de/Branchen/Schrotthandel/Offenburg ; https://www.schrottplatz-info.de/schrottplatz/Offenburg/Intlekofer .]

### Recherche 18.10.2026 — Welle 02:00 / a

- [Recherche 18.10.2026: Beide widersprüchlichen Detailseiten heute direkt gelesen: Gelbe Seiten nennt Griesheimer Str. 27, Schrottplatz-info Nr. 66; beide führen 0781 9708589. Damit ist der Hausnummernkonflikt aktuell im Quellenbestand vorhanden und nicht nur eine alte Suchindexabweichung. Gleiche Telefonnummer beweist weder Umzug noch identischen Rechtsträger, zwei Verzeichnisse bleiben Leads. Schrottplatz-info hat weiterhin keine vollständigen Zeiten/Leistungen. Straße bleibt leer, übrige historische Felder nicht als neu primärbestätigt ausgegeben. Ankauf/Verkauf/Gebühren ohne Preislisten auf beiden Seiten. Offen: Inhaber, Annahmeadresse und belastbare Zeit-/Umzugsfolge aus Betreiber/Kommunalquelle; kein GEO. Quellen (Leads): https://www.gelbeseiten.de/gsbiz/a6ce9769-8a71-43f0-b21d-bca4b96bfd50 ; https://www.schrottplatz-info.de/schrottplatz/Offenburg/Intlekofer .]

### Recherche 01.10.2026 (Tiefenrecherche-Welle)

- [Recherche 01.10.2026: Hausnummernkonflikt — KEIN Street-Fill: Gelbe Seiten (+ oeffnungszeitenbuch 5338452B) nennen Griesheimer Str. 27, 77652 Offenburg-Griesheim vs. schrottplatz-info + oeffnungszeitenbuch 2861405K (Ralf Intlekofer) + Telefonbuch nennen Griesheimer Str. 66, 77652 Offenburg. PLZ 77652 konsistent → Postcode-Fill; Telefon 0781 9708589 2-fach (schrottplatz-info + infobel DE106095535-07819708589). KEINE Betreiber-Website, KEIN Register; Status aktiv → pruefung (Hausnummer offen). Quelle(n): https://www.gelbeseiten.de/gsbiz/a6ce9769-8a71-43f0-b21d-bca4b96bfd50 (Nr. 27) + https://www.schrottplatz-info.de/schrottplatz/Offenburg/Intlekofer (Nr. 66, Tel) + https://www.oeffnungszeitenbuch.de/filiale/Offenburg-Ralf%20Intlekofer%20Schrotthaendler-2861405K.html (Nr. 66).]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Die aktuelle Suche findet weiterhin nur die bereits dokumentierten Verzeichnis-Spuren, darunter das Telefonbuch mit Griesheimer Straße 27 und schrottplatz-info/Öffnungszeitenbuch mit Hausnummer 66. Keine Betreiberquelle aufgetaucht; die Abweichung ist nicht aufgelöst und die vorhandenen Kontaktdaten werden nicht weiter bestätigt. Keine Änderung der Felder oder des Prüfstatus. Für Ankauf, Verkauf und Gebühren keine Zielbetriebs-Preisliste gefunden. Quelle(n): https://www.dastelefonbuch.de/Branchen/Schrotthandel/Offenburg ; https://www.schrottplatz-info.de/schrottplatz/Offenburg/Intlekofer ; https://www.oeffnungszeitenbuch.de/filiale/Offenburg-Ralf%20Intlekofer%20Schrotthaendler-2861405K.html]

### Importiert (Seed-Stand 2026-09-30)

- Schrott
- Größe: klein
