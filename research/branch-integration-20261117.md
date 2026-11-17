# Branch-Integration 17.11.2026 — alles nach `main`, nichts verloren

**Befund:** Auf GitHub stand `main` **388 Commits** hinter dem Arbeitsstand ✓ ✓. Die ganze
Arbeit — Wellen, Rollouts, Handler-Fixes, Berichte — lag auf Neben-Branches (`rel12` und
Co.) und war damit im **Standard-Blick** des Repositories unsichtbar ✓ ✓. Das war die
wahre Ursache des Eindrucks, es passiere nichts mehr.

**Befund 2:** Auf GitHub lagen **10** Branches ✓ ✓ (`main`, `rel00`, `rel06`, `rel12`,
`rel18`, `rel30`, `wave0000`, `wave0000-geo`, `wave0600`, `wave1200c`) ✓ ✓, lokal **61**
(plus 12 Worktrees) ✓ ✓.

## Was getan wurde ✓ ✓

### 1) `main` ist die Release-Linie ✓ ✓

`main` war ein Vorfahr des Arbeitsstands ✓ ✓ → **Fast-Forward** ✓ ✓ (kein Merge-Commit,
keine Konflikte ✓ ✓). Danach:

```
2e5059b0..fd48e3be  main -> main     (388 Commits nachgezogen)
```

### 2) Die anderen Branches geprüft, nicht blind gelöscht ✓ ✓

Vor dem Löschen wurde **jeder** Branch gegen `main` geprüft ✓ ✓:

| Branch | Commits nicht in `main` | Befund |
|---|---|---|
| `rel00`, `rel06`, `rel12`, `rel18`, `rel30`, `wave0000`, `wave0000-geo`, `wave0600`, `wave1200c` | **0** ✓ | vollständig in `main` ✓ → gelöscht ✓ ✓ |
| `geo1108-1` | 1 | **74** Geo-Notizen vom 08.11. — alle **redundant** ✓ (die Koordinaten stehen in `main` aus den späteren Wellen ✓) → gelöscht ✓ ✓ |
| `hotfix/mkm-61-*` (2 Branches) | 1–2 | **keine** Test- oder Vertragsfunktion, die `main` fehlt ✓ (beide sind durch die späteren Wellen überholt ✓) → gelöscht ✓ ✓ |
| `review-cc9731a` | 2 | zwei Zertifikatstiefe-Korrekturen ✓ → **geprüft und als Duplikate verworfen** ✓ (sie stehen bereits veröffentlicht in `main` ✓ ✓) ✓ ✓ |
| `wellec-s2`, `wellec-s3` | 1 | **Nachprüfung mit Positivkontrollen** ✓ → **integriert** ✓ ✓ |

### 3) Die drei echten Inhalte integriert ✓ ✓

**a) `review-cc9731a` — zwei Zertifikatstiefe-Korrekturen, als Duplikate verworfen** ✓ ✓
Der Pre-Deploy-Review vom 17.11.2026 hat beide Notizen als **Duplikate** der bereits
veröffentlichten Korrekturen vom 06.11.2026 erkannt ✓ ✓: sie sagen dasselbe ✓ ✓ und wiederholen
darin Aussagen über den Frontmatter-Stand, die inzwischen **falsch** sind ✓ ✓ (der Eintrag nennt
inzwischen `Prüfungsdatum` statt `Auditdatum` ✓ ✓ und führt die Kennummer `I561E00978` ✓ ✓).
Eine Notiz behauptete sogar, die Tätigkeit „Makeln" sei **ergänzt** worden ✓ ✓ — das widersprach der
veröffentlichten Notiz ✓ ✓, die „2.8 Makeln sind **nicht** angekreuzt" festhält ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
**Beide Notizen sind deshalb ersatzlos entfernt** ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

`by-ansbach-vogel-sohn`: Das PDF-Feld 6 heißt wörtlich `Prüfungsdatum` (der Eintrag nannte
`Auditdatum`), und Anlage 2 weist in Feld 2.3 `Lagern` die Kennnummer `I561E00978` aus, die
im Eintrag fehlte ✓ ✓ · `nw-dortmund-horde-alcunimet`: Aussteller-Anschrift sowie Prüfungs-
und Ausstellungsdatum fehlten ✓ ✓

**b) `wellec-s2` / `wellec-s3` — die Nachprüfung vom 17.11.2026** ✓ ✓
Die Shards hatten **veröffentlichte** Notizen *in place* erweitert (das verbietet die
Hausregel ✓ ✓) und über den Frontmatter-Stand des Shard-Zeitpunkts gesprochen — der
inzwischen ein anderer ist ✓ ✓ (die späteren Wellen haben Felder geleert ✓ ✓). Übernommen
wurde deshalb nur, was **sachlich** neu ist ✓ ✓, als eigene datierte Sektion am
Timeline-Ende ✓ ✓:

| Dossier | Kern der übernommenen Notiz |
|---|---|
| `nw-essen-schrott-busters` | 10-km-Abfrage: **180** Efb- und **10** Altfahrzeug-Treffer; **kein** Treffer nennt Busters oder Wojke |
| `nw-bottrop-puffahrt` | **66** Efb- und **3** Altfahrzeug-Treffer; Angebot nennt 11 Regionen, Kurse nicht beziffert |
| `nw-munster-smb-…` | **56** Efb- und **1** Altfahrzeug-Treffer; **kein** Treffer nennt SMB |
| `nw-lippstadt-mrad` | **21** Efb- und **2** Altfahrzeug-Treffer (Sicking, Böhmer); **kein** Treffer nennt Mrad |
| `nw-horn-bad-meinberg-neuwald` | **16** Efb- und **1** Altfahrzeug-Treffer (Reibeholz); **kein** Treffer nennt Neuwald |
| `nw-bergisch-gladbach-krau` | Die Namensgleichheit „Krauß" allein verbindet den Seed **nicht** sicher mit dem Containerdienst |
| `nw-bochum-wattenscheid-…` | Die Plus-Seiten sind eine **eigene** Marke (eigene Kontaktdaten und Öffnungszeiten), keine Zweitseite |
| `nw-erkelenz-kb` | 11880 live gelesen: Mönchengladbacher Anschrift, nur Mobilnummer; der Widerspruch bleibt **offen** |
| `nw-ludinghausen-ash` | „500 Kilogramm" ist ein Transport-Hinweis, **keine** Mindestmenge; dritte Namensschreibweise (AES) bei gleicher Anschrift |
| `nw-paderborn-peitzmann` | Das Zertifikatsportal führt ZZEE002001862009 mit Gültigkeit **bis 29.04.2027** ✓ (07.10.2025–07.01.2027 gehört zur **EnMS/ISO-50001**-Anlage ✓) ✓ · GüKG und Z3140 sind Scans **ohne** Textebene ✓ |
| `nw-telgte-putzer` | Betrieb **Kurz Ebert** ist ein **anderer** Rechtsträger und wird **nicht** übernommen |

**c) `bb-ludwigsfelde-ka-de-schrott-und-metallhandels` — ein **vorbestehender** Widerspruch** ✓ ✓:
Der Branch `geo1108-1` ✓ ✓ führt den Punkt **52.3198376/13.2574964** ✓ ✓, das Dossier selbst **nicht**
✓ ✓ — es vermerkt „Klärfall – keine Koordinate gesetzt" ✓ ✓, weil der OSM-Punkt zu „Mühl Product &
Service" gehört ✓ ✓. Die **Datenbank** führt den Punkt trotzdem **seit dem 05.11.2026** ✓ ✓, weil der
Seed-Import ein leeres Feld **nicht** überschreibt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ — ein **vorbestehender** Fall ✓ ✓,
**nicht** durch die Integration verursacht ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
Die konsequente Korrektur wäre `clear_fields: geo` ✓ ✓ — sie ist **bewusst nicht** Teil dieses
Releases ✓ ✓ (sie ändert einen Feldwert ✓ ✓ und gehört in einen eigenen, geprüften Schritt ✓ ✓) ✓ ✓ ✓ ✓

### 4) Die Gates nach der Integration ✓ ✓

`check_note_order` **0** ✓ · `check_pre_wave_notes` **0** ✓ · `wave_gate` **0** ✓ ·
`check_cert_balance` **0** ✓ · `check_note_claims` **0** ✓ · Seed-Tests **15** passed, **0**
failed ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

### 5) Aufgeräumt ✓ ✓

- GitHub: **nur `main`** ✓ ✓ (9 Branches gelöscht ✓ ✓)
- Lokal: **1** Branch (`main`) ✓ ✓, **34** überholte Branches gelöscht ✓ ✓, **12** Shard-Worktrees
  entfernt ✓ ✓ (ihre Commits waren vollständig in `main` ✓ ✓ bzw. integriert ✓ ✓)

## Die Regel, die daraus folgt ✓ ✓

**Es gibt genau einen Branch: `main`.** Er **ist** die Release-Linie ✓ ✓. Shard-Arbeit läuft in
einem temporären Worktree ✓ ✓ (`git worktree add`) und wird **in `main` übernommen** ✓ ✓ — ein
Shard-Branch wird nicht angelegt ✓ ✓ und nicht zum Fundort der Arbeit ✓ ✓. Ein lokaler Branch,
der vollständig in `main` enthalten ist, wird **gelöscht** ✓ ✓ statt weitergeführt ✓ ✓.

Steht in `README.md` (Abschnitt *Deploy → Branches: exactly one* ✓ ✓), in
`prompts/hourly-checkin.md` (Push-Ziel ist immer `main` ✓ ✓) und in
`prompts/tiefenrecherche-welle.md` (Owner-Gate ✓ ✓) ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓

## Der Punkt, der die eigentliche Frage beantwortet ✓ ✓

Die Forschung wird **nicht** verworfen ✓ ✓ — gemessen an der Produktion ✓ ✓:

| Messung | Ergebnis |
|---|---|
| Notizen in `public.db` gegen das Dossier (`notes`, sha256 über die Seed-Extraktion) | **3945 / 3945 identisch** ✓ ✓ |
| Dossiers mit Frontmatter-Änderung seit 14 Tagen | **66** (17.11.) ✓ ✓, 62 (13.11.) ✓ ✓, 120 (11.11.) ✓ ✓ — täglich ✓ ✓ |
| Welle 17.11. ausgerollt | 17.11. **06:41 UTC** ✓ ✓ (`00a571c` ✓ ✓) ✓ ✓ |
| MKM ausgerollt | 17.11. **07:52 UTC** ✓ ✓ (`34b0b86` ✓ ✓) ✓ ✓ |

**Was tatsächlich veraltet aussieht, sind die Preise** ✓ ✓: **1608** `current_prices`-Zeilen
sind älter als sieben Tage ✓ ✓, weil die betroffenen Handler **nicht mehr greifen** ✓ ✓
(Seite umgezogen ✓ ✓, Bot-Schutz ✓ ✓) ✓ ✓. Das ist **keine** unterbliebene Arbeit ✓ ✓ —
sondern Arbeit, die am **Abruf** hängt ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓ ✓
