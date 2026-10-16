# Seedreview A: vier Kontakt-/Zeitgates

Prüftag: **16.10.2026**. Ziel: **81a6483208ade84c592e89a8684f56c9774dcea7**. Ausschließlich Recherche/Empfehlungen; keine Dossieränderung, kein Seedlauf, Commit, Deployment oder Produktionswrite. Bausch, SM Metals und Limekon sind nicht Gegenstand dieses Berichts.

## Entscheidung für den Owner

**AGOSI `phone` und `email` freigeben, ausdrücklich als Zentralkontakt. Die sieben vorgeschlagenen Kontakt-/Zeitwerte bei Fernau, Graf und Willeführ noch nicht vorbehaltlos freigeben.** Dort sind konkrete Werte gefunden und teilweise stark zusätzlich abgesichert; die unten benannten verbleibenden README-Kriterien sind aber nicht durch einen tatsächlich gewonnenen aktuellen Statusbeleg ersetzt. Das ist keine Feststellung falscher Telefonnummern oder geschlossener Betriebe.

Die strenge Abgrenzung ist absichtlich sichtbar: Ein aktuell abgerufenes Northdata-Profil mit „Aktuelle gesetzliche Vertreter“ und alten/neuen Bekanntmachungen ist ein brauchbarer Register-/Identitätsbeleg. Es enthält in diesen drei Abrufen aber **kein unternehmensbezogenes positives Aktiv-Statusfeld**. Die Auswahl „Aktiv / Liquidation / Erloschen“ im allgemeinen Suchformular ist kein solcher Beleg. Fehlende Löschungsmeldungen werden hier nicht zu einem positiven Live-Nachweis umgedeutet. Wer den aktuellen Northdata-Strukturstand dafür genügen lässt, kann bei Fernau/Graf eine weniger strenge Entscheidung treffen; dieser Bericht behauptet die vollständige Erfüllung aller Ausnahmebedingungen gerade nicht.

| Dossier | Feld und exakter Releasewert | Empfehlung / konkrete Grenze |
| --- | --- | --- |
| `bw-pforzheim-agosi-ag` | `phone: '+49 7231 960-0'` | **Frei:** Betreiberimpressum/Kontaktseite, HRB 740361 und aktueller GLEIF-Live-Nachweis, aktuelle Betriebsruhe 2026. Zentrale, nicht NE-Annahmetelefon. |
| gleich | `email: 'info@agosi.de'` | **Frei:** expliziter Zentralkontakt, gleiche Qualifikation. Keine persönliche oder NE-spezifische Mail daraus ableiten. |
| gleich | `opening_hours: ''` | **Keep/kein neuer Skalar:** Die Quelle hat mehrere verschiedene Zeittypen; keine davon ohne Funktionsangabe zum einzigen Öffnungszeitwert machen. Präzise belegte Alternative unten. |
| `ni-nienburg-fernau-container-schrott-metall` | `phone: '05032 4683'` | **Wert zusätzlich im echten Auditbericht S.3 bestätigt; Veröffentlichungsgate verbleibt:** positiver aktueller Register-Live-Status nicht gewonnen; Impressums-Geschäftsführung nicht registerkongruent. Nicht als falsche Nummer kennzeichnen. |
| gleich | `email: 'info@fernau-containerdienst.de'` | **Gleiche begrenzte Zurückhaltung:** ebenfalls Audit S.3 plus Betreiber. Audit ist eigenständige fachliche Urheberschaft, aber hier über dieselbe Betreiberwebsite bezogen; nicht ungeprüft als zweiter Beleg aus der geschlossenen README-Liste zählen. |
| gleich | `opening_hours: 'Mo–Fr 07:00–17:00 Uhr'` | **Noch nicht frei nach vollständiger Ausnahmeprüfung:** tatsächlicher Betreiber-Footer belegt genau diese Zeiten, Audit/Zertifikat belegen sie nicht. Keine zweite qualifizierte Stundenquelle gewonnen. |
| `sh-kiel-graf-abfallbeforderung-und-entsorgung` | `phone: '0431 52 16 18'` | **Aktualitäts-Teilgate gelöst, Live-Teilgate offen:** Impressum, standortbezogene Seiten und datierte BFSG-Erklärung bestätigen den Kontakt; HRA/Person stimmen mit Northdata überein. Kein positiver aktueller Registerstatus gewonnen. |
| gleich | `email: 'info@graf-entsorgung.de'` | **Gleiche Grenze:** tatsächlich dekodierter Cloudflare-Token und unverschleiertes Schwester-Impressum stimmen überein; zwei Betreiberdomains sind keine zwei unabhängigen Quellen. |
| gleich | `opening_hours: ''` | **Keep:** keine belastbaren Zeiten in den tatsächlich gelesenen Seiten. Nicht „geschlossen“ oder „nur nach Vereinbarung“ ergänzen. |
| `by-aschaffenburg-obernau-63743-willefuhr-tanksanierung-demontage-metall` | `phone: '06028 21188 / 0170 4765501'` | **Nicht freigeben:** beide Nummern primärquellenbelegt, aber Live-/Aktualitätsgate und ROWIS-Zuordnung nicht vollständig gelöst. Festnetz und Mobilteil sind getrennt betroffen; nicht die kombinierte Zeichenkette pauschal durchwinken. |
| gleich | `email: 'info@willefuehr-whg.de'` | **Nicht freigeben:** in beiden Betreiberimpressen vorhanden; keine zweite unabhängige qualifizierte Kontaktquelle und keine vollständige Ausnahme. |
| gleich | `opening_hours: ''` | **Keep:** keine nachgewiesenen Stunden. Tankservicekontakt ist kein öffentlicher Schrottannahmezeitplan. |

„Frei“ bedeutet nur Quellenfreigabe der bezeichneten Felder. Die bereits vorhandenen `status: aktiv` bei AGOSI/Fernau werden dadurch nicht neu geprüft oder pauschal bestätigt; `pruefung` bei Graf/Willeführ nicht hochstufen. Keine Preis-, Annahme-, Abhol-, Privatkunden- oder GEO-Freigabe.

## Ausgangspunkt und wirkliche Seedwirkung

Die vier committeten Dossiers sowie die committeten Berichte `research/wave-20261012-a.md`, `research/wave-20261013-fresh-a.md`, `research/wave-20261013-wave2-c.md` und die zugehörigen Ownerberichte wurden als Ausgangspunkt gegen `git show 81a64832:<Pfad>` gelesen. Der vorherige `research/checkin-20261016-0300-upgrade-seed-review.md` benennt die Kontaktgates, ist aber kein unabhängiger Primärbeleg. Fremde Änderungen im Arbeitsbaum wurden nicht als neuer Dossierstand übernommen.

Erneuter lesender Abgleich mit `/root/Documents/checkin-20261016-0300-release-1wz2obd5/seed-rehearsal-before.json` und `seed-rehearsal-diff.json`:

- **Alle acht Kontaktfelder** dieser vier Händler stehen im echten Vorher-Snapshot leer und werden durch den Zielseed gefüllt. Es handelt sich nicht bloß um Formatkorrekturen existierender Kontakte.
- Nur Fernau hat zusätzlich eine echte Stundenänderung: leer → `Mo–Fr 07:00–17:00 Uhr`. Die drei anderen Stundenfelder bleiben leer.
- Fernau wechselt tatsächlich von `city: Nienburg` ohne Straße/PLZ auf Neustadt am Rübenberge / Justus-von-Liebig-Straße 22 / 31535. Name/Adresse/Domain werden hier nicht zurück auf den historischen Importort gesetzt. Der Zertifikats-/Registerabgleich unten stützt Neustadt, nicht eine Nienburger Niederlassung.
- Bei allen vier enthält dieser Diff **keine** `lat`-/`lon`-Änderung. Insbesondere bleibt bei Fernau im Vorher-Snapshot `52.6521 / 9.2382` erhalten. Die neuen Neustädter Kontakte machen den alten Pin nicht richtig. Kein Pin wurde hier geografisch verifiziert.

## AGOSI / Pforzheim: Zentralkontaktgate geschlossen

Tatsächlich abgerufen und gelesen:

1. [Impressum](https://agosi.de/impressum/): Agosi AG, Kanzlerstraße 17, 75175 Pforzheim; Amtsgericht Mannheim **HRB 740361**, Telefon +49 7231 9600 und `info@agosi.de`.
2. [Kontaktseite](https://agosi.de/kontakt/): dieselbe Zentrale; getrennte konkrete Pforzheimer Funktionen/Adressen und Zeiten.
3. [Northdata HRB 740361](https://www.northdata.de/Agosi%20AG,%20Pforzheim/Amtsgericht%20Mannheim%20HRB%20740361): Agosi AG / Kanzlerstraße 17 / HRB 740361; LEI **529900IXMCVMVVHZWC76**. Register-/Namenszuordnung passt zum Impressum.
4. [GLEIF-Einzeldatensatz](https://api.gleif.org/api/v1/lei-records/529900IXMCVMVVHZWC76): `entity.status = ACTIVE`, `registration.status = ISSUED`, `corroborationLevel = FULLY_CORROBORATED`, `registeredAs` und `validatedAs` jeweils HRB 740361. Namens-/Sitzdaten identisch; letzter Datensatzupdate 10.04.2026, nächste Erneuerung 14.05.2027, Golden-Copy-Datum 15.10.2026. Dies ist ein tatsächlich gelesener positiver Rechtsträgerstatus, kein behaupteter amtlicher Tagesauszug und kein Beweis geöffneter Annahme am Abrufstag.
5. [Betriebsruhe-PDF 2026](https://www.agosi.de/wp-content/uploads/Holiday-Kalender-2026.pdf): Datei selbst abgerufen und mit `pdftotext` gelesen; nicht nur den Dateinamen gewertet. Enthält Feiertage/Brückentage 2026 und Betriebsruhe 24.–31.12.2026, wieder erreichbar ab 04.01.2027. Damit konkrete aktuelle Betreiberinformation vorhanden.

**Ausnahmeprüfung:** benannter Rechtsträger mit Registerkennung vorhanden; Northdata/GLEIF kongruent und positiver Live-Status; tatsächliche standort-/funktionsbezogene Kontaktseite; inhaltlich gelesene Aktualität 2026. Die README-Ausnahme für die beiden eigenen Zentralkontakte ist damit begründet, nicht pauschal über die ganze Website gelegt.

**Stundenfeld nicht vermischen:**

- Zentrale **telefonisch erreichbar** Mo–Do 08:00–15:00, Fr 08:00–12:00.
- **Kundenschalter Robert-Bauer-Straße:** Mo–Fr 08:00–16:00.
- **NE-Metallhandel Am Hauptgüterbahnhof 24, 75177:** Mo–Fr 07:00–16:00, **Materialanlieferung nur bis 14:30**, Telefon +49 7231 960-430.

Belegbare alternative Stunden-Zeichenkette, falls der Owner später bewusst einen funktionsbeschrifteten Sammelwert will: `Telefon Zentrale: Mo–Do 08:00–15:00, Fr 08:00–12:00; Kundenschalter Robert-Bauer-Straße: Mo–Fr 08:00–16:00; NE-Metallhandel Am Hauptgüterbahnhof 24: Mo–Fr 07:00–16:00, Materialanlieferung bis 14:30; Feiertage/Betriebsruhe laut Betreiber.` **Keine Empfehlung, dies jetzt zusätzlich einzuspielen**: Der geprüfte Zielseed enthält dafür keinen Write und hat keine einzelne qualifizierte Hofadresse. Weder zentrale Telefonnummer noch Telefonzeiten als NE-Hofkontakt umetikettieren.

## Fernau / Neustadt: Werte stark bestätigt, Restgate ausdrücklich isoliert

Tatsächlich gelesene Quellen:

1. [Impressum](https://fernau-containerdienst.de/impressum): Fernau Containerdienst GmbH, HRB 110122 Hannover, Justus-von-Liebig-Straße 22, 31535 Neustadt; Telefon +49 5032 4683, Mail `info@fernau-containerdienst.de`; Footer Mo–Fr 07:00–17:00. Als Geschäftsführer nennt die abgerufene Seite **Jörg Fernau und Klaus Dietrich Urban**.
2. [Über uns](https://fernau-containerdienst.de/ueber-uns): konkreter Neustädter Betrieb, gleiche Kontaktdaten/Zeiten; verlinkt die beiden nachfolgenden Original-PDFs.
3. [Efb-Zertifikat ZZCT004000811009](https://fernau-containerdienst.de/files/pdf/zzct004000811009-efb-zertifikat.pdf), alle acht Seiten als Text gelesen: Zertifikatsnummer **25-8052-18-28**, Institut für Umwelttechnik Dr. Kühnemann & Partner GmbH, Prüfung 20.11.2025, Ausstellung 25.11.2025, **gültig bis 19.05.2027**. Hauptsitz und Anlagen: Justus-v.-Liebig-Straße 22 / 31535 Neustadt, HRB 110122 Hannover. Firmenwortlaut noch „Fernau Containerdienst, Schrott- und Metall-Handelsgesellschaft mbH“. Keine Öffnungszeiten darin. Markierungen von Tätigkeits-/AVV-Kästchen wurden hier nicht bildlich vollgeprüft; aus PDF-Fließtext keine pauschale Abfall-/Annahmefreigabe ableiten.
4. [Auditbericht November 2025](https://fernau-containerdienst.de/files/pdf/251120-auditbericht-fernau-2025.pdf), elf Seiten als Text gelesen: S.2 dokumentiert Registerauszug **06.11.2025**, HRB 110122 und Gewerbeberichtigung Stadt Neustadt; das ist ein Bericht über einen Auszug, nicht der selbst gewonnene heutige Auszug. **S.3 nennt Jörg Fernau / 05032/4683 / info@fernau-containerdienst.de und „weitere Standorte: keine“.** Bericht vom 21./24.11.2025, Audit 20.11.2025. Keine Stundenbestätigung.
5. [Northdata-Unternehmensprofil](https://www.northdata.de/Fernau%20Containerdienst%20GmbH,%20Neustadt%20a%C2%B7%20R%C3%BCbenberge/Amtsgericht%20Hannover%20HRB%20110122): heutiger Name, gleiche Adresse/HRB, aktuelle Struktur **ein Geschäftsführer**; HTML-LD nennt Jörg Fernau.
6. [Registerpublikation 17.04.2026](https://www.northdata.de/?id=5840317941153792), Text tatsächlich gelesen: Gesellschafterbeschluss 10.04.2026, §1 Firma/Sitz geändert, Firma nun **Fernau Containerdienst GmbH**. Damit alter Zertifikatsname und heutiger Betreiber nicht allein durch Ähnlichkeit verbunden, sondern Registerkennung und konkrete Umfirmierung belegt.
7. [Publikation 29.01.2026](https://www.northdata.de/?id=6125770698915840), Text gelesen: **Tim Fernau nicht mehr Geschäftsführer**. Der Auditbericht nennt ihn noch als seit 01.04.2025 hinzugekommen; das ist zeitlich erklärbar. Klaus Dietrich Urban im heute abgerufenen Impressum passt dagegen nicht zum aktuellen Ein-Personen-Strukturstand. Diesen Widerspruch nicht als vollständig kongruentes Impressum darstellen.

**Gelöst:** nicht Nienburg; konkrete Firma-/Namenskette, Standort, tatsächliche aktuelle Zertifikatsgültigkeit; Telefon/Mail zusätzlich in fachlich unabhängig verfasstem Audit genannt. **Nicht gelöst:** positiver aktueller Register-Live-Status und aktuelle Impressums-Geschäftsführung; keine unabhängige qualifizierte Stundenquelle. Der ausgelieferte Registerspiegel ist erheblich stärker als ein bloßer Namensmatch, ersetzt aber diese ausdrücklich offenen Punkte nicht.

**Konkreter Schließungsweg:** aktueller Registerauszug HRB 110122 Hannover mit Fortbestand/Vertretung; Betreiberklärung zum abweichenden Geschäftsführer im Impressum. Bei kongruenter Betriebsgesellschaft und aktuellem Zertifikat können dann **genau** die zwei Kontakte und Footerzeiten über die README-Ausnahme freigegeben werden. Für `phone`/`email` wäre alternativ ein aktueller qualifizierter unabhängiger Kontaktbeleg ausreichend; für `opening_hours` müsste er ausdrücklich die Neustädter Stunden enthalten. Kein Nienburger Kontakt oder alter Name zurückspielen.

## Graf / Kiel: Aktualität neu belegt, positiver Registerstatus bleibt offen

1. [Impressum](https://www.graf-entsorgung.de/info/impressum.html): Martin Graf e.K., Kolonnenweg 31 / 24113 Kiel, HRA 12865 KI, Telefon +49 431 / 52 16 18. Mail im gelieferten Body teilweise Cloudflare-geschützt.
2. [Über uns](https://www.graf-entsorgung.de/info/ueber-uns.html): derselbe lokale Kontaktblock. Geschichte seit 1954/70 Jahre ist allein kein Aktualitätsbeleg für 2026; historische Schrotthandelsnennung bleibt historisch.
3. **Neuer konkreter Aktualitätsfund:** [BFSG-Erklärung](https://www.graf-entsorgung.de/info/bfsg.html) nennt Dienstleistungserbringer Martin Graf, Kolonnenweg 31, Telefon 0431 / 52 16 18 und geschützte Mail; **erstellt 28.06.2025, zuletzt überprüft 23.07.2025**. Dies ist tatsächlicher datierter Betreiberinhalt, nicht bloß das Footerjahr 2024 und kein von uns geprüfter Nachweis materieller Barrierefreiheit.
4. [Northdata-Profil](https://www.northdata.de/Martin%20Graf%20e.K.,%20Kiel/HRA%2012865%20KI): korrekter Firmenwortlaut „GRAF Abfallbeförderung und -entsorgung, Inh. Martin Graf e.K.“, HRA 12865 KI, EUID DEX1517R.HRA12865KI, aktuelle Struktur ein Inhaber. HTML-LD identifiziert Martin Graf; kein unternehmensbezogenes `ACTIVE`-Feld gefunden.
5. [Eintragung 04.11.2025](https://www.northdata.de/?id=4802387006717952): Name, Kiel, Inhaber Martin Graf / Einzelunternehmen tatsächlich im Publikationstext gelesen. Straße ist im zugänglichen Tabellentext maskiert; sie wird dadurch **nicht** unabhängig bestätigt. Diese Bekanntmachung ist historischer Eintrag, kein aktueller Tagesauszug.
6. [Schwester-Impressum graf-entruempelung.de](https://www.graf-entruempelung.de/impressum): gleicher Inhaber, Straße, HRA, VAT DE315269007 und Telefon; **unverschleiert `info@graf-entsorgung.de`** im Impressum. Die prominente Entrümpelungs-Mail `info@graf-entruempelung.de` ist ein anderer Funktionskontakt und ersetzt den Zielwert nicht. Selber Betreiber, daher kein zweiter unabhängiger Beleg.

**Mail tatsächlich geprüft:** Cloudflare-Hexwerte `7a13141c153a1d081b1c571f140e0915081d0f141d541e1f` und `b0d9ded6dff0d7c2d1d69dd5dec4c3dfc2d7c5ded79ed4d5` wurden byteweise mit dem jeweiligen ersten Byte XOR-dekodiert; beide ergeben `info@graf-entsorgung.de`. Keine bloße Vermutung aus dem Domainnamen.

**Restgate:** Nicht mehr „keine datierte Betreiberinformation“, sondern nur noch hinreichend positiver aktueller Registerfortbestand für die Ausnahme beziehungsweise eine zweite qualifizierte Kontaktquelle. Ein aktueller Auszug **HRA 12865 KI, AG Kiel** würde den verbliebenen Registerpunkt klären; dann Telefon/Mail einzeln freigeben, keine Öffnungszeiten erfinden. Das Efb-Logo ist kein gelesener gültiger Zertifikatsnachweis. `status: pruefung` bleibt sachgerecht hinsichtlich heute angebotener Schrottannahme, unabhängig von der Kontaktklärung.

## Willeführ / Aschaffenburg: konkrete Kontakte, aber keine Ausnahmefreigabe

1. [Wix-Impressum](https://www.willefuehr-whg.de/kopie-von-kontakt): Romano A. Willeführ, Ruchelnheimstraße 3B / 63743 Aschaffenburg, HRA 3535 Aschaffenburg; **06028/21188, 0170/4765501, info@willefuehr-whg.de**. Keine Öffnungszeiten oder datierte aktuelle Gültigkeit im gelesenen Text.
2. [Kontakt/Über uns](https://www.willefuehr-whg.de/kontakt): wiederholt diese Kontakte, nennt **ROWIS Industrieservice Aschaffenburg als Partner**. Das ist ein zusätzlich zu erklärender Rollenaspekt, kein Beweis einer neuen Rechtsträgertrennung; die Gleichsetzung sämtlicher ROWIS-/Willeführ-Aktivitäten wird dadurch aber ebenfalls nicht selbstverständlich.
3. [Northdata HRA 3535](https://www.northdata.de/ROWIS%20Industrieservice%20e.%20K.,%20Aschaffenburg/HRA%203535): ROWIS Industrieservice e.K., EUID DED4102V.HRA3535, HTML-LD/Inhaber Romano Willeführ. Zugehörigkeit von Registerkennung und Person ist damit konkret belegt. Kein unternehmensbezogenes positives Aktiv-Statusfeld; sichtbare Veröffentlichungen enden bei alten Vorgängen.
4. [Bekanntmachung 04.12.2009](https://www.northdata.de/?id=6197773236174848): Kopf nennt Geschäftsanschrift **Ruchelnheimstraße 3a**, nicht 3B; Tabellendetail maskiert. Eine alte Registeranschrift macht die heutige Betreiberanschrift weder falsch noch zu einer verifizierten Umzugs-/Nachfolgekette. 3a nicht ungeprüft ins Dossier transplantieren.
5. [Zweite Betreiberdomain, Impressum](https://willefuehr-whg-maengelbeseitigung.de/impressum): Romano Willeführ/Willeführ WHG, Ruchelnheimstraße 3b, dasselbe Festnetz und Ziel-Mail, Footer dieselbe Mobilnummer. Header nennt daneben `info@willefuehr-whg-maengelbeseitigung.de`; nicht still als Ersatzmail verwenden. Footer 2018 und alte ODR-Verweise sind keine nachgewiesene aktuelle Pflege. Dieselbe Betreiberquelle, kein unabhängiger Kontaktbeleg.
6. [Zertifikate-Seite](https://www.willefuehr-whg.de/zertifikate): tatsächlich gelesenes HTML enthält sechs Zeugnis-/Zertifikatsbilder, darunter TPD und TÜV Akademie. **Keine Gültigkeitsdaten aus den Bildern verifiziert.** Originalbilder `450e22_bae482917f4048e396128ae610ae9864~mv2_d_1700_2338_s_2.jpg` und `450e22_edaefe316eee461989b73c1e2b52c84a~mv2.png` ließen sich mit dem Webtool nicht visuell darstellen; in-memory OCR scheiterte am fehlenden `tesseract`. Daher weder „gültig“ noch „abgelaufen“ behauptet. Das Vorhandensein eines Bildlinks löst das Aktualitätsgate nicht.

**Owner benötigt konkret:** aktuellen Registerstand HRA 3535 / Aschaffenburg einschließlich Inhaber/Fortbestand; Klärung der Betriebsbezeichnung Willeführ-WHG gegenüber ROWIS und des 3a-/3B-Rollenunterschieds; tatsächlich lesbaren aktuellen Zertifikats-/sonstigen datierten Betreiberbeleg. Alternativ die drei Zielkontaktbestandteile über eine zweite unabhängige zulässige Quelle absichern. Keine Vermischung mit **Alexander Willeführ/Obernburg**; kein heutiger Metallannahme-/Preisbeleg aus dessen Auftritt.

## Clear versus Keep: konkrete Folgen der Owneralternativen

Nach `docs/trader-seed.md` sind `phone`, `email` und `opening_hours` Enrichmentfelder: leer/missing bedeutet **Keep**, nicht Löschung; nichtleere Werte überschreiben. Der nachgeprüfte Snapshot hat für alle hier gesperrten Felder noch leer.

1. **Empfohlener schmaler Releasepfad bei unverändert offenen Gates:** AGOSI-Telefon/Mail übernehmen. Fernau `phone`, `email`, `opening_hours`; Graf `phone`, `email`; Willeführ `phone`, `email` vor Veröffentlichung nicht mit den gesperrten Zielwerten füllen. In der geprüften Ausgangslage bleibt dadurch leer erhalten. Das setzt eine spätere gezielte Payload-/Dossierentscheidung durch den Owner voraus; dieser Bericht verändert nichts und der unveränderte Commit 81a64832 enthält weiterhin alle neun Writes.
2. **Bewusste Keep-Alternative:** die betreffenden Enrichmentwerte leerlassen/entfernen, ohne `clear_fields`. Das bewahrt eventuell inzwischen vorhandene produktive Werte. Dann ausdrücklich nicht behaupten, ungeprüfte alte Kontakte seien entfernt. Vor Deploy den echten aktuellen Zustand noch einmal nur lesend kontrollieren; der private Snapshot ist kein Versprechen über spätere Produktion.
3. **Explizite Rücknahme schon veröffentlichter Werte:** Für Fernau wäre ein bewusstes `clear_fields: 'phone,email,opening_hours'`, für Graf/Willeführ jeweils `clear_fields: 'phone,email'` die passende begrenzte Löschungsanweisung, falls der Owner diese Felder tatsächlich dauerhaft leer halten will. Begleitende Feldwerte müssen leer sein; mit nichtleerem Wert entsteht ein Konflikt. **Nicht schon wegen eines ungelösten Recherchegates ungefragt einen persistenten Clear setzen.** Ein Clear greift auch gegen spätere Wertdrift; seine Entfernung stellt alte Werte nicht wieder her.
4. **Andere Stundenfelder:** AGOSI/Graf/Willeführ leer lassen ist nur Keep. Sollte der echte spätere Produktionsstand dort ungeprüfte Stunden enthalten, müsste der Owner für die gewünschte Entfernung `opening_hours` ausdrücklich zusätzlich clearen. Dieser Bericht empfiehlt keine pauschale Löschung belegter fremder Enrichmentarbeit.
5. Keine Straße/PLZ/GEO mitlöschen. Leere Kernadressen haben andere Seedsemantik als leere Enrichmentfelder; die Kontaktgates rechtfertigen keine Wiederherstellung der falschen Nienburger Ortszuordnung.

## Abrufgrenzen und reproduzierbare Dokumentbezüge

- PDFs ausschließlich lesend per HTTPS bezogen, im Speicher an `pdftotext -layout - -` übergeben; keine PDF-/DB-Dateien angelegt. AGOSI-PDF HTTP 200, 114309 Bytes, SHA-256 `809b1b4c20d9f131b30d276396379cbf68c0a03a25762f9f1d4e8d4d9ef68920`; Fernau-Zertifikat HTTP 200, 306795 Bytes, SHA-256 `bce75fd6f40b335f48380c7fe44c07c22a171ca934e86ab841eb9e6cd1b9c37d`; Audit HTTP 200, 579351 Bytes, SHA-256 `4f035a3041d8c8e40098659014ca645c95ff8c42b0e9b79f8fbfe7aeef6e7dad`.
- Graf `/info/kontakt.html`: HTTP 403; die erfolgreichen Impressums-/BFSG-/Über-uns-Abrufe bleiben davon unberührt. Fernau `/kontakt`: HTTP 404; Zeiten/Kontakte wurden aus den erfolgreichen Seiten gelesen. Keine Schließung daraus abgeleitet.
- Die tatsächlich entdeckten Creditreform-Einzelseiten zu [Graf](https://firmeneintrag.creditreform.de/24113/2230267925/GRAF_ABFALLBEFOERDERUNG_UND_ENTSORGUNG_INH_MARTIN_GRAF_E_K), [Fernau](https://firmeneintrag.creditreform.de/31535/2190086887/FERNAU_CONTAINERDIENST_GMBH) und [ROWIS](https://firmeneintrag.creditreform.de/63743/8010092497/ROWIS_INDUSTRIESERVICE_E_K) lieferten jeweils **403**. Suchsnippets nicht als gelesene aktuelle Drittbelege gezählt.
- Direktabruf `https://www.handelsregister.de/rp_web/erweitertesuche.xhtml`: HTTP 400. Unternehmensregister-Startseite erreichbar, aber **kein unternehmensbezogener aktueller Auszug gewonnen**. Nicht behaupten, das gesamte Register sei unzugänglich oder es gebe dort keinen Eintrag.
- Graf-Facebook-Abruf lieferte keinen nutzbaren Body. Suchtreffer/Profile von Facebook/Instagram, allgemeine Branchenportale und erfolglose gezielte Kommunalsuchen sind keine ersatzweise Feldfreigabe. Kein kommunaler Gewerberegisterbeleg für die offenen Kontakte gewonnen.
- Keine Kontaktaufnahme, kein Testanruf, keine Testmail. Keine telefonische Erreichbarkeit oder Antwortfähigkeit praktisch geprüft. Keine Zahlentabelle/Preisrichtung neu geprüft; keine Umsatz-, Aktivitäts- oder Pin-Gesamtfreigabe.
