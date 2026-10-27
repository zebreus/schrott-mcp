---
slug: rp-binningen-esper-binningen
clear_fields: 'geo'
name: Esper Binningen
trader_type: sonstige
state: RP
city: Binningen
street: Hauptstr. 56
postcode: '56754'
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: ''
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Wide-Net PROSE-Cluster (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Esper Binningen

## Überblick

Die Betreiberidentität bleibt ungeklärt, die Anschrift ist jetzt aber sauber belegt: Das Feld `street` enthielt den Seed-Mischtext „Esper Binningen Binningen Hauptstr. 56" und wurde auf die reine Hausnummerzeile **Hauptstr. 56, 56754 Binningen** korrigiert. Drei unabhängige Verzeichniseinträge nennen dieselbe Anschrift; der 11880-Eintrag wurde am 26.10.2026 aktualisiert (Eintragsdaten 09.01.2026) und führt zusätzlich die Mobilnummer 0160 3872487, die auch in einer Kleinanzeige vom 16.08.2026 („Schrotthandel Esper") auftaucht. Verzeichnis-Leads sind nach der Quellenhierarchie keine Evidenz, deshalb bleiben Name, Telefon, E-Mail und Öffnungszeiten unverändert und der Status bleibt `pruefung`. Der Namenskonflikt (Frank und Michaela laut Schrottplatz.org, Raphael laut Kleinanzeige) ist offen.

**ANKAUF:** die Kleinanzeige nennt „Wir entsorgen für Sie: Metall, Schrott, Altmetall, Kupfer", aber keinen Kurs; „50 € VB" ist eine Anzeigenpreisangabe ohne erkennbare Material- oder Mengenbezug und damit keine belastbare Preisliste. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife; die 11880-Öffnungszeiten täglich 8 bis 20 Uhr sind Portalangaben und werden nicht übernommen.

## Timeline

### Recherche 27.10.2026: Adressfeld bereinigt, Anschrift dreifach belegt

- [Recherche 27.10.2026: Das corrupte Seed-Feld `street` („Esper Binningen Binningen Hauptstr. 56") wurde auf „Hauptstr. 56" korrigiert und `postcode` mit „56754" gefüllt. Beleglage: 11880-Eintrag „Schrotthandel Esper" (Detailseite direkt gelesen) mit Hauptstr. 56, 56754 Binningen, Mobil (0160) 3872487, Öffnungszeiten täglich 8-20 Uhr, „Geöffnet bis 20:00 Uhr", Eintragsdaten 09.01.2026, aktualisiert 26.10.2026; Schrottplatz.org-Eintrag „Schrott Esper Frank u. Michaela" (direkt gelesen) mit Hauptstr. 56, 56754 Binningen und Festnetz 02672 91 25 87; Kleinanzeigen-Anzeige „Schrott Schrotthandel Eisen Alteisen Metall Altmetall", 56754 Binningen, 16.08.2026, „Schrotthandel Esper", 50 € VB (Suchindex). Das sind Verzeichnisbelege und damit Leads, keine Betreiber-Primärquelle; phone, email, opening_hours bleiben deshalb leer und der Status bleibt `pruefung`. Quellen: https://www.11880.com/branchenbuch/binningen-eifel/060440092B113059782/schrotthandel-esper.html ; https://www.schrottplatz.org/binningen/schrott-esper-frank-u-michaela-aYEPQz.html ; https://www.kleinanzeigen.de/s-schrott-metall-handel/k0 (Suchindex).]
- [Recherche 27.10.2026: Namens- und Nummernkonflikt bleibt offen und wird nicht aufgelöst: Die Mobilnummer 0160 3872487 wird von 11880 und von der Kleinanzeige (dort als Angebot „Raphael Esper" mit schrotthandel.esper@web.de) genannt, das Festnetz 02672 91 25 87 von Schrottplatz.org unter dem Namen „Frank u. Michaela Esper"; der ältere 11880-Lead nannte ebenfalls 02672 912587. Zwei Nummern und zwei Personenkreise bei derselben Adresse sind mit Namensgleichheit nicht auflösbar; es wurde keine Nummer und keine E-Mail ins Frontmatter übernommen und nichts umbenannt. Quellen: wie Vorbullet; zusätzlich https://www.kleinanzeigen.de/s-anzeige/schrott-schrotthandel-alteisen-altmetall-metall-stahl/3543913199-298-17043 (Direktabruf HTTP 403, Suchindex-Snippet).]
- [Recherche 27.10.2026: **ANKAUF:** die Kleinanzeigen-Anzeige nennt „Wir entsorgen für Sie: Metall, Schrott, Altmetall, Kupfer"; die Angabe „50 € VB" ist ohne Material-, Mengen- oder Einheitenbezug und daher keine Preisliste, sie wird nicht als Ankaufspreis übernommen. **VERKAUF:** keine Liste gefunden. **GEBÜHREN:** keine Tarife; die 11880-Angabe täglich 08:00-20:00 Uhr ist eine Verzeichnisangabe ohne Betreiberbestätigung und bleibt außen vor. Offen bleibt die Gewerbe-Identität (Gewerbeamt/Kaisersesch) und die Frage, ob an Hauptstraße 56 eine öffentliche Annahme stattfindet; die 11880-Anzeige „Geöffnet bis 20:00 Uhr" ist kein Betriebsbeleg. Quelle: https://www.kleinanzeigen.de/s-schrott-metall-handel/k0 (Suchindex).]

### Owner-GEO-Prüfung 23.10.2026

- [Owner-GEO-Prüfung 23.10.2026: Read-only-Abgleich der Produktionszeile: gespeicherter Altpunkt 50.2064000, 7.2461000. Die aktuelle Recherche belegt für diesen Seed keinen adressgenauen, dem Betreiber und einer öffentlichen Annahmestelle zugeordneten Punkt; ein Ersatz wäre geraten. `clear_fields: 'geo'` entzieht das Alt-Paar beim regulären Seed; das bedeutet nur „keine kuratierte aktuelle Koordinate“, nicht Schließung oder Annahmeverweigerung. Keine Ersatzkoordinate geraten, Produktionsdatenbank nicht geschrieben. Quellen: read-only-Abgleich `public.db` vom 23.10.2026; Rechercheeintrag 23.10.2026 mit den dort verlinkten Quellen; `docs/trader-seed.md`.]


### Recherche 23.10.2026

- [Recherche 23.10.2026: Gemeindeseite direkt geprüft; sie bestätigt Binningen in Rheinland-Pfalz/Cochem-Zell und VG Kaisersesch, führt aber keinen für diesen Seed verifizierten Esper-Betrieb. DuckDuckGo-Suchergebnisse zeigten Kleinanzeigen-Anzeigen, deren Text „Raphael Esper“, Sitz Hauptstraße 56, 56754 Binningen, Mobilnummer 0160 3872487 und schrotthandel.esper@web.de nennt; beide Anzeigen waren beim Direktabruf wegen HTTP 403 nicht prüfbar. Das ist ein unbestätigter Anzeigen-/Suchindexlead, kein ausreichender Betreiberbeleg. Ältere Verzeichnisprofile schreiben „Frank“ bzw. „Frank und Michaela Esper“ und führen abweichend 02672 912587; keine gesicherte Identitäts-/Telefonkette. ANKAUF/VERKAUF/GEBÜHREN: keine numerische Preisliste oder belastbare Tarifangaben; keine Feldänderung. Hauptstraße 56 erst nach Betreiberbestätigung geocodieren. Quellen/Leads: https://www.binningen.de/ ; https://www.kleinanzeigen.de/s-anzeige/schrott-schrotthandel-alteisen-altmetall-metall-stahl/3543913199-298-17043 (Direktabruf 403; Suchindex-Snippet) ; https://www.kleinanzeigen.de/s-anzeige/schrott-schrotthandel-eisen-alteisen-metall-altmetall-/3486199121-298-17043 (Direktabruf 403; Suchindex-Snippet) ; https://www.11880.com/branchenbuch/binningen-eifel/060440092B113059782/schrotthandel-esper.html .]

### Recherche 14.10.2026 — Welle 4 / Shard B

- [Recherche 14.10.2026: [Offizielle Gemeinde Binningen](https://www.binningen.de/) tatsächlich gelesen: Rheinland-Pfalz/Landkreis Cochem-Zell, seit 01.07.2014 Verbandsgemeinde Kaisersesch. Damit korrekte kommunale Klärroute gefunden; keine Schweizer Gemeinde oder Hilzingen/BW als Standortersatz verwenden.; Quelle: https://www.binningen.de/]
- [Recherche 14.10.2026: Homepage enthält keinen konkreten Esper-Gewerbe-/Betreiberbeleg. Das ist keine vollständige Registersuchenegative und kein Schließungsbeweis. Historische Hauptstraße-56-Spur und malformierter Seed-Straßenwert werden nicht anhand bloßer Ortsgleichheit korrigiert.; Quelle: https://www.binningen.de/]
- [Recherche 14.10.2026: **Ankauf, Verkauf, Gebühren:** keine belastbare Betreiberquelle gefunden. Nächste Arbeit: Auskunft zur exakten Firmenidentität über zuständige Gewerbebehörde/Kaisersesch; keine Anfrage ausgelöst. Frontmatter unverändert, aktuelle Tätigkeit und Identität offen.; Quelle: https://www.binningen.de/]


### Importiert (Seed-Stand 2026-09-30)

- Hauptstr. 56
- urspr. Website-Angabe: keine
- Adresse: Binningen

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall – nur Aggregator-/Kleinanzeigen-Leads (Schrottplatz-Portale), keine verifizierte Betreiber-Website, kein Registerbeleg (Retry 01.10.2026 ohne neuen Beleg); street-Feld enthält Seed-Mischtext und wird nicht überschrieben; Quelle(n): keine belegfähige Quelle (nur Aggregator-/Kleinanzeigen-Leads, zählen nicht)]
- [Recherche 01.10.2026: Felder leer gelassen, Status pruefung unverändert; Quelle(n): siehe Vorbullet]
