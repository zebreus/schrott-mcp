---
slug: bb-premnitz-ot-mogelin-ahab-metall
name: ahab-metall e.K.
trader_type: schrotthaendler
state: BB
city: Premnitz OT Mögelin
street: Carl-Zeiss-Str. 14
postcode: '14727'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
clear_fields: 'phone'
provenance_section: 'Nachtrag Audit-Runde 3 (Stand: 2026-09-27)'
provenance_ankauf_raw: ja
provenance_origin: table
---

# ahab-metall e.K.

## Überblick

„ahab-metall e.K.“ und Carl-Zeiss-Straße 14 sind nur durch den historischen Schrottradar-Lead dokumentiert. Der Direktcheck der NorthData-Suche `ahab-metall` ergab auf der lesbaren ersten Trefferseite keinen orts-/namensgleichen Rechtsträger, nur unpassende Metallfirmen; das ist kein vollständiger Register-Negativnachweis. Für den e.K.-Zusatz fehlen Inhaber und HRA. Deshalb Status auf `pruefung` statt unbelegt `aktiv`; Anschrift als Altangabe erhalten, nicht als bestätigte Annahmestelle ausweisen.

**ANKAUF:** Seed „ja“ ist nicht primär bestätigt, keine Kurse. **VERKAUF / GEBÜHREN:** keine zuordenbare Liste. Offen: korrekte Namensschreibweise, Inhaber/HRA, heutiger Betrieb an der Adresse; Gewerberegister Premnitz abfragen.

## Timeline
- [Owner-Korrektur 28.10.2026: **Der Platzhalter im Telefonfeld wird zurückgenommen.** Die Zeile trug in der Produktion „n.e.“ als Telefonnummer - das ist kein Rufnummernwert, sondern eine Import-Platzhalterangabe („nicht ermittelbar“). Das Dossier führt **keine** belegte Telefonnummer; `clear_fields: 'phone'` nimmt den Platzhalter beim nächsten regulären Seed zurück, das Feld bleibt danach **leer** (unbekannt) und wird nicht mit einer erfundenen Nummer gefüllt. Weitere Kontaktangaben der Zeile bleiben unverändert. Quellen: read-only `public.db` (28.10.2026) ; `docs/trader-seed.md` (Semantik der `clear_fields`-Direktive).]

### Recherche 20.10.2026

- [Recherche 20.10.2026: Erneute Suche nach der exakten Schreibweise „ahab-metall e.K.“ und der Carl-Zeiss-Str. 14 ergab keine aktuelle Betreiberseite, belastbare Registerauskunft oder Kundenannahmestelle. Das ist kein Negativnachweis für eine Firma oder Schließung; der bisherige begrenzte NorthData-Nichtfund bleibt nur ein Recherchelimit. Name, Adresse und e.K.-Zusatz bleiben unbestätigte Seed-Leads; Status pruefung. ANKAUF/VERKAUF/GEBÜHREN: keine konkrete Preisliste. GEO bis zur Auflösung von Rechtsträger und Standort nicht durchführen. Quelle/Abgleich: https://www.northdata.de/?query=ahab-metall (Suchgegenprobe 20.10.2026; keine passende Betreiberbestätigung auf der lesbaren Trefferseite).]

### Recherche 18.10.2026 (Welle 1202 Nordost)

- [Recherche 18.10.2026: Exakte NorthData-Suggest-Abfrage ahab-metall heute HTTP 429/Zu viele Abfragen; kein aktueller Registerbefund und insbesondere keine Bestätigung des e.K.-Zusatzes, Inhabers oder HRA. Der frühere beschränkte Nichtfund bleibt historische Recherche, keine Negativauskunft. Carl-Zeiss-Str. 14 weiter ungesicherter Seed-Lead, kein öffentlicher Ankaufhof nachgewiesen; status pruefung und Bestandswerte erhalten. Ankauf/Verkauf/Gebühren ohne verifizierte Zahlen. Weiterer Belegweg kommunale Gewerbeauskunft Premnitz plus registergenauer Inhaber-/Firmenschreibweisenabgleich, danach Betriebsstätte/GEO. Quelle/Abrufversuch: https://www.northdata.de/suggest.json?query=ahab-metall&countries=DE .]

### Recherche 08.10.2026

- [Korrektur 08.10.2026: Belegaudit: bisheriger Aktivstatus/Ankauf-ja beruhen ausschließlich auf Schrottradar-Lead. NorthData-Suche ahab-metall direkt lesbar, erste Trefferseite ohne eindeutig passendes Premnitzer Unternehmen; keine vollständige Registerauskunft und kein Schließungsbeleg. status pruefung gesetzt, vorhandene Anschrift erhalten; keine Kontakte/Website gefüllt. Für e.K. insbesondere Inhaber und HRA ermitteln. ANKAUF/VERKAUF/GEBÜHREN ohne verifizierte Zahlen. Quelle(n): https://www.northdata.de/?query=ahab-metall ; historische Seednotiz Schrottradar (kein Primärbeleg)]

### Importiert (Seed-Stand 2026-09-30)

- Metall/Schrott
- Adressbeleg: schrottradar-Beleg via Runde 3
