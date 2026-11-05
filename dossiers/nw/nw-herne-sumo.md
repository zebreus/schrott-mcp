---
slug: nw-herne-sumo
name: Sumo
trader_type: mobil
state: NW
city: Herne
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
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Sumo

## Überblick

Der generische Seed-Eintrag „Sumo“ in Herne bleibt ohne Betreiberbeleg. Das amtliche Fachbetrieberegister (Auszug 11.08.2026) kennt in Herne acht anerkannte Betriebe, aber keinen namensgleichen; die Efb-Anerkennung ist freiwillig, das Fehlen ist ein Hinweis ohne Beweiswert. Der einzige belegte Ortsbezug ist der Verzeichnislead „Schrotthandel Sumo, Bielefelder Str. 106, 44625 Herne, 01578 8135010“ (11880), und die Suche nach einem Betreiberauftritt bleibt ohne Treffer.

**ANKAUF / VERKAUF / GEBÜHREN:** keine belegten Werte; „Sumo“ ist als Name zu generisch (Sport, Gastronomie, Ringer-Verein), und aus der Namensgleichheit wird keine Zuordnung abgeleitet.

## Timeline

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Herne

### Recherche 01.10.2026

- [Recherche 01.10.2026: Klärfall, kein Frontmatter-Fill — Name zu generisch (Sumo: Sport, Ringer-Verein, Gastro-Treffer, kein Schrottbezug); keine Betreiberquelle, keine Adresse, kein HR-Eintrag; Status bleibt pruefung; Quelle(n): Websuche ohne belegfähigen Treffer]

### Recherche 03.10.2026

- [Recherche 03.10.2026: Nur website_status auf unbekannt gesetzt; das bedeutet nicht, dass sicher keine Website existiert, sondern dass keine dem Dossier-Subjekt belastbar zuordenbare Betreiberseite gefunden wurde. 11880 listet „Schrotthandel Sumo“ mit Bielefelder Str. 106, 44625 Herne und 01578 8135010, ohne Betreibername, Website, E-Mail oder Öffnungszeiten; die Seite ist ein Aggregator-Lead (aktualisiert 25.09.2026), kein unabhängiger Beleg. Der feste Standort widerspricht dem vorhandenen Typ mobil; die generische Namensgleichheit reicht nicht für Identitäts-/Merge-Zuordnung. Daher Straße, PLZ, Telefon, Typ und Status nicht geändert; gesucht bleibt eine Betreiberquelle und ein zweiter belastbarer Beleg. Quelle(n): https://www.11880.com/branchenbuch/herne-westfalen/120674719B108314157/schrotthandel-sumo.html ; https://lokaleschrottplatz.de/wp-json/wp/v2/search?search=Sumo&per_page=20]
### Recherche 29.10.2026 (Tiefenrecherche-Welle, Shard B)

- [Recherche 29.10.2026: Negativbefund im amtlichen Fachbetrieberegister (Spiegel schrottregister.pages.dev, Auszug 11.08.2026): Die Ortsseite Herne (Postleitzahlen 44649 und 44653) führt acht anerkannte Betriebe — Heinrich Müntefering Industrie- und Städtereinigungs-GmbH (Op der Heide 16), Kösling Metalle GmbH (Hafen Str. 1b), Paul Jost GmbH (Heerstraße 20), REMONDIS Industrie Service GmbH & Co. KG NL Herne (Heerstr. 29-43), REMONDIS Ruhr GmbH (Hafenstr. 4a-b), „Stammsitz Wilhelmstraße (Haus 12)“ (Behandeln, Verwerten) sowie Wilhelm Bötzel GmbH & Co. KG (Hafenstraße 2a und 5-15) — **keinen Betrieb namens „Sumo“** und keinen mobilen Sammelbetrieb. Die Efb-Anerkennung ist freiwillig, das Fehlen ist ein Hinweis ohne Beweiswert und widerlegt den Verzeichnislead nicht. OSM live geprüft: Für die Verzeichnisadresse Bielefelder Straße 106 existiert ein Adressknoten way/330553053 (51.5312849/7.1880885) mit den Tags addr:street=Bielefelder Straße, addr:housenumber=106, addr:postcode=44625, addr:city=Herne, building=yes — der Punkt trägt **keinen Namens-, Betreiber- oder Branchentag**, es gibt also keinen OSM-Hinweis auf einen Schrotthandel an dieser Stelle. Nebenbefund: die Postleitzahl 44625 gehört tatsächlich zu Bönen, das OSM-Tag weist sie „Herne“ zu; die Verzeichnisangabe „44625 Herne“ ist damit in sich uneinheitlich und die Zuordnung „Herne“ nicht verifiziert. Frontmatter unverändert: Straße, PLZ, Telefon und Typ bleiben leer, status pruefung; die generische Namensgleichheit wird nicht zur Identitäts- oder Merge-Zuordnung genutzt, und es wurde nicht geokodiert. ANKAUF/VERKAUF/GEBÜHREN: keine belegten Werte, keine Preislisten, keine Tarife.; Quelle(n): https://schrottregister.pages.dev/ort-herne ; https://www.openstreetmap.org/way/330553053 ; https://www.11880.com/branchenbuch/herne-westfalen/120674719B108314157/schrotthandel-sumo.html (Abruf 29.10.2026)]

### Recherche 05.11.2026

- [Recherche 05.11.2026: **Amtliches Fachbetrieberegister (E-15) am 05.11.2026 erneut geprüft: für „Schrotthandel Sumo“ in Herne kein dem Dossierstandort zugeordneter Registereintrag
  (Negativaussage mit Reichweitenklausel).** Geprüft wurden beide Routen des amtlichen Fachbetrieberegisters (https://fachbetrieberegister.gadsys.de/fachbetrieberegister/),
  Route „Entsorgungsfachbetriebe“ und Route „Altfahrzeugverwertung“, durchgängig mit dem Filter „Art des Betriebs = Egal“ (is=1&e=1); die Standortabfragen liefen mit
  validiertem adr-Parameter, der die Adresse „Bielefelder Straße 106, 44625 Herne“ samt PLZ-Abgleich (44625 per OSM/Nominatim bestätigt, Punkt 7.1880881/51.531294) enthält,
  in den Radien 3 km, 5 km und 10 km. **Namenssuche Route „Entsorgungsfachbetriebe“:** „Sumo“ 0 Treffer, „Schrotthandel Sumo“ 0, „Sumo Schrott“ 0, „Sumo GmbH“ 0,
  „Schrotthandel Sumo GmbH“ 0, „Sumo Schrotthandel GmbH“ 0. **Route „Altfahrzeugverwertung“:** dieselben sechs Varianten, jeweils 0 Treffer. **Standortabfrage:** Route
  „Entsorgungsfachbetriebe“ 3 km → 45 Treffer, 5 km → 95 Treffer, 10 km → 205 Treffer; Route „Altfahrzeugverwertung“ 3 km → 0 Treffer, 5 km → 3 Treffer, 10 km → 4 Treffer.
  **Nachbarsuche (Namenssuche der geografisch benachbarten Treffer):** „Heinrich Müntefering Industrie- und Städtereinigungs-GmbH“ 2 Treffer, „Urbaniak GmbH“ 1 Treffer und
  „Kösling Metalle GmbH“ 1 Treffer (zusammen 4 Anlagen im 3-km-Radius) führen in keiner Zeile zu einem Sumo-Eintrag, eine Umbenennung ist nicht erkennbar. **Abgrenzung:**
  Die Treffer im 3-km-Radius sind „Heinrich Müntefering Industrie- und Städtereinigungs-GmbH“ (ZZET01500116 / 1 und / 2, Op der Heide 16, 44653 Herne, 1,0 km), „Wilhelm
  Bötzel GmbH & Co. KG“ (ZZKT00100301 / 6 und / 7, Hafenstraße 2a, 44653 Herne, 1,9 km), „Kösling Metalle GmbH“ (ZZET00500070 / 1, Hafen Str. 1b, 44653 Herne, 1,9 km),
  „Urbaniak GmbH“ (ZZET01500349 / 1, Heerstraße 79, 44653 Herne, 1,9 km) und „REMONDIS Industrie Service GmbH & Co. KG, NL Herne“ (ZZMT00300052 / 1-4, Heerstr. 29-43, 44653
  Herne, 2,0 km); die Route „Altfahrzeugverwertung“ liefert „Seik Automobilrecycling GmbH“ (ZAES00600007 / 1, Im Emscherbruch 10, 45699 Herten), „Auto Müller e. K.“
  (ZAES01100023 / 1, Horster Weg 14, 45661 Recklinghausen) und „Klaus Neumann Autoverwertung“ (ZAES01500004 / 1, Karlstr. 47d, 45661 Recklinghausen). Fremdbetriebe, keine
  Zusammenführung. **Adressdifferenz:** Die Bielefelder Straße 106 selbst trägt keinen Registereintrag; die nächstgelegenen Einträge liegen an der Op der Heide,
  Hafenstraße, Heerstraße und Hafen Str. in Herne 44653. **Ergebnis:** Zertifikatsnummer, Vorgangsnummer, Zertifizierungsorganisation, Ausstellungsdatum, Gültigkeitsdatum,
  Anlagenanzahl, zertifizierte Tätigkeit und Standort sind für „Sumo“ am Dossierstandort nicht ausgewiesen; das Feld `certifications` bleibt unverändert leer. **Reichweite
  der Aussage:** geprüft wurden die oben genannten sechs Namensvarianten (Rechtsform-Pool und Inhaber-Namensvetter) in beiden Registerrouten (durchgängig mit dem Filter
  „Art des Betriebs = Egal“, is=1&e=1), die Standortsuche mit validiertem adr-Parameter in den Radien 3 km, 5 km und 10 km sowie die Namenssuche der benachbarten Treffer;
  Abrufdatum 05.11.2026; die Trefferzahlen der einzelnen Abfragen sind oben wörtlich genannt. Ein Register-Nichteintrag ist kein Nachweis dafür, dass ein Betrieb keine
  Anerkennung besitzt: die Efb-Anerkennung nach § 56 KrWG ist freiwillig, und der Dritt-Spiegel schrottregister.pages.dev wurde nicht als Beleg herangezogen.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Sumo; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Sumo; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Bielefelder%20Stra%C3%9Fe%20106%2C%2044625%20Herne%3B7.1880881%3B51.531294%3B7.200088%3B51.543294%3B7.176088%3B51.519294&dst=3&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Bielefelder%20Stra%C3%9Fe%20106%2C%2044625%20Herne%3B7.1880881%3B51.531294%3B7.200088%3B51.543294%3B7.176088%3B51.519294&dst=3&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Bielefelder%20Stra%C3%9Fe%20106%2C%2044625%20Herne%3B7.1880881%3B51.531294%3B7.200088%3B51.543294%3B7.176088%3B51.519294&dst=5&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Bielefelder%20Stra%C3%9Fe%20106%2C%2044625%20Herne%3B7.1880881%3B51.531294%3B7.200088%3B51.543294%3B7.176088%3B51.519294&dst=5&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Bielefelder%20Stra%C3%9Fe%20106%2C%2044625%20Herne%3B7.1880881%3B51.531294%3B7.200088%3B51.543294%3B7.176088%3B51.519294&dst=10&is=1&e=1; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Bielefelder%20Stra%C3%9Fe%20106%2C%2044625%20Herne%3B7.1880881%3B51.531294%3B7.200088%3B51.543294%3B7.176088%3B51.519294&dst=10&is=1&e=1]
