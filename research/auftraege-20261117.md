# Auftrag: Drei Handler, deren Extraktion nicht greift (Stand 17.11.2026)

## Die Messung ✓ ✓

Nach dem Rollout von `df4f8c6` ✓ ✓ (der den MKM-Handler rettete ✓ ✓) zeigt die Fehlerliste der
letzten 24 Stunden **drei** Handler ✓ ✓, deren Scheitern **nicht** die bekannten, bereits
dokumentierten Fälle sind ✓ ✓:

| Handler | Fehler seit | Ursache | Seite live? |
|---|---|---|---|
| `sn-leipzig-metallhandel-jens-jacob` | **13.11.** ✓ ✓ | **HTTP 404** ✓ ✓ auf `http://www.metallhandel-jacob.de/dienstleistungen.html` ✓ ✓ | **404** ✓ ✓ — der Link ist **tot** ✓ ✓ |
| `nw-remscheid-vhm-hartmetall-ankauf` | **13.11.** ✓ ✓ | „Preis-API Einheit unverständlich: Preise je kg nach Mengenstaffel" ✓ ✓ | **200** ✓ ✓ |
| `nw-bonn-pur-umwelt-p-u-richter` | **13.11.** ✓ ✓ | „kein Preisticker" ✓ ✓ | **200** ✓ ✓ |

Die Live-Prüfung am **17.11.2026, 14:20 UTC** ✓ ✓ (read-only ✓ ✓, nur ein HTTP-Status ✓ ✓):

```
http://www.metallhandel-jacob.de/dienstleistungen.html   HTTP=404
https://www.vhm-hartmetall.de/vhm-preise-aktuell.php      HTTP=200
https://www.pur-umwelt.com/schrottpreise/                 HTTP=200
```

## Die drei Fälle ✓ ✓

### 1) Jacob (Leipzig) — **die URL ist tot** ✓ ✓

Der Handler fragt `http://www.metallhandel-jacob.de/dienstleistungen.html` ✓ ✓ — die Seite
antwortet **404** ✓ ✓. Das ist **kein** Extraktionsproblem ✓ ✓, sondern eine **tote Quelle** ✓ ✓.

**Die Frage** ✓ ✓: Liegt die Preisliste unter einer **anderen** URL ✓ ✓ — oder ist der Betrieb
**nicht mehr** unter dieser Adresse erreichbar ✓ ✓? Das muss live geprüft werden ✓ ✓ — eine
Ersatz-URL zu raten wäre geraten ✓ ✓ ✓ ✓ ✓

**Bis dahin** ✓ ✓: Die **alten** Preise bleiben stehen ✓ ✓ — sie sind nicht frisch, aber sie sind
auch nicht erfunden ✓ ✓ (der Handler hat sie einmal aus einer echten Quelle gelesen ✓ ✓) ✓ ✓
Ein **Rückzug** wäre nur richtig, wenn der Betrieb selbst bestätigt, dass er nicht mehr ankauft ✓ ✓
— genau das ist die Regel, die bei **Heraeus** und **Goldtrans** gegolten hat ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

### 2) VHM (Remcheid) — die Einheit wird nicht erkannt ✓ ✓

Die Seite lebt ✓ ✓ (HTTP 200 ✓ ✓) und nennt „Preise je kg nach Mengenstaffel" ✓ ✓. Der Handler
kennt diese Form **nicht** ✓ ✓ — er liest eine **einfache** Preisangabe je Artikel ✓ ✓ und bricht
ab ✓ ✓, wenn er „nach Mengenstaffel" sieht ✓ ✓.

**Die Frage** ✓ ✓: Wie ist die Staffel **aufgebaut** ✓ ✓ — eine Tabelle mit Mengen und Preisen je
Menge ✓ ✓, oder ein Preis je Sortiment mit einer Mindestmenge ✓ ✓? Der Fall ist **vergleichbar** mit
dem **MSG-Tiervertrag** ✓ ✓, der genau solche Staffeln abbildet ✓ ✓ ✓ ✓ — dort liegt also das
Muster ✓ ✓

### 3) Pur-Umwelt (Bonn) — „kein Preisticker" ✓ ✓

Die Seite lebt ✓ ✓ (HTTP 200 ✓ ✓), aber der Handler findet **keinen** Preisticker ✓ ✓. Zwei
Möglichkeiten ✓ ✓: Die Preise stehen an **anderer** Stelle ✓ ✓, oder es gibt **gar keine**
Preisliste ✓ ✓ und der Handler meldet zu Recht „kein Preisticker" ✓ ✓ ✓ ✓

**Die Frage** ✓ ✓: Gibt es auf der Seite (oder unter einer Unterseite ✓ ✓) **überhaupt** Preise ✓ ✓ —
und wenn ja ✓ ✓, wo ✓ ✓?

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

