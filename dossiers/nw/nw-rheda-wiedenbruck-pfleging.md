---
slug: nw-rheda-wiedenbruck-pfleging
name: Berni Pfleging GmbH
trader_type: schrotthaendler
state: NW
city: Rheda-Wiedenbrück
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: 'Historischer Unternehmensgegenstand: An- und Verkauf von Eisen- und Stahlschrott; GmbH i.L., gelöscht 24.10.2022.'
clear_fields: 'geo,phone'
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Pfleging

## Überblick

Die Berni Pfleging GmbH (HRB 6222 AG Gütersloh) führte historisch den An- und Verkauf von Eisen-/Stahlschrott. Der nun direkt geprüfte Registerspiegel dokumentiert Liquidator Berni Pfleging und eine Veröffentlichung vom 03.04.2020 sowie die formelle Löschung am 24.10.2022. Damit ist der frühere Status `aktiv` nicht haltbar. Mangels Nachweis einer Fortführung oder eines Rechtsnachfolgers wird der Seed auf `pruefung` gesetzt, nicht als Standort geschlossen behauptet. Fürs Unternehmen sind keine aktuellen Ankauf-/Verkaufsbedingungen oder Gebühren nachweisbar. Fürst-Bentheim-Str. 36 und 05242 401369 bleiben historische Stammdaten, nicht aktuelle Kontakt-/Annahmebestätigung. Der gespeicherte GEO-Punkt löst an einer anderen Straße auf und wird mit `clear_fields: 'geo'` beim nächsten regulären Seed zurückgezogen; keine Ersatzkoordinate.

## Timeline

### Owner-Nachtrag 26.10.2026

- [Owner-Nachtrag 26.10.2026: Eine frische read-only-Abfrage der Produktionszeile bestätigt, dass Fürst-Bentheim-Str. 36, 33378 und 05242 401369 noch in den typisierten Kontaktfeldern standen. Das Dossier bezeichnet diese Werte bereits als historische Stammdaten; die heute erneut gelesene NorthData-Chronik weist die Löschung der Berni Pfleging GmbH am 24.10.2022 aus, liefert aber keinen Nachweis eines aktuellen Betriebs oder Rechtsnachfolgers. Damit sind Adresse und Rufnummer nicht als aktuelle Kontaktdaten kuratiert: Straße/PLZ werden beim nächsten geänderten Seed geleert und `clear_fields: 'phone'` entfernt die alte Rufnummer. Dieser Nachtrag ersetzt die im nachfolgenden Owner-Eintrag festgehaltene frühere Entscheidung, die beiden Werte noch in den strukturierten Feldern zu behalten; die historische Spur bleibt in der Timeline erhalten. Dies ist keine Behauptung, dass die Adresse falsch sei, keine Schließung des Standorts und keine Nachfolgezuordnung; `status: pruefung` bleibt. Der GEO-Altpunkt wird weiterhin separat mit `clear_fields: 'geo'` entzogen. Quellen: read-only `public.db`, 26.10.2026; [NorthData-Chronik HRB 6222](https://www.northdata.de/Berni+Pfleging+GmbH,+Rheda-Wiedenbr%C3%BCck/Amtsgericht+G%C3%BCtersloh+HRB+6222); [Seed-Clear-Semantik](../../docs/trader-seed.md).]

### Korrektur 26.10.2026

- [Korrektur 26.10.2026: North-Data-Detail HRB 6222 AG Gütersloh direkt geprüft. Es führt Berni Pfleging GmbH mit historischem Gegenstand An-/Verkauf von Eisen- und Stahlschrott, Liquidator Berni Pfleging und Veröffentlichung vom 03.04.2020; die Gesellschaft wurde am 24.10.2022 formell gelöscht. Damit ist `status: aktiv` nicht mehr gerechtfertigt. Status auf `pruefung` korrigiert, nicht `geschlossen`, da keine aktuelle Prüfung des Betriebsorts oder belegte Nachfolge vorliegt. Beschreibung auf historischen Gegenstand/Löschdatum präzisiert; Straße und Telefon bleiben im Datensatz als nicht aktuelle Leads, werden nicht als aktive Betreiberkontakte dargestellt. Die read-only Produktionszeile enthielt 51.8497, 8.3002; OSM-Rückwärtssuche löst den Punkt an Zum Eidhagen auf, nicht an der historischen Fürst-Bentheim-Str. 36. Die Adresssuche findet den historischen Hausnummern-Gebäudepunkt rund 1,2 km entfernt, aber keine aktuelle Betreiber-/Annahmestelle. Daher `clear_fields: 'geo'`, ohne Ersatzpin; die gelöschte GmbH wird nicht als heutiger Betrieb geokodiert. Ankauf/Verkauf/Gebühren: keine aktuellen Konditionen, Preislisten oder Tarife; keine Nachfolgezuordnung vorgenommen. Quellen: https://www.northdata.de/Berni+Pfleging+GmbH,+Rheda-Wiedenbr%C3%BCck/Amtsgericht+G%C3%BCtersloh+HRB+6222 ; https://nominatim.openstreetmap.org/reverse?lat=51.8497&lon=8.3002&format=jsonv2&addressdetails=1 ; https://nominatim.openstreetmap.org/search?q=F%C3%BCrst-Bentheim-Stra%C3%9Fe%2036%2C%2033378%20Rheda-Wiedenbr%C3%BCck%2C%20Germany&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/copyright ; read-only public.db (26.10.2026).]

### Recherche 04.10.2026

- [Recherche 04.10.2026: Erneute Firmen-/Namenssuche ergab ein widersprüchliches CompanyHouse-Profil, das die Berni Pfleging GmbH als aufgelöst ausweist; im Gegensatz dazu stehen die vorhandenen Registerhinweise zu HRB 6222 und die älteren, konsistenten Firmen-/Adress-Leads. Kein aktueller Betreiberauftritt oder Primärregisterbeleg zur Auflösung bzw. Fortführung nachgewiesen. Bestehenden Status `aktiv` und alle vorhandenen Felder nicht überschrieben; die leeren E-Mail-, Öffnungszeiten- und Website-Felder bleiben mangels belastbarer aktueller Betreiberbelege leer; Quelle(n): https://www.companyhouse.de/en/p/Berni-Pfleging-XUYqzWKw8nWOJYJPB8CRZkhFWms (widersprüchlicher Firmenprofil-Lead); https://www.northdata.de/ (frühere Register-Gegenprobe, siehe Recherche 02.10.2026); https://webvalid.de/ (frühere Register-Gegenprobe, siehe Recherche 02.10.2026); Abruf 04.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Rheda-Wiedenbrück

### Recherche 02.10.2026

- [Recherche 02.10.2026: Registerstark — Berni Pfleging GmbH, HRB 6222 (AG Gütersloh; webvalid.de + northdata.de), gegr. 25.01.1996, Gegenstand „An- und Verkauf von Eisen- und Stahlschrott"; Adresse Fürst-Bentheim-Str. 36, 33378 Rheda-Wiedenbrück (firmania.de + recyclingpool.de + city-advisor + bauenwohnen, konsistent); Tel. 05242/401369 (recyclingpool.de; telefonbuchsuche-Personeneintrag Burgweg 66 gleiche Nr. = Firmenbezug zweitbelegt, Burgweg-Adresse als Privat-/Altadresse in Historie). Fills: name (GmbH), street, postcode, phone, description; status → aktiv (Register + Konsistenz). Quelle(n): webvalid.de + northdata.de (Register, stark); recyclingpool.de + firmania.de (Adress-/Tel.-Konsistenz, Leads)]
