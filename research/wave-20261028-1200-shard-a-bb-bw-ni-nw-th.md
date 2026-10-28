# 6h-Welle 28.10.2026 12:00 UTC, Shard A (BB, BW, NI, NW, TH) — 20 Dossiers

Recherche ausschließlich lesend; Produktionsdatenbank nur read-only
(`file:…public.db?mode=ro`, `PRAGMA query_only=ON`). Kein Commit, kein Push, kein Deploy.
Jedes der 20 Dossiers trägt mindestens eine neue Notiz vom 28.10.2026 im Format
`[Recherche 28.10.2026: …; Quelle(n): …]`; bestehende Timeline-Notizen wurden nicht
angefasst, Slugs blieben unverändert. `scripts/release/wave_gate.py HEAD` und
`cargo test -p schrott-mcp-ingestion seed` sind grün.

## Die Hebel dieser Welle

1. **Betreiber-Primärquellen**, die in den Vorwellen gesucht, aber nicht gefunden waren:
   `mobschrott.de` (Drebkau), `reinhardt-antik.de` (Freiburg), `renz-rohstoffe.de`
   (Villingen-Schwenningen), `scholz-recycling.com/standort/muehlhausen`.
2. **Amtliches Fachbetrieberegister** (Fachbetrieberegister der Gemeinsamen Stelle
   Altfahrzeuge § 7 Abs. 2a AltfahrzeugV + Register der Entsorgungsfachbetriebe § 56 KrWG,
   Auszug 11.08.2026), gespiegelt über `schrottregister.pages.dev` je Ort- und
   Betriebsseite. Der Spiegel weist sich selbst als „Unabhängiges Verzeichnis · kein
   amtliches Angebot" aus und wird wie in den Wellen vom 26./27./28.10. als
   Registerquelle geführt, nicht als Aggregator. **Negativbefunde beweisen keine
   Nichtexistenz**: die EFB-Anerkennung ist freiwillig, und für Gütersloh, Drebkau,
   Schorbus und Jüterbog existiert im Spiegel überhaupt keine Ortsseite.
3. **IHK-Umweltbüro Berlin-Brandenburg** (`ubb.de`, Anlaufstellen zur Entsorgung von
   Abfällen und Wertstoffen): eine amtlich-nahe Branchenquelle mit Person, Anschrift,
   Telefon, E-Mail und Tätigkeitsangabe pro Betrieb. Sie belegte den Velten-Betrieb
   unabhängig vom Register.
4. **Handelsregister** (AG Dortmund HRB 5378, AG Neuruppin GsR 310) und die
   amtlichen Registerdarstellungen (NorthData, Registercheck, Creditsafe, handelsregister.ai)
   zur Namens-, Anschrift- und Liquidationslage.
5. **GEO-Qualitätsprüfung**: der in der Produktion gespeicherte Punkt wurde für alle
   20 Slugs read-only per Nominatim-Rückwärtssuche aufgelöst und mit einem
   OSM-Hausnummernpunkt abgeglichen. Ergebnis: **17 falsche oder unbrauchbare
   Punkte** (einer rund 150 km daneben), **7 durch einen belegten Hausnummernpunkt
   ersetzt**, **10 zurückgezogen** ohne Ersatz, **1 war bereits korrekt** (Potsdam, jetzt
   zusätzlich im Dossier verankert).

## Ergebnis in Zahlen

| Kennzahl | Wert |
|---|---|
| Bearbeitete Dossiers | 20 (alle mit Notiz vom 28.10.2026) |
| Frontmatter geändert | 19 (nur `th-muehlhausen-mrg-rueckbau-und-recycling` bleibt reine Timeline-Notiz) |
| `status` → `aktiv` | 1 (`bb-velten-kraatz-...`) — **Nachtrag des Owners:** `bb-drebkau-ot-schorbus-...` war am selben Tag auf `aktiv` gesetzt, wurde nach der Prüfung wegen des fehlenden Registernachweises (Einzelunternehmer) aber auf `pruefung` zurückgenommen |
| `status` → `pruefung` | 1 (`bw-freiburg-leonardo-reinhardt-schrotthandel`) |
| `name` korrigiert | 3 (Velten, Potsdam, Dortmund) |
| `certifications` gesetzt | 3 (ZAPS00100020, ZZEE00300224, ZZHT00300696) |
| Neue Koordinaten (lat/lon) | 8 (Drebkau, Potsdam, Ehningen, Uslar, Wilhelmshaven, Dormagen, Jüterbog, Ilmenau) |
| `clear_fields: geo` gesetzt | 10 (Velten, Freiburg, Mannheim, Villingen, Celle, Hildesheim, Gütersloh, Gera, Dortmund, Scholz Mühlhausen) plus Bestand Hagen |
| Belegte **Ankaufs**preise (Zahl) | **1 Fund**: mobschrott.de, €/t, ohne Jahr |
| Belegte **Verkaufs**preise | 0 |
| Belegte **Gebühren**tarife | 0 neu; 1 bestätigte Freistellung (Containerstellung kostenlos, Lucia) |

## Frontmatter-Änderungen im Einzelnen

- **`bb-drebkau-ot-schorbus-mobiler-schrottankauf-christian-lucia`** — Website
  `http://www.mobschrott.de/` mit Impressum, PLZ 03116, Telefon 035602 22909,
  E-Mail c.lucia@t-online.de, `website_status: aktiv`, `status: aktiv`,
  `description`, `pickup_json`, Koordinate 51.6634679/14.2835530.
- **`bb-velten-kraatz-hartmut-fuhrbetrieb`** — Name „Fuhrbetrieb Hartmut Kraatz",
  PLZ 16727, Telefon 03304 502435, E-Mail fuhrbetrieb-kraatz-velten@t-online.de,
  `status: aktiv`, Zertifikat ZAPS00100020 (Demontagebetrieb), `clear_fields: geo`.
- **`bb-potsdam-vhz-schrott-verwertung-handelszentrum`** — Name „VHZ Verwertung &
  Handelszentrum GmbH" (Registerfirmierung nach der Umfirmierung 2015), Zertifikat
  ZZEE00300224 mit Tätigkeiten und Abfallschlüsseln, `description`, Koordinate
  52.3667716/13.1107131.
- **`bw-ehningen-kurt-rie-nachfolger`** — Zertifikat ZZHT00300696 mit Tätigkeiten und
  Abfallschlüsseln, `description`, Koordinate 48.6605542/8.9399967.
- **`th-muhlhausen-scholz-recycling-standort-muhlhausen`** — Straße, PLZ, Telefon,
  E-Mail, `description`, `clear_fields: geo`.
- **`bw-freiburg-leonardo-reinhardt-schrotthandel`** — Straße, PLZ, Telefon, E-Mail,
  Website reinhardt-antik.de (`website_status: aktiv`), `status: pruefung`,
  `description`, `clear_fields: geo`.
- **`ni-uslar-schrotthandel-matthias-warneke`**, **`ni-wilhelmshaven-26386-braune-r`**,
  **`nw-dormagen-dormagen-moo`**, **`bb-juterbog-bottcher-harald-schrotthandel`**,
  **`th-ilmenau-link-manfred-schrottentsorgung`** — `description` und Koordinate auf den
  belegten Adresspunkt (Hausnummer plus Verzeichnisangabe).
- **`bw-mannheim-seckenheim-specht-recycling-specht-frederik`**,
  **`bw-villingen-schwenningen-renz-rohstoffe-renz-container`**,
  **`ni-celle-29225-schrott-abholung-celle`**, **`ni-hildesheim-schrott-handel-miro`**,
  **`nw-gutersloh-gmg`**, **`th-gera-richter-schrott-nicht-schrotthandel-rich`**,
  **`nw-dortmund-walter-leidag`** — `description` und/oder `clear_fields: geo`.
- **`nw-dortmund-walter-leidag`** — Name „Schrotthandlung Walter Leidag GmbH".
- **`th-muehlhausen-mrg-rueckbau-und-recycling`**, **`nw-hagen-derda`** — nur
  Timeline-Notiz (kein Frontmatter-Feld geändert, `clear_fields: geo` in Hagen Bestand).

## Preise und Gebühren (Ankauf, Verkauf, Gebühren getrennt)

**Ankauf.** Ein einziger echter Zahlenfund in der ganzen Welle, und er ist nicht als
Kurs verwendbar: Die Betreiberseite `http://www.mobschrott.de/Schrottpreise.htm` nennt
eine monatlich aktualisierte Liste „Preise pro Tonne" für den **November ohne
Jahresangabe**: Mischschrott 125,00 EUR/t, schwere Schere 135,00 EUR/t, Sorte 3
(Kernschrott) 165,00 EUR/t, Handelsguss 135,00 EUR/t; NE-Metalle ausdrücklich nur
tagesaktuell auf Anfrage, Bezahlung vor Ort, Containerstellung kostenlos. Die
Fußzeile der Seite weist 2012 aus, es ist also weder ein aktueller Kurs noch ein
veröffentlichungsfähiges Datum ableitbar. Der Wert steht ausschließlich im Dossier.
In den übrigen 19 Dossiers wurde **keine** Zahlenpreisliste gefunden; bei Renz
(Villingen-Schwenningen) sind es Tagespreise nach Börsenkurs ohne Liste, bei
Scholz Mühlhausen tagesaktuelle Preise auf Anfrage.

**Verkauf.** Kein Dossier enthält ein belegtes Material-Verkaufsangebot oder ein
Preisblatt für den Verkauf von Recyclingmaterial. Die „Gutschrift nach dem Verkauf"
bei Renz ist eine Verrechnung mit dem Betreiber, kein Verkauf von Material durch ihn.

**Gebühren.** Zwei Funde, die ausdrücklich **nicht** diesem Händler gehören und deshalb
nicht übernommen wurden: die Kleinmengengebühren des kommunalen Entsorgungszentrums
Wilhelmshaven (4,00 / 8,00 / 16,00 EUR je cbm, gewerblich nach Gewicht) und die
Abholgebühren des Abfallwirtschaftsbetriebs Ilm-Kreis (Sperrmüll 30,00 EUR, Elektroschrott
20,00 EUR je Abholung). Beide sind Gebühren öffentlicher Stellen. Echte Betreibergebühr
ist nur die kostenlose Containerstellung bei Lucia; bei Renz sind Containergrößen
(6/7/10/17/36 m³) und eine Metallwert-Gutschrift ohne Betrag genannt. Kein Dossier
belegt eine kostenlose Schrottentsorgung.

## GEO: zurückgezogene Punkte, gesetzte Koordinaten, Geokodierbedarf

**Zurückgezogen (`clear_fields: geo`, kein Ersatz gesetzt)** – die gespeicherten Punkte
lösten sich zu einem anderen Ort auf: Velten (Große Promenade 72 statt Kanalstraße),
Freiburg (Egonstraße 20 statt Auggener Weg), Mannheim (Uhlandstraße 20, 68167),
Villingen (Sebastian-Kneipp-Straße 62 statt Klippeneckstraße 62), Celle
(Sägemühlenstraße 8, 29221 statt Denickestraße 29225), Hildesheim (Zingel 38 statt
An der Innerste-Au), Gütersloh (Stadtring Nordhorn 33332 statt Hermann-Nohl-Straße 12),
Gera (Meuselwitzer Straße 33a statt Söllmnitz 6b), Dortmund (Straßenkörper Juliusstraße
statt Hausnummer 28). **Scholz Mühlhausen** trug einen Punkt in Prichsenstadt, rund
150 km entfernt, ebenfalls zurückgezogen.

**Gesetzt (OSM-Hausnummernpunkt plus Verzeichnis- oder Betreiberangabe)**: Drebkau
(Node 12219086461, name „Mobiler Schrottankauf", identische Telefon- und Webdaten),
Potsdam (way 177628681), Ehningen (way 1125133466), Uslar (way 494920999),
Wilhelmshaven (way 171821766), Dormagen (node 14291083277), Jüterbog (way 254612374),
Ilmenau (way 177687248).

**Geokodierbedarf für den Owner** (bewusst nicht selbst geocodiert):
Kanalstraße 20-24, 16727 Velten (Hausnummernpunkte 20 und 21/22 vorhanden, Zuordnung
des Betriebs offen); Auggener Weg 67, 79114 Freiburg (way 288222636); Rohrlachstraße 34,
68239 Mannheim-Seckenheim sowie Auf dem Kegel 16 und Aufeldstraße 30, 68199 Mannheim
(drei konkurrierende Anschriften); Klippeneckstraße 62, 78056 Villingen-Schwenningen
(way 583234670); Denickestraße, 29225 Celle (nur Straßenkörper, keine Hausnummer);
An der Innerste-Au 2, 31139 Hildesheim (node 5835882655, Zuordnung unbelegt);
Söllmnitz 6b, 07554 Gera; Hermann-Nohl-Straße 12, 33330 Gütersloh (way 276237572);
Juliusstraße 28, 44145 Dortmund und Gottscheestraße 9, 44339 Dortmund;
Friedrich-Engels-Straße 2, 99974 Mühlhausen (nur Straßenkörper way 260082591).

## Offene Owner-Punkte

1. **Aktivitätsnachweis** `bb-drebkau-ot-schorbus-...`: Das Impressum trägt keine
   Registernummer (Einzelunternehmer) und die Website steht inhaltlich auf 2012; die
   README-Ausnahme ist formal nicht erfüllt. Ein Gewerberegister- oder
   Reisegewerbenachweis und ein Telefoncheck zur Aktualität fehlen.
2. **Firmenform** `bb-velten-kraatz-...`: Der Registerabzug nennt „GbR", die
   Registerdaten seit 11.08.2025 „eGbR" (GsR 310 Neuruppin); der Name wurde
   ohne Rechtsformzusatz gesetzt.
3. **Statusentscheidung** `bw-ehningen-kurt-rie-nachfolger`: Zertifikat und
   zertifizierter Umfang sind amtlich belegt, Betreiberkontakt und öffentliche
   Kundenannahme fehlen; Status blieb `pruefung`.
4. **Statusentscheidung** `bb-juterbog-bottcher-...`: Die Beleglage ist eine
   Aggregatorkette, der Aktivstatus ist daraus nicht ableitbar (bereits am 04.10.2026
   als Owner-Punkt offengelegt).
5. **Widerspruch** `nw-dormagen-dormagen-moo`: Die Verzeichnisbefüllung vom 02.10.2026
   widerspricht dem dreifachen Negativbefund vom 30.09./01.10.2026; die zitierte
   11880-Detailseite existiert nicht mehr. Halten oder Entzug der Felder entscheidet
   der Owner.
6. **Typfrage** `bw-freiburg-leonardo-reinhardt-...`: Die Betreiber-Primärquelle weist
   einen Antiquitäten- und Edelmetallankauf aus, keinen Schrotthandel. `trader_type`
   bleibt `schrotthaendler`, weil der Verzeichniseintrag so heißt; die Zipfelmail
   leobaronne777@gmail.com passt nicht zum Nachnamen und ist zu prüfen.
7. **Liquidationslage** `nw-dortmund-walter-leidag`: HRB 5378 ist seit 19.06.2023 in
   Liquidation (Liquidatorin Rosemarie Leidag seit 24.04.2024, letzter Eintrag
   29.01.2025); die Anschriftenkette Juliusstraße 28 (bis 2023) gegen Gottscheestraße 9
   (seit 19.05.2023) ist ohne Originaldokument nicht auflösbar, der Status bleibt
   `pruefung`.
8. **Spiegeldefekt** `nw-gutersloh-gmg`: Für Gütersloh existiert keine Ortsseite
   (defekter Mirror-Link); die Identität „GMG" bleibt offen.
9. **Zertifikatsnummer** `th-muhlhausen-scholz-recycling-standort-muhlhausen`: Der
   Registereintrag Friedrich-Engels-Straße 2, Mühlhausen, hat eine **erreichbare** Detailseite
   (`https://schrottregister.pages.dev/betrieb-muhlhausen-muhlhausen`, live abgerufen am
   28.10.2026); sie nennt den **Hauptsitz Scholz Recycling GmbH** und die **Zertifikatsnummer
   ZZEE00300269** (Tätigkeiten Lagern und Behandeln, Abfallschlüssel 160112, 160116, 160801,
   160803). Die Gleichsetzung mit dem Scholz-Standort ist damit **amtlich belegt** und das
   Zertifikat wird gesetzt. **Nachtrag des Owners:** Die Aussage dieser Welle, die Detailseite
   liefere HTTP 404, war ein **Fehler** und ist hiermit berichtigt.
10. **Merge-Kandidaten** (nicht bearbeitet): `bb-potsdam-vhz-...` teilt das
    Industriegebiet Zum Heizwerk mit `bb-potsdam-kulisch-co-...` und
    `bb-potsdam-richter-recycling`; `th-muhlhausen-scholz-...` und
    `th-gera-scholz-recycling-standort-gera` gehören derselben Kette an.

## Geprüfte Quellen dieser Welle

`mobschrott.de` (Start, Impressum, Kontakt, Leistungen, Schrottpreise),
`reinhardt-antik.de` (Start, Impressum), `dero-reinhardt.de`
(Impressum), `renz-rohstoffe.de` (Start, Impressum), `renzcontainer.de`,
`renz-kanal.de`, `scholz-recycling.com` (Standort, Standorte, Impressum, Sitemap),
`schrottregister.pages.dev` (Ortsseiten und Betriebsseiten für Velten, Potsdam,
Mühlhausen, Uslar, Gera, Ilmenau, Hildesheim, Mannheim, Wilhelmshaven, Dormagen,
Hagen, Celle, Villingen-Schwenningen, Gütersloh, Ehningen),
`ubb.de` (Umweltbüro Berlin-Brandenburg), `online-handelsregister.de`,
`northdata.de`, `registercheck.de`, `creditsafe.com`, `handelsregister.ai`,
`aik.ilm-kreis.de`, `iuwd-gmbh.de`, `tbw-whv.de`, `wilhelmshaven.de`
(Straßenverzeichnis), `nominatim.openstreetmap.org` und
`api.openstreetmap.org` (read-only), `gelbeseiten.de`, `11880.com`,
`dasoertliche.de`, `schrottplatz.org`, `schrottplatz-info.de`, `lokaleschrottplatz.de`,
`cylex.de`, `golocal.de`, `oeffnungszeitenbuch.de`, `branchenbuch.meinestadt.de`,
`stadtbranchenbuch.de`, `branchenbuchdeutschland.de`, `inteka.de`,
`schrottradar.de`, `schrottfinder.de`, `enfmetal.com`, `presseportal.de`,
`uslar.de`.
