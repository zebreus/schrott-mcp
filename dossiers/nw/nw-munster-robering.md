---
slug: nw-munster-robering
name: Robering
trader_type: schrotthaendler
state: NW
city: Münster
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

# Robering

## Überblick

Ein neuer, weiterhin nur verzeichnisbasierter Lead nennt **Hans-Georg Robering Schrotthandel**, Feldkamp 19, 48165 Münster-Hiltrup-Ost, Telefon 02501 5943344. Ein 11880-Profil, zuletzt laut Portal am 15.10.2026 aktualisiert, enthält eine Nutzerbewertung vom 06.07.2020 mit der Behauptung, der Betrieb sei seit 01.11.2019 geschlossen; 11880 weist Bewertungen als ungeprüfte Nutzermeinungen aus. Das ist ein konkreter Klärhinweis, aber kein amtlicher Schließungsbeleg und keine aktuelle Betreiberbestätigung. Frontmatter bleibt leer und `status: pruefung`.

- **Ankauf:** Portalrubriken Altmaterial-/Schrotthandel, aber kein Beleg aktueller vergüteter Annahme oder numerischer Kurse.
- **Verkauf:** keine Liste.
- **Gebühren:** keine Tarifangaben.
- **Geokodierung:** Feldkamp 19 ist ein möglicher GEO-Kandidat aus zwei Verzeichnissen; vor Geokodierung erst Gewerbe-/Betreiberidentität, Annahmestelle und aktuellen Betrieb bestätigen.

## Timeline

### Recherche 25.10.2026

- [Recherche 25.10.2026: Erstmals direkt die vollständige 11880-Detailseite und das ÖffnungszeitenBuch-Profil gelesen. Beide listen Hans-Georg Robering Schrotthandel, Feldkamp 19, 48165 Münster, 02501 5943344; das ÖffnungszeitenBuch hat keine Öffnungszeiten hinterlegt. 11880 zeigt Eintragsdaten/Aktualisierung vom 15.10.2026, aber die dort eingeblendete GoLocal-Nutzerbewertung vom 06.07.2020 behauptet „seit 01.11.2019 geschlossen“; 11880 erklärt, Bewertungen nicht auf Echtheit zu prüfen. Weder Portalaktualisierung noch Einzelbewertung beweisen heutigen Betrieb bzw. amtliche Schließung. Keine Betreiberwebsite, Verkaufs-/Ankaufpreise oder Gebührenliste auf den geprüften Profilen. Adresse bleibt Lead, kein GEO und kein Frontmatter-Fill; Owner-Gewerbeauskunft/Betreiberbestätigung nötig. Quellen: https://www.11880.com/branchenbuch/muenster-westfalen/120674719B100698785/hans-georg-robering-schrotthandel.html ; https://www.oeffnungszeitenbuch.de/filiale/Muenster-Hans-Georg%2520Robering%2520Schrotthandel-3003230X.html.]

### Importiert (Seed-Stand 2026-09-30)

- Schrott (GS)
- Adresse: Münster

### Recherche 05.10.2026

- [Recherche 05.10.2026: Namen-/Ortsgegenprüfung ohne Frontmatter-Fill. Eindeutig abweichende Betreiberseite robering-regale.de: Impressum separat live gelesen, Robering Lagersysteme GmbH & Co. KG, Gutenbergstraße 25, 48282 Emsdetten, HRA 6678 Steinfurt, Christoph/Moritz Robering; Website bietet Lagersysteme/Regale/Stahlbühnen. Das beweist weder Schrottankauf noch Identität mit dem Seed „Robering, Münster“. Keine Übernahme von Emsdettener Kontakten oder Domain, kein weiterer Vollcrawl des fremden Lagersystemanbieters. NorthData-Suche Robering/Münster live gelesen, sichtbarer erster Ergebnisblock unscharfe Fremdtreffer (z. B. Robert Münsterer), kein eindeutig passender Schrotthändler; kein vollständiger negativer Registerbeweis. Quelle(n): https://www.robering-regale.de/impressum; https://www.northdata.de/Robering,+M%C3%BCnster (Suchansicht)]
- [Recherche 05.10.2026: Seed bleibt mangels Vorname/Anschrift/rechtlichem Betreiber ungeklärt. Keine zwei unabhängigen passenden Belege, keine eigene zugeordnete Website mit Impressum/Kontakt/Leistungen/Preisen/Zeiten/Zertifikaten, daher kein Frontmatter-/Service-Fill und keine Betreiber-Ausnahme. Ähnlich benannte Fenster-/Türenbetriebe oder Münsteraner mobile Schrottseiten sind ohne Identitätsbeleg nicht zuzuordnen. status pruefung erhalten, keine Schließungsbehauptung. Keine verifizierte Ankauf-/Verkaufs-/Servicepreisliste. Quelle(n): https://www.robering-regale.de/impressum; https://www.northdata.de/Robering,+M%C3%BCnster]

### Recherche 08.11.2026

- [Recherche 08.11.2026: **Amtliches Fachbetrieberegister: für den Münsterer Lead „Hans-Georg Robering Schrotthandel“ (Feldkamp 19, 48165 Münster-Hiltrup-Ost) keine Efb- oder AltfahrzeugV-Anerkennung ausgewiesen; das Adressobjekt ist ein reines Wohnhaus ohne Betriebsbezug.** Geprüft wurden beide Routen des amtlichen Fachbetrieberegisters (https://fachbetrieberegister.gadsys.de/fachbetrieberegister/), Route „Entsorgungsfachbetriebe“ und Route „Altfahrzeugverwertung“, durchgängig mit dem Filter „Art des Betriebs = Egal“ (`is=1&e=1`); die Standortabfragen liefen mit validiertem `adr`-Parameter um die Dossieradresse (Nominatim-Abgleich 08.11.2026: 51.9037381/7.6722918) in den Radien 3 km und 10 km. **Route „Entsorgungsfachbetriebe“:** die Namenssuche `n=Robering` **0 Treffer**; die Standortsuche 3 km um die Dossieradresse → **9 Treffer**, 10 km um 48165 Münster-Hiltrup → **23 Treffer** — darunter „BASF Coatings GmbH“, Glasuritstaße 1, 48165 Münster (ZZFT00500041 / 1, Lagern/Verwerten/Beseitigen, 1,5 km), „Abfallwirtschaftsbetriebe Münster | Recyclinghof Hiltrup“, Glasuritstraße 1a, 48165 Münster (ZZEE00200150 / 15, Sammeln/Lagern, 2,1 km), „SRM Schrott & Metallrecycling GmbH“, Kesslerweg 37, 48155 Münster (ZZRT00300098 / 1-2, 2,4 km), „REMONDIS GmbH & Co. KG | Behandlungs- und Lageranlage, Betriebsstätte Münster“, Kesslerweg 10, 48155 Münster (ZZET00700006 / 7-11, 2,8 km) — **kein** Treffer trägt den Namen Robering, **keiner** liegt am Feldkamp 19, und keiner wurde übernommen. **Route „Altfahrzeugverwertung“:** `n=Robering` **0 Treffer**; 3 km um die Dossieradresse → **0 Treffer**, 10 km um 48165 Münster-Hiltrup → **1 Treffer** (fremd). **Adressobjekt (neu):** OSM **node/2884286386** trägt `addr:street=Feldkamp`, `addr:housenumber=19`, `addr:postcode=48165`, `addr:city=Münster`, `addr:suburb=Hiltrup`, **`place=house`** — und **keinen** `name=`, `operator=`, `shop=` oder `craft=`-Tag; das Objekt ist ein **reines Wohnhaus** ohne jeden Betriebsbezug, ein Schrottplatz- oder Annahmestellencharakter ist daraus **nicht** ableitbar, und die Feldkamp 19 wird deshalb **nicht** geokodiert. **Zweitbeleg zur Lead-Existenz (Verzeichnis, nur Lead):** die 11880-Detailseite „Hans-Georg Robering Schrotthandel“ (live gelesen) nennt unverändert Feldkamp 19, 48165 Münster (Hiltrup-Ost), (02501) 5943344, Kategorien „Altmaterialhandel & Schrotthandel“ sowie „3 von 5 Sternen (2 Bewertungen)“; die im Dossier vom 25.10.2026 dokumentierte Nutzerbewertung vom 06.07.2020 mit der Behauptung, der Betrieb sei seit 01.11.2019 geschlossen, ist eine **ungeprüfte Plattformbewertung** und bleibt ein Klärhinweis, **kein** amtlicher Schließungsbeleg. **Ergebnis:** Zertifikatsnummer, Vorgangsnummer, Zertifizierungsorganisation, zertifizierte Tätigkeit und Standort sind für Robering am Feldkamp 19 nicht ausgewiesen; `certifications` bleibt leer. **Preise (getrennt geführt):** **Ankauf** — die Portalrubriken „Altmaterialhandel“ und „Schrotthandel“ sind **Kategorien**, kein Beleg einer vergüteten Annahme; **keine** Kurse; **Verkauf** — keine Liste; **Gebühren** — keine Tarifangaben. **Reichweite der Aussage:** geprüft wurde die Namensvariante „Robering“ in beiden Registerrouten, die Standortsuche in den Radien 3 km und 10 km um die Dossieradresse sowie die Namenssuche der benachbarten Treffer; Abrufdatum 08.11.2026. **Ein Register-Nichteintrag ist kein Nachweis**, dass der Betrieb keine Anerkennung besitzt: die Efb- und die AltfahrzeugV-Anerkennung sind **freiwillig**; der Dritt-Spiegel schrottregister.pages.dev wurde nicht als Beleg herangezogen. **Offen bleibt:** Betreiberidentität, Rechtsform und heutige Annahme — der Seed bleibt `pruefung`, und **keine** Schließung wird behauptet.
  Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Robering ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Robering ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=Feldkamp%2019%2C%2048165%20M%C3%BCnster%2C%20Deutschland;7.6722918;51.9037381;7.693087;51.925558;7.651497;51.881918&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Feldkamp%2019%2C%2048165%20M%C3%BCnster%2C%20Deutschland;7.6722918;51.9037381;7.693087;51.925558;7.651497;51.881918&dst=3&is=1&e=1 ; https://www.11880.com/branchenbuch/muenster/120674719B105000963/robering-hans-georg-schrotthandel.html ; https://api.openstreetmap.org/api/0.6/node/2884286386/full.json (Abruf 08.11.2026).]
