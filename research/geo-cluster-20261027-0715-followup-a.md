# Owner-Nachprüfung: Essener GEO-Cluster, 27.10.2026, 07:15

## Ergebnis und Maßstab

Geprüft wurden ausschließlich die sieben offenen Dossiers unten. Der Ausgangspunkt `51.450284375, 7.015053125` stammt aus dem read-only-Abgleich im [Vorbericht](geo-cluster-20261027-0700-a.md); er wurde in dieser Nachprüfung nicht erneut aus der Produktionsdatenbank gelesen. Die direkte [Nominatim-Rückwärtssuche](https://nominatim.openstreetmap.org/reverse?format=jsonv2&lat=51.450284375&lon=7.015053125&zoom=18&addressdetails=1) vom 27.10.2026 löst ihn auf die Straße Freiheit, 45128 Essen, nicht auf einen Händler. Das ist ein Clusterhinweis, kein händlerspezifischer Beweis.

**Entscheidung:** Für keinen der sieben Fälle verbindet die aktuelle Primär-/Betreiberevidenz Händleridentität und konkreten tatsächlichen Betriebs- oder Annahmeort belastbar genug, um den Punkt als falsch für genau diese Dossierzeile zu belegen. Alle sieben Dossiers bleiben unverändert; es wurde kein `clear_fields` ergänzt und keine Ersatzkoordinate abgeleitet.

## Einzelfälle

### `nw-essen-bogner-bogner`: offen, unverändert

Die Dossierhistorie hat Vondernstr. 1 nur als alten Verzeichnislead; der 11880-Eintrag ist mit Stammdaten vom 30.12.2022 versehen. Der [RWE-Beitrag](https://www.rot-weiss-essen.de/2023/03/29/bogner-und-bogner-gibt-rwe-rueckenwind/) bezeichnet Bogner & Bogner als Mülheimer Unternehmen, stellt aber keine Verbindung zur Essener GbR oder deren Adresse her. Damit ist weder die Identität der Essener Zeile noch ein tatsächlicher Annahmeort belegt; die Namens-/Ortsähnlichkeit allein trägt keinen Clear. [Dossier](../dossiers/nw/nw-essen-bogner-bogner.md) · [11880-Lead](https://www.11880.com/branchenbuch/essen-ruhr/060371058B53400069/bogner-bogner-gbr-schrotthandel-und-containerdienst.html).

### `nw-essen-cbr-bracke`: offen, unverändert

Die Dossierhistorie dokumentiert die Löschung der CBR Bracke GmbH, HRB 13634, im Jahr 2024 und eine frühere Essener Registereintragung; beides belegt weder Manderscheidtstr. 91 als deren konkreten Standort noch eine heutige Annahmestelle oder Nachfolge. Der aktuelle [city-map-Eintrag](https://essen.city-map.de/de/cbr-bracke-gmbh) nennt zwar diese Adresse, ist aber ein Verzeichnisprofil ohne Primär- oder Behördenbestätigung. [Dossier](../dossiers/nw/nw-essen-cbr-bracke.md) · [Registerspiegel](https://www.northdata.de/CBR%20Bracke%20GmbH,%20Bochum/Amtsgericht%20Essen%20HRB%2013634).

### `nw-essen-eie`: offen, unverändert

DasTelefonbuch, Gelbe Seiten, Das Örtliche und weitere Verzeichnisse führen EIE Services GmbH an der Gelsenkirchener Str. 181. Bei der Nachprüfung ließ sich dazu weiterhin kein Betreiber-Impressum, Registerbeleg oder Behördennachweis finden; `deinschrottplatz.de` lieferte keinen abrufbaren EIE-Betreiberauftritt. Die übereinstimmenden Verzeichnisangaben sind keine Bestätigung einer konkreten Annahmestelle. [Dossier](../dossiers/nw/nw-essen-eie.md) · [Das Örtliche](https://www.dasoertliche.de/Themen/EIE-Services-GmbH-Essen-Schonnebeck-Gelsenkirchener-Str) · [Gelbe Seiten](https://www.gelbeseiten.de/gsbiz/d36b4c9c-0892-460b-92c0-cb25bda29af8).

### `nw-essen-hade-schrott-metallgrohandel-stamm`: offen, unverändert

Die Dossierhistorie lässt offen, ob die Seed-Adresse Heßlerstr. 77 je zum Essener Häde-Betrieb gehörte oder eine Fehlzuordnung ist. Die aktuelle [Metallion-Betreiberseite](https://metallion.de/) identifiziert dort Metallion und dessen Schrottannahme, nicht Häde. Die ebenfalls geprüfte [Häde-Betreiberseite](https://www.haede-recycling.de/) und ihr [Impressum](https://www.haede-recycling.de/impressum) nennen dagegen Häde Recycling GmbH, René Häde und Alte Bottroper Straße 120, Essen, sowie Schrott-/Metallleistungen. Das ist ein relevanter neuer Betreiberlead, aber die Seite enthält Platzhalter statt einer Handelsregisternummer und stellt keine belegte Kette zur im Dossier genannten Roswitha-Häde-/Heßlerstr.-Spur her; eine eindeutige Annahmestelle dieser Seed-Zeile ist somit nicht nachgewiesen. [Dossier](../dossiers/nw/nw-essen-hade-schrott-metallgrohandel-stamm.md).

### `nw-essen-kollner`: offen, unverändert

Der einzige konkrete Kandidat bleibt Winfried Köllner, Grandhöhe 20, aus einem 11880-Eintrag mit Eintragsdaten vom 29.06.2021. Das Portal-Update 2026 ist kein frischer Betreiberbeleg; weder Gewerbeidentität noch Fortbestand oder Hofannahme sind durch Primär-/Behördenquellen bestätigt. Andere namensähnliche Treffer lösen die Zuordnung nicht. [Dossier](../dossiers/nw/nw-essen-kollner.md) · [11880-Lead](https://www.11880.com/branchenbuch/essen-ruhr/120674719B26210361/winfried-koellner.html).

### `nw-essen-l-g-r`: offen, unverändert

Die Betreiberseiten der [L.G.R. Lothar Gedenk Recycling GmbH](https://lgr-essen.de/impressum/) nennen Karl-Legien-Straße 10, Essen, HRB 12622; die [Dienstleistungsseite](https://lgr-essen.de/dienstleistungen/) bestätigt die direkte Annahme und Verwiegung von Altkleidern und Schuhen im Essener Betrieb. Das belegt eine tatsächliche Annahmestelle der Textilrecyclingfirma, nicht jedoch Schrott-/Metallannahme oder sicher, dass diese Firma die als `schrotthaendler` importierte Dossierzeile ist. Wegen dieses ungelösten Identitäts-/Branchenwiderspruchs wird der Standort nicht auf den Seed übertragen und der gemeinsame Punkt nicht händlerspezifisch als falsch gewertet. [Dossier](../dossiers/nw/nw-essen-l-g-r.md).

### `nw-essen-meinrich`: offen, unverändert

NorthData und Creditreform verbinden die Josef Meinrich GmbH, HRB 12559, mit Keunefeld 9; der Register-/Firmengegenstand allein belegt aber keine gegenwärtige Schrottannahme dort. Die als Website verlinkte [Betreiberseite samt Impressum](https://www.meinrich.de/impressum/) gehört zu „Umzüge Meinrich“, Inhaber Sven Borg, HRA 5769, Wolfsbankring 40, nicht nachweislich zur GmbH HRB 12559. Es fehlt damit sowohl eine sichere Betreiberbrücke als auch ein bestätigter heutiger Schrott-Betriebs-/Annahmeort. [Dossier](../dossiers/nw/nw-essen-meinrich.md) · [NorthData](https://www.northdata.de/Josef%20Meinrich%20GmbH,%20Essen/HRB%2012559) · [Creditreform](https://firmeneintrag.creditreform.de/45355/5110173346/JOSEF_MEINRICH_GMBH).

## Grenzen und Änderungen

- Alle sieben Dossiers und ihre Timelines blieben unangetastet; kein `clear_fields`, keine Statusänderungen und keine Koordinaten ergänzt.
- In dieser Nachprüfung gab es keinen Produktionsdatenbankzugriff. Kein Deploy, Commit oder Push.
- „Offen“ bestätigt den Freiheit-Punkt nicht; es bezeichnet die fehlende händlerspezifische Beweiskette.
