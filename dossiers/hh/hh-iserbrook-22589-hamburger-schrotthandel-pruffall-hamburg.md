---
slug: hh-iserbrook-22589-hamburger-schrotthandel-pruffall-hamburg
name: Hamburger Schrotthandel
trader_type: schrotthaendler
state: HH
city: Iserbrook 22589
street: ''
postcode: ''
phone: ''
clear_fields: 'phone,website'
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Hamburger Schrotthandel→ PRÜFFALL: Hamburger Schrotthandel? Nein: "Hamburger Schrott"

## Überblick

Weiterhin ein **Prüffall**: Die neu direkt geöffnete Schrottplatz.org-Seite führt „Schrott Hamburger Schrott“ am Heerbrook 4, 22589, mit Tel. 0173 611 05 24 – passend zu den historischen Seed-Daten, aber nur als Portal-Lead. Sie belegt weder den Betreiber noch den aktuellen Betrieb; die frühere DNS-Prüfung vom 01.10.2026 dokumentierte `hamburger-schrott.de` als nicht auflösbar. Nicht mit dem anderswo ansässigen Anbieter hamburger-schrottverwertung.de zusammenführen.

**Preise (Ankauf/Verkauf/Gebühren):** weder der Portal-Lead noch eine verifizierte Betreiberquelle nennt Kurse oder Gebühren. **Offen:** Betreiber und Telefonnummer verifizieren; bis dahin keine Anlieferempfehlung, `status: pruefung` bleibt. Quellen- und Prüfverlauf siehe Timeline.

Unbelegte Telefon-/Domainzuordnung aus aktueller Frontmatter zurückgenommen; alte Werte in Timeline. Erneuter rekursiver DNS-Befund NXDOMAIN bestätigt nur fehlende Domainauflösung, nicht Identität oder Schließung. Ohne zugeordnete Betreiberwebsite website_status unbekannt.

## Timeline

- [Owner-Korrektur 10.10.2026: Explizite Seed-Direktive clear_fields für phone und website ergänzt. Leere Dossierwerte allein erhalten bestehende DB-Enrichmentwerte; nach geprüfter Veröffentlichung soll der normale Seed diese unbelegten Kontaktzuordnungen tatsächlich entfernen, nicht nur aus dem Markdown zurücknehmen. Keine direkte Produktionsmutation, keine GEO-Löschung, keine Schließungsbehauptung. Historische Werte und Gegenbelege unten erhalten. Grundlage: docs/trader-seed.md; research/checkin-20261010-waveD-nordwest-owner-gate.md.]

### Recherche 09.10.2026 / Korrektur 10.10.2026

- [Recherche 10.10.2026: Wave-D-Systemresolver gaierror -2 bedeutet nur fehlende Auflösung dieser Umgebung, kein ausgelesener autoritativer RCODE. Unabhängige Google-DNS-A-Abfrage für hamburger-schrott.de ergibt Status 3/NXDOMAIN, negative SOA de., Kommentar Response from 194.246.96.1, AD false. Rekursiver RCODE-Befund, keine eigene direkte autoritative/DNSSEC-Verifikation. Domain nicht auflösbar, kein Betriebsschließungsbeweis oder Nachweis der Betreiberzuordnung. Quelle(n): https://dns.google/resolve?name=hamburger-schrott.de&type=A ; geprüfter Host hamburger-schrott.de (Wave-D-Systemresolver 09.10.2026)]
- [Korrektur 10.10.2026: Unbelegte Legacy phone 0173 6110524 und website https://hamburger-schrott.de aus aktueller Frontmatter geleert; zugeordneter website_status vorher tot auf unbekannt, weil keine verifizierte Betreiberwebsite zugeordnet ist. Alle alten Werte hier erhalten; Domain-NXDOMAIN als technisches Arbeitswissen bleibt gültig. Heerbrook 4/22589/Telefon nur Seed-/Portal-Lead, keine zwei zulässigen Identitäts-/Kontaktquellen. status pruefung, Ort/Slug/GEO/Typ erhalten; keine Anliefer-/Preis-/Gebührenzusage, keine Schließung aus Nichtfund. Namensähnliche Hamburger Anbieter nicht übernehmen; Quelle(n): https://www.schrottplatz.org/hamburg/schrott-hamburger-schrott-aYEPV3.html ; README.md, Source hierarchy; obiger DNS-Befund und ältere Timeline]

### Recherche 07.10.2026

- [Recherche 07.10.2026: Direktabruf der Portal-Detailseite „Schrott Hamburger Schrott“ ergab Heerbrook 4, 22589 Hamburg-Iserbrook und Tel. 0173 6 11 05 24 – identisch zu den historischen Seed-Werten. Das ist ein weiterer Aggregator-Lead, kein Betreiber-/Registerbeleg; keine Telefonnummer/Adresse in Frontmatter übertragen und keine Identität oder Aktivität bestätigt. Profil enthält keine Ankaufskurse, Verkaufspreise oder Gebühren. Klärfall Betreiber/Telefon bleibt offen; Quelle(n): https://www.schrottplatz.org/hamburg/schrott-hamburger-schrott-aYEPV3.html; README.md, Source hierarchy.]

### Importiert (Seed-Stand 2026-09-30)

- Schrott — nur Verzeichnis + Tel 0173 6110524 (PRÜFFALL)
- Adresse: Iserbrook 22589, Heerbrook 4

### Recherche 01.10.2026

- [Recherche 01.10.2026: kein Betreiber auffindbar; hinterlegte Domain tot; namensähnliche Domain gehört anderem Anbieter; Quelle(n): Dossier-PRÜFFALL, https://hamburger-schrottverwertung.de/ (fremder Anbieter)]
- hamburger-schrottverwertung.de ist ein anderer Anbieter (kostenlose Schrottabholung, keine Iserbrook-Adresse), NICHT mit Dossier verknüpfen.
- Klärfall Folgewelle: Tel. 0173 6110524 gegenrecherchieren oder Dossier schließen.

### Recherche 01.10.2026 (Feedback-Triage #3086)

- [Recherche 01.10.2026: Berechtigt — Domain-Tod per DNS bestätigt (NXDOMAIN für hamburger-schrott.de; website_status tot bestand bereits zurecht). Prüf-Notizen aus name-Feld in Timeline überführt (name bereinigt). Namensvetter-Check: kein realer „Hamburger Schrotthandel" in Iserbrook auffindbar (Treffer: hamburger-schrottverwertung.de = fremder Anbieter, PMK/SMH/Schmidt = andere Firmen) — Phantom-Verdacht, KEINE Adress-/Kontakt-Fills, status pruefung bleibt, Klärfall (Tel. 0173 6110524 weiter offen); Quelle(n): DNS-NXDOMAIN + Websuche 01.10.2026]
