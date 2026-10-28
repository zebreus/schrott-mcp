---
slug: nw-erkelenz-kb
name: KB
trader_type: mobil
state: NW
city: Erkelenz
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
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: unklar
provenance_origin: table
---

# KB

## Überblick

Plausibler neuer Betreiberseiten-Kandidat: „KB Schrott&Buntmetalle“, kbschrottbuntmetalle.com. Suchindex nennt Schrottrecycling/Demontage in Mönchengladbach und Erkelenz, 0151 59012122/0177 5991441, kbschrott@gmail.com; „Über uns“ nennt **Frank-Halcour-Straße 3, 41812 Erkelenz**, mehrere Portale dagegen **Franz-Halcour-Str. 3**. Impressums-Suchsnippet nennt zusätzlich 41189 Mönchengladbach. Das sind ungeklärte Standort-/Adressabweichungen und keine freigegebenen Frontmatterdaten.

Direktabrufe per Webfetch scheitern am Transport, requests meldet fehlende Zertifikatsvertrauenskette; HTTP liefert nur „Loading...“. Suchindex-Inhalt nicht als live gelesenes Impressum ausgeben. **Ankauf:** Recyclingwerbung beweist weder vergüteten Ankauf noch Eurokurse. **Verkauf:** keine verifizierte Liste. **Gebühren:** keine verifizierten Abbruch-/Entrümpelungs-/Abholtarife. **GEO:** zuerst rechtlichen Betreiber, Frank/Franz und Erkelenz/Mönchengladbach sowie Geschäftsadresse versus Annahmehof klären. `pruefung` bleibt, keine Website-/Kontaktfills aus Snippets.

## Timeline

### Recherche 28.10.2026: Domain ist eine Parkplatz-Seite, kein Betreiberauftritt

- [Recherche 28.10.2026: Die Domain kbschrottbuntmetalle.com ist erreichbar, liefert aber KEINE Betreiberseite: Das ausgelieferte HTML ist eine Parklogic-Weiterleitung („Redirecting… / Loading“) mit anschließendem POST an router.parklogic.com — die Domain ist derzeit als Parkplatz/Domain-Parking geführt, nicht als Betreiberauftritt. Der frühere Befund „Transportfehler/CERTIFICATE_VERIFY_FAILED“ wird damit präzisiert: TLS ist serverseitig in Ordnung (Let's-Encrypt-Zertifikat für *.kbschrottbuntmetalle.com, gültig 07.10.2026–05.01.2027, Kette über YE2-Intermediate auf „ISRG Root YE“); die lokale Prüfung scheiterte nur, weil diese neue Wurzel im lokalen CA-Bundle fehlt — es lag kein Domainausfall vor. HTTPS- und HTTP-Auslieferung wurden deshalb mit deaktivierter Kettenprüfung gelesen und die Kette zusätzlich per openssl geprüft. Betreiberangaben bleiben Aggregatorstand: Das Bundes-Telefonbuch führt „KB Schrott & Buntmetalle, Franz-Halcour-Str 3, 41812 Erkelenz, (01515) 9012122, https://www.kbschrottbuntmetalle.com, kbschrott@gmail.com“ (Eintrag seit 06.10.2025, Daten zuletzt 06.10.2026), 11880 nennt dieselbe Anschrift und Nummer („Entsorgungsdienst, Schrotthandel & Containerdienste“, keine Öffnungszeiten). Beide Portale sind Aggregatoren (ein Beleg, keine Evidenz), deshalb kein Frontmatter-Fill. Adresswiderspruch teilweise aufgelöst: In OSM existiert „Franz-Halcour-Straße 3, 41812 Erkelenz-Bellinghoven“ (Way 269655394, 51.0745008/6.3287440); die im Suchindex genannte „Frank-Halcour-Straße“ mit derselben Erkelenz-PLZ 41812 ist in OSM nicht als Adresse vorhanden — sie ist als Schreibvariante von „Franz-Halcour-Straße“ zu werten. Der Profiltext im Bundes-Telefonbuch bezeichnet den Betrieb zugleich als „ansässig in Mönchengladbach“, der Impressums-Snippet nannte 41189 Mönchengladbach; die Frage Betriebsanschrift Erkelenz versus Betriebsanschrift/Annahmehof Mönchengladbach bleibt damit offen. Amtliches Fachbetrieberegister (Auszug 11.08.2026): für Erkelenz sind vier Betriebe geführt (Betriebsstätte Erkelenz, Ferdinand-Clasen-Straße 35; KLEE Entsorgung GmbH; NCG Europe GmbH; Schönmackers Umweltdiensten GmbH & Co. KG) — kein „KB“; da ein Einzelunternehmer ohne HRB im Register nicht auftaucht, beweist das keine Nichtexistenz. NorthData-Suche „KB Schrott Buntmetalle“/„Buntmetalle Erkelenz“: kein passender Rechtsträger. ANKAUF: keine Kurse; VERKAUF: keine Liste; GEBÜHREN: keine Tarife (die Portaltexte nennen Abriss, Entrümpelung, Haushaltsauflösung, Entkernung, Buntmetall-/Schrottabholung und Demontage, aber ohne Preise und ohne Betreiberbestätigung). GEO: der gespeicherte Punkt 51.0795/6.3153 löst auf Kirchstraße 4, Erkelenz-Oestrich auf und liegt rund 1,1 km von der Aggregator-Adresse Franz-Halcour-Straße 3; mangels belegter Betriebsanschrift kein Punktfill und kein `clear_fields: geo`. Offen für den Owner: Ist die Betriebsanschrift die Erkelenzer Franz-Halcour-Straße 3 oder eine Mönchengladbacher Betriebs-/Annahmestelle, und existiert der Betrieb unter einer anderen Domain? Quellen: https://www.kbschrottbuntmetalle.com/ ; https://www.kbschrottbuntmetalle.com/impressum ; https://www.kbschrottbuntmetalle.com/ueber-uns ; https://www.bundes-telefonbuch.de/erkelenz/firma/kb-schrott-buntmetalle-kt1759755311388 ; https://www.11880.com/suche/kb-schrott/erkelenz ; https://schrottregister.pages.dev/ort-erkelenz ; https://www.northdata.de/?query=KB+Schrott+Buntmetalle ; https://nominatim.openstreetmap.org/search?q=Franz-Halcour-Stra%C3%9Fe+3%2C+41812+Erkelenz&format=jsonv2&limit=5 ; https://www.openstreetmap.org/way/269655394 ; openssl s_client (Zertifikatskette *.kbschrottbuntmetalle.com); read-only `public.db` (28.10.2026).]

### Recherche 17.10.2026 — Welle 21:00 B

- [Recherche 17.10.2026: Eigene KB/Erkelenz-Suche erschließt Domain kbschrottbuntmetalle.com, Start-/Über-uns-/Impressums-Suchtreffer. Direktabrufe HTTPS mit/ohne www liefern Transportfehler; requests konkret CERTIFICATE_VERIFY_FAILED/fehlender lokaler Aussteller, nicht als erwiesener globaler Domainausfall ausgegeben; HTTP-Über-uns nur „Loading...“. Index nennt Frank-Halcour-Straße 3/41812, Portalcluster Franz-Halcour-Str. 3, Impressumssnippet 41189 Mönchengladbach. Kein lesbares Betreiberimpressum/kein qualifizierter Zweitbeleg, Standortkettenfrage bleibt. Ankauf/Verkauf/Gebühren ohne verifizierte numerische Liste, kein Fill/kein GEO. Quellen und Abrufgrenzen: https://www.kbschrottbuntmetalle.com/ ; https://www.kbschrottbuntmetalle.com/ueber-uns ; https://www.kbschrottbuntmetalle.com/impressum ; http://www.kbschrottbuntmetalle.com/ueber-uns ; Portal-Suchlead https://www.bundes-telefonbuch.de/erkelenz/firma/kb-schrott-buntmetalle-kt1759755311388 .]

### Importiert (Seed-Stand 2026-09-30)

- mobil (GS)
- Adresse: Erkelenz
