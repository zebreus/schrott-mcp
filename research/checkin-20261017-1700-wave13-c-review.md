# Owner-Vorprüfung: Rest von Welle C vom 13.10.

17.10.2026. Vollständigen `wave-20261013-c.md` und die tatsächlichen13 noch offenen Dossierdiffs gelesen. Diese Vorprüfung verändert **keine** geschützte Dossierdatei und ist keine pauschale Veröffentlichungsfreigabe der Welle.

## Bereits integriert: sieben Fälle nicht erneut übernehmen

Alle sieben Dateien sind aktuell sauber; ihre Rechercheabschnitte vom13.10. stehen bereits in `HEAD`:

| Dossier | Letzter relevanter Commit |
|---|---|
| Karlsruhe/Pedack | `86fd612f` |
| Neu-Ulm/Russ | `6cf026ba` |
| Bad Salzuflen/Hilgenberg | `733d9aba` |
| Bochum/Fix | `36463aee` |
| Gütersloh/GMG | `733d9aba` |
| Hamm/Hark | `4bc166df` |
| Kreuztal/Mario Deutsch | `4bc166df` |

Nachweis mit tatsächlichen HEAD-Inhalten und Gitstatus: `/tmp/opencode/checkin-20261017-1700-wave13c-committed.json`. Der alte Wellenbericht ist kein Beweis, dass alle20 noch offen sind. Insbesondere Russ' alte Warnung zur Landeskennung steht als Historie in der Timeline; spätere bestätigte Korrektur nicht rückgängig machen.

## Noch offene13: nur Überblick/Timeline, keine Frontmatteränderung

| Dossier | Entscheidung zum gelesenen Diff |
|---|---|
| Billigheim/Wagner | Als begrenzte Notiz empfohlen; tatsächliche Containerseite erneut gelesen, Kundenzuordnung bleibt ausdrücklich offen. |
| Karlsruhe/MS Brenner | Notiz empfohlen; bestehender Adress-/Telefonkonflikt erneut direkt bestätigt, keine Kontaktsubstitution. |
| Villingen-Schwenningen/Renz | Notiz empfohlen; Größen/Gutschrift direkt bestätigt, keine Gratisentsorgung daraus ableiten. |
| Willstätt/Lehmann | Nur als offener Verzeichnislead empfohlen, kein Betriebsnachweis. |
| Bergisch Gladbach/Blum | Nur als offener lokaler Lead empfohlen, keine Zuordnung zu Niederzier. |
| Bochum/Eco | Nur als ungeklärte Domainattribution empfohlen, keine Bochumer Betreiberidentität. |
| Coesfeld/Eddy | Nur Lead-/Redirectabgrenzung empfohlen; ASH ist keine bestätigte Filiale. |
| Essen/Missal | **Zurückhalten bis Formulierung korrigiert**, siehe unten. |
| Gummersbach/MAR | Nur Portal-/Templatewarnung empfohlen; keine bestätigten Mindestmengen. |
| Hagen/Mock | Nur Verzeichnis-/Namensvetterabgrenzung empfohlen. |
| Hürth/Hartl | Nur Verzeichnislead empfohlen; keine bestätigte Annahme. |
| Mönchengladbach/Müller | Nur expliziter Personen-/Telefonkonflikt empfohlen; keine Verschmelzung. |
| Wuppertal/Halan | Nur Portal-/Zeitwiderspruch empfohlen, keine bestätigten Konditionen. |

Die übrigen Verzeichnisangaben werden hier nicht durch Mehrfachnennung zu unabhängigen Primärbelegen. Keine aktuelle positive Betreiber-/Adress-/Servicefreigabe aus den zwölf Notizempfehlungen. Die datierten13.10.-Befunde bleiben historisch; diese Vorprüfung behauptet keine erneute Liveprüfung jedes damaligen Suchtreffers.

## Tatsächliche Primärabrufe am17.10.

- [Wagner Container](https://www.schrott-wagner.de/container.html): Anlieferungen Wertstoffhof Mo–Fr09–16, Container10–40 bzw.5–12m³. Die Seite nennt **beide** Kontakt-/Lageradressen Fasanenweg2 und Bahnhofstraße8; ordnet den Wertstoffhof aber nicht ausdrücklich eindeutig zu. „Auf der Seite fehlen beide Anschriften“ wäre falsch; die vorhandene neue Notiz spricht begrenzt von fehlender eindeutiger Hofzuordnung. Kein Lorenz→Annegret-Kettenbeweis.
- [MS Brenner Kontakt](https://www.ms-brenner.de/kontakt.html) und [Impressum](https://www.ms-brenner.de/impressum.html): Kontakt Unterer Dammweg6, Impressum Unterfeldstraße30; Mobilnummer im Kopf/Fuß und Festnetz im Inhalt nebeneinander. Die Website selbst enthält den Konflikt; keine eigenständigen Quellen aus zwei Unterseiten machen.
- [Renz Container](http://renzcontainer.de/):6/7/10/17/36m³, ausdrückliche Gutschrift für wertvolles Altmetall nach Verkauf. Kein Preisblatt oder pauschal kostenfreie Containerleistung. Nicht als unabhängige zweite Identitätsquelle zu Renz Rohstoffe werten.
- [Missal Impressum](https://www.schrotthandel-missal-essen.de/impressum/) und [Abholung](https://www.schrotthandel-missal-essen.de/schrott-metallabholung/): Jürgen Missal/Kalthofweg4/45277Essen, genannte Kontakte und mobile Abholung im100-km-Umkreis. Die Leistungsseite nennt sogar „Hier finden Sie uns“ sowie Bürozeiten an dieser Adresse. Das beweist **weder** öffentliche Schrottannahme **noch deren Ausschluss**.

### Konkreter Missal-Befund

Der neue Überblick sagt „keine Kundenannahme am Hof“. Die gelesene Quelle trägt nur **„keine öffentliche Kunden-/Hofannahme belegt“**. Vor Integration diese Negativbehauptung eng korrigieren, ohne die historische Recherche zu löschen oder die Sitzadresse als Annahmehof freizugeben. Frontmatter bleibt bis Identitäts-/Zweitquellengate unverändert. Geschützte Quelldatei in dieser Prüfung nicht editiert.

## Nächster Integrationsschritt

Vor jeder Übernahme zwölf Empfehlungen nochmals an gebundene Dateihashes koppeln, Missal-Korrektur gesondert vornehmen und Seed-/Codegenwirkung im sauberen Export prüfen. Kein Dossier dieser Restgruppe jetzt gestaged, committed oder deployt. Parallel laufen unabhängig die Reviews der WellenA/B; deren Zielpfade wurden nicht bearbeitet.
