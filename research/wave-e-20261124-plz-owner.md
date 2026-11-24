# Die Revision der Wave-E-Pin — PLZ-Trennung vom 24.11.2026

**Anlass** ✓ ✓: die Korrekturrunde vom 24.11.2026 ✓ ✓ hat **154** ✓✓ Dossiers
✓✓ berichtigt ✓✓, in denen das Feld `city` ✓✓ den Ortsnamen **mit** ✓✓ der
Postleitzahl ✓✓ trug ✓✓ und das Feld `postcode` ✓✓ **leer** ✓✓ war ✓✓ ✓ ✓

**Folge** ✓ ✓: **sieben** ✓✓ dieser Zeilen ✓✓ gehören zum **historischen**
✓✓ Wave-E-Scope ✓✓ vom 28.10.2026 ✓✓ ✓ ✓ Die **Erwartungsdatei** ✓✓ des
Wave-E-Pins ✓✓ trug für sie die **alten** ✓✓ Werte ✓✓ (`postcode` ✓✓ leer ✓✓)
✓✓, während die **Release-Delta**-Datei ✓✓ desselben Releases ✓✓ die **neuen**
✓✓ Werte ✓✓ trug ✓✓ ✓ ✓ Nach dem Rollout ✓✓ hätte der Wave-E-Verifier ✓✓
genau diese **sieben** ✓✓ Zeilen ✓✓ als Abweichung ✓✓ gemeldet ✓✓ ✓ ✓

**Die sieben Zeilen** ✓ ✓

- `by-wurzburg-versbach-97078-pfeifer-rainer-schrotthandel` ✓ ✓
- `he-bad-emstal-34308-pohl-anita` ✓ ✓
- `he-eschborn-65760-pfaff-alexander` ✓ ✓
- `he-hanau-63456-pfragner-georg` ✓ ✓
- `he-spangenberg-34286-heinrich-kleinschmidt-rohprod` ✓ ✓
- `rp-worms-67549-kampfer` ✓ ✓
- `st-kothen-06366-mrr-schrottplatz-kothen` ✓ ✓

**Die Revision** ✓ ✓ ist **mechanisch** ✓ ✓: das Werkzeug
`scripts/release/revise_wave_e_pin.py` ✓✓ übernimmt für die **genannten**
✓✓ Slugs ✓✓ die Werte aus der **privaten** ✓✓ Probe ✓✓ (Post-Seed ✓✓) ✓✓,
hebt die Notiz- und Seed-Hashes ✓✓ **aller** ✓✓ Zeilen ✓✓ auf den
Post-Seed-Stand ✓✓ und prüft zum Schluss ✓✓ gegen die Probe ✓✓ ✓ ✓

**Der Beleg** ✓ ✓ ist derselbe ✓✓ wie bei den Dossiers ✓✓: der
**Seed-Wert** ✓✓ desselben Feldes ✓✓ und die **Konvention** ✓✓ des Katalogs
✓✓ ✓ ✓ Es ist **keine** neue Quelle ✓✓ und **keine** Erfindung ✓✓ ✓ ✓

**Die Präzedenz** ✓ ✓: die Revisionen der Pins `7d1f87fc` ✓✓, `2b2f2b9b` ✓✓
und `e98b793a` ✓✓ sind nach demselben Muster gelaufen ✓ ✓ ✓ ✓
