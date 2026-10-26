# Audit der Statusübergänge (`pruefung → aktiv`, `aktiv → geschlossen`), 26.10.2026

## Maßstab und Grenze

Geprüft wurden 28 Dossier-Übergänge `pruefung → aktiv` und ein Übergang
`aktiv → geschlossen` in drei überschneidungsfreien Shards (8 + 8 + 12
Aktivierungen). Die Dossiers und ihre Git-/Recherchehistorien wurden gegen die
README-Quellenhierarchie geprüft. Die README definiert `pruefung` als offene
Prüfung des Ankaufstatus; eine aktive Firma oder kostenlose/kostenpflichtige
Abfallannahme ist deshalb nicht automatisch ein bestätigter Ankauf. Aggregatoren
sind Leads, keine Evidenz. Für Identität gilt der Zwei-Quellen-Standard oder die
vollständig erfüllte autoritative Betreiber-Ausnahme.

Die Dossierhistorien dokumentieren Übergänge zwischen 17. und 25.10.2026. Eine
private Übergangsprobe ist kein Nachweis eines Produktions-Rollouts. Für Shard A
wurde der Produktionsstatus nicht frisch abgefragt. Die Read-only-Proben in
Shard B und C fanden alle 20 dort geprüften Aktivierungen in Produktion noch
auf `pruefung`; der geschlossene A.S.S.-Datensatz steht in Produktion noch auf
`aktiv`. Der Owner korrigierte den Quelldossierstatus von Jäger am 26.10. von
`aktiv` auf `pruefung`, weil der Ankaufstatus nicht ausreichend belegt ist. Es
gab keinen Produktions-DB-Schreibzugriff und keinen Deploy.

## Disposition pro Dossier

| Dossier | Übergang / Disposition | Quellen- und Evidenzgrund | Offen |
|---|---|---|---|
| [Scholz Marienberg](../dossiers/sn/sn-marienberg-scholz-recycling-betrieb-marienberg.md) | 18.10.; **trägt als aktiver Schrottplatz mit Annahme** | Aktueller Betreiberfinder samt konkretem Standort, aktuellem ESN-Zertifikat und nachträglich verifiziertem HRB erfüllen die dokumentierte Betreiber-Ausnahme. Betreiberweite Selbstanlieferungsregel gilt für den gelisteten Standort. | Keine örtliche Ankaufspreisliste; Preis auf Anfrage ist keine Zusage für jede Fraktion. Samstag bleibt widersprüchlich. Keine Änderung nötig. |
| [Nehlsen Radebeul](../dossiers/sn/sn-radebeul-nehlsen-sachsen-standort-radebeul.md) | 25.10.; **trägt als aktiver Wertstoffhof, nicht als Ankauf** | Aktuelle Betreiber-Standortseite weist konkrete Anlieferungszeiten und kostenfreie Schrottannahme aus. NorthData HRA 7257 und Gesellschaftsseite stützen die Betreiberidentität; das Registerprofil nennt eine Veröffentlichung vom 10.09.2026. | Kostenfreie Abgabe ist ausdrücklich kein Ankauf. Keine Auszahlung/Ankaufskurse. GEO offen. |
| [Hofmann Elstertrebnitz](../dossiers/sn/sn-elstertrebnitz-schrott-wetzel-depot-elstertrebnitz.md) | 22.10.; **trägt als aktive Annahme-/Aufbereitungsstelle, Ankauf offen** | Betreiber-Standortseite nennt Eröffnung 13.08.2026, aktuelle Stunden und Betriebsausstattung; der Insolvenzverwalter belegt unabhängig Übernahme und geplante Annahme/Aufbereitung für Privat und Gewerbe. Alter Wetzel-Ost-Standort ist sauber von Schrott Wetzel Mannheim getrennt. | Quellen belegen Annahme, nicht Auszahlung oder konkrete Ankaufskonditionen. Keine Hausnummer/Zufahrt für GEO. |
| [Kühn Karlsruhe](../dossiers/bw/bw-karlsruhe-kuhn-entsorgung.md) | 18.10.; **trägt als aktiver Entsorgungsbetrieb, nicht als Ankauf** | Konkrete Betreiber-Standortseite, gültiges EFB-Zertifikat und NorthData HRB 716354 belegen Identität, Standort und laufenden Entsorgungsbetrieb. Metallannahme ist nur nach Termin/Anmeldung beschrieben. | Keine Vergütung oder Auszahlung belegt; Annahme/Kundenzugang nicht mit Ankauf gleichsetzen. Keine Änderung nötig. |
| [Dinkel Stuttgart-Degerloch](../dossiers/bw/bw-stuttgart-mohringen-dinkel.md) | 22.10. (Recherche 21.10.); **trägt als aktive öffentliche Annahmestelle, nicht als Ankauf** | Betreiber-Standortseite belegt Selbstanlieferung von Schrott/Metall und Zeiten; NorthData HRB 10644 stützt Gesellschaft und Umbenennung. | Keine Auszahlung oder Ankaufspreise belegt; GEO weiter offen. |
| [G & K Utsch](../dossiers/th/th-rudolstadt-g-u-k-recycling-utsch.md) | 22.10.; **trägt als aktiver Entsorgungsbetrieb, nicht als Ankauf** | Betreiber-Impressum/Annahmebedingungen nennen HRA 200995 und aktuelle Anlieferung nach Anmeldung und bestätigtem Angebot. Direkt gelesener NorthData-Eintrag bestätigt den Rechtsträger; seine Anschrift ist nur auf Ortsebene und ersetzt nicht die konkrete Betreiberadresse. | Angebot, Wiege-/Rechnungsgrundlage und mögliche Gebühren belegen keine Auszahlung an den Lieferanten. Keine Kaufkurse; GEO offen. |
| [Jäger Schmalkalden](../dossiers/th/th-schmalkalden-jager-jurgen.md) | 22.10.; Owner-Korrektur 26.10.: `aktiv → pruefung` | Git-Historie zeigt `pruefung → aktiv`; die dazu passende zweite Quelle ist Gelbe Seiten. Laut README ist das ein Aggregator-Lead und zählt nicht als zulässiger Beleg. Die Betreiberwebsite beschreibt den laufenden Autoverwertungsbetrieb, aber keinen allgemeinen Schrottankauf; ein unabhängiger aktueller Ankaufbeleg fehlt. | Kein Schließungs-/Inaktivitätsgegenbeleg. Ankaufstatus bleibt offen. Straße, PLZ, Telefon und E-Mail wurden mangels zweiter zulässiger Quelle aus den typisierten Feldern entfernt. Der Produktionspunkt liegt laut Reverse-Abfrage an Eichelbach 2 statt Burgweg 1a; `clear_fields: 'email,geo,phone'` und leere Adressfelder bereiten das Zurücknehmen der ungeprüften Werte vor. Seed/Deploy noch nicht erfolgt. |
| [Wagner Erfurt](../dossiers/th/th-erfurt-wagner-recycling.md) | 22.10.; **trägt als aktives Recyclingunternehmen, nicht als öffentlicher Annahmehof/Ankauf** | Betreiber-Leistungsseite und direkt gelesener NorthData-Eintrag stimmen bei Wagner Recycling GmbH, HRB 113325 und Salinenstraße 91 überein; das Profil nennt eine Veröffentlichung vom 11.02.2026. Der Betreiber bewirbt Aufbereitung in Lohnarbeit, Verwertung und Entsorgung. | Keine öffentliche Selbstanlieferung und keine Auszahlung belegt. Registerexistenz ist kein Beleg für Publikumsannahme oder Ankauf. GEO nicht als Hofpunkt behandeln. |

## Ergänzende Shards B und C

Die konkreten Quellen und historischen Einzelfunde stehen jeweils in den
verlinkten Dossier-Timelines. „Aktiv“ in der Disposition meint hier den dort
belegten fortbestehenden Betrieb oder eine belegte Schrott-/Metallleistung; es
ist nicht pauschal eine Aussage über Vergütung, öffentliche Annahme oder jede
Materialfraktion.

| Dossier | Disposition und Evidenzgrenze |
|---|---|
| [Fritsch Hagen](../dossiers/nw/nw-hagen-fritsch.md) | Aktivierung trägt: Betreiberseite/Impressum und NorthData stützen Identität, Standort, ausdrücklichen Schrottankauf und Tagespreise. Keine Zahlenpreise; Containerwechsel gratis ist keine allgemeine Gratisabholung. |
| [Schrotthändler Plus Bochum](../dossiers/nw/nw-bochum-wattenscheid-schrotthandler-plus.md) | Trägt als mobiler Ankauf: aktuelle Betreiberseite nennt Schrott-/Altmetallankauf, Angebot und Auszahlung; für zu kleine Mengen kostenlose Mitnahme. Keine stationäre Annahme oder feste Mindestmenge belegt. |
| [Lusch Wuppertal](../dossiers/nw/nw-wuppertal-lusch-heinz.md) | Aktivierung trägt: Betreiberseiten und NorthData stützen Betrieb und ausdrücklichen Schrott-/Metallankauf. Keine Zahlenpreise; Abholung größerer Mengen ab etwa 2 t. |
| [Altfeld Gevelsberg](../dossiers/nw/nw-gevelsberg-altfeld.md) | Trägt als aktiver Verarbeitungsbetrieb: Betreiberseiten/Register belegen Aufbereitung und Entsorgungsleistungen. Öffentliche Anlieferung und vergüteter Ankauf am Werk nicht belegt; Ankauf bleibt `unklar`. |
| [Lietmann Wuppertal](../dossiers/nw/nw-wuppertal-elberfeld-ewald-lietmann.md) | Aktivierung trägt: Betreiberseiten/Register belegen Metall-/Schrottankauf am Betriebsgelände; einmalige angekündigte Schließung am 02.12.2026 ist kein Schließungsnachweis. Keine Zahlenkurse. |
| [SRG Siegen](../dossiers/nw/nw-siegen-srg-schrott-und-recycling.md) | Trägt als aktiver Schrotthandelsstandort: Betreiber-Standortseite und Registerbeleg stützen Standort/Betrieb. SRG-spezifische Privatannahme und Konditionen bleiben offen. |
| [Lohmann Emsdetten](../dossiers/nw/nw-emsdetten-lohmann.md) | Aktivierung trägt: Betreiberseite und NorthData stützen Metallannahme und Vergütung werthaltigen Schrottes. Lohmann Containerdienst GmbH bleibt rechtlich getrennt. |
| [Tamm Wuppertal](../dossiers/nw/nw-wuppertal-ronsdorf-tamm.md) | Trägt als aktiver Entsorgungs-/Recyclingbetrieb mit Schrottannahme. Bar-/Kartenzahlung bezieht sich auf Entsorgung, nicht belegte Auszahlung an Lieferanten. |
| [RWE/ALBA Wismar](../dossiers/mv/mv-wismar-nord-rwe-umwelt-mecklenburg-vorpommern-contai.md) | Bedingt tragen lassen: Registerkette und aktuelle ALBA-Standortseite stützen Wismarer Betrieb. Belegt ist Entsorgung/Containerdienst, kein Metallankauf am Standort; ALBA Metall Nord ist getrennt. |
| [Sassnitz/Rügener Mineralölhandel](../dossiers/mv/mv-sassnitz-schrottannahme-rugener-mineralolhandel.md) | Bedingt tragen lassen: Betreiberseite belegt aktuelle kostenfreie Schrottannahme; Identitätsbelege sind teilweise eingeschränkt, Datenschutzseite nennt abweichenden Verantwortlichen. Kein Ankauf/Auszahlung belegt. |
| [Hochscheider & Gondert Trier](../dossiers/rp/rp-trier-54293-hochscheider-gondert.md) | Trägt als aktiver Metall-/Schrotthandelsbetrieb: Betreiberquelle, NorthData und Zertifikate stützen Betrieb. Private Annahme bleibt wegen Website-Widerspruch offen. |
| [Wehbacher Kirchen](../dossiers/rp/rp-kirchen-wehbacher-kirchen-wehbach.md) | Trägt: Betreiber-Impressum/Register stützen Identität und Betrieb; Betreiberseite nennt Schrottannahme. Preise nur auf Anfrage; GEO/Zugang und EFB-Gültigkeit offen. |
| [Bellersheim Boden](../dossiers/rp/rp-boden-56412-bellersheim-boden.md) | Trägt mit enger Lesart: Altmetallservice und Vermarktung von Kundenmaterial sind belegt; dies ist kein Beleg für Direktankauf. Keine Zahlenkurse/Gebühren. |
| [1A Recycling Neuwied](../dossiers/rp/rp-neuwied-56566-1a-recycling.md) | Trägt: Betreiberseite/Register belegen Betrieb und Schrott-/Fahrzeugankauf. Öffnungszeiten, Zufahrt und Zertifikatsgültigkeit bleiben offen; Dossier widerruft GEO, Produktionspin ist noch vorhanden. |
| [Weinand Bad Kreuznach](../dossiers/rp/rp-bad-kreuznach-weinand-entsorgung-recycling.md) | Trägt als Metallannahme mit Tagesmarktpreisen laut Betreiberseite. Zuordnung des Metallgeschäfts zu einer der zwei Gesellschaften und GEO bleiben ungeklärt; Dossier widerruft GEO, Produktionspin besteht noch. |
| [Reuss Hof](../dossiers/by/by-hof-reuss.md) | Trägt: Betreiberseite/Register belegen laufenden Betrieb und ausdrücklichen Schrott-/Metallankauf. Keine Zahlenkurse; GEO und künftige Samstagszeiten offen. |
| [Müller Weener](../dossiers/ni/ni-weener-muller-schrotthandel.md) | Trägt: Betreiberwebsite und Betreiber-Facebook stützen Betrieb, Direktanlieferung, Verwiegung und Auszahlung. Widerspruch bei Elektroartikeln bleibt; keine Zahlenkurse/GEO. |
| [Heitec Salzgitter](../dossiers/ni/ni-salzgitter-heitec-black-metals.md) | Trägt: Betreiberseite, NorthData und 2026-Zertifikate stützen Standort und Metallannahme mit täglichen Ankaufspreisen. Keine veröffentlichten Zahlenkurse; Ausschlüsse beachten. |
| [Erhardt/Molkenthin Tangerhütte](../dossiers/st/st-tangerhutte-molkenthin-erhardt-metall-rohstoff-recyc.md) | Trägt mit Quellenvorbehalt: aktuelle Betreiberseite nennt Ankauf und direkte Vergütung; Register-Suchindex ist nur eingeschränkt direkt prüfbar. Kontaktangaben der Unterseite widersprechen der Hauptseite. |
| [Brameier Schopsdorf](../dossiers/st/st-schopsdorf-brameier.md) | Bedingt tragen lassen: Shop-/Impressum-, Register- und Stellen-Spuren belegen aktiven Teilehandel der GmbH, aber keinen aktuellen Fahrzeug-/Schrottankauf. Telefon und GEO offen. |
| [A.S.S. Recycling Barsbüttel](../dossiers/sh/sh-barsbuttel-a-s-s-recycling.md) | `aktiv → geschlossen` trägt für genau die A.S.S. Recycling GmbH HRB 10762 HL: Registerblatt geschlossen, Löschung 03.06.2025. Kein Nachfolger am Kielende identifiziert; keine Aussage über Schließung des Standorts. Produktion steht noch auf `aktiv`. |

## Gesamturteil

In Shard A bleiben sieben der acht Dossiers `aktiv`; Jäger wurde auf `pruefung`
zurückgesetzt, weil die dokumentierte Aktivierung auf einem Aggregator als
vermeintlichem Zweitbeleg beruhte und der Ankaufstatus offen ist. In Shards B/C
tragen die meisten Statusübergänge als laufende Betreiber oder belegte
Annahme-/Ankaufstellen; mehrere haben ausdrücklich offene Grenzen bei Ankauf,
Publikumsannahme, Rechtsträger oder GEO. Die A.S.S.-Schließung gilt nur für die
gelöschte GmbH, nicht pauschal für den Standort. Keine Disposition belegt
automatisch einen bezahlten Ankauf aller Materialien.

Die Ergebnisse beziehen sich auf den Dossier-/Seedstand. Produktionswerte aus
Shard A wurden nicht frisch verifiziert; Read-only-Abgleiche für Shards B/C
bestätigen dort noch nicht ausgerollte Dossieränderungen. Jägers GEO-Rücknahme
ist ebenfalls nur im Quelldossier vorbereitet und noch nicht angewendet. Es
erfolgte kein Deploy.
