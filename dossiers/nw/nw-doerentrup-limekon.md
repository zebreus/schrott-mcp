---
slug: nw-doerentrup-limekon
name: Limekon GmbH
trader_type: metallhaendler
state: NW
city: Dörentrup-Bega
street: Bahnhofstr. 23
postcode: '32694'
phone: ''
email: ''
opening_hours: ''
clear_fields: 'phone,email,opening_hours,dropoff_json,pickup_json'
website: https://limekon.de/pages/ankauf.php
website_status: aktiv
status: aktiv
description: 'Metall-/Edelmetallankauf; laut Betreiber Zinn und Zinnlegierungen, versilbertes Besteck, Silber, Gold sowie persönliche Anlieferung, Zusendung und Abholung nach Vereinbarung. Konkrete Servicebedingungen sind nicht unabhängig bestätigt; Versandgebühren und Abholschwellen widersprechen sich zwischen Betreiberseiten.'
dropoff_json: ''
pickup_json: ''
provenance_section: 'Aufnahmegate Audit 7 Feedback #6549 / #7008 (Index 29)'
provenance_ankauf_raw: ja
provenance_origin: feedback
---

# Limekon GmbH

## Überblick

Aktueller vergüteter Ankauf am betrieblichen Standort Bahnhofstr. 23 ist durch die Betreiber-Ankaufseite samt Adressfooter, Impressum und aktuelle Preis-/Öffnungshinweise belegt. NorthData bestätigt unabhängig die Rechtsträgerin LIMEKON GmbH, Amtsgericht Lemgo HRB 7953, an derselben Adresse und den Metallhandelsgegenstand. Der von NorthData verlinkte GLEIF-Datensatz bestätigt zusätzlich ACTIVE, ISSUED und FULLY_CORROBORATED mit Validierung gegen HRB 7953 beim Amtsgericht Lemgo. Die README-Ausnahme für alleinige Betreiberquellen wird nicht beansprucht: Im aktuellen Impressum fehlt die HRB-Angabe.

Lippisches Metall Kontor ist die auf der aktuellen Startseite erklärte Namensauflösung von Limekon, kein belegter früherer Rechtsträger. Die alte Betreiberwebsite zinn-ankauf.de nennt dieselbe GmbH, HRB, Anschrift, Telefonnummer, USt-ID und Person; ihre Januar-2016-Darstellung wird nicht als präzise Rechtsnachfolge übernommen. NorthData nennt davon getrennt den früheren GmbH-Namen Brüggemann Zinn-Handel GmbH und Ausgliederungsereignisse im April 2016. Keine zweite Händleraufnahme für diese Aliase.

Die Preisliste vom 14.10.2026 zeigt Ankauf-Richtpreise, keinen Warenverkauf und keine Entsorgungsgebühren. Die separate Paketmarkengebühr darf nicht in Materialpreise umgedeutet werden. Versandkosten, Abholschwellen und Zinngrade unterscheiden sich zwischen Unterseiten; Einzelheiten und spätere Handlergrenzen stehen in research/feedback-20261014-limekon-admission.md. Keine Preise, Koordinaten oder Zertifizierungsnachweise importiert; die EFB-Aussage ist Betreiber-Selbstauskunft ohne abgerufenes Zertifikat.

## Timeline

### Owner-Korrektur 26.10.2026

- [Korrektur 26.10.2026: Read-only public.db zeigt für Limekon Telefonnummer und E-Mail erneut befüllt; `updated_at` 22:58:30.959Z liegt im erfolgreichen regulären Handler-Step #2854 / Step 14162 (15 Preise). Der lokale Handler las beide Kontakte aus dem Betreiber-Impressum und der allgemeine Recorder überschrieb sie, obwohl dieses Dossier sie mangels unabhängiger Bestätigung mit `clear_fields` ausdrücklich zurücknimmt. Das ist kein Beleg, dass die Kontaktwerte falsch sind; es zeigt einen Konflikt zwischen Quellen-Gate und Scraper-Enrichment. Der Handler behält den nötigen Impressum-Check für Rechtsträger/Adresse, projiziert Telefon/E-Mail aber nicht mehr in `TraderInfo`; Regressionstest deckt auch ein Impressum ohne diese Kontakte ab. `clear_fields` bleibt bestehen, sodass ein freigegebener Seed sie entfernt und spätere reguläre Läufe sie nicht erneut eintragen. Produktion blieb read-only; bis zum freigegebenen Rollout bleiben die derzeitigen Produktionswerte sichtbar. Quelle: `/var/lib/schrott-mcp/internal.db` (mode=ro; Run/Step #2854/14162); `/var/lib/schrott-mcp/public.db` (mode=ro); https://limekon.de/pages/impressum1.php ; README.md, Quellenhierarchie.]

### Owner-Nachprüfung 26.10.2026, 23:45 UTC

- [Der reguläre Lauf #2855 meldete `trader seed up to date (1 rows written)`; ein read-only Abgleich um 23:45 UTC zeigt Telefon und E-Mail jetzt leer. Der letzte Limekon-Handler-Step bleibt #2854 / Step 14162 um 22:58 UTC; die zwei späteren regulären Läufe #2856 und #2857 meldeten Seed-Schreibzahl 0 und enthielten keinen neuen Limekon-Step. Damit wurde die Dossier-Rücknahme durch den normalen Seed-Pfad angewendet, nicht durch einen Agenten-DB-Write. Das ist noch kein dauerhafter Handler-Nachweis: der alte Handler hatte die Werte in #2854 erneut eingetragen und könnte es beim nächsten Limekon-Lauf wieder tun. Der lokale Handler-Fix verhindert diese Wiederanreicherung nach freigegebenem Rollout. Quellen: journal/run #2855–#2857; `/var/lib/schrott-mcp/public.db` und `internal.db` read-only; README.md, Quellenhierarchie.]

### Owner-Nachprüfung 16.10.2026

- [Korrektur 16.10.2026: Identität/Adresse/Domain und Aktivität der LIMEKON GmbH durch Betreiber, NorthData HRB7953/Lemgo und frische GLEIF-Originaldaten bestätigt. Telefon+49 5263 9561906, info@limekon.de, persönliche Zeiten Mo/Di09–17/sonst Absprache und Ferien16.12.2026–10.01.2027 bleiben jedoch nur Betreiber-belegt. Creditreform-Original403, Suchindex nur Lead. Aktuellem Impressum fehlt HRB; die bereits14.10. ausdrücklich nicht beanspruchte README-Ausnahme wird nicht nachträglich behauptet. Daher Kontakt-/Stundenfelder und positive Anliefer-/Abhol-JSON ausdrücklich zurückgenommen, clear_fields verhindert Erhalt gespeicherter Altzusagen. Beschreibung formuliert Detailleistungen als Betreiberangabe statt kuratierter Zusage. Kein negatives allowed:false, keine Schließung, keine Preis-/GEOänderung. Versand7/10EUR und Abholschwellen200/300kg/10000EUR bleiben widersprüchliche Eigenangaben in der Historie. Quellen: https://limekon.de/pages/impressum1.php ; https://limekon.de/pages/ankauf.php ; https://limekon.de/ ; https://limekon.de/pages/paketmarke.php ; https://api.gleif.org/api/v1/lei-records/52990009EHJQWTYJ7757 ; Detailreview: research/checkin-20261016-0300-contact-gates-b.md.]

### Recherche 14.10.2026

- [Recherche 14.10.2026: Owner hat GLEIF-Original-API, Ankaufseite und dedizierte Preisliste separat regulär gelesen: HRB 7953, ACTIVE/ISSUED/FULLY_CORROBORATED, Bahnhofstraße 23; persönlicher Ankauf mit umgehender Auszahlung und Richtpreisliste ab 14.10.2026. Beschreibung verwendet Zinnlegierungen statt unbelegter Zuordnung aller Legierungen zu Lötzinn. Fehlenden aktuellen Produktionsabgleich ergänzt: alle 3881 Händlerfelder normalisiert nach Limekon/Aliasdomain, Register-/USt-/Entsorgernummer, Kontakt und PLZ ohne Kandidaten. Keine Preis-/GEO-Übernahme oder Feedbackquittierung. Quellen: https://api.gleif.org/api/v1/lei-records/52990009EHJQWTYJ7757 ; https://limekon.de/pages/ankauf.php ; https://limekon.de/pages/preisliste.php ; /var/lib/schrott-mcp/public.db (mode=ro, query_only).]

- [Recherche 14.10.2026: Feedback #6549/#7008 und Audit-7-Dublettenprüfung Index 29 als Leads behandelt. Aktuelles Impressum nennt Limekon GmbH, Theodor Brüggemann, Bahnhofstr. 23, 32694 Dörentrup-Bega, +49 5263 9561906, info@limekon.de, DE300828016 und Entsorgernummer E766A7005. HTML enthält nur den Text Zertifikat Entsorgungsfachbetrieb 2026, keinen Zertifikatlink; kein unabhängiger Zertifizierungsnachweis übernommen. Quelle: https://limekon.de/pages/impressum1.php]
- [Recherche 14.10.2026: Unabhängige zulässige Registerquelle NorthData bestätigt LIMEKON GmbH, Amtsgericht Lemgo HRB 7953, Bahnhofstr. 23, 32694 Dörentrup, Metallhandel einschließlich Edelmetallankauf; Geschäftsführerereignis Theodor Brüggemann 10.06.2021, Jahresabschluss 2024 veröffentlicht 25.09.2025. NorthData zeigt keinen erkennbaren Löschungs-/Liquidationshinweis, aber im lesbaren Abruf kein ausdrückliches firmenspezifisches ACTIVE-Feld. Ergänzend bestätigt der dort verlinkte GLEIF-Datensatz LEI 52990009EHJQWTYJ7757 entity.status ACTIVE, registration.status ISSUED und FULLY_CORROBORATED; letzte Aktualisierung 19.08.2026, nächste Erneuerung 19.08.2027, Validierung HRB 7953 gegen RA000288 = Handelsregister/Amtsgericht Lemgo. GLEIF ist Zusatzbeleg, nicht Ersatz der README-Registerquelle. Quellen: https://www.northdata.de/LIMEKON%20GmbH,%20D%C3%B6rentrup/Amtsgericht%20Lemgo%20HRB%207953 ; https://api.gleif.org/api/v1/lei-records/52990009EHJQWTYJ7757 ; https://api.gleif.org/api/v1/registration-authorities/RA000288]
- [Recherche 14.10.2026: Ankaufseite lädt zur persönlichen Anlieferung mit gemeinsamer Sortierung/RFA-Analyse ein und verspricht umgehende Auszahlung; Adressfooter bindet Angebot an Bahnhofstr. 23. Bar gegen Ausweis bis 1500 EUR, alternativ Banküberweisung/PayPal. Startseite nennt Mo/Di 09-17 und andere Wochentage nach Absprache, Betriebsferien 16.12.2026-10.01.2027. Versand mindestens 50 EUR Warenwert/Paket und höchstens 25 kg; Paketmarkenseite nennt 10 EUR bis 200 EUR Ankaufswert, Ankaufseite dagegen 7 EUR, daher Gebührenkonflikt offen. Abholung ca. 200 kg auf Ankaufseite versus 300 kg/10000 EUR auf Startseite, keine allgemeine Mindestankaufsmenge daraus abgeleitet. Quellen: https://limekon.de/pages/ankauf.php ; https://limekon.de/ ; https://limekon.de/pages/paketmarke.php]
- [Recherche 14.10.2026: Aliasabgleich: aktuelle Startseite erklärt Limekon als Lippisches Metall Kontor. Altes HTTP-Impressum zinn-ankauf.de verbindet Limekon GmbH mit derselben HRB 7953, DE300828016, Theodor Brüggemann, Anschrift und Telefon. Betreiber behauptet Umfirmierung/Übertragung Januar 2016; NorthData nennt GmbH-Namensänderung 09.10.2015 und Ausgliederung/Übernahme Brüggemann Zinn Ankauf e. K. 21.04.2016. Ereignisse nicht gleichgesetzt; keine erfundene Nachfolgekette. HTTPS-Abruf zinn-ankauf.de scheiterte mit Transportfehler, altes HTTP separat ohne TLS-Ausnahme gelesen. Quellen: https://limekon.de/ ; http://www.zinn-ankauf.de ; http://www.zinn-ankauf.de/index-Dateien/Page735.htm ; https://www.northdata.de/LIMEKON%20GmbH,%20D%C3%B6rentrup/Amtsgericht%20Lemgo%20HRB%207953]
- [Recherche 14.10.2026: Breiter read-only Bestandscheck vor Aufnahme über 3926 Dossiers nach Namen/Alias/Domain, Personenschreibweisen, PLZ/Ort, Register-/USt-/Entsorgernummer, normalisierter Rufnummer und Bahnhofstr. 23 ergab keine passende Dublette. Gleichnamige Straßen in Drebkau, Lauter-Bernsbach und Dillenburg sind andere Orte/Betreiber; generischer Zinnankauf bei SCHAENGELGOLD Koblenz ist ebenfalls kein Alias. Geschützte Auswahl wave-20261014-selection.json enthält keinen Limekon-/Dörentrup-Fall. Keine lokale public.db im Repo gefunden und keine Produktions-DB abgefragt; Aussage beschränkt sich auf den Repo-Bestand. Genau ein neues Dossier, keine bestehenden Dossiers verändert, kein Handler/GEO/Produktionswrite. Quellen: lokaler Dossier-/Repositorybestand ; /tmp/opencode/wave-20261014-selection.json ; research/feedback-20261014-audit7-duplicates-b.md]
