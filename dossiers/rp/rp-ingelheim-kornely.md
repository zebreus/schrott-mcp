---
slug: rp-ingelheim-kornely
name: Kornely
trader_type: sonstige
state: RP
city: Ingelheim
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
provenance_section: Nachtrag Wide-Net PROSE-Cluster (27.09.2026)
provenance_ankauf_raw: unklar
provenance_origin: table
---

# Kornely

## Überblick

Die Suchindexierung liefert mehrere Verzeichnis-Leads „Schrotthandel und Entrümpelung Kornely“ in Ingelheim, aber keine Betreiber-/Registerquelle. Die Adressangaben widersprechen sich: Seed Hinter der Ohrenbrücke 33, Firmania nennt Nr. 19, weitere Treffer lassen die Hausnummer weg bzw. wiederholen Nr. 33. Kein Standort-/Identitäts-Fill; Status `pruefung`.

OSM-Abgleich (30.10.2026): Auf der Straße „Hinter der Ohrenbrücke“ sind 15 Adressobjekte mit 15 verschiedenen Hausnummern erfasst, darunter sowohl die Nummer 19 (way/258803896) als auch die Nummer 33 (way/952487734); beide Objekte tragen `addr:city=Ingelheim am Rhein` und `addr:postcode=55218`. Die Nummernwahl ist damit OSM-seitig nicht entscheidbar, der Ortsbezug Ingelheim am Rhein mit PLZ 55218 aber belegt.

**Preise:** ANKAUF: Verzeichnis-Snippets nennen Materialarten und „aktuelle Preise“ auf Anfrage, jedoch keine bezifferten Ankaufssätze; Betreiberzuordnung und Bedingungen unbestätigt. VERKAUF: keine Preisliste. GEBÜHREN: keine belegten Tarife.

## Timeline

### Recherche 07.10.2026

- [Recherche 07.10.2026: Neue Suchspur „Schrotthandel und Entrümpelung Kornely“ in Ingelheim: Suchindex-Treffer nennen u.a. Hinter der Ohrenbrücke 19 (Firmania), während Seed und ein anderer Eintrag Nr. 33 nennen; Cylex führt nur „Hinter der Ohrenbrücke“ und eine Telefonnummer. Direkte Abrufe der Cylex-/Firmania-Profile waren nicht verwertbar (403), das indexierte LokaleSchrottplatz-Profil lieferte 404. Das sind nicht unabhängig bestätigte Verzeichnisdaten und kein Nachweis, welche Adresse gilt. Frontmatter bleibt unverändert. ANKAUF: nur unbestätigtes Snippet „Materialien / aktuelle Preise anfragen“, keine Zahlen; VERKAUF: keine Preisliste; GEBÜHREN: keine Angaben. Offen: Betreiber, tatsächliche Adresse und ob das Profil noch aktiv ist; Quelle: https://web2.cylex.de/firma-home/schrotthandel-und-entruempelung-kornely-16663856.html ; https://firmania.de/ingelheim/schrotthandel-und-entruempelung-kornely-8212009 ; https://lokaleschrottplatz.de/schrotthandel-und-entruempelung-kornely/ (404 beim Direktabruf)]

### Importiert (Seed-Stand 2026-09-30)

- Hinter der Ohrenbrücke 33
- urspr. Website-Angabe: keine
- Adresse: Ingelheim

### Recherche 02.10.2026

- [Recherche 02.10.2026: Kein Beleg — schrottregister ort-ingelheim führt kein "Kornely" (nur Abwasserzweckverband + Priedigkeit-Standort); Websuche "Kornely" Hinter der Ohrenbrücke 33 ohne Schrottbezug → kein Fill, status weiter pruefung. Quelle(n): schrottregister ort-ingelheim (GSA/EfB-Extrakt, Stand 2026-08-11, Abruf 02.10.2026)]

### Recherche 30.10.2026

- [Recherche 30.10.2026: **OSM-Abgleich: beide Hausnummern der Seed-Kontroverse sind in OSM vorhanden, die Straße trägt die PLZ 55218.** Die Straße "Hinter der Ohrenbrücke" in Ingelheim am Rhein besteht in OSM aus fünf Ways (way/187244992, way/258803906, way/721956671, way/721956672, way/721956674). Im Rasterfenster **8.0541220/49.9555021 bis 8.0597821/49.9604168** (Way-Union-BBox plus 0,002 Grad Rand) liegen dort **15 Adressobjekte / 15 verschiedene Hausnummern**: 4, 8, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37. Für die Seed-Nummer 33 existiert **way/952487734** mit den Tags `addr:street=Hinter der Ohrenbrücke`, `addr:housenumber=33`, `addr:city=Ingelheim am Rhein`, `addr:country=DE`, `addr:postcode=55218`, `building=house`; für die konkurrierende Nummer 19 existiert **way/258803896** mit `addr:housenumber=19`, `addr:street=Hinter der Ohrenbrücke`, `addr:city=Ingelheim am Rhein`, `addr:country=DE`, `addr:postcode=55218`, `building=house`. Der Adress-String lautet damit in beiden Faellen "Hinter der Ohrenbrücke", der Ort "Ingelheim am Rhein" und die PLZ **55218** (im Dossier-Feld `postcode` bisher leer). Die Nummernwahl 19 gegen 33 laesst sich aus OSM **nicht** entscheiden; kein Objekt trägt einen Betreiber-`name=` oder `operator=`. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json (bbox-Raster, Abruf 30.10.2026); https://api.openstreetmap.org/api/0.6/way/952487734.json; https://api.openstreetmap.org/api/0.6/way/258803896.json]
- [Recherche 30.10.2026: **Neuer Verzeichnis-Spur mit Ortsangabe 19, weiterhin kein Betreiberbeleg.** Eine Websuche liefert einen Beitrag aus einer Facebook-Gruppe mit dem Text "KONTAKT: 015226244573 6244573 55218 Ingelheim der Ohrenbrücke 19" sowie das Suchindex-Snippet des Cylex-Profils "Schrotthandel und Entrümpelung Kornely / Hinter der Ohrenbrücke in Ingelheim am Rhein, 01522 6244..." (Rufnummer im Snippet abgeschnitten). Beides sind Social- bzw. Aggregator-Leads ohne Betreiber- oder Registerbeleg; der Direktabruf der Cylex-Seite scheitert weiterhin an HTTP 403. Die PLZ 55218, die in beiden Quellen mit der Nummer 19 steht, deckt sich mit dem OSM-Bestand und ist deshalb als geografischer Ortsbezug dokumentiert, aber sie **bestätigt die Nummer 19 nicht**. ANKAUF: nur das bereits bekannte Snippet "aktuelle Preise anfragen" ohne Zahlen. VERKAUF: keine Preisliste. GEBÜHREN: keine Angaben. Quelle(n): https://www.facebook.com/groups/848354735559729/posts/2810075462720970 (Suchindex-Lead, Abruf 30.10.2026); https://web2.cylex.de/firma-home/schrotthandel-und-entruempelung-kornely-16663856.html (Direktabruf HTTP 403, Abruf 30.10.2026)]
- [Recherche 30.10.2026: **Amtsregister-Gegenprobe ohne Kornely-Eintrag; kein Negativbefund.** Die Ortsseite Ingelheim des amtlichen Fachbetrieberegisters (Auszug 11.08.2026, Spiegel schrottregister.pages.dev) fuehrt nur "Abwasserzweckverband 'Untere Selz' | Außenliegend 122 | Behandeln" und "Standort / Hauptsitz | Budenheimer Weg 71 | Sammeln, Befoerdern, Handeln, Makeln". Ein Kornely-Eintrag fehlt; da die Efb-Anerkennung freiwillig ist, beweist das **nicht** die Existenz oder Nichtexistenz des Betriebs. Quelle(n): https://schrottregister.pages.dev/ort-ingelheim (Abruf 30.10.2026)]
