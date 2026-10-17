# Owner-Endprüfung: genau zwölf offene Restdiffs der Welle C vom 13.10.

Stand: 17.10.2026; Prüfbezug `HEAD = 31a3b71966a29c876191f993792a86e6f9207b42`.

## Ergebnis und Reichweite

**Alle zwölf unten hashgebundenen Dossierdiffs sind fachlich als begrenzte Recherche-/Klärnotizen empfohlen. Keine positive Identitäts-, Kontakt-, Annahmestellen-, Preis- oder GEO-Freigabe. Keine Veröffentlichungs-/Seedfreigabe.** Keine zusätzliche zwingende Textkorrektur wie im inzwischen erledigten Missal-Fall festgestellt. Offene Identitäten bleiben offen.

Gelesen wurden README vollständig, `docs/trader-seed.md`, `research/checkin-20261017-1700-wave13-c-review.md`, ergänzend `research/wave-20261013-c.md`, alle zwölf tatsächlichen `git diff HEAD -- <Datei>`, die vollständigen zwölf aktuellen Dossiers einschließlich alter Timeline und die dateibezogenen Commitverläufe. Compiler-/Seedwirkung wurde am vorhandenen Quelltext eingeordnet, nicht ausgeführt. Der alte 20er-Wellenbericht ist weder Liste noch Freigabebeleg der jetzt offenen Dateien.

**Kein neuer HTTP-/DNS-/Registerabruf in dieser Endprüfung.** „Aktuell“, „live geprüft“, „weiterhin“ und Abrufgrenzen in den Dossiers sind dem jeweiligen datierten Rechercheeintrag zugeordnet, nicht durch diese Prüfung auf den 17.10. verlängert. Die Primärabrufe vom 17.10. im 17-Uhr-Bericht sind dort dokumentierte Vorprüfungsbefunde, keine hier wiederholten Abrufe. Insbesondere keine aktuelle Nichterreichbarkeit, HTTPS-Störung, 403-Sperre, Redirectlage oder Suchtrefferlosigkeit behauptet.

Einzige geschriebene Datei dieser Prüfung ist dieser Bericht. Keine Dossieränderung, kein Staging, Commit, Test, Build, Codegenlauf, DB-/Produktionszugriff oder GEO-Eingriff.

## Standards: Feld-, Enum-, Overwrite-, Deep-Link- und Emdash-Gate

| Prüfpunkt | Konkreter Befund für diese zwölf Diffstände |
|---|---|
| Umfang | 67 hinzugefügte, 7 entfernte Zeilen. Fünf Dateien ergänzen ausschließlich Timeline; sechs ersetzen den Überblick-Platzhalter und ergänzen Timeline; Wagner präzisiert einen Überblicksabsatz und ergänzt Timeline. |
| Frontmatter/Felder | Bei allen zwölf bytegleich zu HEAD. Keine neuen, entfernten, duplizierten oder umbenannten Frontmatter-Schlüssel; keine `clear_fields`, `lat` oder `lon`. Slug, Name, Ort, Land, Adresse, Kontakte, Status und Services unverändert. |
| Enums | BW/NW unverändert korrekt. Zehnmal `status: pruefung`; nur MS-Brenner/Renz behalten `aktiv`. Typ `mobil` bei Eco/Eddys, `metallhaendler` bei Müller, sonst `schrotthaendler`. `website_status`: zweimal `aktiv`, viermal `unbekannt`, sechsmal leer. Leeres `website_status` ist laut `validate_seeds` zulässig und nicht das ungültige Enum `''` als neue Behauptung. Keine Enumänderung. |
| Nichtleere Feld-Overwrites | **Keine Frontmatter-Overwrites.** Die einzige inhaltliche Überarbeitung vorhandener Überblicksprosa ist Wagners engere Hofzuordnung; kein Umzug nach Neckarelz und kein Personenwechsel strukturiert. Müllers bereits gefüllte Straße/PLZ/Telefon werden weder ersetzt noch durch Wiederholung verifiziert. |
| Historie/Notenformat | Genau ein neuer datierter Bullet unter `### Recherche 13.10.2026` je Datei, jeweils mit Quellen und Abrufdatum. Der gesamte alte Timeline-Text ist in jeder Datei unverändert als exaktes Suffix enthalten. Keine alte Recherche gelöscht, umgeschrieben oder als neue Bestätigung etikettiert. |
| Website-Deep-Links | Nur zwei nichtleere `website`-Felder: `https://www.ms-brenner.de` und `http://renz-rohstoffe.de/`, beide bereits vorhandene Roots. Kein neuer Portal-/Kontakt-/Impressums-/Preislink als Händlerwebsite. Deep-Links in Recherchequellen sind erwünschte Belegstellen und keine Website-Feldverletzung. Ihre Livefunktion wurde hier nicht überprüft. |
| Emdash | Kein `—` in den zwölf Frontmattern, insbesondere kein unbekannter Wert als Emdash-Platzhalter in Telefon/Adresse/URL/Enum. Sechs alte Überblick-Platzhalter mit Gedankenstrich werden durch Text ersetzt. Gedankenstriche in bestehender Prosa/Historie sind kein Datenfehler und bleiben erhalten. |

Die Aussage „nur Notizen“ bedeutet **nicht** „seedneutral“: `crates/ingestion/build.rs:161–192` kompiliert Timeline-Bullets zu `notes`, nicht den Überblick; `crates/ingestion/src/seed_traders.rs:269–305` nimmt `notes` in den Payloadhash auf. Alle zwölf bekommen somit geänderten Seedinhalt nach einem späteren Rebuild. Kein tatsächlicher Codegen-/Hashwert des Seeds oder erfolgreicher Testlauf wird hier behauptet; die unten stehenden Hashes binden Dateiinhalte, nicht `extra_json.seed_hash`.

### Technisches Restgate / konkreter Overwrite-Vorbehalt

`docs/trader-seed.md:83–105` und `seed_traders.rs:352–439` unterscheiden zwingend:

- Leere Kontakt-/Beschreibungs-/Service-Enrichmentfelder bewahren vorhandene DB-Werte. Kein `clear_fields` in diesen zwölf Dateien: alte unbestätigte `dropoff_json`-/`pickup_json`-Behauptungen werden durch neue Warnnotizen oder `pruefung` **nicht** zurückgezogen. Auch alte Legacy-Serviceflags sind keine bestätigten Leistungen.
- Leere **Straße/PLZ** werden beim durch neue Timeline ausgelösten Upsert direkt übergeben und können vorhandene angereicherte DB-Adressen leeren. Das betrifft die zehn Dossiers **MS-Brenner, Renz, Lehmann, Blum, Eco, Eddys, MAR, Mock, Hartl und Halan**; Wagner und Müller haben bereits nichtleere Adressfelder. Umgekehrt können Wagners/Müllers vorhandene Seedwerte von abweichenden DB-Werten wieder Besitz nehmen. Bytegleiche Frontmatter ist kein Nachweis fehlender DB-Overwrites.
- Keine Koordinatenpaare in diesen zwölf Dossiers: bestehende DB-Koordinaten bleiben erhalten, werden also weder bestätigt noch durch Adressleere automatisch entfernt. Keine Abfrage dieser Koordinaten vorgenommen.

**Blocker einer darüber hinausgehenden Seed-/Produktionsfreigabe:** Der reale gespeicherte Enrichment-/Adresszustand und ein isolierter Lauf des später ausgewählten Seedexports sind hier nicht nachgewiesen; Tests, Build und Prod waren ausdrücklich ausgeschlossen. Vor Veröffentlichung muss der übernehmende Owner dieses Gate getrennt erfüllen. Das ist kein Anlass für eigenmächtige Adress-/Clear-/GEO-Korrekturen in dieser Prüfung.

## Spec/Inhalt: Einzelentscheidungen und gebundene Historie

Die Nummern verweisen auf die exakten Pfade und Inhaltsbindungen im nächsten Abschnitt. „Empfohlen“ bedeutet ausschließlich Übernahme der vorliegenden begrenzten Notiz, nicht Freigabe der darin diskutierten Tatsachen als strukturierte aktuelle Kundendaten.

1. **Billigheim / Wagner — empfohlen.** Der einzige geänderte Bestandsabsatz (`Überblick`, Zeile 28) nimmt die zuvor zu sichere Gleichsetzung Lager/Wertstoffhof zurück. Container-/Impressumsquellen stützen die Unterscheidung Büro Fasanenweg 2 vs. Lager Bahnhofstraße 8/8/3; die neue Notiz behauptet nicht, beide Adressen fehlten auf der Seite, sondern keine eindeutige Hofzuordnung. Containergrößen 10–40/5–12 m³ sind keine Preise. Lorenz→Annegret, Heinrich-Dublette und öffentliche Annahme am Fasanenweg bleiben offen. Historie 02.10. (Verzeichnisfill), 05.10. (Betreiber/EFB und Downgrade), 08.10. (Adressabgrenzung) bleibt vollständig. Letzter relevanter Dossiercommit: `2fb74c179610cbd4f4c5c72fb83593d8130db738`.

2. **Karlsruhe / MS-Brenner — empfohlen.** Nur neuer Timelinebullet (Zeile 36), kein Kontakttausch: Unterfeldstraße 30/Festnetz und Unterer Dammweg 6/Mobil/Zeiten werden als ungelöster Websitekonflikt erhalten. Das ist keine Feststellung, an einer Anschrift gebe es ausschließlich die jeweils zugeordnete Nummer; die Vorprüfung beschreibt Mobilnummer in Kopf/Fuß und Festnetz im Seiteninhalt nebeneinander. Gleiche Domain bleibt eine Quelle; Tagespreise ohne Zahlen, AGB/Einkaufs-/Verkaufsbedingungen ohne Preisliste. Die beiden 01.10.-Notizen und 07.10. bleiben ungekürzt; `status: aktiv` löst den Standortkonflikt nicht. Letzter relevanter Commit: `bbfe667634f128a0705ddb24f597aa7965e8aac2`.

3. **Villingen-Schwenningen / Renz — empfohlen.** Nur Timeline (Zeile 34): Größen 6/7/10/17/36 m³ und Gutschrift nach Metallverkauf nicht als Gratisentsorgung oder Verkaufsangebot aus Händlersicht umgedeutet. Zwei Betreiberdomains nicht als unabhängige Identitätsbelege gezählt. Die HSR-Brandenburg-Fehlzuordnung bleibt ausdrücklich widerlegt (30.09.), Einzelbeleggate 01.10. und HTTP-/Containerbefunde 05.10. erhalten. Der damalige HTTPS-Fehler beweist keine heutige Störung oder tote Website. Adresse Klippeneckstraße 62, Kundenfunktion und Zweitidentitätsbeleg bleiben offene strukturierte Fills. Letzter relevanter Commit: `f533be693a5a30664e9c4ea49e55160fad74b4a1`.

4. **Willstätt / Lehmann — empfohlen, nur Lead.** Neuer Überblick/Timeline (Zeilen 27–37): Schulstraße 31/07852 7503 bleiben Verzeichnisangaben. Freiwillige Gemeindeliste ist keine vollständige Negativauskunft; weder Betriebsschließung noch Annahme/Abholung abgeleitet. Die 03.10.-Recherche mit Downgrade `aktiv`→`pruefung` bleibt erhalten. Keine aktuelle Schlussfolgerung aus dem damals defekten Homepage-Link. Letzter relevanter Commit: `71f456813fcebe03ca2692027efc6b0dc21e8a7b`.

5. **Bergisch Gladbach / Blum — empfohlen, nur lokaler Gegenlead.** Überblick/Timeline (Zeilen 27–37) präzisieren den historischen Suchbefund vom 01.10., ohne ihn zu löschen: lokaler Blum-GmbH-Lead statt bloß Niederzier. Auf der Kaule ohne Hausnummer/40 und Scharrenbroichweg 10a sind konkurrierende Suchspuren, kein Standortentscheid. Portalwurzel `https://schrottplatz-info.de/` ist keine exakt nachprüfbare Einzelfundstelle für die Hausnummer; Quelle für diese Spur bleibt schwach. Template-/HTTPS-Befund gehört zum 13.10., nicht zum heutigen Abrufstatus. Keine Identitätsbrücke zu Gebrüder Blum in Niederzier. Historischer Recherchecommit: `7b64a7aa35807ee5121823902cdb09a90e7e1141`; spätere Provenienzbereinigung `d9d551f00201440b0721421f04e3b44d574ca693` ist keine neue Recherche.

6. **Bochum / Eco — empfohlen, Domainattribution offen.** Überblick/Timeline (Zeilen 27–37) beschreiben generische Angebote, nicht einen nachgewiesenen Bochumer Betreiber. Telefon, Mail, Zeiten und „Schrottabholung DE“-Partnernennung bleiben Text über einen nicht zugeordneten Auftritt. Die Northdata-/Suchtreffergrenze der 01.10.-Historie ist kein heutiger Negativregisterbeweis; Erreichbarkeit vom 13.10. keine aktuelle technische Zusage. Recherchecommit: `52867eb4f19236cd796cc89e9eaf3dc5b1938576`; letzte technische Provenienzbereinigung `d9d551f00201440b0721421f04e3b44d574ca693`.

7. **Coesfeld / Eddys — empfohlen, nur Lead-/Redirectabgrenzung.** Ausschließlich Timeline (Zeile 36): Haugen Kamp 1A/0178 5435699/Zeiten bleiben Gelbe-Seiten-Lead. Keine ASH-Filiale oder Betreiberübernahme aus Weiterleitung abgeleitet. 01.10.-Redirect und 07.10.-Leadvertiefung bleiben erhalten; „weiterhin“ ist der datierte Befund 13.10., kein hier erneuerter Redirectcheck. Betreiberidentität, Verhältnis zu ASH und Annahmestelle offen. Letzter relevanter Commit: `bbfe667634f128a0705ddb24f597aa7965e8aac2`.

8. **Gummersbach / MAR — empfohlen, nur Portalwarnung.** Überblick/Timeline (Zeilen 27–37) kennzeichnen Peter-Heuser-Straße 9/02261 5079155 und Mindestmengen 100 kg Eisen/1 kg NE sowie 07–21-Uhr-Angaben als nicht betreiberspezifisch validierte Vorlage. Kein Mindestmengen-/Öffnungszeiten-/Preisfill; Anfrageverweis ist keine Quote. Importhinweise zu Festnetz/lsp-Profil bleiben unverändert. Kein weiterer alter Rechercheeintrag vorhanden. Letzter Dateicommit `d9d551f00201440b0721421f04e3b44d574ca693` ist Provenienzbereinigung, Originalsplit `5fe1ac8299407c9a52c7ddc3e0ae2e9bccb9a39f`.

9. **Hagen / Mock — empfohlen, nur Lead-/Namensabgrenzung.** Überblick/Timeline (Zeilen 27–37): Stefan Mock/Neue Straße 2/02331 3779915 bleiben Aggregatorangaben. Mehrere Verzeichnisse erfüllen den Primär-/Registerstandard nicht; `schrotthagen.de` wird nicht angehängt. Listen-/Suchseiten bei 11880 und Schrottplatz-Info sind keine einzelnen Betreiberbelege. Die damalige sichtbare Domainprüfung ersetzt keinen heutigen Impressumsvollcrawl. Importhinweise bleiben; keine ältere datierte Recherche im HEAD-Dossier. Letzter Dateicommit `d9d551f00201440b0721421f04e3b44d574ca693`, Originalsplit `5fe1ac8299407c9a52c7ddc3e0ae2e9bccb9a39f`.

10. **Hürth / Hartl — empfohlen, nur Verzeichnislead.** Ausschließlich Timeline (Zeile 36), fortgesetzt aus 06.10.: Rondorfer Straße 76/02233 715798 nicht als Kundenannahme bestätigt. Zwei Branchenverzeichnisse bleiben zwei Leads, nicht zwei admissible unabhängige Belege. Keine Annahme-, Kontakt- oder Preisfreigabe. Letzter relevanter Commit: `91a7b3a1e82352e286618fa99e9aba611e384433`.

11. **Mönchengladbach / Müller — empfohlen, nur expliziter Identitätskonflikt.** Überblick/Timeline (Zeilen 27–37) lösen Marcel/Günter, Festnetz/Mobil und Böcklin/Böckling gerade nicht auf. Die Ausnahme-Einzelfills von 01.10. werden als Historie erhalten, nicht neu legitimiert; die Warnung vom 04.10. bleibt. Auch eine Person mit mehreren Telefonnummern wäre möglich, ist aber nicht belegt: keine sichere Trennung oder Verschmelzung behauptet. **Quellenpräzision:** Der neue Das-Telefonbuch-Link enthält im zweiten Namenssegment `M%C3%B6ller` (Möller), der alte 04.10.-Link dort `M%C3%BCller` (Müller). Keine identische Zielseite oder funktionierende Weiterleitung nachgewiesen; URL-Variante nicht als neue/unabhängige Quelle zählen. Der alte Domainabruf scheiterte damals, nicht nachweislich heute. Vor Identitäts-/Kontaktfill tatsächlichen Betreiberbeleg beschaffen; als offener Warnhinweis kein Integrationsblocker. Letzter relevanter Commit: `b9d7ca6aa272195a239e2241da6a7741ca2889a9`; dokumentierter Altfill in `b19c7f8fdb4f9e9e4a00c18c0aa9afe61f5f1883`.

12. **Wuppertal-Unterbarmen / Halan — empfohlen, nur Portal-/Zeitwarnung.** Ausschließlich Timeline (Zeile 36): Burgunderstraße 19/0202 76713022 bleiben Lead. Dynamisches „öffnet morgen 12:00“ widerspricht der dokumentierten Wochenliste, ist weder heutiger Öffnungsstatus noch bestätigte Öffnungszeit. Template-Mindestmengen werden nicht übernommen. Die alte 01.10.-Aussage „ohne Adresse“ ist bereits durch die erhaltene 08.10.-Vertiefung historisch eingeordnet; den damaligen 11880-403 nicht als aktuelle Sperre ausgeben. Unterbarmen, Betreiber und Kundenannahme unbestätigt. Letzter relevanter Commit: `15421f66eaab3ab9695e443c804673fc8d827a3e`.

Bei allen zwölf bleibt getrennt: **Ankauf** ohne hier freigegebene numerische Quote; **Verkauf** ohne nachgewiesene öffentliche Händler-Verkaufspreisliste; **Gebühren** unbekannt/auf Anfrage statt kostenlos oder null. Containergrößen, Mindestmengen, Gutschriften und Werbung erzeugen keine Preisbeobachtungen. Die zulässigen Erstbetreiber-Ausnahmen der README werden hier nicht nachträglich aus mehreren Seiten derselben Domain konstruiert.

## Exakte Inhaltsbindung der zwölf Empfehlungen

Git-Blob-IDs sind SHA-1 über Git-Blobobjekte; SHA-256 unten jeweils über die vollständigen unveränderten Arbeitsdatei-Bytes. Jede Empfehlung gilt nur für das angegebene Paar `HEAD-Blob → Arbeitsdatei-Blob`. Änderungen an Basis oder Inhalt erfordern erneute Prüfung. Keine DB- oder Compiler-Payloadhashes.

| Nr. | Exakter Dossierpfad | HEAD-Blob | Arbeitsdatei-Blob |
|---|---|---|---|
| 1 | `dossiers/bw/bw-billigheim-wagner-lorenz-schrott.md` | `53ec98ce22d68431fbc70c749d835f0387ad22a2` | `9695b4d633ed5542c9b41bdb341e1a23e53143d3` |
| 2 | `dossiers/bw/bw-karlsruhe-ms-brenner.md` | `da6aae378e7af61a2017541f78415ec79018a2ff` | `28e9470fa7cc11b6169581123cf840b580ab2310` |
| 3 | `dossiers/bw/bw-villingen-schwenningen-renz-rohstoffe-renz-container.md` | `049aa5a2ddb1c521cd5dbaefc4d8819142bdf5c1` | `b290a7f1785516b0e013d795d96e41da02fac825` |
| 4 | `dossiers/bw/bw-willstatt-lehmann-willi-schrott.md` | `ee51a4922f4a780fc8e8035d201338d2d3fb3212` | `88fb6cbd192309327ec22ff5caef3cbeb9150870` |
| 5 | `dossiers/nw/nw-bergisch-gladbach-blum.md` | `5afb4e3a43ef144a30e81934fc8514f3da232672` | `2570645571ae62dc3cfbcc9a8ecbb58eaf7882da` |
| 6 | `dossiers/nw/nw-bochum-eco.md` | `5ea4484acdb7834fa42062a91d99dcdd58a1898b` | `3698030182f4cc7224edd122180c84f2ab8ef786` |
| 7 | `dossiers/nw/nw-coesfeld-eddys.md` | `aa4106f5705079bb7dad73c74b5429d58ef766dc` | `b4ef990dc68ee143556a2cf7c3d0dfa5b8d49789` |
| 8 | `dossiers/nw/nw-gummersbach-mar-schrottabholung-gummersbach.md` | `86786d4e1c944a51f133f62367342535eda6438b` | `c538378f57ba9a0fd3e0bd7a19fe4f6b16f1f7f4` |
| 9 | `dossiers/nw/nw-hagen-mock.md` | `494732520cc4773c35983ec27f10c9f19cff8833` | `328fe7bda8f932d43168d9585f516eb2b648f17e` |
| 10 | `dossiers/nw/nw-hurth-hartl.md` | `a0a8e281fb262356463fee554c5689569e41b70e` | `b4580925adaa98d6b40b7218f439c3448bcc7e7a` |
| 11 | `dossiers/nw/nw-monchengladbach-metallrecycling-muller.md` | `4628287249706834bffb3850c1acf3889d3df5e7` | `6696a5f68eb1c6a396c94602818aa3a07e2b4cfb` |
| 12 | `dossiers/nw/nw-wuppertal-unterbarmen-halan.md` | `eaa4777a49aef62bf96e7e3ae59a1d91ccccb4c7` | `9b1c98cb43bdfe91cc112995c24ec0688b50f14d` |

| Nr. | SHA-256 Arbeitsdatei |
|---|---|
| 1 | `a5e1d35c1992740e308d95cf2d6df6325cec5af17fb16c992565d689176b090e` |
| 2 | `512e6fbebc179d3f146b12f8a7e0d10f4928fac9173732f9beae9a2b5a9568db` |
| 3 | `e2de0eceb6e0877d2431813dd4cce489cf5153444a943ad05000853f6e1a9fe2` |
| 4 | `2ddae1902d13993e414179f4c5b849a143b790371fa182095eb01e763db8511f` |
| 5 | `809779da808f498380dc051708928a4b2699440ed9ba44c2d4c76c08771076a2` |
| 6 | `d9e25ca32e3bc59721c39544f70802cc2fdcdc47971f17d0afaf024fe2b70d4b` |
| 7 | `8992925304277748acde9ad95892092e45a52bf4663202bde9698e8efdbdfb28` |
| 8 | `cba9fc1f3b2c50a86464936551114fcf851d4c3b97971f77a82ae777a45bcd8d` |
| 9 | `b1df0a924e68fe54d2673ff0108a35ab20e12344ae728571e7e3af429106922d` |
| 10 | `ab4ae2e5860f503e1eb673a4d6ddd911a65b5cbc8feca80e160a0722bc2eb5e9` |
| 11 | `74e4993d38de2caf342918e50476eae3967c42b4814c284a9cb6fcb7210ad5ef` |
| 12 | `6937bf58ad0600a8f5769fb88b7e725afe6b1a926419f53fcc1097c457dee9b6` |

## Geschützte, bereits erledigte Fälle — nicht wieder aufrollen

Die sieben im Vorbericht integrierten Fälle sind bei der Prüfung ohne lokale Änderungen; ihre folgenden vollständigen letzten relevanten Commit-IDs wurden dateibezogen ermittelt. Keine dieser Dateien wurde bearbeitet:

| Dossier | Commit |
|---|---|
| `dossiers/bw/bw-karlsruhe-metall-service-pedack.md` | `86fd612f88c60f75a692d67757cad7e5ab0af489` |
| `dossiers/bw/bw-neu-ulm-russ-entsorgung.md` | `6cf026ba7fa8a43d70fe0ba9417d741bf6246fa0` |
| `dossiers/nw/nw-bad-salzuflen-hilgenberg.md` | `733d9aba8d136b693a748c97408acb7f7bbf6888` |
| `dossiers/nw/nw-bochum-fix.md` | `36463aeee47dcc23f7ac705e0231dfe7af2b584e` |
| `dossiers/nw/nw-gutersloh-gmg.md` | `733d9aba8d136b693a748c97408acb7f7bbf6888` |
| `dossiers/nw/nw-hamm-heinrich-hark-ndl-hamm.md` | `4bc166dfaf2ff5d8415a7737f09630054e088d51` |
| `dossiers/nw/nw-kreuztal-mario-deutsch-metallrecycling.md` | `4bc166dfaf2ff5d8415a7737f09630054e088d51` |

Auch **`dossiers/nw/nw-essen-schrott-und-metallhandel-j-missal.md` ist sauber**: Die im Vorbericht geforderte Begrenzung „öffentliche Kundenannahme weder belegt noch ausdrücklich ausgeschlossen“ samt erhaltener Historie ist bereits in `dbb61a659a9a2f62506685cd7b2b6dd161f4c29a` committed. Kein 13. offener Fall, keine erneute Korrektur. Russ' historische BW-Warnung nicht gegen die spätere bestätigte Landeszuordnung ausspielen.

## Abschlussgrenzen

- **Standardsachse:** keine neue Feld-/Enum-/Deep-Link-/Emdash-Verletzung und kein Historienverlust in den zwölf gebundenen Diffs. DB-Overwrite-Gate bleibt separat offen.
- **Spec-/Inhaltsachse:** zwölf begrenzte Notizempfehlungen; keine zusätzliche zwingende Dossierkorrektur. Nichtpräzise Lead-Fundstellen bei Blum/Mock und Müllers URL-Variante sind ausdrücklich keine aufgewerteten Primärbelege. Sämtliche im Einzelreview genannten Identitäts-/Annahmestellenfragen bleiben vor strukturierten Fills blockierend.
- **Verifikation:** statische Datei-/Diff-/Historienprüfung, keine Test- oder Publikationsbehauptung. Die vom allgemeinen Review-Skill vorgesehenen zwei zusätzlichen Review-Subagenten konnten wegen des Harness-Tiefenlimits (`Subagent depth limit reached (1)`) nicht gestartet werden; beide Achsen wurden hier selbst gelesen. Keine unabhängige Zweitagentenfreigabe suggeriert.

**Übergabe:** ausschließlich die zwölf exakt gebundenen Recherche-/Klärnotizen fachlich empfohlen; tatsächlicher Integrationsschritt und technische Gates bleiben beim übernehmenden Owner. Acht bereits erledigte Fälle bleiben unangetastet.
