---
slug: mv-ribnitz-damgarten-acs-jager-schrotthandel-abbruch-containe
clear_fields: 'dropoff_json,geo'
name: ACS Jäger (Schrotthandel, Abbruch, Containerdienst)
trader_type: containerdienst
state: MV
city: Ribnitz-Damgarten
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: http://www.acs-jaeger.de
website_status: blockiert
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Betriebe (Tabelle)
provenance_ankauf_raw: ja
provenance_origin: table
---

# ACS Jäger (Schrotthandel, Abbruch, Containerdienst)

## Überblick

„ACS Jäger“ ist als Betreiberidentität nicht bestätigt. Die Website unter `acs-jaeger.de` war bei den dokumentierten Direktabrufen blockiert; das Impressum/der Seiteninhalt ließ sich nicht verifizieren. Branchenverzeichnisse nennen Bei den Borger Tannen 8, 18311 Ribnitz-Damgarten-Borg und „Andreas Jäger“, aber mit widersprüchlichen Telefonnummern (03821 708737 bzw. 0175 7847211). Das ist ein Recherche- und Adresslead, keine belastbare Zuordnung der Seed-Firma; website_status `blockiert` bleibt.

- **Annahme:** keine Betreiberbestätigung für öffentliche Schrottannahme, Abbruchannahme oder Containerstandort.
- **Ankauf:** Seed-Angabe „ja“ und Branchenbezeichnung sind keine bestätigte Vergütung; keine Kurse.
- **Verkauf:** keine Produkt-/Preisliste gefunden.
- **Gebühren:** keine belastbaren Abbruch-, Container- oder Entsorgungstarife.
- **GEO:** Bei den Borger Tannen 8 ist nur ein Verzeichniskandidat; keine Koordinaten gesetzt. Nicht mit „Allround Computer Service“ in Großkarolinenfeld bzw. `jaeger-acs.de` gleichsetzen.

## Timeline

### Owner-GEO-/Annahme-Korrektur 24.10.2026

- [Owner-GEO-Korrektur 24.10.2026: Die read-only Produktionszeile enthielt den Punkt 54.2422, 12.4567 und `dropoff_json={"allowed":true}`. Die aktuelle Recherche fand nur widersprüchliche Verzeichnis-Telefonangaben und eine blockierte Website; Betreiberidentität, heutige Betriebsstätte und öffentliche Annahme sind nicht bestätigt. Beide gespeicherten Zusagen werden deshalb mit `clear_fields: 'dropoff_json,geo'` zurückgenommen, nicht als Beweis für falschen Betrieb oder Annahmeverweigerung. Keine Kontaktdaten oder Ersatzkoordinate ergänzt; Status `pruefung` bleibt. Keine Produktionsdaten geändert. Quellen: read-only `public.db` (24.10.2026); die im Rechercheeintrag 24.10.2026 verlinkten Verzeichnis-/Websitequellen; `docs/trader-seed.md`.]

### Recherche 24.10.2026

- [Recherche 24.10.2026: Aktuelle Verzeichnis-Suchtreffer stützen weiterhin Andreas Jäger/Bei den Borger Tannen 8, 18311 Ribnitz-Damgarten-Borg, lösen die Telefonnummer aber nicht auf: Das Telefonbuch führt 03821 708737, das Stadtbranchenbuch „Andreas Jäger“ an derselben Anschrift mit 0175 7847211. Beides sind Aggregator-Leads, keine Betreiberbestätigung. `http://www.acs-jaeger.de` blieb blockiert (HTTP 403/Timeout); keine Umleitung oder Verbindung zur namensähnlichen `jaeger-acs.de` aus Großkarolinenfeld hergestellt. Wegen nicht verifizierter Betreiberidentität/aktueller Tätigkeit status `aktiv` → `pruefung`; das bedeutet keine Schließungsfeststellung. Öffentliche Annahme, Ankauf, Verkauf und Gebühren bleiben unbelegt; keine Kontaktfelder/Koordinaten ergänzt. Adresse nur Geokodierungskandidat. Quellen: https://www.dastelefonbuch.de/Branchen/Schrotthandel/Stralsund ; https://adresse.dastelefonbuch.de/Ribnitz-Damgarten/2-0115907281761-Container-ACS-J%C3%A4ger-Ribnitz-Damgarten-Bei-den-Borger-Tannen.html ; https://ribnitz-damgarten.stadtbranchenbuch.com/3369216.html ; http://www.acs-jaeger.de ; https://jaeger-acs.de/ .]

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall — Betreiber-Website http://www.acs-jaeger.de nicht erreichbar (https Timeout, http 403, 01.10.2026); website_status blockiert bleibt. Nur Aggregator-Leads (Gelbe Seiten, Das Örtliche, Cylex: „ACS Jäger / Andreas Jäger", Bei den Borger Tannen 8, 18311 Ribnitz-Damgarten-Borg, Tel. 03821/708737). Kein Register, kein Betreiber-Social. Adresse daher nicht frontmatter-fähig.; Quelle(n): Live-Abruf 01.10.2026 + Lead-Recherche (Aggregator-Leads, kein Beleg)]

### Importiert (Seed-Stand 2026-09-30)

- Schrott, Abbruch, Container
