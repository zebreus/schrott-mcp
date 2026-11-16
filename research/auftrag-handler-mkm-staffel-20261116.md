# Auftrag: MKM-Staffel-Vertrag revidieren (Seitenumbau 15.11.2026)

**Stand:** 16.11.2026, 07:45 UTC (Owner-Check-in). Handler `mkm_metals`
(`rp-ludwigshafen-mkm-metals`) ist in Produktion **rot** und bleibt es bis
zur Vertragsrevision + Release.

## Befund (live verifiziert, read-only)

Der Seitenumbau vom 15.11.2026 (Cache-Bust `gesamt.min.css?v=1794748841`,
CSS-Bytes unverändert, sha256 `cf1b76e3…`) hat **zwei** Änderungen:

1. Stylesheet-Referenz — bereits nachgezogen in Commit `8a005e29`
   (Hebt nur das erste Gate; der Handler läuft jetzt weiter in den
   Artikel-Vertrag und scheitert dort).
2. **Die `data-staffel`-Inputs fehlen jetzt für 9 bepreiste Artikel.**
   Live-Zählung: 47 Zeilen mit `data-staffel`, 14 ohne — davon 5
   „auf Anfrage" (korrekt, kein Preis) und **9 mit Preis aber ohne Tiers**:

   | Artikel | Label | Live-Preis | Minimum | Produktdatum |
   |---|---|---|---|---|
   | SW10040 | Zinn 99% | 27,30 €/kg | ab 100 kg | 2026-11-11 |
   | SW10039 | Zinn 90% | 22,05 €/kg | ab 100 kg | 2026-11-11 |
   | SW10042 | Zinn 60% | 10,50 €/kg | ab 250 kg | 2026-11-11 |
   | SW10041 | Zinn 30% | 5,25 €/kg | ab 500 kg | 2026-11-11 |
   | MKM-HM-01 | Hartmetall gemischt | 50,20 €/kg | ab 50 kg | 2026-11-15 |
   | SW10037 | VHM Bohrer | 50,20 €/kg | ab 30 kg | 2026-11-15 |
   | SW10036 | Widia Wendeschneidplättchen | 50,20 €/kg | ab 80 kg | 2026-11-15 |
   | MKM-HM-02 | Hartmetall Stückschrott | 38,85 €/kg | ab 50 kg | ~2026-11-09 |
   | SW10038 | HSS Schnellarbeitsstahl | 1,26 €/kg | ab 500 kg | 2026-11-11 |

   Fehlermeldung dann: `SW10040: tiers disagree with visible upper
   bound/quantity` (overview.rs, Tier-Konsistenz-Gate).

## Warum das keine Kleinigkeit ist

Der Vertrag wird **zweifach** erzwungen: `overview.rs` (Tier-JSON muss
mit sichtbarem Preis/Minimum übereinstimmen, `data-price`-Basispreis)
und `recording.rs` (eigene Tier-Prüfung, Snapshot-JSON `article.tiers`
in `extra_json`, reviewed Oracles, typed60-Historien-Harness). Eine
Revision muss beide Stellen plus Fixtures/Oracles anfassen und die
Recherchehistorie erhalten (keine Ersatzwerte raten).

## Auftrag

1. Neue sanitisierte Live-Fixtures (Overview + Impressum, Stand
   16.11.2026) als reviewed Capture anlegen — Muster:
   `fixtures/mkm-overview-20261018-sanitized.html`.
2. Vertrag explizit erweitern: bepreiste Artikel **ohne** Tier-Input
   werden als flacher „upto"-Preis mit sichtbarem Minimum akzeptiert
   (genau die 9 Artikel oben, fail-closed für alle anderen Formen).
   Tier-Pfad bleibt unverändert für die 47 Artikel mit Staffeln.
3. `recording.rs`-Vertrag, Oracles und typed60-Harness mitziehen;
   Historie darf nicht verloren gehen (Legacy-Rückzüge nur mit Beweis).
4. Tests: neue Fixtures + Mutationstests (Tier-lose Zeile mit Preis 0,
   mehrdeutig, „auf Anfrage" mit Tiers muss weiter rot sein), dann
   `cargo test -p schrott-mcp-ingestion` und Live-Lauf
   `cargo run -p schrott-mcp-ingestion --example live_handlers rp-ludwigshafen-mkm-metals`.
5. Owner-Gate, Commit, Push; Release erst mit grüner Pre-Deploy-Freigabe
   für exakt den geprüften Commit (siehe prompts/hourly-checkin.md).

## Kontext

- Produktionsfehler seit 15.11. ~17:24 UTC zuerst „stylesheet reference
  or media scope changed", nach dem Pin-Update (noch nicht deployed)
  wäre es der Tier-Vertrag.
- Die alten Hotfix-Branches (`hotfix/mkm-61-*`, Oktober) sind **Historie**:
  der 61-Artikel-Vertrag ist bereits in rel12. Nicht cherry-picken.
- Stash `stash@{0}` („preserve candidate rustfmt and Wahl fix") stammt
  aus diesem alten Kontext und wurde am 16.11. nicht angefasst.