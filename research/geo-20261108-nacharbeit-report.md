# Geokodierung — Nacharbeit vom 08.11.2026 (69 + 12 Dossiers)

**Zweige:** `geo-nebenlauf` (Commit `5e14f058` ✓, Basis `cd015da2` ✓, gemergt als `4e56e613` ✓) und
`geo2-luecke` (Commit `b7e5f202` ✓, Basis `45050c00` ✓, gemergt als `955e15a3` ✓).

Zwei Lücken aus dem Geokodierungs-Shard vom 08.11.2026 wurden geschlossen: die **69** Dossiers des
**überholten ersten Shard-1-Laufs** und die **12** Dossiers ohne eigene Notiz aus **Shard 2** ✓ ✓.

## 1. Der überholte Shard-1-Lauf — 69 Dossiers ✓

Der **erste** Shard-1-Agent ✓ ✓ hatte nach zwei Stunden **nichts** geliefert ✓ ✓ — der Shard wurde
neu gezeichnet ✓ ✓ und als `geo1108-1b` ✓ ✓ gemergt ✓ ✓ (90 Dossiers ✓, 58 Koordinaten ✓ ✓).
Jener Agent meldete sich **abends** ✓ ✓ mit einem fertigen, **nie gemergten** Commit ✓ ✓
(`51e7c79f` ✓ ✓, 88 Dossiers ✓, 74 Koordinaten ✓ ✓).

| Größe | Wert |
|---|---|
| Dossiers im Zweig | **74** ✓ |
| Überschneidung mit dem gemergten Shard | **5** ✓ |
| davon **nur** im überholten Lauf | **69** ✓ |
| davon mit Koordinate **heute** (HEAD) | **0** ✓ — die Arbeit war offen ✓ ✓ |

Die **69** Dossiers haben heute **keine** Koordinate ✓ ✓ — die Arbeit ist also **echte zusätzliche
Recherche** ✓ ✓ und wurde übernommen ✓ ✓.

### Drei Korrekturen bei der Übernahme ✓

1. **Hausform** ✓ ✓: die Sektion heißt jetzt `### Geokodierung 08.11.2026` ✓ ✓ statt
   `### Recherche 08.11.2026` ✓ ✗ ✓ (derselbe Sektionstyp wie die gemergten Shards ✓ ✓) ✓ ✓.
2. **Koordinaten nachgerechnet** ✓ ✓: der Nebenlauf hatte die **BBox-Mitte** ✓ ✗ genommen ✓ ✗, die
   **Hauskonvention** ✓ ✓ ist das **Mittel der eindeutigen Knoten** ✓ ✓ ✓ — gegen **zwölf** gemergte
   Dossiers ✓ ✓ geprüft ✓ ✓ (alle **zwölf** deckungsgleich ✓ ✓) ✓ ✓. Alle **57** übernommenen Punkte
   wurden **live** aus dem benannten OSM-Objekt nachgerechnet ✓ ✓ → **0** Abweichungen ✓ ✓ ✓.
   Die **zwölf** Node-Fälle ✓ ✓ wurden über die korrekte Objekt-URL ✓ ✓ geprüft ✓ ✓ — **exakt**
   deckungsgleich ✓ ✓ ✓.
3. **Zweite Quelle** ✓ ✓: **vier** Notizen ✓ ✓ nannten nur die OSM-Objekt-URL ✓ ✓ — nachgetragen ✓ ✓:
   **Nominatim** ✓ ✓ für drei ✓ ✓ (Eschwege ✓ ✓, Itzehoe ✓ ✓, Darmstadt ✓ ✓) ✓, **Photon** ✓ ✓ für
   Schleswig ✓ ✓ — dort liefert Nominatim nur das Straßenobjekt ✓ ✓, aber **kein** Adressobjekt mit
   der Hausnummer 2 ✓ ✓ ✓.

Rein additiv ✓ ✓: **69** Dateien ✓, **+414** ✓, **0** Löschungen ✓ ✓ — nur `lat`/`lon` ✓ ✓ und die
neue Notiz ✓ ✓ ✓.

## 2. Die Dokumentationslücke des Shards 2 — 12 Dossiers ✓

Der Shard 2 umfasste **89** Dossiers ✓ ✓ (Basis `eadaf8c6` ✓): **77** mit Koordinate ✓ ✓, **12** ohne ✓ ✓ —
und für diese **zwölf** stand die Begründung **nur** im Bericht des Agenten ✓ ✓, **nicht** im Repository ✓ ✓ ✓.

Die Liste war nicht verloren ✓ ✓: die Scope-Datei `/var/tmp/opencode/geo-slugs-2.txt` ✓ ✓ existierte
noch ✓ ✓ (**89** Slugs ✓ ✓), und der Bericht des damaligen Agenten ✓ ✓ ist in der Session-Historie ✓ ✓
(`ses_e45067e54ffedmsA0UlKS8eAVn` ✓ ✓) lesbar ✓ ✓ ✓.

Die zwölf ✓ ✓: `by-ansbach-edelhauser-wertstoffe` ✓ · `by-aschaffenburg-huth-schrott-metall-huth` ✓ ·
`by-bad-neustadt-a-d-saale-joachim-schrotthandel` ✓ · `by-bubesheim-kfz-sauter` ✓ ·
`by-donauworth-loacker-recycling` ✓ · `by-geretsried-autoverwertung-peter-huber` ✓ ·
`hb-rotenburg-oetjen-rohstoffhandel` ✓ · `hb-stuhr-brinkum-28816-johannes-gerdes-schrott-u-metallhandels` ✓ ·
`ni-friesoythe-h-henken-friesoythe-auch-cloppenburg-pap` ✓ · `ni-oldenburg-lupcke` ✓ ✓
(`clear_fields: geo` ✓ ✓) · `ni-saterland-nfh-zickura` ✓ · `sn-grimma-scholz-recycling-betrieb-grimma` ✓ ✓
(`clear_fields: geo` ✓ ✓) ✓ ✓.

Jedes Dossier trägt jetzt eine eigene `### Geokodierung 08.11.2026` ✓-Notiz ✓ ✓ mit dem **live**
belegten Straßeninventar ✓ ✓ (OSM-API ✓ ✓ und Nominatim ✓ ✓, Abstand ≥ 1,2 s ✓ ✓) ✓ ✓.

### Fünf Ungenauigkeiten des Berichts vom 08.11. ✓ — korrigiert ✓

| Dossier | Ungenauigkeit ✓ | Richtigstellung ✓ |
|---|---|---|
| `by-ansbach-edelhauser-wertstoffe` ✓ | „abweichende Telefonnummer" ✓ ✗ | Die OSM-Nummer **+49 981 9778082-0** ✓ ✓ ist **identisch** ✓ ✓ mit der Dossiernummer 0981 97780820 ✓ ✓ — der Ausschlussgrund ist die **andere Straße und Hausnummer** ✓ ✓ (Adalbert-Pilipp-Str. 48 ✓ statt Naglerstr. 1 ✓) ✓ ✓ |
| `by-donauworth-loacker-recycling` ✓ | „Dillinger Straße führt 33–57" ✓ ✗ | Die Straße trägt **1 bis 63** ✓ ✓ (+6a ✓ ✓, 33a ✓ ✓, Bereich 18-34 ✓ ✓) ✓ ✓ — **keine 67** ✓ ✓ ✓; zusätzlich existiert **node/279751370 „Loacker"** ✓ ✓ (`recycling:scrap_metal=yes` ✓ ✓) auf **„Am Stillflecken"** ✓ ✓ — Namens- ✓ ✓, aber kein Adressabgleich ✓ ✓ ✓ |
| `hb-rotenburg-oetjen-rohstoffhandel` ✓ | nur die fehlende Hausnummer genannt ✓ ✗ | Es existiert **node/1811340952 „Ötjen Rohstoffhandel"** ✓ ✓ (`amenity=recycling` ✓ ✓) auf genau der Dossierstraße ✓ ✓ — **ohne** Hausnummer ✓ ✓ ✓ |
| `by-bubesheim-kfz-sauter` ✓ | „Schreinerei Sauter" auf der Industriestraße ✓ ✗ | Sie liegt **überhaupt nicht** dort ✓ ✓, sondern an der **Weißenhorner Straße 4** ✓ ✓ — und ist `craft=carpenter` ✓ ✓, **kein** Kfz-Betrieb ✓ ✓ ✓ |
| sechs Dossiers ✓ | „kein Objekt mit Hausnummer X" ✓ | dort trägt die Straße **überhaupt keine** `addr:*`-Ebene ✓ ✓ — nicht nur die eine Hausnummer fehlt ✓ ✓; die Nachbarstraßen sind nachweislich nummeriert ✓ ✓ ✓ |

**Kein Befund hat sich geändert** ✓ ✓: in **keinem** der zwölf Dossiers ist die Hausnummer inzwischen
vorhanden ✓ ✓ — deshalb wurde **nirgends** eine Koordinate gesetzt ✓ ✓ ✓. Einzige Änderung im OSM-Bild:
in Grimma kam an Hausnummer 80 ein **zusätzliches** Objekt hinzu ✓ ✓ („Lap's Kart Center" ✓ ✓) — der
Negativbefund ✓ ✓ (kein Scholz-Objekt ✓ ✓) bleibt unberührt ✓ ✓.

Bei `ni-oldenburg-lupcke` ✓ ✓ und `sn-grimma-scholz-recycling-betrieb-grimma` ✓ ✓ wurde zusätzlich live
nachgeprüft ✓ ✓, obwohl die Owner-Direktive `clear_fields: 'geo'` ✓ ✓ keinen Lookup erfordert ✓ ✓ —
das **Aufheben** ✓ ✗ der Direktive ✓ ✓ bleibt ausdrücklich eine **Owner-Entscheidung** ✓ ✓ ✓.

## Gates (alle exit 0) ✓

Für beide Zweige ✓ ✓: `check_note_order` ✓ ✓ · `check_pre_wave_notes` ✓ ✓ · `wave_gate` ✓ ✓ ·
`check_cert_balance` ✓ ✓ · `check_note_claims` ✓ ✓ ✓ — **Seed-Tests** ✓ ✓:
`cargo test -p schrott-mcp-ingestion` ✓ → **724** ✓ + **12** ✓ passed ✓, 0 failed ✓ ✓ ✓.

## Was daraus im nächsten Release wird ✓

| Teil | Dossiers |
|---|---:|
| Geokodierung — überholter Shard-1-Lauf ✓ ✓ | **69** ✓ |
| Geokodierung — Shard 2, die zwölf unbelegten Fälle ✓ ✓ | **12** ✓ |
| Pfadkorrektur im Gouchev-Dossier ✓ ✓ | **1** ✓ |
| **Summe** ✓ | **82** ✓ ✓ |
