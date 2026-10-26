---
slug: he-wiesbaden-65183-collecta-rohstoff
name: Collecta Rohstoff GmbH
trader_type: metallhaendler
state: HE
city: Wiesbaden 65183
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
clear_fields: 'geo'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 4b – Kandidaten-Sweep (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# COLLECTA Rohstoff GmbH & Co. KG

## Überblick

Der aktuelle Betreiberauftritt und die offiziellen Wiesbadener Registereinträge belegen die **COLLECTA Rohstoff GmbH & Co. KG**, HRA 4165, an Senefelderstraße 3A, 65205 Wiesbaden-Nordenstadt sowie die B2B-Altpapiertätigkeit. Der offizielle Registerauszug zeigt auch, dass die frühere COLLECTA Rohstoff GmbH (HRB 4980) 1999 durch Verschmelzung in die bereits bestehende KG aufgenommen wurde. Die zweite Quelle NorthData bestätigt beide Rechtsträgerendpunkte, gibt die Verschmelzung selbst aber nicht wieder. Da die README für eine Namens-/Verschmelzungskette eine zweite unabhängige Kettenquelle verlangt, bleibt die Zuordnung dieser aktuellen KG zur bestehenden Seed-Zeile `Collecta Rohstoff GmbH` noch nicht für den Seed freigegeben. Name, Metalltyp, Ort, Anschrift und Annahme aus der aktuellen KG werden daher bis zur unabhängigen Kettenbestätigung nicht in die Altzeile übernommen.

**Feldgrenze:** Kontakt, Site, Beschreibung und Services bleiben leer. Die bisherigen Seedfelder (Name „Collecta Rohstoff GmbH“, Typ `metallhaendler`, Stadt „Wiesbaden 65183“) bleiben bis zur Identitätsfreigabe bestehen; `status: pruefung`. `COLLECTA Europe GmbH` (Kunststoff/andere Mail) ist ein anderer Rechtsträger und wird nicht beigemischt.

**Preise/GEO:** keine numerischen Ankaufskurse, öffentlichen Verkaufspreise oder Gebühren. Büro-/Geschäftssitz, keine belegte öffentliche Anlieferung oder lokale Abholleistung. Keine GEO-Freigabe; leere Servicefelder bedeuten unbekannt/Keep, nicht `allowed:false`.

## Timeline

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Read-only Produktion enthält für diese Zeile weiterhin `Collecta Rohstoff GmbH`, `metallhaendler`, Stadt `Wiesbaden 65183` und GEO 50.0778462/8.2453077. Der Punkt löst auf Friedrich-Ebert-Allee, 65185, nicht am aktuellen KG-Geschäftssitz Senefelderstraße 3A, 65205; zudem ist dort keine öffentliche Annahmestelle belegt. `clear_fields: 'geo'` entfernt den falschen Punkt, ohne Geschäftssitzkoordinate zu raten. Der frisch recherchierte amtliche Registerauszug (HRB 4980 und HRA 4165) belegt die Verschmelzung 1999; NorthData bestätigt die Endpunkte, aber nicht unabhängig den Verschmelzungsvorgang. Nach README fehlt damit weiterhin die unabhängige zweite Quelle für die Namens-/Rechtsträgerkette. Deshalb werden die zwischenzeitlich auf die Altzeile übertragenen KG-Firma, Typ `wertstoffhaendler` und Adresse/PLZ zurückgenommen und die bisherigen Seedwerte wiederhergestellt; `status: pruefung` bleibt. Keine Annahme-, Preis- oder Schließungsbehauptung. Quellen: https://www.handelsregister.de/rp_web/normalesuche/welcome.xhtml (AG Wiesbaden, HRB 4980/HRA 4165; amtliche Auszüge, 26.10.2026) ; https://www.northdata.de/COLLECTA%20Rohstoff%20GmbH,%20Wiesbaden/HRB%204980 ; https://www.northdata.de/COLLECTA%20Rohstoff%20GmbH%20%26%20Co.%20KG,%20Wiesbaden/HRA%204165 ; https://www.collecta-rohstoff.de/impressum/ ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.0778461538462&lon=8.24530769230769&zoom=18&addressdetails=1 ; README.md, Betreiberkettenregel.]

### Recherche 26.10.2026

- [Recherche 26.10.2026: Betreiber-Impressum und Unternehmensseite erneut direkt gelesen. Das Impressum nennt COLLECTA Rohstoff GmbH & Co. KG, Senefelderstraße 3A, 65205 Wiesbaden, HRA 4165, Komplementärin COLLECTA Verwaltungs GmbH (HRB 9660), Telefon 06122 5337-0 und info@collecta-rohstoff.de; die Tätigkeitsseite beschreibt B2B-Altpapierhandel und Versorgung von Papierfabriken. Das bestätigt die dortige Selbstdarstellung, aber keine öffentliche Metallannahme oder lokale Abholung. Das Endabnahme-Gate vom 17.10. zu Telefon/Website/Status/Description bleibt maßgeblich; keine Felder oder Services ergänzt. ANKAUF: Altpapier-B2B beschrieben, keine Zahlenkurse; VERKAUF: keine Papier-Verkaufspreise; GEBÜHREN: keine Tarife. Senefelderstraße ist Geschäftssitz, nicht belegte öffentliche Annahmestelle; keine GEO. Quellen: https://www.collecta-rohstoff.de/impressum/ ; https://www.collecta-rohstoff.de/collecta-rohstoff/ ; https://www.northdata.de/COLLECTA%20Rohstoff%20GmbH%20&%20Co%C2%B7%20KG,%20Wiesbaden/HRA%204165 .]

### Endabnahme 17.10.2026 (Kontaktfills zurückgestellt)

- [Korrektur 17.10.2026: Die folgende frühere Teilfreigabe wird eingeschränkt: neue Telefon-/Website-/Website-Status-Fills bis zum nachvollziehbaren unabhängigen Feldabgleich zurückgestellt, im Seed weiterhin leer. KG/Name/H1, Wertstoffhandel und Büroanschrift bleiben im durch Betreiber/NorthData/historisches Zertifikat belegten Umfang korrigiert; keine öffentliche Annahme und keine heutige EFB-Zertifizierung. Creditreform-Suchindex ist eine weitere Spur, kein vollständig gelesener Auszug. Keine neuen Liveabrufe oder Clears in dieser Disposition; alte Timeline bleibt erhalten. Quelle: research/checkin-20261017-2130-wave13-b-owner-final.md; https://www.collecta-rohstoff.de/impressum/; https://firmeneintrag.creditreform.de/65205/6250080496/COLLECTA_ROHSTOFF_GMBH_CO_KG]

### Korrektur 17.10.2026 (Identität und Feldgates)

- [Korrektur 17.10.2026: KG/HRA 4165, Komplementärin HRB 9660 und Senefelderstraße 3A/65205 sind durch Betreiber/NorthData/historisches bvse-Zertifikat Nr. 10515 gestützt; Identität/Adresse und wertstoffhaendler freigegeben, H1 angeglichen. Alte Seedbezeichnung Collecta Rohstoff GmbH war ungenau; keine rechtliche Umwandlungskette bewiesen. Büro/B2B-Altpapier statt öffentlicher Metallhof; COLLECTA Europe GmbH HRB 9659 nicht beimischen. Creditreform-Suchindex stützt Telefon 06122 5337-0/Domain/Adresse, kein Vollabruf oder pauschaler Mail-/Servicebeleg. email/status/description gezielt auf HEAD leer/pruefung/leer gehalten; neue positive Mail/Aktivität/Description bis Zweit-/Aktualitätsbeleg aufgeschoben, keine volle Betreiber-Ausnahme. Zertifikat gültig nur bis 31.05.2026, keine heutige EFB-Zertifizierung/kein Schließungsschluss; doppelt verlinktes PDF nicht doppelt zählen. Servicefelder leer bedeuten Keep/unbekannt, kein allowed:false/Clear. Keine GEO-Freigabe. Quelle: https://www.collecta-rohstoff.de/impressum/; https://www.collecta-rohstoff.de/collecta-rohstoff/; https://www.northdata.com/COLLECTA%20Rohstoff%20GmbH%20&%20Co%C2%B7%20KG,%20Wiesbaden/HRA%204165; https://www.collecta-rohstoff.de/wp-content/uploads/2025/05/EfB-Zertifikat2025_COLLECTA-Rohstoff-GmbHCoKG.pdf; https://firmeneintrag.creditreform.de/65205/6250080496/COLLECTA_ROHSTOFF_GMBH_CO_KG; research/checkin-20261017-1700-wave13-b-review.md; Ownerdisposition 17.10.2026]

### Importiert (Seed-Stand 2026-09-30)

- Metallhandel (GS)
- Adresse: Wiesbaden 65183

### Recherche 13.10.2026

- [Recherche 13.10.2026: Betreiber-Impressum, Unternehmensseite und Ansprechpartnerseite direkt geprüft. Impressum nennt COLLECTA Rohstoff GmbH & Co. KG, Senefelderstraße 3A, 65205 Wiesbaden, Telefon +49 6122 5337-0, info@collecta-rohstoff.de, HRA 4165 sowie COLLECTA Verwaltungs GmbH HRB 9660, Registergericht AG Wiesbaden; NorthData-Profil ist kongruent. Website erklärt den Ankauf von Sekundärrohstoff Altpapier bei Entsorgern und die Versorgung von Papierfabriken. Frontmatter deshalb auf Rechtsträger, Wertstoffhandel, Standort, Kontaktdaten und aktiven Betreiberauftritt aktualisiert; kein Beleg für öffentliche Annahme/Anlieferung oder standortbezogene Abholung, daher dropoff/pickup leer. ANKAUF: Altpapier B2B belegt, aber keine Preiszahlen. VERKAUF: Papierqualitäten/Versorgung der Industrie beschrieben, keine Verkaufspreise. GEBÜHREN: keine Liste. GEO: Senefelderstraße ist Geschäftssitz, keine bestätigte Annahmestelle; keinen Publikums-/Hofpunkt annehmen, Standortart geocodierseitig klären. Quelle: https://www.collecta-rohstoff.de/impressum/; https://www.collecta-rohstoff.de/collecta-rohstoff/; https://www.collecta-rohstoff.de/ansprechpartnerinnen/; https://www.northdata.de/COLLECTA%20Rohstoff%20GmbH%20&%20Co%C2%B7%20KG,%20Wiesbaden/HRA%204165]
