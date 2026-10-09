# AGH Altgoldhandel Handler — Owner status

## Quellen und Abgrenzung

- Preisquelle: <https://www.agh-goldankauf.de/wir-kaufen/altgoldankauf/>. Der servergerenderte `.goldrechner` enthält acht explizite Goldankauf-Feingehalte, EUR/g und den sichtbaren Stand 09.10.2026, 21:10 Uhr.
- Separates Betreiber-Impressum: <https://www.agh-goldankauf.de/impressum/>. Name, Ansprechpartner, Adresse und USt-ID werden bei jedem Scrape gegengeprüft.
- Die benachbarten Goldbarrenpreise (Ankauf/Verkauf) und der separat geladene Ankaufsrechner werden nicht mit Altgoldkursen vermischt. Dessen Werte weichen teils von der Altgoldtabelle ab.
- Goldfeinheiten bleiben als `gold`-Varianten getrennt; kein Preisdatum aus dem Beobachtungstag abgeleitet.

## Stand

- Branch `owner/agh-recovery-20261009`, isoliert von `main`.
- Händlerdossier hat den aktuellen Quellenbefund als Timeline-Eintrag erhalten; Frontmatter unverändert.
- `cargo test --locked -p schrott-mcp-ingestion agh_altgoldhandel`: vier Tests bestanden.
- Rust-Livehandler erfolgreich: HTTP 200, 233460 Bytes, acht Goldfeinheiten,
  EUR/g, exakte Preise, sichtbares Preisdatum 09.10.2026, keine Skips.
  Gold 999 beim Abruf 113,11 EUR/g; Gold 333 33,39 EUR/g.
- Workspace-Gate und Main-Integration bleiben offen. Kein Deployment oder
  Produktionsdatenbank-Write erfolgt; Ownership bleibt bis zum regulären
  erfolgreichen Prod-Scrape offen.
