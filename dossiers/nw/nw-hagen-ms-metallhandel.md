---
slug: nw-hagen-ms-metallhandel
name: MS Metallhandel e.K.
trader_type: metallhaendler
state: NW
city: Hagen
street: Werkzeugstr. 28
postcode: '58093'
phone: 02331 3620236
email: kontakt@ms-metallhandel-iserlohn.de
opening_hours: 'Mo-Sa 06:00-20:00'
website: https://www.ms-metallhandel-iserlohn.de/schrotthandel
website_status: aktiv
status: aktiv
clear_fields: 'dropoff_json,geo'
description: 'MS Metallhandel e.K. (Inh. Martina Sinnen), Hagen (Werkzeugstr. 28): Schrotthandel ausschliesslich fuer Industrie und Gewerbe - Ankauf von Kupfer, Aluminium, Messing, Zink, Edelstahl und Kabelschrott zu tagesaktuellen LME-basierten Preisen, eigene Abholung mit Behaeltern/Big Bags/Containern, oeffentliche LKW-Waage, mobile Metallanalyse vor Ort, kostenlose monatliche und jaehrliche Auswertungen. Kein Ankauf von Privatpersonen.'
dropoff_json: ''
pickup_json: '{"allowed": true, "customer_types": ["gewerbe"], "conditions": "Abholung nach Terminwunsch mit eigenem Fuhrpark; Container, Behaelter und Big Bags in verschiedenen Groessen; Abholung innerhalb von 3 Stunden moeglich (Betreiberseite, 29.10.2026)"}'
provenance_section: Dortmund / Hagen / Ennepetal
provenance_ankauf_raw: ja (Gewerbe)
provenance_origin: table
---

# MS Metallhandel e.K.

## Überblick

MS Metallhandel e.K., Inhaberin Martina Sinnen, Werkzeugstr. 28, 58093 Hagen (Register AG Hagen, HRA 6192, USt-IdNr. DE 302477465). Reines B2B-Schrotthandelsgeschäft: „Unser Angebot richtet sich ausschließlich an Unternehmen und Gewerbetreibende. Ein Ankauf von Privatpersonen erfolgt nicht." Ankauf von Kupfer, Aluminium, Messing, Zink, Edelstahl und Kabelschrott, Bewertung nach LME-Kursen, Abholung mit eigenem Fuhrpark, öffentliche LKW-Waage mit Wiegenachweis, mobile Metallanalyse vor Ort. Büro 02371 8324966, Lager 02331 3620236. **Zahlenpreise werden nicht veröffentlicht** - die Bewertung erfolgt nach aktuellen Börsenkursen, eine Preisliste existiert nicht.

## Timeline

### Owner-Audit 26.10.2026

- [Owner-Audit 26.10.2026: Der aktuelle Registerabdruck des AG Hagen, HRA 6192 (Abruf 26.10.2026, 10:41; letzter Registereintrag 22.03.2019), bestätigt „MS Metallhandel e.K. Inh. Martina Sinnen“, Geschäftsanschrift Werkzeugstraße 28, 58093 Hagen und Inhaberin Martina Sinnen. Das aktuelle Betreiber-Impressum stimmt damit überein; die aktuelle Schrotthandelsseite beschreibt den Ankauf ausschließlich für Industrie/Gewerbe, eigene Abholung und eine öffentliche LKW-Waage. Die Adresse ist damit stark bestätigt, aber kein Kundeneinfahrtspunkt: `clear_fields: 'geo'` bleibt angemessen, ohne Ersatzkoordinate. Die Website sagt nicht ausdrücklich, dass Kunden Schrott selbst anliefern dürfen; `dropoff_json.allowed=true` bleibt dafür ein offener, separat zu prüfender Claim (die LKW-Waage allein belegt keine Anlieferannahme). Keine Produktionsdaten abgefragt oder geändert. Quellen: https://www.handelsregister.de/rp_web/erweitertesuche/welcome.xhtml (AG Hagen, HRA 6192; aktueller Abdruck); https://www.ms-metallhandel-iserlohn.de/impressum/ ; https://www.ms-metallhandel-iserlohn.de/schrotthandel ; https://www.ms-metallhandel-iserlohn.de/service/ (Abruf 26.10.2026).]

### Owner-Korrektur 26.10.2026: nicht belegte Kundenanlieferung zurücknehmen

- [Owner-Korrektur 26.10.2026: Read-only `public.db` enthält für diesen Slug weiterhin `dropoff_json.allowed=true` für Gewerbekunden. Der direkte Recheck der aktuellen Betreiberseite belegt ausdrücklich B2B-Ankauf, eigene Abholung und eine öffentliche LKW-Waage, aber keine Selbstanlieferung von Kunden am Standort. Die Waage allein ist kein Annahmebeleg. `dropoff_json` bleibt deshalb leer und `clear_fields: 'dropoff_json,geo'` setzt den gespeicherten Claim beim nächsten regulären Seed auf unbekannt `{}` zurück; das behauptet weder `allowed:false` noch eine Schließung. Der falsche Hagen-GEO-Punkt wird weiterhin ohne Ersatz gelöscht. Keine Produktionsdaten geändert. Quellen: Read-only `public.db` (26.10.2026); https://www.ms-metallhandel-iserlohn.de/schrotthandel ; https://www.ms-metallhandel-iserlohn.de/service/ (Abruf 26.10.2026); `docs/trader-seed.md`.]

### Korrektur 24.10.2026: Gemeinsamen falschen Hagen-Punkt zurückziehen

- [Korrektur 24.10.2026: Die Produktionszeile `nw-hagen-ms-metallhandel` enthielt den mit mehreren anderen Hagen-Datensätzen geteilten Punkt `51.90933, 7.88825`. Die read-only-Rückwärtssuche löst ihn in Mehringen, Everswinkel, Kreis Warendorf, PLZ 48351 auf, nicht in Hagen. Die Seed-Adresse Werkzeugstr. 28, 58093 belegt diesen gespeicherten Punkt nicht; `clear_fields: 'geo'` zieht ihn zurück. Mangels bestätigtem Kunden-/Einfahrtspunkt wird keine Ersatzkoordinate gesetzt. Die Produktionsdatenbank blieb unverändert. Quelle: https://nominatim.openstreetmap.org/reverse?lat=51.90933&lon=7.88825&format=jsonv2&addressdetails=1 ; https://www.openstreetmap.org/way/81516036.]

### Importiert (Seed-Stand 2026-09-30)

- NE-Metalle/Edelstahl, LME-Kurse, LKW-Waage (Werkzeugstr. 28)

### Recherche 01.10.2026

- [Feedback-Triage 01.10.2026 (ID 2449): KORREKTUREN mit offener Beleglage — postcode '58093' (Impressum + mehrfach site-intern: Werkzeugstr. 28, 58093 Hagen), phone Lager 02331 3620236 (Büro 02371 8324966), hours 'Mo-Sa 06:00-20:00' (Vorteile-Liste, eine Seite). Impressum: MS Metallhandel e.K., Inhaberin Martina Sinnen, HRA 6192 AG Hagen (registerverankert, aber HRA/e.K. → formal keine Owner-Ausnahme; starke Betreiberquelle + Registerverweis). Koordinaten-Hinweis berechtigt: Domain 'iserlohn' irreführend — einziger belegter Standort ist Hagen (kein Iserlohn-Standort auf der Website) → Timeline-Vermerk 'Koordinaten neu geocodieren (Werkzeugstr. 28, 58093 Hagen)'. B2B-only ('für Industrie und Gewerbe') → dropoff_json gewerbe. E-Mail nur Cloudflare-cloaked → kein Fill. Quelle(n): https://www.ms-metallhandel-iserlohn.de/schrotthandel, https://www.ms-metallhandel-iserlohn.de/impressum/ (Abruf 01.10.2026).]

### Recherche 29.10.2026

- [Recherche 29.10.2026: Betreiber-Website erneut direkt gelesen (`https://www.ms-metallhandel-iserlohn.de/schrotthandel`, `/impressum/`, `/service/`, `/schrottverkauf/`, `/oeffnungszeiten/`). **Impressum woertlich**: „MS Metallhandel e.K., Firmeninhaber: Martina Sinnen, Werkzeugstr. 28, 58093 Hagen, Buero: Telefon 02371 8324966, Telefax 02371 8324964, Lager Telefon 02331 3620236, E-Mail: kontakt@MS-Metallhandel-Iserlohn.de, Eingetragen im Handelsregister, Registergericht: Hagen, Registernummer: H RA 6192, Umsatzsteuer-IdNr: DE 302477465, Berufshaftpflichtversicherung: Provinzial Versicherung Iserlohn". Die E-Mail steht in der Seite **Cloudflare-vercloakt** (`data-cfemail`), laesst sich aber aus dem Auslieferungs-HTML zu `kontakt@MS-Metallhandel-Iserlohn.de` dekodieren - der Wert ist damit direkt vom Betreiber ausgeliefert und wird eingetragen (die fruehere Notiz „E-Mail nur Cloudflare-cloaked → kein Fill" bleibt stehen, der Cloaking allein war kein Belegmangel, sondern nur eine Lesehuerde). **Annahme woertlich** (Schrotthandel-Seite): „Schrotthandel fuer Industrie und Gewerbe. Als professioneller Schrotthandel fuer Industrie und Gewerbe sind wir auf den Ankauf und die Verwertung von NE-Metallen und Edelstahl spezialisiert. Wir kaufen unter anderem Kupfer, Aluminium, Messing, Zink, Edelstahl und Kabelschrott zu fairen, tagesaktuellen Preisen an - transparent bewertet und nach den Kursen der London Metal Exchange (LME). ... **Unser Angebot richtet sich ausschliesslich an Unternehmen und Gewerbetreibende. Ein Ankauf von Privatpersonen erfolgt nicht.**" **Abholung woertlich**: „Die Abholung erfolgt nach Ihrem Terminwunsch und wird mit unserem eigenen Fuhrpark flexibel und zuverlaessig durchgefuehrt. Ob kleine Behaelter, Big Bags oder grosse Container"; Vorteilsliste: „Abholung innerhalb von 3 Stunden moeglich", „Behaelterwechsel innerhalb von 3 Stunden", „Oeffentliche LKW-Waage - Mit offiziellem Wiegenachweis vor Ort", „Mobile Metallanalyse vor Ort - Direkte Pruefung und Bewertung Ihrer Metalle", „Kostenlose monatliche Auswertung", „Kostenlose jaehrliche Auswertung", „Containerbereitstellung", „Behaelterbereitstellung", „Auftragsabschluss innerhalb von 48 Stunden". `pickup_json` ist damit erstmals belegt und gefuellt (Gewerbe, eigene Abholung); **`dropoff_json` bleibt bewusst leer** - eine Selbstanlieferung von Kunden wird nirgends genannt, die „oeffentliche LKW-Waage" ist eine Wiegestelle auf dem Betriebsgelonde fuer die abgeholten Mengen, kein Annahmepunkt. **Oeffnungszeiten woertlich** (Seite `/oeffnungszeiten/`): „Mit unseren flexiblen Einsatzzeiten von 6:00 bis 20:00 Uhr passen wir uns Ihrem Betriebsablauf an" und in der Vorteilsliste „Flexible Oeffnungszeiten - Montag bis Samstag von 6:00 bis 20:00 Uhr" - der Eintrag `Mo-Sa 06:00-20:00` ist damit bestaetigt. **ANKAUF**: „Kupfer, Aluminium, Messing, Zink, Edelstahl und Kabelschrott ... zu fairen, tagesaktuellen Preisen ... nach den Kursen der London Metal Exchange (LME)", zusaetzlich Produktionsabfaelle (Aluminium, Edelstahl, Kupfer, Messing, Kabel, Stanzabfaelle, Spaene, Profile) - **keine Zahlenwerte**: die Schrottverkauf-Seite erklaert nur, wie ein Preis entsteht („internationale Metallmaerkte, die Materialart, die Legierung, die Sortenreinheit und die Qualitaet"), es gibt **keine Preisliste und keine LME-Kurs-Tabelle** auf der Seite. **VERKAUF**: „Verwertung" und Verkauf aufbereiteter Sekundaerrohstoffe an die Industrie sind belegt, **ohne Verkaufspreisliste**. **GEBUEHREN**: keine Tarife, keine Mindestmengen, keine Anfahrts-/Sortierkosten ueber die beiden „kostenlosen Auswertungen" hinaus. **Offen**: keine Zahlenpreise, keine Mindestmengen, keine Gebuehren; die Domain „iserlohn" ist weiterhin nur ein Naehegebiet („Raum Iserlohn, Hagen, Menden und Umgebung"), **kein Standort in Iserlohn** - `clear_fields: 'dropoff_json,geo'` bleibt unveraendert bestehen (kein Kunden-Einfahrtspunkt belegt, kein GEO-Ersatz). Quelle(n): https://www.ms-metallhandel-iserlohn.de/schrotthandel ; https://www.ms-metallhandel-iserlohn.de/impressum/ ; https://www.ms-metallhandel-iserlohn.de/schrottverkauf ; https://www.ms-metallhandel-iserlohn.de/service/ ; https://www.ms-metallhandel-iserlohn.de/%C3%B6ffnungszeiten/ (Abruf 29.10.2026).]
