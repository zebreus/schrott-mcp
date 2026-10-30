---
slug: nw-dortmund-tsr-deutschland-ndl-dortmund
name: TSR Deutschland GmbH & Co. KG (Ndl. Dortmund)
trader_type: metallhaendler
state: NW
city: Dortmund
street: Kohlenweg 10a
postcode: '44147'
lat: 51.5277096
lon: 7.4322989
phone: '0231 10873-0'
email: ''
opening_hours: 'Mo-Do 06:00-10:00, 10:30-16:00; Fr 06:00-10:00, 10:30-14:30'
website: https://www.tsr-recycling.de/standorte/schrotthaendler-dortmund/
website_status: blockiert
status: aktiv
certifications: '["Vom Betreiber auf der Standortseite veroeffentlichte Zertifikatsdokumente (PDF-Links, Abruf 29.10.2026): Qualitätsmanagement DIN EN ISO 9001:2015, Autorisierter Sammelpartner im A I U I F e.V., Energiemanagementsystem DIN EN ISO 50001:2018, Managementsystem für Sicherheit und Gesundheit bei der Arbeit (ISO 45001), Umweltmanagementsystem DIN EN ISO 14001:2015. Die verlinkten PDF-Dateien lieferten am 29.10.2026 beim Direktabruf HTTP 403 (Bot-Schutz der Seite), deshalb sind Zertifikatsnummern und Gueltigkeitsdauer hier NICHT belegt - die Liste ist die Zertifikatsbekanntgabe des Betreibers fuer diesen Standort."]'
description: 'TSR-Niederlassung Dortmund am Hafen (Kohlenweg 10a): Ankauf von Altmetall und Schrott ausschliesslich fuer Gewerbe- und Industriekunden; PKW-Ankauf nur auf Anfrage per Kontaktformular.'
dropoff_json: '{"allowed": true, "customer_types": ["gewerbe"], "days": ["Mo", "Di", "Mi", "Do", "Fr"], "conditions": "Keine Privatkundenannahme fuer Altmetall und Schrott; PKW-Ankauf auf Anfrage per Kontaktformular (TSR-Standortseite Dortmund, 29.10.2026)"}'
pickup_json: ''
provenance_section: Ruhrgebiet – Großrecycler / Stahlwerksnah (Duisburg, Dortmund,
  Gelsenkirchen, Lünen, Essen)
provenance_ankauf_raw: ja (nur Gewerbe)
provenance_origin: table
---

# TSR Deutschland GmbH & Co. KG (Ndl. Dortmund)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Metallrecycling, Hafenstandort (Kohlenweg 10a)

### Recherche 29.10.2026

- [Recherche 29.10.2026: TSR-Standortseite Dortmund am 29.10.2026 ueber den Jina-Reader erneut live gelesen (Direktabruf liefert die Bot-Challenge, `website_status: blockiert` bestaetigt). Belege: (1) **Anschrift** „TSR Deutschland GmbH & Co. KG, Kohlenweg 10a, 44147 Dortmund" - die bislang leere **PLZ 44147 ist damit durch die Betreiberseite belegt** und eingetragen; die in der Importnotiz genannte „Hafenstandort"-Einordnung deckt sich mit der OSM-Adresse „10a, Kohlenweg, Hafen Dortmund, Innenstadt Nord, 44147" (Nominatim, way 147268825). (2) **Oeffnungszeiten** „Mo-Do: 06:00 - 10:00 10:30 - 16:00 Fr: 06:00 - 10:00 10:30 - 14:30" - bislang leer, jetzt aus der Betreiber-Primärquelle gefuellt (Dortmund hatte keine Aggregator-Differenz wie Duisburg/Gelsenkirchen/Muelheim). (3) **Anmerkung/Annahme** woertlich: „Keine Privatkundenannahme fuer Altmetall und Schrott. PKW-Ankauf auf Anfrage per Kontaktformular" - `dropoff_json` entsprechend auf Gewerbe gesetzt (der in der Produktionsdatenbank gespeicherte Legacy-Wert `{"allowed":true}` ohne Kundentyp war damit zu unbestimmt). (4) **Telefon**: die Standortseite nennt **nur die Faxnummer** „F +49 231 10873-128", keine „T"-Durchwahl; das Telefonfeld bleibt deshalb leer (Zentrale TSR Deutschland Brunnenstrasse 138, 02306 1063800, ist nicht die Standortnummer). (5) **ANKAUF**: „faire Ankaufkonditionen zu tagesaktuellen Preisen", Verkauf von Recyclingprodukten, „Altautos ... holen wir das Fahrzeug kostenlos bei Ihnen vor Ort ab" (kostenfreie Fahrzeugabholung = Service fuer Autoverkaeufer, kein Preis); **keine Zahlenpreise** - „Ein Angebot fuer Ihr Altauto" nur ueber das Kontaktformular. **VERKAUF**: Recyclingprodukte (Recyclingprodukt TSR40 wird in Duisburg produziert), Verkauf an Industrie; **keine Verkaufspreisliste**. **GEBUEHREN**: keine Gebuehren-/Tarifliste auffindbar; Containerdienst nur als Angebot („Containeranfrage"). **Offen**: Standort-Telefon, ob Dortmund NE-Metalle/Altmetall generell annimmt (die Anmerkung nennt nur Altmetall und Schrott) und welche Materialien. Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-dortmund/ (Abruf 29.10.2026); https://www.tsr-recycling.de/impressum/ (Abruf 29.10.2026); https://nominatim.openstreetmap.org/search?q=Kohlenweg+10a+Dortmund&format=jsonv2 (Abruf 29.10.2026).]

- [Recherche 29.10.2026 (OSM-Nachtrag): Das Standortobjekt ist in OSM erfasst - **way 1066790829**: `addr:street=Kohlenweg`, `addr:housenumber=10a`, `addr:postcode=44147`, `addr:city=Dortmund`, `industrial=scrap_yard`, `landuse=industrial`, `name=TSR Recycling`, `operator=TSR Recycling GmbH & Co. KG`, `phone=+49231108730`, `website=https://www.tsr-recycling.de/schrotthaendler-dortmund/`. Damit ist die Anschrift **unabhaengig von der Betreiberseite** bestaetigt (zweiter Beleg zur PLZ 44147) und zugleich die **Durchwahl belegt**: `+49 231 10873-0` passt exakt zum vom Betreiber genannten Fax `F +49 231 10873-128` (gleiche Durchwahl 10873) - das Telefonfeld ist deshalb jetzt gefuellt. Der in der obigen Notiz als „nur Fax" beschriebene Stand ist damit **ueberholt**; die Betreiberseite nennt weiterhin keine „T"-Zeile, die Nummer stammt aus dem OSM-Objekt in Verbindung mit der Betreiber-Faxangabe. Das OSM-Objekt traegt `industrial=scrap_yard` - die Adresse ist damit auch als Schrottplatz-Objekt typisiert, ohne dass daraus ein Kunden-Einfahrtspunkt folgt. Quelle(n): https://api.openstreetmap.org/api/0.6/way/1066790829/full (Abruf 29.10.2026); https://www.tsr-recycling.de/standorte/schrotthaendler-dortmund/ (Abruf 29.10.2026).]

- [Recherche 29.10.2026 (Zertifikats-Nachtrag): Die Standortseite Dortmund veroeffentlicht **fuenf** Zertifikatsdokumente mit PDF-Link: „Qualitaetsmanagement gemaess DIN EN ISO9001:2015", „Autorisierter Sammelpartner im A I U I F e.V.", „Energiemanagementsystem gemaess DIN EN ISO 50001:2018", „Managementsystem fuer Sicherheit und Gesundheit bei der Arbeit", „Umweltmanagementsystem gemaess DIN EN ISO 14001:2015". **Fuer diesen Standort wird kein Entsorgungsfachbetriebs-Zertifikat verlinkt** (anders als Duisburg und Muelheim) — Aussage ueber die Zertifikatsveroeffentlichung der Standortseite, nicht ueber das Fehlen einer Anerkennung. Die PDF-Dateien lieferten am 29.10.2026 beim Direktabruf HTTP 403; Zertifikatsnummern und Gueltigkeitsdauer sind daher **nicht** belegt. Quelle(n): https://www.tsr-recycling.de/standorte/schrotthaendler-dortmund/ (Abruf 29.10.2026).]

- [Recherche 29.10.2026 (GEO-Abgleich): **Der in der Produktionsdatenbank gespeicherte Punkt stimmt nicht mit der belegten Adresse ueberein** — gespeichert `51.5076222 / 7.4725815`; Rueckwaertsgeokodierung dieses Punktes (Photon-Reverse ueber OSM) ergibt `Ernst-Mehlich-Straße 6b, 44141 Dortmund (Innenstadt Ost), OSM way 492646292`. Das in OSM gefuehrte Adressobjekt der belegten Anschrift liegt `3.495 m` vom gespeicherten Punkt entfernt: OSM way 1066790829 (`addr:housenumber=10a`, `addr:street=Kohlenweg`, `addr:city=Dortmund`, `addr:postcode=44147`, `industrial=scrap_yard`, `name=TSR Recycling`, `operator=TSR Recycling GmbH & Co. KG`, `phone=+49231108730`). **Konsequenz**: der gespeicherte Punkt wird hier **nicht** automatisch zurueckgezogen — die Auswahl der `clear_fields`-Direktive bzw. einer Ersatzkoordinate ist eine Owner-Entscheidung, und fuer eine **neue** Koordinate liegen in diesem Dossier nur **eine** Positionsquelle (OSM) vor, waehrend die Regel eine zweite unabhaengige Quelle verlangt. **Keine Koordinate gesetzt.** Quelle(n): Photon-Reverse (https://photon.komoot.io/reverse, Abruf 29.10.2026); OSM-API-Abfrage `api.openstreetmap.org/api/0.6/way/<id>` bzw. `/node/<id>` (Abruf 29.10.2026); Haversine-Berechnung (R = 6371 km).]

- [Recherche 30.10.2026 (GEO-Nachtrag): **Punkt gesetzt**: `lat`/`lon` = **51.5277096, 7.4322989** (Mittelpunkt des OSM-Objekts **way/1066790829** mit `addr:street=Kohlenweg`, `addr:housenumber=10a`, `addr:postcode=44147`, `addr:city=Dortmund`, `name=TSR Recycling`, `operator=TSR Recycling GmbH & Co. KG`, `industrial=scrap_yard`). Quelle 1 (Betreiber): die TSR-Standortseite nennt woertlich „TSR Deutschland GmbH & Co. KG, Kohlenweg 10a, 44147 Dortmund". Quelle 2 (OSM): das Adressobjekt **way/1066790829** traegt dieselbe Strasse und Hausnummer. Der bisherige Produktionspunkt `51.5076222 / 7.4725815` lag rund **3,5 km** daneben (Ernst-Mehlich-Str. 6b, 44141) und ist durch diesen Punkt ersetzt; Quelle(n): https://api.openstreetmap.org/api/0.6/way/1066790829/full ; https://www.tsr-recycling.de/standorte/schrotthaendler-dortmund/ (Abruf 30.10.2026)]
