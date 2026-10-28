# Recherchebericht: Hofmann-Metall-Kleinannahmestellen (Altmittweida, Chemnitz-Wartburgstraße, Klipphausen)

**Auftrag:** Owner-Check-in 28.10.2026 — Klären, ob je Kleinannahmestelle ein eigenständiger, im
Schrottkatalog führbarer Betrieb vorliegt; zusätzlich Kleinannahme-Strecke in Brandis prüfen.
**Recherchezeitraum:** 28.10.2026 (alle Abrufe am 28.10.2026, soweit nicht anders vermerkt).
**Ergebnis:** Nur **Klipphausen** ist eine echte Dossier-Lücke. Altmittweida und
Chemnitz-Wartburgstraße sind **bereits im Katalog** — es fehlen dort nicht der Betrieb, sondern die
**Betreiberzuordnung**, die Öffnungszeiten und (in beiden Fällen) eine **GEO-Korrektur**.

---

## 0. Regeln und Methodik (Nachweis)

- **Keine Produktionsänderung.** Die Produktionsdatenbank wurde ausschließlich lesend abgefragt:
  `sqlite3 "file:/var/lib/schrott-mcp/public.db?mode=ro"` mit `PRAGMA query_only=ON;` vor jeder
  Abfrage (Bestand 3.938 Händler, Stand 28.10.2026).
- **Kein Commit, kein Push, kein Deploy.** Es wurde ausschließlich diese eine Berichtsdatei
  geschrieben.
- **Keine Dossier-Datei im Repo angelegt oder geändert.** `dossiers/**` ist unangetastet; die
  Dossier-Vorschläge in Abschnitt 5 sind Entwürfe im Bericht.
- **Keine erfundenen Werte.** Wo eine Quelle fehlt, steht das im Text.
- **Koordinaten** nur mit zwei unabhängigen Quellen (Betreiber-Primärquelle für die Adresse +
  OSM/Nominatim-Gebäudegeometrie für den Punkt); ansonsten keine Koordinate.
- **Ankauf / Verkauf / Gebühren** sind in jedem Abschnitt getrennt geführt.
- **Negativbefunde im Register** (schrottregister.pages.dev) belegen keine Nichtexistenz; sie
  belegen nur: die zuständige Stelle führt dort unter dieser Adresse bzw. unter dieser
  Firmierung keinen Eintrag.
- Quellen-Hierarchie nach `README.md` (Z. 184–212): Aggregatoren (Cylex/GoLocal/11880/
  schrottplatz.org/schrottplatz-info.de) sind **Leads, kein Beleg**; Betreiber-Primärquelle,
  Handelsregister, amtliche Register und amtliche Veröffentlichungen sind Belege.

---

## 1. Kurzantwort

| Kleinannahmestelle | Betreiber laut Betreiberseite | Kontakt | Angenommene Materialien | Registerbefund | Im Katalog? | Dossier sinnvoll? |
| --- | --- | --- | --- | --- | --- | --- |
| **Altmittweida**, Hauptstraße 79 | Hofmann Metall GmbH (eigene Seite, keine „Betreiber:"-Zeile) | nur Mobil 0176 / 411 925 44 | FB-Ankündigung: Stahlschrott + Buntmetall | Hofmann Metall GmbH nur unter Zwickau (ZZIT00700096); kein Eintrag in Altmittweida | **ja** — `sn-altmittweida-schrott-buntmetallankauf-buntmetallhande` | **nein** (Bestandsdossier anreichern) |
| **Chemnitz**, Wartburgstraße 2e | Hofmann Metall GmbH (eigene Seite); „Büro"-Nummer gehört laut GoLocal zu „Ankauf von Buntmetall M. Reichel" | Büro 0371 / 54 0 46, Mobil 0176 / 55 56 43 52 | auf der Seite nicht genannt | kein Eintrag an der Adresse | **ja** — `sn-chemnitz-ankauf-von-buntmetallen-stahlschrott` (Status `pruefung`) | **nein** (Klärfall auflösen) |
| **Klipphausen**, Triebischtalstraße 19 | **Sand- und Kiesgruben Sönitz GmbH** (auf der Seite ausdrücklich benannt) | keine Telefonnummer auf der Seite; Betreiber: Tel 035244 41701, Fax 035244 492302 | auf der Seite nicht genannt | **kein** Eintrag; Klipphausen kommt im Register nicht vor | **nein** | **ja** — Dossier-Entwurf in 5.3 |
| **Brandis** | — | — | — | — | ja | **nein** — Bestätigung, siehe 4. |

---

## 2. Strukturbefund: „KAS" ist eine eigene Standortkategorie, kein Betriebsteil

Der entscheidende Primärbeleg ist die **Firmenhistorie** der Betreiberseite
(`https://hofmann-metall.de/firmenhistorie/`), die „KAS" (= Kleinannahmestelle) als eigene
Kategorie neben „BT" (= Betriebsteil) führt:

- **2021:** „Standorterweiterung: Betriebsteil Brandis / Leipzig – KAS Schmölln – KAS
  Chemnitz-Lutherviertel – KAS Chemnitz-Einsiedel"
- **2023:** „Standorterweiterung: Inbetriebnahme der KAS Klipphausen"
- **2024:** „Standorterweiterung: Eröffnung des BT Schmölln und der KAS Altmittweida, sowie
  Intensivierung der Social Media Aktivitäten"
- **2026:** „Standorterweiterung: Übernahme des ehemaligen SWO-Geländes in Elstertrebnitz …"

Daraus die Ausdifferenzierung der drei KAS:

- **KAS Schmölln (2021) → BT Schmölln (2024):** Die Schmöllner Annahmestelle war zunächst eine
  KAS und wurde 2024 zum Betriebsteil aufgerüstet. Die Hofmann-News vom 07.06.2021 („Ab Montag,
  den 07.06.2021 eröffnen wir eine neue Kleinannahmestelle in Schmölln. Diese Annahmestelle ist
  hauptsächlich für Privat- und Kleingewerbekunden vorgesehen") ist die Belegkette für die
  KAS-Stufe. Das bestehende Dossier `th-schmoelln-hofmann-metall-betriebsteil` ist damit korrekt
  als **Betriebsteil** geführt — die Historie gehört als Notiz ins Dossier.
- **KAS Chemnitz-Lutherviertel (2021)** = die KAS an der Wartburgstraße 2e. Belege: die
  Hofmann-Instagram-Hashtags vom September 2025 führen `#kleinannahmestellen
  #chemnitz_lutherviertel #altmittweida` — der Chemnitz-KAS wird vom Betreiber selbst als
  *Lutherviertel* bezeichnet, Klipphausen taucht dort **nicht** auf (passt zur
  Drittbetreiber-Angabe auf der Klipphausen-Seite).
- **KAS Klipphausen (2023):** die einzige KAS mit **fremdem Betreiber**.
- **KAS Altmittweida (2024):** eröffnet laut Betreiber-Historie 2024 und laut Facebook-Post
  „Neu in Altmittweida! Seit Anfang Juli ist unsere Kleinannahmestelle für Stahlschrott und
  Buntmetall geöffnet" (Suchmaschinen-Zeitstempel des Facebook-Posts: **25.07.2024**, in
  thailändisch-buddhistischer Schreibweise 2567 = 2024 n. Chr.; das Jahr 2024 ist durch die
  Betreiber-Historie unabhängig bestätigt).

**Konsistenzprüfung über eine dritte, betreiberfremde Quelle:** Die Pressemitteilung vom
**10.04.2025** (`presseportal.de/pm/172422/6010468`) nennt „Mit **fünf Standorten, drei
Kleinannahmestellen** und einem starken Partnernetzwerk …". Fünf Standorte 2025 = Zwickau,
Chemnitz, Hirschfeld, Brandis, Schmölln (Elstertrebnitz kam erst 13.08.2026 hinzu); drei
Kleinannahmestellen 2025 = Chemnitz-Lutherviertel, Klipphausen, Altmittweida (Schmölln war
bereits BT). Die Pressemitteilung nennt **nicht**, welche drei — das ist mit den drei auf der
Website geführten deckungsgleich. Die News-Meldung zur Elstertrebnitz-Eröffnung (13.08.2026)
sagt: „Mit unserem neuen Betriebsteil in Elstertrebnitz sind wir nun an insgesamt **sechs
Standorten** in Sachsen und Thüringen für unsere Kunden da" — die KAS zählen dabei nicht mit.
**Ergebnis:** Die Website führt 6 Betriebsstellen + 3 KAS; beide Kategorien sind redaktionell
scharf getrennt (nur die 6 erscheinen im Footer-Kontaktblock „Unsere Standorte" und in der
Kontaktseite).

---

## 3. Je Kleinannahmestelle

### 3.1 Altmittweida — Hauptstraße 79, 09648 Altmittweida

**Betreiber.** Die KAS-Seite (`/standorte/kleinannahme-altmittweida/`) nennt **keinen**
„Betreiber:"-Zusatz — im Unterschied zur Klipphausen-Seite. Sie ist eine Seite der
Hofmann-Metall-Website; Betreiber der Gesellschaft ist die Hofmann Metall GmbH (HRB 5475
AG Chemnitz, Sitz Zwickau). Die KAS ist damit als **Hofmann-eigene Annahmestelle** zu lesen.

**Kontakt.** Ausschließlich `Mobil: 0176 / 411 925 44`. Keine Büro-Nummer, keine E-Mail, kein
Fax auf der Seite.

**Öffnungszeiten.** Mo–Do 09:00–17:00, Fr 09:00–16:00, Sa nach Vereinbarung.

**Angenommene Materialien.** Auf der Standortseite **nicht** genannt (kein Ausstattungs- und kein
Materialblock). Belegt ist über den Facebook-Post des Betreibers: „Kleinannahmestelle für
**Stahlschrott und Buntmetall**". Die Sorten-Übersicht der Hofmann-Website
(`/privat-kleingewerbekunden/`) ist die **gemeinsame** Seite für Privat- und Kleingewerbekunden
aller Hofmann-Standorte und wird nicht als standortspezifische Annahmeliste geführt.

**Beleglage.**
- Betreiber-Primärquelle: Standortseite (Adresse, Mobil, Zeiten) + Impressum (Gesellschaft,
  HRB 5475) + Firmenhistorie (2024: Eröffnung KAS Altmittweida) + Facebook-Post (Materialien).
- Handelsregister-Auszug HRB 5475: Rechtsträger bestätigt, Sitz Zwickau, Stammkapital
  130.000,00 EUR (Beschluss 08.03.2017).
- **Zweite unabhängige Quelle für den Standortort**: die beiden Webador-Template-Seiten des
  lokalen Betriebs „Schrott Buntmetallankauf" (`buntmetallhandel-altmittweida.de` **und**
  `schrottbuntmetallankauf.de`) nennen auf ihrer Kontaktseite **dieselbe** Nummer
  **Tel. 017641192544**, dieselben Öffnungszeiten (Mo–Do 9–17, Fr 9–16) und dieselbe Adresse
  („09648 Altmittweida Hauptstr 79"). Damit ist die KAS Altmittweida **kein zusätzlicher
  Betriebspunkt**, sondern der bereits katalogisierte Standort Hauptstraße 79 unter der
  Hofmann-Betriebsbezeichnung.

**Katalogabgleich (read-only).** Zeile `sn-altmittweida-schrott-buntmetallankauf-buntmetallhande`:
Adresse Hauptstr. 79 / 09648 Altmittweida, `phone = 017641192544` (**identisch** mit der auf der
Hofmann-Seite veröffentlichten Mobilnummer), `opening_hours = 'Mo-Do 9:00-17:00, Fr 9:00-16:00'`
(**identisch**), `website = https://buntmetallhandel-altmittweida.de` (aktiv), `status = aktiv`,
laufender Preis-Handler (`crates/ingestion/.../buntmetallhandel_altmittweida.rs`).
**GEO:** Der in der Produktion gespeicherte Punkt `50.9677492 / 12.9570254` löst rückwärts zu
**„78c, Hauptstraße"** auf — die Nominatim-Vorwärtssuche „Hauptstraße 79, 09648 Altmittweida"
liefert das Gebäude **„79, Hauptstraße"** (OSM-Way 201731552) auf
`50.9634358 / 12.9433358`. **Abweichung 1.073 m.** Der Punkt ist falsch (falsche Hausnummer,
anderer Straßenseite-Abschnitt) — er muss korrigiert oder zurückgenommen werden.

**Weitere Nennungen an derselben Hausnummer** (alle Aggregator-Leads, kein Beleg): „Schrott Scharf
Matthias", Hauptstr. 79, Tel 03727 979789 (schrottplatz-info.de); „Buntmetallhandel Altmittweida",
Hauptstraße 79, Mobil 0176 55217721, `matthias-scharf6119@gmx.de`, Website
`schrottbuntmetallankauf.de` (Cylex); „GM Gerüstbau GmbH Mittweida", Hauptstraße 79, Tel
03727/998804 (Firmen-A-Z der Gemeinde Altmittweida). Die beiden Betriebs-Webseiten haben **kein
Impressum** (404) und ein „John Doe"-Testimonial — schwache Betreiberbelege, wie bereits im
Dossier vermerkt.

**Ankauf / Verkauf / Gebühren.**
- **Ankauf:** Der Standort veröffentlicht eine **Ankaufspreisliste** (Webador-Seite), die im
  Katalog als Preisquelle läuft (25 Preise beim letzten erfolgreichen Handler-Lauf). Das ist ein
  **Händlerpreis** der bestehenden Katalogzeile. Für die Hofmann-KAS-Zeile gilt: keine eigene
  Preisveröffentlichung auf hofmann-metall.de.
- **Verkauf:** keine Produkt-/Materialpreisliste.
- **Gebühren:** keine bezifferten Tarife; Containerdienst wird auf der Hofmann-Website
  generell angeboten (Absetzcontainer 6–10 m³, Abrollcontainer 18–36 m³), ohne Preise für
  Kleinannahmen.

**Registerbefund.** `schrottregister.pages.dev` (Erhebung 11.08.2026): Für PLZ 09648 existiert die
Ortsseite **Mittweida** — sie führt **einen** Betrieb: *Wolfrum Kältesystheme GmbH, Zur Torfgrube
2* (Lagern, Behandeln). **Kein** Schrotthändler und **kein** Eintrag an der Hauptstraße 79.
Die **Hofmann Metall GmbH selbst ist im Register geführt** — aber nur mit der Registeranschrift
des Hauptsitzes: `betrieb-hofmann-metall-gmbh-zwickau`, **Äußere Dresdner Straße 80, 08066
Zwickau**, Zertifikatsnummer **ZZIT00700096**, zertifizierte Tätigkeiten **Lagern, Behandeln,
Verwerten, Beseitigen**, 29 Abfallschlüssel (u. a. 170405 Eisen und Stahl, 170407 gemischte
Metalle, 170411 Kabel, 200136/200140 Metalle). Das Register führt **keine** Filialzeilen: die
sechs Betriebsstellen erscheinen einzeln **nicht**. **Negativbefund, kein Nichtexistenzbeleg:**
Das Register führt die rechtliche Einheit mit Registeranschrift; eine Annahmestelle ohne eigene
Zertifikatsführung muss dort nicht auftauchen.

**Bewertung.** Die KAS Altmittweida ist **kein zusätzlicher Betrieb**. Sie ist derselbe
Standort wie der bereits geführte Händler „Schrott Buntmetallankauf" an Hauptstraße 79. Ein
**neues** Dossier würde denselben Betrieb doppelt zählen, die laufende Preishistorie des
Handlers aufspalten und zwei konkurrierende Punkte am selben Ort erzeugen. **Empfehlung: das
Bestandsdossier anreichern, nicht ein neues anlegen** (Änderungsvorschlag in 5.1).

---

### 3.2 Chemnitz-Wartburgstraße — Wartburgstraße 2e (Betreiber: 09123 / OSM+GoLocal: 09126)

**Betreiber.** Auch diese KAS-Seite nennt **keinen** „Betreiber:"-Zusatz → Hofmann Metall GmbH
(eigene KAS im Netz, seit 2021 als „KAS Chemnitz-Lutherviertel" geführt). Komplexität: die
veröffentlichte **Büro**-Nummer `0371 / 54 0 46` ist genau die Nummer, die das Branchenverzeichnis
GoLocal an derselben Adresse einem **anderen** Namen zuteilt (siehe unten). Hofmann nennt also
eine fremde Landline als „Büro" und eine eigene Mobilnummer; die rechtliche Zuordnung der
Annahmestelle ist damit **nicht** abschließend geklärt.

**Kontakt.** `Büro: 0371 / 54 0 46` (5-stellige Kurzwahl-Nummer; GoLocal führt dieselbe Nummer
sowohl als Telefon **als auch** als Fax), `Mobil: 0176 / 55 56 43 52`. Keine E-Mail.

**Öffnungszeiten.** Mo–Mi 09:00–18:00, Do geschlossen, Fr 09:00–16:00, Sa 09:00–12:00.

**Angenommene Materialien.** Auf der Standortseite **nicht** genannt. Der Katalog-Seedname der
bestehenden Zeile („Ankauf von Buntmetallen & Stahlschrott") stammt aus einem Aggregator-Lead
(lokaleschrottplatz) und ist **kein Betreiberbeleg**.

**Beleglage.**
- Betreiber-Primärquelle: Standortseite (Adresse, Büro/Mobil, Zeiten) + Impressum + Firmenhistorie
  (2021: KAS Chemnitz-Lutherviertel) + Instagram-Hashtag `#chemnitz_lutherviertel` (2025).
- **GoLocal** (Aggregator, **Lead**): Eintrag „**Ankauf von Buntmetall M. Reichel**",
  **Wartburgstr. 2 E, 09126 Chemnitz** (Stadtteil Bernsdorf), Telefon **(0371) 540 46**,
  Fax (0371) 540 46, Öffnungszeiten Mo 08:00–12:00 & 13:00–18:00, Di 08:00–12:00, Mi 08:00–12:00 &
  13:00–18:00, Do geschlossen, Fr 08:00–12:00, Sa/So geschlossen; Hinweis „Nutzermeldung – nicht
  von golocal geprüft". Die Nummer- und Adressidentität mit dem Hofmann-Eintrag ist belegt, der
  **Name** „M. Reichel" ist ein nicht geprüfter Verzeichnis-Eintrag und darf den Betreiber nicht
  ersetzen.
- **PLZ-Konflikt:** Hofmann schreibt **09123**, GoLocal und OSM (Wartburgstraße 2e →
  „Bernsdorf, 09126") nennen **09126**. Zwei unabhängige Quellen (Verzeichnis + Kartengeometrie)
  sagen 09126; die PLZ-Grenze Kaßberg/Bernsdorf verläuft genau dort.

**Katalogabgleich (read-only).** Zeile `sn-chemnitz-ankauf-von-buntmetallen-stahlschrott`:
Wartburgstr. 2e, `phone = 0176 55564352` — **identisch** mit der auf der Hofmann-KAS-Seite
veröffentlichten Mobilnummer; `status = pruefung` (seit 02.10.2026, weil damals kein
Betreiberauftritt gefunden wurde — **die Lücke ist mit der Hofmann-Seite jetzt geschlossen**).
**GEO:** Der gespeicherte Punkt `50.8207333 / 12.9104667` löst rückwärts zu **„11, Parkstraße,
Kapellenberg, 09120"** auf und liegt in einem Wohngebiet. Die Nominatim-Vorwärtssuche
„Wartburgstraße 2e, 09123 Chemnitz" liefert das adresskongruente Gebäude (OSM-Way 566201450,
Kategorie `industrial`) auf `50.8197217 / 12.9408697`. **Abweichung 2.141 m.** Bemerkenswert:
dieses Punktpaar ist **identisch** mit dem am 18.10.2026 bereits als falsch markierten
**Altpunkt von TSR Chemnitz** (Owner-GEO-Note 26.10.2026: der Punkt liegt „rund 4,93 km
südwestlich des adresskongruenten OSM-Gebäudes … im kartierten Wohngebiet bei
Parkstraße/Herderstraße"). Die Zeile hat den Altpunkt also **geerbt**.

**Ankauf / Verkauf / Gebühren.**
- **Ankauf:** Keine öffentliche Preisliste dieser KAS. Das Dossier-Seed nennt „Bunt/Stahl-Annahme"
  (Aggregator-Import), ein Betreiber-Preis ist nicht belegt.
- **Verkauf:** keine Liste.
- **Gebühren:** keine bezifferten Tarife.

**Registerbefund.** Die Register-Ortsseite **Chemnitz** umfasst PLZ 09114, 09116, 09117, 09120 und
listet 20 Betriebe — darunter **Hofmann Metall GmbH nicht** (die Hofmann Metall GmbH erscheint nur
unter Zwickau). Kein Eintrag an der Wartburgstraße 2e; **kein** Eintrag für „M. Reichel".
Negativbefund ohne Beweiswert für Nichtexistenz.

**Bewertung.** Hier liegt **keine Dossier-Lücke** vor, sondern ein **Klärfall, der auflösbar ist**:
Die Zeile existiert, der Standort ist durch eine Betreiber-Primärquelle belegt, und die Hofmann-
Seite liefert Betreiber, Zeiten und beide Telefonnummern. Die entscheidende **Identitätsfrage**
(Wer betreibt die Annahmestelle wirklich — Hofmann Metall GmbH, M. Reichel oder ein
Betreiberwechsel/Übernahme?) bleibt zu klären, bevor Name und Typ der Zeile festgeschrieben
werden. Ein **neues** Dossier wäre eine Doppelzeile. **Empfehlung: bestehende Zeile
qualifizieren** (5.2).

---

### 3.3 Klipphausen — Triebischtalstraße 19, 01665 Klipphausen

**Betreiber.** Die Seite nennt ausdrücklich: **„Betreiber: Sand- und Kiesgruben Sönitz GmbH"** —
die einzige der drei KAS-Seiten mit einer Betreiberzeile. Die Hofmann-Website führt die Seite
unter „Kleinannahmestellen" (URL-Segment `kas-klipphausen`); die Firmenhistorie nennt 2023 „Inbetriebnahme der KAS Klipphausen". Hofmann veröffentlicht also eine KAS **an einem Standort, den ein Drittbetreiber betreibt** (Stichwort „Partnernetzwerk" aus der Pressemitteilung 2025).

**Betreiber-Belege.**
- **Handelsregister (online-handelsregister.de, HRB 29510 AG Chemnitz):** Sand- und Kiesgruben
  Sönitz GmbH, GmbH, Geschäftsanschrift **Paracelsusstraße 10, 09114 Chemnitz** (bis 25.06.2015:
  Mark-Twain-Weg 5, 09127 Chemnitz), Stammkapital 25.000,00 EUR, Gesellschaftsvertrag vom
  11.08.2020 mit neuem Gegenstand: „Betrieb und Ausbeutung der Sand- und Kiesgruben in Sönitz und
  alle damit im Zusammenhang stehenden Geschäfte; Handel mit Immobilien, Vermittlung von, Beratung
  bei und Durchführung von Geschäften der Abfalloptimierung wie z.B. Abfallbeseitigung;
  Umweltdienstleistungen für Industriebetriebe …; Rekultivierung und Rückbau baulicher Anlagen;
  **Aufsuchen von Verwertungsmöglichkeiten für Wertstoffe und Abfälle aller Art**; … Vermittlung
  von Aufträgen jeder Art, wie z.B. Umweltdienste." → Der Unternehmensgegenstand passt zu
  Abfall- und Verwertungsdienstleistungen, **nicht** aber zu einem gewerblichen Ankauf von
  Schrott gegen Vergütung.
- **Firmenverzeichnis der Gemeinde Klipphausen** (`klipphausen.de/firmenverzeichnis/`): Kontakt
  **Michael Gräßler**, **An der Kiesgrube 1 a, 01665 Klipphausen (Weitzschen)**,
  **Tel 035244 41701**, **Fax 035244 492302**; Geschäftszweck dort ohne „Schrott"/„Metall", mit
  Abfalloptimierung, Hausmeisterdiensten, Umweltdiensten.
- **Creditreform-Firmeneintrag:** Crefonummer 3030321552, +49 352 4441701 (= 0352 4441701,
  identisch zur Verzeichnisnummer), Sitz Paracelsusstr. 10, 09114 Chemnitz.

**Adress-Konflikt (wichtig).** Hofmann veröffentlicht **Triebischtalstraße 19**; das
Firmenverzeichnis der Gemeinde und OSM führen den SKS-Standort unter **An der Kiesgrube 1a**
(OSM-Way 209515126, Name **„Sandgrube Sönitz"**, addr:housenumber 1a, addr:postcode 01665).
Abstand der beiden Punkte: **489 m** (Nominatim-Gebäude „19, Triebischtalstraße" =
51.1045768/13.4170816; OSM-Mittelpunkt „Sandgrube Sönitz" = 51.107461/13.422355). Beide liegen im
selben Gewerbegebiet Miltitz/Weitzschen. **Welche der beiden Adressen die KAS ist, ist nicht
geklärt.** OSM kartiert dort zusätzlich `landuse=quarry` **„Kiesgrube Sönitz"** und
**„Kiesgrube Sönitz - Schüttgutlagerplatz"** sowie die Straße **„An der Kiesgrube"**.

**Kontakt.** Auf der Hofmann-Seite **keine** Telefonnummer und keine E-Mail. Betreiber-Kontakt
nur über das Gemeindeverzeichnis (035244 41701 / Fax 035244 492302) bzw. Creditreform
(+49 352 4441701) bzw. HR-Sitz (Paracelsusstr. 10, 09114 Chemnitz). **Keine Betreiber-Website
gefunden** (die Firmeneinträge verweisen nicht auf eine eigene Domain; `skr-soenitz.de` gehört der
**anderen** Firma SKR Sönitz GmbH, Lichtenberg/Erz — ausdrücklich **nicht** dieselbe Gesellschaft).

**Öffnungszeiten (Hofmann-Seite).** Mo geschlossen, Di–Fr 08:30–17:30, Sa 08:00–12:00,
So geschlossen.

**Angenommene Materialien.** Auf der Hofmann-Seite **nicht** genannt; keine eigene
Sortenübersicht; keine Material- oder Ausstattungsangabe; keine Preise.

**Beleglage zum Standort.** Betreiber-Primärquelle (Hofmann-Seite: Adresse, Betreiber, Zeiten) +
Handelsregister + Gemeindeverzeichnis für die Betreiberidentität. Für **Was** dort angenommen
wird, für **Vergütung** und für die **Adresse vor Ort** gibt es keinen Beleg.

**Hintergrund zum Standort (amtlich).** Sächsisches Amtsblatt Nr. 12/2025 (Bekanntmachung des
Sächsischen Oberbergamtes, 06.03.2025): „Die Sand- und Kiesgruben Sönitz GmbH hat … eine
Überprüfung der UVP-Pflicht … für die Fortführung und nördliche Erweiterung der Kiessandgrube
Sönitz beantragt. … Die Kiessandgrube Sönitz ist auf den Bestandsflächen seit 2021 vollständig
ausgekiest. **Die Gewinnungstätigkeit in der Kiessandgrube ist eingestellt.**" Der Betrieb ist
damit in der **Rekultivierungs-/Nachnutzungsphase** (Abschlussbetriebsplan, Umweltprüfung) — das
ist konsistent mit dem Kommentar „starkes Partnernetzwerk" (Hofmann holt Material ab) und mit dem
Unternehmensgegenstand „Verwertungsmöglichkeiten für Wertstoffe und Abfälle".

**Ankauf / Verkauf / Gebühren.**
- **Ankauf:** **kein Beleg** für einen Ankauf gegen Vergütung an dieser Stelle. Die Hofmann-Seite
  nennt keine Materialien, keine Preise, keine Annahmebedingungen. „Ankauf" ist damit **unbekannt**,
  nicht „ja".
- **Verkauf:** keine Liste; die Kiesgrube ist ausgekiest, es gibt dort kein Schüttgut-Sortiment
  mehr (das OSM-Objekt „Schüttgutlagerplatz" ist Lagerfläche, kein Verkaufssortiment mit Preisen).
- **Gebühren:** keine Tarife; ob eine Annahme für die Privatbürgerschaft kostenlos ist, ist
  unbelegt.

**Registerbefund.** Im Register (Erhebung 11.08.2026) existiert **keine** Ortsseite Klipphausen
und **kein** Eintrag zu „Sand- und Kiesgruben Sönitz" bzw. „Triebischtalstraße" — die
Namenssuche im Register-Sitemap (6.589 Seiten) liefert nur „SKR Sönitz GmbH" (Lichtenberg), also
eine **andere** Gesellschaft. Auch in der Branchen-Liste der Gemeinde Klipphausen erscheint die
SKS GmbH **ohne** Schrott-/Recycling-Schwerpunkt. **Negativbefund ohne Beweiswert.**

**Bewertung.** Anders als Altmittweida und Chemnitz-Wartburgstraße existiert hier **kein
Katalogeintrag**. Die KAS Klipphausen ist für die Katalognutzer (Privatbürger im Landkreis Meißen)
relevant, und Betreiber, Zeiten und Rechtsform sind belegt. Es fehlen aber: Materialien, Ankauf,
Vergütung, eine Betreiber-Website und die geklärte Vor-Ort-Adresse. **Empfehlung: Dossier
anlegen mit `status: pruefung`** (5.3) — als Fundort mit benanntem Betreiber, ausdrücklich ohne
Preis-, Ankauf- oder Materialbehauptung.

---

## 4. Zusatzfrage: Gibt es in Brandis eine Kleinannahme-Strecke?

**Ergebnis: Nein — Bestätigung für den Bericht, kein Dossier.**

Belege (alle aus der Betreiberseite selbst, keine Fremdquelle nötig):
1. Die Brandis-Seite (`/standorte/brandis-leipzig/`) beginnt mit der **Titelzeile „Betriebsteil
   Brandis-Leipzig"** und beschreibt eine **vollwertige Betriebsstelle**: 60-t-LKW-Waage,
   3-t-Plattenwaage, **Buntmetallschere**, 1 Fuchs-Bagger, 1 LKW Absetzer, 1 LKW Abroller,
   2 Stapler, ca. 5.000 m² Platz + 300 m² Lagerhalle, Neueröffnung 2021.
2. Der Navigationsblock unter der Überschrift „Standorte" führt zwei Zeilen: die **sechs**
   Betriebsstellen („Zwickau | Hirschfeld | Chemnitz | Brandis / Leipzig | Schmölln | Elstertrebnitz")
   und darunter **„Kleinannahmestellen: Chemnitz | Klipphausen | Altmittweida"**. **Brandis steht
   in der KAS-Zeile nicht.** Dasselbe Zeilenmuster steht auf *allen* Seiten, auch auf der
   Brandis-Seite selbst.
3. Der Footer-Kontaktblock „Unsere Standorte" (auf jeder Seite, auch im Impressum) führt
   **Brandis / Leipzig** als „Betriebsteil Brandis/Leipzig" mit Büro und E-Mail — die drei KAS
   erscheinen dort **nicht**.
4. Die **Firmenhistorie** nennt für 2021 „Betriebsteil Brandis / Leipzig" und **keine**
   „KAS Brandis" (KAS-Eröffnungen 2021/2023/2024: Schmölln, Chemnitz-Lutherviertel,
   Chemnitz-Einsiedel, Klipphausen, Altmittweida).
5. Die Hofmann-Instagram-Hashtags (09/2025) führen `#brandis` unter `#standorte`, unter
   `#kleinannahmestellen` nur `#chemnitz_lutherviertel` und `#altmittweida`.

**Schlussfolgerung:** Brandis ist **die Betriebsstelle selbst**, keine zusätzliche Kleinannahme.
Das Dossier `sn-brandis-hofmann-metall-betriebsteil` deckt den Standort vollständig ab; für
Brandis ist **kein** weiteres Dossier sinnvoll. Der einzige sinnvolle Zugewinn ist die
Zeit-/Historiennotiz (Betriebsteil seit 2021) im bestehenden Dossier.

---

## 5. Dossier-Entwürfe und Begründungen

### 5.1 Altmittweida — **kein** neues Dossier; Bestandsdossier anreichern

**Begründung (warum kein neues Dossier):** Der Betrieb existiert bereits im Katalog
(`sn-altmittweida-schrott-buntmetallankauf-buntmetallhande`, Status `aktiv`, mit laufendem
Preis-Handler). Die Hofmann-KAS ist derselbe Standort: **identische Adresse**, **identische
Mobilnummer** (0176 411 925 44 = 017641192544), **identische Öffnungszeiten**. Ein zweites Dossier
würde einen Betrieb doppeln, zwei Punkte an derselben Hausnummer erzeugen und die
Preisbeobachtungshistorie des Handlers auf zwei Slugs aufteilen. Zusätzlich ist die **Rechts-
trägerfrage nicht abschließend geklärt** (Hofmann Metall GmbH vs. lokaler Einzelbetrieb
„Schrott Buntmetallankauf"/„Schrott Scharf Matthias" an derselben Nummer) — dafür gibt es
aktuell **keinen** Beleg, der den lokalen Namen durch den Hofmann-Namen ersetzen ließe; die
Firmen-A-Z der Gemeinde Altmittweida nennt an Hauptstraße 79 sogar einen dritten Namen
(„GM Gerüstbau GmbH Mittweida").

**Empfohlene Änderungen am Bestandsdossier** (nicht ausgeführt — nur Vorschlag):

| Feld | heute | Vorschlag | Begründung |
| --- | --- | --- | --- |
| `description` | leer | Kurztext: Hofmann-Metall-KAS am selben Standort; Adresse/Mobil/Zeiten durch Betreiber-Primärquelle bestätigt | macht die Doppelnennung auflösbar |
| `postcode` | '09648' | unverändert | Betreiber + OSM + Katalog identisch |
| `lat`/`lon` | Dossier leer, DB 50.9677492/12.9570254 | `50.9634358` / `12.9433353` (**Vorschlag**, s. u.) | DB-Punkt löst zu „78c" auf, 1.073 m daneben |
| `opening_hours` | 'Mo-Do 9:00-17:00, Fr 9:00-16:00' | unverändert | identisch mit der Hofmann-Seite |
| `website` | buntmetallhandel-altmittweida.de | unverändert (+ Alternativdomain `schrottbuntmetallankauf.de` in die Notiz) | Preisquelle bleibt die tatsächliche Preisseite |
| `status` | aktiv | unverändert | Preisquelle liefert täglich |
| `notes`/Timeline | — | ergänzen: „Hofmann Metall GmbH führt den Standort seit 2024 als KAS Altmittweida (Firmenhistorie + Facebook 25.07.2024); Nummer/Zeiten identisch" | Historie ohne Feldänderung |

**Zum Koordinaten-Vorschlag:** Die Repo-Praxis (vgl. `sn-hainichen-uhlmann-finke`, GEO-Evidenz vom
27.10.2026) akzeptiert Betreiber-Adresse + Nominatim/OSM-Gebäudegeometrie als zwei Quellen. Für
„Hauptstraße 79" liefert die Nominatim-Vorwärtssuche das adresskongruente OSM-Gebäude (Way
201731552, Kategorie `building`) auf 50.9634358/12.9433358. Der Vorschlag ist ein **Gebäude-/
Adressreferenzpunkt**, keine belegte Anlieferzufahrt — das gehört in die Notiz.

### 5.2 Chemnitz-Wartburgstraße — **kein** neues Dossier; bestehenden Klärfall auflösen

**Begründung (warum kein neues Dossier):** Die Zeile
`sn-chemnitz-ankauf-von-buntmetallen-stahlschrott` ist bereits der Standort Wartburgstr. 2e; ihre
Mobilnummer `0176 55564352` ist **identisch** mit der auf der Hofmann-KAS-Seite veröffentlichten.
Die KAS ist kein zusätzlicher Betrieb, und sie ist **nicht** derselbe Betrieb wie einer der sechs
Betriebsteile (anderer Ort, andere Nummer). Der offene Punkt war 2026 lediglich der fehlende
Betreiberauftritt — dieser liegt jetzt vor. Ein neues Dossier erzeugte eine zweite Zeile am selben
Ort mit derselben Nummer.

**Empfohlene Änderungen am Bestandsdossier** (nicht ausgeführt):

| Feld | heute | Vorschlag | Begründung |
| --- | --- | --- | --- |
| `status` | `pruefung` | `aktiv` | Betreiber-Primärquelle vorhanden; Zeile ist ein realer, seit 2021 geführter Annahmepunkt |
| `name` | „Ankauf von Buntmetallen & Stahlschrott" | „Hofmann Metall GmbH, Kleinannahmestelle Chemnitz" (Slug bleibt unverändert) | Betreiber-Primärquelle; **Slug bleibt** `sn-chemnitz-ankauf-von-buntmetallen-stahlschrott` (Stabilität über Kosmetik) |
| `street` | Wartburgstr. 2e | Wartburgstraße 2e | Schreibweise der Betreiberseite |
| `postcode` | leer | '09126' (**mit Notiz zum Konflikt**) | GoLocal („09126, Bernsdorf") + OSM-Adresse („09126"); Hofmann schreibt 09123 → Konflikt wird in der Notiz offengelegt, nicht verdeckt |
| `phone` | 0176 55564352 | `0176 55564352` + `0371 540 46` (Mehrfachrufnummer) | Mobil = Hofmann-Beleg; Landline = GoLocal (Lead), deshalb mit Quellenkennzeichnung |
| `opening_hours` | leer | 'Mo-Mi 09:00-18:00, Do geschlossen, Fr 09:00-16:00, Sa 09:00-12:00' | Betreiber-Primärquelle |
| `website` | leer | `https://hofmann-metall.de/standorte/kleinannahme-chemnitz/` | Betreiber-Primärquelle |
| `website_status` | unbekannt | aktiv | Seite live abgerufen 28.10.2026 |
| `lat`/`lon` | Dossier leer, DB 50.8207333/12.9104667 | `50.8197217` / `12.9408697` (**Vorschlag**) | DB-Punkt ist der alte, für TSR Chemnitz als falsch markierte Altpunkt (2.141 m daneben) |
| `trader_type` | metallhaendler | metallhaendler (unverändert) | Buntmetall-/Stahl-Annahme, vom Betreiber als Annahmestelle geführt |

**Vorbedingung für die Umbenennung:** die Identität der Landline
(`0371 / 54 0 46` = GoLocal „Ankauf von Buntmetall M. Reichel", Nutzermeldung) ist zu klären, bevor
„Hofmann Metall GmbH" als Betreiber **endgültig** festgeschrieben wird — sonst entsteht ein
Phantom wie bei `rp-kaiserslautern-67655-peter-marco`. Die Notiz muss den Gegenbeleg
(Reichel-Eintrag) ausdrücklich nennen.

### 5.3 Klipphausen — **Dossier-Entwurf** (einzige echte Lücke)

**Begründung (warum hier ein Dossier sinnvoll ist):**
- Für diesen Standort existiert **kein** Katalogeintrag; die Lücke ist real (im Gegensatz zu
  Altmittweida/Chemnitz).
- Der Standort ist eine **seit 2023 betriebene Annahmestelle** im Netz des Betreibers
  (Firmenhistorie), mit veröffentlichten Öffnungszeiten — für die Katalognutzer im Landkreis
  Meißen ist das eine nutzbare Information.
- Der Betreiber ist eine **eingetragene Kapitalgesellschaft mit belegtem Registerstatus**
  (HRB 29510 AG Chemnitz) und mit kommunaler Firmenverzeichnis-Nennung; der Unternehmensgegenstand
  umfasst Abfall- und Verwertungsleistungen.
- Es ist **keine** kommunale/nicht gewerbliche Annahmestelle und **kein** Abholstelle ohne Betreiber
  vor Ort: Der Betreiber ist namentlich benannt.

**Begründung der Einschränkungen (warum `pruefung` und keine Ankaufsbehauptung):**
- Kein Beleg, **was** angenommen wird, **ob** gegen Vergütung angekauft wird, ob die Annahme für
  Privatbürger kostenlos ist, und ob die Hofmann-Adresse vor Ort überhaupt die SKS-Adresse ist
  (489 m Abstand).
- Der Standort ist nicht im amtlichen Register geführt; die Kiesgrube ist ausgekiest und in
  Rekultivierung — der Betriebszweck ist Abfall-/Verwertungsdienstleistung, nicht belegter
  Schrottankauf.
- Deshalb: `trader_type: sonstige`, `status: pruefung`, **keine** Preise, **keine**
  Materialzusage, `dropoff_json`/`pickup_json` leer (fehlendes `allowed` = unbekannt, nicht „nein").

**Entwurf (nicht angelegt):**

```markdown
---
slug: sn-klipphausen-sand-und-kiesgruben-soenitz-kleinannahme
name: Sand- und Kiesgruben Sönitz GmbH, Kleinannahmestelle Klipphausen
trader_type: sonstige
state: SN
city: Klipphausen
street: Triebischtalstraße 19
postcode: '01665'
lat: '51.1045768'
lon: '13.4170816'
phone: ''
email: ''
opening_hours: 'Mo geschlossen, Di-Fr 08:30-17:30, Sa 08:00-12:00, So geschlossen'
website: https://hofmann-metall.de/standorte/kas-klipphausen/
website_status: aktiv
status: pruefung
description: 'Kleinannahmestelle Klipphausen im Annahmenetz der Hofmann Metall GmbH (HRB 5475 AG Chemnitz, Hauptsitz Zwickau). Die Standortseite der Hofmann Metall GmbH nennt als Betreiber ausdrücklich die Sand- und Kiesgruben Sönitz GmbH (HRB 29510 AG Chemnitz, Geschäftsanschrift Paracelsusstraße 10, 09114 Chemnitz). Betriebsgegenstand der GmbH: Sand- und Kiesgewinnung sowie Abfalloptimierung, Umweltdienstleistungen, Rekultivierung und die Aufsuche von Verwertungsmöglichkeiten für Wertstoffe und Abfälle. Der Kiesgrubenbetrieb Sönitz ist seit 2021 ausgekiest und wird rekultiviert. Annahmematerialien, Ankauf gegen Vergütung, Preise und Gebühren werden an dieser Stelle nicht veröffentlicht und sind unbelegt; die Seite nennt keine Telefonnummer. Die Hofmann-Adresse Triebischtalstraße 19 liegt rund 490 m vom vom Gemeindeverzeichnis und OSM geführten Betriebsansitz An der Kiesgrube 1a (Weitzschen) entfernt, welcher als Sandgrube Sönitz kartiert ist.'
dropoff_json: ''
pickup_json: ''
provenance_section: 'Recherche Kleinannahmestellen Hofmann Metall 28.10.2026'
provenance_ankauf_raw: unbekannt
provenance_origin: table
---

# Sand- und Kiesgruben Sönitz GmbH, Kleinannahmestelle Klipphausen

## Überblick

Die Hofmann Metall GmbH führt an ihrer Website unter „Standorte" drei **Kleinannahmestellen** (KAS):
Chemnitz (Wartburgstraße 2e), Klipphausen (Triebischtalstraße 19) und Altmittweida
(Hauptstraße 79). Die KAS Klipphausen ist seit 2023 in Betrieb und wird von Hofmann mit
Öffnungszeiten, aber **ohne** Telefonnummer, ohne Materialangabe und ohne Preise veröffentlicht. Als
Betreiber nennt die Seite ausdrücklich die **Sand- und Kiesgruben Sönitz GmbH** (HRB 29510 AG Chemnitz).
Die KAS ist damit eine **Partner-Annahmestelle** im Netz der Hofmann Metall GmbH — kein eigener
Betriebsteil und keine kommunale Stelle.

**Ankauf:** **unbekannt.** Für einen Ankauf gegen Vergütung an dieser Stelle gibt es keinen Beleg;
die Seite nennt weder Sorten noch Kurse, und die Hofmann-Website veröffentlicht dort keine
Preisliste. **Verkauf:** keine Material- oder Produktpreisliste; die Kiesgrube Sönitz ist seit 2021
ausgekiest, es besteht dort kein Schüttgut-Sortiment mit Preisen. **Gebühren:** keine Tarife
veröffentlicht; ob die Annahme für Privatbürger kostenlos ist, ist unbelegt.

**GEO:** Die Koordinate ist der adresskongruente OSM-Gebäudepunkt zu „Triebischtalstraße 19"
(way 484710243), nicht der vom Betreiber geführte Ansitz „An der Kiesgrube 1a" (rund 490 m
entfernt). Gebäudepunkt, keine belegte Anlieferzufahrt; die Frage, welche der beiden Adressen die
KAS ist, ist offen (siehe Timeline).

## Timeline

### Recherche 28.10.2026

- [Recherche 28.10.2026: Standort als eigene KAS belegt — Hofmann-Standortseite führt „Triebischtalstraße 19, 01665 Klipphausen", „Betreiber: Sand- und Kiesgruben Sönitz GmbH", Öffnungszeiten Mo geschlossen / Di–Fr 08:30–17:30 / Sa 08:00–12:00 / So geschlossen; die Firmenhistorie der Hofmann Metall GmbH nennt für 2023 „Inbetriebnahme der KAS Klipphausen". **Keine Telefonnummer, keine E-Mail, keine Sorten- und keine Preisangabe** auf der Seite. Quellen: https://hofmann-metall.de/standorte/kas-klipphausen/ ; https://hofmann-metall.de/firmenhistorie/ ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: Betreiber rechtlich belegt — Handelsregisterauszug HRB 29510 AG Chemnitz: Sand- und Kiesgruben Sönitz GmbH, GmbH, Geschäftsanschrift Paracelsusstraße 10, 09114 Chemnitz, Stammkapital 25.000,00 EUR, Gesellschaftsvertrag vom 11.08.2020 mit Gegenstand „Betrieb und Ausbeutung der Sand- und Kiesgruben in Sönitz …; Geschäfte der Abfalloptimierung wie z.B. Abfallbeseitigung; Umweltdienstleistungen für Industriebetriebe …; Rekultivierung und Rückbau baulicher Anlagen; Aufsuchen von Verwertungsmöglichkeiten für Wertstoffe und Abfälle aller Art …". Das Firmenverzeichnis der Gemeinde Klipphausen nennt als lokalen Kontakt Michael Gräßler, An der Kiesgrube 1a, 01665 Klipphausen (Weitzschen), Tel 035244 41701, Fax 035244 492302; Creditreform führt +49 352 4441701 unter der Sitzanschrift Paracelsusstr. 10, 09114 Chemnitz. **Abgrenzung:** die SKR Sönitz GmbH (Lichtenberg/Erz, skr-soenitz.de) ist eine andere Gesellschaft. Quellen: https://www.online-handelsregister.de/handelsregisterauszug/sn/Chemnitz/HRB/29510/Sand-und-Kiesgruben-Soenitz-GmbH ; https://klipphausen.de/firmenverzeichnis/ ; https://firmeneintrag.creditreform.de/09114/3030321552/SAND_UND_KIESGRUBEN_SOENITZ_GMBH ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Adressklärung offen.** Die Hofmann-Seite nennt Triebischtalstraße 19; OSM kartiert unter dieser Nummer ein Gebäude (way 484710243) auf 51.1045768/13.4170816, während das Gemeindeverzeichnis und OSM den Betriebsansitz unter „An der Kiesgrube 1a" (OSM-Way 209515126, Name „Sandgrube Sönitz") auf 51.107461/13.422355 führen — Abstand rund 490 m. Beide liegen im Gewerbegebiet Miltitz/Weitzschen. Der eingetragene Punkt markiert die **von Hofmann veröffentlichte** Adresse, nicht den belegten Betriebsansitz. Quellen: https://nominatim.openstreetmap.org/search?q=Triebischtalstra%C3%9Fe+19%2C+01665+Klipphausen&format=jsonv2&addressdetails=1 (28.10.2026) ; https://www.openstreetmap.org/way/484710243 ; https://www.openstreetmap.org/way/209515126 ; Overpass-Abfrage um 51.1045768/13.4170816 (Radius 600 m, 28.10.2026).]
- [Recherche 28.10.2026: **Registerbefund (negativ, ohne Beweiswert).** Im Verzeichnis
  schrottregister.pages.dev (Erhebung 11.08.2026) existiert keine Ortsseite Klipphausen und kein
  Eintrag zu „Sand- und Kiesgruben Sönitz" oder „Triebischtalstraße"; die Namenssuche im
  Register-Sitemap (6.589 Seiten) liefert nur „SKR Sönitz GmbH" (Lichtenberg) — andere Gesellschaft.
  Die Hofmann Metall GmbH selbst ist im Register geführt (betrieb-hofmann-metall-gmbh-zwickau,
  Zertifikatsnummer ZZIT00700096, Äußere Dresdner Straße 80, 08066 Zwickau, Tätigkeiten Lagern /
  Behandeln / Verwerten / Beseitigen, 29 Abfallschlüssel) — aber nur mit der Registeranschrift des
  Hauptsitzes; keine der Betriebsstellen ist einzeln eingetragen. Ein Negativbefund im Register
  beweist keine Nichtexistenz. Quellen: https://schrottregister.pages.dev/betrieb-hofmann-metall-gmbh-zwickau ;
  https://schrottregister.pages.dev/sitemap.xml ; https://schrottregister.pages.dev/ort-chemnitz ;
  https://schrottregister.pages.dev/ort-mittweida ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Betriebszustand (amtlich).** Sächsisches Amtsblatt Nr. 12/2025
  (Bekanntmachung des Sächsischen Oberbergamtes vom 06.03.2025): „Die Sand- und Kiesgruben Sönitz
  GmbH betreibt seit dem Jahr 2015 die bereits seit dem Ende des 19. Jahrhunderts betriebene
  Kiessandgrube Sönitz. Die Kiessandgrube Sönitz ist auf den Bestandsflächen seit 2021 vollständig
  ausgekiest. **Die Gewinnungstätigkeit in der Kiessandgrube ist eingestellt.**" Die Fläche befindet
  sich in Rekultivierung (Abschlussbetriebsplan, Allgemeine Vorprüfung der UVP-Pflicht). Das ist mit
  dem Bild einer Partner-Annahmestelle vereinbar, belegt aber **keinen** Schrottankauf. Quelle:
  https://www.oba.sachsen.de/download/Amtsblatt_12_2025.pdf ; Abruf 28.10.2026.]
- [Recherche 28.10.2026: **Abgrenzung zu den übrigen Hofmann-Standorten.** Die Hofmann-Website
  trennt redaktionell sechs Betriebsstellen (Zwickau, Chemnitz, Hirschfeld, Brandis/Leipzig, Schmölln,
  Elstertrebnitz) von drei Kleinannahmestellen (Chemnitz, Klipphausen, Altmittweida): nur die sechs
  stehen im Footer-Kontaktblock „Unsere Standorte" und auf der Kontaktseite; die Zeile
  „Kleinannahmestellen:" führt auf jeder Seite nur Chemnitz, Klipphausen und Altmittweida. Die
  Pressemitteilung vom 10.04.2025 nennt „fünf Standorten, drei Kleinannahmestellen", die
  Instagram-Hashtags (09/2025) nennen unter `#kleinannahmestellen` nur
  `#chemnitz_lutherviertel` und `#altmittweida` — Klipphausen ist dort nicht enthalten, weil die
  Seite einen fremden Betreiber ausweist. Quellen: https://hofmann-metall.de/standorte/kas-klipphausen/ ;
  https://hofmann-metall.de/privat-kleingewerbekunden/ ; https://www.presseportal.de/pm/172422/6010468 ;
  Abruf 28.10.2026.]
```

**Slug-/Namensentscheidung vor dem Anlegen (Owner):** Der Entwurf trägt den **Betreiber** im Namen
(`sn-klipphausen-sand-und-kiesgruben-soenitz-kleinannahme`), weil die Betreiberseite den Betreiber
ausdrücklich benennt und der Katalog eine Entität führt, die jemand betreibt. Alternativ wäre
`sn-klipphausen-hofmann-metall-kleinannahme` mit dem Namen „Hofmann Metall GmbH, Kleinannahmestelle
Klipphausen" (Betreiber im `description`) möglich und würde die Geschwister-Dossiers
(Betriebsteil-Muster) spiegeln. **Die Slug-Entscheidung ist vor dem Anlegen zu treffen**, weil
Slugs später nie geändert werden.

---

## 6. Offene Punkte

1. **Identität Chemnitz-Wartburgstraße 2e.** Die Hofmann-KAS veröffentlicht als „Büro" die
   Landline `0371 / 54 0 46`, die GoLocal (unbestätigte Nutzermeldung) einem **anderen** Namen
   („Ankauf von Buntmetall M. Reichel") an derselben Adresse zuordnet. Zu klären: Übernahme,
   Zusammenarbeit oder zwei Betriebe an einer Adresse? Erforderlich für Name/Typ der Zeile.
   Hinweis: die 5-stellige Nummer wird als Telefon **und** Fax geführt (Kurzwahl-Verdacht) — eine
   Anrufprobe wäre der schnellste Test (nicht im Auftrag enthalten).
2. **PLZ 09123 vs. 09126** für Wartburgstraße 2e (Betreiber gegen GoLocal + OSM).
3. **KAS-Adresse Klipphausen:** Triebischtalstraße 19 (Hofmann) gegen An der Kiesgrube 1a
   (Gemeindeverzeichnis/OSM) — 490 m Abstand. Der Koordinaten-Vorschlag markiert die Hofmann-Adresse.
4. **Was wird in Klipphausen angenommen, und wird vergütet?** Ohne Beleg weder `dropoff`-Zusage noch
   `trader_type` schärfer als `sonstige`/`pruefung`. Belegbar wäre eine Auskunft des Betreibers
   (035244 41701) oder ein Vor-Ort-Besuch/Handyfoto des Standorts.
5. **KAS Chemnitz-Einsiedel** (Firmenhistorie 2021, Facebook-Video ~2022) wird auf der Website nicht
   mehr geführt — Schließung, Umbenennung oder Aufnahme in einen der sechs Betriebsteile ist **nicht**
   belegt. In der Katalogzeile `sn-chemnitz-recycling-und-containerdienst-ralf-siebe` (Am Rummel 5,
   09123 Chemnitz = Einsiedel) liegt eine **Hypothese**, kein Beleg; sie ist zu prüfen oder zu
   verwerfen.
6. **Rechtsträger Altmittweida:** Hofmann Metall GmbH vs. lokaler Einzelbetrieb („Schrott Buntmetallankauf",
   „Schrott Scharf Matthias", „GM Gerüstbau GmbH") an Hauptstraße 79. Für die Dossier-Benennung ist
   das zu entscheiden; die drei Telefonnummern am Ort (0176 411 925 44 / 0176 55217721 /
   03727 979789) und das Fehlen eines Impressums der beiden Webseiten erschweren die Klärung. Das
   Impressum der Hofmann-Seite deckt nur die Gesellschaft, nicht den Standortbetreiber.
7. **GEO-Drift Dossier ↔ Produktion:** Für beide Bestandszeilen hält die Produktion einen Punkt, der
   **nicht** im Dossier steht (leeres Dossier-Feld = „behalten“). Die Punkte sind falsch
   (Altmittweida 1.073 m, Chemnitz 2.141 m). Nach `docs/trader-seed.md` muss die Korrektur
   **im Dossier** erfolgen; ein leeres Dossier-Feld behält den Altpunkt.
8. **Namensträger der Altmittweida-Katalogzeile:** Der Name „Schrott Buntmetallankauf (Buntmetallhandel
   Altmittweida)" stammt aus dem Webador-Template ohne Impressum; eine Umbenennung auf Hofmann würde
   den laufenden Preis-Handler nicht brechen (er hängt am Slug), sollte aber erst nach Klärung von
   Punkt 6 erfolgen.
9. **Nicht veröffentlichtes EFB-Anlage-3-Zertifikat:** Das EfbV-Zertifikat der Hofmann Metall GmbH
   (TMS 12 150 9618, Auftrag 70002426, Audit 16.03.2026, gültig bis 16.09.2027) gilt ausdrücklich nur
   „in Verbindung mit dem Zertifikat nach Anlage 3 (zu § 25) EfbV und für die dort genannten
   **Standorte**, Tätigkeiten und Abfallarten". Dieses Anlage-3-Zertifikat ist auf der Website nicht
   veröffentlicht — damit ist nicht belegt, **welche** der neun Seiten vom EFB-Zertifikat erfasst
   sind. Das ist die einzige amtliche Quelle, die die KAS-Stellen als EFB-Standorte abdecken könnte.
10. **Katalogweit:** Für die drei KAS gibt es keinen Preis-Handler, weil keine Preise veröffentlicht
    werden. Das ist korrekt und soll so bleiben; falls eine KAS später Preise veröffentlicht, ist
    ein Handler analog `hofmann_metall.rs` zu ergänzen.

---

## 7. Quellen (alle abgerufen am 28.10.2026)

**Betreiber-Primärquellen**
- https://hofmann-metall.de/standorte/kleinannahme-altmittweida/
- https://hofmann-metall.de/standorte/kleinannahme-chemnitz/
- https://hofmann-metall.de/standorte/kas-klipphausen/
- https://hofmann-metall.de/standorte/brandis-leipzig/
- https://hofmann-metall.de/firmenhistorie/
- https://hofmann-metall.de/impressum/
- https://hofmann-metall.de/privat-kleingewerbekunden/
- https://hofmann-metall.de/kontakt/
- https://hofmann-metall.de/zertifikate/ · EfbV-PDF:
  https://hofmann-metall.de/files/hmm/content/img/zertifikate/EfbV%20Zertifikat%2026-27.pdf
- https://hofmann-metall.de/sitemap.xml
- https://hofmann-metall.de/news-details/23/ (KAS Schmölln, 07.06.2021) ·
  https://hofmann-metall.de/news-details/neuer-standort-in-elstertrebnitz-ab-sofort-geoeffnet/ ·
  https://hofmann-metall.de/aktuelles-presse/
- https://www.presseportal.de/pm/172422/6010468 (10.04.2025: „fünf Standorte, drei Kleinannahmestellen")

**Register / Behörden**
- https://schrottregister.pages.dev/ · /betrieb-hofmann-metall-gmbh-zwickau (ZZIT00700096) ·
  /ort-chemnitz · /ort-mittweida · /ort-zwickau · /sitemap.xml (6.589 Seiten)
- https://www.online-handelsregister.de/handelsregisterauszug/sn/Chemnitz/HRB/5475/Hofmann-Metall-GmbH
- https://www.online-handelsregister.de/handelsregisterauszug/sn/Chemnitz/HRB/29510/Sand-und-Kiesgruben-Soenitz-GmbH
- https://www.oba.sachsen.de/download/Amtsblatt_12_2025.pdf (Sächs. Oberbergamt, UVP Kiessandgrube Sönitz)
- https://klipphausen.de/firmenverzeichnis/
- https://firmeneintrag.creditreform.de/09114/3030321552/SAND_UND_KIESGRUBEN_SOENITZ_GMBH

**Leads (Aggregatoren, ausdrücklich kein Beleg)**
- GoLocal: „Ankauf von Buntmetall M. Reichel" (Wartburgstr. 2 E, 09126) und „Hofman Metall GmbH"
  (Blankenburgstr. 104) — https://www.golocal.de/chemnitz/recycling
- Cylex: „Buntmetallhandel Altmittweida" (Hauptstraße 79, 0176 55217721, matthias-scharf6119@gmx.de)
- schrottplatz-info.de: „Schrott Scharf Matthias" (Hauptstr. 79, 03727 979789)
- Gemeinde Altmittweida, Firmen A–Z: „GM Gerüstbau GmbH Mittweida", Hauptstraße 79, 03727/998804
- Hofmann Facebook: „Neu in Altmittweida! Seit Anfang Juli …" (25.07.2024) ·
  Facebook-Video „Kleinannahmestelle in Chemnitz Einsiedel" (~2022)

**Geo/OSM**
- Nominatim-Vorwärtssuche „Hauptstraße 79, 09648 Altmittweida" → 50.9634358/12.9433358 (way 201731552)
- Nominatim-Vorwärtssuche „Wartburgstraße 2e, 09123 Chemnitz" → 50.8197217/12.9408697 (way 566201450, industrial)
- Nominatim-Vorwärtssuche „Triebischtalstraße 19, 01665 Klipphausen" → 51.1045768/13.4170816 (way 484710243)
- Nominatim-Rückwärtsauflösung der Bestandspunkte: 50.9677492/12.9570254 → „78c, Hauptstraße";
  50.8207333/12.9104667 → „11, Parkstraße, Kapellenberg"
- Overpass (Radius 600 m um 51.1045768/13.4170816): way 209515126 „Sandgrube Sönitz" (An der Kiesgrube 1a),
  way 246298218 „Kiesgrube Sönitz" (landuse=quarry), way 217526322 „Kiesgrube Sönitz - Schüttgutlagerplatz",
  way 136173491/136173492 „An der Kiesgrube"

**Produktionsabgleich (read-only, `mode=ro` + `PRAGMA query_only=ON`)**
- `sn-altmittweida-schrott-buntmetallankauf-buntmetallhande`: phone 017641192544, Öffnungszeiten
  identisch mit Hofmann-KAS, DB-Punkt 50.9677492/12.9570254 (1.073 m daneben)
- `sn-chemnitz-ankauf-von-buntmetallen-stahlschrott`: phone 0176 55564352, status `pruefung`,
  DB-Punkt 50.8207333/12.9104667 (2.141 m daneben; identisch mit dem am 18.10.2026 als falsch
  markierten TSR-Altpunkt)
- Kein Katalogeintrag zu Klipphausen / Triebischtalstraße / Sönitz.
