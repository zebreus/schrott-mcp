# SAXONIA-Fix: Review vom 15.10.2026

## Disposition

**Migrationsfix fachlich richtig; eine verbleibende Fail-closed-Lücke am Ortsvergleich vor Freigabe schließen.** Zusätzlich die neue Kursfixture quellennäher halten. Keine Tests, Builds, Deployments oder DB-Schreibzugriffe durchgeführt; die nachstehenden Mutationen sind statisch aus dem Kontrollfluss abgeleitet, nicht als ausgeführte Tests ausgewiesen.

Geprüft: uncommitteter Diff und vollständiger Handler `crates/ingestion/src/traders/handlers/saxonia.rs`, beide neuen Fixtures sowie `research/checkin-20261015-1300-saxonia-source.md`. Unabhängige öffentliche GETs auf Kursseite und Impressum am **15.10.2026, 13:06:18 UTC** lieferten jeweils HTTP 200; ausgewertet wurde das tatsächliche Roh-HTML, ohne JavaScript-Ausführung oder Formularaufrufe. Ausschließlich diese Reviewdatei geschrieben.

## Bestätigte Anforderungen

- **URL:** `https://saxonia.de/tageskursanzeiger/` ist im Handler eingetragen und liefert aktuell die in der Recherche dokumentierten Daten.
- **Betreiber:** Das aktuelle `<main>` des Impressums enthält SAXONIA Edelmetalle GmbH, Erzstraße 9, 09633 Halsbrücke/Sachsen, Registergericht Chemnitz und HRB 31481. Der neue Selektor erfasst diesen Bereich, ohne auf Navigation/Footer auszuweichen. Die wortweise HRB-Prüfung weist insbesondere `HRB 314819` zurück.
- **Keine erfundenen Kontakte:** Im aktuellen Impressumsinhalt stehen weder Telefon noch E-Mail. `unwrap_or_default()` liefert dafür leere Felder; es werden weder Altwerte noch der separate Vertriebskontakt aus der Recherche eingesetzt. Das ist für diesen Quellstand richtig. Persistenz-/Mergeverhalten leerer Felder liegt außerhalb dieser drei geprüften Dateien.
- **Vier explizite Ankauf-Richtwerte:** Aktuelles Roh-HTML und Fixture enthalten Gold 117611.5, Silber 1595.63, Platin 44731.94 und Palladium 29786.96, jeweils ausdrücklich `Ankauf` und `€/kg`. Division durch 1000 ergibt korrekt **117.6115 / 1.59563 / 44.73194 / 29.78696 EUR/g**. `price_kind = "approx"` stimmt mit dem unverbindlichen Spot-Richtwert-Hinweis überein; `confidence = 0.8` ist eine interne Bewertung, keine Quellenangabe.
- **Datum:** Das explizite Kursdatum ist 15.10.2026. Der Handler nimmt es aus der Tageskursüberschrift, nicht aus Chart, HTTP-Zeit oder Sitemap. Die Ausgabe als UTC-Mitternacht ist eine technische Datumskodierung, kein durch die Quelle bestätigter Veröffentlichungszeitpunkt.
- **Richtung/Struktur:** Sechs Nichtankauf-Buttons werden ausgeschlossen. Der Widerspruch beim Palladium-Nichtankauf-Chart wird nicht als Ankauf interpretiert. Fehlendes/ungültiges Datum, unvollständige vier Metallzuordnungen, falsche Ankaufseinheit, ungültige/nichtpositive Zahlen und widersprüchliche doppelte Ankaufwerte führen laut Kontrollfluss zu Fehlern. Identische responsive Duplikate bleiben absichtlich erlaubt.
- **Versteckter Störungstext:** Die aktuelle Quelle versteckt ihn weiterhin mit allen drei `elementor-hidden-*`-Klassen. Ihn nicht pauschal als Ausfallsperre zu verwenden, entspricht dem Recherchebefund.

## Konkrete Befunde

### R1 — Mittel: Ortsänderungen mit unverändertem Präfix passieren den Identitätsguard

**Ort:** `saxonia.rs:65–69`, ergänzende Regressionstests bei `:222–229`.

`rest.starts_with("Halsbrücke/Sachsen")` prüft keine vollständige Ortsangabe. Wird in der neuen Impressumsfixture `Halsbrücke/Sachsen` etwa durch `Halsbrücke/Sachsenhausen` ersetzt, bleiben Firma, Straße und HRB gültig, und der Handler akzeptiert die geänderte Ortsangabe. Ausgegeben wird trotzdem fest `city: "Halsbrücke"`. Damit ist die Anforderung „fail closed bei geänderter Identität“ nicht vollständig erfüllt.

**Einordnung:** Die Präfixprüfung bestand bereits vor diesem Diff; der Migrationsfix verursacht sie nicht neu. Sie bleibt aber im ausdrücklich mitgeprüften Identitätsguard und wird durch die drei neuen Negativmutationen nicht abgedeckt.

**Empfehlung:** Die vollständige normalisierte Ortsangabe aus dem Adressabsatz prüfen oder wenigstens eine echte Grenze nach dem erwarteten Ortsnamen verlangen. Negative Tests für veränderten Ort einschließlich einer Präfixverlängerung ergänzen. Nicht den gesamten nachfolgenden Impressumstext auf Gleichheit prüfen.

**Disposition:** Vor Freigabe korrigieren oder die verbleibende Lücke ausdrücklich akzeptieren; die Behauptung eines vollständigen Identitäts-Fail-closed sonst einschränken.

### R2 — Niedrig: Kursfixture verändert relevante Überschriftsklassen gegenüber der Rohquelle

**Ort:** `fixtures/saxonia-20261015.html:10,14,17`.

Die Datei bezeichnet sich als HTML-Auszug, verändert jedoch Klassen an vom Parser selektierten Elementen:

| Metall | Rohquelle 13:06:18 UTC | Neue Fixture |
| --- | --- | --- |
| Silber | `chart-metall2` | `chart-metall` |
| Platin | `chart-metall` | `chart-metall2` |
| Palladium | `chart-metall` | `chart-metall2` |

Der aktuelle Parser akzeptiert beide Klassen, daher **kein aktueller Preisfehler**. Die Fixture prüft aber nicht exakt die beobachtete Klassenzuordnung und könnte spätere Selektorregressionen kaschieren. Das Entfernen irrelevanter Wrapper, Styles und Chartskripte ist dagegen transparent dokumentiert und unproblematisch. Der rechtliche Fixtureauszug bewahrt die benötigten Angaben; die Aufteilung der Geschäftsführer auf separate Absätze ist für den jetzigen Parser ohne Wirkung.

**Empfehlung/Disposition:** Die drei Klassen auf den beobachteten Rohquellenstand berichtigen; kein eigenständiger fachlicher Freigabeblocker.

## Grenzen und vorgeschlagener Abschlussnachweis

Keine darüber hinausgehenden dokumentierten Standardverletzungen oder fachlichen Abweichungen im Diff festgestellt. Nicht behauptet wird, dass beliebige zukünftige HTML-/Preisstrukturänderungen erkannt werden: unbekannte Zusatzmetalle werden bewusst übersprungen, und der Parser ist kein allgemeiner Sichtbarkeits-/CSS-Prüfer.

Der Hauptagent sollte die vorhandenen Handler-Tests, eine Ortsmutationsregression nach R1 und den vorgesehenen Livecheck ausführen. Beim Livecheck insbesondere vier `approx`-Werte, EUR/g, Kursdatum, aktuelle Betreiberanschrift und tatsächlich leere Kontaktfelder prüfen. Dieses Review ersetzt diese Ausführungsnachweise nicht.

## Direktquellen

- <https://saxonia.de/tageskursanzeiger/> — HTTP 200, Roh-HTML, 15.10.2026 13:06:18 UTC.
- <https://saxonia.de/impressum/> — HTTP 200, Roh-HTML, 15.10.2026 13:06:18 UTC.
- `research/checkin-20261015-1300-saxonia-source.md` — insbesondere Abschnitte Identität, Kurse und Empfehlungen.

## Nachreview: R1 und R2 (15.10.2026)

**Aktuelle Disposition: Beide Befunde behoben; aus R1/R2 verbleibt kein Freigabeblocker.** Diese Bewertung ersetzt die ursprünglichen Dispositionen oben und beschränkt sich auf die beiden Korrekturen.

- **R1 — geschlossen:** `saxonia.rs:68–71` vergleicht jetzt das vollständige erste Ortstoken mit `Halsbrücke/Sachsen` statt nur dessen Präfix. Dadurch werden insbesondere `Halsbrücke/Sachsenhausen` und `Halsbrücke/Sachsen-Nord` zurückgewiesen. Die neue Regression `changed_city_including_matching_prefix_is_rejected` (`:205–214`) deckt beide Präfixverlängerungen sowie `Dresden/Sachsen` ab. Die Änderung erfüllt die im Erstreview empfohlene Grenzprüfung, ohne den nachfolgenden Impressumstext auf Gleichheit festzulegen.
- **R2 — geschlossen:** `fixtures/saxonia-20261015.html:10,14,17` enthält nun für Silber `chart-metall2`, für Platin und Palladium jeweils `chart-metall`. Alle drei Klassen entsprechen damit dem im Erstreview direkt erhobenen Rohquellenstand.

**Evidenzgrenze:** Für dieses begrenzte Nachreview ausschließlich aktuellen Code, Kursfixture und bestehenden Bericht gelesen; keine Tests oder erneuten HTTP-Abrufe durchgeführt. Der Hauptagent meldet eine tatsächlich zunächst rote R1-Regression, anschließend 7 erfolgreiche SAXONIA-Tests, 14 erfolgreiche Seed-Tests, 726 Workspace-Tests bei 0 Fehlern und 7 ignorierten Tests sowie einen erfolgreichen HTTP-200-Livecheck mit vier Preisen. Diese Ausführungsergebnisse sind übernommene Angaben des Hauptagenten, nicht unabhängig in diesem Nachreview reproduziert. Ausschließlich diesen Bericht ergänzt.
