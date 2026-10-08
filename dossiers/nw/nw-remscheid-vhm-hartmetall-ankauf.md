---
slug: nw-remscheid-vhm-hartmetall-ankauf
name: VHM Hartmetall Ankauf
trader_type: sonstige
state: NW
city: Remscheid
street: Grünenplatzstraße 1a
postcode: '42899'
phone: ''
email: info@vhm-hartmetall.de
opening_hours: ''
website: https://www.vhm-hartmetall.de
website_status: ''
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: (1) Behaltene Neueinträge
provenance_ankauf_raw: ja
provenance_origin: table
---

# VHM Hartmetall Ankauf

## Überblick

Hartmetall-Ankauf mit vier veröffentlichten EUR/kg-Preisen und Hartmetallschlamm nach Analyse. Die HTML-Preiskarten enthalten nur Platzhalter; aktuelle Preise und Quell-Datum kommen aus der vom Website-Script geladenen JSON-API.

## Timeline

### 2026-10-08 — Preisquelle korrigiert (Audit #4784)

- Preis-Seite: https://www.vhm-hartmetall.de/aktueller-hartmetall-preis
- Verifiziertes Script: https://www.vhm-hartmetall.de/script.js?v=vhm-preise-20260928-1 — lädt `/vhm-preise-aktuell.php?_=` mit `Date.now()` und `cache: "no-store"`. Fehler setzen die Website-Preise zurück; ausdrücklich kein Rückfall auf alte 65/63 EUR/kg.
- Live-API: https://www.vhm-hartmetall.de/vhm-preise-aktuell.php — HTTP 200, `ok: true`, `fields.einheit: "alle Preise pro kg"`, `fields.marktHinweis: "Aktuelle Ankaufspreise pro kg bei passender Sorte und sauber sortiertem Material"`.
- Quell-Zeitstempel: `updatedAt: "2026-10-05T08:51:01+00:00"`; Handler übernimmt `published_at: "2026-10-05"`, nicht das Abrufdatum.
- Revision: `b850f7af0b621ab51967ade0ecdb6f69f2714b5146e4e3b3cb41fb3b611ff49a`.

| API-Schlüssel | Stabile bisherige Variante (`hartmetall`) | Preis EUR/kg |
| --- | --- | ---: |
| `wendeschneidplatten` | `Wendeschneidplatten` | 53,00 |
| `hartmetallGemischt` | `gemischt` | 50,00 |
| `vhmFraeserBohrer` | `VHM-Fräser & Bohrer` | 53,00 |
| `widia` | `Widia` | 50,00 |
| `hartmetallschlamm` | Annahmebedingungen `Schlamm` | kein Festpreis: nach Analyse |

- API-Struktur: pro Sorte `text`, `amount`, `schemaAmount`; bei vier Preisen stimmen alle drei Darstellungen überein. Schlamm hat ausdrücklich `amount: null` und `schemaAmount: null`.
- Handler lädt ausschließlich die API mit Cache-Buster und No-Store-Header. Fehlendes/ungültiges JSON, `ok: false`, fehlende Sorten/Felder, unbekannte Einheit, widersprüchliche oder nicht positive Zahlen, fehlendes Quell-Datum und unerwarteter Schlamm-Festpreis führen zu Fehlern, nicht Ersatzpreisen. Zusätzliche API-Sorten werden laut übersprungen. Impressum-Enrichment bleibt unverändert.
- Regressionen zuerst rot ausgeführt; Tests prüfen Preisvarianten, Quell-Datum, Schlamm ohne Fantasiepreis, Fehlerfälle und veränderte API-Zahlen ohne hartcodierte Rückfallpreise. Read-only Live-Test ruft exakt `(handler().scrape)(&client)` auf, ohne `record()`/Store.
- Reproduktion: `cargo test -p schrott-mcp-ingestion vhm_hartmetall --lib -- --include-ignored --nocapture`; Formatprüfung: `rustfmt --edition 2021 --check crates/ingestion/src/traders/handlers/vhm_hartmetall.rs`.
- Ergebnis: 6/6 Handler-Tests grün einschließlich exaktem Livehandler (HTTP 200, 679 Bytes, vier Preise und eine Annahme); `rustfmt --check` und `git diff --check` grün.
- Kein Commit, Deployment oder Produktions-Schreibzugriff. Bereits gespeicherte alte Current-Werte wurden in dieser Arbeit nicht verändert; die Korrektur betrifft die nächste reguläre Ingestion.

### Importiert (Seed-Stand 2026-09-30)

- Hartmetall/VHM/Widia/Wendeschneidplatten, €/kg-Preise. PREISLISTE: https://www.vhm-hartmetall.de/aktueller-hartmetall-preis
- Adresse: Remscheid (Ankauf in Remscheid/NRW/bundesweit)
- Adressbeleg: https://www.vhm-hartmetall.de
