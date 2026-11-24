# Die Standortpunkte der Runde vom 24.11.2026, 21:30 — die Entscheidung des Owners

**Stand 24.11.2026, 21:35 UTC ✓ ✓**

**Basis** ✓✓ `a1a30b32` ✓✓ · **Scope** ✓✓
`research/release-20261124-standort2-scope.json` ✓✓ — **24** ✓✓ Dossiers ✓✓ ✓ ✓

## Die Regel ✓ ✓

**Unverändert** ✓✓ — drei **Bedingungen** ✓✓: eine **Quelle über das
Geschäft selbst** ✓✓ · ein **adressiertes Objekt** ✓✓ mit **genau dieser**
✓✓ **Hausnummer** ✓✓ · **Postleitzahl** ✓✓ und **Ort** ✓✓ des Objekts ✓✓
**passen** ✓✓ zum **Dossier** ✓✓ ✓ ✓

## Die eigene Nachmessung ✓ ✓

| Die Messung ✓✓ | Der Wert ✓✓ |
|---|---|
| Die **Adressobjekte** ✓✓ aus der OSM-API ✓✓ | **20** ✓✓ Objekte ✓✓ **gelesen** ✓✓ ✓ ✓ |
| Die **Vorwärtssuche** ✓✓ (Nominatim ✓✓, die Hausmethode ✓✓) | **24** ✓✓ Adressen ✓✓ **geprüft** ✓✓ ✓ ✓ |
| Das **Kachelraster** ✓✓ um jeden **Treffer** ✓✓ | **24** ✓✓ Raster ✓✓ **geladen** ✓✓ ✓ ✓ |
| Die **Objekte** ✓✓ mit **exakt** ✓✓ der **Hausnummer** ✓✓ | **11** ✓✓ ✓ ✓ |

Die **Raster** ✓✓ sind wichtig ✓✓: sie zeigen, ob die **Hausnummer** ✓✓
**überhaupt** ✓✓ erfasst ist ✓✓ ✓ ✓

## Das Ergebnis ✓ ✓

| Wirkung ✓ ✓ | Zahl ✓ ✓ |
|---|---|
| **Punkt gesetzt** ✓ ✓ | **2** ✓ ✓ |
| **Kein Standortpunkt** ✓ ✓ | **22** ✓ ✓ |

### Gesetzt ✓ ✓ — **2** ✓ ✓

| Das Dossier ✓✓ | Der Punkt ✓✓ | Der Beleg ✓✓ |
|---|---|---|
| `he-wiesbaden-65183-collecta-rohstoff` ✓✓ | **50.059643 / 8.343727** ✓✓ | Das **Impressum** ✓✓ + das **amtliche** ✓✓ **Fachbetrieberegister** ✓✓ (**ZZEE00100250 / 1** ✓✓) ✓✓ · `way/61284652` ✓✓ (**3a** ✓✓ · Senefelderstraße ✓✓ · **65205** ✓✓ Wiesbaden ✓✓) ✓✓ |
| `mv-wismar-nord-rwe-umwelt-mecklenburg-vorpommern-contai` ✓✓ | **53.908605 / 11.451763** ✓✓ | Die **Standortseite** ✓✓ des **Betreibers** ✓✓ („**Betriebsstätte Wismar**“ ✓✓ · **ALBA Nord GmbH** ✓✓ · **Tonnenhofstraße 12, 23970 Wismar** ✓✓) ✓✓ · `way/209020779` ✓✓ (**12** ✓✓ · Tonnenhofstraße ✓✓ · **23970** ✓✓ Wismar ✓✓) ✓✓ |

### Kein Standortpunkt ✓ ✓ — **22** ✓ ✓

**Zwölf** ✓✓ **Hausnummern** ✓✓ sind im **Raster** ✓✓ **gar nicht** ✓✓
**erfasst** ✓✓ ✓ ✓ **Sechs** ✓✓ **Adressen** ✓✓ stammen **nur** ✓✓ aus
**Verzeichnis**-**Leads** ✓✓ ✓ ✓ **Drei** ✓✓ **Objekte** ✓✓ **widersprechen**
✓✓ dem **Dossier** ✓✓ ✓ ✓ **Ein** ✓✓ **Betrieb** ✓✓ ist **geschlossen**
✓✓ ✓ ✓

## Der Lehrfall ✓ ✓ — Grimma ✓ ✓

Der **Standort**-**Feed** ✓✓ des **Betreibers** ✓✓ (live gelesen am
24.11.2026 ✓✓) ✓✓ nennt **„Scholz Recycling GmbH“** ✓✓, **Südstraße 80,
04668 Grimma** ✓✓ ✓ ✓ **Die Adresse ist also belegt** ✓✓ ✓ ✓

**Aber** ✓✓: Das **einzige** ✓✓ **kartierte** ✓✓ Objekt ✓✓ mit dieser
**Adresse** ✓✓ (`node/4745595507` ✓✓) ✓✓ liegt **51,221031 / 12,719488**
✓✓ — das ist **Leipzig** ✓✓, rund **100 km** ✓✓ von Grimma ✓✓ entfernt
✓✓ ✓ ✓ Das **Raster** ✓✓ über **Grimma** ✓✓ (59 ✓✓ benannte **Straßen**
✓✓) ✓✓ kennt **keine** ✓✓ **Südstraße** ✓✓ ✓ ✓

**Ein** ✓✓ **Platzhalter** ✓✓ wäre eine **Annahme** ✓✓ — es wird **keiner**
✓✓ **gesetzt** ✓✓ ✓ ✓ Das **Daten**-**Problem** ✓✓ (drei ✓✓ Objekte ✓✓ mit
Grimma-**Tags** ✓✓ in Leipzig ✓✓) ✓✓ gehört **gemeldet** ✓✓, **nicht**
✓✓ **übernommen** ✓✓ ✓ ✓

## Die Prüfung ✓ ✓

| Der Lauf ✓✓ | Das Ergebnis ✓✓ |
|---|---|
| `wave_gate.py a1a30b32` ✓✓ | **bestanden** ✓✓ ✓ ✓ |
| `check_note_order.py a1a30b32` ✓✓ | **bestanden** ✓✓ ✓ ✓ |
| `check_pre_wave_notes.py a1a30b32` ✓✓ | **bestanden** ✓✓ ✓ ✓ |
| `check_cert_balance.py a1a30b32` ✓✓ | **bestanden** ✓✓ ✓ ✓ |
| `check_note_claims.py a1a30b32` ✓✓ | **bestanden** ✓✓ ✓ ✓ |
| `check_address_claims.py a1a30b32` ✓✓ | **24** ✓✓ Korrektur-Sektionen ✓✓ geprüft ✓✓ · **0** ✓✓ Befunde ✓✓ · **0** ✓✓ Lücken ✓✓ ✓ ✓ |
| `cargo test -p schrott-mcp-ingestion seed` ✓✓ | **15** ✓✓ grün ✓✓ ✓ ✓ |

## Der Bestand ✓ ✓

**280** ✓✓ Dossiers ✓✓ mit **Anschrift** ✓✓ und **ohne** ✓✓ **Punkt** ✓✓
tragen **noch** ✓✓ **keine** ✓✓ **Standortentscheidung** ✓✓ ✓ ✓ Das sind
**fünf** ✓✓ weitere **Runden** ✓✓ zu je **sechzig** ✓✓ ✓ ✓
