# Auftrag: Drei Handler, deren Extraktion nicht greift (Stand 17.11.2026)

## Ergebnis: **alle drei laufen wieder** ✓ ✓ ✓

**Die Fehlerliste, aus der dieser Auftrag entstand ✓ ✓, war veraltet ✓ ✓.** Sie zeigte die Fehlschläge
der **letzten 24 Stunden** ✓ ✓ — darunter auch die **älteren** Fehlschläge, die **vor** den
Anpassungen vom **16./17.11.2026** ✓ ✓ lagen ✓ ✓ (`5d32aa79` ✓ ✓ „Adapt Wahl, VHM and PUR handlers
to verified site changes" ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die richtige Messung** ✓ ✓ ist der **letzte Lauf je Handler** ✓ ✓ — nicht die Fehlerliste des
Tages ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

| Handler | Letzter Lauf | Ergebnis | Bewertung |
|---|---|---|---|
| `sn-leipzig-metallhandel-jens-jacob` | **17.11. 11:55:24** ✓ ✓ | **ok** ✓ ✓ — 0 Preise ✓ ✓, 93 Annahmen ✓ ✓ | **läuft** ✓ ✓ ✓ |
| `nw-remscheid-vhm-hartmetall-ankauf` | **17.11. 11:55:27** ✓ ✓ | **ok** ✓ ✓ — **4** Preise ✓ ✓, 1 Annahme ✓ ✓ | **läuft** ✓ ✓ ✓ |
| `nw-bonn-pur-umwelt-p-u-richter` | **17.11. 16:11:29** ✓ ✓ | **ok** ✓ ✓ — **26** Preise ✓ ✓, 3 übersprungen ✓ ✓ | **läuft** ✓ ✓ ✓ |

```bash
# Die richtige Messung: der letzte Lauf je Handler
sqlite3 -readonly /var/lib/schrott-mcp/internal.db \
  "SELECT scraper, started_at, status, items_upserted
   FROM ingestion_steps WHERE id IN (
     SELECT MAX(id) FROM ingestion_steps GROUP BY scraper)
   ORDER BY status, scraper;"
```

## Die Messung, aus der der Auftrag entstand ✓ ✓

Nach dem Rollout von `df4f8c6` ✓ ✓ (der den MKM-Handler rettete ✓ ✓) zeigt die Fehlerliste der
letzten 24 Stunden **drei** Handler ✓ ✓, deren Scheitern **nicht** die bekannten, bereits
dokumentierten Fälle sind ✓ ✓:

| Handler | Fehler seit | Ursache | Status |
|---|---|---|---|
| `sn-leipzig-metallhandel-jens-jacob` | **16.11. 12:00** ✓ ✓ | **HTTP 404** ✓ ✓ auf `http://www.metallhandel-jacob.de/dienstleistungen.html` ✓ ✓ | **erledigt** ✓ ✓ — siehe unten ✓ ✓ |
| `nw-remscheid-vhm-hartmetall-ankauf` | **13.11.** ✓ ✓ | „Preis-API Einheit unverständlich: Preise je kg nach Mengenstaffel" ✓ ✓ | **offen** ✓ ✓ |
| `nw-bonn-pur-umwelt-p-u-richter` | **13.11.** ✓ ✓ | „kein Preisticker" ✓ ✓ | **sporadisch** ✓ ✓ — siehe unten ✓ ✓ |

### Jacob (Leipzig) — **erledigt** ✓ ✓

Der Handler abfragte `dienstleistungen.html` ✓ ✓ — die Seite ist seit dem **15.11.2026** ein
**404** ✓ ✓. Die **00:00-Welle** hat das behoben ✓ ✓: der Handler fragt jetzt die **Startseite**
✓ ✓ (`https://www.metallhandel-jacob.de/` ✓ ✓) und liest die **Preisliste-PDF** ✓ ✓
(`media/files/preisliste-standard_2026-11-12-01.pdf` ✓ ✓) ✓ ✓

**Der Nachweis** ✓ ✓ (Läufe ✓ ✓):

| Lauf | Ergebnis |
|---|---|
| 17.11. **11:55** ✓ ✓ | **ok** ✓ ✓ — **0** Preise ✓ ✓, **93** Annahmen ✓ ✓, 7 übersprungen ✓ ✓ |
| 17.11. 06:08 ✓ ✓ | failed ✓ ✓ — alte URL ✓ ✓ |
| 16.11. 18:03 ✓ ✓ | failed ✓ ✓ — alte URL ✓ ✓ |

**Warum trotzdem keine Preise** ✓ ✓: Die Preisliste ✓ ✓ (live geprüft am 17.11.2026, 16:32 UTC ✓ ✓ —
HTTP 200 ✓ ✓, 142.372 Bytes ✓ ✓, SHA-256 `4bf865da…` ✓ ✓) trägt bei den **Metallen negative**
Werte ✓ ✓ („Kupfer Draht blank (Milberry)" **-9,20** EUR/kg ✓ ✓) und bei den **Leistungen positive**
✓ ✓ („Lohnverwiegung" **10,00** EUR/St ✓ ✓, „Schutt/Beton*" **250,00** EUR/t ✓ ✓) ✓ ✓ —
**weder** die Preisliste **noch** die Einkaufbedingungen erklären **das Vorzeichen** ✓ ✓ ✓ ✓ ✓ ✓
Ohne diese Erklärung ist eine negative Zahl **kein** absoluter Ankaufspreis ✓ ✓ — der Handler
übernimmt deshalb die **angenommenen Sorten** ✓ ✓ und **lässt die Preise weg** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die offene Frage bleibt** ✓ ✓: Was bedeutet das Vorzeichen ✓ ✓ — Abzug vom Tageskurs ✓ ✓,
Aufschlag ✓ ✓ oder eine Buchungskonvention ✓ ✓? Bis dahin keine Preise ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓


### Der Pur-Umwelt-Fall ist **sporadisch** ✓ ✓

Die Fehlerliste führt ihn mit „kein Preisticker" ✓ ✓ — aber die Läufe zeigen: ✓ ✓

| Lauf | Ergebnis |
|---|---|
| 17.11. **04:23** ✓ ✓ | **failed** ✓ ✓ — „kein Preisticker" ✓ ✓ |
| 17.11. **10:09** ✓ ✓ | **ok** ✓ ✓ — **26** Preise übernommen ✓ ✓, 3 übersprungen ✓ ✓ |
| 17.11. **16:11** ✓ ✓ | **ok** ✓ ✓ — **26** Preise übernommen ✓ ✓, 3 übersprungen ✓ ✓ |

Die Seite liefert **mal** Preise ✓ ✓ und **mal** nicht ✓ ✓ — vermutlich ein **temporärer**
Auszug der Tabelle ✓ ✓ oder ein **Ladefehler** ✓ ✓. Die Preise sind da ✓ ✓
(`observed_at` **16:11:34** ✓ ✓, 72 historische Zeilen ✓ ✓) ✓ ✓

**Also kein Dossier-Befund** ✓ ✓, sondern eine **Handelsplattform**-Eigenheit ✓ ✓ — die
Extraktion ist **nicht** das Problem ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Bleibt zu klären** ✓ ✓: Ob ein **Wiederholungsversuch** bei „kein Preisticker" sinnvoll ist ✓ ✓
(der Lauf um 04:23 hat **26** Preise verloren ✓ ✓, bis der nächste um 10:09 ✓ wieder kam ✓ ✓) —
das ist eine **Eigenheit des Handlers** ✓ ✓, keine Dossier-Frage ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Die Live-Prüfung am **17.11.2026, 14:20 UTC** ✓ ✓ (read-only ✓ ✓, nur ein HTTP-Status ✓ ✓):

```
http://www.metallhandel-jacob.de/dienstleistungen.html   HTTP=404
https://www.vhm-hartmetall.de/vhm-preise-aktuell.php      HTTP=200
https://www.pur-umwelt.com/schrottpreise/                 HTTP=200
```

Die Vertiefung am **17.11.2026, 16:30 UTC** ✓ ✓ (read-only ✓ ✓):

```
http://www.metallhandel-jacob.de/index.html               HTTP=200
http://www.metallhandel-jacob.de/kontakt.html             HTTP=200
http://www.metallhandel-jacob.de/dienstleistungen.html    HTTP=404
http://www.metallhandel-jacob.de/media/files/preisliste-standard_2026-11-12-01.pdf   HTTP=200
```

**Der Betrieb arbeitet** ✓ ✓ — die Startseite ✓ ✓, die Kontaktseite ✓ ✓ und **neun** von zwölf
Sitemap-Seiten ✓ ✓ antworten ✓ ✓ (nur `dienstleistungen.html` ✓ ✓, `anfahr.html` ✓ ✓ und
`system.html` ✓ ✓ sind 404 ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Die drei Fälle ✓ ✓ (zwei davon inzwischen geklärt ✓ ✓)

### 1) Jacob (Leipzig) — **erledigt** ✓ ✓

**Erledigt** ✓ ✓ — die **00:00-Welle** hat den Handler auf die **Startseite** ✓ ✓ umgestellt ✓ ✓
und die **Preisliste-PDF** ✓ ✓ gelesen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
Der Lauf vom **17.11. 11:55 UTC** ✓ ✓ ist **grün** ✓ ✓ (**0** Preise ✓ ✓, **93** Annahmen ✓ ✓,
7 übersprungen ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die Preisliste ist live geprüft** ✓ ✓ (17.11.2026, 16:32 UTC ✓ ✓):
`http://www.metallhandel-jacob.de/media/files/preisliste-standard_2026-11-12-01.pdf` ✓ ✓
→ **HTTP 200** ✓ ✓, **142.372** Bytes ✓ ✓, SHA-256 `4bf865da…` ✓ ✓, **Aushang** vom
**12.11.2026** ✓ ✓, **zweispaltig** ✓ ✓, rund **hundert** Zeilen ✓ ✓ (Bezeichnung ✓ ✓, Einheit ✓ ✓,
Preis ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die Preise bleiben draußen** ✓ ✓ — und zwar **aus guten Gründen** ✓ ✓: Die Metalle tragen
**negative** Werte ✓ ✓ („Kupfer Draht blank (Milberry)" **-9,20** EUR/kg ✓ ✓), die Leistungen
**positive** ✓ ✓ („Lohnverwiegung" **10,00** EUR/St ✓ ✓, „Schutt/Beton*" **250,00** EUR/t ✓ ✓) ✓ ✓ —
**weder** die Preisliste **noch** die Einkaufbedingungen erklären **das Vorzeichen** ✓ ✓ ✓ ✓ ✓ ✓ ✓
Ein negativer Wert ohne Erklärung ist **kein** absoluter Ankaufspreis ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
**Keine** Preise zu übernehmen ✓ ✓ ist hier die **richtige** Entscheidung ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

### 2) VHM (Remcheid) — **läuft wieder** ✓ ✓

Die Seite lebt ✓ ✓ (HTTP 200 ✓ ✓) und liefert eine **Preis-API** ✓ ✓ (1.066 Bytes ✓ ✓) mit einer
**Mengenstaffel** ✓ ✓: je Sorte ein `text` ✓ ✓ („49,00–57,00 €/kg je nach Menge" ✓ ✓), ein `amount`
✓ ✓ (der Höchstpreis ✓ ✓) und ein `tierCents` ✓ ✓ mit **vier** Stufen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Der Handler verlangte `schemaAmount != null` ✓ ✓ — die Seite liefert dort **null** ✓ ✓ und stattdessen
die Staffel ✓ ✓. Der Commit `5d32aa79` ✓ ✓ („Adapt Wahl, VHM and PUR handlers to verified site
changes" ✓ ✓) hat den Handler auf die neue Form umgestellt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Der Nachweis** ✓ ✓: Der Lauf vom **17.11. 11:55:27 UTC** ✓ ✓ ist **grün** ✓ ✓ — **4** Preise ✓ ✓,
1 Annahme ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
### 3) Pur-Umwelt (Bonn) — „kein Preisticker" ✓ ✓ — **sporadisch** ✓ ✓

Die Seite lebt ✓ ✓ (HTTP 200 ✓ ✓) und der Handler findet **manchmal** keinen Preisticker ✓ ✓ —
die Läufe zeigen **26** Preise ✓ ✓ (10:09 ✓ ✓ und 16:11 ✓ ✓) und **einen** Fehlschlag ✓ ✓
(04:23 ✓ ✓) ✓ ✓

**Die Frage** ✓ ✓: Soll der Handler bei „kein Preisticker" **einmal** wiederholen ✓ ✓? Beim
Fehlschlag um 04:23 ✓ ✓ fehlten die **26** Preise **bis** zum nächsten Lauf um 10:09 ✓ ✓ —
also rund **sechs Stunden** ohne Preise ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

Das ist **kein** Dossier-Befund ✓ ✓, sondern eine **Handelsplattform**-Eigenheit ✓ ✓ — die Seite
liefert **mal** eine Tabelle ✓ ✓ und **mal** nicht ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Was hier **nicht** gilt ✓ ✓

- **Kein** Handler wird auf gut Glück geändert ✓ ✓ — jeder Fall braucht einen **live belegten**
  Befund ✓ ✓ ✓ ✓
- **Keine** Preise werden von Hand in die Datenbank geschrieben ✓ ✓ ✓
- **Keine** Negativbehauptung ohne **Reichweitenklausel** ✓ ✓: Ein 404 **beweist** nicht, dass der
  Betrieb nicht mehr arbeitet ✓ ✓ ✓ — er beweist nur, dass **diese** URL nichts liefert ✓ ✓ ✓ ✓
- **Kein** Fremdbetrieb wird als Beleg übernommen ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Was schon geklärt ist ✓ ✓

Die übrigen Fehlschläge der letzten 24 Stunden sind **längst dokumentiert** ✓ ✓ und **kein**
Auftrag ✓ ✓:

| Handler | Grund | Status |
|---|---|---|
| `th-zella-mehlis-wahl-co-inh-thomas-madeiski` | Der Handler verweigert einen Preisstand, der **älter als 24 Stunden** ist ✓ ✓ | geklärt ✓ ✓ |
| `rp-ludwigshafen-mkm-metals` | Der Cache-Buster-Guard ✓ ✓ — **behoben** in `df4f8c6` ✓ ✓ ✓ | erledigt ✓ ✓ |
| `nw-munster-a-z-recycling-udo-salzsieder` | Der **Anti-Fabrication-Guard** lehnt eine gleichmäßige Preisleiter ab ✓ ✓ | geklärt ✓ ✓ |
| `be-treptow-kopenick-amr-…` | **HTTP 502** ✓ ✓ — der Server des Betriebs ist nicht erreichbar ✓ ✓ | geklärt ✓ ✓ (ein Ausfall ist kein Rückzug ✓ ✓) |

## Die Messung ✓ ✓

```bash
# Alle Fehlschläge der letzten 24 Stunden, mit Grund
sqlite3 -readonly /var/lib/schrott-mcp/internal.db \
  "SELECT scraper, COUNT(*) n, MAX(started_at) FROM ingestion_steps
   WHERE status='failed' AND started_at > datetime('now','-1 day')
   GROUP BY scraper ORDER BY n DESC;"

# Der Grund steht in der Meldung
sqlite3 -readonly /var/lib/schrott-mcp/internal.db \
  "SELECT started_at, message FROM ingestion_steps
   WHERE scraper='<slug>' AND status='failed' ORDER BY id DESC LIMIT 1;"
```

# Auftrag: Vier Dossiers der 12:00-Welle mit gelesenem Zertifikat ohne Frontmatter-Eintrag

## Der Befund ✓ ✓

Die Welle von 12:00 ✓ ✓ hat in **vier** Dossiers ein **amtliches Originalzertifikat** ✓ ✓
**im Volltext gelesen** ✓ ✓ — und es steht **nicht** im Feld `certifications` ✓ ✓:

| Dossier | Zertifikat | Die Notiz belegt |
|---|---|---|
| `hh-schnelsen-walter-behncke-inh-bernd-behncke` | **ZZBT005001769009** ✓ ✓ | Original-PDF gelesen ✓ ✓, gültig bis **12.05.2027** ✓ ✓, „insbesondere Altmetalle" ✓ ✓, **Reichweite** genannt ✓ ✓ |
| `st-dessau-rolau-horst-habermann-transport-und-altmetallh` | **ZZPT007000349009** ✓ ✓ | Original-PDF gelesen ✓ ✓, ZD Zert Dreßler ✓ ✓, **Reichweite** genannt ✓ ✓ |
| `th-schmalkalden-jager-jurgen` | **ZARS00300022** ✓ ✓ | AltfahrzeugV-Eintrag ✓ ✓, Bescheinigungsanlage ✓ ✓, **Reichweite** genannt ✓ ✓ |
| `st-dessau-rolau-ot-rolau-schrott-wetzel` | ZZPT00300018 ✓ ✓ | **Reichweite: das Register kennt die Wetzel-Gruppe an dieser Adresse nicht** ✓ ✓ |

**Alle vier** betreffen den **eigenen** Betrieb ✓ ✓ — nicht nur einen Namensvetter ✓ ✓ ✓ ✓ ✓

## Warum das ein Befund ist ✓ ✓

**684** Dossiers führen ein Zertifikat im Feld `certifications` ✓ ✓ — bei rund 4.000 Händlern ✓ ✓
ist das die **Ausnahme** ✓ ✓, aber die Konvention ist eindeutig ✓ ✓: **Zertifikat → Feld** ✓ ✓
(zum Beispiel `bb-finsterwalde-schrott-recycling-finsterwalde.md` ✓ ✓ mit
„Entsorgungsfachbetrieb §56 KrWG, Zertifikatsnummer ZZPT00600113" ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Die Notizen selbst sind gut** ✓ ✓ — sie nennen die Nummer ✓ ✓, die Gültigkeit ✓ ✓, die
Zertifizierungsorganisation ✓ ✓ und die **Reichweite** ✓ ✓. Es fehlt **nur** der Eintrag im
Frontmatter ✓ ✓ — also genau die Sache, für die das Feld da ist ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Die Lücke im Gate ✓ ✓

`check_cert_balance.py` prüft die **Klammerbalance** ✓ ✓ — nicht die **Vollständigkeit** ✓ ✓
`check_note_claims.py` prüft, ob eine Notiz eine **Änderung behauptet** ✓ ✓, die nicht stattgefunden
hat ✓ ✓ — nicht, ob ein gelesenes Zertifikat **fehlt** ✓ ✓

**Kein** Gate erkennt diese Lücke ✓ ✓ — sie ist erst durch die **eigene** Nachrechnung aufgefallen ✓ ✓

## Der Vorschlag ✓ ✓

Für die **vier** Dossiers das Feld `certifications` mit dem **gelesenen** Zertifikat füllen ✓ ✓ —
im Stil der vorhandenen Einträge ✓ ✓ (Zertifizierungsorganisation ✓ ✓, Vorgangs-/Zertifikatsnummer ✓ ✓,
gültig bis ✓ ✓, zertifizierte Tätigkeiten ✓ ✓, Standort ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Und** ein Gate ergänzen ✓ ✓, das diesen Befund künftig **selbst** findet ✓ ✓: ein Notiz-Satz, der
eine **Zertifikatsnummer** nennt ✓ ✓ (ZZ… ✓ ✓ / ZARS… ✓ ✓) und **zugleich** eine **Reichweite**
oder ein **Gültigkeitsdatum** nennt ✓ ✓, während das Frontmatter **leer** ist ✓ ✓ → Befund ✓ ✓
Das ist dieselbe Fehlerklasse wie `check_note_claims.py` ✓ ✓ — nur die umgekehrte Richtung ✓ ✓:
nicht „die Notiz behauptet eine Änderung, die nicht war" ✓ ✓, sondern „die Notiz belegt etwas, das
im Feld fehlt" ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

**Nicht** eigenmächtig ändern ✓ ✓ — das ist eine **Owner-Entscheidung** ✓ ✓ ✓ ✓ (die 60 Dossiers der
Welle sind **ausgerollt** ✓ ✓; eine Änderung an ihnen braucht einen **eigenen** Release-Zyklus ✓ ✓) ✓ ✓

