# Unabhängiger Owner-Review: Welle 18.10.2026 02:00, Shards a und b

## Ergebnis und Reichweite

**Alle 40 tatsächlichen Diffs vollständig gelesen; keine pauschale Freigabe.** 37 Dossiers ändern ausschließlich Recherchetext, drei ändern Frontmatter. Historien-/Slug-/Overwrite-Struktur ist in der geprüften Fassung erhalten. Kühn und Tesche wurden zusätzlich unabhängig live nachgeprüft; die Websitezuordnungen sind dadurch wesentlich besser abgesichert, aber nicht mit Annahmevergütung oder vollständiger Betriebsprüfung gleichzusetzen. Altwerte ohne qualifizierte Bestätigung bleiben ausdrücklich offene Datenqualitätsfälle. Der README-Test-/Produktionsgate ist hier **nicht geprüft**, nicht bestanden behauptet.

- Auswahl: `/tmp/opencode/wave-20261018-0200-selection.json`, genau `shards.a` (20) und `shards.b` (20).
- Manifest-HEAD: `10678a919e250aa1cc396ae6a2e39cd4b1aa1afc`; beim Kontrolllauf tatsächlicher HEAD: `4b9bcd32755ea5afba9b2df8476ae0cca10c856c`.
- Für **40/40** stimmen die SHA-256 der aus HEAD gelesenen Ausgangsdateien mit dem Manifest überein. Die HEAD-Fortbewegung verändert damit nicht die hier geprüften Dossierbaselines.
- Vollständige `git diff HEAD -- <Pfad>`-Ausgaben beider Shards gelesen; ergänzend Frontmatter vorher/nachher und Erhalt jeder nichtleeren alten Timeline-Zeile kontrolliert. Git ausschließlich lesend für diese verlangten Vergleiche benutzt; keine Index-/Commit-/Branchänderung.
- Maßstab: README Zeilen 109–134, 162–202, 204–240; ergänzend `docs/trader-seed.md` für Clear-/Keep-Semantik und `crates/ingestion/src/seed_traders.rs` für Enum-Zulässigkeit.
- Keine Dossieränderung, keine GEO-/Produktionsaktion, keine Builds/Tests. Einzige eigene Dateischreibaktion ist dieser Bericht per Patch. Kein Anspruch, alle in 40 Recherchetexten erwähnten Webabrufe selbst wiederholt zu haben; unabhängige Webprüfung gezielt auf die strukturierten Änderungen konzentriert.

## Gates

| Gate | Tatsächlicher Befund | Disposition |
|---|---|---|
| Vollständigkeit | 20 a + 20 b; jede vollständige Diffausgabe gelesen, keine Stichprobe | Erfüllt für diese Fassung |
| Historie | 40/40 alte nichtleere Timeline-Zeilen unverändert enthalten; keine Importhistorie entfernt | Erfüllt |
| Identität | Keine Slug-, Stadt-, Bundesland-, Typ- oder Mergeänderung; HB-Pfad/NI-State bei Seidel bewusst stabil | Struktur erfüllt; zahlreiche Identitäten sachlich offen |
| Frontmatter | Nur Kühn (5 Felder), REWIMET (1), Tesche (5): insgesamt 11 Feldänderungen | Einzelbewertung unten |
| Overwrites | Vier nichtleere Werte ersetzt: Kühn name/website_status/status, Tesche name. Keine Straße/Telefon/GEO überschrieben; Tesche Straße/PLZ waren leer | Kühn benötigt ergänzte Quellenkette im Dossier; Tesche ausreichend belegt |
| Quellen | Aggregatoren, Suchindex, Facebook ohne lesbaren Inhalt, Kartenkopien und Abruffehler werden in den neuen Texten nicht als zwei qualifizierte Quellen ausgegeben | Neue Textgrenzen sinnvoll; keine globale Stammdatenbestätigung |
| Deep Links | Kühn: konkrete Rheinhafen-Seite und konkretes Zertifikat statt Standortübersicht allein. Tesche: Impressum, Service, Über-uns, Kontakt sowie konkreter HRB-Datensatz zitiert | Diese beiden Zuordnungen nachgeprüft; kein universeller HTTP-Linkcheck aller alten/neuen URLs |
| Enums | Alle 40 aktuellen trader_type/state/status/website_status manuell gegen zulässige Kategorien plausibilisiert; neue Werte nur aktiv. Leeres website_status ist im Validator ausdrücklich erlaubt | Kein neuer Enumfehler festgestellt; kein Compiler-/Testnachweis |
| Emdash | In keinem der 40 Frontmatterblöcke U+2014 `—`; keine neue Emdash-Platzhalteradresse/Website. Gedankenstriche in Überschriften/Prosa sind kein Datenplatzhalter | Erfüllt für Frontmatter; kein pauschales Prosa-Verbot unterstellt |
| Clear/Keep | Keine clear_fields-/Service-JSON-/Koordinatenänderungen. Michael-Hammerschmidt-Clears bleiben erhalten | Leere Kontakt-/Servicefelder sind **keine** bewiesene Bereinigung vorhandener DB-Werte |
| Preise | Keine neuen strukturierten Preise. Beispielkurse schrottabholung.org, Registerportalentgelte und Hersteller-Mindestbestellwert werden abgegrenzt | Kein Handler-/Preisfreigabenachweis |
| Tests/Publikation | Nicht ausgeführt, nicht angefragt | Offen; keine Release-/Produktionsfreigabe |

## Besonders: Kühn, Website-/Aktivierungsbeleg

Pfad: `dossiers/bw/bw-karlsruhe-kuhn-entsorgung.md`.

Änderungen: `name` Kühn Entsorgung → Kühn Entsorgung GmbH; leere `website` → Rheinhafen-Deep-Link; `website_status` unbekannt → aktiv; `status` pruefung → aktiv; leere `description` → abgegrenzte Entsorgerbeschreibung. Straße/PLZ/Telefon unverändert. Alte gegenteilige Timeline bleibt erhalten, neue Korrektur ist explizit datiert.

**Ursprüngliche Beleglücke des Diffs:** Gruppenimpressum nennt Rolf Kühl Holding und HRB 11663 Augsburg, nicht Kühn Entsorgung/HRB 716354 Mannheim. Standortseite und gruppengehostetes EFB-PDF sind nicht automatisch zwei unabhängige Quellen aus der README-Liste; auch die autoritative Einquellen-Ausnahme darf ohne Registerkongruenz nicht pauschal beansprucht werden. Im Dossier fehlt bisher ein konkret zitierter Registerabgleich.

Unabhängig am 18.10.2026 gelesen:

1. <https://kuehl-entsorgung.de/standorte/karlsruhe-rheinhafen/> nennt ausdrücklich Kühn Entsorgung GmbH, Nördliche Uferstraße 16/76189, 0721 1831103-0, Gruppenzugehörigkeit, Metallannahme und Anmeldung/Termin. Kein bloßer generischer Kontaktfinder; Zeppelinstraße wird nicht übertragen.
2. <https://kuehl-entsorgung.de/impressum/> bestätigt die oben beschriebene Holding-Impressumgrenze.
3. <https://kuehl-entsorgung.de/wp-content/uploads/2026/08/EFB-Zertifikat_Kuehn-Entsorgung_2026.pdf> vollständig per `pdftotext -layout` gelesen: bvse, Nr. 10708/ZZEE001003878008, Kühn/Adresse, HRB 716354 Mannheim, Prüfung 12.06.2026, Ausstellung 28.07.2026, gültig bis 31.12.2027; Anlagen nennen denselben Standort. Textauslese bildet Auswahlkästchen nicht verlässlich ab, daher keine Freigabe aller gelisteten Tätigkeiten.
4. **Zusätzlicher unabhängiger Registerabgleich dieses Reviews:** <https://www.northdata.de/K%C3%BChn%20Entsorgung%20GmbH,%20Karlsruhe/Amtsgericht%20Mannheim%20HRB%20716354> direkt gelesen: Firma, HRB, Nördliche Uferstr. 16/76189 kongruent; Gegenstand metallische Rohstoffe, Schrott-/Buntmetall-An-/Verkauf, Entsorgung/Container. Publikation 11.03.2026 nennt Rolf Kühl Holding als Gesellschafter, 20.11.2025 Geschäftsführung/Prokura. Damit unabhängiger Anker auch für die Gruppenbrücke, kein Namensvetter-Merge nötig.

**Disposition:** Websiteidentität und Namenspräzisierung sind jetzt auf Review-Ebene zweiquellig nachvollziehbar; laufender Entsorgungsbetrieb wird zusätzlich durch datiertes Zertifikat und Standortangebot gestützt. Der zunächst ungeklärte Registerbeleg ist damit hier nachgeholt, **aber noch nicht im Dossier dokumentiert**. Vor uneingeschränkter Overwrite-Abnahme Quellenkette dort nachtragen lassen, ohne ältere Historie zu entfernen. `aktiv` nur als operativer Entsorger, nicht als garantierter bezahlter Privatankauf verstehen. Keine Ankaufvergütung, Kundentypen oder uneingeschränkte kostenlose Annahme freigegeben. Keine Dossierkorrektur in diesem Review vorgenommen.

## Besonders: Tesche, Website-/Identitätsbeleg

Pfad: `dossiers/nw/nw-wuppertal-heckinghausen-tesche-co.md`.

Änderungen: `name` Tesche & Co GmbH → Tesche & Co. Rohstoffverwertung GmbH; leere Straße/PLZ → Auf der Bleiche 26/42289; leere Website → `https://www.tescheco.de/`; leeres website_status → aktiv. `status: pruefung`, Kontakte/Zeiten/Service-JSON bleiben unverändert leer. Kein Stadtteil-/Slugwechsel.

Unabhängig am 18.10.2026 gelesen:

- <https://www.tescheco.de/impressum/>: Tesche & Co GmbH, Günter/Marlon Tesche, Auf der Bleiche 26, 42289, 54 HRB 3508 Wuppertal. Direkter Betreiberinhalt, nicht nur Portal-Auszug.
- <https://www.northdata.de/Tesche%20&%20Co%C2%B7%20Rohstoffverwertung%20GmbH,%20Wuppertal/HRB%203508>: vollständiger Name, HRB 3508, identische Adresse und Schrott-/Metallgegenstand; Abschluss 2024 veröffentlicht 11.08.2025. Keine aus Namensähnlichkeit erfundene Nachfolge erforderlich.
- <https://www.tescheco.de/kontakt/>: Öffnungszeiten Mo–Do 06:30–13:30/Fr 06:30–13:00 versus ausdrücklich Bürozeiten Mo–Do 08:00–15:30/Fr 08:00–14:30 im Footer. Unterschied im Dossier korrekt erhalten.

**Disposition:** Der angefragte ungeklärte Websitebeleg ist für die Identitätszuordnung durch direkten Impressums-/HRB-Abgleich unabhängig geklärt; fünf konkrete Feldänderungen sind entsprechend nachvollziehbar. **Weiter ungeklärt** sind heutige Annahme-/Auszahlungsbedingungen, Privatkundenzugang, aktuelle EFB-Zertifizierung und betriebliche Aktualität undatierter Websiteinhalte. Website aktiv bedeutet hier erreichbar, nicht Betrieb abschließend aktuell verifiziert. Prüfstatus sachgerecht; keine autoritative Einquellen-Ausnahme oder Kontakt-/Zeit-Freigabe abgeleitet.

## REWIMET: einzige weitere Frontmatteränderung

`dossiers/ni/ni-goslar-clausthal-zellerfeld-rewimet.md`: nur leere description wird zur vorsichtigen Netzwerkbeschreibung ohne belegten Schrottankauf/öffentlichen Annahmeplatz. <https://www.rewimet.de/> zusätzlich selbst gelesen: Innovationsnetzwerk REWIMET e.V., Geschäftsstelle Clausthal-Zellerfeld; Recyclingkapazitäten ausdrücklich diejenigen der Mitglieder. Beschreibung ist sachgerecht abgegrenzt, kein nichtleerer Overwrite und keine fiktive Schließung. Vereinsregister-/Zweitquellenstandard für weitere Stammdaten dadurch nicht ersetzt; Nicht-Händler-Darstellung bleibt Owner-Folgefrage.

## Einzelnachweis aller 40 gelesenen Diffs

`T` = nur Text, kein Frontmatterdelta; `F` = oben einzeln bewertetes Frontmatterdelta. „Text vertretbar“ bestätigt die konservative Dokumentation, **nicht** Existenz/Aktivität oder sämtliche historischen Stammdaten. Die folgenden Slugs bezeichnen exakt die Manifestpfade; Ausnahme HB/NI bewusst wie im Manifest.

### Shard a

| Dossier (Slug) | Delta | Einzelbefund / verbleibende Grenze |
|---|---|---|
| by-munchen-autoverwertung-koller | T | Construction-Seite/.net-DNS und München–Emmering-Telefonkonflikt dokumentiert; keine Zweigstelle behauptet. Text vertretbar, Identität/Hof offen. |
| by-nurnberg-mogeldorf-schrott-b | T | Kurzname/Familienname-Risiko und bloßer GS-Lead klar; keine Aktivierung. Vollständiger Gewerbename offen. |
| by-ichenhausen-michael-hammerschmidt | T | Index-Impressum Michael versus Stuttgart nur Lead, TLS verhindert Direktbeleg; Clears bleiben. Keine Vermischung mit Emil/Philip. |
| by-furth-nock-sohne | T | Überblick korrigiert Personen- versus Firmensuchfilter; frühere Timeline erhalten. Gewerbeeintrag bleibt Lead, kein Kontakt-Fill. |
| by-kummersbruck-schrott-christian | T | Namens-/Rubrikrisiko, Vereinsfunktion kein Gewerbebeleg, 429 kein Registerurteil. Historische Kontakte weiterhin ungeprüft. |
| by-alzenau-felbinger | T | Einzelne Firmengeschichte Mühlheim gelesen laut Text; kein Alzenau-Umzug erfunden. Seed-Brücke offen. |
| by-pressath-92690-dahms-falk-av | T | 403 und aktuelle Portalwiederholung nicht als Zulassung gewertet; blockiert/pruefung bleiben. Behörden-/Betreiberbeleg offen. |
| by-wenzenbach-meier | T | Überblick erweitert Gerhard/Michael/Führsamweg-Konflikt; keine Generationen-/Umzugskette behauptet, Adresse bleibt leer. |
| bw-karlsruhe-kuhn-entsorgung | F | Fünf Felder; siehe besondere Quellen-/Overwritebewertung. Registerergänzung im Dossier noch ausstehend. |
| bw-ohmden-wiedmann-gucklhorn-schrott-und-metallhan | T | Unvollständiges Portal und Suchlimits offen benannt; kein Schließungsurteil. Vollständige Inhaber ungeklärt. |
| bw-schwetzingen-georg-laubinger | T | Website ohne HRB, alte Rechts-/Zertifikatswerbung und Teileverkauf sauber begrenzt. Zweite zulässige Identitätsquelle fehlt. |
| bw-friedrichshafen-regu-recycling-metallhandels | T | Hof-/Registeradresslücke bleibt, Euro-6 kein EFB, Zinn/Zink-Mix nicht normalisiert. Alter Aktivstatus nicht neu verifiziert. |
| by-rotal-schubert | T | Hersteller Otto Schubert nicht zum Händler gemacht; 20 EUR Mindestbestellwert kein Schrottpreis. Ursprungsidentität offen. |
| by-thalmassing-die-entsorger | T | Münchener-/Landeckstraße und Telefonkonflikte nicht zugunsten Portal überschrieben. Kein Negativurteil aus Suchlimits. |
| bw-offenburg-intlekofer-schrott | T | Hausnummer 27/66 beidseitig dokumentiert, gleiche Nummer keine Umzugsbrücke; Straßenfeld bleibt leer. Alte PLZ/Telefon nicht primär bestätigt. |
| by-ichenhausen-emil-philip-hammerschmidt | T | Michael-Index ausdrücklich Gegenwarnung, nicht Emil/Philip-Beweis; Kandidatendomain bleibt unbestätigt. Seed-Personenmischung offen. |
| bw-kabelstr-metallrecycling-muller | T | Kein Ort aus Kabelstr.-Fragment/Nobitz geraten; Importgeschichte bleibt erhalten. Originalzeile benötigt. |
| bw-tauberbischofsheim-distelhau-griesbaum-roland-schrotthandel | T | H. Distelhausen versus Roland Boxberg nicht zusammengeführt; Fremdbetriebs-Containerwerbung kein Ziel-Nulltarif. |
| bw-offenburg-hancer-nazmi-metallhandel | T | Creditreform-403/GS-Lead, zweite Nummer ohne Faxdeutung; keine neue Aktivierung. Zweitbeleg fehlt. |
| by-peienberg-steindl | T | Zwei Rechts-/Ortskandidaten getrennt; unscharfe Kranwerbung und Containerkosten nicht als feste Grenzen normalisiert. Seed-Brücke offen. |

### Shard b

| Dossier (Slug) | Delta | Einzelbefund / verbleibende Grenze |
|---|---|---|
| ni-goslar-clausthal-zellerfeld-rewimet | F | Nur description; Netzwerk statt Händler, unabhängig auf Eigenwebsite plausibilisiert. Keine Löschung/Schließung. |
| ni-delmenhorst-bartels-metallhandels-auch-achim | T | Bremen/Umkreistreffer nicht Filiale; alter aktiv bleibt trotz unbelegtem Zielort. **Altstatus-Folgeprüfung erforderlich.** |
| ni-wolfsburg-kvj-dienstleistungen-schrottplatz | T | Neuer 403 kein alter Proxy-Inhalt; Cylex kein Zweitbeleg. **Website-/Betriebs-aktiv und tatsächlicher Hof nur Altstand.** |
| nw-bergisch-gladbach-krau | T | Branchenklassifizierung des Adolf-Krauß-Leads präzisiert, nicht als Seed-Identität behandelt. Alte Timeline bleibt. |
| hb-stuhr-thomas-seidel-schrotthandel | T | Telefonbuchkopien kein Gewerbebeleg; HB-Pfad/state NI erhalten. Straße/Identität offen. |
| ni-nienburg-gebruder-schaer-schrott-metalle | T | Hagenburg versus Landkreis Nienburg, Hausnummer 2/3 nicht als Filiale/Verlegung verwendet. Seed-Brücke fehlt. |
| ni-wilhelmshaven-wilfried-seeberg-schrott-metallgrohandel | T | Petra-/HRA-Anker historisch, Tonndeichstraße versus Mühlenweg offen. **Alter aktiv/Adresse/Zeiten nicht nachverifiziert.** |
| nw-bergisch-gladbach-rehbach | T | HRB-Kandidat i.L. 2025 und drei Anschriften differenziert; Liquidation nicht mit Platzschließung gleichgesetzt. Betreiberfortführung offen. |
| nw-monchengladbach-kreggenwinkel | T | Facebook-Snippet ohne Direktinhalt nicht als Betreiberprofil verifiziert. **Alte Aktivierung/Kontakte weiter nur Leads.** |
| nw-koln-entschrotter | T | Müller/Adler-Gegenidentität erhalten, Jimdo-Transportfehler kein Schließungsnachweis; keine Mail-/Websiteübernahme. |
| ni-hameln-31785-schmitz-autoverwertung | T | Alte Umweltprämienliste kein aktueller Zulassungsbeleg; nicht Celle. Neue Notiz nennt pruefung, ältere aktiv-Notiz ist als Historie unverändert erhalten, tatsächliches Frontmatter pruefung. |
| ni-horden-am-harz-klaus-schleifer-schrott-metallhandel | T | Ortslisten/Schrottportal keine unabhängigen Gewerbebelege; andere Harzer Gesellschaft nicht zugeordnet. Alte Straße unbestätigt. |
| nw-witten-vb | T | Konkreter neuer Kandidat samt Portalbadge vorsichtig, kein Inhaber aus Mail geraten. Keine Umbenennung/Fills. |
| ni-upgant-schott-gerhard-czaja-schrotthandel | T | Vereinsheft 2007/OCR nur historischer Kandidat; kostenlose Abholung kein aktueller Tarif, Mobilnummer nicht übertragen. |
| nw-nrw-weit-schrottabholung-org-netzwerk-0157 | T | 14/4/6/0,30 EUR/kg ausdrücklich undatierte Beispiele, kein Handler. Einzelquelle und kostenpflichtige Zusatzarbeiten nicht kaschiert. |
| ni-luchow-autoverwertung-krause | T | Feuerwehrübung nicht echter Brand, lokaler Hinweis kein Gewerbe-/Adressbeweis; Kleinanzeigen ohne Impressum nicht zugeordnet. |
| ni-neustadt-am-rubenberge-max-kurt-gust | T | Registerumzug 2014 datiert statt aktueller Annahmenachweis; 29,80 EUR fremde Dokumentgebühr ausgeschlossen. |
| nw-monchengladbach-entsorgungsprofi | T | Büro ausdrücklich kein Hof, Geräte-Ausschlüsse und generischer Seed nicht pauschal verbunden; kein Zweitbeleg. |
| ni-hehlen-schrotthandel-bernd-fricke | T | Kartenkonsens, Portal-Geschlossen/Badge nicht zu Betriebsschließung/EFB hochgestuft; Haushaltswaren-Widerspruch offen. |
| nw-wuppertal-heckinghausen-tesche-co | F | Fünf Felder zweiquellig unabhängig bestätigt; Betrieb/Annahme/EFB weiterhin offen, pruefung bleibt. |

## Offene Owner-Punkte statt Sammelfreigabe

1. Kühn: hier nachgeholten konkreten Northdata-Abgleich in die Dossierbegründung integrieren lassen; erst damit ist die dokumentierte nichtleere Overwrite-Kette in der eigentlichen Seedquelle vollständig nachvollziehbar. Kein Rückgängigmachen bestehender Arbeit durch diesen Review.
2. Tesche: Identitäts-/Website-/Adressfill ist belegt; die ausdrücklich offenen Betriebs-/Annahme-/Zertifikatsfragen nicht als durch Websiteerreichbarkeit erledigt behandeln.
3. Unveränderte Altaktivierungen insbesondere Bartels, KVJ, Seeberg, Kreggenwinkel sowie REGU und der Einzelquellenfall schrottabholung.org sind **keine Erfolge dieses Reviews**. README verlangt bei berührten Aggregatorfeldern Nachverifikation; die neuen Notizen dokumentieren hier überwiegend deren Fehlen. Bloßes Bewahren ist historisch sauber, aber keine fachliche Freigabe dieser Werte. Weitere Entscheidungen separat mit Evidenz und ggf. eigener Clear-/Statusprüfung treffen.
4. 37 Textdeltas sind vorsichtige Recherchefortschreibung, keine 37 abgeschlossenen Identitätsprüfungen. Suchfehler/403/429/TLS bleiben Abrufgrenzen.
5. Nach jeder weiteren Bearbeitung die tatsächlichen Diffs erneut prüfen. Seed-Tests und Produktionsverifikation bleiben außerhalb dieses strikt lesenden Reviewauftrags offen.
