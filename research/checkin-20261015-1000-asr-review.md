# ASR Coswig: Integrationsreview der vorbestehenden Änderungen

Stand: 15.10.2026. Geprüft: `dossiers/sn/sn-coswig-asr-autoteile-service-reifen.md` gegen HEAD `ebc1d15ce1ec7910a4d5a150a3e9f25f8a0eaa9d` (`git show HEAD:…`, aktueller `git diff`). Maßstab: README, insbesondere Zeilen 120–129, 165–199 und 217–237. Sämtliche nachfolgenden Webseiten wurden in diesem Review direkt abgerufen; Suchtreffer dienen nur als Leads.

## Entscheidung

**Noch nicht unverändert integrationsreif.** Die wesentlichen neuen Betreiber-, Standort- und Leistungsdaten sind direkt bestätigt. Es gibt aber eine konkret vorhandene, im Diff nicht behandelte Geschwisterzeile für denselben Standort. Die pauschale Behauptung, der Identitätskonflikt sei vollständig aufgelöst, geht deshalb über die dokumentierte Prüfung hinaus. Außerdem ist das vorgeschriebene Seed-Testgate hier nicht ausführbar.

Kein Anlass, die belegten Kontakt-/Adressdaten als falsch zurückzuweisen. Freigabe nach dokumentierter Geschwister-/Identitätsentscheidung, kleinen Präzisierungen und bestandenem Testgate; keine automatische Zusammenführung, Löschung oder Slugänderung.

## Direkt verifizierte Belege

| Quelle | Gelesener Befund / Reichweite |
|---|---|
| [Betreiber-Impressum](https://www.autoneuteile.de/impressum.html) | ASR GmbH & Co. KG; Fichtenbreite 1, 06869 Coswig; Enith und Melanie Dirscherl; Stendal HRA 5528 und HRB 24922; 034903-4700; info@autoneuteile.de. Die beiden Registernummern nicht als zwei Nummern derselben KG interpretieren. |
| [North Data, HRA 5528](https://www.northdata.de/ASR+GmbH+%26+Co.+KG,+Coswig/Amtsgericht+Stendal+HRA+5528) | Name, HRA, EUID DEW1215.HRA5528 und Fichtenbreite 1, 06869 Coswig (Anhalt) kongruent. Publikation 08.07.2026: Jahresabschluss zum 31.12.2024. Eintragung 07.11.2017, persönlich haftende Gesellschafterin ASR Verwaltungs-GmbH. Kein selbst eingesehener amtlicher aktueller Registerauszug; Jahresabschluss allein ist kein lückenloser Fortbestandsbeweis. North Data ist hier eine laut README zugelassene Registerauswertung, nicht die Betreiber-Primärquelle. |
| [Öffnungszeiten / Anfahrt](https://www.autoneuteile.de/?p=oeffnungszeiten) | Vollständige Startseite mit Öffnungszeiten Mo–Fr jeweils 08–17 Uhr; Anfahrt nennt dieselbe Firma/Adresse sowie ausdrücklich `[51.887961, 12.403071]`. Kein unabhängig vermessener Torpunkt. |
| [Stadt Coswig: Klieken mit OT Buro](https://www.coswigonline.de/de/klieken-mit-ot-buro.html) | Führt Bebauungsplan Nr. 07 „Gewerbegebiet Fichtenbreite“, Klieken. Konkreter kommunaler Beleg für den Ortszusammenhang, nicht bloß die im Diff pauschal verlinkte Stadt-Startseite. Belegt allein keinen ASR-Betreiber. |
| [Schrottankauf](https://www.autoneuteile.de/schrottankauf.html) | Metallschrott, Batterien, „Edelmetalle wie Kupfer“ und noch brauchbare Reifen kostenlose Annahme, errechneter Schrottpreis sofort bar; kleine Entsorgungsgebühren für Schrottreifen und Altöl. Keine Zahlenkurse auf dieser Seite. Kupfer ist fachlich kein Edelmetall; die unpräzise Betreiberwortwahl nicht als Materialklassifikation übernehmen. |
| [Autoverwertung](https://www.autoneuteile.de/autoverwertung.html) | Selbstauskunft: zertifizierter Demontagebetrieb nach § 5 Abs. 3 AltfahrzeugV; vollständige Fahrzeuge kostenlose Annahme, Verwertungsnachweis, kein Termin; Abschleppen nach Absprache. Kaufangebot nur in bestimmten Fällen, z. B. neuere/gefragte Fahrzeuge. Kein kostenloser Transport zugesagt. |
| [Auto An- & Verkauf](https://www.autoneuteile.de/auto-an-verkauf.html) | „Sofortankauf ab 250 €“, anschließend Auswahlmerkmale und Besichtigung/individuelles Angebot. Kein bedingungsloser Mindestankaufspreis für jedes Schrottauto. Auch Verkauf ausgewählter Gebrauchtfahrzeuge. |
| [Teileverkauf](https://www.autoneuteile.de/neu-gebrauchtteileverkauf.html) | Genauer Wortlaut: „Endschalldämpfer ab 25 €, Rohre ab 12 €“ innerhalb der Auspuff-Neuteile. Stück-/Angebotspreise, keine Materialankaufskurse. |
| [AGB](https://www.autoneuteile.de/allgemeine-geschaeftsbedingungen.html) | § 4: Käufer trägt Versandkosten unabhängig vom Bestellwert; § 1 beschränkt den Text auf Fernabsatz gebrauchter Ersatzteile/Aggregate, Ladenbedingungen separat. Ausgewiesener Stand Oktober 2019, daher nicht als aktuell datiertes Zertifikat oder Preisblatt behandeln. |
| [Firmengeschichte](https://www.autoneuteile.de/firmengeschichte.html) | Betreiber beschreibt Standorteröffnung 1991, später ASR eK → GmbH und Unternehmensbezeichnung GmbH & Co. KG seit 2018. Das ist eine Betreibererzählung, keine vollständig durch Registerpublikationen bewiesene Rechtsnachfolgekette; North Data zeigt bereits KG-Eintragung 2017. |

## Betreiber-/Standortkette und entscheidender Integrationsmangel

1. Für den **heutigen Betrieb in Coswig (Anhalt)** ergibt sich eine starke Kette: Betreiber-Impressum ↔ HRA/Adresse bei North Data ↔ standortbezogene Anfahrt/Öffnungszeiten ↔ kommunaler Zusammenhang Fichtenbreite/Klieken. SN ist für diesen Betrieb falsch, ST richtig.
2. Das beweist nicht allein, dass jede ursprünglich im sächsischen Importabschnitt stehende ASR-Zeile dieselbe Entität ist (README: Impressum allein reicht nicht). HEAD enthielt bereits den State-Mismatch als offenen Klärfall; die Seed-Provenienz „Meißen, Radebeul, Coswig …“ bleibt ein Gegenindiz zur ursprünglichen Zuordnung, kein Nachweis einer sächsischen Filiale.
3. Korpusprüfung fand **`dossiers/st/st-coswig-asr-klieken.md`**: gleicher Betreiber, Straße, PLZ, Telefon, Mail, Domain und exakt gleiche Koordinaten. Das Geschwisterdossier erklärt schon ausdrücklich, dass Coswig/Klieken keine zwei nachgewiesenen Filialen sind. Es ist bereits in HEAD vorhanden, nicht Teil des angefragten Diffs. Nachbar `st-coswig-asa-altmetallhandel.md` betrifft dagegen ASA an Fichtenbreite 2 und darf nicht aufgrund der Ähnlichkeit vermengt werden.
4. Im geprüften Diff fehlt jeder Querverweis auf `st-coswig-asr-klieken`. Damit würden zwei Slugs denselben Platz beschreiben, ohne offengelegte Beziehung; zugleich `aktiv` hier versus `pruefung` dort. README verlangt ausdrücklich Geschwisterprüfung und Querverweise statt stiller Zusammenführung.
5. Gegenprüfung per Websuche `"ASR" "Coswig" "01445" Autoverwertung` ergab Leads zum Anhalter Betreiber, keinen belastbaren Nachweis eines zweiten sächsischen ASR. Das ist **kein Negativbeweis**. Vor Freigabe muss die frühere Sachsen-Zuordnung nachvollziehbar als Fehlzuordnung eingeordnet oder als weiter offen markiert werden, einschließlich Geschwisterbeziehung. Keine erfundene Verlagerung SN→ST und keine aus der Firmengeschichte abgeleitete unbelegte Rechtsnachfolge.

## Overwrites, Historie und Seedwirkung

- Nichtleere Änderungen: `state: SN → ST`, `city: Coswig → Coswig (Anhalt)`, `status: pruefung → aktiv`. Adresse/Kontakt/Website/Zeiten/Description waren leer; GEO wird neu gesetzt. Keine bestehende belegte Adresse wird überschrieben.
- ST/Ortspräzisierung sind für den identifizierten Betreiber zweifach gestützt. Die **Zuordnung der importierten Zeile** zu diesem Betreiber und zum Geschwister muss zusätzlich dokumentiert werden. `aktiv` ist als operativer Betreiberstatus plausibel, aber im Diff zu pauschal begründet und gegenüber dem Geschwister nicht reconciliert. Fehlende numerische Gebühren allein erzwingen keinen Prüfstatus.
- Die strenge README-Ausnahme für eine autoritative Betreiberseite verlangt auch aktuelle Inhalte/Zertifikate/Daten. Registerkongruenz und standortbezogene Seiten sind vorhanden; ein aktuell gültiges Zertifikat mit Ablaufdatum wurde hier nicht eingesehen. Copyright 2026 ist kein eigenständiger Aktualitätsnachweis. Die Adresse hat ohnehin Betreiber- und North-Data-Beleg; die allein vom Betreiber stammenden Detailangaben als solche kennzeichnen. Zertifizierung in Description besser ausdrücklich „laut Betreiber“, sofern keine gültige Bescheinigung ergänzt wird.
- Slug und Dateipfad bleiben unverändert: richtig, trotz `sn-` und Ablage in `sn/`; keine kosmetische Umbenennung.
- Import-Bullet „anerk. Altfahrzeug-Demontage“ und vollständiger Rechercheblock 01.10. bleiben erhalten. Nur inhaltsloses `—` und Überblick-Platzhalter entfallen. Die Entfernung ist separat datiert dokumentiert; keine relevante Historie gelöscht, neue Fakten nicht rückwirkend in den Import geschrieben.
- Provenienzfelder bleiben erhalten. Das ist für Nachvollziehbarkeit richtig; sie dürfen nicht als heutige Standortbeschreibung missverstanden werden.
- GEO-Paar stimmt exakt mit Betreiber-Anfahrt überein. Laut README überschreibt es bei Seedänderung bestehende DB-Koordinaten. Der Diff bezeichnet es als Betreiber-Kartenmarker, behauptet also keine Vermessung; noch klarer wäre der Genauigkeitshinweis direkt im Timelineblock.
- `dropoff_json` / `pickup_json` bleiben leer: keine unbelegten Kundenklassen, Mengen oder pauschal kostenlose Abholung strukturiert erfunden.

## Ankauf / Verkauf / Gebühren: Bewertung

**Im Kern korrekt getrennt.** Keine numerischen Schrott-EUR/kg-Kurse gefunden auf den geprüften Seiten. 250 € ist Fahrzeugankauf nach Besichtigung und Auswahl, 25 €/12 € sind Teile-Verkaufspreise. Die kostenlosen vollständigen Altfahrzeuge und kostenpflichtigen Schrottreifen/Altöl werden nicht vermengt. Versandkosten sind keine Schrott-Annahmegebühr.

Vor Integration präzisieren: „Endschalldämpfer ab 25 €“ statt pauschal „Auspuff-Neuteile ab 25 €“; Fahrzeugankauf ausdrücklich zustands-/angebotsabhängig, nicht Garantie für jedes Fahrzeug; „Kupfer“ neutral als Metall statt als Edelmetall klassifizieren. Der Timeline-Satz, der für alle genannten Annahmearten Auszahlung behauptet, folgt zwar dem pauschalen Betreibertext, sollte keine gesicherten Einzelkonditionen für Reifen/Batterien suggerieren. Keine Gebührenhöhe und keinen Nulltarif fürs Abschleppen ergänzen.

## Konkrete Freigabegates

1. **Inhaltlicher Blocker:** Geschwister `st-coswig-asr-klieken` im geprüften Dossier ausdrücklich referenzieren, denselben Standort und die alte Sachsen-Provenienz bewerten; Statusentscheidung nachvollziehbar machen. Nicht als zwei belegte Betriebsstätten ausgeben, nicht eigenmächtig mergen/löschen.
2. **Redaktionell vor Freigabe:** obige Preis-/Materialpräzisierungen und Betreiber-Selbstauskunft zur Zertifizierung; kommunalen Deep-Link statt bloßer Startseite verwenden. Keine unbewiesene historische Betreiberkette ergänzen.
3. **Technischer Blocker:** `cargo test -p schrott-mcp-ingestion seed` wurde versucht, scheiterte sofort mit `/bin/sh: 1: cargo: not found` (Exit 127). Keine Tests ausgeführt, kein bestandener Corpus-/Buildnachweis. Mit verfügbarer Rust-Toolchain auf dem tatsächlichen Integrationsstand wiederholen. Der Test würde ohnehin den gesamten aktuell veränderten Korpus prüfen, nicht isoliert ASR.

## Eingriffsgrenze

Nur dieser Reviewbericht wurde geschrieben. Dossier und andere vorbestehende Änderungen unangetastet; kein Commit, Push oder Deploy. Keine Produktionsdatenbank geöffnet oder verändert. Dieser Bericht ist eine Integrationsentscheidung mit offenen Gates, kein Nachweis einer Produktionsveröffentlichung.

## Owner-Nachprüfung

Bericht vollständig gelesen. Der technische Blocker ist auf den PATH der Agentensitzung beschränkt: Im selben Check-in lief `/root/.cargo/bin/cargo test -p schrott-mcp-ingestion seed` beim Owner erfolgreich (14 bestanden, 0 Fehler). Das ist ein Gesamtkorpustest des gemeinsamen Arbeitsbaums, keine isolierte ASR-Freigabe. Die inhaltlichen Blocker bleiben davon unberührt. Keine Integration des Dossierdiffs freigegeben; zuerst Geschwisterbeziehung und ursprüngliche Sachsen-Zuordnung sowie die genannten Quellen-/Preispräzisierungen klären. Geschützte Dossiers weiterhin unverändert.
