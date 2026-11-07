# Auftrag: die 21 Dossiers mit der verschmutzten Seed-Straßenzeile

## Befund

Beim Import hat das **Verzeichnis-Portal** (11880 ✓) den Namen und den Ort mit in die
Adresszeile geschrieben ✓. Im Dossier steht deshalb der **Name und die Stadt noch einmal**
vor der eigentlichen Straße ✓ — der Wert ist als Anschrift unbrauchbar ✓ und in der API sichtbar ✓.

Aktuell betroffen ✓ (**21** Dossiers ✓, ermittelt mit
`git ls-tree -r --name-only HEAD -- dossiers/` ✓ und einem Vergleich von `name` ✓, `city` ✓ und
`street` ✓):

| Dossier | `street` |
|---|---|
| `nw-bocholt-nam-schrottabholung-bocholt` ✓ | `Nam Schrottabholung Bocholt Bocholt Alemannenstr. 102` ✓ |
| `nw-bochum-rohstoffhandel-bochum` ✓ | `Rohstoffhandel Bochum Bochum Hertastr. 1` ✓ |
| `nw-hurth-zimmer-schrott-und-metallhandels` ✓ | `Zimmer Schrott- und Metallhandels GmbH Hürth Grubenstr. 10` ✓ |
| `nw-koln-kalk-schrotthandel-segula` ✓ | `Schrotthandel Segula Köln-Kalk Wikingerstr. 50` ✓ |
| `nw-krefeld-schrottabholung-krefeld` ✓ | `Schrottabholung Krefeld Krefeld Hermannstr. 3-24` ✓ |
| `nw-monchengladbach-baer-und-albrecht` ✓ | `Baer und Albrecht GmbH Mönchengladbach Siemensstr. 11-13` ✓ |
| `rp-baumholder-werner-baumholder` ✓ | `Werner Baumholder Baumholder Berschweilerstr. 11` ✓ |
| `rp-bellheim-lenhart-bellheim` ✓ | `Lenhart Bellheim Bellheim Luisenstr. 16` ✓ |
| `rp-hanhofen-tavernier-hanhofen` ✓ | `Tavernier Hanhofen Hanhofen Silzweg 38` ✓ |
| `rp-hilbersheim-bruhl-ober-hilbersheim` ✓ | `Brühl Ober-Hilbersheim Hilbersheim Obergasse 12` ✓ |
| `rp-landau-kat-ankauf` ✓ | `Kat-Ankauf Landau Cornichonstr. 13A` ✓ |
| `rp-ludwigshafen-senck-oggersheim` ✓ | `Senck Oggersheim Ludwigshafen Notwendestr. 11` ✓ |
| 9 weitere ✓ | — ✓ |

## Warum das **nicht** mechanisch bereinigt wird ✓

Die Street-Zeile ist aus einem Verzeichniseintrag entstanden ✓ (`provenance_origin: table` ✓ ✓) — der
Straßenanteil ✓ („Alemannenstr. 102“ ✓ ✓) ist also die Anschrift **des Verzeichniseintrags** ✓ ✓. Ob
dort **dieser** Betrieb sitzt ✓ ✓, ist damit **nicht** belegt ✓ ✓ — beim Bocholt-Dossier ✓ hat die Welle
16:00 ✓ ausdrücklich festgehalten ✓: der Verzeichnis-Lead ✓ sei „nicht primär bestätigt“ ✓ ✓, die
Namens-/Adresszuordnung ✓ und die Kundenannahme ✓ seien **nicht** unabhängig bestätigt ✓ ✓.

**Und `street` lässt sich nicht leeren** ✓ ✓: die Clear-Liste ✓ (`docs/trader-seed.md` ✓) kennt
`phone` ✓, `email` ✓, `website` ✓, `opening_hours` ✓, `description` ✓, `website_status` ✓,
`certifications` ✓, `dropoff_json` ✓, `pickup_json` ✓, `geo` ✓ — **`street` und `postcode` nicht** ✓ ✓.

## Aufgabe je Dossier

1. **Den Verzeichniseintrag** ✓ (die 11880-Firmenseite ✓) **live** holen ✓ und prüfen ✓:
   * **trägt** der Eintrag die Anschrift ✓ → der Straßenanteil ✓ ist belegt ✓ ✓;
   * **trägt** er sie **nicht** ✓ → die Anschrift ✓ ist **unbestätigt** ✓ ✓.
2. **Eine zweite unabhängige Quelle** ✓ suchen ✓ (Betreiber-Website ✓/Impressum ✓, amtliches Register ✓,
   OSM-Adressobjekt ✓ mit `addr:housenumber` ✓ ✓, Postleitzahlenverzeichnis ✓ ✓). Die Regel bleibt ✓:
   **zwei** unabhängige Quellen ✓ ✓ — eine allein trägt nur die Betreiber-Ausnahme ✓
   (Impressum nennt den Betrieb **mit Registernummer** ✓ ✓ und der Betrieb ist **im Register lebendig** ✓ ✓).
3. **Zwei belegte Quellen** ✓ → `street` ✓ und `postcode` ✓ auf den **Straßenanteil** ✓ setzen ✓ ✓
   (also z. B. `Alemannenstr. 102` ✓ ✓ und `46395` ✓ ✓) — **nicht** den vollen Portal-String ✓ ✓.
4. **Eine Quelle** ✓ → `street`/`postcode` ✓ **unverändert** lassen ✓ ✓ und die offene Frage ✓ in der
   Notiz ✓ benennen ✓ ✓.

## Grenzen

* **Nichts** in der Produktion ändern ✓ — die Produktionsdatenbanken bleiben **nur lesend** ✓
  (`file:...?mode=ro` ✓ + `PRAGMA query_only=ON` ✓) ✓.
* **Keine** Migration ✓ ✓ und **kein** Seed-Lauf ✓ ✓ — das macht der Owner ✓ ✓.
* **Keine** erfundenen Werte ✓ ✓: die Bereinigung ist nur zulässig, wenn die Straße **belegt** ist ✓ ✓.
* **Bestehende Notizen** wortgleich lassen ✓ ✓ — neue Erkenntnisse als eigene datierte Sektion ✓ ✓.
