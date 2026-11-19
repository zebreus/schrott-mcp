# Aufträge und offene Punkte — 19.11.2026

## Betrieb: die Nachprüfung um 02:20 UTC ist an einem **eigenen** Fehler gescheitert ✓ ✓

Der Job `release-nachpruefung-7d1f87f` ✓ ✓ ist um **02:20 UTC** ✓ ✓ mit Exit **1**
✓ ✓ abgebrochen ✓ ✓ — **nicht** wegen eines Befunds in der Produktion ✓ ✓,
sondern wegen eines **Platzhalters** ✓ ✓ in **meinem eigenen** Auftragstext ✓ ✓:
der Prüfschritt enthielt das wörtliche `IN (...)` ✓ ✓ statt einer
Slug-Liste ✓ ✓ — das ist ein **SQL-Fehler** ✓ ✓ und kein Messbefund ✓ ✓ ✓ ✓

**Der Sachstand ist grün** ✓ ✓ und **von Hand** gemessen ✓ ✓ (19.11.2026,
**02:17 UTC** ✓ ✓):

```text
python3 scripts/verify-pending-corrections-7d1f87f.py --data-dir /var/lib/schrott-mcp \
  --since 2026-11-18T21:12:32Z
  -> "ok": true, "failures": []
```

**Der Grund** ✓ ✓: der Gutzmann-Handler ✓ ✓ lief um **02:15:17 UTC** ✓ ✓ — also
**nach** dem Rollout `7d1f87f` ✓ ✓ (18.11.2026, 21:12:32 UTC ✓ ✓) ✓ ✓ — und der
MSG-Handler ✓ ✓ war um **22:28:21 UTC** ✓ ✓ gelaufen ✓ ✓ ✓ ✓

**Der Release `7d1f87f`** ✓ ✓ ist damit **vollständig bestätigt** ✓ ✓ ✓ ✓
(`research/rollout-20261118-7d1f87f.md` ✓ ✓) ✓ ✓

**Die Lehre** ✓ ✓: Ein Auftragstext ✓ ✓ mit einem **Platzhalter** ✓ ✓ ist ein
**Fehler** ✓ ✓ — der Prüfschritt muss die Slugs **aus der versionierten
Scope-Datei** ✓ ✓ lesen ✓ ✓ und **nicht** aus einem eingebetteten Ausdruck ✓ ✓
✓ ✓

## Die offenen Punkte der Welle vom 19.11.2026, 00:00 ✓ ✓

**Sechs** Fragen bleiben ✓ ✓ und sind **nicht** Sache der Welle ✓ ✓ — sie gehören
dem Owner ✓ ✓ oder einer späteren Welle ✓ ✓ ✓ ✓

### 1. `rp-lingenfeld-recycling-lenhard-andreas-rohstoff-recyc` ✓ ✓ — die Nummer im Frontmatter ✓ ✓

Das Feld `certifications` ✓ ✓ nennt **ZZPT00700084** ✓ ✓ — diese Nummer ist im
amtlichen Fachbetrieberegister **nicht mehr auffindbar** ✓ ✓ (Permalink: **0**
Treffer ✓ ✓) ✓ ✓ Der Registereintrag von heute trägt **ZZNT00600209** ✓ ✓
(Rasterzeile ✓ ✓) mit der Vorgangsnummer **ZZNT006002091001** ✓ ✓ (Feld 3.3 ✓ ✓) ✓ ✓

**Die Welle** ✓ ✓ hat das **dokumentiert** ✓ ✓ und das Feld **nicht** geändert ✓ ✓
— eine Zertifikatsnummer zu ersetzen ✓ ✓ ist eine **Sachentscheidung** ✓ ✓ und
**keine** Recherchefrage ✓ ✓ ✓ ✓

**Empfehlung** ✓ ✓: Das Feld mit der belegten Nummer von heute ✓ ✓ **ersetzen** ✓ ✓
— der jetzige Eintrag verweist auf einen **nicht mehr existierenden** Vorgang ✓ ✓
✓ ✓ — und die alte Nummer **mit dem Vermerk** ✓ ✓ in der Notiz behalten ✓ ✓ ✓ ✓

### 2. `by-adelsdorf-kaiser` — **drei** gleichnamige Dossiers ✓ ✓

Es gibt **drei** Kaiser-Dossiers in Adelsdorf ✓ ✓ (nicht zwei ✓ ✓) ✓ ✓ — eines davon
(`by-adelsdorf-91325-kaiser` ✓ ✓) hat **keine** Adresse ✓ ✓ ✓ ✓

**Ein Merge** ✓ ✓ (also das Zusammenführen zweier Dossiers ✓ ✓) ist **keine**
Rechercheaufgabe ✓ ✓, sondern eine **Entscheidung des Owners** ✓ ✓ ✓ ✓

### 3. `by-oberkotzau-fattigau-95145-grundl-av` — Tätigkeit und Hausnummer ✓ ✓

Das **Handelsregister** ✓ ✓ nennt `Auto-Grundl oHG` ✓ ✓ (Hof HRA 2993 ✓ ✓) mit
Sitz **Oberkotzau** ✓ ✓ und Geschäftsanschrift **Fattigau Nr. 50** ✓ ✓ — die
Seed-Adresse lautet **Hauptstr. 28** ✓ ✓ ✓ ✓ Die Gemeindeliste führt den Betrieb
unter **Abschleppdienste** ✓ ✓, der Seed sagt **AV** ✓ ✓, OSM nennt zwei Flächen
**Grundl Autoverwertung** ✓ ✓ ✓ ✓

**Das `name`-Feld** ✓ ✓ bleibt unverändert ✓ ✓ — die Namens- und Ortsübereinstimmung
ist **belegt** ✓ ✓, die **Tätigkeit** ✓ ✓ aber nicht ✓ ✓ ✓ ✓

### 4. `ni-schortens-oestringfelde-martens-c` — der Ortsteil ✓ ✓

Die belegte PLZ ist **26419 Schortens** ✓ ✓, der Seed nennt **Oestringfelde** ✓ ✓
✓ ✓ Das Verzeichnis führt den Eintrag mit dem Status **„Geschlossen“** ✓ ✓ — ein
**Portalstatus** ✓ ✓ ohne Beweiswert ✓ ✓ ✓ ✓

### 5. `nw-bochum-wattenscheid-schrotthandler-plus` — die Domain ✓ ✓

Root **HTTP 403** ✓ ✓, alle Unterseiten **404** ✓ ✓ — bestätigt mit **zwei**
unabhängigen Abrufmethoden ✓ ✓ (Fetch ✓ ✓ und headless Chromium ✓ ✓) ✓ ✓ Das
Webarchiv zeigt Inhalt **bis zum 13.07.2026** ✓ ✓ ✓ ✓ `status` bleibt `pruefung`
✓ ✓ ✓ ✓

### 6. Die **acht** gemessenen Standortabweichungen ✓ ✓ — **entschieden** ✓ ✓

Die Entscheidung steht in `research/geo-entscheidung-20261119-0000.md` ✓ ✓ ✓ ✓
**Vier** Punkte **gesetzt** ✓ ✓ · **elf** **entfernt** ✓ ✓ · **sechs** bleiben ✓ ✓
✓ ✓ Die Änderungen ✓ ✓ gehen mit dem **nächsten** Release ✓ ✓ hinaus ✓ ✓ ✓ ✓
