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
certifications: ''
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

### Recherche 05.11.2026

- [Recherche 05.11.2026: **Für den Dossierstandort ist im amtlichen Fachbetrieberegister kein Eintrag gefunden worden**; die belegte Negativaussage mit Reichweite steht in diesem Vermerk; das Feld `certifications` bleibt leer — dort stehen nach der Hauskonvention Zertifikate, keine Negativbefunde. **Entsorgungsfachbetrieb § 56 KrWG:** kein Eintrag im amtlichen Fachbetrieberegister für den Standort Grünenplatzstraße 1a, 42899 Remscheid (Abfrage 05.11.2026). **AltfahrzeugV:** kein Eintrag im amtlichen Fachbetrieberegister für den Standort Grünenplatzstraße 1a, 42899 Remscheid (Abfrage 05.11.2026). **Einträge derselben Standortabfrage (3 km) - keiner an der Dossier-Adresse und keiner unter dem Dossiernamen:** „Betriebsfläche“ (ZZET00800051 / 2), Grünenplatzstr. 4, 42899 Remscheid; „Betriebsfläche“ (ZZET00800051 / 3), Grünenplatzstr. 4, 42899 Remscheid; „Birgit Wüster Schrotthandel“ (ZZET00800051 / 1), Kleinsporkert 1, 42287 Wuppertal; „Tamm GmbH“ (ZZET00800044 / 4), Lüttringhauser Str.. 163, 42369 Wuppertal; „Tamm GmbH“ (ZZET00800044 / 1), Gasstr. 11, 42369 Wuppertal; „Tamm GmbH“ (ZZET00800044 / 2), Gasstr. 11, 42369 Wuppertal; „Tamm GmbH“ (ZZET00800044 / 3), Gasstr. 11, 42369 Wuppertal; „Seibel Entsorgungs GmbH & Co. KG“ (ZZFE00100117 / 4), Otto-Hahn-Straße 85, 42369 Wuppertal. **Reichweite der Registerabfrage (Abruf 05.11.2026):** geprüft wurden im amtlichen Fachbetrieberegister beide Routen („Entsorgungsfachbetriebe“ und „Altfahrzeugverwertung“) mit dem Filter „Art des Betriebs = Egal“ (`is=1`, `e=1`) und dem Suchradius `dst=10`; Namenssuche Route „Entsorgungsfachbetriebe“: `n=VHM` -> **0** Treffer, `n=Hartmetall` -> **0** Treffer, `n=Remscheid` -> **7** Treffer; Namenssuche Route „Altfahrzeugverwertung“: `n=VHM` -> **0** Treffer, `n=Hartmetall` -> **0** Treffer, `n=Remscheid` -> **0** Treffer; Standortsuche um die Dossier-Adresse (validierte PLZ-Abfrage `adr=`) Route „Entsorgungsfachbetriebe“: **3 km** -> **9** Treffer, **5 km** -> **13** Treffer, **10 km** -> **71** Treffer; Route „Altfahrzeugverwertung“: **3 km** -> **0** Treffer, **5 km** -> **0** Treffer, **10 km** -> **3** Treffer. **Ein Register-Nichteintrag ist kein Nachweis, dass ein Betrieb keine Anerkennung besitzt**; die Aussage gilt nur für die am 05.11.2026 geprüften Namensvarianten, Radien und Filter. Die Angaben stammen ausschließlich aus dem amtlichen Fachbetrieberegister (https://fachbetrieberegister.gadsys.de/fachbetrieberegister/) und dem dort hinterlegten Zertifikats-PDF; der Dritt-Spiegel schrottregister.pages.dev wurde nicht als Beleg verwendet.
    Quelle(n): https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=VHM ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=VHM ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Hartmetall ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Hartmetall ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Remscheid ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Remscheid ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=42899%20Remscheid,%20Deutschland;7.242890;51.217207;7.38630588;51.30703782;7.09947492;51.12737558&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=42899%20Remscheid,%20Deutschland;7.242890;51.217207;7.38630588;51.30703782;7.09947492;51.12737558&dst=3&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=42899%20Remscheid,%20Deutschland;7.242890;51.217207;7.38630588;51.30703782;7.09947492;51.12737558&dst=5&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=42899%20Remscheid,%20Deutschland;7.242890;51.217207;7.38630588;51.30703782;7.09947492;51.12737558&dst=5&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/adr=42899%20Remscheid,%20Deutschland;7.242890;51.217207;7.38630588;51.30703782;7.09947492;51.12737558&dst=10&is=1&e=1 ; https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=42899%20Remscheid,%20Deutschland;7.242890;51.217207;7.38630588;51.30703782;7.09947492;51.12737558&dst=10&is=1&e=1 (Abruf 05.11.2026).]
