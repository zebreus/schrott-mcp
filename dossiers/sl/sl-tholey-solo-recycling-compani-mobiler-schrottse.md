---
slug: sl-tholey-solo-recycling-compani-mobiler-schrottse
name: SOLO Recycling compani (mobiler Schrottservice/Startup)
trader_type: schrotthaendler
state: SL
city: Tholey
street: Keltenweg 4
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://sites.google.com/view/solo-recycling-compani/inicio
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Nachtrag Audit-Runde 3 (Stand 2026-09-27)
provenance_ankauf_raw: ja (Teilmengen/mobil)
provenance_origin: table
---

# SOLO Recycling compani (mobiler Schrottservice/Startup)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Recherche 01.10.2026

- [Recherche 01.10.2026: Feedback 2617 berechtigt als Klärfall (Einzelbeleg Startup, kein HR). Betreiber-Site live verifiziert: Gewerbegebiet Keltenweg 4, Tholey (66636), Tel. +49 152 10978340, WhatsApp +49 1521 1409971, recyclings@yahoo.com, Mo-Sa 08:00-18:00; mobiler Schrottplatz/Abholservice (trader_type mobil plausibel, aber Einzelbeleg → keine Frontmatter-Änderung). Werte NUR hier; website_status aktiv; Quelle(n): https://sites.google.com/view/solo-recycling-compani/inicio (eine Quelle)]

### Importiert (Seed-Stand 2026-09-30)

- mobile Altmetall-Abholung (geeichte Waage, sortenrein), „je nach Material zahlen wir sogar dafür"; Gewerbe + privat

### Recherche 31.10.2026

- [Recherche 31.10.2026: OSM-Adressprüfung live am Objekt, Raster bbox=7.0050795,49.4897709,7.0291362,49.5110343 (OSM-Way-BBox des gesamten Straßenverlaufs "Keltenweg", way/38350602 + way/288088888 + way/464375941, plus 0,010 Grad Rand; in Kacheln abgetastet), Filter addr:street=Keltenweg mit addr:housenumber: **15 Objekte** | **15 verschiedene Hausnummern** (roh 2, 2A, 4, 6, 8, 8a, 10, 11, 12, 13, 17, 18, 24, 25, 30 — keine Bereichsnotation, daher unverändert aufgelöst). **Die Dossier-Hausnummer 4 ist in OSM nicht erfasst:** auf der Straße "Keltenweg" existiert die Nummer 4 **nicht**; die Nummernfolge lautet 2, 2A, 6, 8, 8a, 10, 11, 12, 13, 17, 18, 24, 25, 30 (way/355515637 mit `addr:housenumber=6`, way/1378843467 mit `addr:housenumber=25` usw.). **Adress-Diskrepanz:** die Betriebs-Seite nennt „[Gewerbegebiet] Keltenweg 4, Thelev 66636 Tholey", das Branchenverzeichnis 11880.com (Eintragsdaten 25.10.2026) nennt „**Gewerbegebiet Keltenweg 2**, 66636 Tholey (Theley)". In OSM existiert **beides** als **zwei verschiedene Straßen**: die Objekte mit `addr:street=Gewerbegebiet Keltenweg` (u.a. way/397315420 mit `addr:housenumber=2`, `name=Sinnewe Baustoffe`; way/397315402 mit `addr:housenumber=4`, `name=Werthund`, `shop=pet`; way/397315107 mit `addr:housenumber=6`, `name=egesa Zookauf`; way/397315111 mit `addr:housenumber=21`, `name=Roger Wagner Dachdecker`; way/233968365 mit `addr:housenumber=17`, `name=Schmiedetechnik Plettenberg`; way/233968366 mit `addr:housenumber=11`, `name=Jäckel Elektrotechnik GmbH`; way/160895807 mit `addr:housenumber=15`, `name=H. Werner Dewes Bau - und Möbelschreinerei`) liegen auf der **anderen** Straße als der Filter `addr:street=Keltenweg`. Die Nummern 2 und 4 sind also im Gewerbegebiet belegt, aber **mit anderen Firmen** — die **SOLO Recycling Company ist in OSM nicht kartiert**. Von den 15 Adressobjekten der Straße „Keltenweg" trägt **0** ein `name=`- oder `operator=`-Tag. Im gesamten Raster tragen 262 Objekte `name=` oder `operator=`; Objekte mit Bezug zu **Schrott/Metall/Recycling** sind dort **0** (Kriterium: `shop=gold_buyer` | `amenity=recycling` | `industrial=` | `landuse=industrial` | `recycling:` oder ein Name/Operator mit Schrott-/Recycling-Bezug). Der namentlich passende Nachbarbetrieb way/283338609 `name=Hans Zewe GmbH Stahlhandel Werk 1` (`addr:street=Gewerbegebiet Hanacker`, `addr:housenumber=1`) und way/397315343 `name=Hans Zewe GmbH Stahlhandel Werk 2` (`addr:street=Gewerbepark BAB 1`, `addr:housenumber=1`) tragen **keinen** Recycling-Tag und sind **nicht** mit dem Dossier gleichzusetzen. Quelle(n): https://api.openstreetmap.org/api/0.6/map.json?bbox=7.0050795,49.4897709,7.0291362,49.5110343 ; https://www.11880.com/branchenbuch/tholey/060444309B112547254/schrotthandel-solo-recycling-company.html (Abruf 31.10.2026)]
- [Recherche 31.10.2026: Preise getrennt nach Ankauf, Verkauf und Gebühren. **Ankauf:** Die Betriebsseite belegt eine **Vergütung**, aber **keinen** Euro-Betrag: „Wir wiegen bei uns vor Ort und zahlen bar" und „Du willst Deinen Metallschrott unkompliziert loswerden? … Wir tragen den Schrott raus, verwiegen ihn mit geeichter Waage und bringen ihn sortenrein zum geeigneten Entsorgungsfachbetrieb. **Je nach Material zahlen wir sogar dafür!**" — die Vergütungszusage und die ** geeichte Waage** sind damit belegt, eine Preisliste je Material **nicht**; die Seite nennt weder Mindestmengen noch Kurse. **Verkauf:** Hier hat der Betrieb ein echtes **Verkaufsprodukt**: Die zweite Unterseite („Documentación") wirbt mit „**HOCHWERTIGE KUPFERGRANULATE** — Wir produzieren Kupfer-Granulate, die den Anforderungen der weiterverarbeitenden Industrie voll gerecht werden"; das Angebot umfasst außerdem **drei Servicepakete** für Gewerbekunden („Sie wählen aus drei Servicepaketen, die die SchrottBienen vor Ort für Sie erledigen. Die **faire Preisgestaltung** in Verbindung mit unserem **Rückvergütungssystem** bildet die Basis für eine langfristige Partnerschaft"), bei denen ausdrücklich „ein digitaler Nachweis über die Entsorgung" und ein „attraktives Arbeitgeberargument" (tausende Tonnen Schrott, die Mitarbeiter nicht schleppen müssen) beworben werden — konkrete **Pauschalen oder Paketpreise** werden **nicht** genannt. **Gebühren:** Die einzige bezifferte Aussage ist die **Gebührenfreiheit für Privathaushalte**: „Für private Haushalte **komplett kostenfrei** und ohne weiteren Aufwand." — die Abholung durch den mobilen Schrottplatz ist für Privatleute damit ausdrücklich **entgeltfrei**; für Gewerbe ist die Vergütung über das „**Rückvergütungssystem**" geregelt, dessen Beträge **nicht** veröffentlicht sind. Die Sorgfaltsregel „Nach den Richtlinien des KrWG dürfen nur Gewerbetreibende Elektroschrott abgeben" gilt hier sinngemäß: die Seite „Was passiert mit dem Schrott?" nennt als angenommene Fraktionen Altmetalle und Schrott sowie Elektroschrott. Quelle(n): https://sites.google.com/view/solo-recycling-compani/inicio ; https://sites.google.com/view/solo-recycling-compani/documentación (Abruf 31.10.2026)]
