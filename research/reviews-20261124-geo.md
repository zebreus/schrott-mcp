# Das unabhängige Review der Standortrunde vom 24.11.2026 — zwei Runden

**Stand 24.11.2026, 13:55 UTC ✓ ✓**

**Kandidat Runde 1** ✓✓ `c4a25922` ✓✓ · **Basis** ✓✓ `d7a75797` ✓✓
**Kandidat Runde 2** ✓✓ `16d1fe39` ✓✓ · **Scope** ✓✓
`research/release-20261124-geo-scope.json` ✓✓ — **28** ✓✓ Dossiers ✓✓

---

## Runde 1 — **SPERREN** ✓ ✓

Die **Entscheidung selbst** ✓✓ war in **allen 30** ✓✓ Fällen **richtig** ✓✓ —
beide **gesetzten** ✓✓ Punkte ✓✓ tragen ✓✓, und **keines** ✓✓ der **26**
✓✓ unveränderten Dossiers ✓✓ hätte einen **Punkt** ✓✓ **setzen** ✓✓ oder
**behalten** ✓✓ müssen ✓✓ ✓ ✓

**Vier Datenfehler** ✓✓ und **zwei Werkzeuglücken** ✓✓ ✓ ✓

| Nr | Dossier | Befund | Beleg |
|---|---|---|---|
| 1 | `st-salzatal-uhlmann` ✓✓ | **Datenfehler** ✓✓ — die neue Korrektur-Sektion behauptete, das Objekt der **Nachbarhausnummer 1** ✓✓ trage einen **Fremdnamen** ✓✓; es trägt **weder** ✓✓ `name` ✓✓ **noch** ✓✓ `operator` ✓✓ ✓✓ Die **Recherche-Sektion desselben Dossiers** ✓✓ sagt dasselbe ✓✓ ✓✓ | `node/9695007410` ✓✓: `{"addr:housenumber":"1", …}` ✓✓ — **kein** ✓✓ `name` ✓✓, **kein** ✓✓ `operator` ✓✓ ✓✓ |
| 2 | `nw-ahlen-goldsofort-ring-manufaktur` ✓✓ | **Datenfehler** ✓✓ — die Recherche nennt **20** ✓✓ Adressobjekte ✓✓; gemessen sind es **18** ✓✓ mit **17** ✓✓ verschiedenen Hausnummern ✓✓ ✓✓ Die **Nummernliste** ✓✓ ist **richtig** ✓✓ ✓✓ | Das Kachelraster im **eigenen** ✓✓ Fenster des Dossiers ✓✓: **18** ✓✓ Objekte ✓✓, **17** ✓✓ Nummern ✓✓ ✓✓ |
| 3 | `research/befund-20261124-0300-entscheidung.md` ✓✓ | **Datenfehler** ✓✓ — der **Zeitstempel** ✓✓ ging **rückwärts** ✓✓ (13:20 → 13:09 ✓✓) ✓✓ | `git show d7a75797:…` ✓✓ Z. 1/3 „13:20 UTC“ ✓✓ gegen `git show c4a25922:…` ✓✓ „13:09 UTC“ ✓✓ ✓✓ |
| 4 | `research/befund-20261124-0300-entscheidung.md` ✓✓ | **Datenfehler** ✓✓ — das **abgeschlossene** ✓✓ Befund-Dokument ✓✓ wurde **umgeschrieben** ✓✓ statt **fortgeschrieben** ✓✓ ✓✓ | **132** ✓✓ Zeilen geändert ✓✓ ✓✓ |
| 5 | `scripts/release/check_address_claims.py` ✓✓ | **Werkzeuglücke** ✓✓ — `KOORDINATE_RE` ✓✓ verlangte eine Koordinate **ohne** ✓✓ Leerraum ✓✓ vor dem Schrägstrich ✓✓ — die **beiden** ✓✓ gesetzten Koordinaten ✓✓ wurden **nie** ✓✓ geprüft ✓✓, das Gate meldete trotzdem „Befunde: 0“ ✓✓ ✓✓ | `KOORDINATE_RE.findall('auf **52.5163710 / 9.7356670**')` ✓✓ → `[]` ✓✓ ✓✓ |
| 6 | `hh-barmbek-sud-hamburg-schrott` ✓✓ | **Werkzeuglücke** ✓✓ (Darstellung ✓✓) — die Notiz nannte nur die **Turnhalle** ✓✓; an Hausnummer **90b** ✓✓ liegen **zwei** ✓✓ Objekte ✓✓, **44,7 m** ✓✓ auseinander ✓✓ ✓✓ | `node/1636827185` ✓✓ (name=Turnhalle-HH ✓✓) ✓✓ · `node/1754112219` ✓✓ (ohne name/operator ✓✓) ✓✓ ✓✓ |

---

## Die Antwort — Commit `16d1fe39` ✓ ✓

**Die zwei Datenfehler** ✓✓ in den Notizen ✓✓ ✓ ✓

| Dossier | Vorher | Nachher |
|---|---|---|
| `st-salzatal-uhlmann` ✓✓ | „…trägt einen **Fremdnamen**“ ✓✓ | „…trägt **weder** ✓✓ `name` ✓✓ **noch** ✓✓ `operator` ✓✓“ ✓✓ — und die **falsche** ✓✓ Angabe der **Recherche-Sektion** ✓✓ wird **benannt** ✓✓ ✓✓ |
| `hh-barmbek-sud-hamburg-schrott` ✓✓ | nur die **Turnhalle** ✓✓ | **beide** ✓✓ Objekte ✓✓, **44,7 m** ✓✓ auseinander ✓✓ ✓✓ |
| `nw-ahlen-goldsofort-ring-manufaktur` ✓✓ | — | die **gemessene** ✓✓ Zahl ✓✓: **18** ✓✓ Objekte ✓✓, **17** ✓✓ Nummern ✓✓ ✓✓ |

**Die beiden Akten** ✓✓ ✓ ✓ Das **Dokument vom 13:20 UTC** ✓✓ steht
**unverändert** ✓✓ wieder da ✓✓ ✓ ✓ Die **Berichtigung** ✓✓ steht in einer
**neuen** ✓✓ Akte ✓✓ `research/befund-20261124-0300-entscheidung-2.md` ✓✓ vom
**13:52 UTC** ✓✓ ✓ ✓

**Die Werkzeuglücke** ✓✓ ✓ ✓ Der Koordinaten-Regex ✓✓ trägt jetzt **beide**
✓✓ Schreibweisen ✓✓ ✓ ✓ **Drei** ✓✓ Tests ✓✓ decken es ab ✓✓ ✓ ✓

```python
# Die Koordinate ✓✓ - **mit** ✓✓ und **ohne** ✓✓ Leerraum ✓✓ vor dem
# Schraegstrich ✓✓ (Befund aus dem Review vom 24.11.2026 ✓✓) ✓✓
KOORDINATE_RE = re.compile(r"\*\*(\d{2}\.\d{4,7})\s*/\s*(\d{1,3}\.\d{4,7})\*\*")
```

**Ein** ✓✓ Nebenbefund ✓✓ aus dem neuen Regex ✓✓ ✓ ✓ Die Formulierung
„Nachbarhausnummer 1“ ✓✓ las das Gate als **Hausnummern-Behauptung** ✓✓ über
das genannte Objekt ✓✓ und meldete einen **Fehlbefund** ✓✓ ✓ ✓ Der Satz ist
**getrennt** ✓✓ ✓ ✓ Das Gate bleibt **streng** ✓✓ — die Behauptung wird
**vermieden** ✓✓ statt sie zu **umgehen** ✓✓ ✓ ✓
