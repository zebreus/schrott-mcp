---
slug: be-online-schrott24-online-ankauf-partnernetz
name: Schrott24 (Online-Ankauf/Partnernetz)
trader_type: schrotthaendler
state: BE
city: online
street: ''
postcode: ''
phone: ''
email: ''
opening_hours: ''
website: https://www.schrott24.de
website_status: aktiv
status: aktiv
description: 'Online-Altmetallankauf ueber Partnernetz (Metaloop Europe GmbH, Graz); tagesaktuelle Schrottpreise, Abholung/Logistik, kein Berliner Betriebssitz. ANKAUF: Die Plattform veroeffentlicht eine maschinenlesbare, mengengestufte Ankaufspreisliste je Sortengruppe (JSON-Endpunkt `/page-data/altmetall-ankauf/<kategorie>/<produkt>/page-data.json`, z.B. Kupfer Millberry 8,20 EUR/kg ab 3 kg / 8,49 ab 100 kg / 8,79 ab 500 kg / 9,04 ab 10.000 kg; Mischschrott E1 0,16 EUR/kg ab 100 kg bis 0,24 EUR/kg ab 20.000 kg). Es handelt sich um Plattform-MAXIMALpreise fuer Deutschland/Oesterreich, nicht um einen lokalen Berliner Händlerpreis; die Preisseite weist als Datenstand "Tagesaktuelle Schrottpreise vom 14.11.2025" aus, die Produkt-JSONs selbst tragen kein Datum. VERKAUF: kein Endkundentarif fuer den Verkauf von Metall an Schrott24 publiziert. GEBUEHREN: 5 EUR Abzug je genutzter Versandmarke, Big Bags 5 EUR je Stueck zuzueglich Versand, Selbstanlieferung erst ab 100 EUR Materialwert, Stornogebuehren bei nicht fristgerechter Anlieferung (14 Tage).'
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Berlin (BE) — Recherche
provenance_ankauf_raw: ja (online)
provenance_origin: table
---

# Schrott24 (Online-Ankauf/Partnernetz)

## Überblick

Schrott24 ist ein bundesweit nutzbares Online-Ankaufs- und Logistikangebot der Metaloop Europe
GmbH (Graz), kein Berliner Betriebsstandort. Für Berlin vermittelt die Plattform Verkauf/Abholung
oder eine Partner-Anlieferstelle; deren konkrete Anschrift und Öffnungszeiten werden nach Online-Verkauf
mitgeteilt.

**Neue Erkenntnis (28.10.2026): Die Plattform hat eine maschinenlesbare, mengengestufte
Ankaufspreisliste.** Der Betreiber publiziert je Produkt einen JSON-Endpunkt
`https://www.schrott24.de/page-data/altmetall-ankauf/<kategorie>/<produkt>/page-data.json`
(HTTP 200, `application/json`, ohne Anmeldung). Das `price`-Feld trägt **Mengenstaffeln in
EUR/kg**: `{"start": 3, "price": 8.2}, {"start": 100, "price": 8.49}, {"start": 500, "price": 8.79},
{"start": 10000, "price": 9.04}` für "Kupfer, blank I, Millberry". Über die 94 Produktseiten der
Sitemap wurden **67 Sorten mit Preisen** erfasst.

**Wichtige Einordnung (README-Regel: Quellen-Trennung und Preisart):** Das sind
**Plattform-Maximalpreise für Deutschland/Österreich** - die Betreiberseite sagt das ausdrücklich
("Angezeigt werden Maximalpreise aus Deutschland und Österreich", "Preise sind abhängig von Ihrem
Standort und der gewählten Transportart"). Sie sind **kein lokaler Berliner Händlerpreis** und
**keine Zusage für einen bestimmten Annahmehof**. Die Erklärseite `/schrottpreise/` weist als
sichtbaren Datenstand "Tagesaktuelle Schrottpreise vom **14.11.2025**" aus; die Produkt-JSONs selbst
tragen **kein Datum**. Deshalb werden die Kurse hier als Plattform-Kontext dokumentiert, nicht als
Händler-Ankaufskurve.

**Preise (Ankauf / Verkauf / Gebühren) getrennt:**

- **Ankauf (Plattform-Maximalpreis, EUR/kg, mengengestuft, ohne Datum in den JSONs):**
  - **Kupfer:** Millberry (blank I) 8,20 ab 3 kg / 8,49 ab 100 / 8,79 ab 500 / 9,04 ab 10.000 kg;
    Kerze (blank II) 8,00 / 8,25 / 8,55 / 8,75; Berry (schwer, min. 98 % Cu) 7,65 / 7,73 / 7,98 /
    8,20; Raff (gemischt, alt) 7,45 / 7,56 / 7,75 / 8,00; Kupfer-Messing-Kühler rein 4,24 / 4,32 /
    4,52 / 4,80; Kupfer-Messing-Kühler behaftet 3,73 / 3,81 / 3,91 / 3,91.
  - **Messing:** MS58 neu stückig 5,06 ab 3 kg / 5,22 ab 100 / 5,44 ab 500 / 5,79 ab 10.000;
    Patronenhülsen 5,29 / 5,49 / 5,69 / 6,14; gemischt 4,48 / 4,58 / 4,68 / 5,01; gelbe Späne
    4,42 / 4,52 / 4,62 / 4,82.
  - **Rotguss:** stückig 6,99 / 7,14 / 7,24 / 7,59; Späne 6,59 / 6,69 / 6,99 / 7,34.
  - **Kabel:** min. 70 % Cu 4,65 ab 3 kg / 4,75 ab 100 / 4,85 ab 500 / 5,10 ab 10.000; min. 60 % Cu
    3,99 / 4,09 / 4,29 / 4,39; min. 38 % Cu 2,61 / 2,75 / 2,89 / 2,94; mit Stecker 2,01 / 2,11 /
    2,15 / 3,37.
  - **Aluminium:** Alublech blank neu 1,73 / 1,83 / 1,93 / 2,14; Alublech alt färbig 1,35 / 1,45 /
    1,55 / 1,82; Alufelgen sauber 1,63 ab 1 kg / 1,66 ab 50 / 1,76 ab 250 / 1,86 ab 500;
    Alufelgen behaftet 1,56 / 1,66 / 1,76 / 2,03; Aluguss neu 1,15 / 1,25 / 1,33 / 1,54;
    Aluguss alt 1,05 / 1,15 / 1,20 / 1,45; Aluspäne 0,80 ab 10 kg / 0,90 / 1,00 / 1,29; gemischt
    1,25 / 1,30 / 1,40 / 1,79; Alu-Kupfer-Kühler rein 3,44 / 3,54 / 3,59 / 3,72; Alu-Offsetplatten
    1,78 / 1,88 / 1,98 / 2,30.
  - **Edelstahl:** V4A Schrott 1,70 / 1,80 / 1,90 / 2,35; Edelstahlspäne (Nirosta) 0,65 / 0,70 /
    0,75 / 0,85; Kaltbandschrott 0,80 / 0,85 / 0,95 / 1,69.
  - **Eisen und Stahl:** Mischschrott E1 0,16 ab 100 kg / 0,20 ab 1.000 / 0,22 ab 10.000 / 0,24 ab
    20.000; Schwerschrott E3 0,18 ab 100 / 0,21 ab 1.000 / 0,25 ab 10.000 / 0,24 ab 20.000.
  - **Zink:** neu 1,81 / 1,91 / 1,95 / 2,10; alt 1,50 / 1,60 / 1,77 / 2,94.
  - **Blei:** Altblei weich 1,15 ab 5 kg / 1,18 ab 100 / 1,43 ab 500 / 1,55 ab 5.000; Auswuchtblei
    0,45 ab 30 kg / 0,45 / 0,50 / 0,60; Bleibatterien 0,30 ab 50 kg / 0,33 ab 300 / 0,40 ab 500 /
    0,50 ab 5.000.
  - **Zinn:** rein min. 99 % 18,48 ab 1 kg / 19,68 ab 50 / 22,00 ab 250 / 22,00 ab 2.500;
    Geschirrzinn 16,40 ab 2 kg / 17,40 ab 50 / 18,50 ab 250 / 20,00 ab 2.500; Zinnschrott/Lötzinn
    13,00 (2/50/250/5.000 kg).
  - **Hartmetall/HSS:** Hartmetall Bohrer und Fräser 23,00 ab 1 kg / 24,00 ab 100 / 26,00 ab 350 /
    27,00 ab 2.500; Hartmetall Wendeschneidplatten 22,05 / 23,00 / 25,00 / 25,50; Hartmetall gemischt
    19,40 / 20,40 / 22,00 / 22,50; HSS gemischt 0,98 ab 1 kg / 1,68 ab 50 / 1,76 ab 250 / 1,80 ab 500.
  - **Elektro:** Elektro-Motoren Low Grade 0,50 ab 10 kg / 0,55 / 0,60 / 0,68; Kleintrafos 0,55 /
    0,60 / 0,65 / 0,65; Silberkontakte 8,80 ab 1 kg / 8,85 ab 50 / 8,85 / 8,85; Speichermodule mit
    Au-Kante 0,05 ab 100 kg / 0,05 / 0,05 / 0,06.
  - **E-Schrott/IT:** Leiterplatten Klasse 1A alt 10,25 ab 5 kg / 10,25 ab 30 / 10,64 ab 100 /
    11,36 ab 1.000; RAM mit Gold-Kontakten 37,20 ab 2 kg / 37,20 / 38,95 / 38,95; RAM mit
    Silber-Kontakten 15,20 / 15,20 / 16,25 / 16,25; Leiterplatten 1B neu 2,95 / 2,96 / 3,23 /
    3,76; Leiterplatten Klasse 2 2,42 / 2,51 / 2,67 / 2,67; Klasse 3 0,60 ab 15 kg; CPU Keramik
    gemischt 144,00 ab 1 kg / 145,00 / 147,00; CPU Kunststoff gemischt 44,00 / 45,00 / 47,00;
    Slot-CPUs 41,80 / 41,80 / 43,01; Handy-/Smartphone-Platinen 14,12 ab 2 kg / 15,38 / 17,49 /
    18,83; Handys/Smartphones 10,92 ab 5 kg / 11,05 / 11,55 / 11,55; Festplatten 1,30 (20/100/500/
    5.000 kg); Laufwerke 0,35 / 0,35 / 0,40 / 0,51; Netzteile 0,50 / 0,60 / 0,70 / 0,70.
- **Verkauf:** Ein **Endkundentarif für den Verkauf von Metall an Schrott24** ist nicht
  publiziert. Die Plattform tritt selbst als lizenzierter Altmetallhändler auf ("voll
  lizenzierter Altmetallhändler", "Angebot, Logistik bis zur Auszahlung"), nennt aber keine
  Ankaufpreise, zu denen sie selbst Metalle zukauft.
- **Gebühren** (alle Betreiberangaben der FAQ, am 28.10.2026 gelesen): **5 EUR Abzug je
  genutzter Versandmarke** vom Erlös (ungenutzte Versandmarken kosten nichts; jede Marke ist nur
  einmal verwendbar); **Big Bags 5 EUR je Stück** zuzüglich Versand (die befüllten Big Bags werden
  abgeholt); **Selbstanlieferung ("Eigenanlieferung") erst ab 100 EUR Materialwert**; bei
  **Stornierung** fallen Stornogebühren nach den AGB an, weil Material nach Verkaufsabschluss
  **innerhalb von 14 Tagen** an der Annahmestelle eingegangen sein muss. Die Palette-/Big-Bag-
  Abholung wird nach Menge und Standort berechnet; konkrete Abholkosten für Berlin sind nicht
  veröffentlicht. Die frühere Notiz vom 07.10.2026 nannte an anderer Stelle 6 EUR für Paketversand
  bis 31,5 kg - die am 28.10.2026 gelesene FAQ nennt dagegen 5 EUR je Versandmarke; der
  Widerspruch bleibt dokumentiert und wird **nicht** aufgelöst.

## Timeline

### Recherche 28.10.2026: maschinenlesbare Ankauf-Preisliste mit Mengenstaffeln gefunden

- [Recherche 28.10.2026: **PREISLISTE GEHUNDEN.** Die Betreiberseite publiziert je Produkt einen
  **maschinenlesbaren JSON-Endpunkt** der Form
  `https://www.schrott24.de/page-data/altmetall-ankauf/<kategorie>/<produkt>/page-data.json`
  (HTTP 200, `application/json`, ohne Anmeldung). Das `price`-Feld trägt **Mengenstaffeln in
  EUR/kg**, z.B. "Kupfer, blank I, Millberry": 8,20 EUR/kg ab 3 kg, 8,49 ab 100 kg, 8,79 ab 500 kg,
  9,04 ab 10.000 kg; Mischschrott E1: 0,16 ab 100 kg bis 0,24 ab 20.000 kg; Messing MS58 neu
  stückig: 5,06 bis 5,79. Über die 94 Produktseiten der Sitemap
  (`wp-sitemap-posts-page-1.xml`) wurden **67 Sorten mit Preisen** erfasst (Aluminium, Blei,
  CPUs, Edelstahl, Eisen/Stahl, Elektronikschrott, Handy/Smartphone, Hartmetall/HSS, Kabel,
  Kupfer, Leiterplatten, Messing, PC-Komponenten, Rotguss, Zink, Zinn); die vollständige
  Staffel-Tabelle steht im Überblick. **Einordnung (wichtig):** Es sind ausdrücklich
  **Plattform-Maximalpreise für Deutschland und Österreich** ("Angezeigt werden Maximalpreise aus
  Deutschland und Österreich"; "Preise sind abhängig von Ihrem Standort und der gewählten
  Transportart"), **keine** lokalen Berliner Händlerpreise und **keine** Zusage für einen bestimmten
  Annahmehof; die Erklärseite `/schrottpreise/` nennt als sichtbaren Datenstand "Tagesaktuelle
  Schrottpreise vom 14.11.2025", die Produkt-JSONs tragen **kein Datum**. Der Konflikt vom
  07.10.2026, es gebe "keine numerische Ankaufspreisliste", ist damit **überholt** (eine
  numerische Liste existiert), die frühere Nichtübernahme als Händlerpreis bleibt aber berechtigt.
  **Gebühren (FAQ, am 28.10.2026 direkt gelesen):** **5 EUR Abzug je genutzter Versandmarke** vom
  Erlös (ungenutzte Marken kosten nichts; jede Marke nur einmal verwendbar), **Big Bags 5 EUR je
  Stück** zzgl. Versand mit anschließender Abholung, **Selbstanlieferung erst ab 100 EUR
  Materialwert**, **Stornogebühren** bei nicht fristgerechter Anlieferung (Material muss innerhalb
  von **14 Tagen** bei der Annahmestelle eingehen). Die Erläuterungsseite nennt, die endgültigen
  Preise könnten von den zunächst angezeigten abweichen, und verweist für die Abholung auf
  Paletten- oder Big-Bag-Transport. Die frühere Notiz vom 07.10.2026 nannte an anderer Stelle
  6 EUR für Paketversand bis 31,5 kg - die heute gelesene FAQ nennt 5 EUR je Versandmarke; der
  Widerspruch bleibt offen dokumentiert. **Verkauf:** kein Endkundentarif publiziert. Frontmatter:
  `description` um die belegte Preis- und Gebührenlage ergänzt; keine numerischen Preise in die
  Datenbank übernommen, da es sich um Plattform- und nicht um Händlerpreise handelt und die
  Produkt-JSONs kein Datum tragen. Quelle(n): https://www.schrott24.de/page-data/altmetall-ankauf/kupfer/kupfer-blank-i-millberry/page-data.json ;
  https://www.schrott24.de/wp-sitemap-posts-page-1.xml ;
  https://www.schrott24.de/schrottpreise/ ;
  https://www.schrott24.de/faq/support/ ;
  https://www.schrott24.de/standorte/schrottplatz-berlin/ ; Abruf 28.10.2026.]

### Recherche 02.10.2026

- [Recherche 02.10.2026: ok — Betreiber verifiziert (Impressum: Metaloop Europe GmbH, Puchstr. 17, 8020 Graz, FN 638630 m, Geschaeftsfuehrer Pannenbaecker/Schlick; Plattform + lizenzierter Altmetallhandel, 400+ Partner). Berlin-Seite geprueft: reines Online-/Partnernetz-Angebot (Preisrechner, Abholung), kein Berliner Sitz — Seed-Adresslage ("kein Sitz in Berlin") bestaetigt. Website auf Domain-Root normiert, live → aktiv. Keine Preisfills (Preise mengen-/ortsabhaengig, keine vollstaendige Angabe). Status aktiv; Quelle(n): https://www.schrott24.de/standorte/schrottplatz-berlin , https://www.schrott24.de/faq/impressum/]
- [Recherche 07.10.2026: Betreiberseiten erneut einzeln geprüft: Berlin-Standortseite wirbt mit Online-Verkauf/Preisrechner und Partnernetz (über 400 Händler/Werke/Recyclingbetriebe); die konkrete Anlieferadresse und Öffnungszeiten eines Partners werden laut FAQ erst nach Online-Verkaufsabschluss mitgeteilt. Das Impressum nennt Metaloop Europe GmbH, Puchstraße 17, 8020 Graz, Firmenbuch FN 638630 m. ANKAUF (Auszahlung an Verkäufer:innen): Die Berliner Seite verlinkt auf tagesaktuelle Preise, aber der Ablauf- und Preisrechner erfordert Auswahl von Material, Menge, Transport und Standort; die Preisübersicht zeigt als sichtbaren Datenstand 14.11.2025 und erklärt, dass Maximalpreise aus Deutschland/Österreich nach Transportart und Standort variieren. Daher keine aktuelle numerische Ankaufspreisliste übernommen. VERKAUF: kein Endkundentarif für Metallverkauf an Käufer publiziert/gefunden. GEBÜHREN: FAQ nennt beim Paketversand bis 31,5 kg 6 € Abzug vom Erlös, an anderer Stelle 5 € je verwendeter Versandmarke (interner Widerspruch); Big Bag 5 € pro Stück zzgl. Versand; Selbstanlieferung erst ab 100 € Materialwert; Palette/Big-Bag-Abholung wird nach Menge und Standort berechnet. Stornoentgelt wird erwähnt, aber hier ohne verlässliche konkrete Höhe. Beträge sind Betreiberangaben der zum Abruf erreichbaren FAQ, nicht Berliner Filialtarife. Quellen (Abruf 07.10.2026): https://www.schrott24.de/standorte/schrottplatz-berlin/ ; https://www.schrott24.de/faq/support/ ; https://www.schrott24.de/schrottpreise/ ; https://www.schrott24.de/faq/impressum/]

### Importiert (Seed-Stand 2026-09-30)

- Online-Ankauf, Express-Auszahlung, Abholung
- Adresse: kein Sitz in Berlin belegt
