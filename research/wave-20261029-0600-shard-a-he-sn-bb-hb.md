# 6h-Welle 29.10.2026, 06:00 UTC — Shard A (BB 2, HB 1, HE 12, SN 5)

20 Dossiers, jeweils mit **einer neuen** Timeline-Notiz (`- [Recherche 29.10.2026: …; Quelle(n): …]`)
und ergänztem `## Überblick`. Keine bestehende Notiz überschrieben, **keine Slugs geändert**,
kein Commit/Push/Deploy, Produktions-DB ausschließlich read-only
(`file:/var/lib/schrott-mcp/public.db?mode=ro` + `PRAGMA query_only=ON`).

Prüfstand: `python3 scripts/release/wave_gate.py HEAD` → **Owner-Gate bestanden**
(Overwrite-Scan, Enum-Validierung, Deep-Link-/Emdash-Check, Notenformat) ·
`cargo test -p schrott-mcp-ingestion seed` → **14 passed / 0 failed**, Build-Codegen grün
("seed codegen ok") · `rustc --edition=2021 --test crates/ingestion/build.rs` → 10 passed.

Hinweis zum Shard-Namen: Die reale Datei/Slug lautet `bb-neustadt-dahm-bohnensack-handels-und-recycling`
(mit doppeltem "n"); der Auftrag nannte die Variante mit einfachem "n". Der Slug wurde **nicht**
angepasst, sondern die vorhandene Datei bearbeitet.

## Quellen dieser Welle

- **Amtliches Handelsregister** (`online-handelsregister.de`, Handelsregisterauszug und
  Suchansicht): Philipp Blam & Sohn HRB 4357, Rich. Herbig Metallhandel Sachsen HRB 23817,
  Gebr. Büchner Transport HRB 10886, Marburger Rohstoffverwertung Johannes Völker HRB 1784.
- **Zertifikat der Zertifizierungsstelle**: ISO 9001:2015 Zertifikat 10575982 der LRQA Limited
  (Anlage "Rich. Herbig Metallhandel Sachsen GmbH", Gompitzer Höhe 1, 01156 Dresden).
- **Amtliches Fachbetrieberegister** (Spiegel `schrottregister.pages.dev`, Auszug **11.08.2026**):
  Ortsseiten Neustadt, Elsterwerda, Offenbach, Dresden, Wiesbaden, Leipzig; Negativbefunde für
  Elsterwerda (kein Taubenheim), Offenbach (kein Blam), Dresden (kein Rich. Herbig), Leipzig
  (kein Büchner), Wiesbaden (kein Kohl, kein Delis). Für Sebnitz, Kirtorf, Limeshain und
  Höchst im Odenwald existiert **keine Ortsseite**.
- **OSM live** (OSM-API `api.openstreetmap.org`, Raster über den ganzen Straßenverlauf,
  Nominatim-Reverse): Adressobjekte, Hausnummern und gespeicherte Punkte **am Objekt** geprüft.
- **Betreiber-Primärquellen**: `dein-schrottplatz.de` (Preisliste, Impressum, Leistungsseiten),
  `firma-bajramaj.de`, `dergoldfachmann.de` (8 Unterseiten).
- **Kommunale Quelle**: Entsorgungsbetriebe der Landeshauptstadt Wiesbaden (ELW),
  Kleinannahmestelle/Wertstoffhöfe.

## Frontmatter-Änderungen

| Dossier | Was neu belegt wurde | Quellen |
|---|---|---|
| `sn-dresden-01156-rich-herbig-metallhandel-sachsen` | **Identität amtlich, Geschäftsmodell widerspricht der Einordnung**: HRB 23817, Gompitzer Höhe 1, 01156 Dresden; ISO 9001 (LRQA 10575982, gültig bis 31.01.2027) mit Geltungsbereich Profil-/Rohr-/Fittinghandel aus Alu und Kupfer, Anlage der Manfred J. C. Niemann-Gruppe. `street` + `postcode` + `description` gefüllt; **kein** Schrottankauf belegt | Handelsregisterauszug HRB 23817, LRQA-Zertifikat, wlw.de, SchrottRegister ort-dresden, OSM |
| `he-offenbach-am-main-philipp-blam-sohn` | **Registerbeleg**: "Philipp Blam & Sohn Gesellschaft mit beschränkter Haftung", AG Offenbach HRB 4357, Waldstraße 87, **63071 Offenbach**, aktiv, eingetragen 21.02.1979. `postcode` gefüllt; keine Efb-Eintragung | Handelsregisterauszug HRB 4357, SchrottRegister ort-offenbach, OSM |
| `sn-leipzig-04316-gebr-buchner` | **Rechtsträger geklärt**: "Gebr. Büchner Transport GmbH", AG Leipzig HRB 10886, Am Bahndamm 8, **04316 Leipzig**, aktiv, seit 29.06.1900. `postcode` gefüllt; OSM-Gewerbeobjekt "Büchner Gebr. Transport GmbH" als zweite Quelle; keine Efb-Eintragung | Handelsregisterauszug HRB 10886, SchrottRegister ort-leipzig, OSM |
| `sn-sebnitz-klaus-dittrich` | **Registerort geokodiert**: OSM way/219541868, `addr:housenumber=1`, `addr:street=Steingässchen` (**Doppel-s**, Register schreibt "Steingäßchen"), denkmalgeschützt. `lat`/`lon` = 50.9711292/14.2713332 gesetzt | OSM-API, Nominatim |
| `he-gelnhausen-63571-der-goldfachmann` | **Anschrift zweifach belegt** (Betreiber-Impressum + OSM way/139883067, `addr:housenumber=10`): `street` "Im Ziegelhaus 10", `postcode` 63571, `lat`/`lon` 50.1985452/9.1906500. Barbarossa-Block ist am 29.10.2026 **nicht mehr** auf der Seite (8 Unterseiten geprüft) | dergoldfachmann.de (8 Unterseiten), OSM-API, Nominatim |
| `he-kirtorf-36320-hahny-s` | **Betreiberdomain entdeckt und geprüft**: `website` = https://www.hahnys-schrotthandel.de, `website_status` = **blockiert** (DNS ok auf 89.22.101.153, Wurzel HTTP 403, alle Unterpfade 404, kein Wayback-Snapshot) | 11880-Profil, eigener DNS/HTTP-Abruf |
| `sn-rochlitz-db-recycling-denis-blum` | **Vollständige Preistabelle dokumentiert** (siehe unten), `description` um Leistungen, Preise, Gebühren, Annahmestopp, Filialen ergänzt | dein-schrottplatz.de (Preise, Impressum, Leistungsseiten), OSM |
| `he-wiesbaden-65189-kohl-waste-trade` | Keine Frontmatter-Änderung; Schließung durch amtlichen Negativbefund gestützt (kein Kohl-Betrieb im Efb-Register Wiesbaden) | SchrottRegister ort-wiesbaden, ELW |
| übrige 13 Dossiers | Keine Frontmatter-Änderung (Beleglage reicht nicht oder bleibt offen) | siehe Timeline |

## Ankauf / Verkauf / Gebühren (getrennt geführt)

**Ankauf (Einkaufspreise des Betreibers), beziffert: 1 Dossier.**
- **DB Recycling (Denis Blum), Rochlitz**: Preisseite `preise.php`, **gültig ab 03.10.2026**,
  am 29.10.2026 unverändert abrufbar, Einkaufspreise je kg: Hartmetall 45,00; Cu Raff 92%
  9,20; Cu schwer 10,00; Cu Oberleitung 10,00; Cu Lackdraht/Berry 9,70; Cu Millberry 11,40;
  Ms schwer 6,20; Ms Raff (max. 10% Anhaftung) 4,00; Ms Raff Späne 5,00; Cu-Ms Kühler 5,00;
  Al Draht blank 1,90; Al Freileitung 1,70; Al Profil Si 0,5 1,90; Al Felgen 2,00; Al Blech/Guß
  1,00; Al Getriebe 0,40; Al-Cu Kühler 4,20; Cu-Shredderkabel 3,80; Cu-Schlitzkabel 60% 5,80;
  Al-Shredderkabel 0,10; Al-Schlitzkabel 1,00; Zinkblech 2,00; Reinzinn 30,00; Altblei 1,20;
  Wucht-/Kabelblei 0,40; Batterieblei 0,30; Chromstahl 0,16; V2A 0,80; V4A 1,60; E-motoren
  0,80; Getriebemotoren 0,30; Schwerer Scherenschrott 0,15; Leichter Scherenschrott 0,13;
  Kernschrott 0,18; Gußschrott 0,18; Mischschrott 0,10; Schreddervormaterial 0,05;
  PC-komplett 0,60; Laufwerk 0,20; Festplatte 1,40; Netzteil 0,20; Leiterplatte Kl. 1 10,00;
  Leiterplatte Kl. 3 0,80; Altpapier 0,08.
  **Annahmegrenze:** für **Textilien** und **Schuhe** weist die Preisliste "Annahmestopp" aus.
- **Goldfachmann**: weiterhin **keine** Ankaufspreise je Gramm/Feingehalt; das Impressum stellt
  klar, dass die Goldpreis-Grafiken gerundete EZB-Historienwerte je Feinunze sind und "kein
  aktueller Ankaufspreis" darstellen.

**Verkauf, beziffert: 1 Dossier, und zwar eine Serviceleistung.**
- **Der Goldfachmann**: "Batteriewechsel ... für nur 4,- €" (Uhrenservice). Kein Produktkatalog,
  keine Schrott-/Edelmetall-Verkaufspreise.
- **DB Recycling**: ausdrücklich "kein Verkauf von gebrauchten Ersatzteilen"; daneben "Ankauf
  von Gebrauchtwagen" (Ankauf durch den Händler, kein Verkauf).

**Gebühren, beziffert: 1 Dossier (Betreiberkosten).**
- **DB Recycling**, Abschnitt "Entsorgungskosten" der Preisliste: Sperrmüll "Netto 0,3= 300EUR/t";
  Bauschutt sauber "Netto 0,04=40 EUR/t"; Grünschnitt/Altholz "Netto 0,15= 150EUR/t";
  "Transport Absetzcontainer 100,00 EUR". Das sind Betreiberkosten für die Entsorgung, **keine**
  Ankaufpreise. Container: Absetz 3,5 bis 10 cbm, Abroll 7,0 bis 38 cbm, "Containerstellung ab
  1 Tonne kostenlos", "nach 14 Tagen keine Standgebühren" bei Absprache.
- **Kommunale Tarife wurden nicht als Betreibergebühren übernommen**: ELW-Wertstoffhöfe
  Wiesbaden (kostenfreie Annahme, max. 700 l je Anlieferung, bestimmte Abfälle kostenpflichtig),
  Wertstoffhof Rochlitz (`recycling:scrap_metal=yes`, kommunal).
- In den übrigen 18 Dossiers ist **keine** Gebührenordnung belegt; Bajramaj nennt nur
  Containerstellungen 7 m³ bis 40 m³ ohne Zahlenpreis.

## Offene Fragen (Owner)

- **Rich. Herbig Dresden**: Die Gesellschaft ist amtlich belegt, das Geschäftsmodell ist
  **NE-Metallhalbzeug-Großhandel, nicht Schrott**. Ob die Zeile im Schrottkatalog überhaupt
  bestehen soll, ist Owner-Entscheidung; Status bleibt `pruefung`. OSM kennt an Gompitzer Höhe 1
  zwei Fachbetriebe (JYSK, Vogel Modellsport), deshalb wurde kein Punkt gesetzt.
- **Hahny's Kirtorf**: Straße/PLZ/Telefon stammen nur aus vier Aggregator-Profilen (Das Örtliche,
  11880, golocal, cylex); die Betreiberseite ist von hier aus nicht abrufbar. Adresse bleibt leer.
- **Rosenbach Bremerhaven**: Der Adresskonflikt (Lange Str. 30 in Lehe vs. Spadener Str. 53)
  besteht fort; OSM kennt die Nr. 30, die Nr. 53 fehlt. Verzeichnisse nennen 27578, OSM 27580.
- **Gebr. Büchner Leipzig**: Der registrierte Name lautet "Transport GmbH", der Seed-Name
  "Gebr. Büchner GmbH Entsorgungsfachbetrieb"; ohne vollständigen Registerauszug nicht
  entscheidbar, ob historische Form oder Fremdfirma.
- **Reinhardt Hanau-Lamboy**: Das cylex-Profil nennt "Friedrich-Engels-Straße 5, 63452 Hanau"
  (Lamboy); im selben Ortsteil steht mit "Kurt Steding Entsorgungs GmbH" (Ruhrstraße 29) ein
  namentlich geführter Schrotthandel. Verbindung oder Abgrenzung nicht belegt, kein Merge.
- **Völker Marburg**: Der einzige Register-Namensvetter "Marburger Rohstoffverwertung Johannes
  Völker GmbH" sitzt in Lahntal (35094), nicht in Marburg; Bezug zu "Karl-Heinz Völker" offen.
- **Blam Offenbach**: Hausnummer 87 fehlt in OSM (78 Adressobjekte, 77 Nummern); der gespeicherte
  Punkt liegt auf der Bieberer Straße. Straße/PLZ bleiben ohne Geokodierung.
- **Der Goldfachmann**: Die Seite veröffentlicht nur `info@dergoldfachmann.com`; in der
  Produktion steht noch `info@dergoldfachmann.de`. Ein `clear_fields: 'email'` wäre eine
  Owner-Entscheidung (Direktive bleibt bewusst ungesetzt).
- **Taubenheim Elsterwerda**, **Waldmann Eppertshausen**, **SAC Recycling Höchst**,
  **Schrottschneider Limeshain**, **Schrott Siegfried Frankfurt**, **Loew Alfred Frankfurt**,
  **Delis Wiesbaden-Kohlheck**: weiterhin nur Verzeichnis-Leads bzw. Negativbefunde, Status
  `pruefung` bleibt; eine Schließung ist nirgends belegt.

## Geokodierungsbedarf (nur Adressen, keine Koordinaten)

- Marburger Str. 42, 36320 Kirtorf (Hahny's; Hausnummer in OSM nicht erfasst)
- Spadener Str. 53, 27578 Bremerhaven (Rosenbach; Hausnummer in OSM nicht erfasst)
- Lange Str. 30, 27578/27580 Bremerhaven-Lehe (Rosenbach; PLZ-Konflikt Verzeichnis/OSM)
- Gewerbegebiet Ost 8, 16845 Neustadt (Dosse) (Dahm & Bohnsack; keine OSM-Hausnummer)
- Jahnstr. 23, 64859 Eppertshausen (Waldmann; Hausnummer existiert, kein Gewerbe-Tag)
- An der Hasselbach 1, 64739 Höchst (SAC Recycling; Hausnummer existiert, kein Gewerbe-Tag)
- Ginseldorfer Weg 44, 35039 Marburg (Völker; Hausnummer existiert, kein Gewerbe-Tag)
- Haincher Str. 10, 63694 Limeshain (Schrottschneider; Hausnummer existiert, kein Gewerbe-Tag)
- Im Berntal 9, 35083 Wetter (Bajramaj; Hausnummer existiert, kein Gewerbe-Tag)
- Friedrich-Engels-Straße 5, 63452 Hanau-Lamboy (Reinhardt; nur Verzeichnislead)
- Colditzer Str. 14, 09306 Rochlitz und Goethestr. 83, 09217 Burgstädt (DB Recycling,
  zwei im Dossier bisher nicht erfasste Standorte des Betreibers)

## Nicht übernommen / bewusst offen gelassen

- **Keine Aggregator-Leads in strukturierte Felder übernommen** (Straße, PLZ, Telefon, E-Mail
  bei Hahny's, Reinhardt, Taubenheim, SAC Recycling, Waldmann, Delis, Loew Alfred, Schrott
  Siegfried, Rosenbach). Der Widerspruch zur Praxis der Vorwellen ist dokumentiert: Der am
  18.10.2026 zurückgenommene Elsterwerda-Fall belegt, dass ein Verzeichniskonsens allein nach der
  heutigen Quellenhierarchie nicht trägt.
- **Keine Koordinaten geraten.** Wo kein belastbarer Adress- oder Betriebspunkt vorlag, wurde
  nichts gesetzt; die jeweils falschen Altpunkte sind im Dossier benannt (Bieberer Straße 1-7
  Offenbach, Senckenberganlage Westend Süd, Elberadweg Dresden-Neustadt, hair & flair Trufanowstraße
  10 Leipzig, B 426 Höchst, Lampertweg 7 Kirtorf, Am Abteiwald 14 Eppertshausen, Händelstraße 46a
  Limeshain, Friedrich-Ebert-Anlage 31a Hanau, Ludwig-Jahn-Str. 60 Elsterwerda, Niedersachsen bei
  Rosenbach).
- **Zählweise getrennt geführt:** Objektzahl und Zahl verschiedener Hausnummern wurden je Straße
  getrennt gezählt (Beispiel Waldstraße Offenbach: 78 Adressobjekte, 77 verschiedene Hausnummern,
  Nummer 87 fehlt; Spadener Straße Bremerhaven: 26 Objekte, 26 Nummern, Nummer 53 fehlt;
  Ludwig-Jahn-Straße Elsterwerda: 39 Objekte, 39 Nummern, Nummer 60 existiert).
- **Keine `clear_fields`-Direktive gesetzt** (Owner-Entscheidung), auch nicht für die
  E-Mail-Variante beim Goldfachmann.
- **Keine Betriebsschließung behauptet** und keine Schrotttätigkeit unterstellt, wo nur ein
  Demontagebetrieb (Bajramaj) oder ein Halbzeughandel (Rich. Herbig) belegt ist.
