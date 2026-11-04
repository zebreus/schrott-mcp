---
slug: he-frankfurt-60489-vebeg
name: VEBEG GmbH
trader_type: sonstige
state: HE
city: Frankfurt am Main
street: 'Rödelheimer Bahnweg 23'
postcode: '60489'
phone: '+49 69 75897-0'
email: 'mail@vebeg.de'
opening_hours: 'Mo–Do 8–12 und 13–16 Uhr, Fr 8–12 und 13–14 Uhr'
website: https://www.vebeg.de
website_status: aktiv
status: aktiv
description: 'Bundeseigene Treuhandgesellschaft, die Überschussmaterial öffentlicher Stellen per Ausschreibung verkauft.'
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net (27.09.2026)
provenance_ankauf_raw: nein
provenance_origin: table
certifications: ''
---

# VEBEG GmbH

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- Surplus-VERKAUF — verkauft (kauft nicht!), bundeseigen (GRENZFALL — eher kein Ankauf → Audit: aufnehmen? eher nein)
- Adresse: Frankfurt 60489

### Recherche 01.10.2026

- [Recherche 01.10.2026: Adresse, Telefon, Mail, Beschreibung verifiziert und Frontmatter gefüllt (Owner-Ausnahme: Impressum Name+HRB+Ort: VEBEG GmbH, Rödelheimer Bahnweg 23, 60489 Frankfurt am Main, HRB B 8255 AG Frankfurt); website live abgerufen daher website_status aktiv; Quelle(n): https://www.vebeg.de/en/others/impressum.htm + https://www.vebeg.de/en/unternehmen/info.htm + https://www.vebeg.de]
- Bundeseigene Treuhandgesellschaft (gegr. 1951 durch Bundesfinanzministerium): verkauft Überschussmaterial öffentlicher Stellen per Tender (kein Ankauf, kein Schrottplatz) — GRENZFALL bleibt bestehen; city-Feld unverändert gelassen (Impressum bestätigt Frankfurt am Main)
- Keine Öffnungszeiten auf Betreiber-Website (Ausschreibungsportal), Feld bleibt leer

### Recherche 01.10.2026 (Feedback-Triage #3092)

- [Recherche 01.10.2026: Feedback #3092 berechtigt (Öffnungszeiten) — Korrektur der Vorgänger-Notiz: Geschäftszeiten stehen DOCH auf der Betreiber-Kontaktseite (Zentrale VEBEG GmbH, Rödelheimer Bahnweg 23, 60489 Frankfurt: Mo–Do 8–12 und 13–16 Uhr, Fr 8–12 und 13–14 Uhr, plus Berliner Büro Grellstr. 24) → opening_hours gefüllt per verifizierter Betreiber-Primärquelle (GmbH, HRB B 8255 AG Frankfurt, HR-kongruent per Northdata + service.bund.de — Owner-Ausnahme); Rest (Adresse/Telefon/Mail) unverändert korrekt; city bleibt bewusst unverändert; Quelle(n): https://www.vebeg.de/de/kontakt/index.htm + https://www.northdata.com (HRB 8255)]

### Korrektur 03.10.2026

- [Korrektur 03.10.2026 (Audit-Feedback #4519/#4521/#4524): Die offizielle Unternehmensdarstellung beschreibt VEBEG als Verwerter/Verkäufer von Überschussbeständen öffentlicher Stellen und nennt Verkäufe per Ausschreibung; die aktuelle Angebotsseite zeigt Verkaufslose, darunter die Kategorie Metalle/Schrott. Das belegt Verkauf, nicht Ankauf von Schrott aus der Öffentlichkeit; `provenance_ankauf_raw` daher von `ja` auf `nein` korrigiert. Den internen Portal-Kommentar „Grenzfaktor“ aus der nutzerorientierten description entfernt; Händler-Scope bleibt ein offener Grenzfall, ohne Annahme/Ankauf zu behaupten. Stadtfeld von `Frankfurt 60489` auf die im Betreiber-Impressum genannte Ortsform `Frankfurt am Main` normalisiert; PLZ bleibt separat `60489`, Slug unverändert. Firmenstatus `aktiv` bezieht sich auf die belegte aktive Gesellschaft, nicht auf Schrottankauf. Quelle(n): https://www.vebeg.de/en/unternehmen/info.htm ; https://www.vebeg.de/en/others/impressum.htm ; https://www.vebeg.de/]

### Recherche 02.11.2026

- [Recherche 02.11.2026: **Zertifikats-Recherche ohne Treffer.** Für `VEBEG GmbH` (Rödelheimer Bahnweg 23, 60489 Frankfurt am Main) wurde im amtlichen Fachbetrieberegister (GESA) weder ein Efb-Zertifikat noch ein AltfahrzeugV-Eintrag gefunden; `certifications` bleibt leer. Suchen und Befunde (live am 02.11.2026, clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt): (1) Namenssuche `…/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=VEBEG` -> **1 Treffer** (EfB), darunter `Vebego Industrial Cleaning Services GmbH`, `Konsumstr. 45, 42285 Wuppertal`, `ZZBT00300192 / 1` - **Namensvetter in Wuppertal**, nicht die Frankfurter VEBEG GmbH; (2) Namenssuche `…/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=VEBEG` -> **0 Treffer** (Altfahrzeug). Die Radiusabfrage mit 3 km um `Rödelheimer Bahnweg 23, 60489 Frankfurt am Main` (`…/Entsorgungsfachbetriebe/adr=Rödelheimer Bahnweg 23, 60489 Frankfurt am Main, Deutschland;…&dst=3&is=1&e=1`) liefert **3 Treffer**: `Stammsitz`, `Eschborner Landstraße 130 - 132, 60489 Frankfurt`, `ZZGE00100037 / 1, 2`, und `Green Construction GmbH`, `Frankfurter Str. 80-82, 65760 Eschborn`, `ZZFT00300218 / 1` - **kein Treffer auf `VEBEG` und keiner auf `Rödelheimer Bahnweg 23`**. Dieselbe 3-km-Abfrage in der Route `…/Altfahrzeugverwertung/adr=…&dst=3&is=1&o=5&e=1` liefert **0 Treffer** - **kein AltfahrzeugV-Vermerk**. Als Positivkontrolle liefert die 10-km-Efb-Abfrage um dieselbe Adresse **43 Treffer** (u. a. `Wayss & Freytag Ingenieurbau AG Umwelttechnik`, `Eschborner Landstraße 130 - 132, 60489 Frankfurt`, `ZZGE00100037 / 1, 2`, `Rhein Main Rohstoffe GmbH`, `Gutleutstraße 371, 60327 Frankfurt`, `ZZHT00400304 / 1, 2, 3`), die 10-km-Abfrage in der Altfahrzeugverwertung-Route **0 Treffer** - die Abfragetechnik funktioniert also.
  Reichweite: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; ein Register-Nichteintrag ist **kein** Nachweis, dass der Betrieb keine Anerkennung besitzt.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=VEBEG ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&o=5&e=1&n=VEBEG ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=R%C3%B6delheimer%20Bahnweg%2023%2C%2060489%20Frankfurt%20am%20Main%2C%20Deutschland%3B8.6071623%3B50.1253337%3B8.619162%3B50.137334%3B8.595162%3B50.113334&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=R%C3%B6delheimer%20Bahnweg%2023%2C%2060489%20Frankfurt%20am%20Main%2C%20Deutschland%3B8.6071623%3B50.1253337%3B8.619162%3B50.137334%3B8.595162%3B50.113334&dst=3&is=1&o=5&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=R%C3%B6delheimer%20Bahnweg%2023%2C%2060489%20Frankfurt%20am%20Main%2C%20Deutschland%3B8.6071623%3B50.1253337%3B8.619162%3B50.137334%3B8.595162%3B50.113334&dst=10&is=1&a=1&gl=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=R%C3%B6delheimer%20Bahnweg%2023%2C%2060489%20Frankfurt%20am%20Main%2C%20Deutschland%3B8.6071623%3B50.1253337%3B8.619162%3B50.137334%3B8.595162%3B50.113334&dst=10&is=1&o=5&e=1 (Abruf 02.11.2026).]

### Korrektur 04.11.2026

- [Korrektur 04.11.2026: **Nachgetragene Notiz vom 01.10.2026 (VEBEG, HRB-Kongruenz).**
  Diese Notiz stand im Dossier an dieser Stelle mit einem fehlerhaften `+- `-Praefix. Die
  Timeline-Referenz liest nur Bullets in Spalte 0, die Notiz war dadurch unsichtbar - sie
  fehlte im Seed und damit in der Produktionsdatenbank. Sie ist jetzt am Timeline-Ende in
  korrekter Form wiederholt; der Wortlaut ist unveraendert.]

- [Korrektur 01.10.2026 (Owner-Gate): HR-Kongruenz per Northdata nachgeholt (VEBEG GmbH, AG Frankfurt HRB 8255, Roedelheimer Bahnweg 23, 60489 Frankfurt) + service.bund.de (Behoerdenportal: gleiche Adresse/Tel/Mail) — Fills damit doppelbelegt; Quelle(n): https://www.northdata.com/VEBEG%20GmbH,%20Frankfurt%20a%C2%B7%20Main/Amtsgericht%20Frankfurt%20am%20Main%20HRB%208255 + https://www.service.bund.de/Content/DE/DEBehoerden/V/VEBEG/VEBEG-GmbH.html]
