# Wave-E-Pin-Revision vom 27.10.2026 (Owner-Freigabe)

## Anlass

Der release-pinned Wave-E-Verifier prüft die elf Skalare des ursprünglichen Wave-E-Scopes
(60 Zeilen aus der Welle vom 11.10.2026) als **Regressionsschutz**: die Werte dürfen sich
ohne ausdrückliche Owner-Revision nicht still ändern. Die 6h-Welle vom 27.10.2026, 18:00
hat **eine** dieser 60 Zeilen verbessert:

`he-nidderau-ab-recycling-bjoern-asal` (Shard A der 18:00-Welle)

Der Wave-E-Pin (Basis `scripts/wave-e-6b3da663-seed-expectations.json`, Werte-Stand
11.10.2026)expects für diese Zeile `street: ""` und `postcode: ""` — der Owner-Review vom
11.10.2026 hatte die Anschrift damals **nicht** als belegt bewertet und die Felder deshalb
leer gelassen.

## Beleglage der neuen Werte (Recherche 27.10.2026)

| Feld | Neuer Wert | Beleg |
|---|---|---|
| `street` | Saalburgstr. 12 | Betreiber-Impressum (`ab-recycling.de/impressum/`) + unabhängiger WLW-Eintrag (derselbe Betrieb, dieselbe Anschrift) |
| `postcode` | 61130 | Betreiber-Impressum + WLW |
| `phone` | 06187 470 88 85 | Betreiber-Impressum („Telefon: 06187-470 88 85“) |
| `email` | abrecycling@gmx.de | Betreiber-Impressum |
| `opening_hours` | Mo-Fr 08:00-18:00 | Betreiber-Kontaktseite („Mo-FR von 8.00-18.00 uhr“) |
| `description` | erweitert | Betreiber-Leistungsseite (Ankauf Schrott/NE-Metall/Kabel, Verwertung von Altfahrzeugen mit Nachweis, keine Gebrauchtteile, Container 3–40 m³, Entsorgung/Containerstellung/Transporte ohne bezifferte Tarife) |

**Unverändert bleiben:** `name` („A-B-Recycling (Björn Asal)“), `status` (`pruefung` — das
Impressum nennt keine Rechtsform und keine Registernummer, ein amtlicher Betriebsbeleg fehlt),
`trader_type` (`schrotthaendler`), `website` und `website_status`. Der Wave-E-Pin hatte den
Status bereits auf `pruefung`; die Status-Rücknahme ist **nicht** Teil dieser Revision.

Die Koordinate (50.2336081 / 8.8601822, OSM way 338962561, Hausadresspunkt) wird **nicht** vom
Wave-E-Pin erfasst (der Pin umfasst keine GEO-Felder) und ist separat im Dossier belegt.

## Owner-Entscheidung

Die neuen Werte sind **besser belegt** als der Pin-Stand vom 11.10.2026 (Betreiber-Primärquelle
plus unabhängiger Verzeichnisbeleg gegen einen Beleg aus einer früheren Recherche ohne
Betreiber-Primärquelle). Der Pin wird deshalb **revidiert**, nicht umgangen: Die Revision ist
als neue Basisdatei `scripts/wave-e-20261027-expectations.json` abgelegt (Kopie der
6b3da663-Basis mit genau **einer** geänderten Zeile), der `dossier_revision` zeigt auf den
Welle-Commit `e6934c81`, und die Datei trägt einen `revision`-Block mit diesem Nachweis. Der
release-pinned Verifier des Releases `e6934c8` wird aus dieser revidierten Basis erzeugt; alle
übrigen 59 Wave-E-Zeilen bleiben **byte-identisch** zur bisherigen Basis.

Die Revision ist **kein** Freibrief: Jede weitere Änderung an den Wave-E-Zeilen bleibt rot und
verlangt wiederum eine ausdrückliche Owner-Revision mit Belegnachweis.

## Nachweis

- `python3 scripts/verify-wave-e-postrollout-e6934c8.py --data-dir target/predeploy-e6934c8/data`
  → `ok: true`, 60 Zeilen, 0 Abweichungen (nach der Revision; vorher 6 Abweichungen in genau
  dieser einen Zeile).
- `python3 scripts/test_verify_wave_e_postrollout-e6934c8.py` → 8 Tests, OK.
- Die Wellen-Dossiers selbst sind in `research/wave-20261027-1800-report.md` und im
  Dossier `dossiers/he/he-nidderau-ab-recycling-bjoern-asal.md` belegt.
