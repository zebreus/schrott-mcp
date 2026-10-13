# SDM Gutzmann — Feedback #5556/#5562, 08.10.2026

## Ergebnis / Entscheidung

- **#5562 berechtigt:** 21 aktuelle Ankaufzeilen auf der Betreiberseite, aber 19 im Live-Handler und in Produktion. Kernschrott S3 **200 EUR/t** und Batterien **100 EUR/t** werden ausdrücklich übersprungen. Das ist eine Katalog-/Mappinglücke, kein fehlender HTTP-Abruf.
- **#5556 teilweise berechtigt:** Tottleben hat tatsächlich null Preise, aber die daraus abgeleitete Forderung nach Preisduplikation ist nicht belegt. Betreiber bezeichnet Tottleben als **Büro & Buchhaltung**, Gotha als **Recyclinghof**. Es fehlt nicht einfach ein zweiter Scraper; zuerst Standortsemantik entscheiden.
- **Zusatzfehler, hohe Priorität:** Gotha-Handler liefert Tottlebener Hauptsitzadresse. Produktion hat deshalb bereits die gemischte Adresse **Hauptstraße 46, 99947, city Gotha** statt **An der Ostbahn 4, 99867 Gotha**. Erst Handler korrigieren, dann nachhaltige Seed-Reparatur; bloßer Redeploy unveränderten Dossiers reicht ggf. wegen Seedhash nicht.
- Nur Diagnose und Dossiernotizen geändert; **keine Codefixes, keine Veröffentlichung, keine Produktionswrites**.

## Primärquellen und fachliche Grenzen

Am 08.10.2026 live abgerufen:

1. <https://www.sdm-gutzmann.de/preislisten/>: „Aktuelle Preise für Schrott“, Tabellenheader `Schrott | Preis €/t`; 21 Zeilen inkl. Kernschrott S3 200,00 € und Batterien 100,00 €. Einleitung behauptet stets aktuelle Preise, jedoch **kein sichtbarer Preisstichtag oder Gültigkeitsbeginn/-ende**. Separate Tabellen „Entsorgung“ und „Baustoffe“ folgen. HTML-Meta `article:modified_time=2022-03-01T08:19:10+00:00` ist Seitentechnik, kein verifizierter Preisstand. Kein aktuelles Preis-PDF für diese Liste erforderlich; Kurse stehen direkt im HTML.
2. <https://www.sdm-gutzmann.de/leistungen/schrott-metallhandel/>: „Wir kaufen das Altmetall zu tagesaktuellen Preisen ein“ verlinkt auf die Preisliste; Abgabe im Recyclinghof oder beauftragte Abholung, Privat- und Gewerbekunden, Personalausweis bzw. Gewerbeschein. Eigener Abschnitt „Schrottverkauf“ beschreibt Belieferung von Stahlwerken/Gießereien, macht die verlinkte Ankauftabelle nicht zu Verkaufspreisen. **Keine Annahme von Nickel-Cadmium-Batterien** (sowie Computerschrott, asbesthaltigen Metallen, Kühlschränken, Nachtspeicheröfen). Label „Batterien“ spezifiziert keine positive Chemieklasse: nicht blind als Blei oder als alle Batteriearten mappen.
3. <https://www.sdm-gutzmann.de/kontakt/>: „Tottleben | Büro & Buchhaltung“, Hauptstraße 46, 99947 Tottleben; „Gotha | Recyclinghof“, An der Ostbahn 4, 99867 Gotha. Die Kontaktzeiten stehen bei „Büroanschrift“ Gotha; sie beweisen keine Tottlebener Annahmezeiten. Preise haben keine explizite Standortspalte, aber Abgabe-/Recyclinghofkontext und Gotha-Bildtexte stützen Gotha, nicht einen zweiten Tottlebener Hof.
4. <https://www.sdm-gutzmann.de/impressum/>: Hauptsitzadresse Tottleben ist korrekt für die Rechtsperson, nicht für den Gothaer Recyclinghof.

Entsorgungspreise werden dem Kunden **berechnet**, netto pro Tonne, nach tatsächlicher Anliefermenge; unter 100 kg pauschal. Sack-/BigBag-Zeilen haben Stück-/Bagbezug. Baustoffpreise sind ausdrücklich **Verkauf**, ebenfalls netto pro Tonne. Diese Tabellen dürfen nicht in positive Schrottankaufkurse geraten. Die Schrotttabelle selbst kennzeichnet keinen Netto-/Bruttostatus: nicht hinzuerfinden.

Kontrollabruf 20:04:01 UTC per Python urllib: 165731 Bytes, SHA256 `a7f0c1f7d8bbc5b36e5aff4cdc7724850b997ac15e77dd5237b1058113262b60`. Dynamische HTML-Ausgabe kann Bytezahl/Hash verändern (Rust-Abruf: 168340 Bytes); Sorten und Kurse waren übereinstimmend. Abrufdatum ist Beobachtung, kein Veröffentlichungsdatum.

## Reproduktion: tatsächlicher Handler, keine DB-Schreibzugriffe

Repo-Stand zu Beginn: `072cf25cee749795e104624daacc2608411e84bd`.
`cargo` war nicht im PATH; absolute Cargo-Adresse funktioniert.

```sh
/root/.cargo/bin/cargo run -q -p schrott-mcp-ingestion --example live_handlers -- th-gotha-sdm-gutzmann
```

Live ausgeführt, HTTP 200, `published_at=None`, `acceptances=0`, `alive=true`:

```text
TraderInfo { street: "Hauptstraße 46", postcode: "99947", city: "Tottleben", ... }
19 Preiszeilen, darunter Mischschrott 120 EUR/t und Kupfer Millberry 6400 EUR/t
SKIP: Kernschrott S3
SKIP: Batterien
```

Die vollständigen 19 Labels: Shreddervormaterial; Mischschrott; Scherenschrott schwer; Gussschrott; Kupfer Millberry; Kupfer Berry; Kupfer Raff; Messing schwer; Kupferkabel; Alu Kabel; E Motore; Alu Geschirr; Alu Blech eisenfrei; Alu Profile ISO gemischt; Alu Felgen; Zink; Blei; V2A; V4A. Geschirr und Blech teilen ein Material, werden aber mit unterschiedlicher Variante gespeichert; keine fehlende Zeile wegen dieser Doppelzuordnung.

Für `-- th-tottleben-sdm-gutzmann` gleicher CLI-Aufruf: **keine Ausgabe, Exit 0**. Tool filtert registrierte Handler und meldet unbekannte Slugs nicht. Das beweist keinen erfolgreichen leeren Scrape, sondern keinen passenden Handler. Registry `crates/ingestion/src/traders/handlers/mod.rs` registriert nur `gutzmann::handler()` mit Gotha-Slug.

Unattended Rot-Signal gegen den wirklichen Pfad (ausgeführt; Exit 1):

```sh
python3 - <<'PY'
import subprocess
s = subprocess.check_output(['/root/.cargo/bin/cargo', 'run', '-q',
    '-p', 'schrott-mcp-ingestion', '--example', 'live_handlers', '--',
    'th-gotha-sdm-gutzmann'], text=True)
for label in ['Kernschrott S3', 'Batterien']:
    assert f'SKIP: {label}' not in s, f'RED: missing purchase price: {label}'
print('GREEN: target grades not skipped')
PY
```

Ausgabe: `AssertionError: RED: missing purchase price: Kernschrott S3`. Kein Fix in dieser Session, daher bleibt rot. Live-Netzwerk ist nicht streng deterministisch; die vorhandene Inline-Fixture enthält exakt beide fehlenden Zeilen und ist die offline reproduzierbare Testbasis. Regression muss später positive Grade-/Wert-/Einheits-/Variantenassertions enthalten, nicht nur Abwesenheit von SKIP.

## Produktionsbeleg — ausschließlich read-only

DBs `/var/lib/schrott-mcp/public.db` und `internal.db` ausschließlich mit Python `sqlite3.connect('file:…?mode=ro', uri=True)` geöffnet. Keine Migration, keine Ingestion, keine Serverausführung auf DBs.

Feedbackdetails live gelesen: #5556 meldet Tottleben 0 vs Gotha 19 sowie beide fehlenden Sorten; #5562 meldet dieselben fehlenden Ankaufkurse und trennt Gebühren/Verkauf korrekt. Beide vom 08.10.2026 19:16 UTC.

```sql
SELECT t.slug,t.street,t.postcode,t.city,COUNT(v.material_slug) AS prices
FROM traders t LEFT JOIN v_current_prices v ON v.trader_slug=t.slug
WHERE t.slug IN ('th-gotha-sdm-gutzmann','th-tottleben-sdm-gutzmann')
GROUP BY t.id;
```

Read-only tatsächliche Einzelabfragen ergaben:

| Slug | Straße | PLZ | Ort | aktuelle Preise |
|---|---|---|---|---:|
| th-gotha-sdm-gutzmann | Hauptstraße 46 | 99947 | Gotha | 19 |
| th-tottleben-sdm-gutzmann | Hauptstr. 46 | 99947 | Tottleben | 0 |

Alle 19 Gotha-Preise `observed_at=2026-10-08T14:09:44.903634166+00:00`; `published_at`, `valid_from`, `valid_to` jeweils NULL. NE-Metalle wurden korrekt EUR/t → EUR/kg katalogkonform normalisiert (z.B. Millberry 6,4 EUR/kg). Kein Umrechnungsfehler erklärt S3/Batterien.

Letzte drei Produktionsschritte: IDs 4700/4595/4469, Status `ok`, jeweils 19 Preise übernommen, 2 übersprungen (`Kernschrott S3; Batterien`), am 08.10.2026 14:09/08:11/02:09 UTC. Produktive Materialsuche bestätigt kein Batteriematerial und keine Sorte-3; Sorte-1 ist als Neuschrott definiert und kein Ersatz für S3.

## Ursache und nächste Schritte für Owner

Nach Reproduktion getestete Hypothesen: (1) bewusst fehlendes Mapping/Katalog, (2) fehlender Tottleben-Handler bei unterschiedlicher Standortfunktion, (3) Impressumsenrichment am falschen Standort. Alle drei durch Output/Registry/Code bestätigt; (2) ist **nicht** automatisch ein Bug.

1. **Katalog-/Mappingentscheidung:** `gutzmann.rs:76–119` hat keinen S3-/Batteriearm; vorhandener Test `mapping_splits_grades_and_skips_catalog_gaps` erwartet ausdrücklich `None` für beide. Katalog in `crates/ingestion/src/pipeline.rs` und Produktion gemeinsam erweitern, semantisch eindeutige S3-Sorte und Batterietyp wählen. Keine Behelfsmappings auf Sorte 1, Scherenschrott oder Blei. Batterieklasse bei Bedarf beim Betreiber bestätigen, NiCd-Ausschluss erhalten.
2. **Regressionsseam:** bestehende Handler-Fixture enthält beide Werte. Künftiger Test soll echten Handler-/Outcome-Mappingpfad auf 21 Preise, S3 200 EUR/t, Batterien 100 EUR/t bzw. entsprechende katalogkonforme Normalisierung prüfen; Gebühren/Verkauf weiterhin ausgeschlossen. Varianten Geschirr/Raff erhalten, Quelle/Datum nicht fingieren.
3. **Adressregression zuerst verhindern:** `gutzmann.rs:55–59,126–220` extrahiert Rechtsträgeranschrift und liefert sie für Gotha. `traders/mod.rs:500–511` reicht das an `set_trader_info`; `store/src/public/traders.rs:363–368` überschreibt Straße/PLZ, Ort nur wenn leer. Daher Hybridadresse. Branchspezifischen Gotha-Kontaktblock extrahieren oder nur sichere Telefon/E-Mail-Felder zurückgeben; Test muss Gothaadresse und unveränderte Tottlebenadresse absichern. Bestehender Impressumtest kodifiziert aktuell die falsche Standortzuordnung, nicht eine Parserstörung.
4. **Seedheilung nach Codefix:** Gotha-Dossier hat bereits die richtige Anschrift, aber Seedhash/Enrichment können eine bloße unveränderte Neuveröffentlichung wirkungslos machen. Die hier ergänzten Timeline-/Überblicksnotizen sichern Verlauf und bieten veränderten Seedpayload; Owner muss Seedgate/rebuild/deploy und Read-only-Spotcheck durchführen, kein direkter Produktions-SQL-Fix durch diesen Agenten. Danach nächsten regulären Handlerlauf prüfen, damit Straße/PLZ nicht erneut kippen.
5. **Tottleben keine Preisduplikation ohne Evidenz:** Dossier als Büro/Firmensitz erklären; bei Bedarf `description`/Servicebedingungen in gemeinsamer Stammdatenentscheidung schärfen. Allgemeiner Firmenankauf ist nicht gleich Annahme in Tottleben. Falls Betreiber getrennten Ankauf dort bestätigt, erst dann Standortpreise/Handler registrieren.
6. **Wechselnde Sorten:** bei unbekannten Zeilen weiter Wert/Einheit/Quelle diagnostisch erhalten und Abdeckungsabweichung sichtbar machen; heutiger Erfolgstatus `ok` trotz 2 bewusster Auslassungen darf nicht als vollständige Liste interpretiert werden. Zusätzlich Lösch-/Entfernungssemantik separat prüfen: dieser Check belegt fehlende aktuelle Sorten, nicht Verhalten bei später weggefallenen Zeilen.

Beide Dossiers um belegte Standort-/Preishistorie ergänzt; alle bisherigen Timeline-Einträge und strukturierten Stammdaten erhalten. Codefix, Katalogänderung, Tests des gemeinsamen Fixes und Veröffentlichung liegen beim Owner.

## Gate der Dokumentationsänderungen

- `/root/.cargo/bin/cargo test -q -p schrott-mcp-ingestion seed_traders`: **6 bestanden**, 0 Fehler; Dossiercodegen ebenfalls erfolgreich.
- `/root/.cargo/bin/cargo test -q -p schrott-mcp-ingestion traders::handlers::gutzmann`: **4 bestanden**, 0 Fehler. Diese bestehenden Tests erwarten die Katalogauslassungen ausdrücklich; grün bedeutet hier **nicht** behobene Preisabdeckung.
- `git diff --check`: erfolgreich. Keine Parser-/Katalog-/Storedatei durch diesen Agenten verändert.
