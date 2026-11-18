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
| `bw-leinzell-stiefel-helmut-sonja-schrott-und-metallh` | 48.8486 / 9.8780 | sein Mittelpunkt aus 13 eindeutigen Knoten liegt bei 48.8485309/9.8779842, der im Dossier gespeicherte Punkt 48.848579/9.877973 weicht davon um rund 5 m im Breitengrad und rund 1 m im Längengrad ab - er ist damit für die Dossier-Adresse richtig und w |
| `by-ammerndorf-dombrowski` | 49.4233 / 10.8501 | https://www.11880.com/branchenbuch/ammerndorf/120674719B107430542/fa-dombrowski.html (Abruf 18.11.2026).] - [Recherche 18.11.2026: **Amtliches Fachbetrieberegister ohne Treffer, und der gespeicherte Produktionspunkt liegt an einer anderen Strasse als |
| `by-aschaffenburg-schrott-peter` | 49.9719 / 9.1480 | https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Schrott%20Peter (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt nicht an der Dossier-Adresse, sond |
| `by-growallstadt-63868-schrott-rosel` | 49.8794 / 9.1534 | der gespeicherte Produktionspunkt 49.8794/9.1534 loest per Nominatim-Reverse auf 8, Moemlinger Strasse, 63868 Grosswallstadt (way/681635101), waehrend die Verzeichnisadresse Frankenstr. |
| `by-zeil-am-main-97475-denk-av` | **kein Punkt** ✓ | https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/dst=10&is=1&e=1&n=Denk (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO bleibt bewusst offen.** Das Dossier fuehrt clear_fields geo, der read-only-Abgleich der Produ |
| `hb-bremen-28201-hanse-entrumpelung` | 53.0929 / 8.8096 | https://www.bremer-branchenbuch.de/item/hanse-entruempelung-bremen/ (Abruf 18.11.2026).] - [Recherche 18.11.2026: **Befund zum gespeicherten Produktions-Geo: der Punkt liegt an anderer Stelle als die Dossier-Adresse - er wird hier nur dokumentiert, n |
| `he-griesheim-muhlum-schrottlogistik` | 49.8609 / 8.5725 | https://www.gelbeseiten.de/gsbiz/aafb1825-35ae-49c8-8af7-629e6deb98ca (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund unveraendert, aber heute erneut belegt: der gespeicherte Punkt liegt am falschen Strassenabschnitt, die Dossier-Adresse e |
| `he-kassel-bettenhausen-tsr-deutschland-standort-kassel` | 51.3158 / 9.4767 | https://www.tsr-recycling.de/fileadmin/user_upload/tsr-local-sites/downloads/TSR_RL_AB-We_02.01_Weigerkostenkatalog_Stand20201120__01.pdf (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt rund 4 km vo |
| `he-limeshain-63694-schrottschneider` | 50.2591 / 8.9776 | http://web.archive.org/web/20180808173738/http://schrottschneider.de/ (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO: die Hausnummer ist in OSM adressgenau vorhanden, der Betrieb aber nicht - deshalb weiterhin keine Koordinate, aber eine belastb |
| `ni-gifhorn-autoverwertung-gifhorn` | 52.4777 / 10.5511 | https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Ey%C3%9Felheideweg%209%2C%2038518%20Gifhorn%2C%20Deutschland%3B10.5351872%3B52.4612773%3B10.5471872%3B52.4742773%3B10.5231872%3B52.4482773&dst=3&is=1&e=1 (Abruf 18. |
| `ni-goslar-jerstedt-bogershausen-willi-adressprazisierung` | 51.9585 / 10.3895 | https://www.bundes-telefonbuch.de/goslar/autoverwertung/willi-boegershausen-bvg021728786 (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt nicht am Hausnummernobjekt der Dossier-Adresse.** Read-only-A |
| `ni-region-ruckzuck-recycling-schrotthandel-umzuge` | 52.1853 / 10.1023 | https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Dorfstra%C3%9Fe%2C%2031174%20Schellerten%2C%20Deutschland%3B10.1027764%3B52.1847898%3B10.1267764%3B52.2077898%3B10.0787764%3B52.1617898&dst=5&is=1&e=1 (Abruf 18.11. |
| `nw-coesfeld-eddys` | 51.9435 / 7.1681 | https://www.11880.com/branchenbuch/coesfeld/060440090B100095219/eddys-schrottservice.html (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt in der Coesfelder Innenstadt und damit rund 1,1 km von der L |
| `nw-herne-sumo` | 51.5426 / 7.1990 | https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Altfahrzeugverwertung/adr=Bielefelder%20Stra%C3%9Fe%20106%2C%2044625%20Herne%2C%20Deutschland%3B7.1880881%3B51.531294%3B7.200088%3B51.543294%3B7.176088%3B51.519294&dst=3&is=1&e=1 (Abruf 18.1 |
| `nw-paderborn-franz-gawor` | **kein Punkt** ✓ | https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Gawor (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO bleibt gesperrt: die Hausnummer 35 fehlt in OSM, und der frueher gespeicherte Altpunkt war |
| `rp-hanhofen-lehr-martin-schrotthandel-u-containerdie` | 49.3167 / 8.3408 | https://fachbetrieberegister.gadsys.de/fachbetrieberegister/Entsorgungsfachbetriebe/dst=10&is=1&e=1&n=Lehr (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt an einer anderen Strasse der Gemeinde Hanho |
| `rp-heidesheim-taurus-entsorgungs` | 50.0000 / 8.1261 | https://eo-entsorgung.de/downloads/ (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund: der gespeicherte Punkt markiert die Seed-Gemeinde Heidesheim, nicht die historische Registeranschrift des Taurus in Bingen.** Read-only-Abgleich der Produ |
| `rp-kaltenengers-56220-sell` | 50.4167 / 7.5500 | https://www.golocal.de/kaltenengers/schrotthandel/schrotthandel-sell-YVpB1/ (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt auf einer Autobahn-Raststaette und damit nicht am Dossier-Ort.** Read-only |
| `rp-trier-west-54294-matthias-alten` | 49.7534 / 6.6371 | https://lokaleschrottplatz.de/matthias-alten/ (Abruf 18.11.2026).] - [Recherche 18.11.2026: **GEO-Befund: der gespeicherte Produktionspunkt liegt auf einem Triejer Buergeramt in der Altstadt und damit rund 1,1 km von der Lead-Adresse entfernt.** Read |
| `th-nordhausen-mrc-mitteldeutsche-recycling` | 51.5018 / 10.7957 | https://www.mrc-umwelt.de/kontakt (Abruf 18.11.2026).] - [Recherche 18.11.2026: **Befund zum gespeicherten Produktions-Geo: es ist ein Stadtzentrums-Punkt von Nordhausen, kein Betriebsstandort - er wird hier nur dokumentiert, nicht eigenmächtig geänd |
