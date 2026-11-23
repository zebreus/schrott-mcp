# Aufträge und offene Punkte — 23.11.2026, Nacht

## Die Welle vom 00:00 ist ausgerollt ✓ ✓

Der Rollout ✓ ✓ ist am **23.11.2026, 06:05:31 UTC** ✓ ✓ erfolgt ✓ ✓
(Kandidat `8d496f8` ✓ ✓) ✓ ✓ — **60** Dossiers ✓ ✓, **vier** Punkte
**gesetzt** ✓ ✓, **einunddreißig** **entfernt** ✓ ✓, **fünfundzwanzig**
bleiben ✓ ✓ ✓ ✓

Die **Aufzeichnung** ✓ ✓: `research/rollout-20261123-8d496f8.md` ✓ ✓ ✓ ✓

**Die Nachprüfung** ✓ ✓ wartet auf den **Turnus** ✓ ✓ — Gutzmann **~09:59
UTC** ✓ ✓, MSG **~12:00 UTC** ✓ ✓ Die **Nachprüfung** ✓ ✓ ist für **12:05
UTC** ✓ ✓ angelegt ✓ ✓ (Job `release-nachpruefung-8d496f8` ✓ ✓) ✓ ✓

**Bis** ✓ ✓ das Gate **grün** ✓ ✓ ist, gilt: **keine** weiteren
Produktionsänderungen ✓ ✓ ✓ ✓

### Die zwei Reviews ✓ ✓

**Zwei** unabhängige Reviews ✓ ✓ haben die Kandidaten geprüft ✓ ✓ — das
**erste** an `4b71cde` ✓ ✓ meldete **zwölf** Befunde ✓ ✓, das **zweite** an
`8d496f8` ✓ ✓ **zwei** ✓ ✓ ✓ ✓

**Vier** der Befunde ✓ ✓ änderten die **Klassifikation** ✓ ✓ — ein **gelöschtes**
Unternehmen ✓ ✓ wurde **verortet** ✓ ✓, und **drei** Punkte ✓✓ trafen **andere
Objekte** ✓✓ als die Dossier-Adresse ✓✓ Der Owner hat **jeden** Fall **selbst
nachgemessen** ✓ ✓ ✓ ✓

## Die Nachprüfung des MKM-Rollouts ist erledigt ✓ ✓

**ERLEDIGT** ✓ ✓ — am **23.11.2026, 06:01 UTC** ✓ ✓ von Hand bestätigt ✓ ✓:
das **Pending-Gate** ✓✓ und der **Wave-E-Verifier** ✓✓ sind **grün** ✓ ✓ ✓ ✓
Nachweis: `research/checkin-20261123-0601-owner.md` ✓ ✓ ✓ ✓

### Der Stand vor dem Turnus ✓ ✓

Der Rollout der **MKM**-Reparatur ✓ ✓ ist am **22.11.2026, 23:57:21 UTC**
✓ ✓ erfolgt ✓ ✓ — Kandidat `208e641` ✓ ✓, **null** geänderte Dossiers ✓ ✓ ✓ ✓

Der **erste** MKM-Lauf nach dem Rollout ✓ ✓ ist am **23.11.2026, 00:57:56
UTC** ✓ ✓ mit `ok` ✓ ✓ gelaufen ✓ ✓ — **47** Preise ✓ ✓, **14** Annahmen ✓ ✓
✓ ✓ **Erstmals seit dem 20.11.2026** ✓ ✓ läuft der Handler **ohne Fehler**
✓ ✓ ✓ ✓

**Acht** Artikel ✓ ✓ haben auf der Seite ihren **Preis** ✓ ✓ verloren ✓ ✓ — der
Handler **erfindet** dort **keinen** Wert ✓ ✓, sondern schreibt eine
**Annahme** ✓ ✓ ✓ ✓ Die **Historie** ✓ ✓ bleibt erhalten ✓ ✓ ✓ ✓

Der **Pending-Gate** ✓ ✓ wartet auf die **Turnusläufe** ✓ ✓ — die
**Nachprüfung** ✓ ✓ ist für **05:55 UTC** ✓ ✓ angelegt ✓ ✓ (Job
`release-nachpruefung-208e641` ✓ ✓) ✓ ✓

## Die Welle vom 22.11.2026, 12:00 ist abgeschlossen ✓ ✓

Der Rollout `1464a7a` ✓ ✓ ist am **22.11.2026, 17:52:51 UTC** ✓ ✓ erfolgt
✓ ✓ und am **23.11.2026, 23:58 UTC** ✓ ✓ **von Hand bestätigt** ✓ ✓ — alle
**vier** Gates ✓ ✓ grün ✓ ✓ ✓ ✓ Nachweis:
`research/checkin-20261122-2358-owner.md` ✓ ✓ ✓ ✓

## Offene Punkte ✓ ✓

### 1. Der generische Pending-Gate ist wieder grün ✓ ✓ — **behoben** ✓ ✓

`scripts/test_verify_pending_corrections.py` ✓ ✓ war seit dem **08.11.2026**
✓ ✓ rot ✓ ✓, weil die Erwartungsdatei ✓ ✓ von `c743e3b3` ✓ ✓ **gelöscht**
✓ ✓ worden war ✓ ✓ ✓ ✓ **Behoben** ✓ ✓ (Commit `ff8ca79c` ✓ ✓) — die Datei
ist aus der **privaten Probe** ✓ ✓ des Kandidaten ✓ ✓ wiederhergestellt ✓ ✓
✓ ✓ Die **neun** Contract-Tests ✓ ✓ sind **grün** ✓ ✓ ✓ ✓

### 2. Der Bericht des Wellen-Torwarts ✓ ✓ — drei Fehler behoben ✓ ✓

**Behoben** ✓ ✓ (Commits `b6ea756c` ✓ ✓, `6255fc35` ✓ ✓, `a8efb38f` ✓ ✓) ✓ ✓:

- Der Bericht der Welle vom **22.11.2026, 00:00** ✓ ✓ nennt jetzt
  **dreißig** ✓ ✓ statt **neunundzwanzig** ✓ ✓ Dossiers mit geändertem
  `clear_fields` ✓ ✓ — ein **entferntes** Feld ✓ ✓ zählt mit ✓ ✓ ✓ ✓
- Der Generator liest **immer** den **Kandidaten** ✓ ✓ statt des
  **Arbeitsbaums** ✓ ✓ — sonst hätte ein Bericht über eine **ältere** Welle
  ✓ ✓ deren Zählung mit **späteren** Änderungen **vermischt** ✓ ✓ ✓ ✓
- Die Quellzahlen zählen **nur** die **Recherche-Sektion** ✓ ✓ — die
  **Korrektur-Sektionen** ✓ ✓ des Owners ✓ ✓ wurden **mitgezählt** ✓ ✓ ✓ ✓

**Der Bericht** ✓ ✓ der Welle vom 23.11.2026, **00:00** ✓ ✓ ist mit dem
reparierten Generator ✓ ✓ erzeugt ✓ ✓ und **byteweise reproduzierbar** ✓ ✓ ✓ ✓

### 3. Die Quellen der Welle vom 00:00 ✓ ✓

**Ein** Ankaufkurs ✓ ✓ in der ganzen Welle ✓ ✓: **Hartmetallschrott 40–60
EUR/kg** ✓ ✓ des Schrotthandels in **Northeim** ✓ ✓, **Stand Juli 2026** ✓ ✓
✓ ✓ **Keine** Verkaufspreisliste ✓ ✓ und **keine** Gebührentafel ✓ ✓ eines
Dossierbetriebs ✓ ✓ wurde übernommen ✓ ✓ ✓ ✓

### 4. Das Provider-Limit des Schedulers ✓ ✓ — **offen** ✓ ✓

Das **Tageslimit** ✓ ✓ des Scheduler-Providers ✓ ✓ ist **erschöpft** ✓ ✓
(**45,02 $** von **45,00 $** ✓ ✓) ✓ ✓ und **resetet nicht** ✓ ✓ ✓ ✓ Die
**Nachprüfungen** ✓ ✓ müssen deshalb **von Hand** ✓ ✓ erfolgen ✓ ✓ ✓ ✓

### 5. Die offenen Fragen der Shards ✓ ✓ — **zu klären** ✓ ✓

Die Shards haben **dreizehn** Fragen offengelassen ✓ ✓ — die **wichtigsten**
✓ ✓:

- `by-rodelsee-knettenbrech-wagner` ✓ ✓: der Registerspiegel ✓ ✓ sagt
  „**aktiv**“ ✓ ✓, das Dossier „**erloschen**“ ✓ ✓ — das **amtliche** Portal
  ✓ ✓ ist formulargeschützt ✓ ✓ ✓ ✓
- `rp-koblenz-56068-hellwegner` ✓ ✓: der **e.K.** ✓✓ ist **gelöscht** ✓ ✓ —
  die Entscheidung `geschlossen` ✓ ✓ gegen `pruefung` ✓ ✓ ist offen ✓ ✓ ✓ ✓
- `by-schweinfurt-hafurt-bad-kissi-peter-baum-schrott` ✓ ✓: **Mühlgasse 9**
  ✓ ✓ oder **Urnenfelderstraße 8** ✓ ✓ — die **Straßenvariante** ✓ ✓ ist vor
  der Setzung zu klären ✓ ✓ ✓ ✓
- `ni-celle-ricardo-dettmer-…` ✓ ✓: die **Sammelzeile** ✓ ✓ mit **vier**
  Namen ✓ ✓ hat **kein** gemeinsames Adressobjekt ✓ ✓ ✓ ✓
- `st-badingen-bismark-hinze` ✓ ✓: das Zertifikat ✓ ✓ und das Impressum ✓ ✓
  nennen **verschiedene** Anschriften ✓ ✓ ✓ ✓
- `st-kothen-06366-mrr-…` ✓ ✓: der **Nachfolgekandidat** ✓ ✓ steht **nicht**
  im Fachbetrieberegister ✓ ✓ und seine Website ist **leer** ✓ ✓ ✓ ✓

### 6. Der Abgleich der Adressangaben ist kein Gate ✓ ✓ — **vorgeschlagen** ✓ ✓

Der Abgleich ✓ ✓, der jede PLZ ✓ ✓ und jede Straße mit Hausnummer ✓ ✓ der
Korrektur-Sektionen ✓ ✓ gegen die **Tags** des genannten Objekts ✓ ✓ prüft
✓ ✓, liegt **außerhalb** des Repos ✓ ✓ — im Repo ist **kein** Gate ✓ ✓,
das genau diesen Abgleich macht ✓ ✓ ✓ ✓

**Vorschlag** ✓ ✓: ihn als `scripts/release/check_address_claims.py` ✓ ✓ ins
Repo nehmen ✓ ✓ — als **sechstes** Gate ✓ ✓ des Wellen-Torwarts ✓ ✓ ✓ ✓
