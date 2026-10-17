---
slug: by-rothenburg-eisenmann
name: Eisenmann
trader_type: sonstige
state: BY
city: Rothenburg
street: ''
postcode: '91541'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: 'Neu: nur Verzeichnis-Beleg (lokaleschrottplatz.de, Ankauf unklar)'
provenance_ankauf_raw: unklar (Register-Prosa)
provenance_origin: prose
---

# Eisenmann

## Überblick

Neue konkrete Betreiberkette statt reiner Suchlücke: Wolfgang Eisenmann GmbH, Ansbach HRB 1114, wurde 2021 auf DERICHEBOURG Umwelt GmbH, Düsseldorf HRB 50882, verschmolzen. Die aktuelle Betreiber-Standortseite nennt in Rothenburg ob der Tauber **Am Igelsbach 9, 91541, 09861/3453, rothenburg@derichebourg.com**, Mo–Fr 07–16 mit Pausen 09–09:30 und 12–12:30. Primärinhalt war erst über die öffentliche Rubedo-Lese-API zugänglich; eine leere Angular-Seite ist kein verschwundener Standort.

Der Seed „Eisenmann“ ist weiterhin nur Nachname/PLZ; diese plausible Zuordnung muss vor Stammdatenmigration im Owner-Gate gegen den Originalimport geprüft werden. Die erloschene GmbH bedeutet keine Hofschließung: der Nachfolger führt den Standort aktuell. **Ankauf / Verkauf / Gebühren:** keine standortbezogenen Zahlenlisten im geprüften Umfang. **GEO:** Adresse als Nachfolgekandidat vorhanden, keine Koordinaten ohne Zuordnung und Punktquelle.

## Timeline

### Recherche 16.10.2026 — Welle 13:00 / Gruppe a

- [Recherche 16.10.2026: NorthData-Firmenprofil Wolfgang Eisenmann GmbH HRB 1114 direkt gelesen, Gegenstand Altmaterial-/Schrotthandel und Containerbetrieb. Verschmelzungspublikation 22.09.2021 einzeln gelesen: Vertrag/Beschluss 26.08.2021, übertragend an DERICHEBOURG Umwelt GmbH HRB 50882. Artikel-Suchauszug EUWID 24/2021 zur Übernahme 1. Juni nur ergänzende Pressespur, kein Ersatz des Registertexts. Anders als leere frühere Suggest-Suche jetzt konkret identifizierter Rechtsträger. Quellen: https://www.northdata.de/Wolfgang%20Eisenmann%20GmbH,%20Rothenburg%20o%C2%B7%20d%C2%B7%20Tauber/Amtsgericht%20Ansbach%20HRB%201114 ; https://www.northdata.de/?id=6804688854 ; Suchlead: https://www.euwid-recycling.de/news/wirtschaft/derichebourg-uebernimmt-fraenkischen-schrotthaendler .]
- [Recherche 16.10.2026: Offizielle Standort-, Impressums- und Zertifikat-Inhalte einzeln über GET der im öffentlichen Frontend verwendeten Rubedo-API gelesen, da Seitenabruf nur Angular-Hülle liefert. Standortinhalt (lastUpdateTime entspricht 28.09.2026 UTC) nennt Rothenburg Am Igelsbach 9/91541, 09861/3453, rothenburg@derichebourg.com, Leiter Patrick Baierlein, Mo–Fr 07–16, Pausen 09–09:30 und 12–12:30. Impressum nennt Düsseldorf HRB 50882. Zertifikatseite behauptet EFB für Rothenburg, Zertifikats-PDF nicht gelesen. Aktuelles ND-Profil des Nachfolgers und zweite Verschmelzungsmeldung beim ergänzenden Shellabruf HTTP429; vollständige aktuelle Registerkonkordanz nicht gewonnen. Deshalb trotz gut belegter Eisenmann→Derichebourg-Kette keine strukturierte Umbenennung/Anschriftenfüllung vor Abgleich mit kurzem Seed. Keine Schließung, keine numerischen Ankauf-/Verkauf-/Gebührenpreise in diesen Inhalten; kein GEO. Quellen: https://www.derichebourg-umwelt.de/de/einzelne/5c861b135804e9b1752a7100/standorte ; https://www.derichebourg-umwelt.de/api/v1/contents/5c861b135804e9b1752a7100?lang=de ; https://www.derichebourg-umwelt.de/api/v1/contents/5c87ac215804e99262fa0b31?lang=de ; https://www.derichebourg-umwelt.de/api/v1/contents/5c8681995804e9692dfa0b31?lang=de .]

### Recherche 14.10.2026 (18:00-Welle B)

- [Recherche 14.10.2026: Gezielt NorthData-Suggest Eisenmann Rothenburg neu geprüft: HTTP200/results leer. Anders als unscharfe Suchseitenprüfung 08.10. ein neuer Suchweg, aber weder vollständiger Registerauszug noch Negativbeweis für ein Kleingewerbe. PLZ 91541 bleibt Seedwert; kein ortsfremder Anlagenbauer als Betreiber übernommen. ANKAUF/VERKAUF/GEBÜHREN keine zuordenbaren Preise; GEO mangels Anschrift gesperrt. Owner: ursprünglichen Portal-/Importbeleg auf vollständigen Vornamen/Gewerbenamen zurückführen und bei Rothenburg ob der Tauber kommunal zuordnen; Quelle: https://www.northdata.de/suggest.json?query=Eisenmann+Rothenburg&countries=DE .]

### Recherche 08.10.2026

- [Recherche 08.10.2026: NorthData-Suche „Eisenmann Rothenburg“ liefert im ersten auslesbaren Block keine kongruente lokale Schrottidentität; unscharfe/paginierte Suche erlaubt keinen Existenz-Negativbeweis. Portal-Kandidat lokaleschrottplatz.de/eisenmann/ HTTP403, Brave-Prüfung 429; keine Betreiber-Website zugeordnet. Kein Frontmatter-Fill, keine ortsfremde Eisenmann-Gesellschaft übernommen. Owner: vollständigen Rechts-/Gewerbenamen und Ort innerhalb PLZ 91541 über Originalimport/Gewerbeauskunft klären; Ankauf/Verkauf/Gebühren bleiben unbelegt; Quelle(n): https://www.northdata.de/?query=Eisenmann%20Rothenburg ; https://lokaleschrottplatz.de/eisenmann/ (Abrufgrenze).]

### Importiert (Seed-Stand 2026-09-30)

- Registerfund ohne geprüfte Website (PLZ 91541)
