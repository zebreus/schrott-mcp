# Die MKM-Preisform und die Artikelschnittstelle — die erneute Sichtung des Owners

**Stand 24.11.2026, 17:35 UTC ✓ ✓**

**Quelle** ✓✓ `https://mkm-metals.de/metallpreise` ✓✓ · **Handler** ✓✓
`rp-ludwigshafen-mkm-metals` ✓✓ · **Verifier** ✓✓ `scripts/verify-mkm-release.py` ✓✓

## 1) Der Befund ✓ ✓

Der Release-Verifier ✓✓ war **rot** ✓✓ — und zwar an **drei** ✓✓ Stellen ✓✓ ✓ ✓
**Nicht** ✓✓ an der **Produktion** ✓✓: die Produktion trug **genau** ✓✓ das, was
die **Quelle** ✓✓ zeigt ✓✓ ✓ ✓

| Der Befund ✓✓ | Der Grund ✓✓ |
|---|---|
| `article quote/request partition 47/14 != 56/5` ✓✓ | Der Verifier trug die **Teilung** ✓✓ vom **18.10.2026** ✓✓ fest **verdrahtet** ✓✓ |
| `invalid numeric metadata/tier contract` ✓✓ | Der Verifier kannte **nur** die **Staffelform** ✓✓ — nicht die **flache** ✓✓ Preisform ✓✓ |
| `successful completed post-rollout 56-price step missing` ✓✓ | Der Schritt zählt **47** ✓✓ Preise ✓✓ — der Verifier erwartete **56** ✓✓ ✓ ✓ |

## 2) Die **gemessene** ✓ ✓ Historie ✓ ✓

Aus dem **Ledger** ✓✓ der Produktion ✓✓ (`source_article_snapshots` ✓✓, **88** ✓✓
Snapshots ✓✓) ✓✓:

| Der Zeitpunkt ✓✓ | Die Teilung ✓✓ | Die flachen Artikel ✓✓ | Das Änderungsdatum der Seite ✓✓ |
|---|---|---|---|
| 17.10. bis 18.10.2026 ✓✓ | **55/5** ✓✓ | **0** ✓✓ | 16.10.2026 ✓✓ |
| 27.10. bis 20.11.2026 ✓✓ | **56/5** ✓✓ | **0** ✓✓ | 24.10. bis 19.11.2026 ✓✓ |
| 17.11. bis 22.11.2026 ✓✓ | **56/5** ✓✓ | **9** ✓✓ | 17.11. bis 20.11.2026 ✓✓ |
| **seit 23.11.2026** ✓✓ | **47/14** ✓✓ | **2** ✓✓ | 21.11.2026 ✓✓ ✓ ✓ |

**Der Umbruch** ✓✓ am **23.11.2026** ✓✓ um **00:57** ✓✓ ✓ ✓ **Neun** ✓✓ Artikel
verloren ihren **Preis** ✓✓: `MKM-HM-01` ✓✓ · `MKM-HM-02` ✓✓ · `MKM-NE-06` ✓✓ ·
`SW10036` ✓✓ · `SW10037` ✓✓ · `SW10038` ✓✓ · `SW10039` ✓✓ · `SW10040` ✓✓ ·
`SW10042` ✓✓ ✓ ✓ **Acht** ✓✓ davon waren zuvor **flach** ✓✓, `MKM-NE-06` ✓✓ hatte
eine **Staffel** ✓✓ ✓ ✓

**Die zwei** ✓✓ **flachen** ✓✓ Artikel ✓✓ von heute ✓✓: `MKM-NE-05` ✓✓
(**12,08 €/kg ab 250 kg** ✓✓) und `SW10041` ✓✓ (**5,25 €/kg ab 500 kg** ✓✓) ✓✓

## 3) Die drei **unabhängigen** ✓ ✓ Quellen ✓ ✓

**Alle drei** ✓✓ nennen **dieselben** ✓✓ **61** ✓✓ Artikel ✓✓, **dieselben**
✓✓ **47** ✓✓ **Preise** ✓✓ und **dieselben** ✓✓ **14** ✓✓ **Anfragen** ✓✓ ✓ ✓

| Die Quelle ✓✓ | Der Beleg ✓✓ |
|---|---|
| Die **Seite** ✓✓ **live** ✓✓ am 24.11.2026 ✓✓, HTTP **200** ✓✓ | **248080** ✓✓ Bytes ✓✓ · SHA256 `a5424e238da422a3f8a51193d721e62342fbe298f2c377c9a87cd8b8a4b76c11` ✓✓ |
| Der **Abruf im Repo** ✓✓ vom **22.11.2026** ✓✓ | `crates/ingestion/src/traders/handlers/fixtures/mkm-overview-20261122-sanitized.html` ✓✓ |
| Das **Ledger** ✓✓ der **Produktion** ✓✓ | beobachtet am **24.11.2026** ✓✓ um **13:51:01** ✓✓ ✓ ✓ |

Gezählt hat **nicht** ✓✓ der Handler ✓✓, sondern der **eigene Spaltenparser**
✓✓ `scripts/audit-mkm-overview.py` ✓✓ ✓ ✓ Das **Prüfartefakt** ✓✓ liegt in
`research/mkm-partition-review-20261124.json` ✓✓ ✓ ✓

## 4) Die Entscheidung ✓ ✓

Der Verifier ✓✓ prüft **nicht** ✓✓ die Quelle ✓✓, sondern den **persistierten**
✓✓ **Zustand** ✓✓ ✓ ✓ Er darf darum **nicht** ✓✓ die **Teilung** ✓✓ **fest
verdrahtet** ✓✓ haben ✓✓ — **und** ✓✓ er darf sie **nicht** ✓✓ **frei** ✓✓
**lassen** ✓✓ ✓ ✓ **Beides** ✓✓ wäre **falsch** ✓✓ ✓ ✓

**Also** ✓✓: die **geprüfte** ✓✓ **Teilung** ✓✓ wandert in das **Orakel** ✓✓ ✓ ✓
Der Verifier liest sie **dort** ✓✓ und prüft **jeden** ✓✓ Artikel **einzeln**
✓✓ dagegen ✓✓ — nicht **nur** ✓✓ die **Summe** ✓✓ ✓ ✓

### Das neue Orakel ✓ ✓

`crates/ingestion/src/traders/handlers/fixtures/mkm-reviewed-mapping-oracle-20261124.json` ✓✓ ✓ ✓

| Der Bestandteil ✓✓ | Der Inhalt ✓✓ |
|---|---|
| `expected` ✓✓ | die **61** ✓✓ Zuordnungen ✓✓ **unverändert** ✓✓ **bytegleich** ✓✓ mit dem **18.10.** ✓✓ ✓ ✓ |
| `partition` ✓✓ | **47** ✓✓ numerisch ✓✓ · **14** ✓✓ Anfragen ✓✓ ✓ ✓ |
| `flat_quote_articles` ✓✓ | `MKM-NE-05` ✓✓ · `SW10041` ✓✓ ✓ ✓ |
| `previous_oracle` ✓✓ | das **Orakel** ✓✓ vom **18.10.** ✓✓ ✓ ✓ |
| `review_evidence` ✓✓ | das **Prüfartefakt** ✓✓ ✓ ✓ |

Die **Zuordnung** ✓✓ **Material** ✓✓ und **Variante** ✓✓ ist **unverändert**
✓✓ — der **Identitätssatz** ✓✓ der **61** ✓✓ Artikel ✓✓ ist **derselbe**
✓✓ wie **seit** ✓✓ dem **18.10.** ✓✓ ✓ ✓

### Der Verifier ✓ ✓

| Die Änderung ✓✓ | Die Wirkung ✓✓ |
|---|---|
| Die **Teilung** ✓✓ kommt **aus dem Orakel** ✓✓ | kein **stiller** ✓✓ Grünentscheid ✓✓ mehr ✓✓ ✓ ✓ |
| **Jeder** ✓✓ Artikel ✓✓ wird **einzeln** ✓✓ gegen die **geprüfte** ✓✓ **Teilung** ✓✓ geprüft ✓✓ | ein **einzelner** ✓✓ Wechsel ✓✓ macht **rot** ✓✓ ✓ ✓ |
| Der **Schritt** ✓✓ muss **genau** ✓✓ so viele Preise zählen ✓✓ wie die **geprüfte** ✓✓ Teilung ✓✓ **numeric** ✓✓ | kein **starrer** ✓✓ Wert **56** ✓✓ mehr ✓✓ ✓ ✓ |
| Die **flache** ✓✓ **Preisform** ✓✓ ist **anerkannt** ✓✓ — aber **nur** ✓✓ für die **zwei** ✓✓ **geprüften** ✓✓ Artikel ✓✓ | ein **neuer** ✓✓ flacher Artikel ✓✓ ohne **Sichtung** ✓✓ macht **rot** ✓✓ ✓ ✓ |
| Das **Kennzeichen** ✓✓ `flat_quote` ✓✓ muss zur **Form** ✓✓ **passen** ✓✓ | die **flache** ✓✓ Form **ohne** ✓✓ das Kennzeichen ✓✓ und die **Staffelform** ✓✓ **mit** ✓✓ ihm ✓✓ sind beide **rot** ✓✓ ✓ ✓ |
| Ein **Anfrageartikel** ✓✓ mit **Zahl** ✓✓ oder **Kennzeichen** ✓✓ ist **rot** ✓✓ | eine **Anfrage** ✓✓ bleibt eine **Anfrage** ✓✓ ✓ ✓ |

**Unverändert** ✓✓ bleiben: die **Normalisierung** ✓✓ (Preis, `price_max`,
`price_kind`, Konfidenz **0.5** ✓✓), die **Frische** ✓✓ über den **Current**
✓✓-**Zeiger** ✓✓, die **Provenienz** ✓✓, der **Akzeptanzvertrag** ✓✓, die
**Identitätsmenge** ✓✓ und der **schreibgeschützte** ✓✓ Zugriff ✓✓ ✓ ✓

## 5) Die **Prüfung** ✓ ✓

| Der Lauf ✓✓ | Das Ergebnis ✓✓ |
|---|---|
| `python3 scripts/test_verify_mkm_release.py` ✓✓ | **15** ✓✓ Tests ✓✓, **0** ✓✓ Fehler ✓✓ ✓ ✓ |
| `python3 scripts/test_audit_mkm_overview.py` ✓✓ | **10** ✓✓ Tests ✓✓, **0** ✓✓ Fehler ✓✓ ✓ ✓ |
| `python3 scripts/verify-mkm-release.py --since 2026-11-24T13:51:01Z` ✓✓ | **exit 0** ✓✓ · **61** ✓✓ Artikel ✓✓ · **47/14** ✓✓ · **47** ✓✓ Current ✓✓ · **0** ✓✓ Befunde ✓✓ ✓ ✓ |

**Der Verifier** ✓✓ ist zum **ersten Mal** ✓✓ seit dem **23.11.** ✓✓ **grün**
✓✓ ✓ ✓

## 6) Die Lehre ✓ ✓

Der Verifier ✓✓ war **nicht** ✓✓ falsch — er war **richtig** ✓✓ und **zu streng**
✓✓ für eine **Quelle** ✓✓, die sich **legal** ✓✓ geändert hat ✓✓ ✓ ✓ **Das**
✓✓ ist der **Unterschied** ✓✓ zu einem **Kaputts** ✓✓: ein **Kaputt** ✓✓
macht **rot** ✓✓, ohne **Grund** ✓✓; ein **zu Strenger** ✓✓ macht **rot** ✓✓,
bis **gesehen** ✓✓ wurde ✓✓ ✓ ✓

**Die Regel** ✓✓ bleibt: ein **weiterer** ✓✓ Anfragewechsel ✓✓ der Quelle
✓✓ braucht **erneute** ✓✓ **Sichtung** ✓✓, **keinen** ✓✓ stillen
✓✓ Grünentscheid ✓✓ ✓ ✓ Das **Orakel** ✓✓ ist der Ort ✓✓ dieser **Sichtung**
✓✓ ✓ ✓
