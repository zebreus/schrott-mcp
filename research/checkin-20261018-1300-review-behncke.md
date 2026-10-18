# Check-in 18.10.2026, 13:00 – Walter Behncke: Kontakt-/Annahmeprüfung

## Ergebnis / Owner-Empfehlung

**Gezielte Rücknahme von `phone` und `dropoff_json` als unbekannt empfehlen; `status: pruefung` halten.** Weder das Alttelefon **040 5508707** noch die aktuell in Produktion behauptete Kundenanlieferung `{"allowed":true}` erreichen den README-Quellenstandard. Keine Schließung, keine Ablehnung von Kunden/Materialien und keine Zusammenführung mit Schrieber festgestellt. Auch bedingte Annahme („nur Gewerbe“, „nach Termin“ usw.) wäre ohne Quelle erfunden.

Neu gegenüber der früheren Recherche: NorthData war diesmal tatsächlich lesbar und liefert **AG Hamburg HRA 76337 / EUID DEK1101R.HRA76337** sowie historische Inhaberangaben. Ein konkreter Creditreform-Link wurde gefunden, sein Direktabruf ist jedoch **403-gesperrt**. Das verbessert die Identitäts-Rechercheansätze, legitimiert aber weder heutige Telefonnummer noch Kundenannahme.

## Scope und Maßstab

- Geprüft: `dossiers/hh/hh-schnelsen-walter-behncke-inh-bernd-behncke.md`, `README.md` (insbesondere Z.183–202) und `docs/trader-seed.md` (Z.22–85).
- Produktionsstand **laut übergebenem lesendem 13:00-Abgleich**, hier nicht selbst erneut aus der DB gelesen: Telefon `0405508707`, `dropoff_json={"allowed":true}`, Status `aktiv`. Lokal gelesen: Telefon `040 5508707`, Drop-off leer, Status `pruefung`.
- Aggregatoren sind nur Leads. Grundsätzlich zwei unabhängige zulässige Quellen; die Einquellen-Ausnahme verlangt eine registerkongruente, aktuelle Betreiberwebsite mit Impressum/Registerkennung und konkreten Standortseiten. Eine solche Betreiberquelle wurde nicht identifiziert. Ein Register-Identitätsbeleg beweist nicht automatisch Kontakt oder Annahme.

## Tatsächlich gelesene Quellen / Zugriffsgrenzen (18.10.2026)

| Quelle | Tatsächlicher Befund | Aussagegrenze |
|---|---|---|
| [NorthData-Suggest](https://www.northdata.de/suggest.json?query=Walter%20Behncke&countries=DE) | Antwort mit genauem Unternehmenslink „Walter Behncke e. K.“, Hamburg. Diesmal keine 429. | Suchzuordnung, noch kein Betriebs-/Kontaktbeleg. |
| [NorthData-Unternehmensdetail](https://www.northdata.de/Walter%20Behncke%20e%C2%B7%20K%C2%B7,%20Hamburg/HRA%2076337) | Direkt gelesen: Walter Behncke e. K.; AG Hamburg HRA 76337; EUID DEK1101R.HRA76337; Ortsangabe nur „Hamburg, Deutschland“. Publikationen: **17.04.2002** Eintragung, Inhaber Bernd Behncke, Firmensitz Hamburg, Rechtsform e. K.; **24.03.2010** „Anschrift“. | Keine gelesene konkrete aktuelle Straßenanschrift, Telefonnummer, Annahmebedingungen oder aktuelle Statusbestätigung. Historische Eintragung nicht als heute nachgewiesener Betrieb verwenden. Premium-Angebot nicht geöffnet/umgangen. |
| [Creditreform: Walter Behncke -Schrottplatz-](https://firmeneintrag.creditreform.de/22457/2150124974/WALTER_BEHNCKE_SCHROTTPLATZ) | Konkrete URL über Suchtreffer gefunden; **Direktabruf HTTP 403**. Suchsnippet nennt Hamburg, Einzelfirma, HRA 76337. | Snippet ist kein selbst gelesener Creditreform-Einzelnachweis; keine frische Bestätigung des Altvermerks vom 02.10., keine Telefon-/Annahmeaussage. Sperre nicht umgangen. |
| [Nichtamtliches Behncke-Profil](https://schrottregister.pages.dev/betrieb-walter-behncke-e-k-inh-bernd-behncke-hamburg) | Name/Inhaber, Kulemannstieg 32, 22457 Hamburg, Lagern/Behandeln, 17 Abfallschlüssel und Kennung **ZZBT00500176**. Wörtlich: „Das Register nennt weder Telefon noch E-Mail — und wir veröffentlichen keine.“ Betreiber entscheidet laut Seite selbst, ob/wann Ankauf zustande kommt. | Seite bezeichnet sich als unabhängiges, nichtamtliches Angebot, Datenabruf **11.08.2026**. Kein heute direkt gelesenes gültiges Zertifikat; keine aktuelle Selbstanlieferzusage. Abfallschlüssel/Tätigkeit ersetzen keine Kundenbedingungen. |
| [Schrottregister-Ortsseite](https://schrottregister.pages.dev/ort-hamburg), [Datenerläuterung](https://schrottregister.pages.dev/daten) | Behncke separat an **32**, Gebrüder Schrieber G.m.b.H. separat an **33**. Datenerläuterung: bislang eine Momentaufnahme vom 11.08.2026. | Gleicher Spiegel, keine zweite unabhängige Quelle. Weder Nachbartelefon noch Nachbarzeiten/-leistungen übertragen. Keine Betreiberkette bewiesen. |
| [Amtliches Fachbetrieberegister](https://fachbetrieberegister.gadsys.de/fachbetrieberegister/) | HTML direkt gelesen: Vaadin-Anwendung, `appName="eEFBV FBR"`, leerer `outlet`; keine Unternehmensdaten. [EFB-Hilfeversuch](https://fachbetrieberegister.gadsys.de/fbrwiki/efb-info): HTTP 503. | Keine erfolgreich ausgeführte Unternehmenssuche, kein positiver oder negativer Behncke-Einzelbefund; ZZBT00500176 bleibt Suchschlüssel, nicht frisch amtlich validiertes Zertifikat. |
| [Gemeinsames Registerportal](https://www.handelsregister.de/rp_web/welcome.xhtml) | Startseite lesbar, zugleich Session-/Anwendungsfehlermeldungen; keine Einzelauskunft abgerufen. | Kein aktueller Registerauszug und kein Löschungs-/Fortbestandsbefund. |

## Suchabdeckung und Grenzen

Die integrierte Websuche scheiterte mit HTTP 401; Google lieferte nur die Suchzugriffshinweisseite. Bing-RSS mit [exaktem Firmennamen](https://www.bing.com/search?format=rss&q=%22Walter+Behncke%22) lieferte den konkreten Creditreform-Link und Aggregator-Leads, aber keine verifizierbare Betreiberwebsite. Zusätzliche RSS-Suchen nach `"Walter Behncke" "Telefon"`, `"0405508707"`, `"Behncke" "Kulemannstieg" "Annahme"`, `"Walter Behncke" site:hamburg.de` und `"ZZBT00500176"` waren überwiegend sachfremd; insbesondere der vermeintliche hamburg.de-Filter lieferte keine verwendbare kommunale Quelle. Keine Negativaussage aus diesen Suchfehlern ableiten. Ein ZKS-Abruf unter `https://www.zks-abfall.de/entsorgungsfachbetriebe/` scheiterte mit Transportfehler.

Damit wurde **kein konkreter heute gelesener zulässiger Telefon- oder Kundenannahmenachweis** gewonnen. Ebenso wurde keine Betreiber-social-Präsenz sicher zugeordnet. Eine bloße Wiederholung der Telefonnummer in Verzeichnissen wäre nach README ohnehin kein tragfähiger Ersatz. Die Recherche beweist nicht, dass die Nummer falsch oder Annahme unmöglich ist; sie zeigt die fehlende Grundlage für die öffentliche positive Behauptung.

## Konkreter, hier NICHT ausgeführter Änderungsvorschlag

Nach Owner-Review im Dossier:

```yaml
phone: ''
dropoff_json: ''
clear_fields: 'phone,dropoff_json'
status: pruefung
```

`clear_fields` wird kanonisch sortiert; die gezeigte Eingabereihenfolge ist zulässig. Das Telefon muss für diese Direktive zugleich geleert werden, sonst liegt ein Validierungskonflikt vor. Erwartete Speichersemanik: `phone=''`, `dropoff_json={}`; fehlendes `allowed` bedeutet **unknown**, ausdrücklich nicht `false`. Eine reine Leerung des Dossier-Drop-offs oder die Statusabstufung entfernt das vorhandene `{"allowed":true}` **nicht**. GEO, Anschrift und übrige Felder nicht anfassen; deren Erhaltung ist keine erneute Verifikation.

Historische Telefonnummer und Annahmebehauptung in einer datierten Timeline als zurückgenommene, nicht aktuell belegte Bestandsaussagen erhalten. Die Direktive ist ein zukünftiger Owner-Beschluss, keine in dieser Recherche vorgenommene Änderung oder Publikationsbehauptung. Rebuild/Seed und lesender Produktionsnachcheck wären danach separat nötig.

## Was eine spätere Neubewertung ermöglichen würde

1. Amtliche Einzelauskunft mit HRA 76337 bzw. gültiges EFB-Einzelzertifikat über ZZBT00500176 zur Standort-/Betreiberidentität; Gültigkeit ausdrücklich prüfen.
2. Aktuelle, dieser Identität sicher zugeordnete Betreiberveröffentlichung zur Nummer und zur Kundenanlieferung (Kundengruppen, Termin-/Mengen-/Materialbedingungen). Registerexistenz oder Abfallbehandlung allein reicht dafür nicht. Ohne qualifizierte autoritative Betreiberwebsite weiterhin Zwei-Quellen-Standard beachten.
3. Kein Nachbartransfer von Schrieber an 32/33. Keine Schließung oder Kundenablehnung aus Zugriffsfehlern, fehlender Website oder Rechercheleere ableiten.

**Änderungsumfang dieses Auftrags:** ausschließlich dieser Research-Bericht; keine Dossier-/DB-/GEO-Änderung, kein Commit, kein Deploy. Eine vom Research-Skill angeregte Unterdelegation war durch das Subagent-Tiefenlimit nicht möglich; Recherche selbst durchgeführt.
