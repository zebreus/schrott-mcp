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
+- [Korrektur 01.10.2026 (Owner-Gate): HR-Kongruenz per Northdata nachgeholt (VEBEG GmbH, AG Frankfurt HRB 8255, Roedelheimer Bahnweg 23, 60489 Frankfurt) + service.bund.de (Behoerdenportal: gleiche Adresse/Tel/Mail) — Fills damit doppelbelegt; Quelle(n): https://www.northdata.com/VEBEG%20GmbH,%20Frankfurt%20a%C2%B7%20Main/Amtsgericht%20Frankfurt%20am%20Main%20HRB%208255 + https://www.service.bund.de/Content/DE/DEBehoerden/V/VEBEG/VEBEG-GmbH.html]
- Bundeseigene Treuhandgesellschaft (gegr. 1951 durch Bundesfinanzministerium): verkauft Überschussmaterial öffentlicher Stellen per Tender (kein Ankauf, kein Schrottplatz) — GRENZFALL bleibt bestehen; city-Feld unverändert gelassen (Impressum bestätigt Frankfurt am Main)
- Keine Öffnungszeiten auf Betreiber-Website (Ausschreibungsportal), Feld bleibt leer

### Recherche 01.10.2026 (Feedback-Triage #3092)

- [Recherche 01.10.2026: Feedback #3092 berechtigt (Öffnungszeiten) — Korrektur der Vorgänger-Notiz: Geschäftszeiten stehen DOCH auf der Betreiber-Kontaktseite (Zentrale VEBEG GmbH, Rödelheimer Bahnweg 23, 60489 Frankfurt: Mo–Do 8–12 und 13–16 Uhr, Fr 8–12 und 13–14 Uhr, plus Berliner Büro Grellstr. 24) → opening_hours gefüllt per verifizierter Betreiber-Primärquelle (GmbH, HRB B 8255 AG Frankfurt, HR-kongruent per Northdata + service.bund.de — Owner-Ausnahme); Rest (Adresse/Telefon/Mail) unverändert korrekt; city bleibt bewusst unverändert; Quelle(n): https://www.vebeg.de/de/kontakt/index.htm + https://www.northdata.com (HRB 8255)]

### Korrektur 03.10.2026

- [Korrektur 03.10.2026 (Audit-Feedback #4519/#4521/#4524): Die offizielle Unternehmensdarstellung beschreibt VEBEG als Verwerter/Verkäufer von Überschussbeständen öffentlicher Stellen und nennt Verkäufe per Ausschreibung; die aktuelle Angebotsseite zeigt Verkaufslose, darunter die Kategorie Metalle/Schrott. Das belegt Verkauf, nicht Ankauf von Schrott aus der Öffentlichkeit; `provenance_ankauf_raw` daher von `ja` auf `nein` korrigiert. Den internen Portal-Kommentar „Grenzfaktor“ aus der nutzerorientierten description entfernt; Händler-Scope bleibt ein offener Grenzfall, ohne Annahme/Ankauf zu behaupten. Stadtfeld von `Frankfurt 60489` auf die im Betreiber-Impressum genannte Ortsform `Frankfurt am Main` normalisiert; PLZ bleibt separat `60489`, Slug unverändert. Firmenstatus `aktiv` bezieht sich auf die belegte aktive Gesellschaft, nicht auf Schrottankauf. Quelle(n): https://www.vebeg.de/en/unternehmen/info.htm ; https://www.vebeg.de/en/others/impressum.htm ; https://www.vebeg.de/]
