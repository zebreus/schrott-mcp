# Welle 17.10.2026, 21:00 — Endreview A + C

## Entscheidung

**Fachliche Freigabe der vorliegenden Recherche-Deltas für genau 40 Dossiers (20 A + 20 C). Kein neuer inhaltlicher Änderungsblocker festgestellt.** Freigegeben sind die begrenzten Aussagen der neuen Texte und ausschließlich die tatsächlich geänderten Frontmatter von REUSS und Wehbach — nicht die pauschale Richtigkeit aller geerbten Felder oder die Auflösung aller Identitäten.

**Keine Commit-, Seed-/Build-, Deployment- oder Produktionsfreigabe durch dieses Review.** Diese gemeinsamen Owner-Gates bleiben ausstehend; hier keine Tests ausgeführt. Keine Aussage über Shard B oder andere Wellen.

Gelesen: README-Quellenstandard, Auswahl `/tmp/opencode/wave-20261017-2100-selection.json`, vollständige Berichte [A](wave-20261017-2100-a.md), [C](wave-20261017-2100-c.md), [C-Owner-Precheck](wave-20261017-2100-c-owner-precheck.md) und sämtliche tatsächlichen `git diff HEAD` der 40 ausgewählten Dateien. Review-HEAD: `dbb61a659a9a2f62506685cd7b2b6dd161f4c29a`; Auswahlbasis: `deca75f9e11d2446934632a96d99d2bb4f61f155`. Die 40 betreffenden HEAD-Dateien sind bytegleich zur Auswahlbasis. Fremde Arbeitsbaumänderungen wurden nicht bewertet oder verändert.

## Lesender Integritätsaudit

- Auswahl exakt 40 eindeutige Pfade; 40/40 SHA256 der Auswahlbasis stimmen mit der Auswahldatei überein.
- 40/40 vollständige historische Timeline-Textblöcke unverändert im Ergebnis enthalten, nicht nur dieselben Bullet-Zahlen. Neue Abschnitte vom 17.10.2026 vorhanden. Historische Irrtümer bleiben als Historie erkennbar und werden nicht rückwirkend umgeschrieben.
- 38/40 Frontmatter unverändert; ausschließlich REUSS (10 Felder) und Wehbach (7 Felder) geändert. Slugs, Bundesländer und GEO unverändert; keine neuen lat/lon-Felder, keine Änderung an Annahme-/Abhol-JSON.
- Frontmatter aller 40 mit YAML lesend geprüft. Keine als unbekannte Werte eingetragenen Em-Dash-Skalare `—`; Gedankenstriche in Prosa/Überschriften sind keine Feldwerte. Geänderte Enum-Werte `schrotthaendler` und `aktiv` entsprechen den vorhandenen Seed-Enums. Dies ist kein Ersatz für Compiler-/Seed-Tests.
- `git diff --check HEAD -- <genau diese 40 Pfade>` ohne Befund.
- Neue Quellenverweise auf konkrete Profile, Register, Filialdetails, Shop-Über-uns und PDF-Ausgabe statt bloßer Such-/Startseiten geprüft. Wo nur Suche, Fremdprofil oder gesperrter Abruf vorliegt, ist das ausdrücklich Lead/Abrufgrenze, keine Quellenfreigabe. Keine behauptete erneute Liveprüfung sämtlicher Links durch dieses Endreview.

## REUSS: Freigabe der zehn Feldänderungen mit enger Quellenreichweite

[Dossier](../dossiers/by/by-hof-reuss.md): `name`, `trader_type`, `street`, `phone`, `email`, `opening_hours`, `website`, `website_status`, `status`, `description` sind nachvollziehbar. Nichtleere Überschreibungen betreffen Kurzname REUSS, Typ `sonstige`, Website-Status `unbekannt` und Betriebsstatus `pruefung`; übrige Änderungen sind Leerfeldfüllungen. PLZ 95028 und Stadt Hof bleiben unverändert.

- [Betreiberseite](https://reuss-recycling.de/) mit eingebettetem Impressum: Firma, Ascher Str. 50, Hof, HRB 5936 AG Hof und GF Wolfgang Weidmann. [NorthData-Entitätsprofil](https://www.northdata.de/Reuss%20Schrotthandel%20%26%20Containerdienst%20GmbH,%20Hof/Amtsgericht%20Hof%20HRB%205936) wurde laut Owner unabhängig gelesen: Firma/Register/Adresse/GF kongruent, September-2026-Publikation. Diese unabhängige Registerbestätigung wird als Owner-Evidenz übernommen, nicht als eigener neuer Abruf ausgegeben. Kein erfundener Rechtsübergang aus dem Seed-Kurznamen.
- Unabhängigen Owner-Rohabruf vom 17.10. selbst nachgelesen: `/root/.local/share/opencode/tool-output/tool_14bb041fe001vs5FFwcmfnNOx3`; zugehöriger Text `/root/.local/share/opencode/shell/42aa29071d6c1a29b482e0269f73393202254764/sh_14bb25371001Htm1REYlZZEUF6.out`. Sichtbarer Body Zeilen 85–110 nennt Büro **Mo–Fr 08–16**, Schrottannahme **Mo–Fr 08–15:30**, Telefon und Mail. Impressum Zeilen 224–235 bestätigt die Firma. Materialliste, Annahmehinweise und Einkaufsbedingungen tragen die Beschreibung einschließlich Volljährigkeit/Ausweis, Gewerbeunterlagen, Elektrogeräte-Grenzen und Fremdstoffabzügen.
- Kontakt, Zeiten und Dienste sind Betreiberangaben, keine NorthData-Zeitenauskunft. Die registerkongruente operative Standortseite mit datiertem Aktualitätsinhalt trägt hier die README-Betreiber-Ausnahme; kein gültiges EFB-Zertifikat wird daraus behauptet. Freigabe der Wochentage als **am 17.10. sichtbare Betreiberangabe**, nicht als unbegrenzt geltende Zusicherung.
- **Samstag nicht übernehmen:** Das tatsächliche `og:description` bewirbt eine Sonderaktion ab April 2026, zweiter Samstag 08–12, und zählt Termine nur bis **10.10.2026** auf. Im gelesenen Body keine Samstagszeiten. Weder monatliche Fortsetzung nach dem letzten Termin noch generelle Samstagsöffnung oder sichere Samstagsschließung ableiten. Das aktuelle Dossier perpetuiert keine solche Öffnung; deshalb kein Korrekturblocker.
- AGB sind tatsächlich HTML-Modals auf derselben Seite, keine unabhängigen Quellen. Root-Link ist hier sachgerecht; keine erfundenen `/agb`- oder `/impressum`-DeepLinks erforderlich. Container-AGB §§4.1/7.1/7.2 tragen 30 EUR, mindestens 99 EUR bzw. 99 EUR inkl. MwSt. unter den genannten Sonderbedingungen. §7.3 enthält wirklich den Kalender-/Werktagswiderspruch bei 1,19 EUR nach vereinbarter Stellzeit (8 Tage): **nicht normalisieren**. Kein sichtbares Preis-Publikationsdatum und keine numerischen Materialankaufskurse. Einkaufs-AGB sind kein Verkaufspreisblatt. BREITZMANN/Bayreuth und gesperrter Creditreform-Abruf bleiben ausgeschlossen.

## Wehbach: Freigabe der sieben Feldänderungen, keine Ausweitung

[Dossier](../dossiers/rp/rp-kirchen-wehbacher-kirchen-wehbach.md): `name`, `trader_type`, `street`, `postcode`, `website`, `website_status`, `status` freigegeben. Nichtleere Überschreibungen sind Fragmentname, Typ `sonstige`, verunreinigtes Straßenfeld und `pruefung`; Hausnummer/Straßenkern waren bereits im Seed enthalten. Keine neue Betreiber-/Nachfolgekette.

Der [Owner-Precheck](wave-20261017-2100-c-owner-precheck.md) dokumentiert die unabhängigen Abrufe von [Impressum](https://wehbacher-rohstoffhandel.de/impressum/) und [NorthData HRB 24540](https://www.northdata.de/Wehbacher%20Rohstoffhandel%20GmbH,%20Kirchen%20(Sieg)/Amtsgericht%20Montabaur%20HRB%2024540): gleiche GmbH, Friedrichshüttenstraße 10, 57548 Kirchen, GF Jens Möbus, AG Montabaur, aktuelle Abschlussveröffentlichung 17.09.2026. Website-Schreibfehler „Montabauer“ nicht als anderes Gericht behandeln. Diese Identitäts-/Adressbestätigung trägt die begrenzte Präzisierung und Aktivierung.

Telefon/Mail/Zeiten bleiben bewusst unstrukturierte Betreiber-Selbstauskunft; **keine pauschale Einquellenfreigabe** dafür. [Schrottannahme](https://wehbacher-rohstoffhandel.de/schrott-altmetallhandel/), [Rohstoffhandel](https://wehbacher-rohstoffhandel.de/rohstoffhandel/) und [Container](https://wehbacher-rohstoffhandel.de/containerdienst/) sind konkrete Leistungs-DeepLinks, aber keine zusätzlichen unabhängigen Identitätsquellen. Preisliste auf Anfrage ist kein veröffentlichter Zahlenkurs; Containergrößen sind keine Tarife. Zertifikatsgültigkeit und konkrete Annahmekosten bleiben offen.

## Einzel-Disposition aller übrigen 38 Dossiers

Alle folgenden **Text-Deltas freigegeben, Frontmatter unverändert**. „Offen“ ist ausdrücklich keine Schließung, Aktivitätsfreigabe oder Berechtigung zum Feldfill. Quellenkorrekturen über REUSS/Wehbach hinaus wurden nicht vorgenommen.

| Shard / Dossier | Freigegebene Aussage und verbleibende Grenze |
|---|---|
| A / TSR Mannheim–Heilbronn–Stuttgart | Drei konkrete Filial-DeepLinks, getrennte Zeiten und Privatkundengrenzen; keine gemeinsame Adresse/Zeiten/GEO. Zertifikats-PDFs nur verlinkt, keine Prüfung behauptet. |
| A / Schrott Jürgen Nürnberg | Portal und fremder Vereinslead sauber getrennt; Thomas-Mann-Str. 49A/Betreiber offen. |
| A / Baur/Bayer/Graf/Bauer Ichenhausen | Impressum-/Datenschutzwiderspruch Hammerschmidt/Bayer bleibt; Sociallead kein Sammelidentitätsbeweis. |
| A / Wagner Katharina Walldürn | Portal-Homepagefehler kein Betreiberlink; Im Barnholz 5 und heutige Annahme offen. |
| A / Künle Memmingerberg | GESA-Behauptung des Portals kein amtlicher GESA-Beleg; Hausnr. 1A/20 und PLZ 87719/87766 offen. |
| A / Grundl Oberkotzau | Kommunaler Abschleppdienst-Eintrag kein Verwertungsbeleg; Instagram-Login kein geprüftes Betreiberprofil. Keine Rathauszeiten übertragen. |
| A / Wagner Michael Karlsruhe | Neuer Suchlead nicht als Vollabruf ausgegeben; Johann/Billigheim/Mosbach getrennt. |
| A / Schrott Franz Weilheim | Bauelemente-/Nachnamensverdacht plausibler Lead, keine definitive Umklassifizierung aus Portalen. |
| A / Bachinger Otting | Mehrere Unterseiten nur eine Betreiberquelle, Impressum 404; Zeiten nicht frisch bestätigt, Annahmehof offen. |
| A / ABC Schrott Esslingen | Transportfehler kein Tot-/Schließungsurteil; kostenlose Abholung bleibt unbestätigter Portal-Altlead. |
| A / Peter Maier Freiburg | Blochackerweg 7 neuer konkreter Lead, keine verifizierte Anschrift. |
| A / Hager Inzigkofen | KORN bestätigt nur KORN; keine Hager-Übernahme oder Schließung daraus. |
| A / Turrina Landsberg | Registerprofil/Alttext von angeblich aktuellem käuflichem Auszug getrennt; Alttelefon unbestätigt, Registeranschrift kein Annahmehofbeweis. |
| A / Katja Hartl Regen | Creditreform und gewerbliche Anzeige nur Suchindex/403; Reinhartsmais 1 und Anbieteridentität bleiben offen. VB kein Tarif. |
| A / Altschäffl Deggendorf | Creditreform-Kandidat mit 403, keine Volltextfreigabe; „ähnliche Firma“ keine Konzernkette. |
| A / Reinhardt Ulm | UG am gleichen Ort kein Reinhardt-Match; HTTP503 kein Schließungsbeleg. Altstatus `aktiv` ausdrücklich nicht bestätigt. |
| A / Erb Obrigheim | Pfalz-Kandidat nicht BW; HRB00000/tbd/USt-Platzhalter kein Registerbeweis. |
| A / Brandstätter Osterhofen | Negative Suche kein Nichtexistenzbeweis; Originaleintrag und Gewerbeidentität offen. |
| A / Kopp Oberpleichfeld | Unvollständige Gemeindeliste kein Negativregister; alter Telefonwert unbestätigt, Gewerbeauskunft nur benannter nächster Schritt. |
| C / Schmidt Ludwig Dillenburg | Oberscheld/Niederscheld- und GbR-/Inhaberspuren ungelöst; keine PLZ-/Adresswahl aus Portalen. |
| C / Klawes Potsdam | Gleiche Nummer an Fritz-Zubeil-/Klopstockstraße kein Verlagerungsbeweis. |
| C / Autoteile Wedding | Domainfehler keine Betriebsaufgabe; Kolonie-/Holzstraße und Betreiberhistorie offen. |
| C / Kieffer Kapellen-Drusweiler | Breiter Container-/Bagger-Portalumfang kein Auszahlungsbeweis; Michael-/Telefonkonflikt offen. |
| C / Sommerfeld Sietow | Sponsorennennung 2025 nur historischer Namens-/Ortskontext, kein aktueller Hof-/Annahmebeleg. Altstatus und Portal-Materialliste nicht neu freigegeben. |
| C / Wallwitz | Konkreter eBay-Über-uns-Tab verbindet BK mit Beutel/Kolbe und Hausnr. 7; kein unabhängiger Gewerbebeleg, keine Rasche-/Seedkette. Alte Shoppreise nicht aktualisiert. |
| C / INTERSEROH Management Perleberg | ALBA-Quitzow-Detail belegt ALBA, nicht Management→ALBA. Ludwigslust-Zwischenüberschrift kein Standortwechsel; keine Geschwisterfusion. |
| C / GK Marnheim | Nummern-/Hausnummernkonflikt und Inhaber offen; Firmenclips nicht als Betreiber-Social qualifiziert. |
| C / Kat-Ankauf Landau | Identitätsloser Portalname; Cadorin/Adressgleichheit keine Fusion. |
| C / ASR Coswig, SN-Seed | Sachsen-Anhalt-Betreiberseite keine Auflösung der SN-Provenienz; kein Fill/Merge oder DB-GEO-Transfer. HRA/HRB keine zwei Filialen. |
| C / Josef Ziegler Fulda | Kategorienmix Arzt/Altmaterial als Portalwarnsignal, nicht zwei Tätigkeiten/Firmen behauptet. Detail-Link ausdrücklich nur entdeckt. |
| C / Enders Stadtallendorf | Infobel-Fremdprofil-/Nachbarschaftslead und Zugriffssperre kein Enders-Volltext. |
| C / Neustadt-Umland-Cluster | Mehrere Einzelbetriebe sauber getrennt; keine gemeinsame Firma, kein gemeinsamer Punkt. Knepflas/Ohlinger offen. |
| C / WRS Gotha | Bereits geerbtes `geschlossen` nur für exakte gelöschte GmbH; kein neuer Schließungsfill, keine heutige Hof-/Nachfolgeraussage. |
| C / Laroche Neuwied | Denise versus Kontakt Macel, ungeprüfte zehnstellige USt-ID und Werbejahr 2022 begrenzen Betreiber-Einzelbeleg. |
| C / Gundlach Berlin | Portalwurzel kein Betreiberlink; falscher Nahbereich kein Standortbeleg. |
| C / H&S Freyburg | Konkretes kommunales Original-PDF, Ausgabe 02/2008 S. 20, trägt Holger Pilling historisch; kein Fortbestand/Adressbeweis 2026. |
| C / Hentschel Mosbach | Örtliche/Umkreistreffer keine zwei Primärquellen; Aue getrennt, heutige Kundenannahme offen. |
| C / Bügler Frankfurt | Bonames-/Nummernspuren nur Leads, Cylex 403; keine Flein- oder Dehner-Zuordnung, keine kostenlose Abholung behauptet. |

## Verbleibende Owner-Aufgaben, keine stillschweigenden Erledigungen

1. **Legacy-Risiken offen protokollieren:** Reinhardt `aktiv`, Sommerfeld `aktiv`/Materialliste sowie Turrina-/Kopp-Alttelefone erfüllen durch diese Welle keinen neuen Belegstandard. Freigabe nur der transparent begrenzenden neuen Texte, nicht der Altwerte. Reinhardt/Sommerfeld sind prioritäre Statusprüffälle; kein Schließungsbeweis und hier keine eigenmächtige Feldänderung.
2. Alle in der Tabelle genannten Identitäts-/Adress-/Nachfolgefragen bleiben Leads. Insbesondere BK/Rasche/Seed, INTERSEROH Management→ALBA, ASR SN/ST und beide Mehrfirmen-Cluster sind **nicht gelöst**. Keine fremden Betreiberfakten transferieren.
3. Keine neuen numerischen Ankaufkurse freigeben. Historische Verkaufspreise nicht zu aktuellen Kursen machen; REUSS-Sondergebühren bleiben bedingt und ohne erfundenes Gültigkeitsdatum. Widersprüchlichen Tagestarif vor Verwendung beim Betreiber klären.
4. Gemeinsame Seed-/Codegen-/Deploymentprüfung durch Owner separat nachweisen. Dieser Bericht ist ausschließlich das A/C-Fachreview; er führt weder Tests noch Produktionsabfragen, GEO-Arbeit oder Commits aus.

Einzige durch dieses Review geschriebene Datei: `research/wave-20261017-2100-ac-owner-final.md`.
