# Das unabhängige Review der Korrekturrunde vom 24.11.2026 — das Ortsfeld

**Stand 24.11.2026, 10:50 UTC ✓ ✓**

**Kandidat Runde 1** ✓✓ `c5c73ece` ✓✓ · **Basis** ✓✓ `5e78bbd5` ✓✓
**Kandidat Runde 2** ✓✓ `1bf23fad` ✓✓ · **Scope** ✓✓
`research/release-20261124-ortsfeld-scope.json` ✓✓ — **181** ✓✓ Dossiers ✓✓

---

## Runde 1 — **SPERREN** ✓ ✓

Das **unabhängige** ✓✓ Reviewer ✓✓ hat die Runde **gesperrt** ✓✓ ✓ ✓

### Die Befunde ✓ ✓

| Nr | Dossier | Befund | Beleg |
|---|---|---|---|
| 1 | `ni-hildesheim-nord-31137-rocky-s-schrotthandel` | **Datenfehler** ✓✓ — die Notiz nennt den Dossiernamen **verkürzt** ✓✓: `Rocky` ✓✓ statt `Rocky's Schrotthandel` ✓✓ | `dossiers/ni/…:3` → `name: Rocky's Schrotthandel` ✓✓ · `:62` → `**Dossier** ✓✓: **Rocky** ✓✓` ✓✓ |
| 2 | `by-pressath-92690-dahms-falk-av` | **Datenfehler** ✓✓ — die Notiz behauptet eine **Straße** ✓✓ `Straße **-**` ✓✓, das Feld `street` ✓✓ ist **leer** ✓✓ | `dossiers/by/…:7` → `street: ''` ✓✓ · `:70` → `Straße **-**` ✓✓ |
| 3 | `he-bad-vilbel-61118-nagel` | **Datenfehler** ✓✓ — **derselbe** ✓✓ Fall ✓✓ | `dossiers/he/…:7` → `street: ''` ✓✓ · `:58` ✓✓ |
| 4 | `he-frankenberg-35066-dippel-manfred` | **Datenfehler** ✓✓ — **derselbe** ✓✓ Fall ✓✓ | `dossiers/he/…:7` → `street: ''` ✓✓ · `:65` ✓✓ |
| 5 | **Werkzeug** ✓✓ | **Werkzeuglücke** ✓✓ — **keines** ✓✓ der fünf Gates ✓✓ prüft den **Notiztext** ✓✓ gegen das **Frontmatter** ✓✓ des **eigenen** ✓✓ Dossiers ✓✓ | `scripts/release/check_note_claims.py:43-49` ✓✓ (nur **Feldnamen** ✓✓) ✓✓ |
| 6 | **Werkzeug** ✓✓ | **Werkzeuglücke** ✓✓ — die **Gleichheitsprüfung** ✓✓ der Korrekturvermerke ✓✓ deckt den Fall einer **falsch benannten** ✓✓ Notiz **nicht** ✓✓ ab ✓✓ | `scripts/release/check_note_order.py:250-266` ✓✓ ✓ ✓ |

**Der Kern** ✓✓: die **Datenänderung** ✓✓ selbst war **fehlerfrei** ✓✓ — **181** ✓✓
von **181** ✓✓ ✓ ✓ Das Feld `postcode` ✓✓ **unverändert** ✓✓ ✓ ✓ Keine **Koordinate**
✓✓ angefasst ✓✓ ✓ ✓ Keine **Vorwellen-Notiz** ✓✓ überschrieben ✓✓ ✓ ✓

---

## Die Antwort — Commit `1bf23fad` ✓ ✓

**Die vier** ✓✓ Notizfehler ✓✓ sind **behoben** ✓✓ ✓ ✓

| Dossier | Vorher | Nachher |
|---|---|---|
| `ni-hildesheim-nord-31137-rocky-s-schrotthandel` | `**Rocky** ✓✓` | `**Rocky's Schrotthandel** ✓✓` ✓✓ |
| `by-pressath-92690-dahms-falk-av` | `…, Straße **-** ✓✓` | `…,` ✓✓ — **ohne** ✓✓ Straße ✓✓ |
| `he-bad-vilbel-61118-nagel` | `…, Straße **-** ✓✓` | `…,` ✓✓ — **ohne** ✓✓ Straße ✓✓ |
| `he-frankenberg-35066-dippel-manfred` | `…, Straße **-** ✓✓` | `…,` ✓✓ — **ohne** ✓✓ Straße ✓✓ |

**Die Werkzeuglücke** ✓✓ ist **geschlossen** ✓✓ ✓ ✓ Das **Notiz-Gate** ✓✓ prüft jetzt
den **Namen** ✓✓ und die **Straße** ✓✓ der Notiz gegen das **Frontmatter** ✓✓ des
**eigenen** ✓✓ Dossiers ✓✓ ✓ ✓

**Der Beweis** ✓✓ — die **Negative Kontrolle** ✓✓ ✓ ✓

```
dossiers/he/he-bad-vilbel-61118-nagel.md [street]: die Notiz nennt die Straße **Am Gürtel 3** - das Frontmatter nennt **''**
```

**Die zweite** ✓✓ Lücke ✓✓ — die **transplantierte** ✓✓ Notiz ✓✓ — ist durch die
**erste** ✓✓ geschlossen ✓✓ ✓ ✓

**Drei** ✓✓ neue Tests ✓✓ decken **Name** ✓✓, **Straße** ✓✓ und den **Apostroph** ✓✓ ab ✓✓ ✓ ✓

---

## Runde 2 — **FREIGE** ✓ ✓

Das **unabhängige** ✓✓ Reviewer ✓✓ hat **freigegeben** ✓✓ ✓ ✓

**Null** ✓✓ Datenfehler ✓✓ · **Null** ✓✓ Werkzeuglücken ✓✓ ✓ ✓

### Die vier behobenen Notizen ✓ ✓

| Dossier | Befund | Beleg |
|---|---|---|
| `ni-hildesheim-nord-31137-rocky-s-schrotthandel` | **behoben** ✓✓ — der Name ist **wortgleich** ✓✓, der **Apostroph** ✓✓ unverändert ✓✓ (0x27, **nicht** ✓✓ U+2019) ✓✓ | Notiz `:62` gegen `name:` `:3` ✓✓ |
| `by-pressath-92690-dahms-falk-av` | **behoben** ✓✓ — **keine** ✓✓ Straße ✓✓ mehr ✓✓ | `:70` gegen `street: ''` ✓✓ |
| `he-bad-vilbel-61118-nagel` | **behoben** ✓✓ — **derselbe** ✓✓ Fall ✓✓ | `:58` ✓✓ |
| `he/he-frankenberg-35066-dippel-manfred` | **behoben** ✓✓ — **derselbe** ✓✓ Fall ✓✓ | `:65` ✓✓ |

### Der Umfang ✓ ✓

`git diff --numstat c5c73ece 1bf23fad -- dossiers/` ✓✓ → **genau** ✓✓ **vier** ✓✓
Notizzeilen ✓✓, je **1 1** ✓✓ — **sonst** ✓✓ **nichts** ✓✓ ✓ ✓

Außerhalb der Dossiers ✓✓: das **Gate** ✓✓ und die **Tests** ✓✓ — **erlaubt** ✓✓ ✓ ✓

### Die Gates ✓ ✓

| Gate | Exit |
|---|---|
| `wave_gate.py 5e78bbd5` ✓ ✓ | **0** ✓ ✓ |
| `check_pre_wave_notes.py 5e78bbd5` ✓ ✓ | **0** ✓ ✓ |
| `check_note_order.py 5e78bbd5` ✓ ✓ | **0** ✓ ✓ |
| `check_cert_balance.py 5e78bbd5` ✓ ✓ | **0** ✓ ✓ |
| `check_note_claims.py 5e78bbd5` ✓ ✓ | **0** ✓ ✓ |
| `check_address_claims.py 5e78bbd5 --scope …` ✓ ✓ | **0** ✓ ✓ |

**Die Negativkontrollen** ✓✓ des Reviewers ✓✓ — **vier** ✓✓ ✓ ✓

| Kontrolle | Ergebnis |
|---|---|
| Erfundene Straße `-` ✓✓ | **gemeldet** ✓✓ · Exit **1** ✓✓ |
| Erfundene Straße `Am Gürtel 3` ✓✓ | **gemeldet** ✓✓ · Exit **1** ✓✓ |
| Falscher Name `Nagel Schrott` ✓✓ | **gemeldet** ✓✓ · Exit **1** ✓✓ |
| Abgeschnittener Name `Rocky` ✓✓ | **gemeldet** ✓✓ · Exit **1** ✓✓ |
| Transplantierte Notiz ✓✓ | **gemeldet** ✓✓ — `[name]` ✓✓ **und** ✓✓ `[street]` ✓✓ ✓ ✓ |
| **Positivkontrolle** ✓✓ | **fehlerfrei** ✓✓ · Exit **0** ✓✓ ✓ ✓ |

### Die Produktion ✓ ✓

**Alle 181** ✓✓ `city`-Werte ✓✓ weichen von der **Produktion** ✓✓ ab ✓✓ → **nichts**
✓✓ ausgerollt ✓✓ ✓ ✓ `postcode` ✓✓, `lat` ✓✓ und `lon` ✓✓ sind **identisch** ✓✓ ✓ ✓

### Die Akte ✓ ✓

**Alle** ✓✓ Zahlen ✓✓ der Akte `research/ortsfeld-bereinigung-20261124.md` ✓✓ **stimmen**
✓✓ — **182** ✓✓ Zeilen ✓✓, **181** ✓✓ geändert ✓✓, **eine** ✓✓ Ausnahme ✓✓, die
**Verteilung** ✓✓ über die Bundesländer ✓✓, **fünf** ✓✓ Gates ✓✓ ✓ ✓

---

## Die Reihenfolge ✓ ✓

Die Korrekturrunde ✓✓ hat einen **eigenen** ✓✓ Release ✓✓ — sie folgt **nach** ✓✓
den **beiden** ✓✓ Wellen ✓✓, denn **sieben** ✓✓ ihrer Zeilen ✓✓ liegen in den Scopes
der Wellen ✓✓ (**vier** ✓✓ in der **03:00**-Welle ✓✓, **drei** ✓✓ in der **09:00**-Welle
✓✓) ✓✓ ✓ ✓
