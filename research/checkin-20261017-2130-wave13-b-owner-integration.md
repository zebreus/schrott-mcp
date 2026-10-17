# Alte Welle B: getrenntes Owner-Integrationsgate

17.10.2026, Fortsetzung des 21:30-Check-ins. Der [Korrekturhandoff](checkin-20261017-2130-wave13-b-corrections.md) ist vollständig gelesen. Genau 19 Dossiers plus alter Wellenbericht sind der Kandidatenumfang, keine pauschale Freigabe des Arbeitsbaums. Unabhängiges Endreview und isolierter Seedtest wurden getrennt gestartet. Dieser Stand enthält noch keine Endentscheidung oder Produktionsveröffentlichung.

## Tatsächlicher read-only Produktionsabgleich

Alle 19 Slugs vorhanden. Produktionszugriff ausschließlich SQLite `mode=ro` und `query_only`; keine GEO-Abfrage oder Mutation. Inventar `/tmp/opencode/checkin-20261017-2130-b-production-gate.json`; textuelle Scalarvergleiche vermeiden einen falschen Schleicher-PLZ-Diff durch YAML-Integerdarstellung.

Die beabsichtigten Kern-/Kontaktänderungen gegenüber den aktuell gespeicherten Werten sind konkret:

- Darmstadt: Mainzer Str. 83 und PLZ 64293 würden als Entzug unqualifizierter Adresskuratierung geleert. Kein Beweis, dass dort kein Betrieb existiert.
- Schrottprofis: gespeicherte Nummer 01633318898 würde ausschließlich durch das explizite `clear_fields: phone` zurückgezogen. Die Ownerentscheidung gilt genau diesem Feld und nicht allen leeren Kontakten.
- Reining: verunreinigtes Straßenfeld `Schaafheim , Karlstr. 1` und PLZ 64850 würden geleert, Ort `Schaafheim 64850 HE` zu `Schaafheim` normalisiert. Kein Ringstraße-Fill und keine abgeschlossene Nachfolgekette.
- Bachmann: Rechtsträgername präzisiert, Telefon/Website neu; Adresse unverändert. E-Mail und Aktivstatus nicht neu gefüllt.
- Blum: Julias Container/Marco Blum, Containerdienst, normalisierte Geschäftsadresse/Ort und PLZ 35435, Telefon/Website. Kein Annahmehof oder Schrottankauf dadurch bestätigt.
- Collecta: KG statt des fehlerhaften GmbH-Seeds, Wertstoffhandel, Büro Senefelderstraße 3A/65205 Wiesbaden, Telefon/Website. Keine Europe-Vermischung oder erfundene Umwandlungskette.

Kundrats Schreberstr. 38a bleibt identisch zur Produktion. Die alten gespeicherten Telefonnummern bei Schindling `(069) 39090562` und Stein `(06157) 9110132` unterscheiden sich zwar von leeren Seedfeldern, werden aber **ohne Clear weiter bewahrt**. Keine nachträgliche Behauptung ihrer Entfernung oder qualifizierten Bestätigung. Unveränderte Service-Enrichmentfelder ebenfalls kein automatischer Rückzug.

`extra_json.seed_file: he` ist noch bei Metzger, Kundrat, Reining, Collecta und Cumic vorhanden. Eine spätere Seedänderung kann diese fünf historischen Metadaten entfernen; noch kein tatsächlich angewandter Seedlauf oder vollständiger Produktionsdelta-Nachweis daraus. Koordinaten würden ohne neues Paar erhalten; hier keine GEO-Bewertung.

## Ausstehende Entscheidung

Die Felddifferenzen sind ein Inventar, **keine Seed-Anwendungssimulation und keine Quellenfreigabe**. Telefon-/Domain-Fills aus den begrenzten Creditreform-Suchindexbelegen bleiben insbesondere Gegenstand der unabhängigen Endprüfung. Erst nach deren konkreten Dispositionen und bestandenem hashgebundenem Seedtest scoped integrieren; GitHub-Push und koordinierter späterer Rollout bleiben getrennte Nachweise.

## Isolierter Seedtest abgeschlossen

Frischer Export von `5ed20b3988f4b945cd39171f0abac7ca1aee859c` mit ausschließlich den 19 B-Dossiers aus der hashgeprüften Handoff-Tabelle: `/root/Documents/checkin-20261017-2130-b-seed`. Keine anderen offenen Änderungen übernommen. `cargo +stable test -p schrott-mcp-ingestion seed`: **14 bestanden, 0 fehlgeschlagen**, Exit 0. Vollständiges Log selbst gelesen: `/root/Documents/checkin-20261017-2130-b-seed.log`, SHA-256 `7c7ec9f421099552f4e756767879d70b5460a9a16bff66865b3adbec5104cee7`. Inputhashes in `scoped-inputs.json` und `/root/Documents/checkin-20261017-2130-b-seed-result.json`.

Das technische Gate ist damit für genau diesen Dateistand erfüllt; die unabhängige fachliche Endprüfung bleibt ausstehend. Keine Integration oder Veröffentlichung allein aufgrund grüner Tests.

## Endabnahme und implementierte Feldrückstellungen

Die [unabhängige Endabnahme](checkin-20261017-2130-wave13-b-owner-final.md) ist inzwischen vollständig eingegangen und vom Owner vollständig gelesen. **Keine unveränderte Gesamtfreigabe:** Creditreform-Suchindex/403 trägt die neuen positiven Kontakt-/Domainfills nicht hinreichend; Blums Identitätspaket zusätzlich nicht abgeschlossen. Owner übernimmt diese konkreten Haltepunkte statt sie mit früherer Teilfreigabe zu überstimmen.

Vor Anpassung drei Dossiers und Wellenbericht vollständig extern gesichert: `/root/Documents/wave13b-owner-field-hold-20261017`, Manifest mit Vorherhashes. Per patch danach:

- Bachmann und Collecta: phone, website, website_status auf dem bisherigen leeren Seedstand bewahrt. Belegte Namens-/Identitätspräzisierung und bei Collecta Büro-/Typ-/Adresskorrektur bleiben. Keine neuen Mail-/Aktivitäts-/Description-Fills.
- Blum: vollständiges Frontmatter sowie H1 auf bisherigem Seedstand bewahrt; damit keine neue Julias-Identitäts-, Typ-, PLZ-, Adress-, Telefon- oder Websiteübernahme. Recherche-/Gebührenbefunde nur dem noch nicht abschließend zugeordneten Kandidaten zugeschrieben.
- Alle drei: neuer datierter Endabnahmehinweis vor bisherigen Timelineblöcken; gesamte zuvor geprüfte Timeline bleibt bytegleich als Suffix erhalten. Overview und Wellenbericht auf gleichen eingeschränkten Stand gebracht. Korrekturhandoff als historischer Zwischenstand mit abweichenden Endhashes ausdrücklich qualifiziert.

Owner hat den vollständigen tatsächlichen Vier-Dateien-Diff gegen diese zusätzliche Sicherung gelesen und separat maschinell geprüft: Blum-Frontmatter exakt HEAD-gleich; bei Bachmann/Collecta die drei gehaltenen Felder exakt HEAD-gleich/leer; kein neuer Emdash-Feldwert. Die übrigen 16 Dossiers bleiben unverändert zum unabhängigen Endreview. Audit mit Vorher-/Nachherhashes `/tmp/opencode/checkin-20261017-2130-b-final-hold-audit.json`. Keine neue Webrecherche oder Produktionsmutation durch diese Disposition.

Das frühere Produktionsinventar oben ist entsprechend enger zu lesen: dort als neue Telefon-/Domainfills und Blum-Änderungen gelistete Differenzen sind im endgültigen Kandidaten **nicht mehr enthalten**. Nur belegte Bachmann-Namenspräzisierung, Collecta-Kernkorrektur, Darmstadt-/Reining-Withdrawal und expliziter Schrottprofis-Clear bleiben. Entfernung genau der fünf dort benannten historischen `seed_file`-Metadaten beim späteren normalen Seed-Upsert wird als begrenzter technischer Nebeneffekt akzeptiert, nicht manuell ausgeführt. Keine pauschale Löschung alter Enrichment-/Service-/GEO-Werte.

Der ursprüngliche grüne Test gilt nicht als Test dieses geänderten Standes. Ein neuer isolierter Testexport `checkin-20261017-2130-b-seed-final-hold` wurde dafür gesondert gestartet; dessen Ergebnis bleibt vor Commit abzuwarten und zu lesen. Kein Deployment und keine Remote-Veröffentlichung vorweggenommen.

## Abschließende lokale Integrationsentscheidung

Erneuter Test abgeschlossen und vollständiges Log vom Owner gelesen: **14 bestanden, 0 fehlgeschlagen**, Exit 0, für genau den feldbeschränkten Endstand. Export `/root/Documents/checkin-20261017-2130-b-seed-final-hold`, Basis `5ed20b3988f4b945cd39171f0abac7ca1aee859c` plus exakt 19 Overlays. Log-SHA-256 `650ae5a075d699302e03b2ec3db9073325b97cdfa66df72c068d352ef1202982`; vollständige 19 Endhashes in dessen `scoped-inputs.json` und externem `-result.json`.

Finaler Owner-Abgleich gegen HEAD: nur noch **fünf** Frontmatterfälle. Darmstadt street/postcode leer; Schrottprofis phone leer/clear_fields phone; Bachmann ausschließlich name; Reining city/street/postcode; Collecta name/type/city/street/postcode. Alle anderen 14 einschließlich Blum vollständig Frontmatter-gleich zu HEAD. Alle bisherigen HEAD-Timelineblöcke als vollständiger unveränderter Block erhalten; gegenüber den ursprünglichen Korrektursicherungen gilt die im unabhängigen Review geprüfte Historienbindung. Ein vollständiger HEAD-Block ist wegen danach angehängter 13.10.-Recherche nicht zwingend Dateisuffix; kein Historienverlust daraus ableiten. Audit `/tmp/opencode/checkin-20261017-2130-b-final-fields.json`.

Alle 100 nicht zur Ownerkorrektur zugelassenen Dateien der ursprünglichen Baseline frisch hashgleich. Dossierhashes vor Staging nochmals gegen den bestandenen Endtest zu binden. Owner erteilt die lokale scoped Integrationsfreigabe für **19 Dossiers und vier Berichte** (alter Wellenbericht, Korrekturhandoff, unabhängige Endabnahme, dieses Integrationsgate). Keine weiteren offenen Dateien, keine neuen Kontakt-/Domainfills oder Blum-Identitätsübertragung, keine pauschale Quellenfreigabe. Push bleibt bis nachgewiesener Authentifizierung/erfolgreicher Remote-Veröffentlichung offen; kein neuer Rollout in diesem Schritt.
