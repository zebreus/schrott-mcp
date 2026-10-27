---
slug: mv-wismar-alba-metall-nord
clear_fields: 'dropoff_json,geo,phone,website,website_status'
name: ALBA Metall Nord GmbH
trader_type: schrotthaendler
state: MV
city: Wismar
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
provenance_section: Betriebe (Tabelle)
provenance_ankauf_raw: ja
provenance_origin: table
---

# ALBA Metall Nord GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Owner-Korrektur 27.10.2026: ALBA-Identität und aktuelle Annahme offen

- [Korrektur 27.10.2026: Die aktuelle read-only Produktionszeile führt diesen Slug als `ALBA Nord GmbH`, mit Status `aktiv`, Tonnenhofstraße 12, Telefon, ALBA-Nord-Website und `dropoff_json={"allowed":true}`. Das war eine zu weit gehende Zuordnung: Die live gelesene ALBA-Nord-Seite belegt die gegenwärtige Betriebsstätte Wismar und Abfallannahme/Containerdienst für ALBA Nord; die aktuelle Metall-Nord-Standortliste enthält Wismar nicht. Das separat direkt geprüfte Zertifikat für den Erhebungszeitraum 2023 führt ALBA Metall Nord GmbH und ALBA Nord GmbH getrennt an derselben Anschrift, belegt aber weder eine aktuelle eigenständige Metall-Nord-Betriebsstätte noch heutige Kundenannahme. Deshalb wird die historische Identität `ALBA Metall Nord GmbH` wieder als Name geführt, die ALBA-Nord-Kontakt-/Websitezuordnung und die unbestätigte aktuelle Straße/PLZ werden zurückgenommen, die gespeicherte Drop-off-Zusage und das falsche GEO-Paar explizit gelöscht. `status` wechselt auf `pruefung`; kein Schließungsurteil und kein Merge. Die Adresse und Kontakt-/Annahmedaten des aktuellen ALBA-Nord-Standorts bleiben ausschließlich im separaten ALBA-Nord-Dossier. Quellen: https://nord.alba.info/unternehmen/standorte/detail/location/betriebsstaette-wismar-1/ ; https://metall.alba.info/unternehmen/standorte/ ; https://metall.alba.info/fileadmin/user_upload/ALBA_Metall_Nord/PDF/2024/Urkunde_Ver_CCF_ALBA_plc_und_Co._KG_GJ2023_final.pdf ; read-only `public.db` (27.10.2026); README.md, Quellenhierarchie/Betreiberkettenregel; docs/trader-seed.md.]

### Owner-Quellenkorrektur 27.10.2026: ALBA-Identität offenhalten

- [Owner-Quellenkorrektur 27.10.2026: Die aktuelle ALBA-Nord-Standortseite bestätigt die Betriebsstätte an Tonnenhofstraße 12. Das direkt geprüfte ALBA-CCF-Zertifikat vom 29.11.2024 führt für den Erhebungszeitraum 2023 ALBA Metall Nord GmbH und ALBA Nord GmbH getrennt unter dieser Adresse auf. Das ist historische Gegen-Evidenz zur simplen Zuordnung, aber kein Nachweis einer separaten aktuellen Betriebs- oder Annahmestelle 2026. Die aktuelle ALBA-Metall-Nord-Standortübersicht führt Wismar nicht auf; die Auslassung beweist für sich keine Schließung. Darum bleibt die Identität dieses Slugs gegenüber dem ALBA-Nord-Sibling offen: kein Ersatzpunkt übertragen, kein Merge und kein Schließungsurteil. Der klar falsche gespeicherte Punkt zeigt weiterhin auf Am Schwedenstein 13 und wird unabhängig von der Identitätsfrage per `clear_fields: geo` zurückgezogen. Quellen: https://nord.alba.info/unternehmen/standorte/detail/location/betriebsstaette-wismar-1/ ; https://metall.alba.info/unternehmen/standorte/ ; https://metall.alba.info/fileadmin/user_upload/ALBA_Metall_Nord/PDF/2024/Urkunde_Ver_CCF_ALBA_plc_und_Co._KG_GJ2023_final.pdf ; README.md, Quellenhierarchie und Betreiberkettenregel.]

### Owner-GEO-Korrektur 27.10.2026

- [Owner-GEO-Korrektur 27.10.2026: Der read-only Produktionsdatensatz enthielt 53.8895333/11.4386667. Nominatim-Reverse löst diesen Punkt als Gebäude Am Schwedenstein 13, Lembkenhof, Wismar, PLZ 23966 auf; das ist nicht die im Dossier belegte Tonnenhofstraße 12, 23970. Die Adresssuche für Tonnenhofstraße 12 findet zwar ein anderes Industriegebäude, aber dieser Sibling ist ein historischer, nicht als aktuell eigenständiger ALBA-Metall-Nord-Betrieb belegter Datensatz. Deshalb kein Ersatzpunkt und keine Übertragung des anderen Slugs; `clear_fields: geo` zieht nur den falschen Punkt beim regulären Seed zurück. Kein Schließungs- oder Annahmeurteil, kein Merge und keine Produktionsänderung. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.8895333333333&lon=11.4386666667&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Tonnenhofstra%C3%9Fe%2012%2C%2023970%20Wismar&format=jsonv2&addressdetails=1 ; read-only `public.db` (27.10.2026); `docs/trader-seed.md`.]

### Importiert (Seed-Stand 2026-09-30)

- Stahl/NE (lt. schrottplaetze.org)

### Korrektur 01.10.2026 (ALBA-Fix)

- Betreiber-Primärquelle: alba.info/standorte (Corporate-Liste) — Tonnenhofstraße 12, 23970 Wismar gehört der ALBA Nord GmbH („Betriebsstätte Wismar“, +49 3841 704678), NICHT der ALBA Metall Nord GmbH (diese führt Wismar auf keiner aktuellen Liste: Delisted-Protokoll für „Metall Nord Wismar“ ohne Schließungsbeleg).
- name auf Betreiber-Stand korrigiert (ALBA Nord GmbH); Altname „ALBA Metall Nord GmbH“ hier in Timeline archiviert. Slug unverändert. website auf nord.alba.info + aktiv.
- Querverweis (kein Merge): mv-wismar-nord-rwe-umwelt-mecklenburg-vorpommern-contai — gleiche Adresse, historischer Vorgängername (RWE Umwelt MV).
