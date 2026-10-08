# Owner-Check-in — SRT Pößneck, 08.10.2026

Service aktiv, Warning-Journal sechs Stunden leer. Read-only-Counts vor
Deployment 3879/52/2280. Keine neuen Feedbacks seit #5517, letzte zehn
erneut gesichtet. Lauf1148 partial: 82 Preise, ein Strausberg-Netzwerkfehler.
Direkter HTTPS-Abruf der exakten Preisquelle mit TLS-Prüfung jetzt HTTP200;
das beweist aktuelle Erreichbarkeit, nicht schon grünen tatsächlichen
Handler-/Produktionslauf. Keine Parseränderung ohne Reproduktion.
NORDKAT letzter regulärer Schritt weiterhin alter404/Lauf1126.

Offenes dedupliziertes Paar #5070/#5505 bearbeitet. Neuaufnahme genau eines
SRT-Pößneck-Standorts, kein bestehender Pößneck-/07381-/Telefonmatch;
Unterwellenborn bleibt eigener Standort. Vollständigen Northdata-Firmenblock
und individuelle Betreiberdaten gelesen, achtseitiges ESN-Efb-PDF und
separate Altfahrzeugbescheinigung vollständig gelesen. Aktuelles Register
HRB506605, Gesellschaftssitz Saalfeld nicht Pößneck. Urkunden bis31.03.2027.

Standort/Telefon in zwei unabhängigen Belegketten (Betreiber/ESN) bestätigt.
Explizites Betreiberpaar 50.690337/11.58344 im Dossier, keine Runtime-GEO.
Hausnummer fehlt in qualifizierten Quellen: keine Portalnummer0/31.
E-Mail/Zeiten nur Betreiber-Einzelbeleg, deshalb nur Timeline; leeres
Samstagsfeld nicht als geschlossen ausgelegt. Allgemeine Scholz-Regeln
nicht pauschal auf eigenständige SRT-Firma übertragen. Konkrete private
Annahme-/Vergütungs-/Abholbedingungen und numerische Preise weiter offen.
Quellen vollständig im neuen Dossier, keine Direktänderung der Datenbanken.

Bekannte Agenten-Nutzungslimits nicht mit Retry-Schleifen belastet;
Saalfeld und andere Händler-/Handlerfälle bleiben im Backlog. AMR-
Quellstörung aus vorherigem Check-in nicht als behoben behauptet.

Owner-Gate: neue Datei, keine Bestandsüberschreibungen. YAML/Slug/Enums,
Root-Website, leere Service-JSON statt ungesicherter Bedingungen, endliches
gepaartes WGS84, Emdash-Platzhalter und datierte Noten geprüft.
Diffcheck/Rustfmt grün, acht Seedtests bestanden, null Fehler.

## Deployment

Commit `02502e9`, beide Release-Binaries aus sauberem Commit gebaut und
atomar installiert. Neustart 08.10.2026 17:05:26 UTC, Bootseed eine Zeile.
Read-only-Spotcheck bestätigt Standort ohne erfundene Hausnummer, Telefon,
Paar50.690337/11.58344 und bewusst leere Mail/Zeiten. 3880 Händler,
52 Materialien, 2280 Preise. Health und installierter Query-Worker grün.
Backups `/var/tmp/schrott-mcp-backup-before-02502e9/`.
Push erneut tatsächlich fehlgeschlagen: fehlender HTTPS-Benutzername.
Keine numerischen Preise eingeführt, keine weiteren Standorte als erledigt
behauptet und kein tatsächlicher Strausberg-Handlererfolg vorgetäuscht.

SHA256 Server: `f3d1ec77c5cd926c3c72c470a00750f067f7800533485687ae7cb570ce5ef59e`

SHA256 Worker: `c72d402b001a779dfc79f93e1972c0da4d86097789fdee0c62c13ce47cd99178`
