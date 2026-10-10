# Owner-Gate: Wave D Nordwest — unabhängige Prüfung 10.10.2026

## Abschließende Owner-Abnahme 10.10.2026

Der Owner hat die vollständigen 20 Main-Dossierdiffs gelesen und die engeren Korrekturen unten abgenommen. Diese Abnahme ersetzt den ursprünglichen Integrationshalt für den **jetzigen Patch**, nicht die dokumentierten offenen Quellenfragen oder eine Produktionsfreigabe. ALLDEMONT erhält nur belegte Name/Anschrift, All-in bleibt Buchholzer Klärfall mit Herner Gegenidentität, Prison nur historischer Preislead. Keine neue Ankaufsliste oder ungeprüfte Kontakt-Fills freigegeben.

Der Kontaktbereinigungsblocker ist auf Code-/Dossierebene durch die separat geprüfte opt-in-Seed-Semantik `clear_fields` gelöst: Iserbrook `phone,website`; All in Buchholz, Müller Leichlingen, Peters Osnabrück und F. Weber jeweils `phone`. Alle sechs Zielwerte sind im Dossier leer und explizit markiert. Bereinigung ausschließlich beim normalen Seed-Upsert nach Deployment; kein direkter Produktions-DB-Schreibzugriff. Schrott24 nutzt weiterhin explizite `{}`-Objekte ohne Behauptung einer Ablehnung oder aktuellen Verfügbarkeit. GEO bleibt unverändert.

Owner-Gates am tatsächlichen Main-Arbeitsstand: genau 20 Dossiers, alle 74 alten Timeline-Bullets wörtlich erhalten; `slug`, `lat`, `lon`, `trader_type` unverändert; Status-/Website-Enums einschließlich historisch erlaubter leerer Website-Statusfelder und Konditions-JSON geprüft. Der erste eigene Enum-Check war zu streng für erlaubte leere Website-Statusfelder; korrigierter Check besteht ohne Dossieränderung.

Tests: `cargo test --locked -p schrott-mcp-ingestion seed` **13 bestanden**; Buildscript-Testbinary **9 bestanden**; `cargo test --locked --workspace` **610 bestanden, 0 fehlgeschlagen, 5 ignoriert**, Log `/root/Documents/seed-clear-main-workspace-20261010.log`. `cargo fmt --all -- --check` und `git diff --check` bestanden. Unabhängiger Drei-Dateien-Clear-Review meldet keine funktionalen Findings.

**Lokal zur getrennten Main-Integration freigegeben; noch nicht veröffentlicht oder produktiv bereinigt.** GitHub-Authentifizierung fehlt weiterhin. Push, koordinierter Redeploy und read-only Produktions-Spotprüfung aller sechs Clear-Werte sowie Schrott24-Bedingungen bleiben erforderlich. Der nachfolgende Audittext bleibt als historische Prüfschritte erhalten.

## Main-Abnahmevorbereitung 10.10.2026

Auf ausdrücklichen Folgeauftrag sind die 20 unten genannten Dossiers jetzt **in Main `/root/Documents/schrott-mcp` per patch vorbereitet**, nicht committed/deployed. Vor dem Edit war `git diff 072cf25 -- <genau diese 20 Pfade>` leer: keine zwischenzeitlichen Main-Dossieränderungen zu überschreiben. Fremde untracked Berichte, insbesondere `research/checkin-20261010-waveD-mittenordost-owner-gate.md`, unberührt. Deploy-Worktree nur gelesen/unverändert gelassen. Nur diese 20 Dossiers und dieser Bericht bearbeitet. Keine Builds/Tests/Commits/Deployments/DB-Zugriffe; Owner prüft Gesamtdiff und Tests.

Die folgende ursprüngliche Audit-Freigabe ist **kein Beibehaltungsstempel**. Umsetzung enger nach Folgeauftrag:

- RSH belegte Büroidentität/Adresse/Kontakte/Website/Description und aktiver B2B-Status, RETRACO Firmenadresse/city/Website-Erreichbarkeit übernommen; Annahmebedingungen leer.
- Müller, Weiler, Peters, Klass auf pruefung. Unbelegte aktuelle street/postcode/phone (soweit vorhanden) zurückgenommen; Müller-„Inh.“-Description durch nichtleeren ausdrücklichen Prüfvorbehalt ersetzt. Alle entfernten Werte datiert in Timeline, alte Bullets erhalten.
- All in Buchholz street/postcode/phone zurückgenommen; Herner Betreibergegenidentität dokumentiert, kein Ortwechsel/Herner Kontakt-Fill.
- F. Weber phone, Iserbrook phone/ungeklärte Websitezuordnung zurückgenommen; Iserbrook website_status unbekannt bei leerer Website, NXDOMAIN weiter technisches Arbeitswissen.
- Weiler-Familienspekulation aus name/Überschrift, AYAN-Garblestraße aus street entfernt, wörtliche alte Werte erhalten. Seed-Orte sonst nicht verschoben; Slugs und GEO unverändert.
- ALLDEMONT **nur name/street/postcode** auf übereinstimmende Betreiber-/Registeridentität gefüllt. Betreiberquelle/Kontaktkandidaten dokumentiert, phone/email/website/Description/Annahmebedingungen bleiben leer: keine unabhängige Kontaktbestätigung und keine Betreiber-Ausnahme beansprucht. Die weitergehende Kontakt-/Typ-Empfehlung des ursprünglichen Audits unten wurde ausdrücklich **nicht umgesetzt**. Alle trader_type-Werte unverändert; ungeklärte Seed-Typen nicht durch neue Vermutung ersetzt.
- Schrott24 beide allowed-true-JSONs durch **explizite leere Objekte `'{}'`** ersetzt und alte Objekte **wörtlich** in datierter Timeline erhalten, nicht zu allowed false umgedeutet. Description/Überblick auf Betreiberwerbung bei ungeklärter heutiger operativer Verfügbarkeit begrenzt.
- Alle 20 haben die jeweilige Wave-D-Ergänzung; zusätzlich ALLDEMONT/All-in/Prison-Gegenbelege und Legacy-Korrekturen dokumentiert. Neue Timeline-Korrekturen gehen alten falschen „aktiv/geocodieren/Register“-Aussagen vor, ohne die Historie zu löschen.

**Noch keine technische/Produktionsfreigabe.** Leere Standort-/Kontakt-/Konditionsfelder dürfen nicht als bewiesene Schließung, Ablehnung oder Gebührenfreiheit gelesen werden. GEO bleibt wie beauftragt erhalten; unbekannte/veraltete DB-GEO wird durch das Leeren einer Adresse nicht automatisch gelöscht (siehe `docs/trader-seed.md`).

### Wichtiger Integrationsblocker: leere Kontakte löschen Bestands-DB nicht

Statische Lektüre von `crates/ingestion/src/seed_traders.rs:258–267,300–307,337–357` (keine Ausführung): `keep(seed, stored, fallback)` erhält bei leerem Seed vorhandene **phone/email/website/Description/dropoff/pickup**. Straße/PLZ werden dagegen direkt aus dem Seed übergeben. Deshalb:

- Schrott24-Bedingungen müssen nichtleere `'{}'` sein, damit vorhandenes allowed true beim normalen Seed ersetzt werden kann. Keine neue Ablehnung impliziert. Müller-Description ist ebenfalls nichtleer und benennt die Beleglücke statt Inhaberschaft.
- Leere Dossier-phone-Werte für Iserbrook, All in, Müller, Peters und F. Weber sowie leere Iserbrook-website nehmen die Quellenbehauptung aus dem Build-Input zurück, **löschen aber alte DB-Werte nicht automatisch**. Der Owner muss die betreffenden Bestandswerte in einem separat autorisierten Bereinigungsschritt oder über eine separat entwickelte explizite Clear-Semantik entfernen. Dafür hier keine Handler-/Compileränderung, kein DB-Schreibzugriff und keine Platzhalter-/Whitespace-Hacks.
- Bis diese Bestandskontakte bereinigt und nachgeprüft sind: **keine Freigabe der effektiven Produktionsdaten**, auch wenn Dossierdiff/Tests später bestehen. Der beauftragte reine 20-Dossier-Patch kann diesen Seed-Semantikblocker nicht vollständig aufheben.

Abschließender statischer Historien-/Scopevergleich: alle **74** alten Timeline-Bullets in Main erhalten, genau **20** Dossierpfade verändert; `slug`, `lat`, `lon`, `trader_type` in allen 20 identisch zu HEAD. Keine Tests ausgeführt.

## Entscheidung

**Keine Gesamtfreigabe der 20 Dossiers.** Freigegeben sind die belegten RSH-Fills, die RETRACO-Adress-/Website-Deltas und die vier Rückstufungen auf `pruefung`. Die übrigen Rechercheergänzungen sind überwiegend als begrenzte Befunde/Leads brauchbar, nicht als Bestätigung vorhandener Frontmatter-Fakten. **ALLDEMONT und All in brauchen substantielle Ergänzungen; mehrere Legacy-Felder bleiben gesperrt.**

Ursprünglicher Prüfgegenstand: `/root/Documents/schrott-mcp-deploy/research/wave-20261009d-nordwest.md` und exakt dessen 20 verlinkte Dossiers, jeweils aktueller Inhalt und `git diff` gegen HEAD. Maßstab: dortige `README.md`, insbesondere Zeilen 148–182 und 216–220. Im ursprünglichen Audit wurden Quellworktree/Dossiers ausschließlich gelesen und nur dieser Bericht geschrieben. Die folgenden ursprünglichen Empfehlungen sind durch die oben beschriebene, ausdrücklich beauftragte Main-Abnahmevorbereitung konkretisiert; sie wurden nicht im Quellworktree ausgeführt. Keine Builds, Tests, Commits, Deployments oder DB-Zugriffe in beiden Phasen.

## Maßstab und belastbare Grenzen

- Zwei unabhängige zulässige Quellen; zehn Verzeichnisse ergeben keinen zweiten Beleg. Kontakt, Impressum, AGB und Homepage desselben Betreibers zählen nicht vierfach.
- Die Betreiber-Ausnahme verlangt **alle** README-Kriterien einschließlich HRB/Registerkonkordanz, standortbezogener Seiten und aktueller Inhalte. Erreichbarkeit, Footer-Jahreszahl oder hochgeladener Jahresabschluss allein beweisen weder heutigen Ankauf noch Publikumsannahme.
- README verlangt bei berührten Dossiers eine Nachprüfung auch der Legacy-Verzeichnisfelder. „Nicht geändert“ und „Herkunft dokumentiert“ sind keine Faktenfreigabe. Historische Timeline erhalten; daraus folgt nicht, dass dieselbe Behauptung als strukturierter aktueller Fakt stehen bleiben darf.
- `status: pruefung` heilt kein unzulässig gefülltes Telefon/Adressfeld. Bei fehlendem Nachweis: Owner soll unbelegte strukturierte Felder in historische Notizen/Provenienz zurücknehmen oder deren Integration ausdrücklich zurückstellen. Slug bleibt unverändert; niemals aus Nichtfund schließen.
- Alle unten als erneut gelesen bezeichneten URLs wurden am **10.10.2026** direkt abgerufen. Websuche diente der Entdeckung; Suchauszüge wurden nicht zu verifizierten Register-/Betreiberbelegen hochgestuft. Suchdienst lieferte teilweise sachfremde Treffer.

## Priorität 1: konkrete Freigaben und notwendige Korrekturen

### 1. RSH Worms: neue strukturierte Felder freigegeben

Erneut gelesene [Kontaktseite](https://www.rsh-gmbh.com/kontakt), [Impressum](https://www.rsh-gmbh.com/impressum) und [Homepage](https://www.rsh-gmbh.com/) belegen:

- RSH GmbH, Büro Worms, Hardtgasse 1–3, 67547 Worms, Telefon +49 6241 5061030, `office@rsh-gmbh.com`.
- Sitz Gaggenau, Ferdinand-Rahner-Straße 8, AG Mannheim HRB 432931, Martin Lang; Handel von Recyclingunternehmen zu europäischen Schmelzbetrieben.
- Unabhängig erneut gelesener [Northdata-Datensatz](https://www.northdata.de/?query=RSH+HRB+432931): gleiche HRB/Sitz; Wormser Zweigniederlassung in Veröffentlichungen 15.05.2014 und 12.05.2015. Originalpublikationen hinter Paywall nicht separat gelesen.
- [Original-EFB-PDF](https://www.rsh-gmbh.com/fileadmin/user_upload/RSH_EfbV_Zertifikat_bis_11-12-2027.pdf) direkt abgerufen und vollständig textuell extrahiert: 2026-0172 / ZZHT010001315009, Prüfung 12.06.2026, Ausstellung 18.06.2026, gültig 11.12.2027. Seite 4/Anlage 2 nennt exakt Hardtgasse 1–3, Worms, und ausdrücklich „Handeln und Makeln von und mit gefährlichen und nicht gefährlichen Abfällen“. Nicht aus verlorenen Checkboxen zusätzliche Tätigkeiten abgeleitet.

**Freigabe:** `name`, `street`, `postcode`, `phone`, `email`, `website`, `website_status: aktiv`, `status: aktiv` und neue Description, ausschließlich als B2B-Handelsbüro. `trader_type: schrotthaendler` ist mit Metallhandel vereinbar, nicht als Hofnachweis zu interpretieren. Keine Privat-/Anliefer-/Abholbedingungen, Öffnungszeiten oder GEO ergänzen. EFB ist keine öffentliche Annahmezusage. Die zwei unabhängigen Quellen tragen die Zuordnung; keine bloße Impressums-/Namensgleichheit.

**Nicht unabhängig abgeschlossen:** RSH-AGB-Bildscans/OCR und Zahlungs-/Nebenkostendetails. Die Wave markiert das korrekt offen. Nicht als numerische Preis-/Gebührenquelle freigeben; kein „vollständige AGB enthalten keinen Tarif“ daraus machen.

### 2. RETRACO: Adress-Fill freigegeben; aktuelle Annahme und Kontakte offen

[Homepage](https://retraco-gmbh.com/) erreichbar. [Einkaufs-AGB](https://retraco-gmbh.com/agb-einkauf.pdf) und [Verkaufs-AGB](https://retraco-gmbh.com/agb-verkauf.pdf) direkt abgerufen, jeweils vier Seiten textuell gelesen: RETRACO GmbH, Neuhöfer Damm 110, 21107 Hamburg, HRB 136833; Stand 01.01.2019. [Northdata](https://www.northdata.de/?query=RETRACO+GmbH) unabhängig gleichlautend; Geschäftsführerwechsel 26.06.2020 zu Filio A. Hadjiev. Damit Betreiber plus Register, **nicht** die Aktualitätsausnahme.

**Freigabe:** `city: Hamburg`, `street`, `postcode`, `website_status: aktiv`; `status: pruefung` beibehalten. Beleg gilt für Firmen-/Kontaktanschrift, nicht verifizierte heutige öffentliche Annahmestelle. Telefon/Email/Zeiten und Konditions-JSON bleiben leer.

AGB bestätigen B2B-Geltung, individuellen Einkaufspreis/14 Tage grundsätzlich, Verkaufsseite ab Werk zzgl. USt./vertrauliche Preise und Lieferkosten beim Kunden. Kein numerischer Materialtarif darin. PDF-Kontakte 040 74042178 und `info@retraco-gmbh.com` bestätigt. **Bild `website.jpg` in dieser Gegenprüfung nicht visuell gelesen; 040 74042179 und Bild-Mobil/Email sind hier nicht zusätzlich unabhängig bestätigt.** Der behauptete Konflikt rechtfertigt jedenfalls keine Auswahl einer Nummer ohne Klärung.

[schrottregister.pages.dev](https://schrottregister.pages.dev/betrieb-retraco-gmbh-hamburg) erneut gelesen: ausdrücklich „kein amtliches Angebot“, ZZBT00300171 und Stand 11.08.2026. Zertifikatsumfang daher nur Lead. Die neue Korrektur des alten „Register-/Zertifizierungsbelegs“ ist richtig; **der alte Aufruf, als zertifizierte Betriebsstätte zu geocodieren, ist überholt und nicht freigegeben**. Original-EFB weiterhin nicht gelesen/verifiziert.

### 3. ALLDEMONT: die behauptete fehlende Betreiberquelle ist überholt

Eigenständige Suche fand eine **passende Betreiberwebsite**, nicht nur weitere Verzeichnisse:

- [Impressum asbest-entsorgung.de](https://asbest-entsorgung.de/impressum/): AsEG-ALLDEMONT GmbH, In der Mark 2, 56414 Weroth, HRB Montabaur 24373, Telefon **06435 2922**, `info@asbest-entsorgung.de`, Mobil 0151 61139426 / 0170 6551890; Geschäftsführerangabe Rolf Börstinghaus.
- [Über uns](https://asbest-entsorgung.de/ueber-uns/): Asbestentsorgung, Schadstoffsanierung, Abbruch, Industriedemontage/Rückbau, Transportleistungen; Betreiber erzählt AsEG-Gründung 1998, Ergänzung ALLDEMONT 2013 und Zusammenschluss 2018. Das ist **Betreibererzählung**, keine unabhängig vollständig bewiesene gesellschaftsrechtliche Kette.
- [Kontakt](https://asbest-entsorgung.de/kontakt/): Anfrage für Sanierung/Entsorgung/Demontage/Gutachten; kein belegter öffentlicher Schrottankauf.
- [Northdata](https://www.northdata.de/AsEG-ALLDEMONT%20GmbH,%20Weroth/Amtsgericht%20Montabaur%20HRB%2024373) erneut gelesen: Firma, HRB, Werother Anschrift und Gegenstand stimmen überein. Historie VIKTOR@ROLF UG → AsEG-ALLDEMONT UG → GmbH bestätigt. Neuere Publikationen sind Jahresabschlüsse.

**Notwendige Korrektur vor Gesamtintegration:** Wave/Dossier dürfen nicht mehr aktuell behaupten, kein Betreiberauftritt/zweiter Beleg sei auffindbar. Neues Korrektur-Bullet mit diesen Quellen; alte Nichtfunde erhalten.

**Konkrete zusätzliche Freigabe nach Dokumentation:** Name `AsEG-ALLDEMONT GmbH`, Firmenanschrift In der Mark 2 / 56414, Website `https://asbest-entsorgung.de/`, `website_status: aktiv`, Festnetz 06435 2922 und Email. Betreiber + unabhängige Registerquelle ordnen das Seed-Fragment zu. Anschrift als Firmenkontakt, nicht Schrottplatz. Sachgerechte Description „Schadstoffsanierung/Asbestentsorgung, Abbruch und Industriedemontage; öffentlicher Schrottankauf nicht belegt“. `trader_type: sonstige` statt unbestätigtem Schrotthändler sachlich begründbar; `status: pruefung` hinsichtlich Zielkategorie/Ankauf belassen.

**Weiterhin keine Freigabe:** Geschäftsführer-Fill (Northdata listet 28.05.2018 „Nicht mehr Geschäftsführer: Rolf Börstinghaus“, Website gleichnamige Senior-/Junior-Historie; Personen auseinanderhalten), Kettenbehauptung als gesicherten Merge, Asbestannahme/Zulassung oder EFB. [Zertifikatsseite](https://asbest-entsorgung.de/zertifikate/) nennt Lehrgänge/TRGS 519; eingebettete Zertifikatsbilder nicht gelesen, keine heutige Gültigkeit überprüft. Werbetext zu Genehmigungen genügt nicht für einen genehmigten Publikumsannahmeplatz. Kein Preis-/Gebührentarif gefunden. Creditreform-Direktabruf 403; Suchtreffer nicht als dritten Beleg gezählt.

### 4. All in Buchholz: neue Gegenidentität, keine Legacyfreigabe

[GS-Profil](https://www.gelbeseiten.de/gsbiz/1bc32d70-6912-4a51-bcac-616ad20a76c2) erneut identisch: Steinbecker Str. 35 / 21244 / 0178 9251327, unverständlicher Beschreibungstext. **Kein zulässiger Buchholzer Standortbeleg.**

Eigenständige Suche nach dem Namen fand [Golocal Herne](https://www.golocal.de/herne/schrotthandel/all-in-schrott-abholung-MJo6W/), dort dieselbe Nummer und ein echter Website-Link. Direkt gelesen:

- [Betreiber-Impressum](https://schrottabholung-allinn.de/impressum): All Inn Schrott, **Fayad Adnan Jaddouh**, Emscherstr. 27, 44649 Herne, +49 178 9251327, `ahmed1982@freenet.de`.
- [Homepage](https://schrottabholung-allinn.de/): Herne und NRW, keine Buchholzer Filiale. Ausdrückliche kostenlose Abholung, auch Kleinmengen; Tagespreis telefonisch für NE-Metalle, keine numerischen Ankaufskurse. 0,00-Euro-Abholung ist Gebühr/Service, **kein Ankaufspreis**. Diese Angaben gehören zum Herner Kandidaten.
- Golocal behauptet Inhaberantwort, warnt aber selbst, Inhaber-Verknüpfung und Nutzermeldungen nicht überprüft zu haben. Kein unabhängiger zulässiger Zweitbeleg aus diesem Portal.

**Integrationshalt für die Buchholzer Legacy-Adresse/PLZ/Telefon als aktuelle Fakten.** Gleiches Telefon ist ein ernstes Fehlzuordnungs-/Umzugsindiz, keine bewiesene Kette. Neues Gegenbeleg-Bullet erforderlich. Nicht nach Herne umziehen, nicht Herner Website/Leistungen im NI-Dossier füllen; Slug stabil, `pruefung`. Historische Werte in Timeline/Provenienz erhalten, unbestätigte strukturierte Standortwerte zurücknehmen oder ihre Integration zurückstellen. „Keine eigene Website“ auf den konkreten GS-Eintrag begrenzen; global ist die Aussage jetzt falsch. Gewerbenachweis/Standortkette Buchholz ↔ Herne bleiben offen.

## Einzelgate für alle 20 Dossier-Deltas

`Notizen frei` bedeutet Freigabe eines korrekt begrenzten Recherche-Bullets, **nicht** Freigabe des gesamten Dossiers oder seiner Legacy-Fakten. Pfade relativ zum Quellworktree.

| # / Dossier | Entscheidung für Delta | Ehrliche Lücke / Owner-Korrektur |
|---|---|---|
| 1. `dossiers/rp/rp-trier-54295-scheller-mathias.md` | Notizen/Überblick frei; Facebook-Login erneut bestätigt. Keine Kontakt-/Zeit-Fills. | Mathias/Matthias und MS-Identität/Gewerbe ungelöst; keine zulässige zweite Quelle. City `Trier 54295` ist Legacy-Fragment, keine frisch bestätigte normalisierte Adresse. |
| 2. `dossiers/hh/hh-iserbrook-22589-hamburger-schrotthandel-pruffall-hamburg.md` | Resolver-Vorbehalt frei; `pruefung` korrekt. | Unabhängiger DNS-Nachtrag unten stützt Domain-NXDOMAIN, nicht Betreiberzuordnung. Legacy-Telefon 0173 6110524 und Zuordnung der Domain zu diesem Betrieb nicht freigegeben. Kein Schluss „oder schließen“ ohne positiven Schließungsbeleg. |
| 3. `dossiers/rp/rp-neustadt-meyer-lachen-speyerdorf.md` | Kandidaten-/Gratisvorbehalte frei, durch Service/Impressum/Autoverwertung/Zeiten direkt bestätigt. | Sascha-Patrick-Seed-Kette und zweiter zulässiger Beleg fehlen; keine Website/Kontakt/Konditions-Fills. Überblick „keine veröffentlichten Abholtarife“ um die zwei bedingten Gratisangebote präzisieren. |
| 4. `dossiers/ni/ni-buchholz-i-d-n-21244-all-in-schrott-abholung.md` | Nur GS-Abruf-Bullet frei; **Nachtrag und Legacy-Gate nötig**. | Herner Betreibergegenbeleg siehe oben. Adresse/PLZ/Telefon nicht bestätigt; keine pauschale Beibehaltungsfreigabe. |
| 5. `dossiers/nw/nw-leichlingen-muller.md` | Northdata-Fund und `aktiv` → `pruefung` frei. | Metzholz 68/42799 nur Register-Einzelbeleg plus Verzeichnis; Festnetz/Description „Inh. Axel Müller“ nicht freigegeben. Zweiter zulässiger Standort-/Kontaktbeleg fehlt. Leverkusener Website unabhängig als HRB **99637** erkannt, nicht HRB 48784. |
| 6. `dossiers/sl/sl-merchweiler-schrott-weiler-matthias-wahrsch-gleiche.md` | Rückstufung/kein Schließungsschluss frei. GS-404 erneut bestätigt. | Neuer Portal-Lead Matthias UND Bärbel Weiler siehe unten; keine Familienkette. Spekulation „wahrsch. gleiche Familie wie Nr. 46“ aus aktuellem `name` entfernen, historisch erhalten. Eisenbahnstr. 1a als aktueller Standort nicht freigegeben. |
| 7. `dossiers/nw/nw-leverkusen-alsleben.md` | Portal-Freshness-/Bewertungsvorbehalt frei, direkt bestätigt. | Voller Rechtsträger und eigener Auftritt fehlen. Gratis nur Portalwerbung. Zusätzlicher Cylex-Suchlead hat andere Nummer, keine Auflösung/kein Fill. |
| 8. `dossiers/hh/hh-wilhelmsburg-hafen-retraco.md` | **Adress-/Website-Deltas frei** mit B2B-/Firmenadressvorbehalt. | Original-EFB/Telefonkonflikt/Publikumsbetrieb offen; Bildinhalt in diesem Audit nicht eigenständig visuell geprüft. |
| 9. `dossiers/ni/ni-delmenhorst-jesewski-autoverwertung.md` | Bremer Gegenbeleg/kein Fill frei; HTTP-Impressum und Homepage direkt bestätigt. | Delmenhorster Identität/Vorgänger/Filiale fehlen. Bremen nicht übertragen. eBay-Konto/Einzelangebote in dieser Prüfung nicht unabhängig geprüft. |
| 10. `dossiers/rp/rp-worms-rsh.md` | **Neue strukturierte Fills/aktiver B2B-Bürostatus frei**, siehe oben. | Kein Privatankauf, Hof, Öffnungszeiten/GEO oder numerischer Tarif; Scan-AGB weiter offen. |
| 11. `dossiers/ni/ni-region-hannover-hildesheim-schrotthandel-f-weber-ort-n-a.md` | Ortskonflikt-/404-Bullet frei; Gegenportal direkt gelesen. | Vorname/Gewerbe/Region ungeklärt. Legacy-Telefon 0173 6115144 nicht als Hannover/Hildesheim-Kontakt freigegeben. Keine Ortsverschiebung nach Schandelah. |
| 12. `dossiers/ni/ni-alfeld-balzers-metallverwertungsgesellschaft.md` | Positiver Register-Einzelfund als Notiz frei, direkt bestätigt. | Zweite zulässige Quelle und aktueller Betrieb/Annahme fehlen. Creditreform gefunden, aber 403 beim Abruf; kein zusätzlicher Quellen-Fill. Überblick „Identität nun konkret“ besser „konkreter Registerkandidat“. |
| 13. `dossiers/ni/ni-osnabruck-reinhard-peters-schrottplatz.md` | Rückstufung/GS-Hausnummernkonflikt frei, GS direkt bestätigt. | 2 vs. 2 A ungelöst. Adresse/PLZ/Festnetz und Typ `schrottplatz` nicht als aktueller Hof bestätigt; `pruefung` ersetzt Quellenprüfung nicht. Keine Wahl einer Hausnummer. |
| 14. `dossiers/rp/rp-neustadt-giannotta.md` | Konkreter GS-Lead als Notiz frei, direkt bestätigt. | Kein Vorname/Rechtsträger, kein Betreiber-/Gewerbebeleg; Materialliste ebenfalls nur Lead, keine Fills. |
| 15. `dossiers/rp/rp-gollheim-ayan-gollheim.md` | Börrstädter Gegenbeleg/kein Fill frei, Impressum/Materialankauf direkt bestätigt. | Göllheimer Betreiber/Standortkette fehlen. **Garbliertes aktuelles `street: AYAN Göllheim Göllheim Königkreuzstr. 13` nicht freigeben**; in historische Notiz zurücknehmen, nicht nur Geocoding untersagen. |
| 16. `dossiers/sh/sh-rellingen-25462-klass-m-autoverwertung-und-abschleppdien.md` | Rückstufung/GS-Leads frei; 2018-Bewertung direkt bestätigt. | Voller Name, aktueller Betrieb und Originalzertifikat fehlen. Adlerstr. 36/Typ/City-PLZ-Fragment nicht frisch bestätigt; weitere Verzeichnisse verbessern Quellenstandard nicht. |
| 17. `dossiers/nw/nw-monchengladbach-prison-wetschewell.md` | Instagram-Login/Portalempfehlung als solche frei. | **Besseren konkreten Facebook-/11880-Lead ergänzen**, siehe unten. Wetschewell/Telefon/Rechtsträger weiterhin nicht zweifach belegt; kein aktueller Preis-Fill. Essen nicht als alleiniger wahrer Betrieb fortschreiben. |
| 18. `dossiers/nw/nw-lippstadt-a-r-schrottabholung-lippstadt.md` | Freshness-/FAQ-Korrektur frei, direkt bestätigt. | Alias Autoankauf ↔ Schrottabholung ungeklärt. Dortmunder Impressum direkt HRB 25836/Planetenfeldstr. 118, kein Lippstadtbeleg. Kein Kontakt-/Standort-Fill. |
| 19. `dossiers/nw/nw-nrw-weit-schrott24-metaloop-online-portal.md` | Preis-/AGB-Vorbehalte frei; hier zusätzlich Ablauf und Register erneut geprüft. | Alte `allowed: true`-Portalbedingungen **keine neue operative Freigabe** bei ungeklärter Abwicklung. Betreiber-/Adress-/Insolvenzstatus offen; keine NRW-Annahmestelle oder aktuelle Gebührentabelle. |
| 20. `dossiers/rp/rp-weroth-alldemont-weroth.md` | Register-Delta frei; **Quellenlücken-Aussage korrigieren**, zusätzliche Firmenkontakt-Fills jetzt belegbar. | Betreiberwebsite/HRB-Konkordanz siehe oben. Keine Schrottankauf-/Asbestannahme-/Genehmigungsfreigabe, Organ-/Mergekette weiter offen. |

## Zusätzliche eigenständige Gegenproben

### Iserbrook: DNS genauer als der lokale Resolver

[Google DNS JSON, A-Abfrage](https://dns.google/resolve?name=hamburger-schrott.de&type=A) lieferte HTTP 200, **`Status: 3` (NXDOMAIN)**, negative SOA für `de.`, Kommentar „Response from 194.246.96.1“, `AD: false`. Das ist ein separat lesbarer rekursiver DNS-Befund mit RCODE, keine eigene direkte autoritative Abfrage/DNSSEC-Verifikation. `website_status: tot` für diese Domain plausibel; Betreiberzuordnung und Fortbestand bleiben unbewiesen. Keine Schließung ableiten.

### Müller: fremde Domain nicht übernehmen

Eigenständige Suche fand ein Verzeichnis, das Leichlingen mit `container-mueller.de` verknüpft. Direkt gelesene [Homepage](https://www.container-mueller.de/) und [Impressum](https://www.container-mueller.de/impressum/) nennen **Müller Containerdienst GmbH, HRB Köln 99637**, Verwaltung Manforter Straße 213, Betrieb Dieselstraße 12, Leverkusen, 02171 582820. Das ist kein zweiter Betreiberbeleg für Müller Metallankauf HRB 48784. Keine Register-/Nachfolgekette geprüft, keine Zusammenführung. GS-Leichlingen samt Festnetz/Mobil/2025-Inhaberantwort erneut gelesen; bleibt Aggregator.

### Weiler: ein GS-Nichtfund übersieht vorhandene Leads

[meineoeffnungszeiten, Schrott Merchweiler](https://merchweiler.meineoeffnungszeiten.de/schrott-branche-2078-1) direkt gelesen: Matthias Weiler, Eisenbahnstraße 1a, sowie Bärbel Weiler, Eisenbahnstraße 1. Damit ist „0 Treffer“ nur für die genannte GS-Suche richtig. Kein zulässiger Identitäts-/Aktivbeleg, kein Nachweis einer Familienbeziehung, kein Beleg für die rätselhafte Nr. 46. Rückstufung bleibt richtig; Korrektur der spekulativen aktuellen Namensangabe erforderlich.

### Prison: Preis-Suchlead konkretisiert, nicht aktuell freigegeben

Eigenständige Suche fand [konkreten Facebook-Post](https://www.facebook.com/61586302467268/posts/26012026-vhm25-per-kilo-/122107678911210082/) statt generischer Facebook-Suche. Direktabruf HTTP 200 ohne Login-Weiterleitung in diesem Lauf; auswertbar sind Titel/OpenGraph-Metadaten **„schrottprofi prison“ / „26.01.2026 VHM 25€ per kilo .“**. Das Datum ist zunächst Text der Anzeige, nicht separat verifiziertes Publikationsdatum. Voller Betreiber, Adresse, Güte-/Mengen-/Zeitbedingungen waren aus dieser Antwort nicht auswertbar. Suchindex behauptete Wetschewell; das ist kein direkt gelesener Standortbeleg.

[Konkretes 11880-Profil Mönchengladbach](https://www.11880.com/branchenbuch/moenchengladbach/060440092B112918406/schrotthandel-prison.html) direkt gelesen: Wetschewell 2, 41199, 0171 4039142, Rechtsformbehauptung Einzelunternehmen und Zeiten. **Lead, kein unabhängiger zulässiger Beleg.** Besserer Nachrecherchepfad als die Essen-Empfehlung, aber kein Telefon-/Adress-/Aktiv-/Preis-Fill. Preisbehauptung nicht mehr nur „ohne konkrete URL“; weiterhin historisch, bedingt und dem Seed-Rechtsträger nicht sicher zugeordnet.

### Balzers: zusätzliche Registerdienst-Suche ohne Scheinfreigabe

[Northdata](https://www.northdata.de/?query=Balzers+Metallverwertungsgesellschaft) direkt bestätigt HRB 204441, Limmerburg 11 A, Eintragung 19.08.2015/Dirk Balzer und Gegenstand. Eigenständig gefundenes [Creditreform-Profil](https://firmeneintrag.creditreform.de/31061/2210204849/BALZERS_METALLVERWERTUNGSGESELLSCHAFT_UG_HAFTUNGSBESCHRAENKT) blieb beim Direktabruf **403**. Suchauszug nennt Firma/Branche, ersetzt keinen gelesenen Beleg. DNB/Companyhouse/Verzeichnisse nicht als bequemer zweiter Quellenstempel benutzt. Standort-/Kontakt-Fills weiter gesperrt.

## Preis-/Gebührengate und Schrott24-Legacy

- **Keine neue aktuelle numerische Material-Ankaufsliste zur Ingestion freigegeben.** Keine öffentliche numerische Händler-Verkaufsliste im hier tatsächlich gelesenen Umfang. Das ist eine Umfangsgrenze, kein universeller Nichtveröffentlichungsbeweis.
- RETRACO: AGB 2019 trennen Einkauf/Verkauf richtig; Aufwand, Zwischenlagerung, Lieferkosten und Haftungsprozente sind keine €/kg-Ankaufsliste.
- Meyer: [Service](https://schrotthandel-meyer.de/index.php/service) bestätigt kostenlose Altautoabholung und kostenlose Mulden-Dauerbereitstellung für Geschäftspartner, nicht jede Privat-/Schrottabholung. [Zeiten](https://www.schrotthandel-meyer.de/index.php/oeffnungszeiten) ausdrücklich ab 01.02.2019; kein Frischebeweis.
- Ayan: [Materialankauf](https://www.schrotthandel-ayan.de/materialankauf) nennt Tagespreise ohne Zahlen, nur Börrstadt, kein Göllheimer Kurs.
- All Inn: neue Herner Betreiberquelle nennt 0,00-Euro-Abholung und telefonische Tagespreise; nicht dem Buchholzer Seed zuordnen.
- Prison: neuer konkreter historischer Social-Metadatenfund ist **kein aktueller allgemeiner Kurs**, keine Ingestionfreigabe.

Schrott24 wurde hier stärker als in der Wave erneut geprüft:

1. [Preisseite](https://www.schrott24.de/schrottpreise/) weiterhin 14.11.2025, DE/AT-Maximalpreise/Standort-/Transportabhängigkeit; kein belastbarer aktueller Zahlenkurs im gelesenen HTML.
2. [AGB](https://www.schrott24.de/faq/agb/) bestätigen onlinebezogene Nettopreise, 2/4 Werktage, Storno 5 %/20 EUR, PayPal 2 %, Paket 6 EUR ausdrücklich Österreich. **Nicht als heutige verbindliche NRW-Tarife übernehmen.**
3. [Ablauf](https://www.schrott24.de/ablauf/) unabhängig neu gelesen: Paket 6 EUR ausdrücklich **Österreich und Deutschland**; auf derselben Seite FAQ 5 EUR pro genutzter Marke. Big Bags 5 EUR plus Versand, Eigenanlieferung ab 100 EUR Materialwert. Die geografische Begrenzung der AGB darf nicht zu „kein deutscher 6-Euro-Fund“ verkürzt werden. Konflikt bleibt.
4. Ablauf weiterhin 5 Arbeitstage allgemein versus 48h an anderen Stellen, Adresse erst nach Onlineabschluss. Keine lokale Hofadresse daraus.
5. [Impressum](https://www.schrott24.de/faq/impressum/) Metaloop Europe GmbH/FN 638630m/Puchstraße 17; AGB nennen Schrott24 GmbH. Impressum verlinkt AGB ausdrücklich, enthält aber ebenfalls alte Schrott24-Nennung im Urheberrecht. Namensaltlast ist kein selbstständiger Beweis von Unwirksamkeit; Betreiber-/Geltungsprüfung nötig.
6. [Northdata Metaloop](https://www.northdata.de/?query=Metaloop+Europe+GmbH) erneut gelesen: Herrengasse 13, Graz; Verschmelzung Schrott24 14.11.2025; Insolvenzeröffnung 19.06.2026; am **01.09.2026** gelistete Veröffentlichung eines Beschlusses **28.08.2026**, 27 S 110/26 t, nur gekürztes „Änderung der Bezeichnung …“. Voller Gerichtsbeschluss/Ediktsdatei nicht gelesen; keine Aufhebung, Sanierung oder Betriebsschließung behaupten.
7. [Altmetall kaufen](https://www.schrott24.de/altmetall-kaufen/) führt tatsächlich zu `https://www.metaloop.com/?resource=schrott24`, individuelles Buying/Selling/Logistics, kein numerisches Verkaufsblatt dort.

**Freigabe nur für diese Recherche-/Konfliktnotizen.** Die bereits strukturierten `dropoff_json`/`pickup_json` mit `allowed: true`, privaten Kunden/80+-Netz und Schrott24-PRO werden durch diese Wave nicht neu autorisiert. Websitewerbung ist gelesen, reale aktuelle Durchführung unter Insolvenz-/Adress-/Betreiberkonflikt nicht geklärt. Owner muss heutige Verfügbarkeit separat belegen oder die Bedingungen explizit als ungeklärte historische Betreiberangaben behandeln; keine neuen Partner-/Preis-/NRW-Fills.

## Historien- und Überschreibaudit

Für jedes der 20 Dossiers wurde der alte `## Timeline`-Bereich aus HEAD gegen die Arbeitsdatei verglichen: **alle 74 alten `- `-Bullet-Zeilen sind wörtlich erhalten, kein fehlendes altes Bullet**. Das beweist Erhalt der alten Bullet-Texte, nicht semantische Wahrheit oder vollständige Unverändertheit sämtlicher Formatierung.

Frontmatter-Deltas dieser Wave: RSH Name/Adresse/Kontakte/Website/Status/Description; RETRACO city/Adresse/Website-Status; Status-Rückstufungen Müller, Weiler, Peters, Klass. Keine Slug-/GEO-Deltas in diesen 20 Dossierdiffs. RSH/RETRACO-Überschreibungen sind oben quellenbasiert geprüft. Statuskorrekturen brauchen keinen Schließungsbeleg, weil sie unbewiesene Aktivbehauptungen zurücknehmen, nicht neue Schließungen setzen.

**Owner-Reihenfolge vor Integration:**

1. Freigegebene RSH-/RETRACO-Deltas und vier Statuskorrekturen von vollständiger Dossierfreigabe unterscheiden.
2. ALLDEMONT-Betreiberfund, All-in-Herne-Gegenbeleg und konkreten Prison-Lead datiert ergänzen; alte Timeline erhalten.
3. Unbelegte Legacy-Felder einzeln prüfen/zurücknehmen oder Integration dieser strukturierten Payloads zurückstellen: insbesondere All-in-Adresse/Telefon, Müller-Telefon/„Inh.“-Description, Weiler-Familienspekulation/Adresse, Peters-Hausnummer/Kontakt/Hof-Typ, F.-Weber-Regiontelefon, Klass-Standort/Typ, AYAN-Garblestraße, Iserbrook-Betreibertelefon/-Domainzuordnung. Kein „alle Legacy-Felder bleiben aus Vorsicht“ als Freigabebegründung.
4. Schrott24-Verfügbarkeits-/Bedingungsfelder nicht als neu bestätigte operative Zusage exportieren. Preis-/Gebührenfunde nur mit Quelle, Richtung, Datum und Vorbehalt führen.
5. Erst danach die README-Seed-Tests und eventuelle Produktionsprüfung durch den Owner. **In diesem Audit nicht ausgeführt; technische Integrationsfreigabe ausdrücklich nicht erteilt.**

## Offene Prüfgrenzen

Kein telefonischer Kontakt, kein kommunaler Gewerbeauszug, keine separat gelesenen amtlichen Registeroriginale hinter Northdata-Paywall, kein vollständiger österreichischer Insolvenzbeschluss. Kein originaler RETRACO-EFB, keine visuelle Gegenprüfung des RETRACO-Kontaktbilds oder ALLDEMONT-Zertifikatsbilder, keine RSH-AGB-OCR. Ein nach Research-Skill versuchter zusätzlicher Rechercheagent scheiterte am Subagent-Tiefenlimit; sämtliche berichteten Gegenprüfungen wurden deshalb selbst vorgenommen. Keine Lücke wurde durch eine pauschale Legacy-/Aggregatorfreigabe ersetzt.
