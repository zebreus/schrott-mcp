# Tatsächlicher Upgrade-Seed – 16.10.2026, Check-in 03:00

## Entscheidung

**HOLD für eine uneingeschränkte Freigabe des gesamten Bootseeds von `81a6483208ade84c592e89a8684f56c9774dcea7`.** Nicht nur die zuletzt geprüften 60 Dossiers werden wirksam. Die echte Probe umfasst **715 Bestandsänderungen und 12 Einfügungen**. Davon haben **170 Bestandszeilen insgesamt 464 Änderungen außerhalb von notes/extra_json/updated_at**. Die übrigen 545 sind technisch Notiz-/Metadatenänderungen, keine 545 neu verifizierten Unternehmen.

Die neun neuen GEO-Paare sind im committeten Quellenverlauf konkret begründet und passen numerisch zur tatsächlichen Wirkung. **Keine neun frisch vermessenen Einfahrten und keine Verifikation aller Bestands-Pins.** Die 28 Straßenrücknahmen sind als Rücknahmen unqualifizierter Angaben nachvollziehbar; sie löschen zugleich 14 PLZ, aber keine Koordinaten. Die wichtigsten noch offenen Freigabefragen sind:

1. **AGOSI, Fernau, SM Metals, Graf/Kiel und Willeführ sowie Bausch/Ravensburg:** bestimmte neue Kontakt-/Zeit-/Aktivfelder haben in der gelesenen Akte keine vollständig nachgewiesene README-Ausnahme oder unabhängige Feldbestätigung. Exakte Grenzen unten; keine pauschale Sperre ihrer belegten Identitätsdaten. Gleiches Detailquellengate bei Limekon.
2. **Hark/Hamm:** Die neue historische Adresse Am Ortsgüterbahnhof 9/59063 ist durch den Owner-Nachtrag15.10. mit Behördenbericht und separatem Zertifikat begründet, nicht durch die ältere Hafenkarte widerlegt. F-D als historischer Standort bei pruefung, **kein gegenwärtiger Annahme-/Pinbeweis**. Diese spätere Evidenz überholt den offenen Adressstand13.10.
3. **Legacy-Annahmeflags:** Alle zwölf Inserts erhalten tatsächlich `accepts_dropoff=1`, auch die bewusst unbekannten/ungeklärten Fälle; `accepts_pickup=0` selbst bei positivem pickup_json. Diese Spalten sind kein inhaltlich freigegebener Servicevertrag. Owner muss ihre öffentliche Verwendung/Kompatibilitätsbedeutung ausdrücklich entscheiden, nicht `{}` als komplette Rücknahme aller Annahmezusagen ausgeben.
4. **Keep-Semantik:** Mehrere im Ziel-Dossier zurückgenommene Angaben bleiben tatsächlich in der DB. Besonders Brosda, Papierfritze-Mail und alte positive dropoff_json. Weder Status `pruefung` noch eine leere Dossierzeile entfernt diese Fakten aus SQL-Ergebnissen.
5. **Provenienzverlust:** In 153 Zeilen verschwindet `extra_json.seed_file`. Fachlich kein Kontakt-/Preisverlust, aber eine reale zusätzliche skalare Wirkung, die der Owner akzeptieren oder vor Release adressieren muss.

Dies ist eine **dokumentenbasierte fachliche Freigabeempfehlung mit expliziten Teil-Holds**, keine Produktions-/Installationsfreigabe. Keine Produktivwrites, Deploys, Commits, Seeds, Builds oder Tests ausgeführt. Ein angefragter weiterer Researchagent wurde wegen Subagent-Tiefengrenze abgewiesen; keine zweite erfolgreiche Prüfung behauptet. Ausschließlich diese Ergebnisdatei geschrieben.

## Prüfbasis und eigenständig bestätigte Wirkung

- Arbeits-HEAD zum Prüfzeitpunkt: `81a64832`. Bestehende fremde Änderungen nicht übernommen, nicht zurückgesetzt und nicht freigegeben.
- Primäre Wirkungsartefakte: `/root/Documents/checkin-20261016-0300-release-1wz2obd5/seed-rehearsal-before.json`, `seed-rehearsal-diff.json` und `seed-rehearsal/public.db`. Die private DB wurde ausschließlich mit SQLite-URI `mode=ro` und `PRAGMA query_only=ON` gelesen. Kein Zugriff auf eine produktive DB notwendig.
- **Frisch nachgerechnet:** alle Vorherzeilen gegen alle Spalten der privaten Nachher-DB verglichen. Ergebnis exakt gleich `changed` des Diff-JSON, hinzugefügte Slugmenge exakt gleich `added`; 3881 → 3893, 715 geändert, 12 neu, 0 entfernt. Keine Stichprobenhochrechnung.
- SHA256 vorher JSON: `407688a00bcd26abc3be0ce0c21814040643899fadd085bde6b93e1376b03d05`; Diff JSON: `145028e03e0786871be9175f2e8f517e317d6e4e4f59a08c8414cdc4d5cb2e1a`.
- **Alle 3892 Dossierdateien des sauberen Archivs frisch bytegenau mit `git show HEAD:<Pfad>` verglichen: 0 Abweichungen.** Zielskalare aus flacher Frontmatter gelesen und gegen die wirksamen nichtleeren Felder aller 715 geänderten Zeilen verglichen: 0 Abweichungen. Der erste Versuch mit allgemeinem PyYAML scheiterte an einem unquotierten Doppelpunkt in bestehender Frontmatter; danach die in `build.rs` dokumentierte flache Schlüssel-/Fortsetzungsstruktur verwendet. Das ist kein neuer Rust-Codegen-Lauf und kein Bytevergleich des kompletten eingebetteten JSON.
- Eine DB-Zeile ohne Ziel-Dossier bleibt erhalten: `st-sangerhausen-mainmetall-grohandelsges`. Daher 3892 Seed-Dossiers, aber 3893 DB-Händler nach Upgrade. Keine neue Dublette aus diesem Zahlenunterschied ableiten.
- Nachher-DB: 19.805 Preisbeobachtungen und 2.426 Current-Pointer. **Keine Preisänderungen und zweiter Seed 0 Writes sind Nachweise des vorgegebenen Probelaufs**, hier nicht durch erneutes Seeden reproduziert. Die beiden Trader-JSON allein enthalten keinen Vorher-Preis-Snapshot.
- Quellenstandard: `README.md:183–202` und Betreiberkettenabschnitt; Wirkungsvertrag `docs/trader-seed.md:7–90`, `crates/ingestion/src/seed_traders.rs:333–448`, `crates/ingestion/build.rs:100–149,161–193`.
- Fachbelege unten sind **selbst gelesene committete Dossiers/Ownerhistorien**, nicht heute erneut abgerufene externe Webseiten, PDFs, Register oder OSM-Objekte. Ihre dort zitierten URLs/Abrufdaten sind nachvollziehbare Vorbelege; keine neue Tatsachenprüfung dieser externen Quellen behauptet. Gegenwärtige Erreichbarkeit/Zertifikatsgültigkeit über die dokumentierten Angaben hinaus nicht getestet.

### Vollständige Spaltenbilanz der 715 Bestandszeilen

| Feld | geänderte Zellen |
|---|---:|
| notes / extra_json / updated_at | je 715 |
| status | 83 |
| street / postcode | 61 / 54 |
| phone / email | 49 / 21 |
| description / website_status | 39 / 39 |
| website / opening_hours | 32 / 22 |
| name / city | 19 / 17 |
| lat / lon | 9 / 9 |
| trader_type / state | 4 / 1 |
| dropoff_json / pickup_json | 3 / 2 |

Keine weiteren Bestands-Spaltenänderungen. Insbesondere keine Änderung an first_seen_at, country, Mengenlimits, certifications, accepts_dropoff, accepts_pickup oder website_checked_at. Das ist ein Snapshotbefund, kein Versprechen für eine später weiter angereicherte Produktion: der Upsert übergibt min/max=None.

Die 83 Statusänderungen: 70 aktiv→pruefung, 10 pruefung→aktiv, 1 unbekannt→aktiv, 1 unbekannt→pruefung, 1 geschlossen→pruefung. Letztere ist DAS Recycling/Lauta: Rücknahme einer unbelegten Schließung, keine bestätigte Wiedereröffnung.

## Neun GEO-Paare: einzeln geprüft gegen Akte und Wirkung

`F-D` bedeutet im Folgenden: auf Basis der gelesenen dokumentierten Belegkombination fachlich vertretbar, **kein neuer externer Livebeweis**. Die Tabellenwerte sind tatsächliche DB-Werte, nicht nur Wunschpayloads. Alle übrigen Änderungen der jeweiligen neun Zeilen sind durch diese Tabellenzeile mit umfasst, mit den genannten Grenzen.

| Slug | tatsächlich vorher → nachher (lat, lon) | dokumentierter Bezug / Entscheidung |
|---|---|---|
| `bw-plochingen-schrott-und-metallhandel-m-kaatsch` | 48.7107,9.4195 → 48.7153089,9.4058424 | F-D: Am Nordseekai 8/73207, Telefon/Mail; Betreiber + HRB210181, OSM way254687352, 12.10. Gebäude-/Adresspunkt. Nordseekai nur Stahlschrott; NE/Rheinkai getrennt, Zeiten zu Recht nicht vereinheitlicht. |
| `bw-ravensburg-bausch-entsorgung` | 47.77813333333334,9.595866666666666 → 47.7928391,9.6111644 | GEO F-D als dokumentierter Gebäude-/Adresspunkt: Bleicherstraße35/88212, way193316231,12.10., nicht Eingang. **Teil-H für Adresse/Kontaktdetail/Aktivierung nach README:** lokale Nummer -9, Mail/Zeiten/Beschreibung, pruefung→aktiv stammen in dieser Akte aus Betreiber-Hofseite/Impressum; unabhängige Identitäts-/Adressbestätigung oder vollständig dokumentierte Ausnahme fehlt. Geometrische Adressübereinstimmung mit OSM ersetzt keinen zugelassenen Identitätsbeleg. |
| `by-traunstein-jakob-schaumaier-nachf` | 47.8682,12.6433 → 47.8882515,12.6271388 | F-D: Industriestr.12/83278, Typ autoverwertung, Website/Telefon/Zeiten/Beschreibung, unbekannt→aktiv. HRB1086 und Zertifikatsseite bis28.02.2027 dokumentiert; way365190932, 12.10. Firmen-/Adresspunkt. Industriestr.24 und deren Gebühren nicht übertragen. |
| `sn-torgau-scholz-recycling-betrieb-torgau` | 51.5602,12.9962 → 51.55352,13.00196 | F-D: Betreiber-JSON `/standorte-json-nicht-loeschen/`, Objekt ort=Torgau, 15.10.; Am Wasserturm3/04860, lokale Kontakte/Zeiten. Direkter Betreibermarker, nicht unabhängige Einfahrtsmessung. Dritter Samstag als Sonderzeit erhalten. Unveränderter Typ autoverwertung bleibt ungeklärt, nicht neu bestätigt. |
| `sn-halsbrucke-muldenhutten-remondis-industrie-service-nl-freiberg-m` | 50.95,13.35 → 50.901934,13.3823251 | F-D: Muldenhütten10/09599, Telefon/Website/B2B-Beschreibung; way117118086 und Owner-Nachtrag12.10. OSM-Altname Fehr nicht verschwiegen; Übernahmespur dokumentiert. Gebäudeschwerpunkt, status pruefung, kein privater Schrottankauf belegt. |
| `bw-oberderdingen-cr-recycling-aydemir-cinar` | 49.0656,8.8031 → 49.0792416,8.7783427 | F-D: Robert-Bosch-Ring9/75038, Website/Beschreibung, pruefung→aktiv als Recyclingbetrieb. Betreiber + Stadt-Firmenpark, way1204307532, 12.10.; Flächenschwerpunkt. Kontaktkonflikte nicht gefüllt, Privat-/Vergütungszusage offen. |
| `mv-gustrow-18273-fahrzeugtechnik-und-karosseriebau-schwit` | 53.7972,12.1734 → 53.7931548,12.2242372 | F-D: Primerburg3/18273, city Güstrow, Telefon/Website/Werkstattbeschreibung. Betreiber + Creditreform, way169451888, 12.10.; Gebäudeschwerpunkt. Werkstattbeleg ist kein Demontage-/Ankaufsbeleg; pruefung bleibt. Email/Zeiten im Ziel clear-markiert, hier vorher schon leer. |
| `rp-speyer-hohl-containerdienst` | 49.3208,8.4311 → 49.3008874,8.4467507 | F-D mit Telefonvorbehalt: Alte Rheinhäuser Straße15/67346, Website/Mail/Zeiten/Beschreibung, pruefung→aktiv als Wertstoffhof. Betreiber + HRB53219, way287560070, 12.10.; Gebäude-/Adresspunkt. Abweichende GS-Nummer als Lead, Betreibertelefon06232 810800 gewählt. Container-/Betonkiespreise sind keine Ankaufpreise. |
| `st-schonebeck-scholz` | 53.0595,12.2331 → 52.0254671,11.6917707 | F-D: Wilhelm-Dümling-Str.10/39218, HTTP-Website; Betreiber + Stadt, node8661973565/way538349970, 12.10.; POI, nicht Tor. Keine Kette zur Scholz-Gruppe konstruiert, status pruefung; Kontakte/Zertifizierung nicht freigegeben. |

Belegpfad je Slug: `dossiers/<erstes Slugsegment>/<slug>.md`, jeweils committete Koordinatenprüfung/Owner-GEO-Nachtrag und zugehörige Recherche. Ältere Formulierungen „keine GEO geändert“ sind durch spätere GEO-Nachträge zeitlich überholt, nicht Beschreibung der Upgrade-Wirkung. Keine erneute externe Koordinatenabfrage in diesem Review.

## Zwölf Inserts: tatsächliche Vollzeilen statt bloßer Neuanlage-Liste

Alle zwölf haben tatsächlich lat/lon=NULL, country=DE, min/max=NULL, first_seen_at neu, website_checked_at leer. Zertifizierungen werden nur bei Hauser und Giesen-Wekos eingefügt; die übrigen `[]`. **Für alle gilt der oben genannte Legacyflag-Hold.** Die fachlichen Bewertungen darunter beziehen sich auf die kuratierten Felder/JSON, nicht auf eine ungeprüfte Interpretation der Legacyflags.

| Slug | tatsächlicher Kern / fachliche Entscheidung |
|---|---|
| `bb-frankfurt-oder-rpr-gmbh-recycling-park-rohstoffhandel` | F-D: aktiv; Georg-Richter-Str.13/15234 Frankfurt(Oder), Tel0335 4000099, info@rpr-recyclingpark.de, Mo–Fr07–16, RPR-Root. Betreiber+Northdata HRB21955, Eintragung07.01.2026. Nachfolgerdatum/Priebels-Übergang nicht reproduziert, aber nicht als Merge importiert. dropoff/pickup `{}`; kein Annahme-Ja aus fehlendem Nachfolgeakt ableiten. |
| `bw-bretten-hauser-recycling-transporte` | F-D: aktiv; Gewerbestr.59/75015 Bretten-Gölshausen, 07252 95545, info@hauser-gmbh.net, Mo–Fr08–12/13–16:30, Sa08–12, `/leistungen/`. HRB240545, Betreiber/Register und dokumentiertes Original-DEKRA-PDF bis28.07.2027; Zertifizierung entsprechend eingeschränkt importiert. Beide Service-JSON `{}`. |
| `by-hirschaid-hermann-korn` | F-D ausschließlich als pruefung: Hermann Korn e.K., Jägerstr.2/96114, h-korn.de; Kontakte/Zeiten leer, JSON `{}`. HRA8076 kongruent; Zertifikat seit29.02.2024 abgelaufen, aktueller Register-/Betriebsfortbestand offen. Nicht Korn GmbH Burgpreppach, nicht Kauder Jägerstr.8. Keine Aktivfreigabe. |
| `ni-wedemark-bissendorf-recycling` | F-D: aktiv; Auf der Haube5/30900 Wedemark, +49 5130 3767666, bissendorfrecycling@web.de, Mo–Fr08–16/Sa zu, `/angebot/`. HRB215994; Akte mit amtlichem AD14.10., Ownerprüfung und ausdrücklich belegtem Platzbesuch. JSON Anlieferung/Abholung mit unbekannten Details; abweichende Footer-Mail dettmerrecycling nicht importiert. |
| `nw-doerentrup-limekon` | **Teil-HOLD:** Identität/Adresse/Website/Beschreibung F-D (Bahnhofstr.23/32694; HRB7953, Betreiber+Northdata+GLEIF). +49 5263 9561906, info@limekon.de, Mo/Di09–17/sonst Termin, Betriebsferien und positive Service-JSON stammen aus Betreiberquelle. Dossier erklärt ausdrücklich keine Betreiber-Ausnahme wegen fehlender HRB im aktuellen Impressum; unabhängiger Feldbeleg oder ausdrückliche Owner-Abweichungsentscheidung fehlt. Versand7/10EUR und Abholung200/300kg/10000EUR nur als Konflikt, keine einheitlichen Schwellen importiert. |
| `nw-dorsten-asm-recycling` | F-D nur als pruefung/Markenlead: ASM Recycling, Dorsten, asm-recycling.de; Straße/PLZ/Kontakte/Zeiten leer, JSON `{}`. Rechtlicher Betreiber offen, Impressum404, sichtbare Nummer versus tel-Link widersprüchlich. Keine Aktiv-/Annahmestellenfreigabe. Typ schrottplatz bleibt Werbekategorie, nicht verifizierter Hof. |
| `nw-dusseldorf-euromet-structured-rec` | F-D als B2B-Handelsgesellschaft aktiv: Dreischeibenhaus/40211, euromet-structured.de, Kontakte/Zeiten leer/JSON `{}`. HRB107685 Düsseldorf kongruent. HRB17773/Mobilmarke und unadressierter Dortmunder Platz ausdrücklich nicht übertragen. Kein öffentlicher Hof am Geschäftssitz. |
| `nw-kleve-pietsch-rohstoffe` | F-D: aktiv; Lise-Meitner-Str.24–30/47533, 02821 89 999-33, info@pietsch-rohstoffe.de, Mo–Fr07–17/Sa08–12. HRB1861, Betreiber/Register und Originalzertifikat bis30.06.2027 dokumentiert. dropoff sauber sortierte Metalle/gewichtsbasiert/Tagespreis; pickup `{}`. Kein Merge mit Lichtenow. |
| `nw-kreuztal-rohstoffhandel-hein` | F-D nur pruefung: Kreuztal/rohstoffhandel-hein.de, Straße/PLZ/Kontakte/Zeiten leer/JSON `{}`. Büro Hilchenbach versus Platz Austrasse28a Kreuztal-Ferndorf nicht vermischt. Unabhängiger Platzbeleg/aktuelle Telefonbestätigung fehlen; weder Mario Deutsch noch Heinrichs Gelsenkirchen zugeordnet. |
| `nw-menden-sk-metallhandel` | F-D sparsam aktiv als Handelsunternehmen: Fröndenberger Str.287/58708, sk-metallhandel.de, HRA5675 kongruent. Kontakte/Zeiten leer, JSON `{}`; kein Impressums-/Aktualitäts-Ausnahmebeweis, daher diese Detailfelder bewusst nicht importiert. Seit1987 nur Selbstdarstellung, nicht KG-Gründungsdatum. |
| `nw-neuss-giesen-wekos` | F-D: Duisburger Str.15c/41460, HRA6020, Betreiber-Impressum+Northdata+GLEIF, aktuelle Standort-/Annahmebedingungen und ESN-PDF dokumentiert. Kontakte02131 1619-10/zentrale@giesen-wekos.de, Mo–Do06–16:30/Fr06–14:30 und vorsichtige Service-JSON durch diese Aktenkombination gestützt. Zertifikatsfeld HRB11146 betrifft die Komplementärin und wird ausdrücklich nicht als KG-Identitätsbeleg verwendet; dieser bekannte Konflikt ist allein kein neuer Aufnahmeblocker. Zertifikat bis30.04.2027 importiert; Ausschlüsse haben Vorrang vor AVV-Katalog, Privatannahme ausdrücklich unbestätigt. |
| `nw-sundern-bierhoff-metallhandel` | F-D: aktiv; Illingheimer Str.39/59846, 02393 483, info@bierhoff-metall.de, Mo–Do14–17/Fr14–16/Sa Termin; Betreiber+HRA5623, datierter Aktualitätsbefund09.10. dokumentiert. Privat-dropoff mit Ausweis, gewerblicher pickup48h/0,5–10m³. Eintägige Schließung09.10. nicht als Dauerschließung importiert; keine Privatabholung aus Gewerbezusage. |

Belege: jeweilige committete Dossiers, vor allem Korrekturen10.10. und Aufnahmeprüfungen14.10.; `research/checkin-20261009-owner-gate-neue-dossiers.md`, `research/checkin-20261010-seed-clear-independent-review.md`, `research/feedback-20261014-{hauser,bissendorf,limekon,giesen-wekos,korn}-admission.md` als dort referenzierte Ausgangspunkte. Hier wurden die tatsächlichen Dossiertexte und Nachherzeilen gelesen; nicht jeder dieser referenzierten Zusatzberichte erneut vollständig geprüft. Frühere breitere Bestandsabgleiche mit 3923–3926 lokalen Dossiers sind historische Berichte, nicht die hier frisch bestätigte saubere 3892er-Zielmenge.

## 28 Adressleerungen: genaue Rücknahmen

Alle nachfolgenden **street→leer** sind in Wirkung und Dossier kongruent. F-D für die Rücknahme, **nicht** für Schließung, Wegfall des Unternehmens oder Verifikation des erhaltenen Pins. `+PLZ` bedeutet zusätzlich gespeicherte PLZ→leer. Weitere scalar-Zellen der jeweiligen Zeile werden mitgenannt. Beleg jeweils committete Timeline unter demselben Slug/Pfad.

| Slug | zurückgenommene Straße / PLZ | übrige tatsächliche Wirkung / offener Fall |
|---|---|---|
| `by-ichenhausen-michael-hammerschmidt` | Karl-Königsdorfer-Str.25A +PLZ89335 | phone/email leer, aktiv→pruefung; Verzeichniskonsens war kein Identitätsbeweis. |
| `by-munchen-r-muller` | Georg-Thiele-Str.2 +PLZ81829 | phone/email/Zeiten/description leer, aktiv→pruefung; GmbH/e.K.-Zuordnung offen. |
| `he-darmstadt-wixhausen-der-zerseeger-schrotthandel` | Am Sportplatz12 | phone leer; Levy-Nachbarschaft keine Identitätsbrücke. |
| `he-erlensee-ruckingen-auto-s3-autoverwertung` | Schubertstr.2 | phone leer; Betreiber/Website-Ziel/aktuelle Annahme offen. |
| `he-alsfeld-schrotthandel-entsorgung-delis` | Bürgermeister-Haas-Str.18 +PLZ36304 | phone/Zeiten leer; Soldanstr.-Konflikt nicht gelöst. |
| `he-heidenrod-schrottmeister-rhein-main` | Napoleonstr.19 | gleichadressierter Schrott-Experte/Zohir Boulahoual nicht als Seedidentität nachgewiesen. |
| `he-dreieich-stefan-schramm` | Herrnroth11 +PLZ63303 | phone leer, aktiv→pruefung; tellows/Cylex keine qualifizierte Doppelquelle. |
| `ni-osnabruck-reinhard-peters-schrottplatz` | Kleiner Muskamp2A +PLZ49078 | phone leer, aktiv→pruefung; Hausnummer2 versus2A nicht scheinbereinigen. |
| `nw-aachen-elektroschrott-aachen` | Elektroschrott Aachen Aachen Robensstr.20 | kein Adressfill aus Express Entrümpelung oder EAS Herdecke. |
| `nw-bochum-schrottankauf-klar` | Schrottankauf Klar Bochum Robertstr.1 | Robertstr./Herner Str./Telefon-/Domainkonflikt bleibt. |
| `sl-merchweiler-schrott-weiler-matthias-wahrsch-gleiche` | Eisenbahnstr.1a | Name ohne spekulative Familiennotiz, aktiv→pruefung; Bärbel/Matthias nicht gemergt. |
| `by-munchen-hoffmann-karl-heinz` | Rappenweg162 | ausschließlich Verzeichnisleads, keine neue Zuordnung. |
| `by-augsburg-reinhardt-robert` | Kobelweg78a | Kobelweg78/Dr.-Nick-Str.17 nicht als Umzug belegt. |
| `he-spangenberg-34286-heinrich-kleinschmidt-rohprod` | Untergasse5 +PLZ34286 | phone leer, aktiv→pruefung; kommunale Inhaber-/Standortprüfung offen. |
| `he-eschborn-65760-pfaff-alexander` | Am Burggraben7c +PLZ65760 | phone leer, Typ mobil→sonstige; frühere Mobilitätsvermutung zurückgenommen. |
| `he-babenhausen-64832-kohlmann-frank` | Am Fuchsberg8 +PLZ64832 | phone leer, city ohne PLZ; Betreiber-/Annahmebeleg offen. |
| `he-bad-emstal-34308-pohl-anita` | Wichdorfer Str.2 +PLZ34308 | phone leer; Anita/Arthur und Felsberger Namesake nicht identifiziert. |
| `ni-buchholz-i-d-n-21244-all-in-schrott-abholung` | Steinbecker Str.35 +PLZ21244 | phone leer; identische Nummer in Herne beweist keine Buchholzer Filiale. |
| `nw-bunde-janus` | Mindener Str.74 +PLZ32257 | phone leer; Niederfeldstr.-Lead/GoLocal-Profil keine Umzugskette. |
| `nw-leichlingen-muller` | Metzholz68 +PLZ42799 | phone leer, aktiv→pruefung, description mit Prüfvorbehalt statt Axel-Inhaberbehauptung. HRB48784/99637 nicht vermischt. |
| `st-bernburg-heine` | Schachtstr. | aktiv→pruefung; Friedenshaller Ring4 nur Betreiber-Einzelbeleg, kein Ersatzfill. |
| `sh-rellingen-25462-klass-m-autoverwertung-und-abschleppdien` | Adlerstr.36 | aktiv→pruefung; 2018er Bewertung keine heutige Annahme. |
| `bw-tauberbischofsheim-distelhau-griesbaum-roland-schrotthandel` | Bundesstr.13 | **PLZ wird stattdessen neu97941**, city ohne PLZ; nur Ortsnormalisierung, kein Roland-/H.-Griesbaum-Standortbeweis. |
| `by-growallstadt-63868-schrott-rosel` | Frankenstr.30 | city ohne PLZ; Betreiber/aktuelles Gewerbe offen. |
| `by-sennfeld-97526-mertel-autoverwertung` | Schweinfurter Str.6 +PLZ97526 | phone leer; aktuelle Demontagezulassung/6 versus6–8 offen. |
| `by-wurzburg-versbach-97078-pfeifer-rainer-schrotthandel` | Brunnfloßgasse12 +PLZ97078 | phone leer; Rainer/Christian/Jürgen nicht vermischt. |
| `rp-kirrweiler-riehm-kirrweiler` | Riehm Kirrweiler Kirrweiler Marktstr.42 | keine scheinverifizierte Marktstr.42 und kein Umadressieren nach Edenkoben. |
| `rp-gollheim-ayan-gollheim` | AYAN Göllheim Göllheim Königkreuzstr.13 | Resul Ayan/Börrstadt kein Nachweis Göllheim/Bechtolsheim. |

Historische Notizen „DB-Adresse/GEO nicht verändert“ bei einzelnen Owner-Clears betreffen damaligen read-only-Zugriff oder die Enrichment-Direktive allein. **Das kommende vollständige Upsert leert trotzdem die Kernadresse**, wie jetzt tatsächlich nachgewiesen. Alle 28 behalten ihre bisherigen GEO mangels Zielpaar; keine kartografische Freigabe aus diesem Abschnitt.

## Übrige 75 Zeilen mit skalaren Änderungen

Abdeckung: 9 GEO + 28 Adressrücknahmen + diese75 + die58 reinen Statuszeilen im nächsten Abschnitt = **170**, disjunkt. Alle genannten Felder sind die tatsächlichen Änderungen; exakte Vorher-/Nachherstrings stehen im oben gehashten Diff. F-D gilt nur für die geänderten kuratierten Fakten, nicht automatisch für alle unveränderten Bestandsfelder. `H` ist eine konkrete offene Freigabe, nicht bloß ein allgemeiner Recherchewunsch.

| Slug | geänderte Felder / Entscheidung und Beleggrenze |
|---|---|
| `bw-pforzheim-agosi-ag` | description,phone,email: **H Kontakte** +49 7231 960-0/info@agosi.de; Akte12.10. enthält Betreiber-Kontakt-/Kursseiten, aber keinen aufgelösten HR-/Ausnahmebeleg gegenüber der expliziten Altblockade. Description als attribuierte Geschäftsbeschreibung F-D; keine einzelne Straße zurecht gewählt. |
| `bw-lorrach-mv-lorrach` | street,postcode,phone,email,opening_hours,description,dropoff_json,pickup_json: F-D; HRB412919 + Standort + aktuelles Zertifikat2026-0191 bis13.10.2027 dokumentiert. Straßenvarianten44–50/46–50/48–50 offen, keine Einzelhausnummer/GEO behauptet. |
| `bw-neu-ulm-russ-entsorgung` | name,street,postcode,state,phone,email,opening_hours: F-D; HRA13628, Otto-Hahn26/89231, **BW→BY** bei stabilem bw-Slug; aktuelle Zertifikatsprüfung im Review15.10. dokumentiert. Kein Umzug, kein Ankauf-Ja, pruefung bleibt. |
| `by-geisenhausen-lorenz-wittmann` | website,opening_hours: F-D; HTTPS und Zentrale-Zeiten, Owner-Gate09.10./HRB1526. Keine Warenannahmezeiten oder Privat-Einzelankäufe daraus. |
| `by-dietersburg-eibl` | phone→leer: F-D expliziter Clear12.10.; Lead01512 3518337 nicht qualifiziert. |
| `by-burgpreppach-korn` | name→Korn GmbH: F-D; Gemeinde + HRB3319; nicht Hermann Korn/Hirschaid. |
| `be-marzahn-hellersdorf-papierfritze-schrott-altpapierankauf` | phone-Präfix entfernt,description: F-D; Paletten-/Gitterboxenankauf, HRB233965B. Vorhandene Zweitmarken-Dublette offen, kein zweiter Händler verifiziert; leere email im Dossier löscht gespeicherte info@papierfritze.de nicht. |
| `be-lichtenberg-p-m-metall-schrott` | status unbekannt→pruefung,description: F-D als historischer Lead/Wohnbaukonflikt, keine Schließungsbehauptung. |
| `hb-ronnebeck-altmetall-bunker` | website,website_status,status→pruefung: F-D nur als Historien-/Betreiberlink; Heidlerchen-Rönnebeck nicht als aktueller Hof bestätigt, aktueller Auftritt nennt vorläufig keinen Metallankauf. |
| `hh-wilhelmsburg-hafen-retraco` | street,postcode,city,website_status: F-D; RETRACO/HRB136833, AGB-PDFs/Adresse; city Hamburg statt Quartiersbezeichnung. Kein aktueller zertifizierter Publikumshof aus altem AGB-Stand. |
| `he-fulda-mittelrode-bohm-michael-schrotthandel` | phone→leer: F-D Clear12.10.; 06619015432 nur Lead. |
| `he-ginsheim-gustavsburg-tsr-deutschland-ginsheim-gustavsburg` | street,postcode,phone,opening_hours,description,website,website_status,status→aktiv: F-D, Owner16.10. + Release-Review mit Original-Efb/Standortdetail. Landdammstraße15, Pausen erhalten. GEO49.9711/8.3453 ausdrücklich nicht geprüft. |
| `he-dietzenbach-b-samuel-schrottabholung` | website_status tot→unbekannt: F-D Unsicherheitskorrektur; Transportfehler kein Domain-/Betriebstod. |
| `he-lampertheim-a-s-brito` | description: F-D Paletten-/Gitterboxenhandel; Betreiber+HRB62115 dokumentiert. Keine Übertragung von Metallannahmezeiten auf Ladungsträger. |
| `he-weiterstadt-grafenhausen-kunstler-recycling` | website_status tot→unbekannt: F-D; kein zugeordneter Domain-Tod. RK Darmstadt nicht als Weiterstadt übernommen. |
| `ni-hannover-n-rohrmeyer-schrott-metallhandel` | status→pruefung,description: F-D; Tischbeinstr.-Lead nicht neu bestätigt; Gewerbeauskunft noch nicht eingeholt. |
| `ni-wietze-gebruder-zimmermann-recycling` | street,postcode,website,website_status: F-D für DEA-Str.17/29323/Domain; Betreiber + unabhängig ausgestellte TÜV-Urkunde dokumentiert, aber seit11.08.2026 abgelaufen. Keine aktuelle Zertifizierung; Detailkontakte/-zeiten zu Recht nicht gefüllt. |
| `ni-nienburg-fernau-container-schrott-metall` | name,street,postcode,city,phone,email,website,website_status,opening_hours,description: **teil-H**; Identität/Adresse/Ort Neustadt statt Servicegebiet Nienburg F-D über Betreiber+HRB110122. Neue Kontakte05032 4683/info@fernau-containerdienst.de und Stunden nur Betreiber; dokumentierte aktuelle Ausnahme (alle Kriterien) oder unabhängige Felddoppelbelege fehlen. Alter Pin bleibt trotz Ortswechsel erhalten. |
| `ni-cuxhaven-machulez-m-cycle-einziger-schrottplatz-c` | name,street,postcode: F-D; M-Cycle/Gruppe statt Werbeaussage einziger Platz; Baudirektor-Hahn38/27472 über Betreiber+Herbert Machulez HRB110098/Zertifikat. Nicht Containerdienst-HRB110233 umadressiert. |
| `ni-hambergen-zinnrecycling-und-metallhandel-ramm-zinn` | opening_hours: F-D; Warenannahme08–11:30/13–16:30, keine unbelegten Samstagsstunden; Kundenverkehr bis16.10. geschlossen/Pakete ausgenommen, Wiederöffnung19.10. als Betreiberankündigung, danach erneut prüfen. |
| `ni-northeim-groschel-recycling-container` | name,street,postcode,phone,email,website,website_status,opening_hours,description: F-D für heutigen Resebeck-Standort laut Betreiber + dokumentiertem bvse-Zusammenhang; keine exakte rechtliche Umfirmierung behauptet. Am Mühlenanger46, keine Göttinger Samstag-/Altautoleistungen. Aktuelle Register-/Detailausnahme nicht frisch extern nachgeprüft; Owner-Vorbeleg akzeptiert, kein GEO-Abschluss. |
| `ni-uberregional-alba-gruppe-mehrere-ni-standorte` | website_status→aktiv: F-D erreichbar dokumentierter Gruppenlink, kein Sammel-Hofbeweis. |
| `ni-surwold-emsland-metallhandel-hinrichs-elmh` | website_status tot→unbekannt: F-D; TLS/Redirect-Probleme nicht Löschung. |
| `ni-region-hannover-hildesheim-schrotthandel-f-weber-ort-n-a` | phone→leer: F-D; 0173 6115144 nur Portal, Schandelah/Hannover-Konflikt offen. |
| `nw-nrw-weit-schrott24-metaloop-online-portal` | description,dropoff_json,pickup_json: F-D Rücknahme zu `{}` wegen Betreiber-/Adress-/Insolvenzkonflikt; **Legacy-accepts_dropoff bleibt1**, keine universelle Rücknahme aller Flags behaupten. |
| `nw-hamm-heinrich-hark-ndl-hamm` | street,postcode,status: F-D **historischer Standort Am Ortsgüterbahnhof9/59063** bei pruefung. Entscheidender späterer Owner-Nachtrag15.10./`research/checkin-20261015-1930-hark-review.md`: behördliche Vor-Ort-Inspektion27.05.2025/Bericht24.07.2025 plus separates ESN98-030085(25), abgelaufen31.08.2026. Hafenstraße110 ist ein eigener zusätzlicher Standort; weder Hafenkarte noch Sitzverlegung beweisen Umzug. Aktueller Fortbetrieb/Annahme und Pin51.2629/7.7239 bleiben ungeklärt, aber historische Adresse nicht als unbelegten Neufill sperren. |
| `nw-bestwig-tital-howmet-aerospace` | street,postcode,description: F-D Werk Kapellenstraße44/59909 über Howmet+HRB8889. Hersteller, kein öffentlicher Schrottankauf. |
| `nw-kevelaer-berk-recycling` | street,email,opening_hours,phone: F-D Owner14.10.; HRB18672, Standort-/Sitemap-Aktualität explizit akzeptiert. Nur Schlussklammer an Telefonnummer entfernt. PLZ47624/47625 bewusst offen; GEO51.58322/6.24828 ungeprüft. |
| `nw-borken-bernhard-stienemann` | opening_hours,description: F-D Pausen und Ladungsträgerhandel; Betreiber+HRA3518 dokumentiert; keine Ankaufkriterien pauschalisiert. |
| `nw-kerpen-sindorf-lhonneux-containerdienste` | name,street,postcode,website,website_status: F-D aktuelle HRB120852-Gesellschaft/12–14; Betreiber+Register+ZER-PDF. Kein geprüfter rechtlicher Übergang vom Einzelunternehmer, keine Detailkontakte/-zeiten ohne Ausnahme eingefüllt. |
| `nw-brakel-schrotthandel-kass` | website_status tot→unbekannt: F-D alte Bindestrichdomain unbrauchbar, neuer Domainkandidat kein pauschaler Totbefund. |
| `nw-munster-metall-schrottagentur-munster` | postcode,website,website_status: F-D HRA8938/48159, aktuelle Domainidentität. Straße143/143c, aktueller Betrieb/Detailkontakte offen; kein André-Wielers-Merge. |
| `nw-dusseldorf-hermann-arend` | website,website_status,description: F-D **Kandidaten-/Historienlink** und konservative Beschreibung, nicht aktuell erreichbarer oder rechtsidentischer Hof verifiziert. `unbekannt` passend zu fehlgeschlagenen Direktabrufen; keine neuen Kontakte. |
| `rp-worms-mainz-mohr-schrotthandel-mobil` | website: F-D Domainroot statt Worms-Landingpage; kein Wormser Betriebssitz. |
| `sl-homburg-loacker-saar-recycling` | street,postcode,description: F-D An der Remise20/66424 über Betreiber+HRB3724, Umfirmierung2018. HomburgII getrennt, keine Privatannahme-/GEO-Freigabe. |
| `sl-saarbrucken-umland-schrottjungs-mobil` | description: F-D Rücknahme pauschal kostenlos/Barauszahlung; Einsatzgebiet, kein SL-Hof. |
| `sl-lebach-autoverwertung-volz-renate-volz` | description: F-D heutige Anerkennung zurückgenommen; eigene Anzeige2002 ist historischer Tätigkeits-, kein heutiger Anerkennungsbeleg. |
| `sn-online-schrott24-metaloop-liz-altmetallhandler` | website_status,description: F-D als erreichbares Onlineangebot/Partnerlogistik; **kein bestätigter sächsischer Partner/aktueller Abwicklungsstatus**, Konflikt analog NRW bleibt. |
| `st-querfurt-tsr-recycling-standort-querfurt` | name,street,postcode,phone,email,website,website_status,opening_hours,description: F-D aktueller TSR-Standort Johannes-Schlaf17/06268; HRA19743/Betreiber + datiertes A|U|F2026. Älteres Efb seit28.08.2026 abgelaufen, nicht aktuell zertifiziert nennen; lokale Mail aus Token, keine Zentrale. |
| `st-halle-schrott24-metaloop-online-ankauf-partner` | description: F-D nur Plattform/Partneradresse nach Verkauf; kein lokaler Hof, kein Hallenser Zahlenkurs. |
| `sh-kiel-graf-abfallbeforderung-und-entsorgung` | name,street,postcode,phone,email,website,website_status,description: **teil-H** Telefon0431 52 16 18/info@graf-entsorgung.de: Betreiber+HRA12865KI belegen Identität/Adresse, aber aktueller Ausnahmebeleg für eigene Kontaktdetails nicht vollständig dokumentiert. Name/Adresse/Website und eingeschränkte Entsorgungsbeschreibung F-D, status pruefung bleibt. |
| `sh-neumunster-tsr-recycling-nl-neumunster` | name,street,postcode,phone,email,opening_hours: F-D; aktueller TSR-Deutschland-Standort, HRA19743/Register+GLEIF, Standort-Efb/ISO/A|U|F im Owner14.10. dokumentiert. Keine bloße Umfirmierung Recycling→Deutschland; neuer Name beschreibt heutigen Betreiber, nicht vollständigen Übertragungsvertrag. GEO54.082725/9.988375 ungeprüft. |
| `sh-lensahn-milon-sonke-schrott-u-autorecyclinghof` | name,postcode: F-D e.K.-Normalisierung/23738 über Landesportal+HRA2053OL; kein heutiger Öffnungs-/Zertifikatsbeweis. |
| `bw-kirchheim-unter-teck-73230-bora-metallhandel` | city,postcode: F-D syntaktische PLZ-Trennung, keine Wiederbelebung der2025 gelöschten HRB746956. |
| `bw-karlsruhe-metall-service-pedack` | name,trader_type,phone,email,website,website_status,opening_hours,status,description: F-D Owner15.10., HRB110265/konkrete Umfirmierungsbekanntmachung2022, Standort und datierter Pflegehinweis. Pedack→CRONIMET CREMETAL, keine Holding-Kontakte; Vergütung ab5EUR ist keine Mindestmenge. |
| `bw-kehl-weber-rohstoff` | website,website_status: F-D Gruppenlink schroll.fr, Weber-Tochter dokumentiert; keine eigenständige lokale Kontaktseite/Metallannahme. |
| `bw-kabelstr-metallrecycling-muller` | city Kabelstr.→unbekannt: F-D Importfehler zurückgenommen, keine Nobitz-Zuordnung. |
| `by-neustadt-a-d-aisch-stirling-industrie-metallhandel` | website_status unbekannt→blockiert: F-D HTTP403 im Quellenverlauf, keine Betriebsschließung. |
| `by-munchen-herbig-metallhandel-munchen` | name,website,website_status,description: F-D HRB134478 + Betreiberkarten-JS + LRQA-Anhang bis31.01.2027. Halbzeughandel, kein Schrottankauf; kein Nürnberger Namesake. |
| `be-mitte-gold-inn-ag-mitte` | phone —→leer,status→pruefung: F-D Platzhalter-/Aktivitätsrücknahme; Hotelgesellschaft/Ankaufstelle ungeklärt. |
| `hb-bremen-28719-altun-dienstleistung-all-rounder` | website_status tot→unbekannt: F-D keine hinterlegte bestätigte tote Domain; nicht Altun Service Hanau. |
| `he-lampertheim-68623-sm-metals` | city,phone,email,website,website_status,opening_hours,status,description: **teil-H** Kontakte06206 9510626/info@sm-metals.com, Stunden und pruefung→aktiv. Akte12.10. nennt Identität nur plausibel mit dokumentiertem HRB92336; ein als2026 benanntes Zertifikat ist kein gelesener Gültigkeits-/Aktualitätsbeleg. Identitäts-/Domainzuordnung und attribuierte Beschreibung F-D; UG sm-metallhandel.com bleibt getrennt. Postanschrift nicht Hof. |
| `he-limburg-65549-scheuer` | website,website_status,description: F-D Betreiber + konkret gelesene HRB3767, eingeschränkte Schrottplatzbeschreibung ohne Vergütung; keine Kontaktskalare ergänzt. |
| `he-frankfurt-65929-cimen` | city,phone→leer: F-D Ortsnormalisierung + expliziter Clear12.10.; 11880 kein Kontaktbeleg. |
| `he-sontra-36205-ladner-franz` | city: F-D PLZ aus Ortsstring entfernt, keine neue Annahmefreigabe. |
| `he-hasselroth-63594-habersack-klaus` | description: F-D aktuelle Zertifizierung nicht verifiziert statt Anerkennungsversprechen. |
| `mv-holthusen-19075-rch-recycling-holthusen` | city,postcode: F-D syntaktisch Holthusen/19075; keine Aufnahme neuer RCH-/HAMEG-Kette. |
| `ni-gottingen-37081-koch-willy-altmetalle-rohprodukte` | city,postcode: F-D syntaktische Trennung; kein Gewerbe-/Hofbeweis. |
| `nw-gutersloh-bartsch` | website,website_status: F-D Domainidentität; Marienstraße20/26 versus Elisabethstraße48 und abgelaufenes Zertifikat09.10. bleiben offen. Keine Hofadresse oder aktuellen Kontakte gefüllt. |
| `nw-harsewinkel-guthmann` | name,street,postcode,phone,email,website,website_status,opening_hours,status,description: F-D dokumentierte gleiche HRB7652-Namenskette/Betreiber-Impressum, konkreter Standort und Zertifikatsstand bis12/2026. Zertifikatsvollprüfung hier nicht erneut gemacht, keine Service-Ja-JSON und kein GEO ergänzt. |
| `rp-mainz-55127-arnold` | city,postcode: F-D syntaktisch Mainz/55127, WordPress-Fehler kein Betreiber-/Betriebsbeweis. |
| `rp-andernach-autodemontage-mittelrhein-nettersheim` | name,street,postcode,phone,email,website,website_status,opening_hours,status,description,dropoff_json: F-D Betreiber/Register + Owner-Bildprüfung ICS003AV26 bis30.09.2027. Bahnweg2, kostenlose Annahme Alt-/Unfallfahrzeuge nur mit Vorbehalt zu Papieren/Sonderfällen; keine Gratisabholung/Metallpreise. |
| `rp-worms-rsh` | name,street,postcode,phone,email,website,website_status,status,description: F-D Büro Worms/Hardtgasse1–3 über HRB432931/Betreiber + Original-Efb Anlage2. Büro-/B2B-Handel, kein Publikumshof; vollständige Einkaufs-/Verkaufs-AGB nicht geprüft. |
| `sn-kirchberg-08107-containerdienst-kaminski-sohn` | street,postcode,website: F-D **neuer kommunaler Gewerbeeintrag14.10.** überholt alte Einzelquellenblockade. Auerbacher70A/08107, Service-Deeplink; unterschiedliche Telefonnummern nicht harmonisiert, Kontakte/Zeiten nicht gefüllt. |
| `st-edersleben-nordhauser-rohstoffhandel` | street,postcode,phone,website,website_status,opening_hours,status,description: F-D Owner16.10./Original-PDF + bereits direkt geprüfter Release-Review. Der Hutdeckel/06528, lokale Zeiten/Pausen; keine pauschale AVV-/Privatannahme. GEO51.4167/11.2833 bleibt ungeprüft. |
| `st-salzatal-uhlmann` | name,website,website_status: F-D KG/HRA2691 plus tatsächliches Betreiberimpressum über HTTP; frühere Webdesigner-Fehlinterpretation korrigiert. Adresse/Telefon/Mailkonflikte nicht blind übernommen. |
| `th-gotha-sdm-gutzmann` | street,postcode,phone: **F-D als dokumentierter Owner-Ausnahmefall**, An der Ostbahn4/99867 statt Tottleben-Hauptsitz; Tel nur Formatänderung. Akte10.10. reproduziert Handlerfehler und korrigierten Enrichmentpfad; aktuelle unabhängige Registerbestätigung des alten Ausnahmefalls bleibt Grenze. Keine neue GEO-Prüfung. |
| `bw-mosbach-neckarelz-74821-container-wagner-uwe-entsorgungsbetrieb` | city: F-D Ortsstring ohne PLZ; Uwe/Annegret und Bahnhofstr.8/8–3 als Annahmeort offen, kein neuer Adressbeweis. |
| `by-zeil-am-main-97475-denk-av` | website_status tot→unbekannt: F-D Unsicherheitskorrektur; Brigitte/Elfriede/Fränklin und Zeil/Haßfurt nicht zusammengeführt. |
| `by-aschaffenburg-obernau-63743-willefuhr-tanksanierung-demontage-metall` | name,city,postcode,phone,email,website,website_status,description: **teil-H** Kontakte06028 21188/0170 4765501/info@willefuehr-whg.de; HRA3535/ROWIS und Romano dokumentiert, aktueller Registerstatus ausdrücklich nicht bestätigt. Name/Domain/attribuierte Tankservicebeschreibung F-D; kein aktueller Detailkontakt-Ausnahmebeweis, keine Übernahme Alexander Obernburg. |
| `by-miltenberg-63897-schrott-mehring` | phone→leer: F-D; 09371/3187 nur Miltenberg-Lead, Dorfprozelten nicht transplantiert. |
| `hh-iserbrook-22589-hamburger-schrotthandel-pruffall-hamburg` | phone→leer,website→leer,website_status→unbekannt: F-D Rücknahme ungeklärter Identität/Domain; keine Zuordnung des fremden Hamburger Anbieters. |
| `rp-weroth-alldemont-weroth` | name,trader_type,street,postcode,description: F-D AsEG-ALLDEMONT/HRB24373/Betreiber, In der Mark2, Typ sonstige statt Schrotthändler. Geschäftsführer-/Zusammenschlussgeschichte offen; keine Asbestzulassung/öffentlicher Ankauf aus Website abgeleitet. |
| `sn-halsbrucke-09633-saxonia-edelmetalle` | street Erzstraße9→Erzstr.9: F-D Schreibnormalisierung, kein Umzug. Handler-/Preisfix nicht Bestandteil dieser Trader-Spaltenänderung. |
| `th-zedlitz-07557-entsorgung-und-recycling-bernd-zauritz` | city,postcode: F-D syntaktisch Zedlitz/07557, keine Gleichsetzung Transportservice/Recycling. |

## 58 reine Statuszeilen: Freigabe genau der Unsicherheitskorrektur

Für diese komplette Restmenge ist **nur status** außerhalb notes/extra_json/updated_at verändert. F-D als quellenkritische Zurücknahme einer nicht ausreichend belegten Feststellung. Kein erhaltenes Adress-/Kontakt-/Annahmefeld wird dadurch neu qualifiziert. 57 aktiv→pruefung; nur DAS Recycling/Lauta geschlossen→pruefung. Individueller Quellen-/Namesakeklärfall steht in der selbst gelesenen Statuskorrektur der jeweiligen committeten Dossier-Timeline. Diese Liste ist die vollständige Menge, kein „alle715 frei“:

```text
bw-mannheim-ah-schrott-und-metallhandel
bw-karlsruhe-rudi-kuhn-schrott-und-metallhandel
bw-freiburg-msw-metallhandel-sudwest
bw-albstadt-spindler-karl-schrotthdl
bw-freiburg-sahin-metall-schrotthandel
bw-offenburg-hancer-nazmi-metallhandel
bw-wertheim-pfeifer-rainer-schrotthandel
by-erlangen-konigs
bb-potsdam-hrr-stahlschrott-und-metallrecycling
bb-potsdam-klawes-rainer
bb-falkensee-bohnsack-schrott-container-abri
bb-lichtenow-pietsch-andre-schrottannahme
hb-delmenhorst-grundler-siegfried
hh-altstadt-walter-ritscher
hh-moorburg-harburg-heiko-miersen-schrotthandel
he-dietzenbach-stalcant-schrotthandler
mv-ludwigslust-schrott-metalle-torsten-ruge-inh-steffen
ni-wolfsburg-ute-haucke-schrotthandel
ni-osnabruck-mario-jennebach-schrott-metalle-kabel
ni-papenburg-wittrock-diehl-schrott-und-metallhandel
ni-oldenburg-schrott-and-more
ni-jever-brosda-schrott-und-metalle
ni-norden-weerts-peters-schrotthandel
ni-celle-albert-schmidt-schrott-und-metallhandel
ni-stade-schrott-u-altmetallhandel-stade
rp-ludwigshafen-maudach-s-l-a-altmetall-schrotthandel
sl-losheim-bachem-schrotthandel-bachem-name-unsicher-evtl
sl-bexbach-oberbexbach-bexbacher-buntmetallverwertung
sn-leipzig-schanko-altmetallhandel-schrotthandel-jo
sn-chemnitz-becke-jens-buntmetallhandel
sn-ostritz-steffen-schnei-schrotthandel-entkernung
sn-leipzig-anas-metall-leipzig
sn-arnsdorf-ot-fischbach-schrotthandel-hausmeisterservice-m-baum
sn-kamenz-rainer-kalberer
sn-lauta-das-recycling-inh-raphael-duffort
sn-geithain-ot-rathendorf-64-schrotthandel-a-t-driessen
sn-weiwasser-uwe-bellmann-schrott-buntmetall
sn-werdau-euro-metall-korner
sn-wilkau-halau-containerdienst-schrotthandel-konrad-eim
sn-kamenz-leon-anders
sn-durrrohrsdorf-dittersbach-cs-container-recycling
sh-lubeck-topschrott
sh-gettorf-auto-center-nemitz
sh-selk-thomas-sonksen
by-kitzingen-lenhart-ludwig-sen
be-tempelhof-schoneberg-friedenauer-munzenhaus
bb-prenzlau-ot-klinkow-ramm-klinkow
bb-drebkau-kielow-eberhard
hh-allermohe-van-der-meulen-edelmetaal-b-v
nw-dusseldorf-reisholz-velbo
nw-unna-mairitsch
nw-iserlohn-kettig
nw-bergheim-frechen-gartner
nw-ludenscheid-reininghaus
th-weimar-gert-wiesser
by-vilsbiburg-84137-hertenberger-av
by-miltenberg-63897-h-g-gold-an-verkauf
sh-kiel-kieler-altmetall
```

Konkrete nicht gelöste Identitätsketten bleiben unter anderem AH OHG HRA702222→GmbH HRB737082, MSW Freiburg/Breisach→TSR, HRR Hennigsdorf→INTERSEROH/Potsdam, Haucke Wolfsburg→Grußendorf, Arnold/Mario Jennebach, Bexbacher Feix-/Adresskette, Lenhart Ludwig→Angelika und Van der Meulen Joure→Hamburg. Statusrücknahme braucht diese Ketten nicht positiv aufzulösen; eine Umbenennung, Kontakttransplantation oder Hofaktivierung dagegen schon. Miersen-Stilllegungsquellen haben widersprüchliche Chronologie, daher gerade kein geschlossen.

## Metadaten, Keep-Verhalten und unveränderte problematische Bestände

### extra_json und Notizen

Alle715 `seed_hash` ändern sich, alle715 updated_at werden gesetzt. **Außer seed_hash ändert sich in extra_json ausschließlich seed_file: 153 mal entfernt.** seed_section/ankauf_raw bleiben in diesen Zeilen unverändert; keine anderen Enrichment-Keys verloren. Ursache ist der vollständige Neuaufbau in `seed_traders.rs:407–411`, kein Merge. Technisch erklärbar, aber Ownerentscheidung zur Provenienz erforderlich; aus erhaltenem seed_section lässt sich die entfernte historische Datei nicht in jedem Consumer automatisch rekonstruieren.

Bei698 der715 Notizen ist der vorherige vollständige String unverändert im neuen enthalten. Bei17 nicht; Nachprüfung auf ` | `-Bullet-Ebene zeigt **alle früheren Bulletstrings erhalten, einzig ASR/Coswig verliert den Platzhalter `—`**. Damit kein nachgewiesener Verlust alter fachlicher Timelinebullets; Reihenfolge/neu eingefügte Einträge erklären die übrigen Abweichungen. Das prüft Bestandserhalt, nicht die Wahrheit aller neuen Notizen. Die545 reinen Notiz-/Metadatenzeilen werden **nicht als vollständig fachlich neu auditiert** freigegeben. Vorliegende Ownerreviews sind Belegstufen, kein Ersatz für jede Einzelaussage; dieser Auftrag liefert insbesondere die vollständige skalare Wirkungsabdeckung.

### Tatsächlich erhalten trotz leerem Frontmatter

Die Ziel-/Wirkungsgegenprüfung fand keine unerwartete Überschreibung nichtleerer Zielskalare. Dagegen hält der normale Keep-Vertrag absichtlich Altwerte. Besonders entscheidungsrelevant:

- `ni-jever-brosda-schrott-und-metalle`: Ziel phone leer, aber **04461 965331 bleibt**. Akte14.10. erklärt ausdrücklich Kontakt als nicht qualifiziert zurückgenommen; ohne `clear_fields: phone` wird diese Absicht nicht wirksam. Nur status aktiv→pruefung ist tatsächlicher skalarer Diff. **HOLD für Behauptung „Brosda-Kontakt zurückgezogen“**; aktuelle Nummer separat belegen oder ausdrückliche Clear-Entscheidung in künftigem Commit.
- `be-marzahn-hellersdorf-papierfritze-schrott-altpapierankauf`: email im Ziel leer, **info@papierfritze.de bleibt**. Die Akte bewahrt zudem eine Standort-/Zweitmarkendublette. Keine neue Mail-/Dedupfreigabe.
- Weitere erhaltene nichtleere Telefonwerte bei leerem Dossierfeld: `be-mitte-gundlach-klaus` 0171 9935369; `he-darmstadt-nord-hartmann-recycling` 01578 6795832; `he-altenstadt-63674-ing-norbert-szielasko-kabelzerlegungen-m` 060472901; `he-seligenstadt-63500-juwelier-dittmeier` 06182 21961. Bei Dittmeier bleiben außerdem info@juwelier-dittmeier.eu und Mo–Fr10–18/Sa10–14/So geschlossen. Keine dieser erhaltenen Angaben durch diesen Review positiv bestätigt.
- `ni-ostrhauderfehn-r-leimberger` behält **https://schrottplatz-info.de** trotz leerem Zielwebsitefeld. Das ist ein Portal, keine frisch qualifizierte Betreiberwebsite.
- `nw-aachen-ocher-schrottmannchen` behält trotz leerer Zieldescription die alte mobile Leistungsbeschreibung. Kein neu bestätigter Abhol-/Ankaufvertrag.
- Ungültige Telefonplatzhalter bleiben ebenfalls bei leeren Zielwerten: `—` bei Berlin Wolf Marko, Sperber Otto, B&M, Schrottmeister, Friedrichshain/Kreuzberg-Abholung, Gold Inn Wilmersdorf; `n.e.`/`n.e` bei Rose Prenzlau, Petermann/Hackfahrel, RSO Doberlug, Dymke Lauchhammer, Husic Großräschen, Kaden Stahnsdorf, Ramm Klinkow und Krasulsky Cottbus. Dies sind benannte Restdaten, keine neuen validen Kontakte. Gold Inn **Mitte** dagegen wird tatsächlich geleert.
- Zahlreiche leere Ziel-dropoff_json erhalten ein vorheriges `{"allowed":true}`. Das betrifft auch heruntergestufte AH, Rudi Kühn, MSW, Spindler, Hancer, Klawes, Bohnsack, Pietsch/Lichtenow, Altmetall Bunker, Miersen, Stalcant, Haucke, Peters, Mario Jennebach, Wittrock/Diehl, Schrott&More, Brosda, Weerts/Peters, Albert Schmidt, Stade, Bachem, Bexbacher Buntmetallverwertung, Weiler, Becke, Anas, Baum, Kälberer, DAS/Lauta, Driessen, Bellmann, Körner, Eißmann, Leon Anders, Topschrott, Nemitz und Sönksen. **Keine Behauptung alle Annahmezusagen zurückgenommen.** Diese Liste benennt kritische Beispiele, keine neue inhaltliche Bestätigung der anderen erhaltenen JSONs.

Explizite Clears wirken hingegen tatsächlich: die im skalaren Inventar benannten phone-Rücknahmen (einschließlich Eibl, Böhm, Weber, Cimen, Mehring und Hamburger Prüffall), Hammerschmidt/R.Müller auch email, R.Müller/Delis auch Zeiten, R.Müller description sowie NRW-Schrott24 beide Service-JSON. Diese Wirkungen sind im Diff vorhanden; kein bloßes Vertrauen auf die Direktiven.

### Bestands-GEO ist kein Nebenprodukt einer Adressprüfung

Die neun geänderten Paare sind die einzigen GEO-Änderungen. **3893 Händler minus9 ist keine Menge verifizierter Pins.** Auch alle28 Adressrücknahmen, Fernau-Ortswechsel, neue Resebeck-/TSR-/NRH-Adressen und Historien-/Prüffälle können alte Koordinaten behalten. In den vorliegenden Reviews besonders benannt: Hark/Hamm, Heine/Bernburg, TSR Ginsheim, NRH Edersleben und TSR Neumünster. Die im schmutzigen Arbeitsbaum bearbeiteten Bamberg/Florian- und Roding/Jansen-Dossiers wurden nicht übernommen; keine GEO-Freigabe aus ihnen abgeleitet.

## Was der Owner vor einer Gesamtfreigabe konkret entscheiden muss

1. Die **H-/Teil-H-Zeilen** dieses Berichts auflösen oder deren begrenzte Veröffentlichung ausdrücklich mit Begründung freigeben: AGOSI Kontakt, Fernau Kontakt/Stunden, SM Metals Kontakt/Stunden/Aktivierung, Graf Kontakt, Willeführ Kontakt, Limekon Detailbedingungen/Kontakte und Bausch Adresse/Kontaktdetails/Aktivierung. Bestehende Dokumente zuerst gegen die jeweiligen fehlenden Kriterien lesen; keine durchgefallene Ausnahme pauschal mit einem Northdata-Namensmatch ersetzen. Hark ist nach späterer Owner-/Behördenevidenz als historischer Standort freigabefähig; Giesen-Wekos' offengelegter Zertifikatsregisterfehler wird nicht zum erfundenen neuen Blocker gemacht.
2. Legacy-Annahmeflags bei den zwölf Inserts und erhaltenen positiven JSONs als **bekannte Semantik-/Bestandsgrenze** ausdrücklich akzeptieren oder über einen gesondert geprüften regulären Pfad korrigieren. SQL-Nutzer können diese Werte direkt sehen. Das ist unabhängig von neuer JSON-Sorgfalt und von unveränderten Preisen.
3. Rücknahmeabsicht versus Keep-Ergebnis entscheiden, mindestens Brosda und die oben konkret genannten erhaltenen Kontakte/Portalwebsite/Platzhalter. Kein Report darf behaupten, diese Werte seien mit diesem Payload bereits entfernt.
4. Entfernung von153 seed_file-Keys als bewusstes Provenienz-/Kompatibilitätsdelta akzeptieren oder zielgerichtet beheben; keine715 Metaänderungen als reine zusätzliche Notizen verkaufen.
5. Die545 übrigen Notiztexte nur mit ihrem tatsächlich dokumentierten bisherigen Reviewstatus veröffentlichen; **keine pauschale neue Quellen-/Unternehmens-/Pinverifikation**. Dieser Bericht bestätigt technische Wirkung/Bestandserhalt und die aufgeschlüsselten skalaren Entscheidungen, nicht alle neuen narrativen Tatsachenbehauptungen.

Der 60er-Review `research/checkin-20261016-0300-release-review.md` ist für seinen Vergleich `fbfd06c0→81a64832` weiterhin ein brauchbarer Vorbeleg, aber **keine vollständige Upgradefreigabe aus dem tatsächlichen Produktionssnapshot**. Die uncommitteten Owner-/Rolloutberichte wurden als Kontext gelesen, nicht als freigegebener Produktinput. Remote-/Build-/Verifier-/Hofmann-Altkey-Fragen sind separate technische Gates; keine davon wird durch diesen Seedreview als erledigt ausgegeben.

**Endstand: vollständige Bestands-Spaltenbilanz und alle170 skalaren Zeilen zugeordnet, alle12 Inserts und9 GEO-Paare einzeln behandelt; Gesamtfreigabe bleibt wegen der konkret benannten Fach-/Semantikentscheidungen HOLD. Keine Produktionsänderung vorgenommen.**
