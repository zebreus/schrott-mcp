---
slug: he-neuhof-schrottplatz-neuhof
name: Schrottplatz Neuhof
trader_type: schrottplatz
state: HE
city: Neuhof
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
provenance_section: Nachtrag Audit-Runde 2 (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Schrottplatz Neuhof

## Überblick

„Schrottplatz Neuhof“ ist bislang nur ein generischer Seed-/Verzeichnisname ohne Straße oder zugeordneten Rechtsträger. In Neuhof ist separat **Friedrich Görlich Schrott und Metalle GmbH** unter Vor der Harth, 36119 Neuhof, belegt; dazu existiert bereits das eigenständige Dossier `he-neuhof-gorlich-friedrich-schrott-u-metalle`. Die genaue Anschrift/Kontaktangabe wird durch dessen Betreiberseite und den bvse-Mitgliedereintrag gestützt. Ein generischer LokaleSchrottplatz-Eintrag „Schrottplatz Neuhof“ ohne Straße beweist keine Identität mit Görlich. Deshalb kein Merge und kein Feldtransfer; für den Seed selbst sind weder öffentlicher Hofbetrieb noch Ankauf oder Abholung belegt.

**Preise:** Ankauf nicht bestätigt; keine numerische Ankaufpreisliste gefunden. Verkaufspreise und Annahme-/Entsorgungsgebühren sind in den ausgewerteten Quellen nicht veröffentlicht. Status `pruefung`; Adress- und Betreiberklärung offen.

## Timeline

### Recherche 22.10.2026

- [Recherche 22.10.2026: Zweiseitige Dubletten-/Identitätsprüfung: bvse führt Friedrich Görlich Schrott und Metalle GmbH, Vor der Harth, 36119 Neuhof, Telefon 06655 2524; die Betreiberwebsite bestätigt diese Geschäftsidentität (vgl. eigenständiges Dossier `he-neuhof-gorlich-friedrich-schrott-u-metalle`). Daneben listet lokaleschrottplatz.de sowohl Görlich als auch generisch „Schrottplatz Neuhof“ ohne nähere Straße. Gleiches Portal/Ortsnähe begründen keine Identität oder Filiale; generische Seedzeile bleibt separat und ungeklärt. Keine Frontmatter-Übernahme oder Geokodierung. ANKAUF: für den Seed keine bestätigte Annahme oder Preisliste; VERKAUF: keine Preisangaben; GEBÜHREN: keine Tarife. Quellen: https://www.bvse.de/ueber-uns/mitglieder/1673-goerlich-schrott-und-metalle-gmbh.html?ml=1 ; https://goerlich-schrott.de/ ; https://lokaleschrottplatz.de/?city=alsfeld-hessen (Aggregatorlead, beide Neuhof-Einträge).]

### Recherche 07.10.2026

- [Recherche 07.10.2026: NorthData-Suche nach „Schrottplatz Neuhof Hessen“ ergab kein eindeutig zuordenbares Firmenprofil. Das ist kein Nachweis der Nichtexistenz, aber auch keine Bestätigung von Betreiber, Ort oder Betrieb. Die Seed-Bezeichnung bleibt unverifizierter Lead; keine Frontmatter-Felder ergänzt. ANKAUF: nicht bestätigt/keine numerische Liste; VERKAUF: keine Liste gefunden; GEBÜHREN: keine Angaben in den ausgewerteten Quellen. Offen: konkreter Ortsteil, Betreiber/Rechtsträger und aktuelle Annahmebedingungen; Quelle: https://www.northdata.de/?query=Schrottplatz%20Neuhof%20Hessen]

### Importiert (Seed-Stand 2026-09-30)

- Schrottplatz (Verzeichnis, Tel.)

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Amtliches Fachbetrieberegister geprüft; die nachgewiesene EfbV-Erkennung in Neuhof gehört eindeutig zur Friedrich Görlich Schrott + Metalle GmbH und nicht zum generischen Seed.** Live am 08.11.2026 im amtlichen Fachbetrieberegister (clientseitig gerenderte Vaadin-App, Ergebnisgrid vollständig durchscrollt) in beiden Routen abgefragt, jeweils mit dem Filter "Art des Betriebs = Egal" (is=1&e=1): Route "Entsorgungsfachbetriebe" mit den Namensvarianten "Schrottplatz Neuhof" (0 Treffer), "Görlich" (0 Treffer) und "Görlich" mit Umlaut (3 Treffer); Route "Altfahrzeugverwertung" mit "Schrottplatz Neuhof" (0 Treffer), "Görlich" (0 Treffer) und "Görlich" mit Umlaut (0 Treffer). Die drei Treffer sind Anlagenzeilen derselben Einheit: **Friedrich Görlich Schrott + Metalle GmbH**, Vor der Harth, 36119 Neuhof, Zertifikatsnummer **ZZFE00100068**, mit den Anlagen / 1 (Sammeln, Befördern, Handeln, Makeln), / 2 (Lagern) und / 3 (Behandeln). Der Anlagenbogen ZZFE0010006811.pdf wurde live aus dem Register geladen und mit `scripts/release/pdf_checkboxes.py` ausgewertet: Anlage 1 kreuzt 2.1 Sammeln, 2.2 Befördern, 2.7 Handeln (Kennung F72H000096) und 2.8 Makeln (F72M002421), kreuzt aber **nicht** 2.3 Lagern, 2.4 Behandeln, 2.5 Verwerten und 2.6 Beseitigen; Anlage 2 kreuzt 2.3 Lagern (F72RD00175) und 2.5 Verwerten; Anlage 3 kreuzt 2.4 Behandeln (F72RD00175) und 2.5 Verwerten. Die Standortabfragen um Neuhof (3 km und 10 km) liefern in der Route "Entsorgungsfachbetriebe" 0 bzw. 27 Anlagenzeilen (u. a. Schrimpf GmbH & Co. Basaltwerke KG Verwaltung Neuhof, Hubertusstraße 1, 36119 Neuhof, ZZHT00300205 / 1, 3,1 km) und in der Route "Altfahrzeugverwertung" 1 bzw. 2 Zeilen (Winfried Erb GmbH & Co. KG Autoersatzteile, In der Au 8-12, 36119 Neuhof-Dorfborn, ZALS00800042 / 1, 2,9 km; Dorschel GmbH, Böcklerstraße 29 a, 36041 Fulda, ZALS00800043 / 1, 7,8 km). **Reichweite der Suche:** geprüft wurden die Namensvarianten, die Standortradien 3 km und 10 km und beide Routen; die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist **kein** Nachweis, dass ein Betrieb keine Anerkennung besitzt. **Bewertung für diesen Seed:** der Seed ist ein generischer Verzeichnisname ohne Straße und ohne Betreiber; die nachgewiesene EfbV-Anerkennung in Neuhof gehört der Friedrich Görlich Schrott + Metalle GmbH (eigenes Dossier `he-neuhof-gorlich-friedrich-schrott-u-metalle`) und wird **nicht** auf den Seed übertragen. Das Feld `certifications` bleibt leer, ebenso `street`, `postcode` und `website`; `status` bleibt `pruefung`. ANKAUF: keine Kursliste; VERKAUF: keine Preisliste; GEBÜHREN: kein Tarif. Offen bleibt allein die Betreiberzuordnung des generischen Seeds.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=G%C3%B6rlich ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrottplatz%20Neuhof ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=G%C3%B6rlich ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Neuhof%2C%2036119%2C%20Deutschland%3B9.6002656%3B50.4837077%3B9.7414434%3B50.5735388%3B9.4590878%3B50.3938766&dst=10&is=1&e=1 (Abruf 08.11.2026).]
