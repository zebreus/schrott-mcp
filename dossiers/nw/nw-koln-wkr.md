---
slug: nw-koln-wkr
name: WKR GmbH
trader_type: schrotthaendler
state: NW
city: Köln-Rodenkirchen
street: Kirschbaumweg 8
postcode: '50996'
clear_fields: 'geo'
phone: 0221 97060622
email: ''
opening_hours: 'Annahme: Mo-Fr 08:00-16:00; Sa geschlossen'
website: https://www.wkr-schrott.de/
website_status: aktiv
status: aktiv
description: 'Schrottplatz mit Metall-Schrotthandel und Abfallrecycling; Betreiber nennt Annahme für Unternehmen und Privatpersonen.'
dropoff_json: '{"allowed":true,"customer_types":["privat","gewerbe"],"conditions":"Betreiberseite nennt für die WKR-Niederlassung Köln-Rodenkirchen Schrottannahme Mo-Fr 08:00-16:00; Samstag geschlossen. Die Website öffnet beide WKR-Schrottplätze ausdrücklich für Unternehmen und Privatpersonen."}'
pickup_json: ''
provenance_section: Köln / Bonn / Leverkusen / Dormagen
provenance_ankauf_raw: ja
provenance_origin: table
---

# WKR GmbH

## Überblick

Dieses Dossier bildet die WKR-Niederlassung **Köln-Rodenkirchen** ab: Die Betreiber-Kontaktseite nennt Kirschbaumweg 8, 50996 Köln und 0221 97060622; dieselbe Telefonnummer steht bereits im read-only Produktionsbestand. Longerich (Hugo-Junkers-Str. 10a) ist eine getrennte WKR-Niederlassung und wird nicht in dasselbe Adressfeld gemischt. Die ebenfalls auf der Kontaktseite genannte Niederlassung Niehl gehört zur separaten MKR Rothenbücher GmbH. Der bisherige Produktionspunkt löst an der Longericher Straße 193 auf, nicht in Rodenkirchen; `clear_fields: 'geo'` entfernt ihn beim nächsten regulären Seed. Für den bestätigten Zweig ist keine präzise Ersatzkoordinate freigegeben.

## Timeline

### Owner-Nachkorrektur 26.10.2026

- [Owner-Nachkorrektur 26.10.2026: Die vorherige Owner-Auswahl Longerich wird anhand der vorhandenen Produktions-Telefonnummer 0221 97060622 korrigiert: Die Betreiber-Kontaktseite ordnet genau diese Nummer dem WKR-Standort Rodenkirchen (Kirschbaumweg 8, 50996) zu, während Longerich die andere Nummer 0221 97997814 hat. Die Frontmatter-Adresse bildet daher Rodenkirchen ab; die nachfolgende ältere Owner-Timeline bleibt als Recherchehistorie stehen, wird durch diese Nachkorrektur aber ausdrücklich supersediert. Die direkt gelesene WKR-Start- und Kontaktseite belegt an beiden Standorten Schrottannahme, nennt für Rodenkirchen Montag bis Freitag 08:00-16:00, Samstag geschlossen, und sagt ausdrücklich, dass beide WKR-Schrottplätze Unternehmen und Privatpersonen offenstehen. Niehl bleibt separate MKR. GEO 50.9807/6.9212 löst an Longericher Straße 193 auf und wird gelöscht; kein Ersatzpunkt. Quellen: https://www.wkr-schrott.de/kontakt/kontaktdaten/ ; https://www.wkr-schrott.de/ ; Read-only public.db-Abgleich vom 26.10.2026.]

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Die Betreiber-Kontaktseite direkt gelesen. Sie führt WKR in Longerich mit Hugo-Junkers-Str. 10a, 50739 Köln und 0221 97997814 sowie Rodenkirchen separat mit Kirschbaumweg 8, 50996 und 0221 97060622. Niehl wird als MKR Rothenbücher GmbH, Geestemünder Straße 34, separat ausgewiesen. Deshalb ist das kombinierte `street`-Feld auf Longerich begrenzt; keine WKR/MKR- oder Filialzusammenführung. NorthData HRB 63135 und das Impressum bestätigen den Rechtsträgerbezug. Read-only Produktionspunkt 50.9807/6.9212 löst zu Gebäude 193, Longericher Straße auf, nicht zur Niederlassung Hugo-Junkers-Str. 10a; `clear_fields: 'geo'`, kein Ersatzpunkt geraten. Quellen: https://www.wkr-schrott.de/kontakt/kontaktdaten/ ; https://www.wkr-schrott.de/kontakt/impressum/ ; https://www.northdata.de/WKR%20Wertstoff-%20und%20Kreislaufwirtschaft%20Rodenkirchen%20GmbH,%20K%C3%B6ln/HRB%2063135 ; https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=50.9807&lon=6.9212&zoom=18&addressdetails=1 .]

### Importiert (Seed-Stand 2026-09-30)

- 3 Schrottplätze (Rodenkirchen/Longerich/Niehl), alle Altmetalle
- Adressbeleg: wkr-schrott.de

### Tagespreise beobachtet (24.10.2026)

- [Recherche 24.10.2026: Die elf Zähler in der
  Preisliste <https://www.wkr-schrott.de/preise/> stehen im statischen HTML auf
  `0`; die verlinkte First-Party-Datei
  <https://www.wkr-schrott.de/wp-content/uploads/bb-plugin/cache/240-layout.js?ver=5ba8bcef84a0cd44215a69a7b64e903e>
  enthält die an dieselben Modul-IDs gebundenen Preise und animiert
  `.uabb-number-int` mit `settings.number`. Abruf 22:30 UTC; SHA-256 der
  JS-Antwort: `277542f8ca2eb7766a272abf118f9cb9756cefa3d522baae19f8fe24ec3e61d2`.
  Werte: Alu Geschirr 10% 1,20 €/kg; V2A 0,70 €/kg; V4A 1,40 €/kg; Zink
  1,50 €/kg; Mischschrott 120 €/t; Millberry 10,50 €/kg; Kupferschwer
  9,50 €/kg; Kupferkabel 40% ohne Stecker 3,80 €/kg; Kupferschälkabel 50%
  4,60 €/kg; Kupferlitzenkabel 50% 4,10 €/kg; Messing 5,70 €/kg. Die Seite
  sagt: Preise gelten ab 50 kg je Sorte, kleinere Mengen erhalten einen
  Preisabzug; sie sind freibleibend. Kein Veröffentlichungsdatum ausgewiesen.
  Die Einleitung nennt Longerich, Rodenkirchen und Niehl, die Überschrift des
  Zählerblocks dagegen nur Longerich und Rodenkirchen; Niehl-Zuordnung bleibt
  daher offen. Die WordPress-REST-Darstellung
  <https://www.wkr-schrott.de/wp-json/wp/v2/pages/240> enthält ebenfalls
  statische Nullen; ihr Änderungszeitpunkt ist kein Preisdatum.]
