# Owner-Gate — Recherchewelle 20.10.2026, 12:01 UTC

## Umfang und struktureller Audit

Die zufällig aus der read-only Produktionsauswahl gezogenen 60 Slugs sind nach Bundesland disjunkt (20 NW, 20 HE, 20 NI); alle 60 zugehörigen Dossiers wurden bearbeitet. Kein vorheriger Timeline-Text ging verloren oder wurde umgeordnet. Die Owner-Prüfung fand keine Änderung an `slug`, `state`, `trader_type` oder `status` und keine neu eingetragenen Dossierkoordinaten. Alle geänderten Dossiers haben datierte Rechercheeinträge und trennen Ankauf, Verkauf und Gebühren. Die neu hinzugefügten Em-Dashes stehen nur in Überschriften; Quellen-URLs sind syntaktisch valide. `git diff --check` ist sauber.

NW und NI haben keine Frontmatter-Änderungen. HE hat in neun Dossiers geprüft und korrigiert:

- Stadt normalisiert: Fischer (Buseck) und MEH Jäger (Heppenheim).
- Straße/PLZ/Telefon zurückgenommen, wo diese nur als ungeklärte Import-/Portalwerte geführt waren: Fischer, MEH Jäger und Harry Sell.
- Telefon zurückgenommen: zusätzlich TDN, Baydar und Sabri.
- Straße zurückgenommen: Scholz Rico, Theobald Deichmann und Reinhardts.
- Bei den sechs Telefon-Rücknahmen (`Fischer`, `MEH Jäger`, `Harry Sell`, `TDN`, `Baydar`, `Sabri`) wurde jeweils `clear_fields: 'phone'` ergänzt. Laut `docs/trader-seed.md` würde ein leeres Enrichment-Feld ohne diese Direktive den gespeicherten Wert behalten. Die jeweiligen Timeline-Einträge erläutern Datum, Beleggrenze und die nicht als falsch bewiesene historische Angabe.

## Identität, Quellen und fachliche Beleggrenzen

- Direkte Stichproben bestätigten die dokumentierten Lead-Grenzen: Gelbe Seiten führt Harry Sell in Babenhausen mit Ostring 15/0171 8333254; es bleibt ein Aggregator und wird nicht mit Sell Recycling/Kitzingen, Oskar Sell/Schaafheim oder der historischen Dublette zusammengeführt. Der Fischer-Eintrag auf 11880 wurde separat als direkter Profil-Link ergänzt; beide Treffer bleiben Verzeichnisleads.
- Beim Sabri-Auftritt bestätigen Start-/Kontaktseiten zwar Robert-Koch-Straße, Kontaktdaten und Leistungswerbung, das Impressum lieferte aber einen themenfremden Template-Artikel statt eines Rechtsträgers. Kein Website-Link oder Identitäts-/Annahmestellen-Fill.
- Bei Freigericht bestätigen die direkt gelesenen AC-Betreiberseiten und das Impressum die gesonderte AC-Rechtsträger-/Preisquelle. Deren Preis- und Gebührenangaben bleiben ausdrücklich **nicht** dem generischen Freigericht-Zieldossier zugeordnet; die Seite zeigt Abweichungen zwischen Entfernungstabelle und Bild-/Alttext. Kein Fill oder GEO für das Ziel.
- Rinderknecht ist auf SchrottplatzMetall als Kandidat gelistet; die Ressource erklärt selbst, kein Schrottunternehmen zu vertreten. Sie bleibt ein Drittanbieterlead. Die verlinkte Händlerdomain war nicht erreichbar, ohne daraus Schließung abzuleiten.
- **Owner-Korrektur Theobald Deichmann:** Zusätzlich zum Cylex-/Infobel-Lead wurde `schrott-deichmann.de` gefunden und direkt einschließlich Impressum und Abholungsseite geprüft. Die Selbstauskunft nennt Theobald Deichmann, Speyerstr. 40, 68649 Groß-Rohrheim, Telefon und E-Mail; Abholung nur nach Voranmeldung; Entrümpelung gegen nicht bezifferten Aufpreis. Das ist ein Betreiberauftritt, aber noch keine unabhängige zweite qualifizierte Bestätigung oder bestätigte öffentliche Anlieferstelle. Cylex-Suchergebnis nennt Adresse/Telefon, direkter Abruf 403; Infobel zeigt eine Nutzungs-/Captcha-Sperre; Creditreform-Suchergebnis führt einen gleichnamigen Betrieb, direkter Abruf 403. Daher kein strukturierter Address-/Website-Fill, keine Zuordnung zu Namensvettern und keine GEO-Freigabe. Dossiertext und Timeline wurden so korrigiert, dass sie den gefundenen Betreiberauftritt nicht mehr leugnen.
- Die übrigen geänderten HE-Felder bleiben bewusst aufgelöst bzw. leer, wo Identität, aktuelle Kundenannahme, Betreiberbezug oder ein zweiter zulässiger Beleg fehlt. Verzeichnisübereinstimmung allein wird nicht als zwei unabhängige Quellen gezählt.

## GEO-Blocker (read-only Produktionsabgleich)

Die neun HE-Dossiers mit Frontmatter-Adressrücknahme enthalten **keine Koordinaten im Dossier**. In der aktuellen `public.db` sind dennoch folgende ältere Koordinatenpaare gespeichert. Nach `docs/trader-seed.md` lässt ein Dossier ohne Koordinatenpaar bestehendes GEO unverändert; eine Adressrücknahme löscht oder validiert den Pin ebenfalls nicht.

| Dossier | Bestehendes GEO in `public.db` |
|---|---:|
| `he-wiesbaden-erbenheim-scholz-rico-schrott` | 50.0778461538, 8.2453076923 |
| `he-buseck-35418-fischer-wohnwagenverwertung` | 50.6148, 8.783 |
| `he-heppenheim-64646-meh-jager` | 49.6407, 8.6367 |
| `he-babenhausen-64832-sell-harry-schrotthandler` | 49.0551, 9.6023 |
| `he-gro-rohrheim-theobald-deichmann-schrotthandel` | 49.7211, 8.4828 |
| `he-darmstadt-arheilgen-tdn-recycling-schrott` | 49.8718375, 8.6533 |
| `he-frankfurt-baydar-schrott` | 50.1194048780, 8.6539731707 |
| `he-dietzenbach-schrotthandler-sabri` | 50.0098, 8.7778 |
| `he-maintal-dornigheim-reinhardts-schrott-metallhandel` | 50.15, 8.8333 |

**Bestätigte grobe Fehlzuordnung:** `49.0551, 9.6023` wird von vier Produktionsdatensätzen in Babenhausen geteilt: Harry Sell sowie die separaten Einträge `he-babenhausen-64832-a-f-w-recycling`, `he-babenhausen-64832-kohlmann-frank` und `he-babenhausen-64832-ries-peter`. Read-only Reverse-Lookup und OSM-Objektseite ordnen den Punkt dem Weg `268472967` bei Wielandsweiler/Hütten, Mainhardt, Baden-Württemberg (74523) zu, nicht Babenhausen. Das beweist nicht den wahren Standort irgendeines der vier Händler; es belegt aber, dass dieser gemeinsame Punkt für die Babenhausen-Dossiers unplausibel ist. Er ist ein Altbestand und wurde von dieser Welle nicht geschrieben.

Für die übrigen acht HE-Punkte wurde im Rahmen dieses Gates keine unabhängige Koordinaten-/Kundenstandortprüfung vorgenommen. Sie bleiben ungeklärt. Es gibt keinen Seed-seitigen GEO-Entzug im aktuellen Vertrag; keine Produktions-SQL-Änderung und keine Rust-Geocodierung. **Kein Redeploy dieser Adresskorrekturen**, bis ein zulässiger Owner-Weg für die alten Punkte geprüft und der betroffene gemeinsame Babenhausen-Bestand sicher behandelt wurde.

Belege: [Seed-/GEO-Vertrag](../docs/trader-seed.md), [OSM-Way 268472967](https://www.openstreetmap.org/way/268472967), [Nominatim-Reverse-Abfrage](https://nominatim.openstreetmap.org/reverse?lat=49.0551&lon=9.6023&format=jsonv2). Die öffentliche Produktionsdatenbank wurde nur lesend abgefragt.

## Tests und Veröffentlichungsstatus

- `git diff --check`: bestanden.
- Die Owner-Struktur-/Enum-/Timeline-/Emdash-/Preisformat- und URL-Prüfungen liefen lokal mit read-only Python-Auswertung; 60/60 Scope-Dossiers bestanden.
- Kein Cargo-Seed-Test/Build: Root hatte bei der frischen Prüfung 1.235.275.776 Bytes frei (~1.178,1 MiB), rund 22 MiB unter dem festgelegten 1.200-MiB-Minimum. Der bekannte private Kurzboot-/Artifact-Hold bleibt bestehen.
- **Nicht committet, nicht gepusht, nicht deployed.** `main` ist 32 Commits vor `origin/main`, und die bestehende uncommittete Arbeit aus anderen Scopes bleibt unangetastet. Der GEO-Befund ist zusätzlicher, unabhängiger Veröffentlichungsblocker. Es gab keine Produktionsdatenbank-Schreibzugriffe.
