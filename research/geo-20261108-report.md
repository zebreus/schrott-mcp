# Geokodierung — zwei Shards (08.11.2026)

**Zweige:** `geo1108-2` (Commit `43a8abeb`) und `geo1108-3` (Commit `1fa81263`), beide auf der
Basis `eadaf8c6` — gemergt in `rel12` (Merge `81ef3804` und `4ab7c7a8`), anschließend in die
Hausform gebracht (Merge `86f8a267` und `2c4f23d6`).

Ziel war, Dossiers ohne Koordinate eine **belegte** Geokoordinate zu geben: jede gesetzte
Koordinate muss einem echten OSM-Objekt mit passender Hausnummer und Straße entsprechen, und
jede Koordinate wird mit **zwei** Quellen belegt (OSM-Objektdaten über die OSM-API und eine
Suche — Nominatim, ersatzweise Photon).

## Ergebnis (nachrechenbar)

| Größe | Shard 2 | Shard 3 |
|---|---|---|
| Dossiers im Auftrag | **89** | **87** |
| Koordinate gesetzt (Dossiers geändert) | **77** | **71** |
| unverändert (kein Hausnummer-Treffer oder `clear_fields: 'geo'`) | **12** | **16** |
| Notizen, die einen **Betriebspunkt** nennen | **24** | **14** |
| Notizen, die einen **Fremdbetrieb** an der Adresse nennen | **20** | **25** |

Nachrechnen:

```bash
git diff --name-only eadaf8c6 geo1108-2 -- dossiers/ | wc -l    # 77
git diff --name-only eadaf8c6 geo1108-3 -- dossiers/ | wc -l    # 71
git diff eadaf8c6 geo1108-2 -- dossiers/ | grep -c '^+lat:'      # 77
git diff eadaf8c6 geo1108-3 -- dossiers/ | grep -c '^+lat:'      # 71
```

Die **Punktgüte** steht in jeder Notiz:

- **Betriebspunkt** — das OSM-Objekt trägt `name`/`operator`/`phone`/`website` des
  Dossierbetriebs; die Koordinate belegt den **Betrieb**.
- **Hausnummer-Treffer** — reines Adressobjekt ohne Betriebsbezug; die Koordinate belegt die
  **Anschrift**, nicht die Betreiberzuordnung. Das steht so in der Notiz.
- **Fremdbetrieb am Adresspunkt** — an der gefundenen Hausnummer sitzt ein anderer Betrieb
  (namentlich genannt); die Zuordnung bleibt offen.

## Shard 3 — die sechzehn Dossiers ohne Koordinate

Zwölf **ohne Hausnummer-Treffer** (Straßeninventar aus OSM-API/Overpass belegt):
`bw-bosingen-bihl` (Daimlerstraße: 1, 3, 4, 7 — keine 6) · `bw-konigsbach-stein-frey`
(Brettener Straße: 1…77 — keine 100) · `bw-lauchringen-smv-wiederkehr-recycling`
(Detzelner Straße: 3, 4 — keine 1; das Betriebsobjekt hängt an der **3**) ·
`bw-mannheim-ah` (1, 3, 8, 14, 15, 29 — keine 13) · `bw-mannheim-gunter-prag` (keine 7) ·
`bw-monchweiler-hezel` (keine 33) · `nw-arnsberg-ritschny` (27, **29a**, 31 — keine 29) ·
`nw-herdecke-eas-recycling-solution` (12 = „H&F Classic Cars", **keine 12A**) ·
`rp-bitburg-adams-entsorgung` (Alte Röhler Straße: 0 Adressobjekte) ·
`rp-schonenberg-kubelberg-m-r-…` (Bahnhofstraße endet bei 65 — keine 66) ·
`st-klostermansfeld-dave-s` (16a/16c/16e — **keine 16 d**) · `st-konnern-schrottys-…`
(Karl-Marx-Straße: 0 Adressobjekte).

Vier **mit `clear_fields: 'geo'`** — die Owner-Direktive nimmt das GEO-Paar zurück; eine
Koordinate zu setzen widerspräche der Direktive und bricht den Build:
`nw-dusseldorf-reisholz-velbo` · `nw-essen-firma-la-rohrproduktenhandel` ·
`nw-essen-seliger` · `rp-germersheim-alba-metall-sud-germersheim`.

## Shard 2 — die zwölf Dossiers ohne Koordinate

Nach den Reason-Klassen des Shard-Berichts: **Hausnummer im OSM-Straßeninventar nicht
vorhanden** oder **`clear_fields: 'geo'`** durch die Owner-Direktive. Die
Dossier-einzelnen Begründungen stehen im Shard-Bericht des Rechercheagenten; sie liegen
**nicht** im Repository — das ist eine **offene Dokumentationslücke** und als Aufgabe für
den nächsten Durchgang notiert (Auftrag: `research/auftrag-veraltete-current-preise.md`
liegt vor; die Lücke selbst ist in `research/geo-20261108-report.md` vermerkt).

## Betriebspunkte mit Namen (Shard 3, 14 Stück)

`Kiro Schrotthandel` · `Herfel Autoverwertung` · `Schwarz Rohstoff GmbH` · `Willi Lutz GmbH` ·
`Schuler Rohstoff GmbH` · `Autoverwertung Baumgarten & Sohn` · `OSR Metallrecycling GmbH & Co. KG` ·
`STEINEL RECYCLING GmbH + Co KG` · `Weber Containerdienste und Entsorgungssysteme` ·
`eR erft RECYCLING GmbH` · `EMC Edel Metall Contor` · `Spedition Baumann` ·
`Accumulatorenwerke HOPPECKE` · `Bürck Kraftwagenspedition GmbH & Co. KG`.

## Fremdbetriebe am Adresspunkt (Shard 3, vier Stück)

`Kleinmann Automobile` (bw-pforzheim-schirdewan, Hausnummer 44 ∈ 43-45) · `Rolandsbogen`
(rp-bad-kreuznach-loritz-sohn) · `Otto Simon e.K.` (rp-idar-oberstein-55743-edelmetallkontor) ·
`Autoverwertung Gland` (st-magdeburg-recyclingservice; dort tragen fünf Objekte dieselbe
Hausnummer 5 — alle sind in der Notiz genannt).

## Hausform

Die Geo-Notizen tragen den **eigenen** Sektionstyp der Hausform:

```
### Geokodierung 08.11.2026

- [Geokodierung 08.11.2026: **Koordinate belegt: `lat`/`lon` = …** …]
  Quelle(n): https://api.openstreetmap.org/api/0.6/way/… ; https://nominatim.openstreetmap.org/search?… (Abruf 08.11.2026).]
```

Der Sektionstyp ist im Gate `check_note_order.py` ausdrücklich vorgesehen: eine
Geokodierungs-Sektion zählt **nicht** als Wellen-Sektion und steht **nach** der Wellen-Sektion.
Zwei Dossiers tragen beide Sektionstypen (`nw-dusseldorf-theiner-wir-sind-die-entsorger`,
`rp-westheim-hms-westheim`).

## Grenzen des Auftrags

- **Fünf Dossiers mit vier Adressobjekten** tragen jetzt eine Koordinate, die nur die
  **Anschrift** belegt — der Betriebsbezug bleibt offen und ist so benannt.
- Für **28 Dossiers** (12 + 16) wurde **keine** Koordinate gesetzt; dort ist die Adresse nicht
  belegbar. Die Dossiers tragen in diesem Fall **keine** neue Notiz — der Negativbefund steht
  im Shard-Bericht.
- **Nominatim** sperrt die IP nach einem Burst (HTTP 429, Varnish „Error 54113"). Die Shards
  arbeiteten mit 15 Sekunden Abstand und Backoff (60/120/300/600 s) weiter; alle Slugs wurden
  abgedeckt.
- **Keine erfundenen Koordinaten**: jeder gesetzte Punkt ist ein OSM-Objekt mit exakt
  passender Hausnummer und Straße; Bereichsnummern nur, wenn die Folge als ein Objekt belegt
  ist, Buchstaben-Suffixe nur mit demselben Suffix („12A" ≠ „12").
