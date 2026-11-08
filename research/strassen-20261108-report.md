# Straßenzeile der Seed-Daten — 22 Dossiers (08.11.2026)

**Zweig:** `strassen` (Commit `eff0cb59`, Basis `6f1e3d48`) — **nicht** gepusht, in `rel12`
gemergt (Merge `70cfebd0`).

Die 22 Dossiers trugen im Seed eine **Straßenzeile, die zu einem fremden Betrieb gehört**
oder die es so nicht gibt. Geprüft wurde jede Zeile gegen zwei unabhängige Quellen; die
gefundene Anschrift steht jetzt im Frontmatter, die offene Zuordnung in der Notiz.

## Ergebnis (nachrechenbar)

| Größe | Wert |
|---|---|
| Dossiers im Auftrag | **22** |
| `street` geändert | **16** |
| `postcode` geändert | **15** (ein Dossier behält `50354` — die PLZ stimmte dort schon) |
| Dossiers ohne Feldänderung | **6** |
| neue Timeline-Notizen | **22** (je genau eine Sektion am Timeline-Ende) |

Nachrechnen:

```bash
git diff --name-only 6f1e3d48 strassen -- dossiers/ | wc -l          # 22
git diff 6f1e3d48 strassen -- dossiers/ | grep -c '^+street:'        # 16
git diff 6f1e3d48 strassen -- dossiers/ | grep -c '^+postcode:'     # 15
```

## Je Dossier

| Dossier | Ergebnis | Quellen | Offene Zuordnung |
|---|---|---|---|
| `nw-bocholt-nam-schrottabholung-bocholt` | **Alemannenstr. 102 / 46395** | 11880 + lokaleschrottplatz.de (beide +49 2871 3463107), OSM way/254219656 | Betreiberidentität (kein Registereintrag), keine Preise |
| `nw-bochum-rohstoffhandel-bochum` | **unverändert** | Portalprofil „Rohstoffhandel Bochum" trägt Hertastraße 1; rohstoffhandel.ruhr + 11880 nennen Dieselstr. 18-20, Portalprofil H.R.B. nennt Daimlerstraße 13 | Identität Seed ↔ H.R.B. (drei Adressen, zwei Nummern); Hertastr. 1 bleibt Verzeichnislead |
| `nw-bochum-schrotthandler-1` | **Paulstr. 9d / 44803** | Portalprofil + Betreiber-Impressum (beide +49 176 88 24 2010), OSM way/330141627 | keine Rechtsform/Register (Betreiber-Ausnahme nicht erfüllt) |
| `nw-essen-hade-schrott-metallgrohandel-stamm` | **unverändert** | Portalprofil „Häde … Containerdienst" Heßlerstr. 77 trägt die Nummer der **metallion GmbH** (Impressum: Heßlerstr. 77, HRB 34136); Häde Recycling GmbH laut Impressum + 11880 an Alter Bottroper Str. 120 | **Falschzuordnung benannt**: Heßlerstr. 77 gehört der metallion GmbH; welche der drei Häde-Spuren der Seed ist, bleibt offen |
| `nw-hurth-zimmer-schrott-und-metallhandels` | **Grubenstr. 10 / 50354 (PLZ unverändert)** | Portalprofil, Betreiber-Impressum (HRB 43379), northdata (HRB 43379), OSM way/380679199 | **Abweichung benannt**: 11880 trägt „Alleestr. 8" (auch 2019); Namensform „…metallhandelsgesellschaft mbH" |
| `nw-koln-kalk-schrotthandel-segula` | **Wikingerstr. 50 / 51107** | 11880 + Portalprofil + Betreiber-Impressum + koeln.de | kein Registereintrag (Familienbetrieb), Öffnungszeiten-Widerspruch |
| `nw-krefeld-schrottabholung-krefeld` | **unverändert** | Portalprofil Hermannstraße 3-24; OSM: 36 Einzelnummern, **keine Bereichsnummer 3-24**; Domain = A. Lahib, Neukirchener Str. 45 | Identität/Standort des Seed-Betriebs |
| `nw-monchengladbach-baer-und-albrecht` | **Siemensstr. 11-13 / 41063** | Portalstadtseite, northdata (HRB 11914, früherer Name „Baer & Albrecht"), archiviertes Betreiberimpressum 03.11.2024, 11880, OSM | **PLZ-Widerspruch benannt** (Register 41066); Insolvenz/Liquidation 13.10.2025 offen, Korrektur ist keine Fortführungsaussage |
| `rp-baumholder-werner-baumholder` | **Berschweilerstr. 11 / 55774** | OSM way/206815256, Registerspiegel (Erich Werner GmbH, AfV ZAGS00300038), Cylex/Infobel-Index | Namenszuordnung „Werner Baumholder" ↔ „Erich Werner"/„Werner Erich" |
| `rp-bellheim-lenhart-bellheim` | **Luisenstr. 16 / 76756** | 11880 + lokaleschrottplatz.de (beide 07272 7778886), OSM way/759165331 | vier Lenhart-Schrotthändler in Bellheim; Karlheinz-Lenhart-Daten (Am Weidensatz 33) **nicht** übertragen |
| `rp-hanhofen-tavernier-hanhofen` | **Silzweg 38 / 67374** | lokaleschrottplatz.de (live) + Cylex (Lesestand 14.10.2026/Index, heute 403) + OSM way/109789061 | kein Betreiber-/Registerbeleg (business.site 404); Dublettenfrage zum Schwester-Dossier offen |
| `rp-hilbersheim-bruhl-ober-hilbersheim` | **Obergasse 12 / 55437** | lokaleschrottplatz.de (live) + Cylex (Index/Lesestand 15.10.2026) + OSM-Kachelraster (15 Nummern, 12 nicht erfasst) | kein Betreiber-/Registerbeleg, keine Preise |
| `rp-landau-kat-ankauf` | **Cornichonstr. 13A / 76829** | 11880 „Ankauf von Katalysatoren" + Gelbe Seiten „Cadorin Jeffrey" Cornichonstr. 13a + OSM (47 Nummern, Buchstabenformen) | Betreiberidentität (J. Cadorin an den Lerchenwiesen 0, 705 m entfernt) nicht zusammengeführt |
| `rp-ludwigshafen-senck-oggersheim` | **Notwendestr. 11 / 67071** | branchen-info.net + auto-werkstatt.de (gleicher Portalverbund, **nicht** unabhängig) + OSM way/483658559 | **Abweichung benannt**: Yelp nennt Hausnummer 7 (heute 403); Betreiberidentität offen |
| `rp-mulheim-karlich-golz-mulheim-karlich` | **Goethestr. 29A / 56218** | 11880 + metallbauer.io (beide 0170 3623689), weitere Portale im Index; OSM: 40 Nummern, 29A **Lücke** (auch 8, 33, 35, 39 fehlen) | Betreiberidentität, keine Preise |
| `rp-nassau-stoll-nassau` | **Bahnhofstr. 16 / 56377** | 11880 + lokaleschrottplatz.de (beide +49 1573 5709600) | kein Betreiber-/Registerbeleg; Koblenzer Lead nicht als Filiale belegt |
| `rp-neustadt-krahl` | **Moltkestr. 18 / 67433** | 11880 + Cylex-Index (0160 95825480, auch im Betreiber-Suchindex, Seite heute 403) + OSM way/365640662 | kein lesbarer Primärbeleg/Register; Ankaufsbedingungen nicht übernommen |
| `rp-neustadt-trojanowski` | **unverändert** | einzige betriebsnamende Quelle (Cylex „Scott Trojanowski", 0172 8497817) heute **403**; OSM way/610834987 belegt nur die Hausnummer; Register kennt „Trojanowski" nicht | Namens-/Adresszuordnung |
| `rp-rivenich-hutmacher-rivenich` | **Weinstr. 3 / 54518** | 11880 + lokaleschrottplatz.de (beide +49 176 34295698), meinestadt-Index, OSM way/717899226 | Betreiberidentität (Trierer Seite gehört Hans Peter Hutmacher) |
| `rp-trier-m-trier-west` | **unverändert** | keine live lesbare Quelle trägt Hornstr. 23; 11880 „Schrotthändler MW" trägt jetzt Eifelstr. 4, „Kochann Uwe" Hornstr. 24; Portalstadtseite: 3 andere Betriebe; OSM way/50281243 = „Lidl" | Zuordnung M Trier-West, 23 oder 24 |
| `rp-westheim-hms-westheim` | **unverändert (Referenzfall)** | 11880-Suche ohne Westheim-Treffer; OSM node/4566744070 + Ortsgemeinde belegen Straße/PLZ 67368, dort liegt aber ZZET01700389 (kommunale Biogasanlage); neu belegt: Rohstoffe Trapp (Hauptstr. 142) und Trapp-Service GmbH (Ludwig-Hoffmann-Weg 7) | Identität „HMS" ↔ „Maximilian Jäger" |
| `rp-wierschem-ries-wierschem` | **Pappelstr. 3 / 56294** | 11880 + adressennet.de live (beide 02605 847638), weitere Index-Portale, OSM way/490951019 | Betreiberidentität/heutige Tätigkeit (keine Betreiberseite, kein Register, Gemeindeliste ohne Ries); Punkt vom 04.11.2026 bleibt |

## Gates (alle exit 0 gegen `6f1e3d48`)

`check_note_order` (22 Dossiers, 22 neue Notizen) · `check_pre_wave_notes` (0 Abweichungen) ·
`wave_gate` (0 Gedankenstriche, Notenformat ok) · `check_cert_balance` (0 Abweichungen) ·
`check_note_claims` (22 Dossiers im Scope, 0 Befunde).

Kein Zugriff auf Produktionsdatenbanken; keine Migration, kein Seed-Lauf. Die Rust-Testsuite
konnte im Agenten-Arbeitsverzeichnis nicht laufen (`cargo` dort nicht installiert) — sie läuft
im Release-Zyklus des Kandidaten.

## Was offen bleibt

- **Sechs Dossiers** behalten ihre Seed-Straßenzeile: dort ist die Zuordnung nicht belegbar
  (fehlender Beleg, fremder Betrieb an der Hausnummer oder nicht existierende Bereichsnummer).
- **Elf Dossiers** tragen nach der Korrektur eine offene Namens- oder Identitätszuordnung; die
  Notiz nennt sie jeweils ausdrücklich.
