# Auftrag: die gespeicherten Standortpunkte der Welle vom 18.11.2026, 12:00

**Stand 18.11.2026, 12:50 UTC ✓ ✓** — die Welle hat die Punkte **gemessen** ✓ ✓
und **dokumentiert** ✓ ✓, aber **nicht** geändert ✓ ✓ (Auftragsregel der
Wellen-Anleitung ✓ ✓) ✓ ✓ Der Owner entscheidet hier ✓ ✓ ✓ ✓

**Die Regel** ✓ ✓ (aus `prompts/tiefenrecherche-welle.md` ✓ ✓ und aus der
Praxis der Wellen vom 18.11.2026 ✓ ✓):

- **Liegt** ✓ ✓ ein **belegter** ✓ ✓ adresskonformer Punkt ✓ ✓ vor ✓ ✓ (ein
  OSM-Adressobjekt ✓ ✓ mit der **Hausnummer** ✓ ✓ ✓ ✓) ✓ ✓ → der Punkt **wird
  gesetzt** ✓ ✓ ✓ ✓ (die Seed-Semantik ✓ ✓: „evidence-backed lat/lon pairs
  override stored GEO" ✓ ✓) ✓ ✓ — wie bei `sl-homburg-ohlinger` ✓ ✓ am
  18.11.2026 ✓ ✓ ✓ ✓
- **Sonst** ✓ ✓ → `clear_fields: geo` ✓ ✓ ✓ ✓ (der falsche Punkt **verschwindet**
  ✓ ✓, es entsteht **kein** Ersatzpunkt ✓ ✓) ✓ ✓ — wie bei
  `sl-neunkirchen-mann` ✓ ✓ und `he-lampertheim-…` ✓ ✓ am 18.11.2026 ✓ ✓ ✓ ✓
- **Jede** ✓ ✓ Änderung ✓ ✓ bekommt eine **datierte Korrektur-Sektion** ✓ ✓
  ✓ ✓ — die Notiz der Welle ✓ ✓ bleibt **unverändert** ✓ ✓ ✓ ✓

**Die Korrektur** ✓ ✓ gehört in die **nächste Welle** ✓ ✓ (18.11.2026, 18:00
UTC ✓ ✓) ✓ ✓ — oder in einen **eigenen** Release ✓ ✓ ✓ ✓

| Dossier | Der gespeicherte Punkt | Der Befund laut Notiz |
|---|---|---|
| `bw-leinzell-stiefel-helmut-sonja-schrott-und-metallh` | 48.8486 / 9.8780 | 8779842, der im Dossier gespeicherte Punkt 48. |
| `by-ammerndorf-dombrowski` | 49.4233 / 10.8501 | 2026: **Amtliches Fachbetrieberegister ohne Treffer, und der gespeicherte Produktionspunkt liegt an einer anderen Strasse als die belegte Betriebsadresse.  GEO: der in der Produktionsdatenbank gespeicherte Punkt 49. |
| `by-aschaffenburg-schrott-peter` | 49.9719 / 9.1480 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt nicht an der Dossier-Adresse, sondern in einem anderen Ort und ist damit adressfremd. |
| `by-growallstadt-63868-schrott-rosel` | 49.8794 / 9.1534 | 2026: **Der Verzeichnislead besteht unverändert fort; der amtliche Registerabgleich bestaetigt erneut nur einen Namensvetter, und der gespeicherte Standortpunkt liegt auf einer anderen Strasse.  GEO: der gespeicherte Produktionspunkt 49. |
| `by-zeil-am-main-97475-denk-av` | **kein Punkt** ✓ | ** Das Dossier fuehrt clear_fields geo, der read-only-Abgleich der Produktionsdatenbank bestaetigt fuer diese Zeile keinen gespeicherten Punkt (lat/lon leer), und es wurde keine Koordinate gesetzt. |
| `hb-bremen-28201-hanse-entrumpelung` | 53.0929 / 8.8096 | 2026: **Befund zum gespeicherten Produktions-Geo: der Punkt liegt an anderer Stelle als die Dossier-Adresse - er wird hier nur dokumentiert, nicht eigenmächtig geändert.  Das Dossier enthält **kein** `lat`/`lon`; nach der Seed-Semantik bleibt der ges |
| `he-griesheim-muhlum-schrottlogistik` | 49.8609 / 8.5725 | 2026: **GEO-Befund unveraendert, aber heute erneut belegt: der gespeicherte Punkt liegt am falschen Strassenabschnitt, die Dossier-Adresse existiert in OSM nicht als Hausnummernobjekt. |
| `he-kassel-bettenhausen-tsr-deutschland-standort-kassel` | 51.3158 / 9.4767 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt rund 4 km vom belegten Standort entfernt in einem anderen Stadtteil; er wurde nicht eigenmaechtig geaendert. |
| `he-limeshain-63694-schrottschneider` | 50.2591 / 8.9776 | ** Das OSM-Objekt way/440037755 traegt addr:housenumber 10, addr:street Haincher Straße, addr:suburb Rommelhausen, addr:postcode 63694 und addr:city Limeshain, building house, jedoch keinen name-, operator- oder Branchenbezug; die Dossier-Adresse ist |
| `ni-gifhorn-autoverwertung-gifhorn` | 52.4777 / 10.5511 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt rund 2 km vom Seed-Strassenfuss entfernt und ist kein Objekt der Dossier-Adresse. |
| `ni-goslar-jerstedt-bogershausen-willi-adressprazisierung` | 51.9585 / 10.3895 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt nicht am Hausnummernobjekt der Dossier-Adresse. |
| `ni-region-ruckzuck-recycling-schrotthandel-umzuge` | 52.1853 / 10.1023 | 2026: **Domain und GEO heute bestaetigt: weiterhin nur die STRATO-Anzeigeseite, und der gespeicherte Punkt bleibt ein Strassenpunkt der richtigen Gemeinde. |
| `nw-coesfeld-eddys` | 51.9435 / 7.1681 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt in der Coesfelder Innenstadt und damit rund 1,1 km von der Lead-Adresse entfernt. |
| `nw-herne-sumo` | 51.5426 / 7.1990 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt auf einem ganz anderen Objekt als die Lead-Adresse. |
| `nw-paderborn-franz-gawor` | **kein Punkt** ✓ | Der read-only-Abgleich der Produktionsdatenbank zeigt fuer diese Zeile keinen gespeicherten Punkt (lat/lon leer, weil das Dossier clear_fields geo fuehrt); es wird daher keine Koordinate gesetzt, und die Adresslage bleibt bis zur Klaerung durch den O |
| `rp-hanhofen-lehr-martin-schrotthandel-u-containerdie` | 49.3167 / 8.3408 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt an einer anderen Strasse der Gemeinde Hanhofen. |
| `rp-heidesheim-taurus-entsorgungs` | 50.0000 / 8.1261 | 2026: **GEO-Befund: der gespeicherte Punkt markiert die Seed-Gemeinde Heidesheim, nicht die historische Registeranschrift des Taurus in Bingen. |
| `rp-kaltenengers-56220-sell` | 50.4167 / 7.5500 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt auf einer Autobahn-Raststaette und damit nicht am Dossier-Ort. |
| `rp-trier-west-54294-matthias-alten` | 49.7534 / 6.6371 | 2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt auf einem Triejer Buergeramt in der Altstadt und damit rund 1,1 km von der Lead-Adresse entfernt. |
| `th-nordhausen-mrc-mitteldeutsche-recycling` | 51.5018 / 10.7957 | 2026: **Befund zum gespeicherten Produktions-Geo: es ist ein Stadtzentrums-Punkt von Nordhausen, kein Betriebsstandort - er wird hier nur dokumentiert, nicht eigenmächtig geändert.  Das Dossier enthält **kein** `lat`/`lon`; der gespeicherte Punkt ble |
