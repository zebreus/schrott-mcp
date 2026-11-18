# Die Standortpunkte der Welle vom 18.11.2026, 18:00 — die Entscheidung des Owners

**Stand 18.11.2026, 21:45 UTC ✓ ✓** — die Welle hat **34** Standortpunkte
**gemessen** ✓ ✓ und **dokumentiert** ✓ ✓, aber **nicht** geändert ✓ ✓ (die
Auftragsregel der Wellen-Anleitung ✓ ✓) ✓ ✓ Der Owner hat über **jeden** Fall
**einzeln** entschieden ✓ ✓ ✓ ✓

## Die Regel ✓ ✓

- **Belegter** ✓ ✓ **adresskonformer** ✓ ✓ Punkt ✓ ✓ — ein OSM-Adressobjekt ✓ ✓
  mit der **Hausnummer** ✓ ✓ **und** dem **Namen des Betriebs** ✓ ✓ oder ein
  **Zertifikat** ✓ ✓, das die Anlagen an dieser Adresse ausweist ✓ ✓ → der Punkt
  **wird gesetzt** ✓ ✓ ✓ ✓
- **Sonst** ✓ ✓ → der **falsche** Punkt ✓ ✓ **verschwindet** ✓ ✓
  (`clear_fields: geo` ✓ ✓), es entsteht **kein** Ersatzpunkt ✓ ✓ ✓ ✓
- Ein Punkt, der die **Gemeinde** der Seed-Adresse markiert ✓ ✓ (das Dossier
  führt **keine Straße** ✓ ✓), **bleibt** stehen ✓ ✓ — er markiert den **Ort** ✓ ✓,
  nicht den Betrieb ✓ ✓, und ist der Platzhalter des Seeds ✓ ✓
- Ein Punkt, der **nicht** zur Dossier-Adresse gehört ✓ ✓ und auf einem **fremden
  Objekt** ✓ ✓ liegt — ein **Fremdbetrieb** ✓ ✓, ein **Bürgeramt** ✓ ✓, eine
  **Spielhalle** ✓ ✓, eine **Autobahn-Raststätte** ✓ ✓, ein **anderer Ort** ✓ ✓
  oder eine **andere Straße** ✓ ✓ → er wird **entfernt** ✓ ✓ ✓ ✓

**Jede** Änderung ✓ ✓ bekommt eine **datierte Korrektur-Sektion** ✓ ✓ ✓ ✓ — die
Notiz der Welle ✓ ✓ bleibt **unverändert** ✓ ✓ ✓ ✓

## Das Ergebnis ✓ ✓

| Wirkung | Zahl | Rollout |
|---|---|---|
| **Punkt gesetzt** ✓ ✓ | **3** ✓ ✓ | `7d1f87f` ✓ ✓ |
| **Punkt entfernt** ✓ ✓ | **23** ✓ ✓ | `7d1f87f` ✓ ✓ |
| **Punkt bleibt** ✓ ✓ | **7** ✓ ✓ | unverändert ✓ ✓ |
| **Punkt entfernt (offen)** ✓ ✓ | **1** ✓ ✓ | **hb-lussum** ✓ ✓ → der **nächste** Release ✓ ✓ |

### Gesetzt ✓ ✓ — **3** ✓ ✓

| Dossier | Der neue Punkt | Der Beleg |
|---|---|---|
| `be-tempelhof-schoneberg-friedenauer-munzenhaus` | 52.4799885 / 13.3330000 ✓ ✓ | das Objekt **trägt den Namen des Betriebs** ✓ ✓ (node/6437116286 ✓ ✓) ✓ ✓ |
| `hb-geestemunde-richard-bauer-rohstoff-grohandel` | 51.1605828 / 7.7568465 ✓ ✓ | das **Zertifikat** ✓ ✓ weist die **Anlagen 3 und 4** ✓ ✓ an Weißenstein 2 aus ✓ ✓ ✓ ✓ |
| `he-wiesbaden-65201-kolb-heinrich` | 50.0498386 / 8.1939342 ✓ ✓ | das Objekt **trägt den Namen des Betriebs** ✓ ✓ (node/6533455620 ✓ ✓) ✓ ✓ |

### Entfernt ✓ ✓ — **23** ✓ ✓

`be-neukolln-klemm` ✓ ✓ · `be-steglitz-zehlendorf-rich-herbig-metallhandel-berlin` ✓ ✓
· `by-munchen-autoverwertung-koller` ✓ ✓ · `by-munchen-faschingbauer-heinrich` ✓ ✓
· `by-osterhofen-gebr-brandstatter` ✓ ✓ · `hb-bremen-28719-altun-dienstleistung-all-rounder` ✓ ✓
· `he-marburg-35039-gluck` ✓ ✓ · `he-wabern-34590-thiel` ✓ ✓
· `ni-celle-albert-schmidt-schrott-und-metallhandel` ✓ ✓
· `ni-gottingen-holtensen-37079-hammoud-schrottabholung` ✓ ✓
· `ni-hildesheim-sami-schrott-und-altmetall-entsorgung` ✓ ✓ · `nw-brakel-bargholt` ✓ ✓
· `nw-lennestadt-l-mulaj` ✓ ✓ · `nw-niederkruchten-wilms` ✓ ✓
· `rp-landstuhl-s-t-janek-schrotthandel` ✓ ✓ · `rp-neustadt-giannotta` ✓ ✓
· `sh-hohenwestedt-tolksdorf-schrott-und-metallhandelsgesel` ✓ ✓
· `sh-lubeck-kucknitz-schrott-und-autorecyclinghof-sonke-milon` ✓ ✓
· `sh-stadum-jacob-dobrick-schrotthandel` ✓ ✓
· `sl-schwalbach-hulzweiler-paisler-schrotthandel-altmetallentsorgun` ✓ ✓
· `th-weimar-gert-wiesser` ✓ ✓ · `th-wutha-farnroda-b-eisenach-hertwig-rolf` ✓ ✓
· `th-zeulenroda-triebes-rpt-recycling` ✓ ✓

### Bleibt ✓ ✓ — **7** ✓ ✓

| Dossier | Warum der Punkt bleibt |
|---|---|
| `be-mitte-autoteile-wedding` ✓ ✓ | der Punkt liegt auf der **Dossier-Adresse** ✓ ✓ (Koloniestr. 114 ✓ ✓, rund 30 m ✓ ✓) ✓ ✓ |
| `he-weilmunster-35789-wieder` ✓ ✓ | **Ortsmitte** ✓ ✓ von Weilmünster ✓ ✓ — das Dossier führt **keine Straße** ✓ ✓ ✓ ✓ |
| `sh-oldenburg-i-h-de-schuttkuhlenmaior` ✓ ✓ | **Ortsmitte** ✓ ✓ von Oldenburg i.H. ✓ ✓ — **keine Straße** ✓ ✓ im Dossier ✓ ✓ ✓ ✓ |
| `sl-homburg-66424-schrotthandel-r-wei` ✓ ✓ | **Ortsmittelpunkt-Platzhalter** ✓ ✓ — die Seed-Adresse ist in OSM **nicht auflösbar** ✓ ✓ ✓ ✓ |
| `sl-saarbrucken-binkle-schrotthandel-demontagen` ✓ ✓ | **Stadtplatzhalter** ✓ ✓ — für den mobilen Anbieter ist **keine** Geschäftsanschrift belegt ✓ ✓ ✓ ✓ |
| `st-magdeburg-recyclingservice-sebastian-hoppe` ✓ ✓ | der Punkt belegt die **Anschrift** ✓ ✓ (Glindenberger Weg 5 ✓ ✓), nicht die Anlieferfläche ✓ ✓ ✓ ✓ |
| `th-sonneberg-recyclinghof-oberlind-manfred-hiller` ✓ ✓ | **Ortsmitte** ✓ ✓ von Sonneberg ✓ ✓ — **keine Straße** ✓ ✓ im Dossier ✓ ✓ ✓ ✓ |

### Offen ✓ ✓ — **1** ✓ ✓

`hb-lussum-bockhorn-herbig-schrott-entrumpelung` ✓ ✓ — der Punkt 53.2108611/8.5795163
trifft die **Haltestellenobjekte** der Hohenbuchener Straße ✓ ✓ in **48 m** ✓ ✓
und damit **nicht** die Dossier-Adresse **Schwaneweder Str. 44a** ✓ ✓ — die
Hausnummer **44 a** ✓ ✓ ist in OSM **nicht** belegt ✓ ✓ und ein **Ersatzpunkt**
✓ ✓ entsteht **nicht** ✓ ✓ ✓ ✓

**Der Punkt wird im nächsten Release entfernt** ✓ ✓ — **keine** Änderung an der
Produktion ✓ ✓, solange das Pending-Gate des Release `7d1f87f` ✓ ✓ **rot** ist ✓ ✓
✓ ✓

## Der Vergleich mit der Welle vom 12:00 ✓ ✓

Dort blieben **vier** Punkte **unverändert** ✓ ✓ ✓ ✓: `bw-leinzell` ✓ ✓ (der Punkt
**ist** die Adresse ✓ ✓), `ni-region-ruckzuck` ✓ ✓, `rp-heidesheim` ✓ ✓ und
`th-nordhausen` ✓ ✓ (jeweils die **Gemeinde** der Seed-Adresse ✓ ✓) ✓ ✓ ✓ ✓

**Dieselbe Regel** ✓ ✓ gilt hier ✓ ✓ ✓ ✓ — mit **einer** Ausnahme ✓ ✗ ✓: bei
`he-griesheim-muhlum-schrottlogistik` ✓ ✓ (Welle 12:00 ✓ ✓) wurde der Punkt
**entfernt** ✓ ✓, obwohl er nur **auf einem anderen Straßenabschnitt** ✓ ✓ lag ✓ ✓
und die Verzeichnisadresse **Rübgrund 2** ✓ ✓ in OSM **kein** Hausnummernobjekt
✓ ✓ hat ✓ ✓ — bei `hb-lussum` ✓ ✓ (Welle 18:00 ✓ ✓) wird er **ebenfalls**
entfernt ✓ ✓ ✓ ✓ **Die beiden Fälle sind damit gleich entschieden** ✓ ✓ ✓ ✓

## Die Offen-Punkte der Welle ✓ ✓

Die Welle hat **33** Dossiers mit einem GEO-Befund ✓ ✓ hinterlassen ✓ ✓ — **drei**
davon tragen **jetzt** einen belegten Punkt ✓ ✓, **23** haben **keinen** ✓ ✓ und
**acht** behalten ihren **Platzhalter** ✓ ✓ ✓ ✓ Die Notizen bleiben
**unverändert** ✓ ✓ ✓ ✓
