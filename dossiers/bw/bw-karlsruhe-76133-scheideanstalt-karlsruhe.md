---
slug: bw-karlsruhe-76133-scheideanstalt-karlsruhe
name: Scheideanstalt Karlsruhe
trader_type: sonstige
state: BW
city: Karlsruhe
street: Karlstr. 25
postcode: '76133'
phone: 0721.98 19 36 62
email: 'info@scheideanstaltka.de'
opening_hours: ''
website: https://scheideanstaltka.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net (27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Scheideanstalt Karlsruhe

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 06.10.2026 (Feedback #4767)

- [Recherche 06.10.2026: Der Betreiberauftritt bleibt widersprüchlich: Impressum nennt Mo–Fr 9:00–18:00 und Sa 9:30–14:00; Startseite nennt Mo–Fr 9:00–17:45 und Sa 9:00–13:45; Anfahrt nennt Mo–Fr 9:00–17:45 sowie Sa 9:00–13:00 im Sommer, 15:00 im Winter und 16:00 zur Weihnachtszeit. Die Startseite und Anfahrt stimmen werktags überein, aber nicht vollständig samstags; das Impressum weicht an beiden Tagen ab. Der Zeitstempel der Preistabelle auf der Startseite (06.10.2026) datiert nicht erkennbar den Öffnungszeitenblock, und die Saisonangabe der Anfahrt ist nicht datiert. Da alle drei Seiten derselben Betreiber-Domain angehören und keine belastbare Vorrangregel/aktuelle Saison ableitbar ist, bleibt `opening_hours` leer. Offen: Welche Zeiten gelten aktuell, und wann genau gelten die saisonalen Samstagszeiten? Quelle(n): https://scheideanstaltka.de/impressum-2/; https://scheideanstaltka.de/; https://scheideanstaltka.de/anfahrt/]
- [Quellenabgleich 06.10.2026 (Feedback #4767): Auch die Telefonnummern sind nicht einheitlich priorisiert: Impressum nennt 0721 98 19 36 62, die Startseite im Kopf 0721 151 9733; die Anfahrt führt beide als Kontaktalternativen auf. `phone` bleibt bei der Impressumsnummer, die auch auf der Anfahrt bestätigt wird; welche Nummer der Betreiber als Hauptanschluss priorisiert, ist unklar. Quelle(n): https://scheideanstaltka.de/; https://scheideanstaltka.de/impressum-2/; https://scheideanstaltka.de/anfahrt/]

### Produktionsabgleich 06.10.2026

- [Produktionsabgleich 06.10.2026: Obwohl `opening_hours` seit der Dossierkorrektur vom 05.10. leer ist, enthielt `public.db` noch den alten Impressumswert mit Konflikthinweis. Die Seed-Logik bewahrt gespeicherte Werte, wenn das Dossierfeld leer ist; daher wurde der widersprüchliche Produktionswert nach Backup und mit Slug-/Altwert-Guard gezielt geleert. Die uneinheitlichen Betreiberangaben bleiben ungelöst; Öffnungszeiten werden nicht mehr als aktuell behauptet. Keine weitere Feld- oder Geokoordinatenänderung.]

### Korrektur 04.10.2026 (Feedback #4767)

- [Korrektur 04.10.2026: Der Widerspruch der Öffnungszeiten ist bestätigt: Das Impressum nennt Mo-Fr 9:00-18:00 und Sa 9:30-14:00; die Anfahrtsseite weist abweichende Werktags- und saisonale Samstagszeiten aus. Da beide Angaben vom selben Betreiber stammen und kein eindeutiger Vorrang belegt ist, bleibt das nichtleere opening_hours-Feld unverändert; Konflikt bleibt offen; Quelle(n): https://scheideanstaltka.de/impressum; https://scheideanstaltka.de/anfahrt]

### Korrektur 05.10.2026 (Feedback #4767)

- [Korrektur 05.10.2026: opening_hours geleert, da Betreiber-Impressum, Startseite und Anfahrtsseite unterschiedliche Werktags-/Samstagszeiten nennen und kein Vorrang belegt ist. Die ältere Entscheidung, den nichtleeren Freitextwert beizubehalten, beruhte auf keiner dokumentierten Repository-Regel; hier ist ein leeres Feld ehrlicher als ein möglicherweise falscher Zeitplan. Status und übrige Frontmatter bleiben unverändert. Quelle(n): https://scheideanstaltka.de/impressum-2/; https://scheideanstaltka.de/; https://scheideanstaltka.de/anfahrt/]

### Recherche 01.10.2026

- [Feedback-Triage 01.10.2026 (ID 3034): berechtigt — city „Karlsruhe 76133" → „Karlsruhe" (PLZ separat in postcode); E-Mail info@scheideanstaltka.de aus verifiziertem Betreiber-Impressum (ScheideanstaltKa.GmbH, HRB 721844, Karlstr. 25, 76133 Karlsruhe) übernommen; Öffnungszeiten (Mo-Fr 9–18, Sa 9:30–14) bereits korrekt; Quelle: https://scheideanstaltka.de/impressum]

### Importiert (Seed-Stand 2026-09-30)

- Edelmetalle/Zahngold — Laden-Barankauf, Zahngold gelb 70€/g → HANDLER-KANDIDAT
- Adresse: Karlsruhe 76133, Karlstr. 25
- Adressbeleg: https://scheideanstaltka.de, https://scheideanstaltka.de/impressum
