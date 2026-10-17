# Unabhängige Endabnahme — alte Welle 13 B, 17.10.2026

## Entscheidung

**Teilfreigabe, keine Freigabe des unveränderten Gesamtpakets.** Redaktionelle Gegenbefunde, Darmstadt-/Reining-Withdrawal und der ausdrücklich autorisierte Schrottprofis-Telefon-Clear sind nachvollziehbar. Die belassenen neuen Telefon-/Domainfills bei Bachmann, Collecta und Blum schließen das Quellenfeldgate nicht hinreichend; Blums Identitäts-/Typ-/Adressübertragung bleibt zusätzlich gehalten. Eine frühere im Handoff berichtete Owner-Teilfreigabe ist kein Ersatz für den README-Belegstandard. Diese Endabnahme ändert keine Dossiers und implementiert keine Rückstellungen.

Prüfbasis: aktueller `HEAD` **`5ed20b3988f4b945cd39171f0abac7ca1aee859c`**, nicht bloß die ältere Reviewbasis `4b5a0f3b`. Alle 19 tatsächlichen HEAD-Diffs und alle 19 aktuellen Dateien einschließlich vollständiger Timeline gelesen; Wellenbericht vollständig und dessen gesamter Sicherung→Arbeitsstand-Diff gelesen. `research/wave-20261013-b.md` ist **ungetrackt/nicht in HEAD**: sein leerer normaler Git-Diff ist kein Gleichheitsbeleg. Er ist als komplette neue Datei zu integrieren. Die 19 Links der Berichtstabelle treffen exakt die 19 Sicherungsdossiers; Bamberger ist ausdrücklich kein zwanzigstes Änderungsziel.

README §§ Feedback triage/Betreiber-Ketten, `docs/trader-seed.md` und die Enum-/Anwendungslogik in `crates/ingestion/src/seed_traders.rs:144–239,310–439` geprüft. Externe Quellenbefunde sind hier **aus dem 17-Uhr-Review und Korrekturhandoff übernommen**, nicht erneut selbst vollständig abgerufen. Insbesondere kein Creditreform-Vollabruf, keine Umgehung eines 403 und keine neue unabhängige Webverifikation behauptet.

## Feldentscheidungen: genau sechs Frontmatterfälle

| Dossier | Frei | Gehalten / genaue Grenze |
|---|---|---|
| Darmstadt / Schrott Aller Art | `street: ''`, `postcode: ''`; Textkorrekturen | Entzug unqualifizierter Kuratierung, **kein Beweis einer falschen Mainzer Str. 83**, keiner Schließung. `status: pruefung` halten; keine konkurrierende Portaladresse füllen. |
| Frankfurt / Schrottprofis | `phone: ''` **und ausschließlich** `clear_fields: 'phone'` | Explizite Autorisierung durch diesen Auftrag; 01633318898 bleibt historischer Identitätskonflikt Bügler/156 versus Schrottprofis/166, nicht nachgewiesen ungültig. Keine anderen Clears/Servicefills. |
| Reining / Schaafheim | `city: Schaafheim`, `street: ''`, `postcode: ''`; korrigierte Nachfolgespur als Lead | Withdrawal der unqualifizierten alten Kuratierung, **kein Falschortbeweis**. „fehlerhaft“ betrifft insbesondere das Seedformat `Schaafheim , Karlstr. 1`, nicht einen Beweis gegen Karlstr. 1/64850. Kein Ringstraße-7-/Kontaktfill, keine bewiesene Rechtsnachfolge. Suchindex nennt Gencsoy/seit Mai 2020/Reining; gelesenes öffentliches JS-Modul bestätigt nur Ansprechpartner/Leistungen. |
| Bachmann / Kassel | `name: Matthias Bachmann Autoverwertung GmbH & Co. KG`, passende H1. Bestehende Adresse/Typ/Kerndescription unverändert halten | **Neue `phone: '0561 523934'`, `website: https://www.bachmann-ks.de/`, daran gebundenes `website_status: aktiv` zur Integration gehalten.** Telefon/Domain haben Betreiberbeleg plus nur berichteten Creditreform-Suchindex, keinen abgeschlossenen unabhängigen Feldabgleich. `email: ''`, `status: pruefung` und bisherige Description sind tatsächlich HEAD-gleich und bleiben so. Kein aktuelles Demontagezertifikat aus dem bis 24.12.2024 gültigen PDF ableiten. |
| Collecta / Wiesbaden | `name: COLLECTA Rohstoff GmbH & Co. KG`, H1, `trader_type: wertstoffhaendler`, `city: Wiesbaden`, `street: Senefelderstraße 3A`, `postcode: '65205'` als Büro-/Geschäftssitz | Betreiber/NorthData/historisches bvse-Zertifikat tragen Identität/Adresse, keine behauptete GmbH→KG-Umwandlung und kein Europe-Merge. **Neue `phone: '06122 5337-0'`, `website: https://www.collecta-rohstoff.de/`, `website_status: aktiv` gehalten:** unabhängiger kontaktfeldbezogener Abgleich nur Suchindex. `email: ''`, `status: pruefung`, `description: ''` HEAD-gleich halten. Wertstoffhandel ist grobe Tätigkeit, keine öffentliche Annahme-/Abholberechtigung. Zertifikat endete 31.05.2026. |
| Blum / Wettenberg | Vorsichtige Betreiber-/Gebühren-/Identitätsspur als Text, nicht als endgültig freigegebener Match | **Neue `name: Julias Container (Marco Blum)`, `trader_type: containerdienst`, `city: Wettenberg`, `street: Höhenstraße 27a`, `postcode: '35435'`, `phone: '0641 49815208'`, `website: https://www.julias-container.de/`, `website_status: aktiv` als zusammenhängendes neues Identitäts-/Standortpaket gehalten.** Person/Altadresse passen stark, aber Legacy-Lead ist keine zweite Evidenzquelle; Creditreform bleibt Suchindex/403, Social-Datierung kein gelesener unabhängiger Identitäts-/Adressbeleg. H1 nicht losgelöst als endgültige Identitätsfreigabe behandeln. `email: ''`, `status: pruefung`, `description: ''` sind HEAD-gleich zu halten. Keine Metallankaufs-/Annahmehoffreigabe. |

**Konkreter Blocker:** Für die genannten Kontaktfills fehlt ein hinreichend nachvollziehbarer unabhängiger feldbezogener Zweitbeleg; für Blum zusätzlich ein abgeschlossener unabhängiger Identitäts-/Geschäftsadressabgleich. Creditreform ist eine grundsätzlich zulässige Quellenklasse, der hier dokumentierte Suchindex ist eine relevante zweite Spur, aber kein nachgeprüfter vollständiger Datensatz mit nachvollziehbarer Aktualität/Zuordnung. Ein 403 beweist weder Richtigkeit noch Unrichtigkeit. Mehrere Betreiber-Unterseiten sind nur eine Quelle. Die vollständige Betreiber-Ausnahme wird in den Unterlagen ausdrücklich nicht beansprucht (Bachmann Aktualitätsgate; Collecta keine dokumentierte vollständige Ausnahmeprüfung; Blum insbesondere kein Registerbeleg). Benötigt wird ein lesbarer qualifizierender Zweitnachweis je gehaltenem Feld bzw. bei Blum eine Gewerbe-/Creditreform-Identitätsbestätigung; alternativ für die dafür geeigneten Firmen die **vollständig** dokumentierte README-Betreiber-Ausnahme. Nur sprachlich als „indexgestützt“ markierte positive Frontmatterwerte bleiben beim Seed positive Fills/Overwrites.

`website_status: aktiv` bedeutet Erreichbarkeit der Website, nicht `status: aktiv` des Händlers. Die Erreichbarkeit ist berichtet; die gehaltene Freigabe betrifft die endgültige Zuordnung der neuen Domain zum Seed unter dem Quellenstandard, nicht einen behaupteten technischen Ausfall.

## Die übrigen 13 Dossiers: Text frei, keine Stammdatenfreigabe

Frontmatter jeweils bytegleich HEAD, einschließlich **Kundrat `street: Schreberstr. 38a`**. Textfreigabe bestätigt keine alten Kontakt-/Servicefelder rückwirkend.

| Dossier | Freigabeumfang / verbleibende Grenze |
|---|---|
| Metzger Marco | AV-Gegenlead Rehgartenstr. 5/02774 918815 neben Raumausstatter Waldweg 7; weder Gleichsetzung noch sichere Personenverschiedenheit. |
| Schindling | 59/83 und Frankfurt-(Oder)-Fehler als Portalspuren; LinkedIn nur historische Personenspur. Alt-Telefon ohne Clear nicht als wirksam zurückgenommen ausgeben. |
| Freigericht | AC und TSR getrennte fremde Betreiber; AC-Gebühren keine Zielpreise. TSR am 17.10. nicht erneut lesbar bestätigt. |
| Bernhardt | 14998 Fax-/Telefonkonflikt und 12312 ungefüllt; GUTH Castrop-Rauxel nicht transplantieren. |
| JK-Recycling | Kasch/GoLocal nur Lead; weder Aglasterhausen/Darmstadt/SKC noch US-Namensvetter zuordnen. |
| Klein | Betreiberangaben zu Sorten/Tageskursverzicht/Abholung auf Anfrage; bestehendes Einzelbeleg-/Owner-Gate offen. Keine kostenlose Abholung oder Hofannahme. |
| Kundrat | Registergegenbefund HRB 5518/Schrott-Metallgegenstand/Publikation 09.06.2026 korrigiert den falschen Negativbefund. HEAD-Straße bewahren; keine neue PLZ-/Kontakt-/Betriebsstättenfreigabe. |
| Stein | Alte Bereinigung 07.10., keine neue Wellen-Feldleistung; kein zusätzlicher Telefon-Clear autorisiert. |
| Richter | Nur neue qualifizierende Notiz. Ältere positive HEAD-Kontakt-/Service-/Aktivitätsfills bleiben mit offenem Zwei-Quellen-Altgate bestehen, werden hier nicht legitimiert. Kostenlose Schrottabholung ist kein kostenloser Containerdienst; Mengen Wirtschaftlichkeitsorientierung, keine Mindestmengen. |
| Haas | Steingraben/Nummern nur Portale; keine Übernahme des Cylex-Punkts. |
| Cumic | Konkrete Gelbe-Seiten-ID `235948f0-16b9-4729-b0f8-528dbb78a688` und Inteka `Eintrag=1219877` vorhanden und als Suchtreffer/Portal-Leads ausgewiesen. Keine Mailfüllung; Benjamin/Ikram/Kfz-Kooperative nicht gleichsetzen. |
| Donndorf | Neue Korrektur stellt klar: `name` steht unverändert im Frontmatter; gemeint sind keine neuen Adress-/Kontaktfills. Portalspur kein Aktivitäts-/Annahmebeleg. |
| Schleicher | Rohr eigene 036844 30245 gegenüber Hämbach 03695 609811; alte Portal-Mehrheitsverifikation ausdrücklich überholt. Bestehende HEAD-Adresse/Telefon bleiben ungelöster Altbestand, keine neue Verifikation/Zertifikatsfreigabe. |

## Querschnitt / Bericht

- **Enums:** Alle 19 `trader_type`, `state`, `status` und leeren/gesetzten `website_status` statisch gegen die tatsächlichen erlaubten Werte gelesen/abgeglichen; kein ungültiges Enum. Keine Test-/Compilerfreigabe daraus ableiten.
- **Emdash:** Kein Frontmatterfeld enthält `—` als alleinigen Leerwert. Gedankenstriche in Prosa/Provenienz sind Text, kein Löschsignal. H1/Frontmatter der drei Identitätsfälle sind redaktionell konsistent; bei Blum bleibt die sachliche Zuordnung trotzdem gehalten.
- **Overwrite:** Nichtleere neue Werte überschreiben vorhandene Anreicherung. Straße/PLZ leer ersetzen beim geänderten Seed gespeicherte Kernadressen; leere Mail/Description/Services normalerweise **Keep**, keine garantierte DB-Leerung. Nur Schrottprofis hat `clear_fields`, exakt `phone`, ohne Wertkonflikt. Keine Alt-Clears bei Stein/Schindling oder pauschale Servicelöschliste freigegeben.
- **Timeline:** Für **19/19** ist der komplette vor Korrektur gesicherte Inhalt ab `## Timeline` bytegleich als Suffix erhalten; neuer Korrekturblock steht davor. HEAD-Timeline-Zeilen zusätzlich im aktuellen Inhalt erhalten. Alte widerlegte Aussagen werden nicht still gelöscht; aktuelle Qualifier zu Kundrat/Reining/Donndorf/Schleicher und Seedsemantik stehen davor. Timeline wird Seed-`notes`, Textänderungen sind deshalb nicht automatisch DB-neutral.
- **Links:** Konkrete Cumic-Deep-Links nachgetragen; alte Startseiten werden ausdrücklich nicht als Einzelbelege gezählt. Bamberger-Cylex-ID-Verwechslung im Bericht als Erratum korrigiert. Das ist Nachvollziehbarkeitsprüfung, keine frische Verfügbarkeits-/Inhaltsprüfung jedes externen Links.
- **GEO:** Slug und vorhandene/fehlende `lat`-/`lon`-Zeilen aller 19 gegenüber HEAD unverändert. Kein GEO-Auftrag ausgeführt. Ohne geliefertes Paar bleibt gespeichertes GEO trotz Adresswithdrawal erhalten; Seed geocodiert nicht. Kein Hofpunkt und keine GEO-Freigabe.
- **Bericht:** Bamberger-Erratum, Preisarten-/MwSt.-/Vorauszahlungsabgrenzung, Bauschutt-getrennt-Qualifier, Ketten-/Abrufgrenzen und Keep/Clear-Erklärung redaktionell frei. Bamberger vorhanden, getrackt, bytegleich HEAD; nicht bearbeiten/ersetzen. AC bleibt fremder Betreiber; Gebühren sind keine Zielankaufspreise. **Bericht nicht unverändert als abschließende Quellenfreigabe integrieren:** seine Blum-Freigabesätze sowie belassenen Telefon-/Domain-Teilfreigaben sind durch die strengere feldbezogene Disposition dieser Endabnahme eingeschränkt. Ein späterer Integrationsstand muss diese Einschränkungen auch in Bericht/Dossier-Notes konsistent darstellen oder die Belege nachliefern; hier nicht geändert.

## Sicherung und überprüfte Hashes

Externe Sicherung: `/root/Documents/wave13B-backup-20261017T213118923269`, vollständige relative Pfade. **20/20** Sicherungsdateien gegen ihre Manifest-SHA-256 nachgerechnet; alle stimmen. Alle **20/20** aktuellen Hashes entsprechen der Nachher-Spalte des Korrekturhandoffs. Die Sicherung belegt den übernommenen Vorher-Arbeitsstand, nicht HEAD und nicht Produktionsdaten. Ihr im Handoff behaupteter Zeitpunkt vor erster Änderung wird hier nicht aus Hashgleichheit als unabhängig bewiesener Uhrzeitnachweis ausgegeben.

- Manifest: `1081ef759347a5b13ea409b98a91c5871bf916d9062ed8c4d62079967d3c0f9f`.
- Review 17:00: `9f5aac4550e4f0f587c5dac528c617130fa6a2ade9b9af9d2b690fa3b1f23abd`.
- Korrekturhandoff 21:30: `e30f329dbc3c9fe3ed29cb352218c16d735b66bdbbfd3c0c2f0f981d79a9c605`.
- Wellenbericht vorher: `0769f8a6c4bf650349bb4eca57f827e4a89290d28a55e133e72cc7de28360628`.
- Bamberger unverändert: `6df0f6a940347c6b3a58d7bf7f82974c89cb3b645a1bd40b87834e44930c1956`.

Die folgende Liste identifiziert **geprüfte Arbeitsstände, nicht pauschal freigegebene Gesamtdateien**. Vollständige Vorherhashes stehen zusätzlich im verifizierten Manifest und Korrekturhandoff.

| Datei relativ zum Repository | Aktueller SHA-256 |
|---|---|
| `dossiers/he/he-darmstadt-nord-64293-schrott-aller-art.md` | `0360a9501dad8243cc709e09117b4a2d493bcd1fe6ee1f924fe518ab8d622a98` |
| `dossiers/he/he-eschenburg-35713-metzger-marco.md` | `175c0ca7499eec6370491a3c087485dd1edb2f42eec95d5e240c0bd02cdb3768` |
| `dossiers/he/he-frankfurt-65933-schindling-stoffstrom-vertriebs.md` | `63ca48b8dd2f896d5e380fcb0d60117f5ced87952add8f072a29004c00aaffb8` |
| `dossiers/he/he-frankfurt-eschersheim-die-schrottprofis-abholung-von-schrott-a.md` | `3c53819360523cbcda181d2e511d1b2969cf7e032d37c896b94ef13dd25290c5` |
| `dossiers/he/he-freigericht-schrott-metallhandel-freigericht.md` | `f4943c3837e0b7a4ca13f4c5e179ed3e1a232d459e6d165119c7c40c58db7226` |
| `dossiers/he/he-hanau-63452-bernhardt-thomas.md` | `8de62c4fb73b37c862f2dea63abc7f7ed9075023d72925e1de4af287403761e2` |
| `dossiers/he/he-hasselroth-63594-jk-recycling.md` | `aa6de8457aea3466c77447703ac062c46b47cc9930f2939a0dc7637c1911df99` |
| `dossiers/he/he-kassel-bettenhausen-bachmann-matthias-autoverwertung.md` | `6e283cc163a255a9f206d4e16ba0bd5acbeee840bec019041b986aca880273b1` |
| `dossiers/he/he-marburg-klein-gotthart-schrotthdl.md` | `c0ba2ed5647133e727654c1e8a96cb3aed2c0ef66ca5a2ea2540c7ea8514825a` |
| `dossiers/he/he-offenbach-am-main-kundrat-schrott-containerdienst.md` | `3d54a73e828bfa60220c51168a81ebc84ca7c391342b1af01b043d06f4db7e33` |
| `dossiers/he/he-pfungstadt-64319-stein-alexander-sebastian.md` | `81c56248fedb14812782edecbc27e07f4c46236159484a92a35e181c6b5b80c6` |
| `dossiers/he/he-schaafheim-64850-he-reining-klaus-containerdienst.md` | `585ebfe4f53679406be1fac19c3270ab3a8ede269ff2308cf4d2657a5125c5f2` |
| `dossiers/he/he-tourgebiet-kassel-schrotthandel-richter-mobil-fur-kassel-s.md` | `751d645dda1d15a1ab12b5c9b829fde9149f9e514bdc56efd52f75874cc6c7b9` |
| `dossiers/he/he-weimar-lahn-j-haas-schrott-metallhandel.md` | `899add04217215c9bf94926edf7f7de01ab0d64f9c702ce8811d99de5e370455` |
| `dossiers/he/he-wettenberg-krofdorf-blum-marco-schrottentsorgung.md` | `b6967e1e2e7f848beaaf3d4eecb39dd2a0e8693ba0669042383852b65c85b9fc` |
| `dossiers/he/he-wiesbaden-65183-collecta-rohstoff.md` | `abb04607f5a9d97fb960f5a8051f3524fbbf46028475385014f774513896ff99` |
| `dossiers/he/he-wiesbaden-biebrich-cumic-benjamin-autoverschrottung.md` | `fc33c931dbf26ca54ff6119274fe3755fbb9072b9f276421fa0768ef822ec298` |
| `dossiers/th/th-stadtroda-donndorf-herbert-schrott-und-metallhande.md` | `e7cc56c56f8d745f105d45fec16a7eb3da9185bf28bb78326922df023e5328e0` |
| `dossiers/th/th-tiefenort-b-bad-salzungen-autoverwertung-schleicher.md` | `bc4341cb73ef4b41bd163f1597cc144eea1f66e9d2fc277213c78c438aa96e94` |
| `research/wave-20261013-b.md` | `386f50d279684873d4959f8471859d2893e020db480e73e47e2850b5746ab8dc` |

Scoped `git diff --check HEAD -- <19 Dossiers + Bericht>`: Exit 0, nur Whitespacekontrolle; ungetrackter Bericht wird davon nicht inhaltlich erfasst und wurde separat vollständig gelesen. Keine Tests, Builds, Seedläufe, Datenbank-/Produktivzugriffe, GEO-Arbeit, Commits, Pushes oder Deployments. Ausschließlich diese Endabnahmedatei geschrieben. Spätere scoped Seedtests und Veröffentlichungs-/Produktivprüfung bleiben ein eigener, hier nicht erteilter Auftrag.
