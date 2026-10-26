---
slug: sh-eckernforde-tietjen-stefan
clear_fields: 'dropoff_json,geo,phone'
name: Tietjen Stefan
trader_type: schrotthaendler
state: SH
city: Eckernförde
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: ''
website_status: unbekannt
status: pruefung
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Schleswig-Holstein — Recherche (Stand 27.09.2026)
provenance_ankauf_raw: ja
provenance_origin: table
---

# Tietjen Stefan

## Überblick

Verzeichnis-Seed mit unbestätigten Kontakt-/Adresswerten Siemensstr. 12, 24340 Eckernförde, 0172 3721669. Das Örtliche führt genau diesen Treffer in einer Deutschland-Suche unter „Private Einträge“; das ist eine Verzeichnisklassifikation, weder Betreiber-/Registerbeleg noch Gegenbeweis zu einem Gewerbe. Die Domain stefan-tietjen.de gehört laut Impressum zu einer Person in Lilienthal; ihr externer Schrottpreislink ist keine Tietjen-Preisliste für Eckernförde und kein Umzugsnachweis. Nicht mit Barmstedter Tietjen oder Lilienthal gleichsetzen.

- **Ankauf:** Seed behauptet ja; heutige Vergütung unbestätigt, keine Händlerkurse.
- **Verkauf:** keine zugeordnete Liste.
- **Gebühren:** keine zugeordneten Tarife.
- **GEO:** Siemensstraße 12 erst nach Betriebsbestätigung geocodieren.

## Timeline

### Recherche 26.10.2026: Eintrag als privater Verzeichnistreffer markiert

- [Recherche 26.10.2026: Die Deutschland-Suche von Das Örtliche listet drei „Tietjen Stefan“-Treffer ausdrücklich unter „Private Einträge“; einer davon führt Siemensstr. 12, 24340 Eckernförde und 0172 3721669. Das bekräftigt, dass der Datensatz nicht als verifizierte Firmen-/Betreiberseite behandelt werden kann, beweist aber nicht, dass die Person kein Gewerbe betreibt oder der alte Firmenlead beendet ist. Der separate Domain-Impressumstreffer bleibt Stefan Tietjen in Lilienthal (Westerweder Straße 38, andere Nummer); keine Verbindung/Preisübertragung. Keine Frontmatter-Fills, `clear_fields` bleibt bestehen und keine GEO. **Ankauf:** keine Eckernförder Kurse; **Verkauf:** keine Liste; **Gebühren:** keine Tarife. Quellen: https://www.dasoertliche.de/Themen/Stefan-Tietjen.html ; https://www.stefan-tietjen.de/impressum.htm .]

### Owner-Adress-/GEO-/Annahme-Korrektur 24.10.2026

- [Owner-Korrektur 24.10.2026: Read-only-Produktion enthielt GEO 54.4685, 9.8382 und `dropoff_json={"allowed":true}`. Das Dossier belegt Siemensstr. 12, 24340 Eckernförde und 0172 3721669 nur über Branchenverzeichnisse; die ähnlich benannte Domain gehört laut Impressum einer Person in Lilienthal und ist nicht zugeordnet. Da aktuelle Händleridentität, Adresse und Kundenannahme unbestätigt bleiben, wurden Straße, PLZ und Telefon aus den strukturierten Seed-Feldern entfernt. `clear_fields: 'dropoff_json,geo,phone'` entzieht beim nächsten regulären Seed auch die gespeicherte Annahmezusage, den nicht zuordenbaren Punkt und das Telefon. Historische Leads bleiben dokumentiert; keine Schließung behauptet und keine Produktionsdaten geändert. Quellen: read-only `public.db` (24.10.2026); Recherche- und Quellenbelege vom 23.10.2026 oben; `docs/trader-seed.md`.]

### Recherche 23.10.2026: erneute Adress-/Kontaktprüfung

- [Recherche 23.10.2026: Das Örtliche führt Tietjen Stefan, Siemensstr. 12, 24340 Eckernförde und 0172 3721669; dieselben Daten stehen auf Gelbe Seiten. Das sind Portal-Leads, kein unabhängiger Betreiber-/Registerbeleg. Die Suche nach der exakten Adresse/Rufnummer ergab keine Betreiberwebsite oder aktuelle Geschäftsinformation; die Lilienthaler Domain bleibt ein Namens-/Preislink ohne Ortsverbindung. Ankauf: keine Zahlenkurse; Verkauf: keine Liste; Gebühren: keine Tarife. Keine Stammdatenänderung oder Geocodierung. Quellen/Leads: https://www.dasoertliche.de/Themen/Tietjen-Stefan-Eckernf%C3%B6rde-Siemensstr ; https://www.gelbeseiten.de/gsbiz/b8c972ce-123c-4d2e-85d1-eb40938c5332 ; https://www.stefan-tietjen.de/impressum.htm .]

### Namesake-/Preisprüfung 08.10.2026

- [Recherche 08.10.2026: Suchtreffer stefan-tietjen.de mit Schrottpreisverweis direkt gegengeprüft: Homepage Nützliches im Internet verweist auf externe schrottpreise-info.de, ausdrücklich regional/händlerabhängig; Impressum Stefan Tietjen,WesterwederStraße38,28865Lilienthal,01749458612. Andere Adresse/Telefon, keine belegte Verbindung zum Eckernförder Verzeichnis-Seed. Domain/Preise deshalb nicht übernehmen und keinen Umzug ableiten; vorhandene Seed-Adresse/Telefon bleiben ausdrücklich unbestätigt, status pruefung. Keine numerische Betreiber-Ankauf-/Verkaufspreisliste oder Gebühren gefunden. Quelle(n): https://www.stefan-tietjen.de/ ; https://www.stefan-tietjen.de/impressum.htm ; https://www.gelbeseiten.de/gsbiz/b8c972ce-123c-4d2e-85d1-eb40938c5332 (Suchindex-Lead)]

### Recherche 02.10.2026

- Beleglage: 1) Gelbe Seiten: Tietjen Stefan, Schrott, Siemensstr. 12, 24340 Eckernförde, Tel. 0172 3721669. 2) Kein zweiter unabhängiger Beleg: DuckDuckGo-/DasÖrtliche-Suche nach Siemensstr. 12 + Telefon ohne weiteren Treffer (Barmstedt-Tietjen gehört zu anderem Betrieb Tietjen Stefan/Tietjen Ursula Schrotthandel, nicht übernommen).
- Adresse/Telefon als Erstbefüllung aus 1 Quelle übernommen (Owner-Ausnahme Telefon), `status: pruefung` bleibt. Kein `## Überblick` angelegt.

### Importiert (Seed-Stand 2026-09-30)

- Kleiner Schrotthandel (Gelbe Seiten)

### Korrektur 30.09.2026

- [Korrektur 30.09.2026: status aktiv → pruefung — Existenz nur durch Verzeichnisbeleg, kein unabhaengiger Zweitbeleg (Feedback 1464/1465); Re-Pruefung per Folgewelle.]
