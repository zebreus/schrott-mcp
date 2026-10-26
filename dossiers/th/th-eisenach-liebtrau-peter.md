---
slug: th-eisenach-liebtrau-peter
clear_fields: geo
name: Liebtrau Peter
trader_type: schrotthaendler
state: TH
city: Eisenach
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
provenance_section: Schrottankauf Thüringen (TH) — Recherche
provenance_ankauf_raw: Ankauf unklar
provenance_origin: table
---

# Liebtrau Peter

## Überblick

Das Telefonbuch führt „Liebtrau Peter Schrotthandel“ an Amsdorfstr. 24, 99817 Eisenach mit 03691 841988. Ein zweites, ebenfalls nicht vom Betreiber stammendes Containerdienst-Portal führt unter demselben Namen und derselben Adresse eine abweichende Nummer 03691 8893175. Das erhöht die Zahl der Verzeichnis-Spuren, ersetzt aber weder Betreiber-/Registerbestätigung noch eine aktuelle Annahmeerklärung. Amsdorfstr. 24 bleibt Recherche- und möglicher Geokodierungskandidat; Straße, PLZ und Telefon sind nicht in die strukturierten Felder übernommen.

- **Ankauf:** Branchenlabel „Schrotthandel“ ist kein Beleg für aktuell vergüteten Ankauf; keine Materialien oder Kurse bestätigt.
- **Verkauf:** keine Teile-/Materialliste.
- **Gebühren:** keine Abhol-, Annahme- oder Entsorgungsgebühren.
- **Kundenannahme:** öffentliche Annahmestelle oder nur Geschäfts-/Kontaktadresse ungeklärt.
- **GEO:** Amsdorfstr. 24, 99817 Eisenach ist nur ein Geokodierungskandidat. Die vom Telefonbuch angezeigte Kartenposition wurde nicht als Händlerkoordinate übernommen.

**Preise:** Keine verifizierten Ankaufskurse, Verkaufspreise oder Liebtrau-spezifischen Containergebühren gefunden. Der generische Containerdienst-Text auf 123Entsorgung sagt nur, dass die Kosten allgemein von Behältergröße/Transportweg und Angebot bzw. Gewichtsabrechnung abhängen; das ist keine Liebtrau-Preisliste.

## Timeline

### Owner-GEO-Prüfung 26.10.2026

- [Owner-GEO-Prüfung 26.10.2026: Die aktuelle Nominatim-Suche für die aus einem Telefonbuch stammende Kandidatenadresse Amsdorfstraße 24, 99817 Eisenach liefert OSM Way 318280150 bei 50.9879749, 10.3101383; OSM klassifiziert das Objekt als Wohngebäude (`apartments`). Das bestätigt die Geokodierung der Adresse in Eisenach, aber weder Liebtraus Anwesenheit an diesem Gebäude noch eine öffentliche Annahmestelle. Da die Adresse weiterhin nur Verzeichnis-Evidenz hat, bleiben `street`, `lat` und `lon` strukturiert leer und `clear_fields: geo` bleibt gesetzt. In der read-only Produktionsdatenbank steht weiterhin der falsche Punkt 50.41535, 8.41595; keine direkte Produktionsänderung. Quelle: https://nominatim.openstreetmap.org/search?q=Amsdorfstraße%2024%2C%2099817%20Eisenach&format=jsonv2&addressdetails=1&limit=5 ; OSM-Daten © OpenStreetMap-Mitwirkende, ODbL 1.0; https://www.dastelefonbuch.de/Details/Eisenach/Schrott-Peter-Liebtrau-Eisenach-Amsdorfstr.html ; read-only `public.db` (26.10.2026).]
- [Owner-Gate-Korrektur 26.10.2026: Der im Eintrag vom 25.10. genannte `verify-mkm-release.py` ist ein Post-Rollout-Verifier, keine Voraussetzung der Pre-Deploy-Freigabe. Die separate Freigabe für den heutigen exakten Release-Input einschließlich Owner-/erforderlichem unabhängigem Review und privater Upgradeprobe bleibt offen. Der falsche Produktionspunkt wird daher weder direkt geändert noch durch eine unbelegte Ersatzkoordinate ersetzt. Nach grüner Pre-Deploy-Freigabe den freigegebenen Seed regulär deployen und anschließend die Entfernung des GEO-Punkts read-only verifizieren.]

### Owner-Produktionsprüfung 25.10.2026

- [Owner-Produktionsprüfung 25.10.2026: Read-only-Abfrage von `public.db` bestätigt, dass der falsche Bestands-GEO-Punkt 50.41535, 8.41595 weiterhin gespeichert ist. Eine erneute Nominatim-Rückwärtssuche löst ihn nach Weilmünster, Landkreis Limburg-Weilburg, Hessen auf, nicht nach Eisenach. Das Dossier enthält weiterhin `clear_fields: geo` und keine Ersatzkoordinaten; die lokalen Seed-Tests bestehen. Die Produktionsdaten sind damit ausdrücklich noch nicht korrigiert: Der installierte Server stammt vom 17.10.2026 und der MKM-Release-Verifier bleibt rot, daher weder Deployment noch direkter Produktionsdatenbank-Schreibzugriff. Nach grünem Release-Gate neu bauen/deployen und den gelöschten Punkt anschließend read-only verifizieren. Quelle: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.41535&lon=8.41595&zoom=18&addressdetails=1 ; read-only `public.db` (25.10.2026); `docs/trader-seed.md`.]

### Recherche 25.10.2026

- [Recherche 25.10.2026: 123Entsorgung-Seite zu Containerdiensten in Eisenach direkt geprüft. Der Eintrag „Liebtrau Peter Schrott- und Containerservice“ nennt Amsdorfstr. 24, 99817 Eisenach und 03691 8893175; Das Telefonbuch nennt für den gleichnamigen Eintrag an derselben Adresse 03691 841988. Beide Quellen sind Verzeichnisse, keine Betreiberbestätigung; Nummernkonflikt bleibt offen. Die allgemeine Seite erläutert Kosten nach Größe/Transportweg sowie mögliche Pauschal- oder Gewichtsabrechnung und unverbindliche Angebote, nennt aber keine Liebtrau-spezifischen Beträge. Kein bestätigter Schrottankauf, Materialverkauf oder Tarif. Adresse bleibt Geokodierungskandidat, keine GEO-Füllung; `clear_fields: geo` aus der Owner-Korrektur bleibt maßgeblich. Quellen: https://www.123entsorgung.de/containerdienst/eisenach/ ; https://www.dastelefonbuch.de/Details/Eisenach/Schrott-Peter-Liebtrau-Eisenach-Amsdorfstr.html .]

### Owner-GEO-/Adress-Korrektur 24.10.2026

- [Owner-GEO-Korrektur 24.10.2026: Der read-only Produktionspunkt 50.41535, 8.41595 reverse-geocodiert nach Weilmünster, Hessen, weit entfernt von Eisenach. Die Amsdorfstr. 24 stammt allein aus einem Telefonbuch-Lead ohne Betreiber-/Register-/Kommunalbeleg und wurde daher ebenso aus `street` zurückgenommen; der historische Kandidat bleibt unten dokumentiert. `clear_fields: geo` entfernt den falschen Altpunkt. Kein Ersatzpunkt, keine Aussage zu Betrieb oder öffentlicher Annahme und keine Produktionsänderung. Quellen: https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.41535&lon=8.41595&zoom=18&addressdetails=1 ; https://www.dastelefonbuch.de/Details/Eisenach/Schrott-Peter-Liebtrau-Eisenach-Amsdorfstr.html ; read-only `public.db` (24.10.2026); `docs/trader-seed.md`.]

### Recherche 24.10.2026

- [Recherche 24.10.2026: Das-Telefonbuch-Detail/Suchtreffer erneut geprüft: „Liebtrau Peter Schrotthandel“, Amsdorfstr. 24, 99817 Eisenach, 03691 841988; Aktualisierungsdatum des Eintrags 22.10.2026. Der Anbieterhinweis erklärt, dass Einträge aus Verlags-, Dritt- oder Nutzermeldungen stammen können; keine Betreiberkontrolle, Website oder Öffnungszeiten sind ausgewiesen. Die dort dargestellte Kartenposition ist daher kein Beleg für den Händlerpunkt. Keine Kontakt-Fills, keine Aussage zu öffentlicher Annahme oder Betrieb; Ankauf, Verkauf und Gebühren unbestätigt. Adresse bleibt Geokodierungskandidat ohne Koordinaten. Quelle (Verzeichnis-Lead): https://www.dastelefonbuch.de/Details/Eisenach/Schrott-Peter-Liebtrau-Eisenach-Amsdorfstr.html .]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (allg.)
