# Die Standortpunkte der Wellen vom 19.11.2026 — die Entscheidung des Owners

**Stand 20.11.2026, 01:40 UTC ✓ ✓** — die Wellen vom 19.11.2026, **06:00** ✓ ✓ und
**18:00** ✓ ✓ haben ihre Standortpunkte **gemessen** ✓ ✓ und **dokumentiert** ✓ ✓,
aber **nicht** geändert ✓ ✓ (die Auftragsregel der Wellen-Anleitung ✓ ✓) ✓ ✓ Der
Owner hat über **jeden** Fall **einzeln** entschieden ✓ ✓ ✓ ✓

## Die Regel ✓ ✓

- **Belegt** ✓ ✓ ist eine Adresse **durch eine Quelle über das Geschäft selbst**
  ✓ ✓ — das **Impressum des Betreibers** ✓ ✓, ein **Zertifikat** ✓ ✓, das **amtliche
  Fachbetrieberegister** ✓ ✓ oder ein **OSM-Objekt mit dem Namen des Betriebs** ✓ ✓
  — **und** das **Adressobjekt** ✓ ✓ trägt diese **Hausnummer** ✓ ✓ → der Punkt
  **wird gesetzt** ✓ ✓ ✓ ✓
- **Belegt** ✓ ✓ ist die Adresse **nur durch ein Verzeichnis** ✓ ✓ → ein
  **Adressobjekt ohne** `name` ✓ ✓ **und ohne** `operator` ✓ ✓ ist **kein**
  Standortnachweis ✓ ✓ → der Punkt **wird entfernt** ✓ ✓ ✓ ✓
- **Widerspricht** ✓ ✓ der Punkt einer **dokumentierten** Adresse ✓ ✓ (andere
  Straße ✓ ✓, anderer Ort ✓ ✓, fremdes Objekt ✓ ✓, Bürgeramt ✓ ✓, Spielhalle ✓ ✓,
  Autobahn-Raststätte ✓ ✓, archäologischer Fundort ✓ ✓, Tankstelle ✓ ✓) → er
  wird **entfernt** ✓ ✓ ✓ ✓
- **Trifft** ✓ ✓ der Punkt die **Dossier-Adresse selbst** ✓ ✓ → er **bleibt** ✓ ✓
- **Trifft** ✓ ✓ der Punkt nur die **Gemeinde** ✓ ✓ (das Dossier führt **keine
  Straße** ✓ ✓ oder die Seed-Koordinate ✓ ✓ ist auf die **Minute** ✓ ✓ gerundet
  ✓ ✓) → er **bleibt** ✓ ✓ als Platzhalter ✓ ✓ des Seeds ✓ ✓

**Jede** Änderung ✓ ✓ bekommt eine **datierte Korrektur-Sektion** ✓ ✓ ✓ ✓ — die
Notiz der Welle ✓ ✓ bleibt **unverändert** ✓ ✓ ✓ ✓

## Das Ergebnis ✓ ✓

| Wirkung | Zahl | Wellen |
|---|---|---|
| **Punkt gesetzt** ✓ ✓ | **3** ✓ ✓ | alle drei aus der Welle 18:00 ✓ ✓ |
| **Punkt entfernt** ✓ ✓ | **29** ✓ ✓ | 21 aus der Welle 18:00 ✓ ✓ · 8 aus der Welle 06:00 ✓ ✓ |
| **Punkt bleibt** ✓ ✓ | **26** ✓ ✓ | 14 aus der Welle 18:00 ✓ ✓ · 12 aus der Welle 06:00 ✓ ✓ |

### Gesetzt ✓ ✓ — **3** ✓ ✓

| Dossier | Der neue Punkt | Der Beleg |
|---|---|---|
| `bw-pforzheim-agosi-ag` ✓ ✓ | 48.8922374 / 8.7187427 ✓ ✓ | `node/1155112188` ✓ ✓ trägt die Hausnummer **17** ✓ ✓ und den **Namen des Betriebs** ✓ ✓ (`Allgemeine Gold- und Silberscheideanstalt AG` ✓ ✓) ✓ ✓ |
| `sl-neunkirchen-thirion-ralf-e-metallgrohandlung` ✓ ✓ | 49.3521884 / 7.1848943 ✓ ✓ | das **EfbV-Zertifikat** ✓ ✓ weist die Anlagen an **Haydnstr. 9** ✓ ✓ aus ✓ ✓ (Vorgangsnummer ZZKT003000436009 aus Feld 3.3 ✓ ✓) ✓ ✓ |
| `th-nordhausen-autoverwertung-boesenberg-manuela-sachse` ✓ ✓ | 51.5044115 / 10.8362408 ✓ ✓ | das **amtliche Altfahrzeugregister** ✓ ✓ weist die Anerkennung an **Schwarzer Weg 8** ✓ ✓ aus ✓ ✓ (Vorgangsnummer ZARS00300023 ✓ ✓) ✓ ✓ |

### Entfernt ✓ ✓ — **29** ✓ ✓

**Neun** Punkte liegen auf einem **fremden Objekt** ✓ ✓: eine **Tankstelle in
Lichtenstein (Württemberg)** ✓ ✓ statt in Lichtenstein (Sachsen) ✓ ✓, ein
**archäologischer Fundort** ✓ ✓, eine **Spielhalle** ✓ ✓, ein **Bürgeramt** ✓ ✓,
ein **Theater** ✓ ✓ (Kölner Philharmonie ✓ ✓), ein **Recycling-POI eines fremden
Betreibers** ✓ ✓, ein **Sanitätshaus** ✓ ✓, ein **Straßenobjekt** ✓ ✓ und ein
**Wohnobjekt** ✓ ✓ ✓ ✓

**Zwanzig** widersprechen einer **dokumentierten** Adresse ✓ ✓ — zwischen **90 m**
✓ ✓ (auf einer anderen Straße ✓ ✓) und **150 km** ✓ ✓ (in einem anderen Ort ✓ ✓)
✓ ✓

### Bleibt ✓ ✓ — **26** ✓ ✓

Diese **26** Punkte treffen die **Dossier-Adresse** ✓ ✓ (zwischen **0 m** ✓ ✓ und
**250 m** ✓ ✓) ✓ ✓ oder markieren nur die **Gemeinde** ✓ ✓ — darunter
`by-obertraubling-kaiser-recycling` ✓ ✓ (die Seed-Koordinate 48.9667/12.1667 ist
auf die Minute gerundet ✓ ✓ und trifft Obertraubling ✓ ✓),
`st-klostermansfeld-dave-s` ✓ ✓ (51.5833/11.5 ✓ ✓, gerundet ✓ ✓) und
`sn-mobil-leipzig-schrottjungs-schrottabholung-leipzig` ✓ ✓ (Stadtmitte ✓ ✓) ✓ ✓

**Ein** Sonderfall ✓ ✓: `sn-leipzig-scholz-recycling-…` ✓ ✓ — der Punkt ist der
**Mittelwert** der beiden Finderpunkte ✓ ✓ und löst auf ein **Geschäftslokal** im
Leipziger Zentrum auf ✓ ✓; er gehört **keinem** der beiden Höfe ✓ ✓ und wird
deshalb **entfernt** ✓ ✓ ✓ ✓

## Der Stand in der Produktion ✓ ✓

Die **32** Änderungen ✓ ✓ dieser Entscheidung ✓ ✓ gehen mit dem Release der Welle
vom 19.11.2026, **18:00** ✓ ✓ hinaus ✓ ✓ — **68** Zeilen ✓ ✓: die **60** der
Welle ✓ ✓ und **acht** Standortpunkte ✓ ✓ aus der Welle vom **06:00** ✓ ✓ ✓ ✓

Die **fünfzehn** Korrekturen ✓ ✓ der Welle vom **00:00** ✓ ✓ sind bereits mit dem
Release `e98b793` ✓ ✓ ausgerollt ✓ ✓ ✓ ✓
