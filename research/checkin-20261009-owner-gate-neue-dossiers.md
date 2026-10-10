# Owner-Gate: acht neue Dossiers und zwei Bestandsänderungen

Stand: 09.10.2026. Auftrag: Identität, aktuelle Tätigkeit, Namensvetter/Dubletten, Quellen und Frontmatter vor Integration prüfen. **Die zehn Dossiers wurden nicht verändert.** Keine Builds, Tests oder Deployments ausgeführt.

## Ergebnis / Integrationsentscheidung

Keine pauschale Freigabe aller zehn Dateien. Vier belastbare Identitäten können integriert werden; bei weiteren Einträgen sind Quellenpräzisierungen oder ein expliziter Prüfstatus nötig. Eine erreichbare Website ist kein Beweis für aktuelle Zertifizierung, eine Registerfirma kein Beweis für sämtliche Annahmebedingungen.

| Dossier | Owner-Gate | Vor Integration |
| --- | --- | --- |
| RPR Frankfurt (Oder) | Identität und Ankauf bestätigt; Nachfolgergeschichte nicht erneut live belegbar | Vorgängerbehauptung als historischen/Indexbefund kennzeichnen oder Originalbeleg sichern; siehe 1 |
| Hermann Korn Hirschaid | Identität/Adresse bestätigt; Aktualitätskriterium der Owner-Ausnahme offen | Register-Abrufalter und abgelaufenes Zertifikat dokumentieren; Kontakt-/Bedingungsfelder nicht pauschal freigeben; siehe 2 |
| ASM Dorsten | Nur Prüfdatensatz; Betreiberidentität und Standort-Zweitbeleg hier nicht abschließend verifiziert | Social-Belege mit sichtbaren Angaben sichern oder Adresse/Annahme ins Arbeitswissen zurücknehmen; siehe 3 |
| Euromet Structured Düsseldorf | Identität und B2B-Ankauf bestätigt | Mobilmarke weiter strikt ausklammern; leere Kontaktdaten zulässig; siehe 4 |
| Pietsch Kleve | Freigabefähig | Aktuelles Zertifikat als Owner-Ausnahme ergänzen; siehe 5 |
| Hein Kreuztal | Zurückhaltender Prüfdatensatz plausibel | Creditreform-Telefon nur im Suchindex erneut bestätigt; Zweitbeleggrenze ausdrücklich notieren; siehe 6 |
| SK Menden | Identität/Adresse und Ankauf bestätigt; Kontaktdaten nur teilweise zweifach belegt | Fehlendes erreichbares Impressum verhindert Owner-Ausnahme; Mobil/Mail/Zeiten nachbelegen oder leer lassen; siehe 7 |
| Bierhoff Sundern | Freigabefähig | Northdata als erneut gelesenen Registerabgleich ergänzen; HRA/Komplementär-HRB auseinanderhalten; siehe 8 |
| Pietsch Lichtenow | Änderung `aktiv` → `pruefung` freigeben | Keine Klever Daten übertragen; indexiertes altes Betreiber-Impressum als neue Spur dokumentieren; siehe 9 |
| Papierfritze Berlin | Neue Leistungsbeschreibung belegt, gesamtes Frontmatter noch bereinigungsbedürftig | `street` auf `Alt-Biesdorf 12` begrenzen; Marken-/Standortdublette explizit dokumentieren; siehe 10 |

## Maßstab und Prüfgrenzen

- Grundlage: README, insbesondere „Feedback triage“, Punkte 6–7, „Operator rename/merge chains“ und Owner-Gate. Verzeichnisse zählen nicht als Beleg. Zwei unabhängige qualifizierende Quellen oder vollständig erfüllte Betreiber-Ausnahme erforderlich. Mehrere Unterseiten derselben Website sind nicht mehrere unabhängige Quellen.
- Betreiber-Ausnahme verlangt Registerkongruenz, standortbezogene Angaben und aktuelle Inhalte. Ein automatisch fortgeschriebenes Copyright-Jahr allein wurde hier **nicht** als Aktualitätsbeleg behandelt.
- Betreiberseiten wurden direkt gelesen; Northdata wurde als vom README ausdrücklich zugelassener Registerindex genutzt, nicht als amtlicher Originalauszug ausgegeben. Creditreform-Detailabrufe lieferten bei SK, Pietsch, Bierhoff und Hein HTTP 403. Frühere Agentenbehauptungen hierzu sind dadurch weder widerlegt noch in diesem Gate vollständig reproduziert.
- Suchindexauszüge sind als solche markiert, keine vorgetäuschten Live-Abrufe. Ein 404 oder NXDOMAIN beweist keine Betriebsschließung. Preislosigkeit bedeutet hier: keine numerische Metallankaufpreisliste auf den geprüften relevanten Seiten; kein Vollbeweis über alle denkbaren Publikationen.
- Lokaler Namens-/Adressabgleich im Dossierkorpus einschließlich der genannten Geschwistereinträge. Keine Produktionsdatenbank abgefragt; Aussagen über fehlende Dubletten gelten für die Dateien, nicht als SQL-Audit des Live-Bestands.
- Zusätzlich statische Prüfung aller zehn Frontmatter: bekannte/eindeutige Schlüssel entsprechend `crates/ingestion/build.rs`, Dateiname gleich Slug, lesbare flache Skalare und gültiges JSON in befüllten Bedingungsfeldern. Ohne Auffälligkeit. Das ist **kein** Ersatz für den späteren Seed-Test.
- Die Bestandsdiffs bewahren die bisherige Timeline: Lichtenow ändert nur Status und ergänzt Recherche, Papierfritze füllt Beschreibung und ergänzt Recherche. Keine historische Recherche entfernt. Für die acht Neuanlagen kein alter Slug überschrieben; keine GEO-Werte ergänzt.

## 1. RPR Frankfurt (Oder)

Datei: `dossiers/bb/bb-frankfurt-oder-rpr-gmbh-recycling-park-rohstoffhandel.md`

**Bestätigt:** Betreiber-Startseite nennt ausdrücklich Ankauf von Buntmetallen, Schrott und NE-Metallen, Privat-/Gewerbekunden und Tagespreise. Kontakt und Impressum bestätigen Georg-Richter-Str. 13, 15234 Frankfurt (Oder), 0335 4000099 und Mo–Fr 7–16 Uhr. Der percent-kodierte `mailto`-Wert ergibt `info@rpr-recyclingpark.de`; die sichtbare Anti-Spam-Zeichenfolge ist nicht die Mailadresse. Northdata bestätigt exakten Namen, Anschrift, HRB 21955 Frankfurt/Oder, Unternehmensgegenstand und Eintragung 07.01.2026. Impressumsfehler „HBR“ korrekt erkannt.

**Identitätskette:** `https://www.priebels.de/index.html` lieferte jetzt 404, ebenso HTTP-/Non-www-Varianten. `https://www.priebels.de/` liefert nach Weiterleitung den RPR-Auftritt. Die Websuche findet noch einen indexierten alten Priebels-Text mit Nachfolgerhinweis zum Jahr 2026. Das stützt die Recherche-Spur, reproduziert aber nicht den vollständigen live gelesenen Beleg für Einstellung/Übergang **01.01.2026**. Weiterleitung allein beweist keinen rechtlichen Betriebsübergang.

**Notwendige Korrektur:** Im Überblick/Timeline den Nachfolgerbefund nicht als in diesem Gate aktuell live bestätigte Tatsache ausgeben. Frühere Beobachtung erhalten, mit Abrufgrenze und indexierter Spur ergänzen; exaktes Übergangsdatum erst mit gesichertem Original/Archivbeleg. Neue RPR-Identität ist unabhängig davon belastbar. Nicht als Umbenennung einer alten Zeile behandeln. Lokaler Abgleich findet außer der Neuanlage keinen Priebels-/Georg-Richter-Treffer; Veolia Mittelweg 32 bleibt getrennt.

`dropoff_json`/`pickup_json` leer ist vertretbar: keine konkreten Metall-Selbstanliefer-/Abholbedingungen aus den gelesenen Seiten ableiten; Google-Rezensionen nicht als Beweis verwenden. „Zertifizierter Entsorgungsfachbetrieb“ steht auf der Seite, aber ohne hier geprüftes Zertifikat nicht als neues Zertifizierungsfeld übernehmen.

Quellen:
- https://rpr-recyclingpark.de/
- https://rpr-recyclingpark.de/kontakt/
- https://rpr-recyclingpark.de/impressum/
- https://www.northdata.de/RPR%20GmbH%20Recycling-Park-Rohstoffhandel,%20Frankfurt%20(Oder)/Amtsgericht%20Frankfurt%2FOder%20HRB%2021955
- https://www.priebels.de/ (Weiterleitung), https://www.priebels.de/index.html (404; Suchindex noch mit Vorgängertext)

## 2. Hermann Korn Hirschaid

Datei: `dossiers/by/by-hirschaid-hermann-korn.md`

**Bestätigt:** Impressum Hermann Korn e.K., HRA 8076 AG Bamberg, Jägerstraße 2; Kontakt exakt gleiche Anschrift, Telefon und die übernommenen saisonalen Öffnungszeiten. Metallseite ausdrücklich Ankauf, Privat-/Gewerbekunden und Abholung großer Mengen per Kranwagen/Container. Startseite fordert zur Anlieferung von Abfällen/Wertstoffen am Recyclinghof auf.

**Wichtige Befunde:**
1. Kontaktseite enthält nicht nur `info(at)h-korn.de`, sondern zwei normale Links `mailto:info@h-korn.de?subject=...`. Die Timelinebegründung „obfuskiert und daher nicht übernommen“ ist sachlich unvollständig. Obfuskation wäre ohnehin kein grundsätzliches Belegverbot nach README. Leeres Mailfeld ist zulässig, aber die Begründung muss berichtigt werden.
2. `handelsregister.live` zeigt einen chronologischen Auszug mit **Abruf 25.04.2024**. Die Kennzeichnung „aktuell“ ist nicht mit einem frisch amtlich abgerufenen Registerstand vom 09.10.2026 gleichzusetzen. Inhalt bestätigt Firma/Anschrift und Ausgliederung des Teilbetriebs Organikentsorgung 2018 an SEK; kein Beleg für vollständige Übernahme des Metallbetriebs.
3. Betreiber-Zertifikat direkt geladen: ausgestellt 09.12.2022, gültig **bis 29.02.2024**, nächstes Audit spätestens 31.08.2023. Standort und damaliger Metallhandel bestätigt; **keine aktuelle Zertifizierung**. Die undatierte Website plus altes Register-PDF erfüllen das Aktualitätskriterium der Owner-Ausnahme nicht nachweislich.

**Notwendige Korrektur / Gate:** Registerdatum und Zertifikatsablauf in Timeline ergänzen. Identität/Anschrift dürfen bleiben; `website_status: aktiv` ist belegt. Für aktuelle Kontakt-/Öffnungs-/Annahme-/Abholbedingungen und `status: aktiv` aktuellen qualifizierenden Zweitbeleg oder nachweisbar aktuelle Betreiberquelle beschaffen. Bis dahin konservativ `status: pruefung`; nicht vollständig belegte Betriebsbedingungen und Telefon in Timeline belassen statt als freigegebene neue Skalare ausgeben. `email: info@h-korn.de` erst nach geklärter Ausnahme/Zweitbeleg füllen, nicht nur wegen des gefundenen Links.

**Namensvetter:** Kauder AV, Jägerstraße 8, ist anderer Standort/Betreiber. Zusätzlich bestehenden Korn-Eintrag Burgpreppach (Korn GmbH, HRB 3319, Fitzendorfer Str. 14) ausdrücklich abgrenzen: gleicher Name/Branche, aber andere Firma, Nummer und Ort. Keine Fusion/Nachfolge belegt.

Quellen:
- https://www.h-korn.de/schrott_metalle.html
- https://www.h-korn.de/startseite.html
- https://www.h-korn.de/Kontakt.html
- https://www.h-korn.de/impressum.html
- https://handelsregister.live/hra/bayern/ag-bamberg/8076/hermann-korn-e-k
- https://www.h-korn.de/zertifikate.html
- https://www.h-korn.de/zertifikate_files/HZ-01%20400%201302749-2022.pdf (Deckblatt und Standortanlage gelesen)

## 3. ASM Dorsten

Datei: `dossiers/nw/nw-dorsten-asm-recycling.md`

**Bestätigt als Betreiberwerbung:** Root/Kontakt nennen ASM Recycling, Schrott-/Metallannahme und Ankauf, Adresse An der Wienbecke 30, Mo–Sa 8–18. Sichtbare Rufnummer 015771546610; `tel`-Links stattdessen +49 2362 7876971. Waeil El-Lahib steht als Warenannahme/Verkauf, **nicht** als rechtlich verantwortlicher Betreiber. Impressum weiterhin 404.

**Nicht abschließend reproduziert:** Facebook lieferte keinen auswertbaren Inhalt; Instagram nur Login-/Bildmaterial. Die vorhandene Timeline behauptet übereinstimmende standortbezogene Betreiberprofile; dieser zweite Publikationsbeleg ist in diesem Gate nicht erneut lesbar. Deshalb Adresse und `dropoff_json` nicht bedingungslos freigegeben. README lässt Betreiber-Social als Quelle zu, aber Profilinhalt, Standort und tatsächliche Betreiberkontrolle müssen sichtbar belegt sein.

**Neue Identitätsspur, kein Fill:** Suchindex von Creditreform-Nachbarprofilen nennt „Aeemad Hussein El-Lahib, ASM Recycling“, Dorsten. Ein Verzeichnis nennt dazu Wenger Höfe 13; anderer Creditreform-Treffer nennt Aeemad Hussein El-Lahib/Legdener Metallrecycling in Legden. Das ist ein Anlass zum zweiseitigen Betreiber-/Standortcheck, keine Erlaubnis, Waeil durch Aeemad zu ersetzen oder Dorsten/Legden zu mergen. Direkten Wirtschafts-/Gewerbebeleg anfordern; keine Register-ID aus Nachbar-Snippets konstruieren.

**Notwendige Integration:** `status: pruefung` und leere phone/email/opening_hours/pickup beibehalten. Vor Übernahme von street/postcode/dropoff lesbaren zweiten qualifizierenden Standortbeleg sichern; andernfalls diese Angaben nur als Betreiber-Einzelbeleg in Timeline und Frontmatter leer. Die belegte Website-Erreichbarkeit darf `aktiv` bleiben. Keine Verbindung zur Dortmunder A&R-Firma allein über El-Lahib ableiten.

Quellen:
- https://asm-recycling.de/
- https://asm-recycling.de/kontakt
- https://www.asm-recycling.de/impressum (404)
- https://www.facebook.com/ASMRecyclingDorsten/ (nicht auswertbar)
- https://www.instagram.com/asm_recycling/ (kein lesbarer Standortbeleg)
- https://firmeneintrag.creditreform.de/46284/4050153487/HUMBERT_BAUSTOFF_RECYCLING_GMBH (nur indexierte Nachbarspur)
- https://firmeneintrag.creditreform.de/48739/4250628587/AEEMAD_HUSSEIN_EL_LAHIB_LEGDENER_METALLRECYCLING (Suchindex, andere Ortsangabe)

## 4. Euromet Structured Düsseldorf

Datei: `dossiers/nw/nw-dusseldorf-euromet-structured-rec.md`

**Bestätigt:** Impressum HRB 107685 Düsseldorf, Euromet Structured Rec. GmbH, Dreischeibenhaus 40211, Sascha Nimphius. Northdata kongruent, Unternehmensgegenstand An-/Verkauf, Im-/Export und Recycling von Altmetallen. Registerindex online-handelsregister führt Firma ausdrücklich aktiv. Root/Ankaufseite bestätigen NE-Metallhandel und Materialsorten. Geschäftssitz nicht als frei zugänglichen Schrottplatz ausgeben.

**Abgrenzung korrekt:** Mobilmarken-Impressum nennt zwar Structured, aber HRB 17773 Gelsenkirchen und andere USt-ID. Northdata ordnet diese Nummer der separaten **Euromet Rec. GmbH** zu, heute HRB 287610 B Charlottenburg, Schönhauser Allee 163 Berlin; Sitzverlegung mit Publikationen 22.05./08.06.2026. Kein Beweis für Rechtsidentität mit HRB 107685. Gemeinsamer Geschäftsführer/Markenlink ist kein Verschmelzungsbeleg. Mobile Verbraucherbedingungen und Kontaktdaten zu Recht nicht übernommen; keine neue Dortmunder Betriebsstätte ohne Anschrift.

**Integration:** Aktuelle sparsame Frontmatter freigeben. Leere Kontaktdaten sind kein Fehler. Optional nach belegter Owner-Ausnahme ergänzen: eigenes Impressum nennt **+49 (0) 211 88 25 0294**, **admin@euromet-structured.de**, Root zusätzlich 0800 46 85 790. Diese sind ausdrücklich von +49 178 2495588 / info@buntmetalle-ankauf-mobil.de zu unterscheiden. Keine Mail-/Telefonpflicht allein wegen verfügbarer Einzelquelle. `dropoff`/`pickup` weiter leer.

Quellen:
- https://euromet-structured.de/
- https://euromet-structured.de/impressum-von-euromet-structured-rec-gmbh/
- https://euromet-structured.de/ankauf-von-ne-metallen-aller-art/
- https://www.northdata.de/Euromet%20Structured%20Rec%C2%B7%20GmbH,%20D%C3%BCsseldorf/HRB%20107685
- https://www.online-handelsregister.de/handelsregisterauszug/nw/Duesseldorf/HRB/107685/Euromet-Structured-Rec-GmbH
- https://buntmetalle-ankauf-mobil.de/impressum/
- https://www.northdata.de/Euromet%20Rec%C2%B7%20GmbH,%20Berlin/Amtsgericht%20Charlottenburg%20(Berlin)%20HRB%20287610%20B

## 5. Pietsch Kleve

Datei: `dossiers/nw/nw-kleve-pietsch-rohstoffe.md`

**Freigabe:** Anschrift, Telefon, Mail, Öffnungszeiten, bezahlter Ankauf sortierter Metalle und Anlieferung sind exakt auf Betreiberseiten vorhanden. Northdata bestätigt HRB 1861, Anschrift und Metall-/Wertstoffhandel; Veröffentlichung des Jahresabschlusses am 27.01.2026. Schreibweise mit Bindestrich im Register ist Alias, nicht zweite Gesellschaft.

**Owner-Ausnahme jetzt konkret belegbar:** Zertifikat 10718 / ZZEE001005080006 der bvse-Entsorgergemeinschaft, geprüft 09.12.2025, ausgestellt 26.02.2026, gültig **bis 30.06.2027**. Nennt Pietsch - Rohstoffe GmbH, HRB 1861 Kleve, Lise-Meitner-Str. 24–30; Anlage 2 Betriebsstätte Kleve, Anlage 1 Abfalllogistik mit eigenem Fuhrpark. Damit aktuelle Inhalte, Registerkongruenz und standortbezogener Betrieb belegt, unabhängig vom hier blockierten Creditreform-Abruf.

**Integrationskorrektur:** Diese Quelle/Datumsangaben in Timeline ergänzen und Owner-Ausnahme ausdrücklich begründen. Vorhandene Skalare können bleiben. `pickup_json` leer ist vertretbar: Fuhrpark/Zertifizierung allein keine Ankauf-Abholbedingungen. Lichtenow getrennt halten; keine Person André Pietsch aus dem Klever Namensbestand ableiten.

Quellen:
- https://pietsch-rohstoffe.de/wertstoffhof/schrott-und-metallhandel/
- https://pietsch-rohstoffe.de/impressum/
- https://www.northdata.de/Pietsch%20-%20Rohstoffe%20GmbH,%20Kleve/HRB%201861
- https://pietsch-rohstoffe.de/wir-ueber-uns/zertifizierung/
- https://pietsch-rohstoffe.de/wp-content/uploads/2026/03/2025_ZZEE001005080006_EFB-Zertifikat_Pietsch_10718.pdf

## 6. Rohstoffhandel Hein Kreuztal

Datei: `dossiers/nw/nw-kreuztal-rohstoffhandel-hein.md`

**Betreiberbeleg:** Dorothe Hein im Impressum, Büro Hauptstrasse 124 Hilchenbach; gesondert Platz/Gewerbe/Annahme Austrasse 28a, 57223 Kreuztal Ferndorf. Zeiten für Büro und Platz sind getrennt; der neue Datensatz vertauscht sie nicht. Betreiber-Leistungsseite belegt Vergütung nach Wiegung und gewerbliche Container/Abholung. Impressum hat leere Registergericht-/Nummernfelder: keine Owner-Ausnahme. Marco Deutscher ist Fahrer, kein Beweis für Identität mit Mario Deutsch.

**Zweitbeleggrenze:** Direkter Creditreform-Abruf 403. Gezielte Suchabfrage der exakten Creditreform-URL liefert Dorothe Hein Rohstoffhandel, Hauptstr. 124 Hilchenbach, +49 2733 6808 und Domain. Dies reproduziert die Telefon-/Domainspur im Index, nicht den gesamten aktuellen Wirtschaftsauskunftsinhalt und nicht den Kreuztaler Platz.

**Integration:** `pruefung`, leere street/postcode/hours/dropoff/pickup sind angemessen. Aktuell vorhandene Telefonnummer ist durch Betreiber plus Creditreform-Index gestützt; für streng direkt gelesenen Doppelbeleg Originalauszug sichern, bis dahin ausdrücklich Indexbeleg kennzeichnen (oder phone leer lassen). Mail bleibt Einzelbeleg. Tätigkeit/Platz in Beschreibung als „laut Betreiber“ kennzeichnen; keine vermeintlich verifizierte tagesaktuelle Ankaufserie behaupten. Kein Merge mit Heinrichs Gelsenkirchen, Glück, Müller oder ungeklärtem Mario-Deutsch-Lead.

Quellen:
- https://www.rohstoffhandel-hein.de/das-unternehmen/
- https://www.rohstoffhandel-hein.de/unsere-leistungen/rohstoffhandel/
- https://www.rohstoffhandel-hein.de/impressum/
- https://firmeneintrag.creditreform.de/57271/4330089071/DOROTHE_HEIN_ROHSTOFFHANDEL (403 direkt; exakter Suchindex mit Telefon/Domain/Büroadresse)

## 7. SK Metallhandel Menden

Datei: `dossiers/nw/nw-menden-sk-metallhandel.md`

**Bestätigt:** Root nennt KG, Standort Fröndenberger Straße 287, Altmetallverkauf an den Betrieb, Telefonnummern, Mail und Zeiten inklusive Pause. Familienbetrieb seit 1987 als eigene Unternehmensdarstellung, nicht rückwirkendes KG-Gründungsdatum. Northdata bestätigt KG/HRA 5675 und Anschrift; jüngste sichtbare Publikation 03.06.2026 (Abschluss 2022). Creditreform-Suchindex bestätigt Festnetz 02373 15802, Domain und Adresse; direkter Abruf 403.

**Gate-Lücke:** Alle geprüften Impressumsvarianten `/kontakt/impressum/`, `/impressum/`, HTTP/HTTPS und Non-www lieferten 404. Startseite nennt keine Registernummer. Damit ist die Ausnahme für Betreiberangaben nicht erfüllt, selbst wenn Firma und Adresse registerkongruent sind. Mobil 0176 55903788, Gmail und genaue Zeiten haben hier keinen zweiten qualifizierenden Beleg.

**Notwendige Korrektur:** Mobilnummer aus `phone` zunächst entfernen; Festnetz nur nach offen dokumentiertem Creditreform-Indexbeleg bzw. gesichertem Original behalten. `email` und `opening_hours` bis zu weiterem zulässigen Beleg leer lassen; Werte in Timeline erhalten. Oder erreichbaren vollständigen Betreiber-/Register-/aktuellen Standortbeleg sichern, dann keine Leerung nötig. Nicht pauschal mit „Northdata + Creditreform“ die nur auf Root vorhandenen Betriebsdetails freigeben. `dropoff`/`pickup` leer bleiben. Aktuelle Ankaufwerbung nicht „Altmetallverkauf“ im Sinn eines Händler-Verkaufstarifs missverstehen.

**Dubletten:** Kein anderer lokaler Name-/287-Adressmatch. Meisterjahn und Krabbenhöft bleiben getrennt; keine Verbindung über bloße Nähe.

Quellen:
- https://www.sk-metallhandel.de/
- https://www.sk-metallhandel.de/kontakt/impressum/ (404)
- https://www.sk-metallhandel.de/impressum/ (404)
- https://www.northdata.de/SK%20Metallhandel%20GmbH%20%26%20Co%C2%B7%20KG,%20Menden/Amtsgericht%20Arnsberg%20HRA%205675
- https://firmeneintrag.creditreform.de/58708/4190092914/SK_METALLHANDEL_GMBH_CO_KG (Index, direkt 403)

## 8. Bierhoff Sundern

Datei: `dossiers/nw/nw-sundern-bierhoff-metallhandel.md`

**Freigabe:** Privatseite nennt bezahlten Ankauf am Standort, Ausweis und genau die übernommenen Zeiten; Gewerbeseite eigene Abholung binnen 48 Stunden, feste Termine nach Absprache, eigene Behälter 0,5–10 m³. Zusätzlich Transporte größerer Container über Spedition; das derzeitige Bedingungsfeld behauptet keine Ausschließlichkeit und ist korrekt. Kontakt/Impressum bestätigen Firma, Anschrift, Festnetz und zentrale Mail.

Northdata bestätigt KG, **HRA 5623**, Anschrift und persönlich haftende Bierhoff Verwaltungs-GmbH. **HRB 6485 gehört nicht als zweite Nummer zur KG**, sondern ist als Komplementärbezug zu behandeln; keine zweite Händlerzeile aus zwei Registernummern ableiten. Aktuelles Datum auf Kontaktseite: Warenannahme **09.10.2026 geschlossen**, kein Dauerstillstand. Zusammen mit standortbezogenen Angaben/Impressum und Registerkongruenz überzeugender Aktualitätsbeleg der Betreiber-Ausnahme. Website `aktiv`, Händler `aktiv`, reguläre Zeiten können bleiben.

**Integrationskorrektur:** Erneut gelesenen Northdata-Link, datierten Aktualitätsbeleg und HRA/Komplementär-Unterscheidung in Timeline aufnehmen. Creditreform-403 nicht als frisch reproduzierten wirtschaftsaktiven Vollbeleg ausgeben. Keine lokale Sundern-/Bierhoff-Dublette gefunden.

Quellen:
- http://www.bierhoff-metall.de/Privat/
- http://www.bierhoff-metall.de/Gewerblich/
- http://www.bierhoff-metall.de/Kontakt/
- http://www.bierhoff-metall.de/Impressum/
- https://www.northdata.de/Bierhoff%20GmbH%20%26%20Co.%20KG,%20Sundern/Amtsgericht%20Arnsberg%20HRA%205623

## 9. Pietsch Lichtenow (Bestand)

Datei: `dossiers/bb/bb-lichtenow-pietsch-andre-schrottannahme.md`

**Freigabe der Änderung:** `aktiv` → `pruefung` beseitigt Widerspruch zur alten Timeline. Keine Schließung behaupten; keine Klever GmbH-Daten zur Lichtenower Person verschieben. Leere Kontakte/Adresse bleiben angemessen.

**Neue Spur:** Die Suche nach Pietsch/Lichtenow findet noch einen indexierten Betreiber-Impressumstext unter `schrottlieb.de/sites/impressum.htm`: André Pietsch, Allround Dienstleistungs Service, 15345 Rüdersdorf OT Lichtenow, 033434/46255, Mobil 0177/6123024. Direkte HTTP-/HTTPS-Abrufe scheiterten mit DNS-Auflösung. Damit nicht nur Verzeichnisnamen vorhanden, sondern eine historische Betreiber-Spur; gleichwohl **kein aktueller** erreichbarer Primärbeleg für Adresse oder Ankauf. Der genaue alte Adresswert Chausseestr. 22A wird durch diesen Indexauszug nicht automatisch bestätigt.

**Korrektur:** Absolutformulierung „keine Betreiberquelle“ auf „keine aktuell erreichbare/verifizierte Betreiberquelle; historische indexierte Betreiber-Spur“ präzisieren, ohne die früheren Negativbefunde umzuschreiben. Archiv-/Gewerbeprüfung erforderlich, wenn später aktiv gesetzt werden soll. Ein leerer Prüfdatensatz ist kein bestätigter aktiver Betrieb und keine freigegebene Phantomidentität.

Quellen:
- https://www.schrottlieb.de/sites/impressum.htm (Suchindex; DNS-Fehler direkt)
- Betreiber-/Registerquellen Pietsch Kleve aus Abschnitt 5 als Abgrenzung, nicht als Lichtenow-Beweis

## 10. Papierfritze Berlin (Bestand)

Datei: `dossiers/be/be-marzahn-hellersdorf-papierfritze-schrott-altpapierankauf.md`

**Neue Beschreibung bestätigt:** Paletten- und Gitterboxen-Seiten belegen Ankauf, Bedingungen und eigene Stückpreise; Seite zeigt jeweils „aktuell/tagaktuell vom 9. Oktober 2026“. Das ist die sichtbare Seitenangabe, nicht unabhängig bestätigtes Änderungsdatum aller Preise. Kein EUR/kg-Metallpreis daraus erzeugen. Beschreibung zutreffend, allerdings sehr eng für den umfassenden Wertstoff-/Metallbetrieb; optional um Metall/Altpapier ergänzen, nicht als reinen Palettenhändler umklassifizieren.

**Identität/aktueller Betrieb:** Impressum Rustech Recycling GmbH, HRB 233965 B, Alt-Biesdorf 12; Northdata RusTech Recycling GmbH mit derselben Nummer/Anschrift und passendem Gegenstand einschließlich Paletten/Kisten/Boxen; Veröffentlichung 16.06.2026. Öffnungszeiten Biesdorf und Telefon stimmen mit aktueller Kontaktseite überein. Registerkongruenz plus datierte Preisangaben und standortbezogene Kontaktseite stützen die Owner-Ausnahme.

**Zwingende Frontmatterkorrektur:** `street: Alt-Biesdorf 12, (+ Filiale Neukölln)` → **`street: Alt-Biesdorf 12`**. Neukölln ist ein getrennter auf der Kontaktseite genannter Standort (Stubenrauchstraße 96/97 bzw. Kanalstraße 5/5a, 12357 Berlin), kein Bestandteil der Biesdorfer Straße. Inhalt in Timeline erhalten, nicht durch einen Sammeladresswert seed-en. Der Zusatz ist nicht „unbelegte Filialexistenz“, sondern eine sachlich unzulässige Zusammenfassung belegter Standorte.

**Echte bestehende Dubletten-/Markenspur:** Schwesterdatei `dossiers/be/be-marzahn-hellersdorf-neukolln-schrottfritze-zweitmarke-papierfritze-gr.md` beschreibt denselben Biesdorfer Standort, Telefonnummer und denselben Rechtsträger. Die live gelesenen beiden Impressen plus Registerabgleich belegen **Zweitmarke desselben Betriebs**, nicht zwei unabhängige Händler. Neue Änderung legt keine weitere Dublette an, aber ownerseitig explizit Querverweis/Klärfall anbringen und bei späterer Dedup-Entscheidung stabile Slugs behalten. Kein eigenmächtiger Merge/Delete im Rahmen dieses Gates. Auch Klettwitzer Straße 1 steht als weitere Filiale auf den Impressumsseiten; nicht mit Biesdorf zusammenziehen.

`email` leer und `dropoff`/`pickup` leer sind konservativ zulässig. Auf Papierfritze keine eigene zentrale Mail fälschlich aus Schrottfritze übertragen, auch wenn der Betreiber derselbe ist; eine spätere Ergänzung muss die Markenquelle offen benennen.

Quellen:
- https://www.papierfritze.de/ankauf-paletten-berlin/
- https://www.papierfritze.de/ankauf-gitterboxen-berlin/
- https://www.papierfritze.de/kontakt/
- https://www.papierfritze.de/impressum/
- https://schrottfritze.de/impressum/
- https://www.northdata.de/Rustech%20Recycling%20GmbH,%20Berlin/Amtsgericht%20Charlottenburg%20(Berlin)%20HRB%20233965%20B

## Konkrete Übergabe an Integration

1. **Nicht ungeprüft alles auf `aktiv` übernehmen.** Korn-Aktualität und SK-Einzelbelegfelder benötigen Nachbelege oder konservative Reduktion; ASM/Hein bleiben Prüfdatensätze.
2. **Papierfritze-Straße bereinigen**, Schwester-/Markenverweis ergänzen; Paletten-/Gitterboxenbeschreibung ist belegt.
3. **Korn-Mailbegründung, Registeralter und Zertifikatsablauf korrigieren**, Burgpreppach-Namensvetter ergänzen. Kein EFB-Status 2026 aus dem PDF von 2022 ableiten.
4. **RPR-Vorgängerquelle mit Abrufgrenze dokumentieren**, exaktes Übergangsdatum nicht aus Weiterleitung oder gekürztem Index erfinden.
5. **Pietsch-Kleve-Zertifikat und Bierhoff-Register-/Aktualitätsbeleg ergänzen**; dadurch sind deren Frontmatter-Fills nachvollziehbar ohne einen angeblich hier gelesenen Creditreform-Vollbeleg.
6. **ASM-Social-/Betreiberzweig und Hein/SK-Creditreform-Spuren sichern**, wenn die betreffenden Felder unverändert integriert werden sollen. Keine aus der Recherche entstandenen Identitätsleads automatisch seed-en.
7. Vor Commit später durch Owner `cargo test -p schrott-mcp-ingestion seed` ausführen. Hier auftragsgemäß keine Builds/Deployments, keine technische Produktionsfreigabe. Alle vorhandenen Dossieränderungen bleiben für die Integration erhalten.
