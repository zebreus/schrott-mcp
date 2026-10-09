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

**Deployment abgeschlossen:** Commit `7cd8f4f` am 08.10.2026 09:34 UTC.
Handler-Fixes allein entfernen keine alten Preispointer. Migration 6 ist
jetzt implementiert und geprüft: transaktionale, idempotente Entfernung
von elf anhand Händler, Quell-URL, Material, Variante und Quelllabel
belegten falschen Pointern. 236 historische Fehlzuordnungen bleiben mit
Originalpreis/Datum/Provenienz erhalten, werden als Fehlmapping #4782
annotiert (`approx`, Konfidenz 0). Keine erfundenen Ersatzbeobachtungen,
kein Rückgriff auf eventuell ebenfalls falsche ältere Preise.
30 Store-Tests einschließlich Replay, unveränderter Nachbarvarianten und
Rollback grün. Normale Bootmigration in Produktion angewandt, Version 6:
elf alte Current-Pointer entfernt, 243 historische Fehlzuordnungen
annotiert (seit dem Vorab-Snapshot kamen sieben Beobachtungen hinzu).
Keine direkten SQL-Korrekturen. Der breitere Restzeilen-Backlog
#4783/#4809 bleibt offen.

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
Die übrigen Adressfälle bleiben offen; Deploymentprüfung bestätigt die
fünf neuen Koordinatenpaare für EMR, PMK, SD, RHM und SHP in Produktion.

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
Deployment dieser Arbeiten aus sauberem Commit abgeschlossen. Die laufende Dossierwelle wird
getrennt geprüft und nicht ungeprüft mit ausgerollt.

## Neue Leads beim Anschluss-Check-in

Feedback #4788–#4879 ist noch nicht vollständig verifiziert; keine pauschale
Freigabe für neue Händler oder Massenkorrekturen. Seit der vorherigen
Prüfung kamen #4838–#4879 (42 Meldungen) hinzu. Die hoch priorisierten
Heinen-Standorte #4876/#4877 und der Identitätsfall KVR #4875 werden separat
an Primärquellen geprüft; Berichte dienen der späteren Owner-Integration.
Dienst aktiv, keine Warnungen in den letzten sechs Stunden, Bestand beim
Check-in 3871 Händler / 52 Materialien / 2276 aktuelle Preise.

### Anschluss nach unterbrochenem Release-Build

- Commit `7cd8f4f` enthält die fertig geprüften Import-/Preisreparaturen.
  Letzter tatsächlicher Push scheiterte wieder an fehlenden HTTPS-
  Zugangsdaten; die frühere erfolgreiche Veröffentlichung bis `530538e`
  bleibt davon unberührt.
- Release-Build durch Neustart unterbrochen, anschließend im isolierten
  Worktree desselben Commits fortgesetzt. Produktionsmigration noch nicht
  angewandt (`user_version = 5`); Dienst und öffentliche Health-URL gesund.
- Recherchewelle: Shard 1 mit Bericht abgeschlossen; Shard 2/3 und die
  Heinen-/KVR-Prüfungen durch Nutzungslimit gestoppt. Vorhandene Änderungen
  bleiben erhalten; Shard-3-Bericht ist noch kein abgeschlossener Test-/
  Owner-Gate-Nachweis. Keine zusätzliche Welle gestartet.
- Feedback inzwischen bis #5236: seit #4879 weitere 357 Hinweise
  (60 high, 205 medium, 92 low), nicht pauschal verifiziert.
- #5227 einzeln bestätigt: Brandmayr in Thierhaupten anhand Betreiber-
  Leistungs-/Impressumsseiten plus unabhängigem kommunalem Firmeneintrag.
  Neues Dossier erstellt, keine geratenen Koordinaten/Preise. Samstagregel
  widerspricht dem genannten Oktoberdatum und wird nicht schematisiert.
  #5228/#5229 und übrige neue Leads bleiben offen.

### Deployment-Verifikation

Beide Release-Binaries aus sauberem Worktree von `7cd8f4f` gebaut; dort
acht Seed-Tests bestanden. Backups der vorherigen Binaries liegen unter
`/var/tmp/schrott-mcp-backup-before-7cd8f4f/`. Normaler Dienstneustart:
Seed schreibt 86 geänderte Datensätze, Händlerzahl unverändert 3871.
Version 6 aktiv, Current-Preiszahl erwartungsgemäß 2265 statt 2276;
alle elf konkret benannten alten Preis-IDs ohne Current-Pointer.
243 markierte Historienzeilen bleiben erhalten (`approx`, Konfidenz 0).
Keine Ersatzpreise erfunden; frische Preise müssen regulär gescrapt werden.

Öffentliche Health-URL und Query-Worker-Roundtrip grün. Vor dem Neustart
ein einzelner Worker-Spawn mit EAGAIN; danach erfolgreicher MCP-Aufruf
und Worker-Test, Dienst mit 3 Tasks weit unter TasksMax 3647.
Beobachten, keine unbelegte Ressourcen-Konfigurationsänderung.

Installed SHA256:
- Server: `368c39993f00adfb9dfa5facd8345b09953560440a9259cf5cc08075d1822b80`
- Worker: `93b3fc1b87abf9a248762f27fbcedddfc62bdcc9fdfbaf37d72d18f81f371ac3`

### Nächster stündlicher Check-in

Dienst aktiv, keine Warnungen im aktuellen Sechs-Stunden-Fenster. Migration
6 weiterhin aktiv; Bestand 3871 Händler / 52 Materialien / 2267 Current-
Preise. Letzte Ingestion #1116: 36 Preise übernommen, null Händlerfehler;
#1115 ebenfalls fehlerfrei. Neue Feedbacks #5237–#5277 (41 Hinweise) noch
nicht einzeln verifiziert. Tatsächlicher Pushversuch weiterhin ohne
HTTPS-Zugangsdaten gescheitert.

Abgeschlossene Süd-/Ost-Teilwelle (20 Dossiers) vom Owner vollständig
reviewt; Primär-/Registerbelege der beiden Stammdaten-Fills nachgeprüft.
Diese Teilwelle wird unabhängig von den am Nutzungslimit unterbrochenen
40 Dossiers integriert. Keine neue Agentenwelle am bekannten Limit.

Release `2fb74c1` am 08.10.2026 10:04 UTC deployed, beide Binaries aus
isoliertem Commit-Worktree. Acht Seed-Tests dort bestanden, Rustfmt und
Diffprüfung grün. Normaler Seed-Start schreibt 21 Datensätze (20 geprüfte
Dossiers plus Brandmayr). Produktion: 3872 Händler / 52 Materialien /
2267 Current-Preise. Read-only Spotcheck bestätigt Gröger-Straße,
PreZero-Pyral-Stammdaten, Berger-Status `pruefung` und neues Brandmayr-
Dossier; Query-Worker liefert Brandmayr/Thierhaupten, öffentliche Health
und MCP-Aufrufe erfolgreich. Übrige 40 Dossieränderungen nicht deployed.

Backup: `/var/tmp/schrott-mcp-backup-before-2fb74c1/`.
Server-SHA256: `f9557061a9928fb7658da699dc10a9b754e6b4fce8f8ad9e3a250cf3e6854478`.
Worker-SHA256: `57d32b2ff1d720d6bbb67bbae2e790ab87c63add3c40e90dc59cdc78394873e1`.

### Anschluss-Check-in: Mittel-/Nordost-Teilwelle

Neue Feedbacks #5278–#5292 (15 Hinweise) noch ungeprüft. Dienst aktiv,
keine Warnungen im aktuellen Sechs-Stunden-Fenster; Bestand weiterhin
3872 Händler / 52 Materialien / 2267 Current-Preise. Ingestion #1118
übernimmt 105 Preise mit einem Fehler: AMR-Quelle HTTP 502, bereits
bekannter externer Quellfehler. Keine Ersatzpreise oder ungeprüfte
Handleränderung; andere Händler und Dienst funktionieren.

Die 20 vorhandenen Änderungen der gestoppten Mittel-/Nordost-Teilwelle
vollständig vom Owner gelesen. Nur zwei Statuskorrekturen und ein
Websitezustand im Frontmatter, sonst Quellen-/Klärfallnotizen. Schanko,
Demand-Fahrzeugbedingungen und Weisi-Annahmepause separat live geprüft.
Bericht `research/wave-20261008-shard2.md` unterscheidet Agentenabbruch
von Owner-Abschluss. Die Nord-/West-Teilwelle bleibt separat offen.
Tatsächlicher HTTPS-Push scheitert weiterhin an fehlenden Zugangsdaten.

Teilwelle deployed als `877ba54` am 08.10.2026 10:35 UTC. Acht Seed-Tests
im isolierten Commit-Worktree bestanden, beide Binaries gebaut und gesichert
unter `/var/tmp/schrott-mcp-backup-before-877ba54/`. Seed schreibt 20
geänderte Dossiers; Voigt/Sell/Schanko read-only in Produktion bestätigt,
Health und Worker-Roundtrip grün. Ingestion #1119: 60 Preise, null Fehler.
Die noch uncommitteten 20 Nord-/West-Dossiers wurden nicht eingebettet.
Server-SHA256: `08f2a756cd32a94cc8477da382c812fb1ef415460b3fa601f1cbabb0d876e17e`.
Worker-SHA256: `13f26d4f7837c49d194b374f706bba3ed4206a83e83b35ba97588248c8f857f5`.

### Anschluss-Check-in: neue Standorte #5323/#5324

Dienst aktiv, keine Warnungen im aktuellen Sechs-Stunden-Fenster;
Ingestion #1123 mit 237 Preisen und null Händlerfehlern. Bestand vor
Ergänzung: 3872 Händler / 52 Materialien / 2267 Current-Preise.
Feedback #5313–#5327 (15 neue Hinweise) gesichtet, nicht pauschal freigegeben.

- #5323 bestätigt mit wichtiger Identitätskorrektur: Recycling-KG ist
  LR Leitl GmbH & Co. Recycling KG, HRA 7824; Leitl GmbH, HRB 4228,
  ist die gruppenübergreifende persönlich haftende Gesellschaft.
  Betreiber-Gruppendetail plus beide Registerprofile gelesen. Keine
  pauschale GmbH-Gruppenzeile als Ankaufshof. Hofannahme belegt,
  Vergütung offen; genaue Betreiber-Firmenmarkerkoordinate aus dem
  Kartenlink statt abweichendem Feedback-Koordinatenfenster dokumentiert.
- #5324 bestätigt: Carnuth-Straubing fehlt als separates Werk, Carnuth-
  Bogen/Furth besteht bereits. Betreiberkontakt, Impressum, Registerprofil
  und PÜG-PDF gelesen; PDF seit 30.09.2026 abgelaufen, keine aktuelle
  Zertifizierung behauptet. Freitagszeiten widersprechen sich in mehreren
  HTML-Blöcken, daher keine schematische neue Zeitfüllung. Privatannahme/
  Vergütung offen, keine erfundenen Preise oder Koordinaten.

Owner prüft drei Dossieränderungen, Historie/Slugs erhalten, zwei neue
Root-URLs, keine Placeholder-Fills. Tatsächlicher Git-Push weiterhin ohne
nutzbare HTTPS-Zugangsdaten fehlgeschlagen. Übrige neue Feedbacks offen.

Deployment `79dd103` am 08.10.2026 11:36 UTC aus sauberem Commit-Stand:
acht Seed-Tests, Rustfmt und Diffprüfung grün, beide Release-Binaries
gebaut. Backup `/var/tmp/schrott-mcp-backup-before-79dd103/`.
Boot-Seed schreibt drei Dossiers; read-only Datenbank-/Workerprüfung
bestätigt beide neuen Standorte, Leitl-Firmenmarker und bewusst leere
Carnuth-Koordinaten. Health erfolgreich. Bestand nun 3874 Händler /
52 Materialien / 2267 Current-Preise; keine erfundenen neuen Preise.
Server-SHA256: `9fb15f5578f6c89b592f67d5905f212594dd18d6b9965567ccb0eb2cdc4d325d`.
Worker-SHA256: `13f26d4f7837c49d194b374f706bba3ed4206a83e83b35ba97588248c8f857f5`.

### Anschluss-Check-in: Nord-/West-Teilwelle

Dienst aktiv, keine Warnungen im aktuellen Sechs-Stunden-Fenster; 3872
Händler / 52 Materialien / 2267 Current-Preise. Letzte Ingestion #1121:
87 Preise und null Händlerfehler. Neue Feedbacks #5293–#5312 (20 Hinweise)
noch ungeprüft; Push weiterhin wegen fehlender HTTPS-Zugangsdaten blockiert.

Letzte 20 Dossierdiffs der Welle vom Owner vollständig gelesen. Alunorf-
Betreiberidentität und JV-Partner gegen Betreiberseiten plus Registerdaten
geprüft; Schlör-Adresse gegen Impressum plus Original-GZQ-Zertifikat
bestätigt. Weber-Websitefill trotz erreichbarer Seite nicht freigegeben:
Identität bisher nur Einzelbeleg, URL bleibt im Text. Keine FMT-Schließung
ohne unabhängige Bestätigung, keine geratenen Koordinaten oder Ankaufkurse.

Release `fa2e115` am 08.10.2026 11:05 UTC deployed: acht Seed-Tests,
Rustfmt und Diffprüfung grün; Arbeitsbaum vor Build/Installation sauber.
Seed schreibt 20 Dossiers, Read-only Spotprüfung Alunorf/Schlör/Schriever/
Weber bestätigt die freigegebenen Felder. Öffentliche Health-URL, Worker
und MCP-Aufruf erfolgreich. Alle 60 Wellen-Dossiers nun integriert;
Identitäts-/Annahme-Klärfälle bleiben offen, nicht pauschal geschlossen.
Push auch mit explizitem `gh auth git-credential` ohne nutzbare Credentials.
Backup `/var/tmp/schrott-mcp-backup-before-fa2e115/`.
Server-SHA256: `dd936ff679092b16c5daa416173ed57141c137ca55e03d07e2e348b3ee24f6d9`.
Worker-SHA256: `13f26d4f7837c49d194b374f706bba3ed4206a83e83b35ba97588248c8f857f5`.

VHM-Quellpfad vom Owner direkt gelesen:
<https://www.vhm-hartmetall.de/script.js?v=vhm-preise-20260928-1>
lädt <https://www.vhm-hartmetall.de/vhm-preise-aktuell.php> mit
Cache-Buster und `no-store`. JSON am 08.10.2026 HTTP 200, `ok: true`:
Wendeschneidplatten und VHM-Fräser/Bohrer 53,00 EUR/kg, gemischt und
Widia 50,00 EUR/kg, Schlamm `amount: null` / „nach Analyse“.
Hinweis: sauber sortiertes Material, alle Preise pro kg.
