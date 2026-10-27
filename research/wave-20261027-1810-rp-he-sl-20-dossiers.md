# Recherchewelle 2026-10-27, Shard A (RP, HE, SL) - 20 Dossiers

20 vorgegebene Dossiers wurden anhand ihrer bisherigen Timeline als Ausgangspunkt erneut
geprüft und in den Dossierdateien fortgeschrieben: Frontmatter-Skalare nur mit belegten
Werten, `## Timeline` um datierte Notizen im Format `[Recherche DD.MM.YYYY: ...; Quelle(n): ...]`
ergänzt. Bestehende Timeline-Historie, Slugs und Fremdquellen wurden nicht überschrieben.
Kein Commit, kein Push, kein Deploy; die Produktionsdatenbank wurde ausschließlich
read-only (`mode=ro`) abgefragt, es wurde keine Produktionszeile geschrieben.

Maßgeblich waren die Quellenhierarchie und der Zwei-Quellen-Standard aus `README.md`:
Aggregatoren (11880, Gelbe Seiten, Cylex, Schrottplatz-Portale, LokaleSchrottplatz,
Stadtbranchenbuch, WLW, gold.de, MapQuest) bleiben Leads, keine Belege. Eine
Betreiber-Primärquelle stützt nur die von ihr selbst ausdrücklich geführten Fakten.
Amtliche Gegenproben wurden über den EU-VIES-Dienst geführt (Abruf 27.10.2026); ein positives
VIES-Ergebnis bestätigt die Existenz und Zuordnung der USt-IdNr, nicht die Kundenannahme.
Keine Koordinate wurde geraten; `clear_fields: 'geo'` wurde nur dort gesetzt, wo die Zeile eine
eigene belegte Ortsangabe hat, die dem gespeicherten Punkt widerspricht.

## Frontmatter-Änderungen je Dossier

| Slug | Änderung | Kernbeleg |
|---|---|---|
| `rp-trier-54294-dres-georg` | street `Andreas-Hoevel-Str. 6`, postcode `54294`, phone `0651 84609`, city `Trier 54294` → `Trier`, `clear_fields: 'geo'` | Gelbe Seiten (direkt gelesen) + Cylex + werkenntdenbesten nennen denselben Betrieb; Seed-Name und PLZ stimmen überein |
| `rp-binningen-esper-binningen` | street `Hauptstr. 56` (korrigiert aus Seed-Mischtext), postcode `56754` | 11880 (aktualisiert 26.10.2026) + Schrottplatz.org + Kleinanzeige 16.08.2026 |
| `rp-bechhofen-paul-bechhofen` | keine | schrott-paul.de: DNS vorhanden, Abruf ohne Antwort (kein Schließungsbeleg); kein Registerbeleg |
| `rp-dhron-feuereisen-neumagen-dhron` | keine | 11880-Eintrag aktuell (Eintragsdaten 19.06.2026) mit beiden Nummern; Facebook-Betreiberprofil weiter login-gesperrt |
| `rp-breitscheid-frohlich-breitscheid` | `clear_fields: 'geo'` | gespeicherter Punkt löst auf Horressen/56410 auf, rund 35 km von Breitscheid (Altenkirchen) entfernt |
| `rp-ludwigshafen-weingartner` | keine | vier Verzeichnisse bestätigen die Anschrift; Geo-Punkt (67063) widerspricht der Zeilenadresse (67059), beide Adressvarianten sind offen |
| `rp-geinsheim-sudpfalz-metall-geinsheim` | street `Böbinger Straße 9`, postcode `67435`, email `mail@suedpfalz-metall.de`, website `https://suedpfalz-metall.de/` (website_status `tot`), description | archiviertes Betreiber-Impressum (13.01.2026): Benjamin Glaser, USt-IdNr DE359650878, VIES gültig; Seite noch bis 07.02.2026 online |
| `rp-hordt-wetzel-hordt` | street `Wörthstr. 8` (korrigiert aus Seed-Mischtext), postcode `76771`, phone `07272 74864` | 11880 (direkt gelesen) + Stadtbranchenbuch Hördt, deckungsgleich |
| `rp-koblenz-56073-r-a` | `clear_fields: 'geo'` | gespeicherter Punkt = `2, Devorastraße` = Schrottplatz 1A Schrotthandel, ein anderer Koblenzer Betrieb |
| `rp-neuwied-laroche-neuwied-block` | street `Kirchoff 1`, postcode `56566`, phone `0176 77627100`, email `denisevs94@hotmail.com`, opening_hours, website + `website_status: aktiv`, description | Betreiber-Impressum + unabhängiger 11880-Eintrag `schrotthandel-westerwald` mit identischer Anschrift und Nummer |
| `rp-bellheim-faust-bellheim` | keine | LokaleSchrottplatz-Profil voll gelesen (Hördter Str. 48, 07272 91693, Mindestgewichte, „Geschlossen"); keine Betreiberquelle |
| `rp-mainz-55116-gold-silber-ankauf` | keine | DEG e.K. führt weiter keine Mainzer Filiale; USt-IdNr DE232448782 VIES gültig; aktueller Mainzer Goldhändler ist SOLIT/GoldSilberShop (anderer Rechtsträger) |
| `he-wiesbaden-mainz-kastel-metall-rohstoffhandel-lagerin` | postcode `55252`, phone `0176 80439557`, email `info@mrl-lagerin.de`, website + `website_status: aktiv`, description, `clear_fields: 'geo'` | Betreiber-Impressum; gespeicherter Punkt löst auf Friedrich-Ebert-Allee Wiesbaden auf, Beleganschrift liegt in Mainz-Kastel |
| `he-limburg-65549-roth-heinrich` | city `Limburg`, street `Stephanshügel 14`, postcode `65549`, website + `website_status: aktiv`, status `pruefung` → `geschlossen`, description | Betreiberseite: „mit Sitz in 65549 Limburg Stephanshügel gegründet" + „Unser Betrieb bleibt geschlossen"; 11880-Eintrag Limburg dauerhaft „Geschlossen" |
| `he-nidderau-ab-recycling-bjoern-asal` | street `Saalburgstr. 12`, postcode `61130`, phone `06187 470 88 85`, email `abrecycling@gmx.de`, opening_hours `Mo-Fr 08:00-18:00`, description, **lat/lon gesetzt** | Betreiber-Impressum + Kontakt-/Leistungsseiten; WLW-Eintrag nennt dieselbe Anschrift; Hausadresspunkt OSM way 338962561 |
| `he-wetzlar-jori-recycling-schrott-wetzlar` | keine | Stadt Wetzlar führt als Annahmestelle das GWAB-Recyclingzentrum Westendstraße 15; Betreiberprofil nur als Suchindex sichtbar |
| `he-wiesbaden-mainz-kastel-willefuhr-industriedemontagen` | street `An der Gabelung 28`, postcode `55252`, email `willefuehr-Industriedemontagen@gmx.de`, website + `website_status: aktiv`, status `pruefung` → `aktiv`, description | Impressum aus dem ausgelieferten JS-Bündel gelesen; USt-IdNr DE327851795 VIES gültig; 11880 bestätigt Anschrift und Nummern |
| `he-gieen-wieseck-pr-recycling-sandro-roggendorf` | street `Am Siegborn 336`, postcode `35396`, phone `0177-5545410`, email `info@pr-recycling.de`, opening_hours `Mo-Sa 06:00-21:00, So geschlossen`, description, `clear_fields: 'geo'` | Betreiber-Impressum + Schrottplatz- und Containerdienstseiten desselben Betriebs; AGB nennen jetzt „PR RECYCLING, Am Siegborn 336" als Anbieter; gespeicherter Punkt = Ostanlage Gießen 35390 |
| `he-eschborn-65760-pfaff-alexander` | keine | weiterhin nur 11880; `clear_fields: 'phone'` bleibt |
| `sl-riegelsberg-reinhard-rojan` | phone `06806 480361` (Leerfeld gefüllt) | 11880 + Stadtbranchenbuch + Schrottplatz.org nennen die Festnetznummer deckungsgleich; Mobilnummer 0172 6857888 nur in Gelbe Seiten |

## Preise, Gebühren und Ankaufsbedingungen

- **Ankauf (numerisch):** in keinem der 20 Dossiers wurde eine händlerspezifische,
  materialbezogene Ankaufspreisliste gefunden, die dem jeweiligen Seed zugeordnet werden
  darf. Die einzigen bezifferten Kurse stammen aus dem Mainzer GoldSilberShop
  (SOLIT Management GmbH, Rheinstraße 103; Silber 625er ca. 0,86 EUR/g bis 925er ca. 1,36 EUR/g,
  Quelle gold.de) und gehören zu einem **anderen** Betreiber; sie wurden ausdrücklich nicht
  übernommen.
- **Ankauf (Verfahren):** PR Recycling wirbt tagesaktuelle Preise, kostenlose Verwiegung und
  Sofortauszahlung; MRL Lagerin „Tageshöchstpreise"; A-B-Recycling „Tagespreise";
  Südpfalz Metall setzt die Preise täglich neu fest; Gold & Silber (DEG e.K.) wirbt ein
  freibleibendes „10 % mehr" nur bei schriftlichem lokalem Konkurrenzangebot; Schrotthandel
  Laroche kalkuliert nach Börsenkurs, der Spitzenpreistext ist ausdrücklich auf 2022
  datiert.
- **Gebühren:** PR Recycling: AGB nennen An- und Abfahrt gesondert abgerechnet, eine
  Sortiergebühr bei Fehlwurf und eine mögliche Vollberechnung bei Storno unter 24 Stunden,
  jeweils ohne Betrag; zusätzlich eine Spende von 10 EUR je Neukunde an einen wohltätigen
  Zweck. MRL Lagerin: Containerstellung kostenfrei, ausdrücklich nur für Metalle, nicht
  für Müll oder Bauschutt. A-B-Recycling: Container 3 bis 40 m³, keine Tarife.
  Südpfalz Metall: Container und Abholung ohne Tarif, Annahme nur mit Termin.
  Kein Dossier nennt eine bezifferte Entsorgungs- oder Demontagetarifliste.
- **Annahmebedingungen:** Südpfalz Metall nimmt verölten Schrott, Späne, Schlacken und
  Rohre mit Glasfaserdämmung nicht an und nimmt nur mit Termin an; A-B-Recycling verwertet
  Altfahrzeuge mit Nachweis, führt aber keine Gebrauchtteile; MRL Lagerin bietet nur
  Abholung, keine Anlieferung, und keine Autoverwertung. Mindestgewichte der Schrottportale
  (Eisen ab 100 kg, NE ab 1 kg) sind Mengen, keine Preise.

## Koordinaten: Klärungsbedarf

- `rp-ludwigshafen-weingartner`: gespeicherter Punkt 49.4866, 8.4181 löst auf
  Industriestraße 2b, 67063 auf, die Zeilenadresse lautet 67059 (Nominatim 49.4814437,
  8.4273481). Punkt nicht angetastet, weil beide Adressvarianten offen sind.
- `rp-koblenz-56073-r-a`: `clear_fields: 'geo'` gesetzt; der Punkt gehörte zu
  „Schrottplatz 1A", Devorastraße 2. Ersatzpunkt nicht gesetzt (nur Verzeichnisbeleg).
- `rp-neuwied-laroche-neuwied-block`: Punkt 50.442, 7.4925 = Rudolf-Diesel-Straße 9
  (Industriegebiet); „Kirchoff 1" ist in OSM nicht mit Hausnummern verzeichnet, ein
  Ersatzpunkt wäre geraten.
- `rp-trier-54294-dres-georg`, `rp-breitscheid-frohlich-breitscheid`,
  `he-wiesbaden-mainz-kastel-metall-rohstoffhandel-lagerin`,
  `he-gieen-wieseck-pr-recycling-sandro-roggendorf`: `clear_fields: 'geo'` gesetzt
  (Punkt lag in 54290 Trier, in Horressen 56410, in Wiesbaden 65185 bzw. in der
  Ostanlage Gießen 35390).
- `rp-hordt-wetzel-hordt`: Punkt 49.1658, 8.3264 löst auf Grünwaldstraße 5 auf, die
  belegte Adresse Wörthstraße 8 liegt rund 240 m entfernt (49.1679559, 8.3268008). Gleicher
  Ort und PLZ, daher nicht zurückgezogen; Kandidat für eine spätere Korrektur.
- `rp-dhron-feuereisen-neumagen-dhron`: Punkt löst auf Konstantinstraße 23 auf, die
  Zeilenadresse ist Hochwaldstr. 33 (Ortsteil Papiermühle). Möglichkeit einer
  Betriebsverlagerung, offen.
- `he-limburg-65549-roth-heinrich`: Punkt löst auf Diezer Straße 69 auf, die Anschrift
  Stephanshügel 14 liegt rund 420 m entfernt; Betrieb ist zudem geschlossen.
- `he-eschborn-65760-pfaff-alexander`: Punkt löst auf Hauptstraße 15 (Eschborn) auf, die
  Kandidatenadresse lautet Am Burggraben 7 c. Kein belegter Annahmepunkt.
- `he-wetzlar-jori-recycling-schrott-wetzlar`: Punkt liegt auf der Bergstraße in Wetzlar,
  die belegte Straße (Siegmund-Hiepe-Straße) hat keine Hausnummer.
- `he-nidderau-ab-recycling-bjoern-asal`: Koordinate neu gesetzt (Hausadresspunkt,
  Gebäudemittelpunkt, kein Anlieferpunkt).

## Merge- und Umbenennungsklärungen für den Owner

- `rp-geinsheim-sudpfalz-metall-geinsheim`: Marken-Namensvetter „Südpfalz Metall GmbH"
  (Janneck Knecht, Rohrbach, HRB 33727, USt-IdNr DE361039244 VIES gültig, Domain seit
  14.11.2024 archiviert) gegenüber dem Geinsheimer Einzelbetrieb Benjamin Glaser (Domain
  bis 07.02.2026 online). Beide bestanden zeitgleich; eine Nachfolge ist nicht belegt, daher
  **keine** Zusammenführung. Ein eigenes Dossier für die Rohrbacher GmbH fehlt im Korpus.
- `rp-mainz-55116-gold-silber-ankauf`: der aktuelle Mainzer Edelmetallhändler
  (SOLIT Management GmbH / GoldSilberShop, Rheinstraße 103, 06131/2674774) hat **kein**
  eigenes Dossier; der Seed (Flachsmarkt 5, DEG e.K.) bleibt davon getrennt. Dossierlücke.
- `rp-trier-54294-dres-georg`: Seed-Name „Dres Georg" entspricht der Verzeichnisform;
  eine Umbenennung nach „Georg Dres Schrott- und Metallhandel" wurde nicht vorgenommen,
  weil keine Betreiber-Primärquelle existiert.
- `rp-neuwied-laroche-neuwied-block`: Namensvariante Denise vs. Macel Laroche ungeklärt;
  der Seed-Name „Laroche Neuwied-Block" wurde nicht umbenannt (Betriebsfirma laut Betreiber
  und 11880: „Schrotthandel Westerwald"). Koblenzer Namensvetter (Roxana Laroche,
  Dominic Larohe) bleiben getrennt.
- `rp-binningen-esper-binningen`: Namensvarianten „Frank und Michaela Esper" gegen
  „Raphael Esper" an derselben Adresse, zwei Rufnummern, offen.
- `he-limburg-65549-roth-heinrich`: Diezer Betrieb (Stephanie Roth eKfr.) und der Limburger
  Standort sind dieselbe Familie; die Diez-Kontaktdaten wurden nicht in den Limburger Seed
  übernommen. Der Seed steht jetzt auf `geschlossen`.

## Hinweise für das Owner-Gate

- Die Seed-Codegen läuft über alle Dossiers fehlerfrei (`crates/ingestion/build.rs`,
  `seed codegen ok`), die build.rs-Unit-Tests bestehen (10/10).
- `cargo test -p schrott-mcp-ingestion seed` schlägt derzeit an einem **fremden,
  nicht zu dieser Welle gehörenden** Zwischenstand fehl: `crates/ingestion/src/traders/
  handlers/mc_schrott.rs:883` verwendet im neuen Testcode `PriceRow`, das dort nicht
  aufgelöst wird (`E0422: cannot find struct, variant or union type 'Row'`). Dieselbe
  Datei und `dossiers/by/by-ichenhausen-baur-bayer-graf-bauer.md` waren bereits vor Beginn
  dieser Welle uncommitted geändert und wurden hier nicht angefasst.
- Keine Produktionsdatenbank wurde beschrieben; der Abgleich erfolgte read-only über
  `sqlite3 "file:/var/lib/schrott-mcp/public.db?mode=ro"`.
