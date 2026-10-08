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
