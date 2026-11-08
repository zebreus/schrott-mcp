# Auftrag: veraltete Current-Preise beim Händler `be-marzahn-hellersdorf-efrem-gouchev-schrottankauf` (26 Paare)

**Status 08.11.2026, 16:20 UTC: erledigt** ✓ — **25 Rückzüge** ✓ per **Migration 12** ✓
(`2a58c60c` ✓, Liste `crates/store/src/public/stale_current_prices_12.tsv` ✓ mit **25** Zeilen ✓),
**ein Mapping-Gap** ✓ (kein Rückzug ✓) ✓. Das Dossier trägt die datierte Notiz
`### Korrektur 08.11.2026 (Preispflege)` ✓ mit allen 26 Paaren ✓ ✓.

**Die Probe gegen eine Kopie der Produktionsdatenbank** ✓ (08.11.2026 ✓):
`user_version` **11 → 12** ✓, `current_prices` **3590 → 3565** ✓ (genau die **25** geprüften
Paare ✓ ✓), Historie **46964 → 46964** ✓ unverändert ✓ ✓.

**Der Ausgangszustand** ✓ (der Auftrag war: **bestätigter** Datenbefund ✓ — der Handler läuft ✓,
die Seite bietet die Paare unter den alten Bezeichnungen **nicht mehr** an ✓).

## Der Befund

`scripts/release/find_stale_current_prices.py` ✓ (read-only ✓, Lauf 08.11.2026 ✓) meldet für
diesen Händler **26** veraltete Paare ✓ (beobachtet **04.11.2026** ✓, ein Paar vom **06.11.2026** ✓)
bei **56** Current-Pointern ✓. Der Handler selbst läuft **fehlerfrei** ✓ — letzter Lauf
08.11.2026 08:33 UTC ✓, Status `ok` ✓, **30** Preise übernommen ✓. Er erzeugt die 26 alten
Paare **nicht mehr** ✓: sie stehen als Current-Pointer weiter im Bestand ✓, ohne dass der Handler
sie noch bestätigt ✓.

**Es ist keine Handleränderung** ✓: `crates/ingestion/src/traders/handlers/gouchev.rs` ✓ wurde
zuletzt am **13.10.2026** ✓ (`ca654bfc` ✓) geändert ✓ — der Unterschied zwischen dem Satz vom
04.11. und dem vom 08.11. ist eine **Änderung der Seite** ✓ ✓.

## Die Seite heute (Live-Abruf 08.11.2026, HTTP 200, 56 010 Bytes ✓)

`https://www.schrottankauf-bitterfelderstr23.de/schrottpreise` ✓. Die Preiskarten stehen heute
**ohne Mengenstufen** da — nur „ab 1 kg" ✓; Mengenstufen („ab 200 kg", „ab 1000 kg") tragen **nur**
die Karten Kupfer ✓, Kupfer Millberry ✓, Messing ✓, Kupferkabel ✓, Kupfer Schwer ✓, Kupfer Kerze ✓
und Kupfer verzinnt ✓.

| Karte | Text heute (gekürzt) |
|---|---|
| Kupfer ✓ | „Kupfer ohne Eisen- oder Messinganhaftungen ab 1 kg 9,65 €; ab 200 kg: 9,75 € / 9,65 € Überweisung / Bar; ab 1000 kg: 9,85 € / 9,65 € Überweisung / Bar" ✓ |
| Kupfer Millberry ✓ | „Kupfer Millberry, nicht angelaufen, nicht lackiert ab 1 kg 10,30 €; ab 200 kg: 10,40 € …" ✓ |
| Messing ✓ | „Messing ohne Schläuche, keine Wasseruhren ab 1 kg 5,80 €; ab 200 kg: 5,90 € / 5,80 € Überweisung / Bar …" ✓ |
| Kupferkabel ✓ | „Kupferkabel, kein Antennen-, Fett-, ALCU-, Eisenkabel ab 1 kg 3,05 €; ab 200 kg: 3,15 € …" ✓ |
| Mischschrott ✓ | „Mischschrott ab 1 kg 0,10 €" ✓ |
| Alu Profile/Sorte1 ✓ | „Alu Profile 1,90 €; Alu Sorte1. 1,10 €; Alu Felgen 1,60 €; Alu Kupferkühler 3,90 €; Alu Guss 1,10 €" ✓ |
| Aluminiumschrott ✓ | „Aluminiumschrott mit max. 5 % Anhaftung ab 1 kg 1,00 €" ✓ |
| Edelstahlschrott V2A ✓ | „ab 1 kg 0,55 €" ✓ |
| Altblei ✓ | „ab 1 kg 1,00 €" ✓ |
| Auswuchtblei ✓ | „ab 1 kg 0,15 €" ✓ |
| Kabelschälblei ✓ | „ab 1 kg 0,30 €" ✓ |
| Elektromotoren ✓ | „ab 1 kg 0,45 €" ✓ |
| Kupfer Schwer ✓ | „Kupfer Schwer (ohne Lötstellen und Farbe) ab 1 kg 9,95 €; ab 200 kg: 10,05 € …" ✓ |
| Kupfer Kerze ✓ | „Kupfer Kerze (neu, ohne Anhaftung, nicht angelaufen) ab 1 kg 10,10 €; ab 200 kg: 10,20 € …" ✓ |
| Kupfer verzinnt ✓ | „Kupfer verzinnt ab 1 kg 9,75 €; ab 200 kg: 9,85 € / 9,75 € Überweisung / Bar …" ✓ |
| Zinnschrott Lötzinn ✓ | „ab 1 kg 5,00 €" ✓ |
| Zink ✓ | „ab 1 kg 1,70 €" ✓ |

## Die 26 Paare und ihre Bewertung

| Material | alte Variante | alter Preis | beobachtet | Bewertung |
|---|---|---|---|---|
| `aluminium-gemischt` | *(leer)* | 1,10 €/kg | 04.11. ✓ | **kein Rückzug** ✓ — der Preis **1,10 €** steht heute als „**Alu Sorte1.**" auf der Seite ✓ → **Mapping-Gap** ✓: der Handler überspringt die Karte, weil „Alu Sorte1." weder „aluminium" noch „5 %" enthält ✓ (`grade_for` ✓, `crates/ingestion/src/traders/handlers/gouchev.rs` ✓) ✓ |
| `aluminium-gemischt` | `5% Anhaftung, ab 200 kg Überweisung` | 1,15 €/kg | 04.11. ✓ | **Rückzug** ✓ — die Karte Aluminiumschrott ✓ trägt heute **keine** Mengenstufen ✓ |
| `aluminium-gemischt` | `5% Anhaftung, ab 1000 kg Überweisung` | 1,25 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `aluminium-gemischt` | `ab 200 kg Überweisung` | 1,20 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `aluminium-gemischt` | `ab 1000 kg Überweisung` | 1,30 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `blei` | `ab 200 kg Überweisung` | 1,10 €/kg | 04.11. ✓ | **Rückzug** ✓ (Altblei ✓ ohne Stufen ✓) |
| `blei` | `ab 1000 kg Überweisung` | 1,20 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `blei-auswucht` | `Auswuchtblei, ab 200 kg Überweisung` | 0,15 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `blei-auswucht` | `Auswuchtblei, ab 1000 kg Überweisung` | 0,15 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `blei-auswucht` | `Kabelschälblei, ab 200 kg Überweisung` | 0,40 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `blei-auswucht` | `Kabelschälblei, ab 1000 kg Überweisung` | 0,50 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `edelstahl-v2a` | `ab 200 kg Überweisung` | 0,55 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `edelstahl-v2a` | `ab 1000 kg Überweisung` | 0,55 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `elektromotoren` | `ab 200 kg Überweisung` | 0,60 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `elektromotoren` | `ab 1000 kg Überweisung` | 0,70 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `kabel-mit-stecker` | `mit Stecker, ab 200 kg Überweisung` | 1,10 €/kg | 04.11. ✓ | **Rückzug** ✓ (Kupferkabelschrott ✓ ohne Stufen ✓) |
| `kabel-mit-stecker` | `mit Stecker, ab 1000 kg Überweisung` | 1,20 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `loetzinn` | `Lötzinn, ab 200 kg Überweisung` | 6,00 €/kg | 04.11. ✓ | **Rückzug** ✓ (Zinnschrott Lötzinn ✓ ohne Stufen ✓) |
| `loetzinn` | `Lötzinn, ab 1000 kg Überweisung` | 7,00 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `mischschrott` | `ab 200 kg Überweisung` | 100,00 €/t | 04.11. ✓ | **Rückzug** ✓ |
| `mischschrott` | `ab 1000 kg Überweisung` | 110,00 €/t | 04.11. ✓ | **Rückzug** ✓ |
| `zink` | `ab 200 kg Überweisung` | 1,90 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `zink` | `ab 1000 kg Überweisung` | 2,00 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `zinn-geschirr` | `90-95%, ab 200 kg Überweisung` | 11,00 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `zinn-geschirr` | `90-95%, ab 1000 kg Überweisung` | 12,00 €/kg | 04.11. ✓ | **Rückzug** ✓ |
| `zinn-geschirr` | `90-95%` | 10,00 €/kg | **06.11.** ✓ | **Klärfall** ✓ — der Preis **10,00 €/kg** steht auf keiner der heutigen Karten ✓; der Handler erzeugt den Schlüssel `zinn-geschirr` / `90-95%` **nicht mehr** ✓ → **Rückzug** ✓, aber die Herkunft des Paares (06.11. ✓, zwei Tage nach dem übrigen Satz ✓) ist zu klären ✓ |

**Bilanz:** **25 Rückzüge** ✓ und **ein Mapping-Gap** ✓ (`aluminium-gemischt` / *(leer)* ✓ — der
Preis wird weiterhin angeboten ✓, nur unter einer Bezeichnung, die der Handler nicht zuordnet ✓).

## Auftragsinhalt

1. **Je Paar live belegen** ✓ (nicht aus dieser Tabelle übernehmen ✓, sondern selbst abrufen ✓):
   die Seite **live** holen ✓, den Kartentext zitieren ✓ und entscheiden ✓ — *steht die alte
   Bezeichnung mit dem alten Preis noch auf der Seite* ✓ → **kein** Rückzug ✓; *steht sie nicht
   mehr* ✓ → Rückzug ✓ ✓. Der Abruf braucht **zwei** Quellen ✓ (die Seite selbst ✓ und ein
   Zwischenspeicher/Index ✓, soweit vorhanden ✓) — die Beleglage wird **wörtlich** ins Dossier
   geschrieben ✓ ✓.
2. **Das Dossier** bekommt eine datierte Sektion am Timeline-Ende ✓:
   ```
   ### Korrektur 08.11.2026 (Preispflege)

   - [Korrektur 08.11.2026 (Preispflege): **…** …]
     Quelle(n): https://www.schrottankauf-bitterfelderstr23.de/schrottpreise (Abruf 08.11.2026).]
   ```
   mit **einer** Notiz ✓, die alle 26 Paare nennt ✓ (Preis, Variante, Beobachtung, Urteil ✓) ✓
   und die **25 Rückzüge** vom **einen** Mapping-Gap trennt ✓ ✓.
3. **Die Rückzugsliste** als TSV in derselben Form wie
   `crates/store/src/public/stale_current_prices.tsv` ✓ (7 Felder ✓:
   `slug|material|variant|preis|einheit|beobachtet|quelle` ✓) — **nur die 25 Rückzüge** ✓ ✓.
   Der Owner schreibt daraus die **Migration 12** ✓ (fail-closed ✓ wie Migration 11 ✓ ✓) — der
   Agent schreibt **keinen** Migrationscode ✓ ✓.
4. **Der Mapping-Gap** (`aluminium-gemischt` / *(leer)* ✓ = „Alu Sorte1." ✓) gehört **nicht** in
   die Migrationsliste ✓ ✓ — er ist ein **Handler-Befund** ✓ (die Karte wird übersprungen ✓ ✓)
   und wird als solcher im Dossier **benannt** ✓ ✓. Ein Vorschlag für die Zuordnung
   (`grade_for` ✓) darf genannt, aber **nicht** ohne Beleg eingebaut werden ✓ ✓.
5. **Grenzen** ✓: keine DB-Schreibzugriffe ✓, kein Force-Ingest ✓, kein Deployment ✓; die
   Migration 12 landet im **nächsten** Release ✓ und braucht dort die Pre-Deploy-Prüfung ✓ ✓.
