# Schrotthändler-Recherche — alle 16 Bundesländer

Stand: 2026-09-27. Vier Runden pro Bundesland: Ersterhebung (16 Agenten) +
Audit-Runde 2 (16 Agenten, andere Winkel) + Audit-Runde 3 (16 Agenten,
inkl. 4 Spezialisten-Audits für BW/HE/NW/SN) + Final-Sweep Runde 4
(16 Agenten, Dedup-/Korrekturpass). Dateien: ein Markdown-Table pro
Bundesland (`<kürzel>.md`), neue Funde je Runde als `## Nachtrag`
angehängt, nichts gelöscht (Korrekturen als Listen, keine Edits).

Ziel: jedes Unternehmen in Deutschland, an das man Schrott VERKAUFEN
(oder von dem man Schrott kaufen) kann — egal ob es sich
„Schrotthändler", „Metallhandel", „Altmetallankauf", „Wertstoffhändler",
„Autoverwertung" (nur mit Schrottankauf, geflaggt) o. Ä. nennt.
Generalisten UND Spezialisten (Kupfer, Kabel, Kats, Platinen, Hartmetall,
E-Schrott, Dental/Medizin, VA …), groß UND klein.

## Umfang (Tabellenzeilen pro Datei, inkl. Nachträge)

| Land | Datei | Zeilen |
|---|---|---|
| BW | bw.md | 163 |
| BY | by.md | 127 |
| BE | be.md | 95 |
| BB | bb.md | 127 |
| HB | hb.md | 74 |
| HH | hh.md | 66 |
| HE | he.md | 244 |
| MV | mv.md | 82 |
| NI | ni.md | 301 |
| NW | nw.md | 314 |
| RP | rp.md | 78 |
| SL | sl.md | 58 |
| SN | sn.md | 176 |
| ST | st.md | 118 |
| SH | sh.md | 93 |
| TH | th.md | 83 |
| **Summe** | | **≈ 2.200** |

Jeder Eintrag: Name, Ort, Website (per Abruf verifiziert oder „keine
Website gefunden"), ggf. Spezialisierung, Flag `Ankauf ja / unklar /
nur Altauto`. Unsichere Einträge sind als solche markiert — lieber
mit Flag aufgenommen als unterschlagen. Cross-State-Dedup-Check
(27.09.): nur 15 Namensdopplungen über Ländergrenzen, fast alle
beabsichtigt (Konzernstandorte wie Scholz/TSR/ALBA je Bundesland,
grenzüberschreitende Anhänge, bekannte Spillover).

## Quellen-Winkel (Auswahl)

Stadtsuchen („Schrottankauf <Stadt>", „Altmetall", „Metallrecycling",
„NE-Metall", „Kabelschrott", „Schrottabholung", „Schrottauto Ankauf"),
Gelbe Seiten / Das Örtliche / 11880 / Telefonbuch je Ort,
schrottradar.de, lokaleschrottplatz.de, schrottplatz-info.de,
schrottregister (Efb-/GSA-Register), wlw.de/europages, Verbände
(VDM→CMA, BDSV, bvse — meist login-geschützt, nur teilweise nutzbar),
Konzern-Standortlisten (Scholz, TSR, ALBA, Steil), Kleinanzeigen-Pro
(teilweise bot-blockiert), material-spezifische Queries (Kat, Platinen,
Hartmetall, VA, Bleiakkus …), Abbruchfirmen, Hafen-/Schiffs-Winkel,
Dorf-Level-Queries für weiße Flecken.

## Bekannte Restlücken / dokumentierte Negative

- Kleinst-/Hinterhof-Aufkäufer ohne Webpräsenz (nur Telefon/Vor-Ort
  klärbar, ~je nach Land als „unklar" markiert).
- Facebook-/Kleinanzeigen-mobile Aufkäufer (Bot-Schutz, flüchtig).
- Echte weiße Flecken (verifiziert leer): z. B. Fehmarn/Föhr/Amrum/
  Pellworm/Helgoland, Neumarkt-Stadt, Suhl-Stadt, Genthin-Stadt,
  Pirmasens/Südwestpfalz (nur mobil), Schiffsrecycling HB (strukturell
  null), Scheideanstalt mit Sitz BY/SN/ST (existiert nicht — alle in
  BW/HE/NW/BE/HH/SN-Background: Pforzheim/Hanau-Cluster).
- VDM/BDSV-Mitgliederlisten + Efb-Einzelregister: hinter Login/JS,
  nur teilweise ausgewertet — größtes verbliebenes Reservoir für
  eine künftige Runde mit Browser-Zugang.
- Rund 100+ „Ankauf unklar"-Einträge brauchen Telefon-/Vor-Ort-Check.

## Verwendung

Nächster Schritt: diese Listen in `traders` (+ `trader_materials`)
der `public.db` überführen (Ingestion, noch nicht begonnen). Vorher
empfohlen: Telefonstichprobe der „unklar"-Einträge + Handelsregister-
Abgleich der offenen Identitätsfälle (in den Dateien als
CORRECTIONS/Dedup-Verdacht notiert).
