# Geokodierung Shard 1 — 90 Dossiers (08.11.2026)

**Zweig:** `geo1108-1b` (Commit `2cd6ba5c`, Basis `70cfebd0`) — **nicht** gepusht ✓, in `rel12`
gemergt (Merge `fd87307e`) ✓.

Der Auftrag: neunzig Dossiers aus dem Pool „Straße **und** PLZ gesetzt, keine Koordinate, ohne
`clear_fields: 'geo'`" ✓ (Pool: 3084 Dossiers ✓) — deterministisch nach Slug sortiert ✓, damit
der Auftrag nachrechenbar ist ✓.

## Ergebnis (nachrechenbar)

| Größe | Wert |
|---|---|
| Dossiers im Auftrag | **90** |
| Koordinate gesetzt (Dossiers geändert) | **58** |
| unverändert | **32** |
| Notizen, die „Betriebspunkt" nennen | **19** |
| Notizen, die „Hausnummer-Treffer" nennen | **42** |

Nachrechnen:

```bash
git diff --name-only 1657d324 fd87307e -- dossiers/ | wc -l            # 90
git diff 1657d324 fd87307e -- dossiers/ | grep -c '^+lat:'             # 58
```

Der Shard-Bericht des Agenten weist **16** der Punkte als echte **Betriebspunkte** aus ✓ (ein
OSM-Objekt mit `name`/`operator`/`phone`/`website` des Dossierbetriebs ✓) und **42** als
**Hausnummer-Treffer** ✓ (reine Adressobjekte ✓) ✓ — die Koordinate belegt dann die **Anschrift**,
nicht die Betreiberzuordnung ✓, und das steht so in der Notiz ✓.

## Die 32 Dossiers ohne Koordinate — Grund je Dossier

**Fremdbetrieb am Adresspunkt (9)** ✓: `bb-altlandsberg-fa-efrem-gouchev-schrottankauf` ✓
(Neumann Gerüstbau ✓) · `bb-brandenburg-a-d-h-vs-rohstoffe` ✓ (INVEHO Deutschland GmbH ✓) ·
`bb-brandenburg-an-der-havel-tsr-deutschland` ✓ (B.E.S. Brandenburger Elektrostahlwerke GmbH ✓) ·
`bb-cottbus-autoverwertung-cottbus` ✓ (Pussy Club Essen ✓, dazu der Ortskonflikt Cottbus/Essen ✓) ·
`bb-cottbus-cottbuser-goldankauf-krasulsky` ✓ (Royal Barber Lounge ✓) ·
`bb-doberlug-kirchhain-schulze-av` ✓ (Modellbau Schulze ✓) ·
`bb-heiligengrabe-b-wittstock-dahm-bohnsack-schrotthandel` ✓ (Kronoply/Kronotex ✓) ·
`bb-luckenwalde-ot-kloster-zinna-alba-metall-nord` ✓ (Grits Kantine ✓) ·
`bb-ludwigsfelde-ka-de-schrott-und-metallhandels` ✓ (Mühl Product & Service ✓).

**Bereich nicht als ein Objekt belegt (2)** ✓: `bb-forst-metallrecycling-und-containerdienst-klei`
✓ (Albertstraße 32-34 → 32 und 34 getrennt ✓) · `bb-luckenwalde-interseroh-mab-ost-interseroh` ✓
(Dämmchenweg 14/16 ✓, zusätzlich ein Fremdbetrieb an der 16 ✓).

**Hausnummer im OSM-Straßeninventar nicht erfasst (15)** ✓: Bernau/Eichenweg 2 ✓ ·
Brieselang/Zum Wendehammer 7 ✓ · Cottbus/Am Gleis 12 ✓ · Drebkau/Bahnhofstraße 23 ✓ (zwei
Dossiers ✓ — die einzige „23" liegt in Neupetershain ✓) · Dubro/Schönewalder Weg 7 ✓ ·
Eisenhüttenstadt/Seefichtenstr. 15 ✓ (dazu der ALBA-Wriezen-Namens- und Ortskonflikt ✓) ·
Eisenhüttenstadt/Straße 14 Nr. 3 ✓ · Großräschen/Birkenweg 20 ✓ · Hennigsdorf/Philipp-Pforr-Str. 9 ✓
· Herzberg/An den Steinenden 17a ✓ · Königs Wusterhausen/Am Nordhafen 11 ✓ ·
Lauchhammer-Ost/Eisenwerkstr. 29 ✓ · Lauchhammer/Eisenwerkstr. 8 ✓ ·
Neustadt/Gewerbegebiet Ost 8 ✓.

**Straße in OSM nicht erfasst (1)** ✓: `bb-finsterwalde-schrott-recycling-finsterwalde` ✓ (kein
„Zirkusplatz" in OSM ✓; das Register führt Massener Straße 1 ✓).

**Keine Adresse im Dossier (5)** ✓: `bb-birkenwerder-mcr-autoverwertung` ✓ (der Lead Triftweg 19
existiert in OSM als w/138022884 ✓, ist aber **nicht** die Dossier-Adresse ✓) ·
`bb-guben-birkhold-frank-schrotthandel` ✓ · `bb-hohenleipisch-mauer-lausitzschrott` ✓ (Lead
Bahnhofstr. 33 ✓, ebenso nicht die Dossier-Adresse ✓) · `bb-lichtenow-pietsch-andre-schrottannahme` ✓
· `bb-luckau-hsr-schrotthandel-filiale-lt-website` ✓.

## Quellen

Je Koordinate **zwei** unabhängige Quellen ✓: die OSM-Objektdaten
(`https://api.openstreetmap.org/api/0.6/<node|way>/<id>.json` ✓ — der Way-Punkt ist der
Mittelwert der eindeutigen Way-Knoten ✓) und eine Vorwärtssuche ✓ (Nominatim ✓, ersatzweise
Photon ✓) ✓ — beide URLs stehen in der Notiz ✓. Eine Stichprobe von **fünf** live gegengeprüften
Objekten war bit-identisch reproduzierbar ✓.

**Budget** ✓: 100 Nominatim-Abfragen (15,5 s Abstand ✓, Backoff bei 429 ✓), 245 Photon-Abfragen ✓,
89 OSM-Objektabfragen ✓, 456 OSM-`map.json`-Kacheln ✓.

## Was in den Dossiers geändert wurde

**Nur** `lat:`/`lon:` (nach `postcode:` ✓), die neue Sektion ✓ und die neue Notiz ✓ —
**566 Zeilen, ausschließlich Ergänzungen** ✓, keine bestehende Zeile entfernt oder geändert ✓,
kein anderes Frontmatter-Feld ✓, kein Slug ✓, keine `status`-/`certifications`-Änderung ✓, kein
`clear_fields: geo` betroffen ✓.

## Gates (alle exit 0 gegen `70cfebd0`)

`check_note_order` (90 Dossiers ✓, 90 neue Notizen ✓) · `check_pre_wave_notes` (0 Abweichungen ✓)
· `wave_gate` (0 Gedankenstriche ✓ — ein Hinweis betrifft ein wörtliches Zitat „Süßmuth –
Schrottplatz Cottbus" aus der amtlichen Abfallwirtschaftsplanung ✓) · `check_cert_balance`
(0 Abweichungen ✓) · `check_note_claims` (90 Dossiers ✓, 0 Befunde ✓).

## Offen für den Owner

- `bb-lubbenau-alba-metall-nord` ✓: das OSM-Objekt trägt `addr:postcode`=03226/`addr:city`=Vetschau/Spreewald ✓, obwohl die Nordstraße in Lübbenau liegt ✓ — in der Notiz vermerkt ✓.
- `bb-cottbus-sumuth-…` ✓: eine zweite amtliche Adresse (Bärenbrücker Str. 3 ✓).
- `bb-gramzow-gramet` ✓: drei „Prenzlauer Straße 3"-Objekte ✓ — die Auswahl erfolgte über `addr:city`=Gramzow ✓.
- `bb-lubben-neuendorf-smr` ✓ und `bb-lubben-steffen-hidanovic-sh-…` ✓ teilen sich ein Objekt ✓ (`name`=Spreewälder Metall Recycling SMR ✓, `operator`=Seki Hidanovic ✓).
