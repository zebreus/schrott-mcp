# Owner-Gate Welle F — 12.10.2026

## Umfang und Belege

Die drei Shards (`wave-f-nordwest`, `wave-f-ost-mitte`, `wave-f-sued`) enthalten jeweils 20 unterschiedliche Dossiers. Der Owner-Abgleich fand alle 60 Slugs in der Produktionsdatenbank; Bundesland stimmte überein und jeder Eintrag erfüllte weiterhin `website='' OR street=''`. Die Shards sind nach Bundesland disjunkt. Produktionsdatenbanken wurden ausschließlich mit read-only-Verbindungen abgefragt.

Alle 60 geänderten Dossiers entsprechen genau diesen Slugs. Die bestehenden Timeline-Abschnitte sind als einzelne historische Blöcke unverändert erhalten; IDs, Bundesland, Händlerart und Dossierkoordinaten wurden nicht geändert. Enum-, Frontmatter-, Überblicks-, Datums- und `clear_fields`-Prüfungen sowie `git diff --check` sind sauber.

Der Owner hat insbesondere folgende Quellen nochmals direkt geprüft:

- **Agosi:** Betreiber-Kontaktseite trennt Kanzlerstraße, Kunden-Schalter in der Robert-Bauer-Straße und NE-Metall-Standort am Hauptgüterbahnhof. Die direkt gelesene Kurstabelle vom 09.10.2026 stimmt mit dem Dossier überein; Eröffnungs-Ankauf: Gold 117,51 €/g, Silber 1.691,40 €/kg, Platin 46,70 €/g, Palladium 32,55 €/g. Die Kurse sind datiert, indikativ und keine allgemeinen Schrottkurse.
- **Ladner/Sontra:** Die Stadtquelle nennt Ladners Gelände am Brodberg für Batterien/Zubehör und Reifen mit Felgen. Die kommunalen 30 €/Fahrt gelten für den Bauhof-Elektrogeräteabtransport, nicht für Ladner.
- **Amsbeck, SM Metals, CR Recycling und REMONDIS:** Betreiber-/Kommunal-/Registerspiegel-Belege decken die dokumentierten Rechtsträger, Standorte und jeweiligen Leistungsgrenzen ab. Wo Telefonnummer/E-Mail kollidieren oder eine Annahme für Privatkunden unbelegt bleibt, wurde kein Wert ergänzt.
- **Schönebeck:** Die Stadt Schönebeck führt „Auto Scholz Autoverwertung“ an Wilhelm-Dümling Straße 10. Das bestätigt Name und Adresse unabhängig, nicht Telefon/E-Mail; diese bleiben im strukturierten Seed leer. Betreiberangebote und Zertifizierung bleiben bis zu einer aktuellen unabhängigen Bestätigung Selbstaussagen.
- **Güstrow:** Betreiberseite und Creditreform bestätigen Name, Primerburg 3, Telefonnummer und Website. E-Mail und Öffnungszeiten sind nur auf der Betreiberseite belegt und wurden aus den Seed-Feldern entfernt bzw. mit `clear_fields` ausgeschlossen. Die Werkstattseite belegt keine Schrottannahme.
- **Preis- und Identitätsgrenzen:** Die drei Agentenberichte trennen Ankauf, Verkauf und Gebühren. Verzeichnis-/Suchtreffer, historische Angebote und Preisranges wurden nicht als aktuelle Händlerkurse oder Schließungsbelege übernommen; Namensvetter und nahe Betriebe bleiben getrennt.

Quellen: [Nordwest](wave-f-nordwest-20261012.md), [Ost/Mitte](wave-f-ost-mitte-20261012.md), [Süd](wave-f-sued-20261012.md), [Agosi Kontakt](https://agosi.de/kontakt/), [Stadt Schönebeck](https://www.schoenebeck.de/de/industriepark-west.html), [Schwittau Impressum](https://www.kfz-schwittau.de/impressum), [Creditreform Schwittau](https://firmeneintrag.creditreform.de/18273/3190435545/MAIK_SCHWITTAU_FAHRZEUGTECHNIK_KAROSSERIEBAU), [Sontra](https://www.sontra.de/seite/254956/sonstiger-abfall.html).

## Owner-Korrekturen

- Fünf Telefonnummern, die nur durch nicht qualifizierte Verzeichnis-Leads gestützt waren, sind nun explizit widerrufen: `by-dietersburg-eibl`, `he-babenhausen-64832-kohlmann-frank`, `he-dreieich-stefan-schramm`, `he-frankfurt-65929-cimen` und `he-fulda-mittelrode-bohm-michael-schrotthandel`. `clear_fields: 'phone'` ist erforderlich, weil ein leeres Telefonfeld bestehende Seed-Werte normalerweise behält. Die zugehörigen Timeline-Korrekturen erklären Beleglage und Seed-Effekt.
- Bei Kohlmann und Schramm entfernt der nächste Seed Straße/PLZ; bei Schramm soll außerdem der Status `pruefung` den gespeicherten `aktiv`-Wert ersetzen. Bestehendes GEO wird dadurch nicht gelöscht.
- Für Rathenow wurde der leere Überblick durch einen vorsichtigen Identitäts-/Belegstand ersetzt.

## GEO- und Veröffentlichungsblocker

`docs/trader-seed.md` bestätigt: Leere `street`/`postcode` ersetzen diese Felder beim nächsten geänderten Seed, während gespeichertes GEO unverändert bleibt. Es gab keine Koordinatenänderung und keinen Geocode in Rust. Der read-only-Abgleich fand folgende Paare, die vor einer Seed-Veröffentlichung geprüft werden müssen:

| Dossier | Produktions-GEO (nur gelesen) | Warum prüfen |
|---|---|---|
| `st-schonebeck-scholz` | 53.0595, 12.2331 | Weicht deutlich von der neu belegten Anschrift in Schönebeck ab. |
| `he-babenhausen-64832-kohlmann-frank` | 49.0551, 9.6023 | Wirkt deutlich unplausibel für Babenhausen; Straße/PLZ werden zurückgenommen. |
| `he-dreieich-stefan-schramm` | 50.0200, 8.6961 | Verbleibt trotz zurückgenommener Adresse; Zuordnung separat bestätigen. |
| `he-heidenrod-schrottmeister-rhein-main` | 50.1649, 7.9535 | Die Adresse ist einer anders benannten Firma zugeordnet; Alias-/Dublettenfrage offen. |
| `by-munchen-hoffmann-karl-heinz` | 48.1436973, 11.5598080 | Rappenweg-Adresse wird zurückgenommen; Pin-Zuordnung erneut prüfen. |
| `by-augsburg-reinhardt-robert` | 48.3654357, 10.8975357 | Adresse/Standort bleibt widersprüchlicher Verzeichnis-Lead. |
| `by-growallstadt-63868-schrott-rosel` | 49.8794, 9.1534 | Straße wird zurückgenommen; keine bestätigte Annahmestelle. |

Auch die vorhandenen Punkte für neue bzw. vervollständigte Standorte in Güstrow (53.7972, 12.1734), Freiberg-Muldenhütten (50.9500, 13.3500), Oberderdingen (49.0656, 8.8031), Lampertheim (49.5979, 8.4725) und Harsewinkel (51.9622, 8.2277) wurden nicht als genaue Anschriftspunkte verifiziert. Keine Koordinate ist aus einer Adresse abgeleitet oder automatisch korrigiert. Daher **kein Seed-Rollout/Redeploy dieser Dossiers**, bis die GEO-Punkte unabhängig geprüft sind; Produktionsdatenbanken bleiben read-only.

## Seed-Tests und Produktions-Check-in

- `/root/.cargo/bin/cargo test --offline -p schrott-mcp-ingestion seed`: **14 bestanden, 0 fehlgeschlagen** (594 gefiltert). Umfasst unter anderem Batch-Validierung, `clear_fields` und Erhalt alter GEO bei leerer Adresse.
- `rustc --edition=2021 --test crates/ingestion/build.rs -o /tmp/opencode/seed-build-tests-wave-f` und `/tmp/opencode/seed-build-tests-wave-f`: **9 bestanden, 0 fehlgeschlagen**.
- Kein Workspace-Testlauf: Diese Welle ändert keinen Rust-Code; Dateisystem ist zu 97 % belegt (2,3 GB frei; `target` 6,8 GB). `rustfmt` war nicht nötig. Die bereits integrierten Handler wurden anhand aktueller Produktionsläufe read-only geprüft: ELNO #1424 **36**, AGH #1416 **8**, Saxonia #1431 **4** (sechs nicht explizite Ankaufkurse übersprungen), Hofmann Rastatt #1418 **23**, reGOLD #1432 **20**, Trapper #1428 **21**; alle Status `ok`. Der jüngste Gesamtlauf #1433 endete 00:44 UTC mit 73 Preisen und 0 Händlerfehlern.
- Dienst aktiv seit 09.10.2026 20:04 UTC, `NRestarts=0`, ca. 81 MB. Neueste Feedback-IDs bleiben 6080–6071 (09.10.). System-Journal-Warnungsabfrage ist leer; strukturierte Logs enthalten zwei frühere Warnungen: A‑Z Münster wurde wegen verdächtiger Preisleiter fail-closed abgelehnt, AMR lieferte HTTP 502. Die gefundenen `/tmp/opencode/*.log`-Geocode-Treffer sind alte Unit-Test-Ausgaben, keine offenen Geocoding-Läufe.

## Veröffentlichung

Die initial geprüften Dossiers und Berichte wurden lokal auf `main` als `8ae25764` committed. Push und Redeploy bleiben zurückgestellt: vor diesem Commit lag `main` bereits 34 Commits vor `origin/main`; der Branch ist seither ahead und enthält zusätzliche nicht hier geprüfte Änderungen. Außerdem darf wegen der noch ungeklärten GEO-Punkte kein Dossier-Seed in Produktion angewendet werden. Es gab keine Produktionsdatenbank-Schreibzugriffe. Remote-Ancestry und GEO sind vor einer späteren Veröffentlichung separat zu klären.

## GEO-Nachtrag nach Commit

Ein anschließender read-only-Nominatim-Abgleich hat jetzt vier hoch-konfidente Punkte für bereits bestätigte Adressen ergeben; die Koordinaten stehen mit OSM-Objekt- und Genauigkeitsangabe in den Timeline-Nachträgen der Dossiers:

| Dossier | Dossier-GEO | OSM-Beleg / Punktart |
|---|---|---|
| `st-schonebeck-scholz` | 52.0254671, 11.6917707 | Benannter Auto-Scholz-POI, POI-Punkt statt vermessener Einfahrt. |
| `mv-gustrow-18273-fahrzeugtechnik-und-karosseriebau-schwit` | 53.7931548, 12.2242372 | Maik-Schwittau-Werkstattgebäude, Gebäudeschwerpunkt. |
| `bw-oberderdingen-cr-recycling-aydemir-cinar` | 49.0792416, 8.7783427 | CR-Recycling-Flächenobjekt, Polygon-Schwerpunkt. |
| `sn-halsbrucke-muldenhutten-remondis-industrie-service-nl-freiberg-m` | 50.9019340, 13.3823251 | Gebäude an der offiziellen Muldenhütten-10-Adresse; OSM führt noch den Fehr-Namen. Die dokumentierte Übernahme der Fehr-Sonderabfallstandorte durch REMONDIS umfasst Muldenhütten. |

Dies ändert ausschließlich die Dossierquellen; die Produktionspunkte bleiben unverändert, bis ein zulässiger Seed-Rollout erfolgt. Der Muldenhütten-Label-Konflikt ist durch die dokumentierte Fehr-Übernahme und die aktuelle offizielle REMONDIS-Niederlassung weitgehend aufgelöst; der Punkt bleibt als Gebäudeschwerpunkt und nicht als Eingang gekennzeichnet. Harsewinkel lieferte nur eine Straßenlinie. Für SM Metals liefert OSM einen nicht mit Betreibername markierten Gebäudepunkt an der als Postanschrift angegebenen Adresse; der wird nicht als Kundenstandort übernommen. Damit sind vier zuvor offene Standortpunkte genauer belegt, die übrigen alten/unsicheren GEO-Punkte und die Produktions-Read-only-Sperre bleiben offen. Die Seed-Tests wurden nach den vier Koordinatenänderungen erneut erfolgreich ausgeführt (14 bestanden, 0 fehlgeschlagen; 594 gefiltert). Der GEO-Follow-up bleibt ein lokaler Commit; kein Push oder Redeploy.
