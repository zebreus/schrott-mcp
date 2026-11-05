# Überblick-Kuratierung 05.11.2026 — 103 Dossiers

## Anlass

Nach den Dossier-Vollsplits (30.09.2026) trugen **1671** Dossiers den Platzhalter

`_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._`

Der `## Überblick`-Abschnitt ist der Einstieg für einen menschlichen Leser: er sagt, **was** der
Betrieb ist, **wo** er sitzt, **welche Belege** die Identität tragen und — entscheidend — **was
offen bleibt**. In dieser Welle wurden **30** (Geokodierungs-Welle 15:15) und jetzt **103**
Dossiers kuratiert; der Platzhalter-Bestand sinkt damit von 1671 auf **1538**.

## Vorgehen

Vier Agenten, **bundesland-disjunkt** (NW 30 · SH 29 · HE 23 · BY 21), arbeiteten ausschließlich aus
dem **vorhandenen Dossierinhalt**: Frontmatter, `description`, `certifications` und die
Timeline-Notizen. Es wurde **nicht** recherchiert (kein Netzzugriff) und **nichts** erfunden —
jede Zahl und jede Bezeichnung im Überblick ist im Dossier bereits belegt.

Format: **5–10 Zeilen reiner Fließtext** ohne Listen, Überschriften oder Hervorhebungen — genau so,
wie die bereits kuratierten Dossiers (Vorbild `bb-beelitz-ot-zauchwitz-recycling-center-zauchwitz.md`).

## Maschinell geprüft

| Prüfung | Ergebnis |
|---|---|
| geänderte Dateien | **103** (+939 / −105 Zeilen) |
| Dateien mit unerlaubter Änderung (Notiz, Frontmatter, `lat`/`lon`, `street`, Überschriftverschiebung) | **0** |
| `check_note_order.py` · `check_pre_wave_notes.py` · `wave_gate.py` | je **exit 0** |
| verbleibende Platzhalter | **1538** |
| Kurationsergebnisse (Auswahl) | „Offen bleibt die fehlende Anerkennung nach AltfahrzeugV …“, „die Abweichung zwischen Impressum (HRB 4164) und Registerspiegel (HRA 3233) …“, „die Öffnungszeiten sind dreifach widersprüchlich …“, „die Adresse ist nur betreiberseitig belegt, die zweite Quelle stützt nur die Registeridentität …“ |

## Was die Kuratierung sichtbar macht

Die Schlusssätze benennen **konkrete** Beleggrenzen statt allgemeiner Floskeln — das ist der eigentliche
Wert der Kuratierung:

* **Negativbefunde am Register** (kein Efb-Eintrag, kein AltfahrzeugV-Eintrag) bei Dossiers, deren
  Website einen Schrottplatz bewerbt (`nw-horn-bad-meinberg-krisch`, `nw-huckeswagen-christian-bacher`).
* **Widersprüche zwischen Quellen:** Impressum gegen Registerspiegel
  (`by-ansbach-vogel-sohn`: HRB 4164 vs. HRA 3233; `nw-borken-bernhard-stienemann`: HRA 3518 vs.
  Zertifikat HRA 5413), Öffnungszeiten gegen Öffnungszeiten
  (`sh-trittau-buhck-betriebshof-trittau-recyclinghof`: dreifach widersprüchlich),
  Dossier-Feld gegen Live-Adresse (`he-wiesbaden-dotzheim-h-l-r-rohstoffhandel-herbert-lagerin`:
  Mainz statt Dotzheim).
* **Fehlende Hausnummern in OSM**, die den Dossierstandort oder die Koordinate blockieren
  (`sh-gettorf-autoverwertung-gettorf-jess`: 66 → 68; `sh-pinneberg-gebruder-fabian`: 1b fehlt).
* **Preise fehlen durchgehend**, wo keine bezifferte Preisliste veröffentlicht ist — der
  Einkaufspreis ist in diesen Fällen **nicht** belegt und wird nirgends behauptet.

## Ein Sonderfall

`by-freigericht-somborn-63579-he-ac-containerdienst` liegt in `dossiers/he/` (durch eine frühere Welle
dorthin verschoben) — die Datei wurde an Ort und Stelle kuratiert, **nicht** verschoben ✓.

## Offen

* **1538** Dossiers tragen noch den Platzhalter.
* **1847** Dossiers führen im Überblick nur die kurze Importnotiz der Seed-Übernahme („Schrott,
  Abriss, Entrümpelung (lt. 11880)“ u. ä.). Das ist laut README zulässig („free-form working notes“),
  aber dünn — eine eigene Kuration könnte auch diese ersetzen.
