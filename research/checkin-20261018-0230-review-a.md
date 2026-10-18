# Begrenzter Integrationsreview – Shard a

Review: 18.10.2026. Referenz: `HEAD 8ac7c1d3b7a6d71cc964db74a691e892383bec8a` und exakt die sechs `shards.a`-Pfade aus `/tmp/opencode/checkin-20261018-0230-review-scope.json`.

**Ergebnis: zwei Änderungen lokal integrierbar, vier nur nach den unten genannten Textkorrekturen. Keine pauschale Bestätigung der Händlerdaten oder der Recherche am 12.10.** Alle sechs Dateien, ihre vollständigen Diffs gegen HEAD und die letzten dateibezogenen Commits wurden gelesen. Alle sechs SHA-256 stimmen mit dem Scope überein. Frontmatter ist jeweils bytegleich; die gesamte alte Timeline bleibt als geordnete Teilfolge erhalten. Kein Slugwechsel, kein Feld-Fill und keine Löschung alter Erkenntnisse. Änderungen an Überblicken betreffen vorherige Platzhalter (Karola, SPARO, Lachermeier).

## Maßstab und Grenzen

- `README.md:160–202`: zweiseitige Identitätsprüfung, keine Namensvetter-/Nachfolgeverschmelzung; Verzeichnisse sind Leads, nicht Belege. Zwei unabhängige zulässige Quellen oder die eng definierte Betreiber-Primärquellen-Ausnahme; letztere ersetzt keinen zweiten Identitätsbeleg.
- `README.md:204–240`: Betreiberketten belegen, Historie bewahren, keine unbelegten Feldüberschreibungen. Unveränderte Altfelder sind damit nicht automatisch richtig. Timeline-Bullets werden laut `README.md:111–122` zu Seed-Notizen; auch reine Textintegration ist keine wirkungslose Ablage.
- Eigene Direktabrufe erfolgten am **18.10.**, nicht am 12.10. Ein heutiger Seiteninhalt beweist den damaligen Abruf nicht. Negative Suchbefunde und fehlende Preislisten gelten nur innerhalb der konkret geprüften Quellen, nicht als allgemeine Nichtveröffentlichungsbehauptung.
- Keine Dossiers geändert, keine Gitmutation, keine Builds/Tests, keine Produktionszugriffe oder GEO-Arbeit. Die vom geladenen Review-Skill vorgesehene Unterdelegation scheiterte am Subagent-Tiefenlimit; Standards und Auftragsabgleich wurden hier selbst geprüft.

## Sechs explizite Dispositionen

### 1. `dossiers/bb/bb-peitz-besse-udo-schrotthandel.md`

**Disposition: ANNEHMEN – ausschließlich als begrenzte Recherche-/Unsicherheitsnotiz.**

Der neue Bullet (Z. 46) wiederholt die bereits am 03./07.10. dokumentierte Trennung Udo/Lutz ohne neue operative Zuordnung. Direkt gelesen: [amtliche Landkreis-Detailseite](https://www.lkspn.de/kreisverwaltung/untereabfallbehoerde/schrottplaetze/4304-schrotthandlung-und-containerdienst.html). Sie nennt tatsächlich Lutz Besse, Ottendorfer Straße 1 a als Betreiberanschrift, Gewerbeparkstraße 12 als Standort, Mo–Fr 08–12/13–15:30 und **03560 22589**. Kein Preis ist dort ausgewiesen. Das bestätigt die wiedergegebenen Seitenangaben, nicht den Udo-Seed oder dessen Tätigkeit. Die abweichende Verzeichnisnummer **035601 22589** wurde hier nicht separat erneut verifiziert; sie ist bereits historisch dokumentierter Konflikt, keine neue Kontaktfreigabe.

Keine erforderliche sachliche Dossierkorrektur für diese Integration. Die Preisnegativa im Kontext ausdrücklich auf diese geprüfte Quelle beziehen, nicht als marktweite Aussage lesen. `pruefung` bleibt erforderlich; kein Übertragen von Lutz-Daten.

### 2. `dossiers/bb/bb-teltow-schrott-karola.md`

**Disposition: NUR NACH KORREKTUR ANNEHMEN – Lead ja, doppelt reproduzierbarer Abruf noch nein.**

Der neue Überblick und Bullet (Z. 27–29, 44) behandeln die Portale zutreffend nicht als Betreiberbelege und behalten den früheren MISS als historischen Befund. [Gelbe-Seiten-Detailseite](https://www.gelbeseiten.de/gsbiz/056e1281-d058-4f1f-9fc5-4534674933bb) direkt gelesen: „Schrott Karola“, Elbestr. 11A, 14513 Teltow, 03328 30 44 90 stimmen als **Verzeichnisangaben**. Der zweite Nachweis ist lediglich die Startseite `https://www.dasoertliche.de/`; eine ergänzende Websuche lieferte hier keinen belastbaren konkreten Treffer. Das widerlegt den Alt-Abruf nicht, macht ihn aber nicht reproduzierbar.

**Erforderlich:** konkreten Das-Örtliche-Profillink/Abrufbeleg ergänzen oder im neuen Text auf „Gelbe Seiten führen …; ein weiterer Verzeichnisabgleich ist hier nicht reproduzierbar dokumentiert“ begrenzen. „Zwei Branchenverzeichnisse führen …“ nicht als im Review bestätigte Aussage übernehmen. Preisbefund formulieren als „in den genannten Verzeichnisquellen keine belastbaren Preis-/Tarifangaben“. Kein Frontmatter-Fill, kein Aktivitäts- oder Schließungsurteil.

### 3. `dossiers/be/be-spandau-sparo-spandauer-rohstoffe.md`

**Disposition: NUR NACH KORREKTUR ANNEHMEN – sachgerechte Einschränkung, aber alte Quellenfreigabe nicht ausreichend überholt.**

[North Data](https://www.northdata.de/SPARO+Spandauer+Rohstoffe+GmbH,+Berlin/HRB+73519+B) direkt gelesen: Name, HRB 73519 B, Brunsbütteler Damm 430 und Gegenstand „Handel mit Schrott, Altmetallen, NE-Metallen und ähnlichen Rohstoffen ausschließlich deren Verwertung“ stehen dort. Kein Primär-Registerauszug wurde gelesen. [Implisense](https://implisense.com/de/companies/sparo-spandauer-rohstoffe-gmbh-berlin-DEEPKBNWUK13) lieferte **403**; keine frische Zweitquellenbestätigung. North Data ist hier ein laut README zulässiger Register-Sekundärbeleg, keine aktuelle Annahme-/Kontaktbestätigung.

Der neue Text (Z. 27/33) stuft Telefon/E-Mail richtig zu Leads zurück. Die erhaltene alte Timeline Z. 43 erklärt dennoch „2-Beleg-Standard ERFÜLLT (Register + Verzeichnisse), status aktiv bestätigt“ und begründet Telefon/E-Mail-Fills ausschließlich mit Aggregatoren. Das widerspricht `README.md:183–191`. Die neue Wendung „Frontmatter-Status bleibt wie zuvor“ darf nicht stillschweigend die alte Freigabe fortschreiben.

**Erforderlich:** neuen Bullet ausdrücklich ergänzen: „Die historische Aussage vom 01.10. zum erfüllten Zweiquellenstandard wird nicht bestätigt; Verzeichnisse zählen nicht als zweiter Beleg. Telefon, E-Mail und Status aktiv sind unverändert übernommene, hier nicht revalidierte Altfelder; Feldprüfung offen.“ Im Überblick „Das Register belegt“ zu „North Data dokumentiert“ präzisieren. Implisense-403 als Review-Abrufgrenze festhalten, nicht rückwirkend zum 12.10. behaupten. Altbullet nicht löschen/umschreiben. Für diese begrenzte Textintegration keine eigenmächtige Feldlöschung; auch keine Datenfreigabe.

### 4. `dossiers/bw/bw-ehningen-kurt-rie-nachfolger.md`

**Disposition: NUR NACH KORREKTUR ANNEHMEN – Quellenaufwertung ist ein Integrationshindernis.**

[Ortsseite](https://schrottregister.pages.dev/ort-ehningen) und [verlinkte Betriebsdetailseite](https://schrottregister.pages.dev/betrieb-kurt-riess-nachfolger-schrott-und-metallhandel-gbr-ehningen) direkt gelesen. Das Angebot bezeichnet sich ausdrücklich als **„Unabhängiges Verzeichnis · kein amtliches Angebot“**. Seine Quellenangabe GESA/EfB und sein selbst genannter Abrufstand 11.08.2026 machen es nicht zum amtlichen Register. Es nennt Name/Adresse, aktuell **„Lagern, Behandeln“**, auf der Detailseite Zertifikatsnummer **ZZHT00300696**. Das sind Ansatzpunkte für eine Primärprüfung, keine in diesem Review verifizierte Zertifizierung. Kein amtlicher Datensatz/Zertifikat wurde abgerufen.

Der neue Bullet Z. 38 nennt die Zusammenstellung weiterhin „Beleg für die registrierte Adresse/Tätigkeit“ und wiederholt damit die überstarke historische Quellenwertung aus Z. 27/34/46. Die sinnvolle Begrenzung bezüglich öffentlicher Kundenannahme heilt das nicht.

**Erforderlich:** neuen Bullet auf „Privates Verzeichnis nennt …; amtliche Registrierung, Zertifikatsgültigkeit und Standortzuordnung hier nicht primär verifiziert“ ändern; ausdrücklichen Vorbehalt zu den älteren Formulierungen „verifiziert“/„starker Einzelbeleg“ ergänzen. Wenn Tätigkeiten referiert werden, beide heutigen Verzeichnisangaben nennen oder kenntlich machen, dass „Lagern“ nur die damalige Angabe war. Preisnegativum nur auf gelesene Seiten beziehen. Kein GEO-Freibrief aus der Verzeichnisadresse und keine neue Register-/Annahmebehauptung. Altfelder bleiben im Rahmen dieses Auftrags unangetastet, aber nicht freigeprüft.

### 5. `dossiers/bw/bw-freudenstadt-heinzelmann-containerdienst.md`

**Disposition: ANNEHMEN – als Nichtzuordnungsnotiz, nicht als Ausschluss einer Nachfolge.**

[Klumpp-Startseite](https://www.klumpp-rohstoffe.de/) und [Impressum](https://www.klumpp-rohstoffe.de/impressum/) direkt gelesen: Klumpp Rohstoffe GmbH, Alte Poststraße 35, 72250 Freudenstadt, 07441 4819, Geschäftsführer Rolf/Pascal Klumpp, HRB 430709 werden selbst angegeben. Keine der beiden Seiten dokumentiert Heinzelmann oder einen Übergang. Das neue Z. 50 bleibt korrekt bei „Verbindung offen“ und übernimmt keine fremden Preise/Kontakte. Es ergänzt die erhaltenen Prüfungen vom 02./04./07.10. konsistent.

Keine notwendige Dossierkorrektur. „Nicht eine Heinzelmann-Filiale oder dokumentierte Übernahme“ ist als fehlende Dokumentation **auf diesen Seiten**, nicht als erwiesenes Nichtbestehen einer Kette zu verstehen. Keine registerseitige Verifikation von Klumpp im Review, kein Schließungsbeleg für Heinzelmann; dessen Identität bleibt offen.

### 6. `dossiers/by/by-kirchdorf-lachermeier.md`

**Disposition: NUR NACH KORREKTUR ANNEHMEN – gute Feldzurückhaltung, aber Abruf-/Aktualitätspräzision fehlt.**

[Startseite](https://lachermeier-schrott.de/), [Impressum](https://lachermeier-schrott.de/impressum), [Sitemap](https://lachermeier-schrott.de/page-sitemap.xml) und [North-Data-Suche](https://www.northdata.de/?query=Georg+und+Maria+Lachermeier+GbR) selbst gelesen. Name, Marsstr. 4, Ortsteil Nörting, Mail und Zeiten stehen auf der Startseite; USt-ID DE 128059895 im Impressum. Achtung: Startseite zeigt **08166 7902**, Impressum sichtbar **+49 (0) 1866 7902**, dessen `tel:`-Ziel aber `004981667902`. Die Zusammenfassung „Betreiberseite und ihr Impressum … 08166 7902“ verdeckt diesen internen Schreibkonflikt.

Die Sitemap enthält für die Startseite `lastmod=2026-10-12T12:01:23+00:00`; das ist eine technische Eigenangabe, kein unabhängiger Aktualitätsbeleg für Betrieb, Stunden oder Identität. North Data zeigt 15 von 678 Resultaten, darunter keinen passend zuordenbaren Treffer auf der gelesenen ersten Seite. Das ist keine umfassende Suche und erst recht keine negative Registerauskunft; die vorhandene Einschränkung letzterer ist richtig. Die im neuen Text erwähnten alten Marsstr.-3/4-Leads haben keine konkreten Links im Dossier und waren in HEAD nicht dokumentiert.

**Erforderlich:** Telefonangaben nach Startseite/Impressum trennen und Konflikt nennen; „aktuelle Primärquelle“ begrenzen auf „am Reviewtag erreichbare Betreiber-Selbstauskunft, Aktualität der Betriebsdaten nicht unabhängig bestätigt“; Sitemap ausdrücklich nur als `lastmod`-Eigenangabe; North-Data-Negativbefund auf die erste Ergebnisseite beschränken; Hausnummern-Leads konkret belegen oder als nicht reproduzierbaren Recherchehinweis markieren. „Keine … veröffentlicht“ auf „auf Startseite/Impressum keine konkreten Bedingungen/Kurse/Tarife gefunden“ begrenzen. Die Feldzurückhaltung ist nach README sinnvoll: keine Registernummer/Live-Registerbestätigung für die Ausnahme, kein unabhängiger zweiter Identitätsbeleg. Weder USt-ID noch Sitemap sind zweite Quellen.

## Auftragsabgleich / Freigabeumfang

Der Änderungssatz bleibt strukturell im Auftrag: vorhandene Recherchetexte, keine Skalar-/Service-/GEO-Änderung, keine Historienlöschung. Die vier textlichen Nacharbeiten sind vor einer vorbehaltlosen lokalen Übernahme nötig und wurden **nicht** in den Dossiers ausgeführt. Für zwei akzeptierte Änderungen bedeutet „annehmen“ nur: die begrenzte Notiz integriert sich ohne neue belegwidrige Zuschreibung. Es bedeutet weder verifizierten Händlerbetrieb noch erfolgreiche Seed-Tests, Commit, Merge oder Produktionseinspielung.
