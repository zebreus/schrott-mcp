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

### Geokodierung 03.11.2026

- [Geokodierung 03.11.2026: **Klärfall - keine Koordinate gesetzt.** Die Dossier-Hausnummer **56** ist in OSM auf der
  Dossier-Straße **Hauptstraße** zwar **eindeutig** belegt, aber **es fehlt die zweite unabhängige Quelle**: die
  Dossier-Adresse stammt ausschließlich aus **Verzeichnis-Leads**, und nach der README-Quellenhierarchie sind
  Verzeichnisse ausdrücklich **Leads, keine Evidenz** - ein Verzeichnis plus OSM bleibt ein **Einzelbeleg**.
  **Was geprüft wurde (live am 03.11.2026):** (a) Die Straße **Hauptstraße** in Binningen trägt in OSM **über 80
  Adressobjekte**; die **Hausnummer 56 ist genau einmal** belegt und damit eindeutig: **way/492658719** mit
  `addr:housenumber`=**56**, `addr:street`=**Hauptstraße**, `addr:city`=**Binningen**, `addr:postcode`=**56754**,
  `addr:country`=DE und `building`=residential (Kachelraster über **bbox=7.2300,50.1950,7.2600,50.2150**, live
  abgerufen 03.11.2026); Objekt-BBox 50.2046042/7.2436404 bis 50.2046735/7.2437773. (b) Eine Suche über **alle**
  Tag-Werte nach „esper“ oder „schrott“ im Raster liefert **0 Objekte**, die **den eigenen Betrieb** nennen - der
  Betrieb ist in OSM **nicht namentlich kartiert**; das Objekt trägt **weder `name=` noch `operator=`** und ist als
  `building`=**residential** kartiert, was eine **Wohnnutzung** und **keine** belegte gewerbliche Annahme nahelegt.
  **Quelle 1 (Verzeichnis):** Die **11880-Detailseite** (live abgerufen 03.11.2026, HTTP 200,
  https://www.11880.com/branchenbuch/binningen-eifel/060440092B113059782/schrotthandel-esper.html) führt
  „Schrotthandel Esper“ mit „**Hauptstr. 56, 56754 Binningen, Eifel**“, Mobil **(0160) 3872487**,
  E-Mail schrotthandel.esper@web.de, Öffnungszeiten täglich 08:00–20:00 (Profilaktualisierung 26.10.2026,
  zugrunde liegende Eintragsdaten 09.01.2026); der **Schrottplatz.org-Eintrag** (live abgerufen 03.11.2026,
  https://www.schrottplatz.org/binningen/schrott-esper-frank-u-michaela-aYEPQz.html) nennt dieselbe Anschrift mit
  dem Festnetz 02672 91 25 87. Beides sind **Verzeichnisse**; eine **Betreiberseite**, ein **Impressum** oder ein
  **Registereintrag** existiert **nicht**. Der im Dossier dokumentierte **Namens- und Nummernkonflikt** bleibt offen:
  11880 und die Kleinanzeige nennen **Raphael Esper**, Schrottplatz.org nennt **Frank u. Michaela Esper** - die
  Einträge sind damit **nicht eindeutig demselben Betrieb** zuzuordnen. **Quelle 2 (OSM):** way/492658719 und die
  übrigen Adressobjekte der Hauptstraße (live abgerufen 03.11.2026 über
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.2300,50.1950,7.2600,50.2150). **Was fehlt:** eine
  **Betreiber- oder Registerquelle**, die Anschrift, Inhaber und Betrieb bestätigt, sowie die Klärung, ob an der
  Hauptstraße 56 überhaupt eine öffentliche Annahme stattfindet. `status` bleibt **pruefung**,
  `clear_fields: 'geo'` bleibt bestehen. Quelle(n):
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.2300,50.1950,7.2600,50.2150 (Kachelraster, live abgerufen
  03.11.2026) ; https://www.11880.com/branchenbuch/binningen-eifel/060440092B113059782/schrotthandel-esper.html ;
  https://www.schrottplatz.org/binningen/schrott-esper-frank-u-michaela-aYEPQz.html (live abgerufen 03.11.2026).]

### Geokodierung 05.11.2026

- [Geokodierung 05.11.2026: **Klärfall - weiterhin keine Koordinate, weil die zweite unabhängige Quelle fehlt.** Der
  OSM-Befund der Vorphase ist **live bestätigt** und der Adresspunkt **eindeutig**, aber die Dossier-Adresse stammt
  ausschließlich aus **Verzeichnis-Leads**; nach der README-Quellenhierarchie sind Verzeichnisse ausdrücklich **Leads,
  keine Evidenz** - ein Verzeichnis plus OSM bleibt ein **Einzelbeleg**. **Was geprüft wurde (live am 05.11.2026):**
  (a) **Suchfenster (0,01°-Kacheln):** **bbox=7.2350,50.2000,7.2550,50.2100** = 2 Kacheln à 0,01°
  (7.2350/50.2000/7.2450/50.2100 und 7.2450/50.2000/7.2550/50.2100), live abgerufen am 05.11.2026; **Zensus:** **79**
  Adressobjekte mit `addr:street`=Hauptstraße im Fensterausschnitt (Nummern 1, 2, 3, 6, 8, 10, 12, 13, 14, 15, 16, 17A,
  18, 19, 21, 22, 22A, 23, 25, 25A, 26, 27, 28, 29, 30, 31, 32, 33, 33A, 34, 35, 36, 37, 39, 40, 41, 42, 43, 44,
  45, 46, 47, 48, 49, 49A, 50, 51, 52, 53, 54, 55, **56**, 57, 58, 59, 60, 61, 63, 64, 65, 66, 67, 68, 69, 70, 71,
  72, 73, 74, 75, 76, 77, 78, 79, 79A, 80, 81, 81A, 82, 83) - die **Hausnummer 56 ist genau einmal** belegt und damit
  eindeutig; eine Suche über
  **alle** Tag-Werte nach „**Esper**" oder „**Schrott**" liefert **0 Treffer** - der Betrieb ist in OSM **nicht namentlich
  kartiert**. (b) **Objekt (live geladen):** **way/492658719** mit `addr:housenumber`=**56**, `addr:street`=**Hauptstraße**,
  `addr:city`=**Binningen**, `addr:postcode`=**56754**, `addr:country`=DE, `building`=**residential** (live am 05.11.2026
  über https://api.openstreetmap.org/api/0.6/way/492658719/full.json); 5 Knotenreferenzen, davon **4 eindeutige**;
  Objekt-BBox 50.2046042/7.2436404 bis 50.2046735/7.2437773; Mittelwert der eindeutigen Knoten 50.2046336/7.2436952,
  liegt **innerhalb** der Objekt-BBox; die **Objekt-BBox liegt vollständig im Suchfenster** (Prüfung ✓). Das Objekt trägt
  **weder `name=` noch `operator=`** und ist als `building`=**residential** kartiert, was eine **Wohnnutzung** und **keine**
  belegte gewerbliche Annahme nahelegt. (c) **Registerabgleich (neu):** Der Abzug des **Fachbetrieberegisters der
  Gemeinsamen Stelle Altfahrzeuge und des Registers der Entsorgungsfachbetriebe** (Stand 11.08.2026, über den Spiegel
  schrottregister.pages.dev) kennt für **Binningen (56754)** **keinen** Betrieb; eine Suche im **vollständigen
  Registerdatensatz** (4.507 Einträge, Stand 11.08.2026) nach „**Esper**" liefert **0** Treffer. **Was fehlt:** eine
  **Betreiber- oder Registerquelle**, die Anschrift, Inhaber und Betrieb bestätigt, sowie die Klärung, ob an der
  Hauptstraße 56 überhaupt eine öffentliche Annahme stattfindet; der im Dossier dokumentierte **Namens- und
  Nummernkonflikt** (11880/Kleinanzeige nennen Raphael Esper, schrottplatz.org nennt Frank u. Michaela Esper) bleibt
  offen. `status` bleibt **pruefung**, `clear_fields: 'geo'` bleibt bestehen. **Reichweitenklausel:** Das Fehlen eines
  Registerbelegs oder einer Efb-Anerkennung ist **kein** Existenz- oder Schließungsbeweis. Quelle(n):
  https://api.openstreetmap.org/api/0.6/way/492658719/full.json ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.2350,50.2000,7.2450,50.2100 ;
  https://api.openstreetmap.org/api/0.6/map.json?bbox=7.2450,50.2000,7.2550,50.2100 ;
  https://schrottregister.pages.dev/assets/institute.json (vollständiger Registerdatensatz, Stand 11.08.2026) ;
  https://www.11880.com/branchenbuch/binningen-eifel/060440092B113059782/schrotthandel-esper.html ;
  https://www.schrottplatz.org/binningen/schrott-esper-frank-u-michaela-aYEPQz.html (live abgerufen 05.11.2026).]

### Recherche 11.11.2026

- [Recherche 11.11.2026: **Der Betrieb ist im amtlichen Fachbetrieberegister weder als Entsorgungsfachbetrieb noch als Altautoverwertungsbetrieb geführt — die Namenssuche liefert in beiden Routen 0 Treffer, und die Standortsuche um die Dossieranschrift ergibt im 10-km-Radius nur zwei Efb-Einträge einer Caritas-Werkstatt in Mayen, also keinen einzigen zertifizierten Schrottbetrieb im Umfeld.** Route `Entsorgungsfachbetriebe`, `n=Esper` mit `dst=10`: **0 Treffer**; `n=Schrotthandel Esper`: **0 Treffer**; Die Namenssuche der Route `Altfahrzeugverwertung` mit `n=Esper` war am 11.11.2026 **nicht** durchführbar (das Register-Backend verweigerte ab der zweiten Tageshälfte die Verbindung) — ein AfV-**Namens**befund liegt damit **nicht** vor; die AfV-Route ist über die Standortabfrage um die Dossieranschrift abgedeckt (dort **0** Anlagenzeilen in 3 km und 10 km). Die Standortabfrage um **Hauptstr. 56, 56754 Binningen** (50.2046336/7.2436952) mit Radius **3 km** liefert in **beiden** Routen **0** Anlagenzeilen; mit Radius **10 km** sind es **2** Anlagenzeilen, beide von einem **anderen** Rechtsträger: „**St. Raphael Caritas Alten- und Behindertenhilfe GmbH - Caritas Werkstätten St. Johannes**, Ludwig-Erhard-Straße 17, 56727 Mayen, **ZZIT00700139 / 1**, Sammeln, Befördern" (8,9 km) und derselbe Rechtsträger an der **Anschrift in Mayen** (8,7 km). **Damit ist für Schrotthandel Esper weder eine EfbV-Zertifizierung noch eine AltfahrzeugV-Anerkennung belegt** — und der Umkreis enthält keinen zertifizierten Schrottbetrieb. **Reichweitenklausel:** Die Efb- und die AltfahrzeugV-Anerkennung sind freiwillig, ein Register-Nichteintrag ist **kein** Nachweis für fehlende Zertifizierung, für Nichtexistenz oder für Betriebsende. Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Esper ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Esper ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Schrotthandel+Esper (Abruf 11.11.2026).
  **Der 11880-Eintrag nennt jetzt eine E-Mail-Adresse, die im Dossier vom 27.10.2026 noch als Kleinanzeigen-Snippet geführt wurde; sie steht im 11880-Seitenquelltext als schema.org-Kontaktangabe — die Zuordnung zum Betrieb ist damit portalintern belegt, die Betreiber-Identität bleibt es nicht.** Live abgerufen (11880, HTTP 200, `…/binningen-eifel/060440092B113059782/schrotthandel-esper.html`): Der eingebettete schema.org-Block lautet wörtlich „`"@type":"RecyclingCenter","name":"Schrotthandel Esper","url":"https://www.11880.com/branchenbuch/binningen-eifel/060440092B113059782/schrotthandel-esper.html","email":"schrotthandel.esper@web.de","address":{"@type":"PostalAddress","postalCode":"56754","addressLocality":"Binningen, Eifel","addressRegion":"Rheinland-Pfalz","streetAddress":"Hauptstr. 56"},"geo":{"@type":"GeoCoordinates","longitude":7.243709,"latitude":50.204639}`" mit **Öffnungszeiten Montag bis Samstag 08:00–20:00** (Sonntag **nicht** aufgeführt). Die Seitenüberschrift der 11880-Suche bestätigt den Adress- und E-Mail-Bezug: „Alle Ergebnisse für **Esper in Binningen, Eifel**, **1 Treffer**", Filterzeile „**Firmen (1)**, **Altmetallrecycling (1)**, hat Öffnungszeiten (1), jetzt geöffnet (1), **E-Mail-Adresse (1)**, Telefon (0), **Mobil-Telefon (1)**, **Webseite (1)**" mit dem Treffer „**Schrotthandel Esper**, Altmetallrecycling, Geöffnet, **Hauptstr. 56, 56754 Binningen, Eifel**, **(0160) 3872487**". Der **„Webseite"-Eintrag** der 11880-Filterzeile verweist jedoch **nicht** auf eine Betreiberdomain, sondern auf den 11880-eigenen Bewertungs-Widget-Link (`…/altmetallrecycling/binningen-eifel/schrotthandel-esper-bewertungen.html`) — es gibt **keine** eigene Website des Betriebs. Die **Gelbe Seiten**-Ortssuche „Schrott in Binningen" (live 11.11.2026, HTTP 200) führt **2 Treffer**, **keinen** davon für Esper, sondern „**In der Hohl 5, 56753 Naunheim**" (7,6 km) und „**Burg-Eltz-Str. 20, 56294 Wierschem**" (7,8 km); die **11880**-Namenssuche „Esper in Binningen" weist als „Esper in der Umgebung" **„W.M.Esper Auto-Industrieteile**, Am Eichenhain 1, 56812 Cochem (Brauheck), (02671) 8900" (10,9 km) aus — ein **Namensvetter** aus dem Automobilteilehandel. **Der im Dossier dokumentierte Namens- und Nummernkonflikt bleibt offen:** 11880 und die Kleinanzeige nennen **Raphael Esper** mit der Mobilnummer 0160 3872487, der Schrottplatz.org-Eintrag **„Schrott Esper Frank u. Michaela"** mit dem Festnetz 02672 91 25 87 — die E-Mail **schrotthandel.esper@web.de** stammt aus dem 11880-Eintrag und wird **nicht** in das Frontmatter übernommen, weil die Namens- und Nummernlage zwei Personenkreise offenlässt. **ANKAUF:** die Kleinanzeigen-Aussage „Wir entsorgen für Sie: Metall, Schrott, Altmetall, Kupfer" nennt **keinen** Kurs; die „50 € VB" sind ohne Material-, Mengen- oder Einheitenbezug und damit **keine** Preisliste. **VERKAUF:** keine Liste. **GEBÜHREN:** keine Tarife; die Öffnungszeiten 08:00–20:00 sind **Portalangaben** und werden nicht übernommen. Das Frontmatter bleibt unverändert (`status: pruefung`, `phone`/`email`/`opening_hours`/`website` leer, `clear_fields: 'geo'` bleibt bestehen), eine Koordinate wird **nicht** gesetzt. Quelle(n): https://www.11880.com/branchenbuch/binningen-eifel/060440092B113059782/schrotthandel-esper.html ; https://www.11880.com/suche/esper/binningen ; https://www.gelbeseiten.de/suche/schrott/binningen (Abruf 11.11.2026).]]
