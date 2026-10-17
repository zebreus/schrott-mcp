# Check-in 17.10.2026, 21:30 — alte Welle 13 B: Korrekturhandoff

**Nachträgliche Owner-Einschränkung 17.10.2026:** Dieser Handoff und seine Nachherhashes dokumentieren den ersten Korrekturschritt, nicht den abschließend integrierbaren Stand. Die [unabhängige Endabnahme](checkin-20261017-2130-wave13-b-owner-final.md) hielt neue Kontakt-/Domainfills und Blums Identitätspaket zurück. Owner hat danach genau diese Felder gezielt auf dem bisherigen Seedstand bewahrt und datierte Endabnahmehinweise ergänzt, ohne frühere Timeline zu löschen. Drei Dossiers und der Wellenbericht haben deshalb neue Hashes; die übrigen 16 Dossiers sind unverändert zum Handoff. Erste Korrektursicherung bleibt erhalten; zusätzliche Sicherung `/root/Documents/wave13b-owner-field-hold-20261017`. Endgültiger Prüf-/Test-/Integrationsstand ausschließlich im [Owner-Integrationsgate](checkin-20261017-2130-wave13-b-owner-integration.md). Die nachfolgenden früheren Freigabesätze sind dadurch eingeschränkt, keine nachträgliche Quellenbestätigung.

## Ergebnis und Umfang

**Redaktioneller Korrekturumfang abgeschlossen:** genau die 19 Dossiers des vollständigen Reviews `checkin-20261017-1700-wave13-b-review.md` und `wave-20261013-b.md` überarbeitet; zusätzlich ausschließlich dieses Handoff neu angelegt. Keine pauschale Quellen-/Seed-/Releasefreigabe. Basis: vollständige Dateien/Timeline, README und `docs/trader-seed.md`, HEAD-Feldvergleich und Ownerdisposition vom 17.10. Alle Edits per patch, kein blindes Zurücksetzen ganzer Dateien.

**Externe Sicherung vor erster Änderung:** `/root/Documents/wave13B-backup-20261017T213118923269`, mit vollständiger relativer Verzeichnisstruktur und `manifest.json` (20 Vorher-SHA-256). Alle Sicherungshashes gegen die Originalkopien nachgeprüft. Vorher bedeutet übernommener Arbeitsstand, nicht HEAD; Nachher bedeutet Stand dieses Korrekturdurchlaufs.

`dossiers/he/he-hanau-bamberger.md` nicht bearbeitet, vorhanden und weiterhin bytegleich HEAD; SHA-256 `6df0f6a940347c6b3a58d7bf7f82974c89cb3b645a1bd40b87834e44930c1956`. Die falsche Fehlmeldung ist im Wellenbericht als datiertes Erratum korrigiert. Keine anderen Dossiers, insbesondere keine alte-C-Arbeitsdateien bearbeitet. Andere parallele Änderungen nicht zurückgesetzt.

## Dispositionen / abgeschlossene Korrekturen

- **Kundrat:** falschen Negativbefund im Überblick und neuer Timeline korrigiert, vorhandenen HEAD-Wert `street: Schreberstr. 38a` bewahrt. NorthData HRB 5518, Schrott-/Metallgegenstand, Veröffentlichung 09.06.2026 widerlegen „kein Registerbeleg“. Keine neue PLZ, Kontakte, Aktivität oder Betriebsstätten-/Servicefreigabe.
- **Schrottprofis:** einziges neues Clear im Umfang: `phone: ''` plus `clear_fields: 'phone'`, ausdrücklich ownerautorisiert. Rückstufung der Nummer zum unbestätigten Lead aufgrund Bügler/Schrottprofis-/156/166-Konflikt, keine behauptete Ungültigkeit oder Schließung. Quelle/Begründung/Keep-Clear-Wirkung in neuer Timeline.
- **Reining:** City normalisiert, Kernadress-Withdrawal ownerfreigegeben als Entzug unqualifizierter Kuratierung, nicht falscher-Ort-Beweis. Gencsoy/seit Mai 2020/Reining-Gründer als konkrete Suchindex-Nachfolgespur mit öffentlich gelesenem Über-uns-JS-Modul dokumentiert; Modul belegt Ansprechpartner/Leistungen, nicht die ganze Kette. Kein Ringstraße-7-/Kontaktfill.
- **Bachmann:** Rechtsträgername/H1 präzisiert, Bestandsadresse/Kerntätigkeit bewahrt. Neue E-Mail/Aktivstatus/Ankauferweiterung der Description gezielt auf HEAD leer/`pruefung`/bisherige Description gehalten. Telefon/Domain verbleiben feldbezogen Creditreform-indexgestützt; erreichbarer Website-Status nicht als heutige Betriebs-/Zertifikatsbestätigung ausgegeben.
- **Collecta:** KG/H1, Wertstoffhandel, Büroanschrift/PLZ freigegeben, keine erfundene GmbH→KG-Kette oder Europe-Vermischung. E-Mail/Status/Description auf HEAD leer/`pruefung`/leer gehalten. Telefon/Domain Creditreform-indexgestützt; Zertifikat nur bis 31.05.2026, keine heutige EFB-Zusage.
- **Blum:** Julias Container/Marco Blum, H1, Identitäts-/Geschäftsadress-/Typkorrektur freigegeben. Telefon/Domain feldbezogen indexgestützt; Mail/Description auf HEAD leer gehalten, `pruefung` bleibt. Creditreform-403 und Social-Suchindexgrenze ausdrücklich offen. Keine Annahmehof-/Metallankaufsfreigabe; Gebühren Betreiberangaben mit Bauschutt-getrennt-Qualifier.
- **Metzger:** AV-Gegenlead Rehgartenstr. 5/02774 918815 neben Raumausstatter Waldweg 7; weder Gleichsetzung noch sichere Personenverschiedenheit. **Cumic:** konkrete Gelbe-Seiten-/Inteka-Deep-Links statt Startseiten, weiter nur Leads. **Donndorf:** neuer Korrekturhinweis: Name steht im Frontmatter; keine neuen Adress-/Kontaktfills gemeint. **Schleicher:** Rohr eigene 036844 30245, alte Portal-Mehrheitsverifikation ausdrücklich überholt; keine neue Feld-/Zertifikatsfreigabe.
- Übrige Ziele: neue datierte Präzisierungen zu Lead-/Alt-Gates, bereits integrierter Arbeit, Freigericht-TSR-Abrufgrenze, Bernhardt Fax-/Telefonkonflikt und Keep/Clear/GEO-Semantik. Keine neuen unbelegten Kontakt-/Servicefills.

## Tatsächliche zusätzliche Recherche dieses Durchlaufs

Nicht sämtliche Quellen des 17-Uhr-Reviews erneut abgerufen; aus dem Review übernommene Befunde sind ausdrücklich so attribuiert.

- Websuche `"Cumic Benjamin" Autoverschrottung Wiesbaden`: konkrete Suchtreffer https://www.gelbeseiten.de/gsbiz/235948f0-16b9-4729-b0f8-528dbb78a688 und https://inteka.de/Firmenverzeichnis/index.cfm?Eintrag=1219877; Portalbelege, keine Primärbestätigung. Letzterer nennt Maillead, bewusst nicht gefüllt.
- Websuche `"Marco Metzger" "Rehgartenstr"`: https://www.11880.com/suche/schrottplatz/eschenburg mit Marco Metzger/AV/Rehgartenstr. 5/02774 918815, nur Lead.
- Direktabruf https://www.autoteile-schleicher.de/ bestätigt getrennte Standorte/Funktionen und Rohr-Nummer; Werbetext kein aktueller Zertifikatsbeleg.
- Erneuter Direktabruf https://firmeneintrag.creditreform.de/35435/6110597532/MARCO_BLUM_ENTSORGUNGSTEAM weiterhin HTTP 403. Gate dadurch nicht geschlossen; keine Vollabruf- oder Mailbestätigung erfunden.

## Kontrollen (keine Tests)

- **19/19 alte vollständige Timeline-Abschnitte bytegleich erhalten:** jeweiliger Vorher-Inhalt ab `## Timeline` ist nach neuen Korrektureinträgen unverändert als Suffix vorhanden, nicht nur einzelne alte Bullets. Neue Korrekturen sind zusätzliche Seed-Notes.
- **20/20 Sicherungshashes verifiziert**, Vorher-/Nachherliste unten. Slug und vorhandene/fehlende lat/lon-Zeilen unverändert.
- Scoped `git diff --check -- <genau diese 20 Dateien>`: Exit 0. Nur Whitespacekontrolle, kein Test-/Build-/Seedbeleg.
- Frontmatter gegen HEAD gelesen: effektiv nur sechs Dossiers mit Stammdatendifferenz (Darmstadt, Schrottprofis, Bachmann, Reining, Blum, Collecta); Kundrat wieder auf HEAD-Kernwert, übrige 13 nur Text. Genau ein `clear_fields`, nur `phone` bei Schrottprofis. Mail/status/description der drei Identitätsfälle entsprechen jeweils gezielt HEAD.
- **Nicht ausgeführt:** Tests, Builds, Seedlauf, Datenbank-/Produktivabfragen oder -mutationen, GEO-Arbeit, Commit, Push, Deployment. Keine Veröffentlichung behauptet.

## Restgates / nächster Owner-Schritt

1. **Quellenfeldgates:** Bachmann/Collecta/Blum Creditreform-Suchindex ist kein vollständig gelesener Auszug; insbesondere Blum/403 und unabhängiger Vollabgleich bleiben offen. Neue positive E-Mail-/Aktivitäts-/Description-Fills bis qualifizierendem Zweit-/Aktualitätsbeleg aufgeschoben. Keine vollständige README-Betreiber-Ausnahme beansprucht. Telefon/Domain sind belassene, ausdrücklich begrenzte indexgestützte Teilkorrekturen, keine Universalfreigabe.
2. **Ketten/Standorte:** Kundrat zweite Betreiber-/Gewerbequelle und heutige Betriebsstätte; Reining unabhängiger Übernahme-/Gewerbenachweis. Keine Ringstraße, kein pauschaler Umzug oder Annahmehof. Schleicher Rechtsträger und Verhältnis An der B 62 4/Hämbach/Rohr weiter offen. Übrige Portal-Leads unverifiziert.
3. **Altfills/Clears:** Richter-Einzelunternehmer-Zweitquellen-Gate für ältere Kontakt-/Servicestruktur bleibt offen; HEAD-Felder nicht verändert und nicht rückwirkend legitimiert. Stein/Schindling keine neuen Clears autorisiert: phone leer bedeutet Keep, nicht garantierter DB-Rückzug. Auch leere Servicefelder können gespeichertes JSON behalten; keine pauschale Löschliste.
4. **GEO:** leere Kernadressen ersetzen gespeicherte Adresse beim geänderten Seed, aber ohne geliefertes Koordinatenpaar bleibt gespeichertes GEO erhalten. Seed geocodiert nicht. Historische gegenteilige Timeline-Aussagen sind durch neue Qualifier überholt, nicht gelöscht. Keine GEO-Freigabe.
5. Erst nach separater Ownerbewertung scoped Seed-/Diffprüfung und spätere Veröffentlichungs-/Produktivverifikation. Diese redaktionelle Fertigstellung schließt bekannte Beweislücken nicht durch Formulierung und ersetzt keine Test-/Releasefreigabe.

## Exakte Dateiliste und SHA-256

| Datei | Vorher (extern gesichert) | Nachher |
|---|---|---|
| `dossiers/he/he-darmstadt-nord-64293-schrott-aller-art.md` | `98d43995cf3560f201ae1fc11619f6658e6d24431433113f39745ac4faa8220e` | `0360a9501dad8243cc709e09117b4a2d493bcd1fe6ee1f924fe518ab8d622a98` |
| `dossiers/he/he-eschenburg-35713-metzger-marco.md` | `6ed15ddfb81a48544e26026f00b8ed89edd225a8a7be8c03af83dad8e0173366` | `175c0ca7499eec6370491a3c087485dd1edb2f42eec95d5e240c0bd02cdb3768` |
| `dossiers/he/he-frankfurt-65933-schindling-stoffstrom-vertriebs.md` | `7c6b1e3bf054bc157af72c4905a7daaf3a6415c7f04bd1dd3dafd3f0b0dc4a33` | `63ca48b8dd2f896d5e380fcb0d60117f5ced87952add8f072a29004c00aaffb8` |
| `dossiers/he/he-frankfurt-eschersheim-die-schrottprofis-abholung-von-schrott-a.md` | `e81445359b4dcbee8aaaf51bd4620b3506dc8fc42455df48cab56ed4db230410` | `3c53819360523cbcda181d2e511d1b2969cf7e032d37c896b94ef13dd25290c5` |
| `dossiers/he/he-freigericht-schrott-metallhandel-freigericht.md` | `f06f96f67d5c0dc86a97f2ec425aa8513730736ca50750105664cf60727f0067` | `f4943c3837e0b7a4ca13f4c5e179ed3e1a232d459e6d165119c7c40c58db7226` |
| `dossiers/he/he-hanau-63452-bernhardt-thomas.md` | `0c4d3fe20d806cc571babbc15ea35f2e7e6af3dcf87ac1e77eac92b1748dd669` | `8de62c4fb73b37c862f2dea63abc7f7ed9075023d72925e1de4af287403761e2` |
| `dossiers/he/he-hasselroth-63594-jk-recycling.md` | `25dbf826fb45859dc0a794799b369b9e41af2b06669a16495df026e854e10be7` | `aa6de8457aea3466c77447703ac062c46b47cc9930f2939a0dc7637c1911df99` |
| `dossiers/he/he-kassel-bettenhausen-bachmann-matthias-autoverwertung.md` | `e7194eee0fb4905eabd2c287f880f44ae918bc5bf89d047a84f72bfbd2ffd1e9` | `6e283cc163a255a9f206d4e16ba0bd5acbeee840bec019041b986aca880273b1` |
| `dossiers/he/he-marburg-klein-gotthart-schrotthdl.md` | `c5a44a5a0c72afee54372742e81dc48de7ab9cd5c257388aa2ee30012f022394` | `c0ba2ed5647133e727654c1e8a96cb3aed2c0ef66ca5a2ea2540c7ea8514825a` |
| `dossiers/he/he-offenbach-am-main-kundrat-schrott-containerdienst.md` | `54e09e074ad7e63b71e72ff55603a11573c44c30493db5c24bfb4109bbe5e73b` | `3d54a73e828bfa60220c51168a81ebc84ca7c391342b1af01b043d06f4db7e33` |
| `dossiers/he/he-pfungstadt-64319-stein-alexander-sebastian.md` | `948e0cea0508b23d8029fe596ab3ca9c87d3c40b58833ae566a436bfb3e3dabd` | `81c56248fedb14812782edecbc27e07f4c46236159484a92a35e181c6b5b80c6` |
| `dossiers/he/he-schaafheim-64850-he-reining-klaus-containerdienst.md` | `ba2dfe6ab3e081b6ac471a0e1f56a7777e355baafe2f35ebcd226bbbb4ad878c` | `585ebfe4f53679406be1fac19c3270ab3a8ede269ff2308cf4d2657a5125c5f2` |
| `dossiers/he/he-tourgebiet-kassel-schrotthandel-richter-mobil-fur-kassel-s.md` | `a4a2ba74087350a6380fbe4fe5be2a8a5a34467f1293a32ad366968eaf51aff9` | `751d645dda1d15a1ab12b5c9b829fde9149f9e514bdc56efd52f75874cc6c7b9` |
| `dossiers/he/he-weimar-lahn-j-haas-schrott-metallhandel.md` | `490037ab7b5240f2f830493c288a72b71321a8c7f524ca3fd46e1b8d7f6f1232` | `899add04217215c9bf94926edf7f7de01ab0d64f9c702ce8811d99de5e370455` |
| `dossiers/he/he-wettenberg-krofdorf-blum-marco-schrottentsorgung.md` | `730f31e9b639c9dfad007b74f995faea13d3cda1c292f6a4c4e0677264341993` | `b6967e1e2e7f848beaaf3d4eecb39dd2a0e8693ba0669042383852b65c85b9fc` |
| `dossiers/he/he-wiesbaden-65183-collecta-rohstoff.md` | `2ed36d5c491bede0723dc9356302d96c03ea53becf4548152fb245a688616c6f` | `abb04607f5a9d97fb960f5a8051f3524fbbf46028475385014f774513896ff99` |
| `dossiers/he/he-wiesbaden-biebrich-cumic-benjamin-autoverschrottung.md` | `5d1fdc183d0a41246387f8ecd6d85454bcf4c5cbd5f1490681d248363f933d28` | `fc33c931dbf26ca54ff6119274fe3755fbb9072b9f276421fa0768ef822ec298` |
| `dossiers/th/th-stadtroda-donndorf-herbert-schrott-und-metallhande.md` | `44d9eeb21f314aefa824c48b977ea3fdec8ab0852860208b156b9f82c2d8facc` | `e7cc56c56f8d745f105d45fec16a7eb3da9185bf28bb78326922df023e5328e0` |
| `dossiers/th/th-tiefenort-b-bad-salzungen-autoverwertung-schleicher.md` | `25e15ffa5e79db2840bf86fe5939b28e207f48756a6fcb277d17e5f87a22d15c` | `bc4341cb73ef4b41bd163f1597cc144eea1f66e9d2fc277213c78c438aa96e94` |
| `research/wave-20261013-b.md` | `0769f8a6c4bf650349bb4eca57f827e4a89290d28a55e133e72cc7de28360628` | `386f50d279684873d4959f8471859d2893e020db480e73e47e2850b5746ab8dc` |

Dieses Handoff ist die einzige zusätzliche neue Datei: `research/checkin-20261017-2130-wave13-b-corrections.md`, vorher nicht vorhanden. Kein selbstreferenzieller Inhaltshash; die Tabelle erfasst exakt die 20 vor Änderung gesicherten Arbeitsdateien.
