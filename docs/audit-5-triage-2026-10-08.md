# Audit 5: Triage am 08.10.2026

Feedback #4770–#4782 ist ein Rechercheauftrag, keine automatische
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
Handler-Fixes löschen keine falschen Historienzeilen und keine alten
Preispointer mit nun unbenutzten Varianten. Deren Bereinigung benötigt
eine gesonderte, getestete Migration; keine direkten SQL-Korrekturen.

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
Seed-Compiler validiert Dossierkoordinaten, exportiert sie aber nicht;
ein Dossier-Commit alleine korrigiert die Produktionskarte daher nicht.
Diese Importlücke und die übrigen Fälle bleiben offen.

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

## Parallel abgeschlossene Preislistenrecherche

Schrottjungs Bremen und Leipzig: 19 unverbindliche Großmengen-Preisobergrenzen
bestätigt. Mobile Einsatzgebiete sind keine belegten lokalen Höfe.
Abholgebühren und Eisen-Mindestmenge widersprechen sich zwischen Seiten.
Ein stadtweise vervielfältigter Tagespreis-Handler wäre daher irreführend;
kanonische Betreiberzuordnung und sichtbare Konditionen sind noch zu klären.

## Veröffentlichung

Normale HTTPS-Pushes scheiterten zuletzt an fehlenden Zugangsdaten.
Ein Fehler von `gh` allein ist kein Beweis, dass Git-Push unmöglich ist;
maßgeblich ist der tatsächliche Pushversuch.
