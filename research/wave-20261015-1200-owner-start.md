# Recherchewelle 15.10.2026, 12:00 UTC: Start und Owner-Check-in

`prompts/tiefenrecherche-welle.md` und `prompts/hourly-checkin.md` gelesen. Doppelte gleichlautende Wellen-Anforderung als einen Auftrag behandelt; genau eine neue 60er-Auswahl, nicht zwei überlappende Wellen.

## Frische Betriebsprüfung

12:00:25 UTC beide Produktionsdatenbanken ausschließlich SQLite `mode=ro` plus `PRAGMA query_only=ON`. Feedback 7381 bis 7390 unverändert; keine neue Meldung. Public: 3881 Händler, 54 Materialien, 2426 aktuelle Preise. Service active; https://schrottindex.de/health `ok:true`. Systemd-Warnprioritäten sechs Stunden leer. App-WARNs Hofmann 09:39 gemischte Zeilendaten, Münster 11:10 Preisleiter, AMR 11:25 HTTP 502; bekannte Fehlerklassen. Keine Geo-Logs. Einziger Worktree main, Ausgangs-HEAD `99da1af2`. Kein Produktionswrite, keine erzwungene Ingestion oder Rolloutbehauptung.

## Auswahl und Schutzgrenzen

Frisch aus `public.db`: `SELECT slug,state FROM traders WHERE website='' OR street='' ORDER BY RANDOM()`, 1968 Poolzeilen. Daraus je 20 mit vorhandenem Dossier und disjunkten Bundeslandgruppen gewählt; vorbestehende Dirty-Pfade sowie die vorige 60er-Auswahl ausgeschlossen. Keine Übernahme einer alten Ergebnisliste als neue Auswahl. Insgesamt 60 unterschiedliche Slugs. Index zu Beginn leer; 154 bestehende Fremdpfade mit SHA256 eingefroren. Der bereits integrierte Pedack-Pfad gehört nicht mehr zu diesen 154 offenen Änderungen.

Vollständiges maschinenlesbares Manifest mit Ausgangs-HEAD, Pfaden, Einzelhashes, Schutzpfaden und Shards: `/tmp/opencode/wave-20261015-1200-selection.json`. Auswahl und Verantwortlichkeiten zusätzlich hier dokumentiert.

### A: NW, HE, RP, SL

Session `ses_ec0903431ffeVX69ByZxeBegto`; Bericht `research/wave-20261015-1200-a.md`.

nw-wesel-jack-schrotersk; rp-koblenz-pms-schrotthandel; nw-essen-rainer-ludwig; he-messel-64409-schrottler-eddie; rp-neuwied-56566-1a-recycling; he-kassel-34123-dietz; he-darmstadt-nord-hartmann-recycling; nw-ennigerloh-ws; he-kassel-bettenhausen-pfeil-autoverwertung-ellenbacher-str; nw-kreuztal-martin-muller-schrott-und-metallhandel; he-marburg-35039-gluck; nw-horn-bad-meinberg-neuwald; he-frankfurt-60327-schrott-wetzel; rp-koblenz-56070-alten-familie-gino-markus-klara-angelo; nw-munster-metall-schrottagentur-munster; nw-lippstadt-a-r-schrottabholung-lippstadt; nw-paderborn-franz-gawor; he-gelnhausen-schrotthandel-willefuhr; sl-bexbach-kleinottweiler-schaller-alfons-rohprodukte; nw-pulheim-steinhaus.

### B: BW, BY, SN, TH

Session `ses_ec090109affeO1zv70d3uRcnH5`; Bericht `research/wave-20261015-1200-b.md`.

bw-freiburg-msw-metallhandel-sudwest; sn-lichtenstein-otto-stiegler-schrott-metallhandel-inh-h; th-erfurt-optima-computer-elektronikrecycling; sn-rodewisch-08228-rau-rita-und-joachim; sn-kamenz-rainer-kalberer; bw-wertheim-pfeifer-rainer-schrotthandel; by-bergkirchen-hartmann; sn-arnsdorf-ot-fischbach-schrotthandel-hausmeisterservice-m-baum; by-memmingerberg-eisenhut; th-bad-lobenstein-sinnig-john-schrotthandel-und-hausservic; by-hafurt-lenhart-mathias; by-kleinlangheim-lenhardt; by-tegernheim-stuber; bw-fichtenau-knodel-dieter-schrotthandel; sn-leipzig-schanko-altmetallhandel-schrotthandel-jo; by-konigsberg-anger; by-arnstein-heugrumbach-97450-funk-johannes-schrott-und-metallhandel; by-miltenberg-63897-car-transplant; bw-edingen-neckarhausen-met-schrott-mehmet-sahin; bw-bobingen-simsek.

### C: ST, HB, MV, SH, NI, BE

Session `ses_ec08ff29affe3zvIsp4IQDEX5Q`; Bericht `research/wave-20261015-1200-c.md`.

st-veckenstedt-b-wernigerode-lasar-ulrich-schrotthandel; hb-hemelingen-metalports; mv-stralsund-schrotthandel-u-aufbereitung-inh-frank-b; sh-kiel-graf-abfallbeforderung-und-entsorgung; ni-upgant-schott-gerhard-czaja-schrotthandel; ni-westerstede-muller-autoverwertung; ni-varel-schrottentsorgung-varel-wilhelmshaven; be-steglitz-zehlendorf-kujat-artur-schrotthandel; mv-rostock-18119-wsr-warnemunder-schrott-und-recycling; ni-syke-schrott-und-metallhandel-c-c; st-zeitz-gertloff-heuer-koch; hb-bremen-28195-albers-kinzel; ni-diepholz-49356-albert-berg; mv-robel-17207-mca-muritzer-container-abbruch; be-mitte-gundlach-klaus; ni-schwaforden-weser-recyclinghof; be-marzahn-hellersdorf-wolf-marko-schrott-altmetall-abholung; hb-martfeld-mannott-hans-heinrich-autoschrott-metall; st-gardelegen-hero-recycling; st-radegast-eks-ankauf-von-katalysatoren-und-batteri.

## Laufstatus und Abschlussgate

Drei konkrete `general`-Rechercheagenten abgeschlossen: jeweils 20 Manifestdossiers und eigener Bericht. Anschließender Formatnachlauf B ebenfalls abgeschlossen. Quellen-/Betreiberstandard aus README; historische Befunde und Slugs erhalten, Ankauf/Verkauf/Gebühren getrennt. Keine Agentencommits, keine Rust-/GEO-Codeänderungen, kein Zugriff mit Schreibrechten auf Produktionsdatenbanken.

Owner-Gate jetzt bestanden: sämtliche 60 Diffs geprüft, vollständige alte Timelines und stabile Slugs maschinell bestätigt, 154 Schutzhashes unverändert. Frische Seedtests 14 bestanden, Workspace 724 bestanden/0 Fehler/7 ignoriert, rustfmt sauber. Isolierter Codegen aus HEAD plus exakt 60 Wellendossiers erfolgreich, ohne offene Fremddossiers. Details und Quellenabgrenzungen im [Owner-Abschlussgate](wave-20261015-1200-owner-gate.md). Ein gemeinsamer, auf diese Welle begrenzter Commit ist vorbereitet; Veröffentlichung separat prüfen.

Push bleibt durch den zuletzt tatsächlich reproduzierten fehlenden GitHub-HTTPS-Benutzernamen blockiert. Keine veränderte Authentifizierung festgestellt. Ohne erfolgreichen Push und geprüften Build kein Redeploy beider Binaries und keine produktiven Wellen-Spots als erfolgt ausgeben. Vorhandene offene RUSS-/Fix-/ASR-Nacharbeiten bleiben getrennt von dieser Auswahl erhalten.
