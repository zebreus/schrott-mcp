# ELNO: echter Schedulerfehler vom 17.10.2026, 11:31:52 UTC

## Befund / Owner-Entscheidung

**Kein Sortimentsschwund und keine fehlende PDF-Zeile. Ein exaktes Label-Mapping ist zu eng:** Die alte Primär-PDF enthält tatsächlich `u-Kerze, sauber/ neu`; die neue enthält `Cu-Kerze, sauber/ neu`. Beide Schreibweisen sind auch im gerenderten Dokument sichtbar, nicht nur im Textextrakt. Der Handler kennt ausschließlich die alte Schreibweise. Daher 35 statt 36 zugeordnete Preise und bewusster Abbruch der gesamten Liste.

Die neue Liste hat weiterhin **38 numerische Ankaufzeilen = 36 exakt zuordenbare + 2 schon bisher unzugeordnete Sorten**, außerdem eine Anfragezeile und drei getrennte Entsorgungsgebühren. Eine Reduktion der Sollzahl wäre falsch: Sie würde die vorhandene Kupfer-Kerze unterschlagen. Kein Nachweis eines Poppler-/Zeilenumbruchfehlers; es ist eine Labeländerung der Quelle (plausibel eine Schreibfehlerkorrektur, Betreiberabsicht nicht behauptet).

**Empfehlung zur Freigabe:** Nur den belegten Alias ergänzen und die echte neue PDF als zusätzliche Regression pinnen; 36er-Gate, positive/exakte Beträge, Einheiten und Altfixture unverändert lassen. Isolierter Kandidat ist rot/grün geprüft, **nicht in Main angewandt**. Owner entscheidet über Übernahme und spätere reguläre Scheduler-Verifikation.

## Umfang / Ausgangspunkt

- README, `crates/ingestion/src/traders/handlers/elno.rs`, bestehende fünf ELNO-Regressionen, Fixture-Metadaten und `research/handler-owner-elno-20261008.md` gelesen. Kein `GLOSSARY.md` oder einschlägiges ADR gefunden.
- Ausgangs-HEAD: `8d0bd4932a887dc64cdb613d381ebb565628be36`.
- Handler SHA-256: `01414d537a5ad5f2f35d0644a064e29a12d7035cb2b792061b20b17dfadd34cb`.
- Kein DB-Zugriff, kein Service-/Deploy-/Ingestion-Force-Run, kein Commit/Push. Keine Dossiers oder fremden Wellen geändert. Kein Cargo-/Releasebuild; vorhandene Test-/Example-Binaries benutzt. Zusätzlich nur kleiner isolierter Rust-Testlink gegen vorhandene Debug-Abhängigkeiten, ohne Cargo/Buildskript.
- Einzige neue Repo-Datei dieses Auftrags ist dieser Bericht. Kleine Diagnoseartefakte liegen unter `/tmp/opencode/elno-review-20261017/` (temporär, nicht als dauerhafte Fixture ausgegeben).

## Primärbelege

Abruf am 17.10.2026 um ca. 16:01 UTC:

- Startseite: <https://www.elno-container.de/> verlinkt `Unsere Altmetall-Preise` direkt auf <https://www.elno-container.de/wp-content/uploads/2026/10/Preisliste_16.10.pdf>.
- Aktuelles PDF: HTTP 200, `application/pdf`, **210166 Bytes**, 2 A4-Seiten. SHA-256 **`74ba81979f1d8a1b7c83a71798bcb57b09e0597a2bd9c5f82d3dd97944cb76c6`**.
- HTTP `Last-Modified: Fri, 16 Oct 2026 11:20:19 GMT`; PDF-Metadaten CreationDate/ModDate `16.10.2026 11:19:59 UTC`. **Kein gedrucktes Veröffentlichungsdatum**: weder URL noch Dateimetadaten als `published_at` importieren.
- Altfixture `crates/ingestion/src/traders/handlers/fixtures/elno-20261008.pdf`: 210199 Bytes, 2 Seiten, SHA-256 `f406df3eb717c230c488010fb073f37eba0e6e0184a3176658edd7fe4dcde6e9`; damalige URL <https://www.elno-container.de/wp-content/uploads/2026/09/Preisliste_11.09.pdf>.
- Beide PDFs mit **derselben realen Handler-Extraktion** untersucht: `/usr/bin/prlimit --as=268435456 --cpu=15 -- /usr/bin/pdftotext -raw PDF -`. Zusätzlich `-layout` als unabhängige Lesart und `pdftoppm`-Ansicht beider relevanten Seiten geprüft. Layoutausgabe dient nur Gegenprüfung, nicht Ersatzparser.
- Privatkunden; Einstufung vor Ort; freibleibend/tagesabhängig; Gewerbekunden telefonisch anfragen. Keine pauschalen Gewerbe-/garantierten Preise ableiten.

## Zuerst echter Repro, dann vier prüfbare Hypothesen

Vor der Ursachenfestlegung tatsächlich ausgeführt:

```sh
target/debug/examples/live_handlers be-spandau-elno-container-und-dienstleistungs
```

```text
=== be-spandau-elno-container-und-dienstleistungs ===
FAILED: response from https://www.elno-container.de/wp-content/uploads/2026/10/Preisliste_16.10.pdf was not usable: incomplete ELNO price list: only 35 mapped prices
Exit 1
```

Das vorhandene Example ist laut Quelle DB-frei; es ruft den echten registrierten Handler auf, inklusive Startseiten-Linksuche, Impressumsprüfung, PDF-Abruf, begrenzter Extraktion und `parse_text`. Danach wurde der Feedbackloop mit gepinntem PDF offline deterministisch gemacht.

| Hypothese / falsifizierbare Vorhersage | Prüfung und Ergebnis |
|---|---|
| 1. Echte entfernte Sorte: neue PDF hat eine numerische Ankaufzeile weniger. | Falsch: alt und neu jeweils 38 ` €/Kg`-Quotes, alle Sorten vorhanden. |
| 2. Geänderte Schreibweise: ein bisheriges Label unterscheidet sich; Austausch nur dieses Tokens stellt 36 wieder her. | **Bestätigt:** `Cu-Kerze` → `u-Kerze` im neuen Extrakt macht den unveränderten Parser erfolgreich. |
| 3. Layout-/Umbruchverlust: sichtbare Sorte ist in `-raw` getrennt/falsch einsortiert, in `-layout` oder Render aber vollständig. | Falsch für den Fehler: neue Kerze steht vollständig in einer Raw-Zeile. Beide Schreibweisen werden jeweils visuell bestätigt. Bekannte Akku-/Al-Draht-/Kühlerumbrüche bleiben korrekt zugeordnet. |
| 4. Einheit/Vorzeichen/Preisformat geändert: unbekannter Einheitentoken oder nichtpositiver Betrag verursacht die fehlende Zuordnung. | Falsch für die Quelle: alle 38 Ankaufquotes positiv, unverändert `€/Kg`. Negative/Range-/Einheiten-Mutationen bleiben auch mit Kandidat rot. |

Kleinster kausaler Unterschied: **ein führendes `C` im Sortenlabel**, kein Preis-/Einheiten-/Schwellwertwechsel. `grade()` (Zeile 191) matcht nur `u-Kerze, sauber/ neu`. Das neue Label landet daher in `skipped_labels`; `parse_text()` bricht bei 35 Preisen ab (Zeilen 291–299), bevor ein Outcome mit diesen Diagnose-Skips zurückgegeben wird. Der Count-Fehler ist die Schutzwirkung, nicht die Ursache.

## Vollständige numerische Ankaufzeilen

Alle Beträge in beiden folgenden Preisspalten sind **positiv, ohne Minus-/Pluszeichen**, Originaleinheit **`€/Kg`**, semantisch **EUR/kg**. Keine €/t-Konversion, keine Ranges. Die Nummerierung folgt der Raw-Extraktion innerhalb der PDF-Gruppen. `M` = bekannte Katalogzuordnung; `X` = bereits in Altfixture absichtlich nicht exakt zugeordnet; `BUG` = nur aktuelles Label scheitert in Main.

| # / Seite | Sorte (neue PDF) | Altfixture | Neue PDF | Main / Bewertung |
|---|---|---:|---:|---|
| 1 / 1 | Cu-Raff 95% | 9,30 | 9,70 | M |
| 2 / 1 | Cu-Millberry | 10,30 | 10,70 | M |
| 3 / 1 | Cu.Haardraht | 9,90 | 10,30 | M |
| 4 / 1 | Cu-Kerze, sauber/ neu | 9,90 | 10,30 | **BUG**, alt `u-Kerze, sauber/ neu` |
| 5 / 1 | Cu-Schienen, sauber | 10,20 | 10,50 | M |
| 6 / 1 | Cu Kabel ca. 38% | 3,20 | 3,40 | M |
| 7 / 1 | Ms-Milbe, 1% Anhaftung | 5,40 | 5,70 | M |
| 8 / 1 | Ms-Raff (Wasseruhren etc.) | 3,00 | 3,10 | M |
| 9 / 1 | Ms-58 | 5,60 | 5,90 | M |
| 10 / 1 | Cu-MS Kühler | 2,90 | 3,00 | X, keine exakte Katalogzuordnung |
| 11 / 1 | V2A | 0,60 | 0,60 | M |
| 12 / 1 | V2A Verhüttung | 0,20 | 0,20 | M |
| 13 / 1 | V4A | 1,35 | 1,35 | M |
| 14 / 1 | Mischschrott | 0,130 | 0,130 | M |
| 15 / 1 | Shreddervormaterial | 0,07 | 0,07 | M |
| 16 / 1 | Bremsscheiben | 0,165 | 0,165 | M |
| 17 / 1 | Sorte 3 | 0,200 | 0,210 | X, keine exakte Katalogzuordnung |
| 18 / 1 | Leichte Schere | 0,145 | 0,145 | M |
| 19 / 1 | Schwere Schere | 0,155 | 0,155 | M |
| 20 / 1 | Guss | 0,140 | 0,140 | M |
| 21 / 1 | Alt-Zink | 1,30 | 1,50 | M |
| 22 / 1 | Neu-Zink | 1,40 | 1,60 | M |
| 23 / 1 | Alt-Blei | 0,90 | 0,90 | M |
| 24 / 1 | Akku-Blei, z.B. Autobatterien | 0,275 | 0,275 | M, mehrzeilig |
| 25 / 1 | Sammelzinn | 5,00 | 5,00 | M |
| 26 / 2 | Al-Draht, ohne Anhaftungen | 1,95 | 1,95 | M, mehrzeilig |
| 27 / 2 | Al-Profile, gemischt | 1,90 | 1,90 | M |
| 28 / 2 | Al-Profile, Länge über 1m | 1,70 | 1,70 | M |
| 29 / 2 | Al-Blech ohne Anhaftung | 1,20 | 1,20 | M |
| 30 / 2 | Al-Geschirr, max. 5% FE | 1,00 | 1,00 | M |
| 31 / 2 | Al-Verhüttung | 0,20 | 0,20 | M |
| 32 / 2 | Al-Kabel, dick | 0,40 | 0,45 | M |
| 33 / 2 | Al-Guss ohne Anhaftung | 0,95 | 0,95 | M |
| 34 / 2 | Al-Guss, 5% FE | 0,85 | 0,85 | M |
| 35 / 2 | Al-Felgen mit Anhaftung | 1,60 | 1,60 | M |
| 36 / 2 | Al-Felgen ohne Anhaftung | 1,60 | 1,60 | M |
| 37 / 2 | Al-Cu Kühler mit Anhaftung | 3,50 | 3,50 | M, mehrzeilig |
| 38 / 2 | Al-Cu Kühler ohne Anhaftung | 3,70 | 3,70 | M, mehrzeilig |

Weitere Zeilen (alt wie neu), vollständig und **nicht** als Ankaufquote importieren:

| Seite | Zeile | Originalquote / Einheit | Einordnung |
|---|---|---|---|
| 1 | Elektronikschrott (Platinen, Netzteile, Rechner etc.) | Auf Anfrage | Keine numerische Quote; bekannter Skip |
| 2 | Styropor | 89 €/cbm | Positive Netto-Entsorgungsgebühr, kein Ankauf |
| 2 | Baumischabfälle | 290 €//to | Positive Netto-Entsorgungsgebühr; doppelter Slash bereits im Original |
| 2 | Asbesthaltige Baustoffe | 280 €/to | Positive Netto-Entsorgungsgebühr, kein Ankauf |

Die drei Gebühren bleiben getrennt unter `Abfälle, Preise Netto`. Nicht durch Vorzeichenwechsel in Ankaufpreise verwandeln. Vier mehrzeilige Ankaufbeschriftungen (Akku, Draht, beide Al-Cu-Kühler) wurden gegen Render und beide Extraktionsmodi geprüft.

## Ausgeführte Regressionen und isolierter Patchentwurf

1. Vorhandenes Testbinary, ohne Build: `target/debug/deps/schrott_mcp_ingestion-530d7e54aa916dc8 elno --nocapture` → **5 bestanden**, einschließlich echter Alt-PDF und Mutationen.
2. Isolierter Harness aus dem **vollständigen aktuellen Handlerquelltext**, dessen echten gemeinsamen Daten-/HTTP-/Zahlenhelfern und lokaler Error-Typschale, ohne Store/Scheduler/DB. Vorhandene `reqwest`-/`tokio`-/`scraper`-rlibs benutzt; direkter `rustc --test`-Link. Alle fünf Originaltests mitgenommen.
3. Neuer echter Binär-PDF-Test erwartet 38 numerische Zeilen, 36 Preise, drei bekannte Skips, identische Material-/Variantenschlüssel zur Altfixture, `kupfer-berry` / `Kerze, sauber/neu` = 10,30 EUR/kg mit **neuem Raw-Label**, keine erfundene Publikation. Entfernte Kerze und Duplikat müssen fehlschlagen; negative Quote, Range, €/t, kleingeschriebenes €/kg und Anfragequote werden abgelehnt.
4. **Rot vor Patch:** `python3 /tmp/opencode/elno-review-20261017/repro.py` → 6 bestanden, 1 fehlgeschlagen, Exit 101, exakt `incomplete ELNO price list: only 35 mapped prices`. Laufzeit der Tests ca. 0,08 s.
5. **Grün nur in Scratch-Kopie:** `python3 /tmp/opencode/elno-review-20261017/repro.py --candidate` → **7 bestanden**, Exit 0, ca. 0,11 s. 36 Preise, gleiche 36 Material-/Variantenschlüssel wie vorher; Skips exakt `Cu-MS Kühler`, `Sorte 3`, Elektronikschrott auf Anfrage. Alle 36 Preis-/Einheitenzeilen wurden ausgegeben und gegen obige Liste geprüft.

Einziger funktionaler Kandidat, **nicht angewandt**:

```diff
--- a/crates/ingestion/src/traders/handlers/elno.rs
+++ b/crates/ingestion/src/traders/handlers/elno.rs
@@
-        "u-Kerze, sauber/ neu" => ("kupfer-berry", "Kerze, sauber/neu"),
+        "u-Kerze, sauber/ neu" | "Cu-Kerze, sauber/ neu" => ("kupfer-berry", "Kerze, sauber/neu"),
```

Owner sollte dazu das verifizierte neue PDF als `fixtures/elno-20261017.pdf` plus Herkunft/Hash pinnen und den unten beschriebenen Test in das vorhandene ELNO-Testmodul übernehmen. Keine unscharfe Normalisierung, keine Umklassifizierung von Cu-MS Kühler/Sorte 3, keine Sollzahländerung. Alter Alias bleibt für Regression/historische Quellen nötig.

### Reproduzierbarkeit ohne Cargo-/Releasebuild

Voraussetzungen: vorhandene kompatible `target/debug/deps`-rlibs, Poppler/prlimit, stabiler Rust-Compiler **1.98.1**. Die hier voreingestellte rustup-Nightly 1.100 ist mit diesen alten rlibs inkompatibel; erster Linkversuch scheiterte daran, ohne Änderungen am Projekt. Daher expliziter Stable-Pfad im Harness. Kein Dependency-Rebuild gestartet.

```sh
mkdir -p /tmp/opencode/elno-review-20261017
curl --fail --location --silent --show-error --max-time 45 --max-filesize 5000000 --output /tmp/opencode/elno-review-20261017/current.pdf https://www.elno-container.de/wp-content/uploads/2026/10/Preisliste_16.10.pdf
sha256sum /tmp/opencode/elno-review-20261017/current.pdf
python3 /tmp/opencode/elno-review-20261017/repro.py
python3 /tmp/opencode/elno-review-20261017/repro.py --candidate
```

Vor erneutem Repro den oben genannten SHA-256 prüfen; bei geänderter Quelle ist es nicht mehr dieselbe Regression. Das Scratch-Skript bleibt lokal verfügbar. Für dauerhaften Regressionseinbau folgende echte Test-Seam benutzen (innerhalb des vorhandenen ELNO-Testmoduls, sobald Owner die zusätzliche Fixture freigibt):

```rust
#[tokio::test]
async fn actual_october_pdf_preserves_kerze_and_all_grades() {
    let text = extract(include_bytes!("fixtures/elno-20261017.pdf"), URL)
        .await.unwrap();
    assert_eq!(text.matches(" €/Kg").count(), 38);
    let out = parse_text(&text, URL).unwrap(); // Main rot: only 35 mapped prices
    assert_eq!(out.prices.len(), 36);
    assert_eq!(out.skipped_labels.len(), 3);
    assert!(out.published_at.is_none());
    assert!(out.prices.iter().all(|p| p.unit == "EUR/kg"
        && p.currency == "EUR" && p.price > 0.0));
    let kerze = out.prices.iter().find(|p| p.material == "kupfer-berry"
        && p.variant == "Kerze, sauber/neu").unwrap();
    assert_eq!(kerze.price, 10.30);
    assert!(kerze.label.starts_with("Cu-Kerze, sauber/ neu;"));
    for bad in ["-10,30 €/Kg", "10,30-11,00 €/Kg", "10,30 €/t",
        "10,30 €/kg", "Auf Anfrage"] {
        let changed = text.replace("Cu-Kerze, sauber/ neu 10,30 €/Kg",
            &format!("Cu-Kerze, sauber/ neu {bad}"));
        assert_ne!(changed, text);
        assert!(parse_text(&changed, URL).is_err());
    }
    assert!(parse_text(&text.replace("Cu-Kerze, sauber/ neu 10,30 €/Kg", ""), URL).is_err());
    assert!(parse_text(&format!("{text}\nCu-Kerze, sauber/ neu 10,30 €/Kg"), URL).is_err());
}
```

Der ausgeführte Scratch-Test prüft zusätzlich die unveränderte vollständige Schlüsselmenge und identische Skips zur Altfixture. Der kausale Kontrolltest ersetzt im aktuellen Extrakt **nur** `Cu-Kerze, sauber/ neu` durch `u-Kerze, sauber/ neu`: Main wird damit bereits ohne Codeänderung grün. So hängt das Ergebnis nicht allein an der Kandidatenimplementierung.

## Offene Grenzen / Übergabe

- Schedulerzeitpunkt 11:31:52 UTC ist die übergebene Meldung, nicht hier per Prod-DB nachrecherchiert. Der gleiche Fehler wurde später live exakt reproduziert.
- Isolierter Kandidat belegt Parserheilung, **keinen erfolgten Main-Fix, Release, Deploy oder erfolgreichen Produktionslauf**. Originalhandler bleibt unverändert und wird bis Owner-Freigabe weiter scheitern.
- Nach Owner-Entscheidung: neue Fixture/Regression + Einzeilenalias isoliert übernehmen, ELNO-Regressionen laufen lassen; vorhandenen separaten Releaseprozess nicht duplizieren. Anschließend regulären Schedulerlauf abwarten und dessen Ergebnis nach dem bestehenden Owner-Gate verifizieren, ohne erzwungene Ingestion.
