# Owner-Check-in 08.10.2026 — Wolf Straubing

Dienst aktiv, HTTPS-Health ok, keine Journalwarnungen seit sechs Stunden.
3881 Händler / 52 Materialien / 2280 Current-Preise. Ingestion1157 ok,
19 Preise übernommen, null Fehler;1156 ebenfalls ok. Zehn jüngste
Feedbacktitel erneut gelesen, maximal5517 unverändert; offene
Autoverwertungs-Leads bleiben unverifiziert. Deploy-Worktree anfangs sauber.

Offenen Koordinatenpunkt aus Welle C abgearbeitet: Betreiber-Impressum
bestätigt weiter Wolf Entsorgung GmbH & Co. KG, HRA7265, Röntgenstraße11,
94315Straubing. Strukturierte Adresssuche liefert OSMway217805990 mit
48.8890789/12.625867. Originalelement direkt geprüft: Firmenname,
Straße/Hausnummer/PLZ/Stadt, Telefon und Website passen. Geometrie ist
ein Gebäudepunkt, nicht die behauptete öffentliche Anlieferungseinfahrt.
Abweichende OSM-Öffnungszeiten nicht importiert; Betreiberzeiten behalten.
Historisches DB-Paar48.8813/12.5739 war nicht adressverifiziert.
Keine direkten DB-Schreibzugriffe; explizites Dossierpaar wird über Seed
übernommen. Keine Runtime-Geokodierung in Rust, keine neue Preiszusage.
ECOPROEKT-Prüfpunkt bleibt offen und betrifft eine Geschäftsadresse,
keinen belegten öffentlichen Schrottplatz.

Quellen/Abrufparameter vollständig in der Dossiertimeline dokumentiert.

Owner-Gate grün: alle bestehenden Frontmatter-Felder/Timeline-Bullets
erhalten, nur explizites endliches WGS84-Paar ergänzt; Root-Website,
Enums/Slug/Noten und Platzhalter geprüft. Acht Seedtests bestanden,
null Fehler; Rustfmt und Diffcheck grün.

## Veröffentlichung bestätigt

Commit `adc73fb`, beide Release-Binaries aus sauberem Commit gebaut und
atomar installiert; Neustart08.10.2026 20:00:44UTC. Bootseed aktualisiert
eine Zeile. HTTPS-Health grün, installierter Read-only-Worker bestätigt
Röntgenstraße11 und exakt48.8890789/12.625867, Status unverändert aktiv.
Backups `/var/tmp/schrott-mcp-backup-before-adc73fb/`.
Push erneut konkret gescheitert: HTTPS-Benutzername nicht verfügbar.

SHA256 Server: `67de1e9e49ce12ec890cf6d5d6f05c6107e92463a9648c1d0dc343c9bbea824d`

SHA256 Worker: `c72d402b001a779dfc79f93e1972c0da4d86097789fdee0c62c13ce47cd99178`

## Weitere Check-in-Nachrichten während des Abschlusses

Neuprüfung um20:00UTC: Feedbackmaximum5562 statt5517, also45 neue Hinweise.
Zehn jüngste Einträge5553–5562 einschließlich Details vollständig gelesen:
Jacob-PDF, SMS/DB-Rochlitz-Identität/Abdeckung, Saxonia-An-/Verkauf,
SDM-Standort-/Sortenfragen, MKM-Abdeckung, MSG-Staffeln,
Kiro/Gouchev/Bauer fehlende Sorten. Hinweise sind Leads, noch kein
pauschaler Bug-/Fixnachweis. Ingestion1161/1160 beide ok,26/60 Preise,
null Händlerfehler. Die früheren Einträge5518–5552 bleiben weiterer
Feedback-Sichtungsbedarf, nicht als erledigt verbucht.

Drei Hintergrund-Diagnoseaufträge mit methodischem Freiraum gestartet:

| Thema | Session | Bericht |
| --- | --- | --- |
| MKM #5557 | ses_ee2e43f32ffe3STo3RBEatNkmW | research/checkin-20261008-mkm-price-coverage.md |
| MSG #5558 | ses_ee2e43f0fffeYsmBRJ458byr6C | research/checkin-20261008-msg-price-tiers.md |
| SDM #5556/#5562 | ses_ee2e43ef8ffejT77FyxBT9EuSx | research/checkin-20261008-sdm-price-coverage.md |

Quellen/Identität/Preissemantik und tatsächlichen Handleroutput prüfen,
Produktionsdatenbanken read-only. Noch keine bestätigten Handlerfixes;
gemeinsamer Gate/Codeänderungen erst nach reproduzierbaren Befunden.
