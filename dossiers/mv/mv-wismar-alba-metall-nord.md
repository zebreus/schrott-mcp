---
slug: mv-wismar-alba-metall-nord
clear_fields: geo
name: ALBA Nord GmbH
trader_type: schrotthaendler
state: MV
city: Wismar
street: Tonnenhofstr. 12
postcode: '23970'
phone: +49 3841 704678
email: ''
opening_hours: ''
website: https://nord.alba.info/
website_status: aktiv
status: aktiv
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

### Owner-GEO-Korrektur 27.10.2026

- [Owner-GEO-Korrektur 27.10.2026: Der read-only Produktionsdatensatz enthielt 53.8895333/11.4386667. Nominatim-Reverse löst diesen Punkt als Gebäude Am Schwedenstein 13, Lembkenhof, Wismar, PLZ 23966 auf; das ist nicht die im Dossier belegte Tonnenhofstraße 12, 23970. Die Adresssuche für Tonnenhofstraße 12 findet zwar ein anderes Industriegebäude, aber dieser Sibling ist ein historischer, nicht als aktuell eigenständiger ALBA-Metall-Nord-Betrieb belegter Datensatz. Deshalb kein Ersatzpunkt und keine Übertragung des anderen Slugs; `clear_fields: geo` zieht nur den falschen Punkt beim regulären Seed zurück. Kein Schließungs- oder Annahmeurteil, kein Merge und keine Produktionsänderung. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=53.8895333333333&lon=11.4386666667&zoom=18&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=Tonnenhofstra%C3%9Fe%2012%2C%2023970%20Wismar&format=jsonv2&addressdetails=1 ; read-only `public.db` (27.10.2026); `docs/trader-seed.md`.]

### Importiert (Seed-Stand 2026-09-30)

- Stahl/NE (lt. schrottplaetze.org)

### Korrektur 01.10.2026 (ALBA-Fix)

- Betreiber-Primärquelle: alba.info/standorte (Corporate-Liste) — Tonnenhofstraße 12, 23970 Wismar gehört der ALBA Nord GmbH („Betriebsstätte Wismar“, +49 3841 704678), NICHT der ALBA Metall Nord GmbH (diese führt Wismar auf keiner aktuellen Liste: Delisted-Protokoll für „Metall Nord Wismar“ ohne Schließungsbeleg).
- name auf Betreiber-Stand korrigiert (ALBA Nord GmbH); Altname „ALBA Metall Nord GmbH“ hier in Timeline archiviert. Slug unverändert. website auf nord.alba.info + aktiv.
- Querverweis (kein Merge): mv-wismar-nord-rwe-umwelt-mecklenburg-vorpommern-contai — gleiche Adresse, historischer Vorgängername (RWE Umwelt MV).
