# Owner-Gate: 43 Main-only-Dossiers, 26.10.2026

## Umfang und Ergebnis

Der aktuelle kompilierte Seed enthält 3.935 Händler; die read-only
Produktionsabfrage enthält 3.893. Ein Slug, der historische
Mainmetall-Datensatz aus Sangerhausen, bleibt zusätzlich nur in Produktion.
Die 43 Seed-only-Slugs wurden von drei unabhängigen Recherche-Shards mit
disjunkten Bundesländern und ohne Datei- oder Produktionsänderungen geprüft.
Die Einzelquellen und vorhandenen Recherchehistorien stehen in den jeweils
verlinkten Dossiers. NorthData wurde als Registerspiegel, nicht als
Handelsregister-Original, behandelt. Die GESA-Oberfläche war nicht lesbar;
Aggregator-Treffer wurden nicht als Betreiberbeleg verwendet.

Der Owner hat die Berliner und Bad Homburger pro-aurum-Standortseiten sowie die
Ahlen-Seiten von Goldsofort direkt nachgelesen. Berliner Seedangaben wurden
korrigiert und ergänzt; zusätzlich wurde die Bad Homburger Beschreibung
präzisiert. Die verbleibenden Unsicherheiten sind im Seed konservativ belassen.
Diese Auswahlprüfung ist keine Freigabe des vollständigen Händler- und
Material-Seeddiffs.

## Private Datenübergangsprobe 02:38 UTC

- Eine konsistente SQLite-Online-Sicherung nur von Produktions-`public.db` wurde
  privat getestet. `PublicDb::open` führte die Kopie von Schema v6 auf v8; der
  aktuelle Arbeitsbaum-Seed lief anschließend darüber. Die 43 neuen Seedzeilen
  ergaben 3.936 Händler, wobei der nur in Produktion vorhandene historische
  Mainmetall-Datensatz erhalten blieb. Die zweite Trader-Seed-Anwendung schrieb
  null Zeilen.
- Die privaten Zähler nach Migration und Seed: 83 Materialien, 29.547 Preise,
  2.659 aktuelle Preise. `integrity_check` war `ok`, Current-Zeiger-Mismatches
  waren null. Die 24 vorbestehenden FK-Waisen zu Händler 15757 blieben
  bestehen; dies ist keine FK-Reparaturfreigabe.
- Die neue Berlin-Adresse, E-Mail, Öffnungszeiten und Beschreibung sowie die
  Bad-Homburg-Samstagsgrenze kamen genau wie dokumentiert im privaten Seed an.
  Auch Liebtraus `lat` und `lon` waren nach dem Seed in der Kopie `NULL`.
  Datenbankkopie und temporärer Test-Helper wurden danach entfernt.
- Dies war eine Public-DB-Quellcode-/Datenprobe, kein vollständiger Service- oder
  Release-Binary-Test; `internal.db` und beide Release-Binaries waren nicht
  Bestandteil. Produktion blieb read-only.

## Befunde je Dossier

| Dossier | Owner-Disposition am 26.10.2026 |
|---|---|
| [BE Aurofix](../dossiers/be/be-berlin-aurofix-gmbh-industriegold.md) | Betreiber, Anschrift, Kontakt und Industriegold-Versand durch Betreiberquellen gestützt; keine persönliche Anlieferung behaupten. |
| [BE pro aurum Berlin](../dossiers/be/be-charlottenburg-wilmersdorf-pro-aurum-filiale-berlin.md) | Betreiberseite nennt Berlin, Adresse, Kontakt, E-Mail und Zeiten. `city`, E-Mail, Öffnungszeiten und Detail-URL ergänzt; Samstagsservice ausdrücklich eingeschränkt. |
| [BY Rheinische Scheidestätte](../dossiers/by/by-augsburg-rheinische-scheidestaette-filiale.md) | Filialdaten gestützt. Seitenhinweis „geschlossen“ belegt keine dauerhafte Schließung; Status nicht geändert. |
| [BY Piske](../dossiers/by/by-bamberg-autoverwertung-piske.md) | Betreiber und unabhängiger Firmenverzeichniseintrag stützen Standort und Fahrzeugannahme; keinen allgemeinen Schrottankauf ableiten. |
| [BY Sauter](../dossiers/by/by-bubesheim-kfz-sauter.md) | Betreiberkontakt und Fahrzeugverwertung gestützt; ähnlich benannte frühere GbR nicht als eigene aktuelle Firma behandeln. |
| [BY Huber](../dossiers/by/by-geretsried-autoverwertung-peter-huber.md) | Betreiberangaben vorhanden, unabhängige Identitätsbestätigung fehlt; keine neuen strukturierten Werte. |
| [BY AVB Beck](../dossiers/by/by-gerhardshofen-avb-autoverwertung-beck.md) | Betreiberangaben vorhanden, qualifizierte unabhängige Bestätigung fehlt; keine neuen strukturierten Werte. |
| [BY IB Autoverwertung](../dossiers/by/by-ingolstadt-ib-autoverwertung.md) | Aktueller Standort und Betreiber gestützt; frühere Schierlinger Gesellschaft nicht als heutiger Standort übertragen. |
| [BY pro aurum Goldhaus](../dossiers/by/by-muenchen-riem-pro-aurum-goldhaus.md) | Aktuelle Betreiberstandortseite und Unternehmensquellen stützen das Goldhaus; keine neue Preis- oder Annahmebedingung ergänzt. |
| [BY pro aurum Schwabing](../dossiers/by/by-muenchen-schwabing-pro-aurum-filiale.md) | Standort, Kontakt und Edelmetallleistungen durch Betreiberseite und Unternehmensquellen gestützt. |
| [BY Schindelar Süd](../dossiers/by/by-munchen-schindelar-center-autoverwertung-sued.md) | Süd-GmbH und Autoverwertungsleistungen gestützt; andere Schindelar-Gesellschaften nicht zusammenführen. |
| [BY AREG](../dossiers/by/by-neuburg-am-inn-areg-abfall-recycling.md) | Standort, Wertstoffe und Betreiber gestützt; gleichadressierte Otto Ammermüller GmbH bleibt getrennt. |
| [BY Brummer](../dossiers/by/by-pfarrkirchen-heinrich-brummer-autoverwertung.md) | Nur Aggregatorhinweis; Hausnummern 4/6 widersprüchlich, keine Betreiber- oder direkt gelesene GESA-Bestätigung. Offen lassen. |
| [BY Burlafinger](../dossiers/by/by-schrobenhausen-sandizell-autoverwertung-burlafinger.md) | Betreiber-Microsite nennt Betrieb und Leistungen, aber rechtliche Identität offen. Gleichadressierte Kfz-Burlafinger-Seite hat abweichende Kontakte; keine Verbindung behaupten. |
| [BY AVF](../dossiers/by/by-schwabach-avf-autoverwertung-franken.md) | Annahmebedingungen und Betreiberangaben gestützt; abweichende GmbH-Namensdarstellung zwischen Betreiber-Impressum und Registerspiegel transparent lassen. |
| [BY Asen](../dossiers/by/by-thurmansbang-autorecycling-asen.md) | Betreiberseiten und Handwerkskammer-Eintrag stützen Adresse, Tätigkeit und Kontakt. |
| [BW Sandkühler](../dossiers/bw/bw-heilbronn-pfandkredite-sandkuehler.md) | Filial- und Pfandkredit/Edelmetall-Leistungen gestützt; keine konkreten Ankaufskurse daraus ableiten. |
| [BW pro aurum Stuttgart](../dossiers/bw/bw-stuttgart-pro-aurum-filiale.md) | Standort und Zeitangaben durch aktuelle Betreiberseite gestützt; keine zusätzliche Änderung nötig. |
| [BW pro aurum Ulm](../dossiers/bw/bw-ulm-pro-aurum-filiale.md) | Standort und Zeitangaben durch aktuelle Betreiberseite gestützt; keine zusätzliche Änderung nötig. |
| [HE pro aurum Bad Homburg](../dossiers/he/he-bad-homburg-pro-aurum-filiale.md) | Standortseite bestätigt: an verkaufsoffenen Samstagen kein Direktankauf von Altgold oder Schmuck zur Schmelze. Die strukturierte Beschreibung wurde präzisiert. |
| [HE ZIREC](../dossiers/he/he-bensheim-64625-zirec-kabelaufbereitung.md) | Website erreichbar, meldet aber vorübergehend eingeschränkten Auftritt; öffentliche Kundenannahme und aktuelle Rechts-/Zertifikatslage offen. `pruefung`, leere Annahme- und GEO-Felder beibehalten. |
| [NI Reichert](../dossiers/ni/ni-bergen-reichert-entsorgungsfachbetrieb.md) | Betreiber und Standort durch Betreiber- und Stadtseite gestützt. EFB-/Genehmigungsangaben sind historisch; keine aktuelle Zertifizierung behaupten. |
| [NI Herrmann](../dossiers/ni/ni-geestland-autoverwertung-herrmann.md) | Betreiber, Ort, Adresse und Fahrzeugverwertung durch Betreiber- und Stadtseite gestützt; andere Geestland-Standorte getrennt halten. |
| [NI Voth Lingen](../dossiers/ni/ni-lingen-guenther-voth.md) | Betreiberseitig genannter Standort, nicht als Registerzweigniederlassung bestätigt. EFB- und ISO-Nachweise abgelaufen; Annahme offen. Bestehendes `pruefung` beibehalten. |
| [NI GWM](../dossiers/ni/ni-meppen-autoverwertung-gwm.md) | Fahrzeugbetrieb auf Betreiberseite plausibel; AGB/Impressum klären die Rechtsform nicht widerspruchsfrei. Keine Adresse/Kontakte ergänzen, `pruefung` beibehalten. |
| [NI Paul Schulz](../dossiers/ni/ni-nienburg-paul-schulz-co-ohg.md) | OHG, Standort, Annahme und aktuelles EFB-Zertifikat durch Betreiber- und Zertifikatsseiten gestützt. |
| [NI Grützmacher](../dossiers/ni/ni-wolfsburg-autoverwertung-gruetzmacher.md) | Betreiberseite widerspricht sich bei PLZ und belegt öffentliche Annahme an der Wolfsburger Adresse nicht klar. Felder leer, `pruefung` beibehalten. |
| [NW Rheinische Scheidestätte Aachen](../dossiers/nw/nw-aachen-rheinische-scheidestaette-filiale.md) | Filialdaten gestützt. Seitenstatus „geschlossen“ nicht als dauerhafte Schließung interpretieren. |
| [NW Paul & Sohn Alstätte](../dossiers/nw/nw-ahaus-alstaette-48683-wilhelm-paul-sohn-lager-ii.md) | Betreiber bestätigt Lagerstandort, nicht konkrete Publikumsannahme. Kartenpin ist kein geprüfter Eingang; `pruefung` und leere Annahmefelder beibehalten. |
| [NW Goldsofort Ahlen](../dossiers/nw/nw-ahlen-goldsofort-ring-manufaktur.md) | Betreiberseite bestätigt persönliche Edelmetallverkäufe. Impressums-HRB 11125 widerspricht dem Registerspiegel zu Ring Manufaktur GmbH HRB 17133; Dossier markiert die Kennung bereits als ungeklärt. Keine Nummer als gesichert übernehmen. |
| [NW pro aurum Bielefeld](../dossiers/nw/nw-bielefeld-pro-aurum-filiale.md) | Filialdaten und Altgoldservice gestützt; größere Mengen nur mit möglicher Terminvereinbarung. |
| [NW VFG Bonn](../dossiers/nw/nw-bonn-vfg-autoverwertung.md) | Kundenstandort 4a und Kontaktdaten durch Betreiberseite gestützt; Verwaltungsanschrift 6 und benachbarte Jungheim-Firma getrennt. Aktuelle Zertifizierung nicht geprüft. |
| [NW pro aurum Düsseldorf](../dossiers/nw/nw-duesseldorf-pro-aurum-filiale.md) | Filialdaten gestützt; einzelne Samstagsöffnungen nicht als reguläre Wochenzeiten ausgeben. |
| [NW Paul & Sohn Gronau](../dossiers/nw/nw-gronau-48599-wilhelm-paul-sohn-lager-verwaltung.md) | Lagerstandort bestätigt, konkrete Kundenannahme nicht belegt. Kartenpin ist kein Eingang; `pruefung` beibehalten. |
| [NW Schwartze & Co.](../dossiers/nw/nw-minden-schwartze-co-voth.md) | Betreiberkette und Zweigniederlassung gestützt. Öffentliche Annahme offen; verlinkte EFB-/ISO-Nachweise abgelaufen. |
| [NW Auto Rebig](../dossiers/nw/nw-monchengladbach-autoverwertung-rebig.md) | Rechtsträger gestützt, Betreiberwebsite sichtbar veraltet und aktueller Kundenbetrieb nicht belegt. Kontaktdaten leer und `pruefung` beibehalten. |
| [NW Voth Paderborn](../dossiers/nw/nw-paderborn-guenther-voth.md) | Gesellschaft, Anschrift und Kontakt gestützt; Kundenannahme und aktuelle Zertifizierung nicht belegt. `pruefung` beibehalten. |
| [NW Neumann](../dossiers/nw/nw-recklinghausen-klaus-neumann-autoverwertung.md) | Standortangaben stammen nur aus Betreiberquelle; keine unabhängig gelesene aktuelle Bestätigung. Keine Abholung oder zusätzliche Felder ergänzen. |
| [NW Markowski](../dossiers/nw/nw-sassenberg-markowski-entsorgungsfachbetrieb.md) | Betreiber, Standort und Selbstanlieferung gestützt; aktuelles EFB-Zertifikat nicht unabhängig geprüft. Keine Abholung oder Preise ergänzen. |
| [NW Cremer](../dossiers/nw/nw-weilerswist-autoverwertung-cremer.md) | Betreiberwebsite widerspricht sich bei Derkum/Ottenheim. Adresse und Felder offen, `pruefung` beibehalten; nicht lesbarer Creditreform-Treffer ist kein Beleg. |
| [SL pro aurum Saarbrücken](../dossiers/sl/sl-saarbruecken-pro-aurum-filiale.md) | Filialseite bestätigt Standort und Seed-Telefon; abweichende Impressumsnummer nicht vor die konkrete Standortnummer stellen. |
| [SN pro aurum Dresden](../dossiers/sn/sn-dresden-pro-aurum-filiale.md) | Standortdaten durch aktuelle Filialseite gestützt; leere optionale Kontakt-/Zeitfelder nicht ohne zusätzliche Owner-Prüfung füllen. |
| [ST Borgsdorf-Recycling](../dossiers/st/st-zerbst-borgsdorf-recycling.md) | Betreiber, öffentliche Annahme und aktuelles EFB-Zertifikat stützen den Standort. GEO ist adressgenauer Gebäudepunkt, kein Eingang; E-Mail-Anzeige und Mailto/Zertifikat weichen ab und sind bereits dokumentiert. |

## Seed-Änderungen und verbleibende Freigabe

- Geänderte Dossiers: nur pro aurum Berlin und Bad Homburg. Alle alten
  Timeline-Einträge blieben unverändert; Slugs, Staaten, Typen und Provenienz
  blieben erhalten. Es gibt keine neuen Emdashs oder neuen Enumwerte. Relative
  Dossierlinks wurden auf vorhandene Ziele geprüft.
- Keine neuen Preise, Annahme- oder Abholzusagen und keine Koordinaten wurden
  aus dieser Prüfung abgeleitet. Unsichere Identitäten, Annahmebedingungen,
  Zertifikate und Adressen bleiben ausdrücklich offen.
- Der vollständige historische Seed-Diff, 29 Materialdefinitionen und
  Zeitstempelwirkungen, Owner-Entscheidungen zum Gesamtrelease sowie die
  Bindung an exakt geprüfte Release-Binaries bleiben offen. Die private Probe
  deckt Migration und aktuellen Seed auf `public.db`, aber nicht diese
  vollständige fachliche Freigabe ab. Kein Deployment oder
  Produktionsschreibzugriff ist hiermit freigegeben.

## Direkt vom Owner erneut gelesene Quellen für die Feldkorrekturen

- [pro aurum Berlin](https://www.proaurum.de/unternehmen/standorte/berlin/): Ort Berlin, Hardenbergstraße 8, Kontakt, Mo-Fr 10:00-18:00, Sa 10:00-14:00 und eingeschränkter Service am Samstag.
- [pro aurum Bad Homburg](https://www.proaurum.de/unternehmen/standorte/bad-homburg/): regulärer Altgoldservice; an verkaufsoffenen Samstagen kein Direktankauf von Altgold oder Schmuck zur Schmelze.
- [pro aurum Altgoldservice](https://www.proaurum.de/leistungen/altgoldservice-und-goldankauf/): Betreiberbedingungen und Leistungsumfang.
- [Goldsofort Ahlen Impressum](https://www.goldsofort.de/impressum) und [Filialseite](https://www.goldsofort.de/filialen/ahlen): aktuelle Kontakt-/Verkaufsangaben; die bereits dokumentierte HRB-Abweichung bleibt ungelöst.
