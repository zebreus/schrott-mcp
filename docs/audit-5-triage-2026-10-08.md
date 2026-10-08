# Audit 5: Triage am 08.10.2026

Feedback #4770–#4787 ist ein Rechercheauftrag, keine automatische
Freigabe für Datenänderungen. Produktionsabfragen erfolgen read-only.

## Dringend: #4782 Preiszuordnung

Die gemeldeten aktuellen Werte wurden im Produktionsbestand überprüft.
Vier Schrott-Handler werden getrennt untersucht; Historie und aktuelle
Preispointer müssen dabei unabhängig bewertet werden.

- Lungwitz und DB Recycling: bestätigt. `Zinkblech` trifft einen zu breiten
  Aluminium-Blech-Matcher. Lokale Fixes verlangen ausdrücklich `Al Blech`;
  DB Recycling erhält eine Zink-Zuordnung. Regressionstests zuerst rot,
  anschließend grün; Livequelle bestätigt Zink 2,05 bzw. 2,00 EUR/kg und
  Aluminium 1,00 EUR/kg. Quellen:
  <https://vlschrott.de/einkaufspreise/>,
  <https://www.dein-schrottplatz.de/pages/preise.php>.
- Triebsch und ALBUS: bestätigt und lokal repariert. Kabel wird vor
  generischem Kupfer erkannt; ALBUS bewahrt Materialsorte und Kundenkarten-
  Bedingung als getrennte Schlüssel. Alu-Profile erhält das richtige Material.
  Regressionstests zuerst rot, danach grün, beide Liveabrufe HTTP 200.
  Quellen: <https://www.schrott-triebsch.de/>,
  <https://www.albus-leipzig.de/preise>.
- Hansa: bestätigt. Die Feinheit `875` fehlte in der expliziten Tabelle;
  `875er Gold` wurde als `gold` mit leerer Variante ausgegeben. Test zuerst
  rot, nach Ergänzung grün. Liveabruf erfolgreich. Quelle:
  <https://hansa-goldankauf.de/>.
- Goldankauf-Börse Erfurt: bestätigt. `625` fehlte ebenso; `625er Silber`
  wurde als `silber` mit leerer Variante ausgegeben. Test zuerst rot,
  nach Ergänzung grün. Liveabruf erfolgreich. Quelle:
  <https://www.goldankauf-boerse.de/ankaufsrechner/>.

Die Mappingversion wird auf 2 erhöht: Ein Unterschied gegenüber einer
Beobachtung mit falscher alter Zuordnung darf kein vermeintliches neues
Veröffentlichungsdatum erzeugen.

**Noch nicht erledigt:** Veröffentlichung/Deployment und Produktionsverifikation.
Handler-Fixes allein entfernen keine alten Preispointer. Migration 6 ist
jetzt implementiert und geprüft: transaktionale, idempotente Entfernung
von elf anhand Händler, Quell-URL, Material, Variante und Quelllabel
belegten falschen Pointern. 236 historische Fehlzuordnungen bleiben mit
Originalpreis/Datum/Provenienz erhalten, werden als Fehlmapping #4782
annotiert (`approx`, Konfidenz 0). Keine erfundenen Ersatzbeobachtungen,
kein Rückgriff auf eventuell ebenfalls falsche ältere Preise.
30 Store-Tests einschließlich Replay, unveränderter Nachbarvarianten und
Rollback grün. Noch nicht in Produktion angewandt; keine direkten
SQL-Korrekturen. Der breitere Restzeilen-Backlog #4783/#4809 bleibt offen.

## Dringend: #4770 Koordinaten

Read-only bestätigt wurden die vom Audit genannten Produktionswerte
für EMR Hamburg, PMK Hamburg, SD Frankenthal, RHM Mülheim und FLAXRES
Dresden. Diese passen offensichtlich nicht zur angegebenen Region.
Die behaupteten 134 Fälle sind noch nicht vollständig einzeln geprüft;
Bounding-Boxen allein beweisen weder Betreiberidentität noch die richtige
Ersatzkoordinate. Teilweise kann auch das Bundesland falsch sein.

Beauftragte Adress-/Betreiberprüfung: EMR Hamburg, PMK Hamburg,
SD Frankenthal und RHM Mülheim. Koordinaten gehören mit Belegen in die
Dossiers, nicht in einen Rust-Geocoding-Lauf. Wichtig: Der aktuelle
Seed-Compiler validierte Dossierkoordinaten, exportierte sie aber nicht;
ein Dossier-Commit alleine korrigierte die Produktionskarte daher nicht.
Die Importlücke ist nun mit paarweiser WGS84-Validierung, Hash-/Update-
Semantik und Regressionstests repariert. Rust führt kein Geocoding aus.
Die übrigen Adressfälle bleiben offen; Deploymentprüfung steht noch aus.

Owner-Nachprüfung: SD-Kontaktseiten-HTML enthält den vollständigen
Adressmarker 49.5513872/8.3477812; RHM verlinkt seinen Firmenmarker
51.4378111/6.8418507 (nicht den Kartenmittelpunkt). Dossieränderungen
inklusive bestehender Historie geprüft; Seedtests 5 bestanden.
Implementierung des Dossier-Koordinatenimports ist separat beauftragt.

## Weitere Meldungen: offen, nicht pauschal verworfen

| ID | Einordnung / nächste Prüfung |
| --- | --- |
| 4771 | ASCII-Umschreibungen in FTS: Suchverhalten reproduzieren; Normalisierung ist eine Produkt-/Indexänderung, keine Händlerkorrektur. |
| 4772 | Stadtteil ohne Stadt im Ortsfeld: adressweise Primärquellenprüfung; keine pauschale Ortsersetzung. |
| 4773 | Sammeldossiers: genannte Händler einzeln verifizieren. Aggregatorfund allein reicht nicht für neue aktive Händler. |
| 4774 | Notizfragmente im Namen: Herkunft/Identität prüfen. Keine ungeprüfte Löschung oder Schließung von Datensätzen. |
| 4775 | Fremdtext im Straßenfeld: vollständige Betreiberadresse prüfen, vor allem bei Mehrstandortangaben. |
| 4776 | PLZ im Ortsfeld: Quellenabgleich und strukturierte Trennung; keine blinde Massenkorrektur aus Legacytext. |
| 4777 | Ortsfragmente: einzelne Identitäts-/Adressprüfung nötig. |
| 4778 | Alte Slugs bleiben stabile Schlüssel. Falsches `state` ist dagegen zu korrigieren, wenn der Standort belegt ist; Siegen-Fall offen. |
| 4779 | Telefonplatzhalter sind keine Nummern; konkrete Felder und Betreiberkontakte prüfen. |
| 4780 | FTS-Wortsuche ist keine Teilwortsuche. Erwartung und tatsächliche Suchoberfläche prüfen, nicht allein aus LIKE-Vergleich einen Defekt ableiten. |
| 4781 | Händlerübergreifende Schreibvarianten beweisen keine Kollision. Nicht ohne fachliche Prüfung Sorten verschiedener Händler vereinheitlichen. |
| 4783 | Veraltete Mapping-Schlüssel: Alter allein ist kein Löschbeleg. Eng belegte Fehlmapping-Pointer werden per getesteter Migration bearbeitet; übrige 115 gemeldete Zeilen noch einzeln prüfen. |
| 4784 | Bestätigt und repariert: Betreiber-Script lädt `/vhm-preise-aktuell.php`; Live-JSON liefert 53/50/53/50 EUR/kg und Schlamm nur nach Analyse, updatedAt 2026-10-05T08:51:01+00:00. Exakter Livehandler geprüft; alte 65/63-Werte sind keine aktuellen Quellpreise. Nächste reguläre Ingestion nach Deployment erforderlich. |
| 4785 | Bestätigt und repariert: Betreiber-JS/CSS blendet MwSt./Brutto für Ankauf aus. Handler übernimmt die tatsächlich angezeigte Nettospalte statt verstecktem Brutto. Regression zuerst rot, danach grün; 44 Live-Ankaufzeilen geprüft. Keine erfundene Privat-/Gewerbesteuerregel; konkrete Abrechnung bleibt beim Betreiber. |
| 4786 | Rötgesbüttel: wechselnde Quellfassungen reproduzieren. Keine stille Auswahl des höheren oder niedrigeren Preises. Offen. |
| 4787 | Zinn: Huth-Lötzinn/Geschirr gegen Originalsorten prüfen. Unspezifisches Zinn nicht nur anhand niedrigen Preises als Legierung einstufen. Offen. |

## Parallel abgeschlossene Preislistenrecherche

Schrottjungs Bremen und Leipzig: 19 unverbindliche Großmengen-Preisobergrenzen
bestätigt. Mobile Einsatzgebiete sind keine belegten lokalen Höfe.
Abholgebühren und Eisen-Mindestmenge widersprechen sich zwischen Seiten.
Ein stadtweise vervielfältigter Tagespreis-Handler wäre daher irreführend;
kanonische Betreiberzuordnung und sichtbare Konditionen sind noch zu klären.

## Veröffentlichung

Normale HTTPS-Pushes funktionieren wieder: Änderungen bis `530538e`
wurden erfolgreich auf `owner/dossier-quality-2026-10-07` veröffentlicht.
Ein Fehler von `gh` allein ist kein Beweis, dass Git-Push unmöglich ist;
maßgeblich ist der tatsächliche Pushversuch. Kein Force-Push auf main.

## Laufende Anschlussarbeiten

Vier Code-/Quellenprüfungen abgeschlossen: Dossier-Koordinatenimport,
eng belegte Fehlmapping-Preispointer, VHM-Live-JSON sowie Metallorum-
Ankaufspalte. Gemeinsamer Workspace-Test: 563 bestanden, 0 fehlgeschlagen,
3 ignorierte Live-/Spezialtests; `cargo fmt --all -- --check` grün.
Noch kein Deployment dieser Arbeiten. Die laufende Dossierwelle wird
getrennt geprüft und nicht ungeprüft mit ausgerollt.

## Neue Leads beim Anschluss-Check-in

Feedback #4788–#4879 ist noch nicht vollständig verifiziert; keine pauschale
Freigabe für neue Händler oder Massenkorrekturen. Seit der vorherigen
Prüfung kamen #4838–#4879 (42 Meldungen) hinzu. Die hoch priorisierten
Heinen-Standorte #4876/#4877 und der Identitätsfall KVR #4875 werden separat
an Primärquellen geprüft; Berichte dienen der späteren Owner-Integration.
Dienst aktiv, keine Warnungen in den letzten sechs Stunden, Bestand beim
Check-in 3871 Händler / 52 Materialien / 2276 aktuelle Preise.

VHM-Quellpfad vom Owner direkt gelesen:
<https://www.vhm-hartmetall.de/script.js?v=vhm-preise-20260928-1>
lädt <https://www.vhm-hartmetall.de/vhm-preise-aktuell.php> mit
Cache-Buster und `no-store`. JSON am 08.10.2026 HTTP 200, `ok: true`:
Wendeschneidplatten und VHM-Fräser/Bohrer 53,00 EUR/kg, gemischt und
Widia 50,00 EUR/kg, Schlamm `amount: null` / „nach Analyse“.
Hinweis: sauber sortiertes Material, alle Preise pro kg.
