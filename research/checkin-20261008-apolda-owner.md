# Owner-Check-in — Scholz Apolda, 08.10.2026

## Gesundheit und Feedback

Service aktiv, Warning-Journal sechs Stunden ohne Einträge. 3878 Händler,
52 Materialien, 2280 aktuelle Preise vor Veröffentlichung; Läufe 1141–1144
sämtlich ok, null Händlerfehler. Keine neuen Feedbackmeldungen seit #5517;
letzte zehn erneut gesichtet. Regulärer NORDKAT-Schritt nach Fix weiterhin
offen, alter 404 aus Lauf 1126 nicht als neuer Produktionsfehler behandelt.
Working Tree vor Arbeit sauber.

## Offenen Recherchefall abgeschlossen

Aus dem im letzten Check-in deduplizierten Paar #5075/#5503 genau einen
Scholz-Apolda-Standort verifiziert. Read-only-DB-Prüfung Apolda/99510/
Flurstedter Marktweg: nur W.K.W. Am Kalkteich 6 vorhanden, kein Scholz-Match.
Neues Dossier `dossiers/th/th-apolda-scholz-recycling.md`.

Impressum und vollständig gelesenes Northdata-Profil HRB 733963 Ulm
kongruent, Registerpublikation 20.08.2026. Betreiber-Standortdatensatz
individuell gelesen: Adresse, Telefonnummer, Mail, Zeiten/Feiertage2026
und explizites Kartenpaar. Efb-PDF elf Seiten vollständig textuell gelesen,
alle drei Anlagen Apolda, ESN 98-040109(25), gültig bis 28.02.2027.
Bild-PDF Altfahrzeugbescheinigung zusätzlich visuell gelesen, gültig bis
03.03.2027; dort genannte Folgeprüfung September2026 nicht separat bestätigt.
Damit Kriterien der README-Betreiber-Primärquellen-Ausnahme erfüllt,
keine bloße Übernahme aus Verzeichnissen oder Feedback.

Services ausdrücklich für alle Scholz-eigenen Standorte: Kleinmengen,
Privat/Gewerbe, Tagesmarktpreisabrechnung nach Qualität/Gewicht; notwendige
Unterlagen und Gefahrenausschlüsse dokumentiert. Keine numerischen Preise,
keine kostenlose Fahrzeugrücknahme aus Zertifikat abgeleitet, kein
pauschaler Metall-/Fahrzeugabholdienst. Paar 51.0324081/11.5251902 aus
explizitem Betreiber-Standortpunkt im Dossier, kein Rust-Geocoding und
kein Produktions-SQL-Schreibzugriff.

Alle konkreten Quellen stehen im Dossier; keine neue Agentenwelle gegen
bekanntes Nutzungslimit gestartet. SRT-Pößneck/Saalfeld und übrige
Handler-/Händler-Leads bleiben offen, nicht durch diese Neuaufnahme erledigt.

## Gate

Neuanlage ohne Änderung von Bestandsdossiers. Slug/YAML/Enum/Root-Website,
Emdash-Platzhalter, Service-JSON, WGS84-Paar und datierte Timeline geprüft.
Seedcodegen plus acht Seedtests grün, null Fehler; Rustfmt und Diffcheck grün.
Gemeinsamer Commit, beide Release-Binaries und Produktions-Spotchecks.

## Deployment-Evidenz

Commit `9f71ac6`; beide Release-Binaries aus sauberem Commit gebaut,
atomar installiert. Neustart 08.10.2026 16:05:04 UTC, Bootseed eine Zeile
geschrieben. Read-only-Spotcheck bestätigt Apolda-Adresse, Aktivstatus,
Telefon/Mail, standortspezifische Zeiten, Servicebedingungen und exakt
51.0324081/11.5251902. 3879 Händler, 52 Materialien, 2280 Preise.
Öffentliche Health und installierter Read-only-Query-Worker grün.
Keine numerischen Preiszeilen aus dieser Aufnahme erzeugt.
Backups `/var/tmp/schrott-mcp-backup-before-9f71ac6/`.
Push erneut konkret an fehlendem HTTPS-Benutzernamen gescheitert.

SHA256 Server: `092c47bf2595e2d639e29e26ed96aa1e6a529776b4b30e6212149fa566d5e0d9`

SHA256 Worker: `c72d402b001a779dfc79f93e1972c0da4d86097789fdee0c62c13ce47cd99178`
