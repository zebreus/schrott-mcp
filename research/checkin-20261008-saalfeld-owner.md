# Owner-Check-in — SRT Saalfeld, 08.10.2026

Service aktiv, sechs Stunden Warning-Journal ohne Einträge. Read-only vor
Deployment: 3880 Händler,52 Materialien,2280 Preise. Kein neues Feedback
seit #5517, letzte zehn Titel erneut gesichtet. Lauf1150 partial,
237 Preise/ein Fehler bei Schrott Anton; Lauf1149 ok. Direkter TLS-geprüfter
Abruf der exakten Preisquelle reproduziert curl35/TLS alert internal error,
HTTP000. Kein behaupteter Fix, kein -k und kein geratener Preisfallback;
genauere entfernte TLS-Ursache nicht bewiesen. Handlerstörung bleibt offen.

Offenes Feedback#5504 bearbeitet. Neuer SRT-Hof Ortsstraße33,07318 Saalfeld/
Aue am Berg, getrennt von Unterwellenborn und Pößneck. Individuellen
Betreiber-Datensatz live gelesen, aktuellen Registerbezug aus bereits
vollständig gelesener SRT-Quelle gegengeprüft. Drei Original-PDFs vollständig
gelesen: sechzehnseitiges Efb, Fahrzeugbescheinigung, zweiseitiges ElektroG.
Zwei unabhängige Urheberketten bestätigen Adresse/Standorttelefon.
Betreiberpaar50.673711/11.322541 im Dossier. Kein Produktions-SQL-Schreiben,
keine Rust-Geocodierung. Nur eine neue Betriebszeile, keine Alt-Slugs geändert.

Wichtiger Qualitätsfund: ElektroG-Bescheinigung deckt nur Kategorie4 ab,
ohne Photovoltaik-/Nachtspeichergeräte. Andere Kategorien dort nicht
zertifiziert; keine allgemeine Annahme-/Vergütungsbehauptung daraus.
Separates ElektroG-PDF und aktuelles Efb-PDF haben unterschiedliche
Vorgangsreferenzen; Klärbedarf ausdrücklich dokumentiert. Dokumente
bis31.03.2027, genannte Folgeprüfungen2026 nicht als durchgeführt bestätigt.
Mail/Zeiten nur Betreiber-Einzelbeleg, vorerst nur Timeline; persönliche
ElektroG-Ansprechpartnerkontakte nicht als generelle Firmenkontakte gefüllt.
Keine numerischen Kurse, Gebühren oder kostenlose Fahrzeugrücknahme erfunden.

Bekannte Agentenlimits nicht durch Wiederholungsaufrufe belastet;
weitere Händler-/Handler- und Koordinatenaufgaben bleiben im Backlog.

Owner-Gate: neue Datei, keine Bestandsüberschreibungen. YAML/Slug/Enums,
Root-Website, leere JSON statt ungesicherter Servicebedingungen, endliches
WGS84-Paar, Platzhalter und datierte Noten geprüft. Acht Seedtests,
Diffcheck und Rustfmt grün, null Testfehler.

## Deployment

Commit `199f760`, beide Release-Binaries aus sauberem Commit gebaut und
atomar installiert. Neustart 08.10.2026 17:34:47 UTC, Bootseed eine Zeile.
Read-only-Spotcheck bestätigt Ortsstraße33/07318, Telefon und explizites
Paar50.673711/11.322541 sowie ElektroG-Einschränkungen in Notizen;
Mail/Zeiten wie beabsichtigt leer. 3881 Händler,52 Materialien,2280 Preise.
Öffentliche Health und installierter Read-only-Query-Worker grün.
Keine numerischen Preise eingeführt. Backups
`/var/tmp/schrott-mcp-backup-before-199f760/`.
Push konkret erneut fehlgeschlagen: HTTPS-Benutzername nicht verfügbar.
Entfernten Schrott-Anton-TLS-Fehler nicht als behoben behauptet.

SHA256 Server: `43c8b1d33708ef24e08abc5c09d07fad0f7b7898aa44e77194175cf9009b2cd4`

SHA256 Worker: `c72d402b001a779dfc79f93e1972c0da4d86097789fdee0c62c13ce47cd99178`
