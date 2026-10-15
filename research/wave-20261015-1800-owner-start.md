# Tiefenrecherche-Welle 15.10.2026, 18:00 UTC: Start und Auswahl

Die doppelt eingegangene identische 6h-Anforderung als **eine** 60er-Welle verarbeitet. `prompts/tiefenrecherche-welle.md` vollständig gelesen; 60 Slugs frisch aus dem aktuellen Produktionspool ausgewählt, keine alte Wellenliste wiederverwendet. Auswahl 18:00:28 UTC, Ausgangs-HEAD `1f91f55cb72502433f8869626d966b57e52f81cf`.

Produktion ausschließlich `file:…public.db?mode=ro` und `PRAGMA query_only=ON`. Poolabfrage `SELECT slug,state FROM traders WHERE website='' OR street='' ORDER BY RANDOM()`. Auf bestehende lokal saubere Dossiers eingeschränkt, um 149 vorbestehende offene Pfade nicht zu überschreiben. Bundesländer anhand der aktuellen Produktionswerte disjunkt auf drei Gruppen verteilt, innerhalb jeder Gruppe 20 zufällig gewählt. Genau 60 eindeutige Slugs und Disjunktheit geprüft. Historische Verzeichnis-/Slugpräfixe nicht als heutiger Bundeslandbeweis verwendet.

Manifest mit Auswahl, HEAD-Texten und 149 Schutzhashes: `/tmp/opencode/wave-20261015-1800-selection.json`. Isolierte Seed-Basis aus Git-Archiv dieses HEAD vorbereitet: `/root/Documents/wave-20261015-1800-isolated-yz9mzfmx`, unverändertes `build.rs` kompiliert und ausgeführt, 3892 Zeilen. Nach Agentenabschluss ausschließlich die ausgewählten Dossiers überlagern und gesamten Payloadvergleich durchführen; kein indirekter Releasebeweis aus fremdem Arbeitsbaum.

## Konkrete Shards

### A, Session `ses_ebf46d68affew25c4uOM1CXIOd`

Bundesländer NW/ST/BW/BB/SH/HB. Ergebnisziel `wave-20261015-1800-a.md`.

`nw-munster-alpha-recycling`, `nw-kerpen-sindorf-lhonneux-containerdienste`, `nw-munster-bindemann`, `nw-gutersloh-bartsch`, `nw-ibbenburen-fuhrer`, `st-dessau-rolau-decker`, `st-leuna-schrott-recycling-johann-steffens`, `nw-mulheim-bittsching`, `nw-bochum-wattenscheid-schrotthandler-plus`, `bb-prenzlau-rose-christine-metallaufbereitung`, `nw-essen-schrotthandler-traut`, `hb-schonebeck-schrott-metall-peter-ziegler`, `bw-karlsruhe-oberreut-wagner-michael`, `nw-hagen-ates`, `sh-twedt-steffen-walter-altmetallentsorgung-schro`, `nw-monchengladbach-rheydt-kupferrecycling-rheydt`, `nw-gummersbach-metallhandel-akawad`, `nw-dusseldorf-gerresheim-adam`, `hb-bremen-28201-hanse-entrumpelung`, `nw-nrw-weit-schrott24-metaloop-online-portal`.

### B, Session `ses_ebf46acd1ffeEv8I834yoWW2nC`

Bundesländer HE/BY/MV/BE/HH. Ergebnisziel `wave-20261015-1800-b.md`.

`he-munster-bierhenkel`, `he-wabern-34590-flink-flott`, `he-frankfurt-60386-autoverschrottung-cassellastr`, `be-tempelhof-schoneberg-sperber-otto-autoverwertung`, `he-bad-soden-salmunster-63628-seipel-michael`, `by-hafurt-97437-lenhart-valentin`, `mv-duvendiek-18442-dapa`, `by-adelsdorf-michalak`, `mv-locknitz-17321-frank-dreblow`, `he-hattersheim-65795-ferdinand-wegner-helga-hippe`, `he-staufenberg-35460-stahlhugel`, `mv-schwerin-fair-metall-schrott-altmetallhandel`, `mv-greifswald-17489-kfz-service-ziegelhof`, `by-alzenau-jr-recycling`, `by-ammerndorf-dombrowski`, `he-babenhausen-64832-ries-peter`, `he-niestetal-sandershausen-schneider-schrott`, `by-nurnberg-horski-an-und-verkauf`, `mv-neubrandenburg-neubrandenburger-buntmetallhandelsgesell`, `hh-billbrook-alt-und-buntmetallhandel-dejan`.

### C, Session `ses_ebf46816affe5QrO7bZBgFIyPG`

Bundesländer RP/NI/SN/TH/SL. Ergebnisziel `wave-20261015-1800-c.md`.

`ni-delmenhorst-jesewski-autoverwertung`, `sn-meien-01662-sewzyk-heiko-papier-schrothandel`, `ni-lachendorf-dreler-altmetallhandel-michael-dreler`, `rp-bengel-ostermann-bengel`, `rp-landstuhl-firma-peter-hoschele`, `ni-wittingen-uta-adam-schrott-und-metallhandlung`, `rp-urzig-jorg-schmitz-schrotthandel-industrieabbr`, `sl-volklingen-metall-recycling-kunzer`, `rp-kaiserslautern-67655-kratz-pfandkredit`, `rp-landstuhl-s-t-janek-schrotthandel`, `rp-neustadt-sterz-neustadt-wied`, `sl-saarbrucken-westrich-lothar-autoverwertung`, `th-sonneberg-gertz-altmetallhandel`, `ni-norden-weerts-peters-schrotthandel`, `sn-wilkau-halau-containerdienst-schrotthandel-konrad-eim`, `sn-lobau-02708-schrott-gerda`, `th-sonneberg-recyclinghof-oberlind-manfred-hiller`, `rp-koblenz-56070-schrotthandel-s-alten`, `ni-langwedel-auto-recycling-pankowski`, `rp-mulheim-karlich-56218-goldankauf-mulheim-karlich`.

Alle drei Hintergrundagenten erfolgreich gestartet. Kurze ergebnisorientierte Aufträge mit eigener Methodenwahl, begrenzt auf ihre 20 Dossiers und ihren Bericht. Historie und Slugs erhalten, Quellen-/Identitätsregeln nach README; Ankauf/Verkauf/Gebühren unterscheiden, Geo-Bedarf benennen. Keine Produktionswrites, kein Rust-GEO und keine Agenten-Commits/Deployments. Ergebnisse zum Start noch offen, keine Fertigstellung behauptet.

## Parallel eingegangener stündlicher Check-in

Anleitung `prompts/hourly-checkin.md` aus dem unmittelbar vorherigen Check-in weiterhin im Kontext; neue Betriebsprüfung tatsächlich um 18:01:12 UTC ausgeführt. Beide Produktionsdatenbanken read-only/query-only. Feedback IDs 7390 bis 7381 unverändert seit 13.10.; Public 3881 Händler / 54 Materialien / 2426 aktuelle Preise. Service active, öffentliches HTTPS-Health `ok:true`. Keine Warnprioritäten sechs Stunden, App-WARNs separat: SAXONIA 12:55 alte URL 404, Hofmann 15:41 gemischte Zeilendaten, Münster 17:12 Preisleiter, AMR 17:27 HTTP 502. Keine neue Fehlerklasse oder Geo-Logs. Bereits lokal erfolgreiche SAXONIA-/Hofmann-Livetests aus 17:30 sind kein Produktionsrollout. Kein erneuter AMR-Fallback oder erzwungene Ingestion.

## Ausstehendes gemeinsames Owner-Gate

Erst nach allen Shards: vollständige Berichte/Diffs lesen, Identität/Geschwister/Quellen/Overwrite prüfen, Althistorie und geschützte Pfade hashen, Enum-/Deep-Link-/Emdash-/Notenformatgate, finale Seedtests, ein scoped Commit und tatsächlicher Push. Bekannter GitHub-HTTPS-Anmeldeblocker nicht umgehen. Beide Binaries nur nach erfolgreicher Veröffentlichung und geprüftem Release bauen/redeployen; danach tatsächliche Produktions-Spots. Der laufende Dienst ist kein Nachweis dieser noch offenen Welle.
