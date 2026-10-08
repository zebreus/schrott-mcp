# Recherchewelle 08.10.2026 B — Owner-Protokoll

Einmal gestartet, trotz doppelt zitierter Wellen-Anforderung. Auswahl:
60 Slugs frisch per read-only Abfrage aus `traders`, `website='' OR street=''`,
zufällig je 20 aus drei disjunkten Bundeslandgruppen. Die 60 Dossiers der
abgeschlossenen Welle (Commits 2fb74c1/877ba54/fa2e115) ausgeschlossen.
Keine Agentenpolls, keine doppelte Bearbeitung durch den Owner.

| Shard | Bundesländer | Hintergrundsession | Bericht |
| --- | --- | --- | --- |
| 1 | BW, BY, TH, SN, HB | ses_ee49c3efdffeNNxDfLXPPbU4fu | research/wave-20261008b-shard1.md |
| 2 | HE, RP, ST, MV, BB | ses_ee49c3ee3ffeDb8h4EBlmGe1RC | research/wave-20261008b-shard2.md |
| 3 | NI, NW, SH, BE, SL, HH | ses_ee49c3e8affeMtmq2er3B7kllO | research/wave-20261008b-shard3.md |

Alle drei Aufträge wurden als Hintergrundläufe angenommen. Noch keine
Abschluss-/Qualitätsfreigabe. Agenten bekommen Ziel, konkrete Sluglisten,
knappen Kontext und methodischen Freiraum. Gemeinsamer Owner-Gate und
Veröffentlichung erst nach den Ergebnissen; Produktion bleibt read-only.

## Gleichzeitig angeforderter stündlicher Check-in

Dienst aktiv, keine Warnungen im Sechs-Stunden-Fenster. 3874 Händler /
52 Materialien / 2267 Current-Preise. Feedback bis #5353 gesichtet;
#5328–#5353 (26 neue Hinweise) noch nicht einzeln verifiziert.
Ingestion #1126: 17 Preise, ein Fehler beim NORDKAT-Impressum (404).
Owner repariert diesen Quellpfad getrennt von den Wellen-Dossiers:
roter exakter Handler-Livetest, Primärnavigation und neuer Impressumspfad
bestätigt, danach grüner Handler-Test. Keine Preisinterpretation oder
Materialkatalogerweiterung im Rahmen dieses kleinen Fixes.

NORDKAT-Fix `5fff4db` getrennt von laufenden Dossieränderungen aus
isoliertem Commit-Worktree gebaut und am 08.10.2026 12:07 UTC deployed.
Vorher Livetest rot mit HTTP 404; danach alle vier NORDKAT-Tests inklusive
echtem Livehandler grün. Workspace: 563 bestanden, null Fehler, vier
ignorierte Spezial-/Livetests; Rustfmt grün. Beide Release-Binaries
gesichert unter `/var/tmp/schrott-mcp-backup-before-5fff4db/`.
Öffentliche Health-URL und Query-Worker grün, 3874 Händler bestätigt.
Der Fix erfindet keine numerischen PDF-Preise; normaler nächster NORDKAT-
Ingestion-Lauf steht als separate Produktionsverifikation noch aus.
Git-Push scheitert weiterhin an fehlenden HTTPS-Zugangsdaten.

Installed SHA256:
- Server: `72555cf938868d5fd71e34dfad0610ba454d4c485ecdd79180c3a1bb3f112afa`
- Worker: `c72d402b001a779dfc79f93e1972c0da4d86097789fdee0c62c13ce47cd99178`

## Shard 1 abgeschlossen — Teilreview

Bericht `research/wave-20261008b-shard1.md` gelesen. MAR-Lauingen-
Kontaktseite unabhängig erneut abgerufen: Adresse Nr. 25, Telefon,
Materialannahme und standortspezifische Pausenzeiten bestätigt.
Frontmatter-Website auf die Betreiber-Hauptseite normalisiert;
Standort-/Kontaktbelege bleiben in der Timeline. Koordinaten noch offen.
ARR-Löschung und Seitz-Schließung bleiben transparente Prüfhinweise,
keine ungeprüfte Umstellung auf geschlossen. Gemeinsamer vollständiger
Owner-Gate, Commit und Deployment erst nach Abschluss aller Shards.

## Shard 3 abgeschlossen — Teilreview

Bericht `research/wave-20261008b-shard3.md` und Waldi-/Missal-Diffs gelesen.
Waldi-Kontaktseite unabhängig erneut gelesen: Kahlenbergstraße 9,
Telefon und E-Mail kongruent; verlinkte Google-Place-Geometrie bestätigt
49.2750147 / 7.1569562, noch nicht als geprüfter Anlieferpunkt übernommen.
Abgelaufene Zertifikate werden nicht als heute gültig bezeichnet.
AuDie/Gudrun-Branchenzweifel bleiben Prüffälle, keine Löschung aufgrund
von Namensähnlichkeit. Missal-Zusatzarbeiten dürfen nicht als pauschal
kostenlos gelten; structured Pickup-Formulierung im gemeinsamen Gate prüfen.

## Gemeinsamer Owner-Gate

Alle drei Shards abgeschlossen, alle Berichte gelesen; 60/60 Dossiers.
Automatischer Vergleich gegen HEAD: sämtliche alten Timeline-Bullets
erhalten, Slugs/Frontmatter-Schlüssel stabil; YAML, Enumwerte aus Seedcode,
Root-Websites, fehlende Emdash-Platzhalter, Service-JSON, endliche gepaarte
WGS84-Werte und datierte Nachträge geprüft. Diff-Check und Rustfmt grün;
acht Seedtests bestanden, null Fehler.

Nichtleere Überschreibungen einzeln geprüft: MAR-Typ/Aktivstatus durch
identifizierten aktuellen Betreiber; ahab/Reinert zurück auf pruefung wegen
fehlender belastbarer Aktivbestätigung; Missal-Kostenformulierung anhand
erneut gelesener Betreiberpreise eingeschränkt. BBW bleibt als Schrott-
Ankauffall pruefung trotz belegtem aktivem Baustoffbetrieb. Die Agenten-
Recherche dazu bleibt erhalten, Owner-Korrektur ausdrücklich ergänzt.
ASR-Standortpaar und Öffnungszeiten live auf Betreiberseite bestätigt;
kommunaler Ortsteilbeleg löst Klieken/Coswig auf. Waldi-Paar aus expliziter
Google-Place-Geometrie übernommen, als Adresspunkt, nicht Toreinfahrt.
MAR und BBW: Nominatim nur Straßenobjekte ohne bestätigte Hausnummer;
MAR-Kurzlink nur Kartenzentrum. Deshalb keine Ersatzkoordinaten erfunden.

Keine neue qualifizierte numerische Metallankaufpreisliste. BBW-PDF:
Baustoffverkauf und Kippgebühren, Hauptliste bereits am 30.09. abgelaufen;
keine Preisbeobachtung erzeugt. ARR/Seitz/Wetzel/ReMi und Identitätszweifel
AuDie/Gudrun bleiben konkrete offene Prüffälle statt behaupteter Schließung.

## Veröffentlichung und Produktionsprüfung

Gemeinsamer Dossier-/Berichtscommit `7465638`. Beide Release-Binaries
aus sauberem Working Tree dieses Commits gebaut; Deployment/Neustart
08.10.2026 12:23 UTC. Installation atomar ersetzt, nachdem direktes
Kopieren auf die laufende Serverdatei mit Text-file-busy abgewiesen wurde.
Backups: `/var/tmp/schrott-mcp-backup-before-7465638/`.
Bootseed: genau 60 Zeilen aktualisiert. Service aktiv, öffentliche Health
grün, installierter Read-only-Query-Worker bestätigt 3874 Händler.
Sieben Produktions-Spotchecks bestätigen MAR-Kontakte/Zeiten/Root-Website,
Waldi-/ASR-Paare, BBW/ahab/Reinert-Prüfstatus und Missal-Kostenpräzisierung.
Aktuell 2277 Preiszeilen; normale parallele Ingestion, kein Preisimport
durch diese Dossierwelle. Git-Push erneut konkret fehlgeschlagen:
fehlender HTTPS-Benutzername, keine Remote-Veröffentlichung behauptet.

**Offene Geo-Altlast:** MAR und BBW haben trotz fehlendem Dossier-Paar
noch historische Produktionskoordinaten (MAR 48.5654/10.4297,
BBW 52.12701333333334/11.617686666666666). Der Seed erhält fehlende
Paare absichtlich; diese Altwerte sind daher nicht als neu verifizierte
Betriebspunkte zu verstehen. Hausnummer-genaue Dossierpaare bleiben
Folgearbeit; keine direkte DB-Löschung oder Rust-Geocodierung ausgeführt.

Installed SHA256 nach Welle:
- Server: `2654b72aae93f81dd098fc9cffd325c12d33b223b39b12f198c3b6749cb4fdb8`
- Worker: `c72d402b001a779dfc79f93e1972c0da4d86097789fdee0c62c13ce47cd99178`
