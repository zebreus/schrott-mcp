# 6h-Welle 28.10.2026 00:00 UTC, Shard C (BY, NI, TH, BB, SN, MV) — 20 Dossiers

Recherche ausschließlich lesend; Produktionsdatenbank nur read-only
(`file:…public.db?mode=ro`, `PRAGMA query_only=ON`). Kein Commit, kein Push, kein Deploy.

## Der Hebel: das amtliche Fachbetrieberegister

Ausgewertet wurde das Registerabzug-Spiegel `schrottregister.pages.dev`
(Fachbetrieberegister der Gemeinsamen Stelle Altfahrzeuge nach § 7 Abs. 2a AltfahrzeugV
und Register der Entsorgungsfachbetriebe nach § 56 KrWG, **Auszug 11.08.2026**) über die
je Ort- und Betriebsseite gelisteten Zertifikatsnummern, zertifizierten Tätigkeiten und
Abfallschlüssel. Der Spiegel weist sich selbst als „Unabhängiges Verzeichnis · kein
amtliches Angebot“ aus und wird hier - wie in den Wellen vom 26./27.10.2026 - als
Registerquelle geführt, nicht als Aggregator. **Negativbefunde im Register beweisen keine
Nichtexistenz**: die Efb-Anerkennung ist freiwillig und Annahme-/Rücknahmestellen werden
nur unvollständig erfasst.

## Ergebnis in Zahlen

| Kennzahl | Wert |
|---|---|
| Bearbeitete Dossiers | 20 (alle mit Notiz vom 28.10.2026) |
| Frontmatter geändert | 9 |
| `status` → `aktiv` | 2 (`ni-oldenburg-lupcke`, `bb-finsterwalde-schrott-recycling-finsterwalde`) |
| `status` → `geschlossen` | 2 (`th-tel-eintrag-mit`, `th-erfurt-optima-computer-elektronikrecycling`) |
| `clear_fields: geo` | 1 (`ni-oldenburg-lupcke`) |
| Neue Koordinate | 1 (`sn-grorohrsdorf-ot-bretnig-schrottaufbereitung-rosler`) |
| Belegte Ankaufspreise | **0** - in keinem der 20 Dossiers ist eine Zahlenpreisliste belegt |
| Belegte Gebührentarife | 0 |

## Frontmatter-Änderungen

- **`ni-oldenburg-lupcke`** — Stadt **Oldenburg → Wardenburg**, Straße/PLZ/Telefon/E-Mail/
  Öffnungszeiten/Website gefüllt, `status` → `aktiv`, `clear_fields: geo`, Zertifikat
  BREG2000-06/25 (Vorgang ZZDT001000418009, gültig bis 30.04.2027) mit 170401–170407/170411.
  Namenskette: „Lüpcke GmbH“ (HRB 2498) → „Michael Lüpcke GmbH“ (HRB 203176).
- **`sn-grorohrsdorf-ot-bretnig-schrottaufbereitung-rosler`** — Straße/PLZ gefüllt
  (Register + Betreiber + IWA-Zertifikat), `certifications` ZZPT00400159, neue Koordinate
  51.1615045/14.0464781 (OSM-Way 65233991 trägt selbst den Namen „Schrott Rösler“).
- **`th-triptis-spedition-kfz-service-c-bernhardt`** — Straße/PLZ gefüllt, `certifications`
  ZZFT01100062, description; **kein** Demontagebetrieb (nur Sammeln/Befördern/Handeln/Makeln).
- **`bb-finsterwalde-schrott-recycling-finsterwalde`** — `status` → `aktiv`,
  `certifications` ZZPT00600113, description; Zirkusplatz 1 amtlich bestätigt.
- **`mv-rostock-jade-entsorgung`** — Straße/PLZ gefüllt, `website` + `website_status: tot`
  (Domain nicht im DNS), description mit Betreiberkette.
- **`ni-alfeld-balzers-metallverwertungsgesellschaft`** — Straße/PLZ aus dem Handelsregister,
  description mit „Jetzt geschlossen“-Aggregatormeldung.
- **`by-miltenberg-63897-schrott-mehring`** — `website`/`website_status` + description
  (Betreiberkette), **keine** Dorfprozeltener Adresse übernommen (Schwesterdossier).
- **`th-erfurt-optima-computer-elektronikrecycling`** — `status` → `geschlossen`, description.
- **`th-tel-eintrag-mit`** — `status` → `geschlossen`, description (Importartefakt).

## Offene Owner-Punkte

1. **Namensänderung** `ni-oldenburg-lupcke`: „Lüpcke GmbH“ → „Michael Lüpcke GmbH“.
2. **Namens-/Betreiberfrage** `mv-rostock-jade-entsorgung`: „Jade-Entsorgung GmbH“ →
   „Interseroh Jade Entsorgung GmbH“ → heute ALBA Metall Nord GmbH (Zum Wasserwerk 6).
3. **Orts-/Umadressierung** `by-miltenberg-63897-schrott-mehring` (historischer Standort
   Nikolaus-Fasel-Str. 2 vs. Schwesterdossier `by-dorfprozelten-mehring`).
4. **Hausnummernkonflikt** `mv-rostock-jade-entsorgung`: Zum Wasserwerk 1 (schrottplatz-info)
   gegen Zum Wasserwerk 6 (amtlich).
5. **Dossier-Lücke** `th-tel-eintrag-mit`: Herkunft des Importfragments; die Zeile ist kein Betrieb.
6. **Negativbefund mit Gegenzeichen** `ni-alfeld-balzers-metallverwertungsgesellschaft`:
   Cylex „Jetzt geschlossen“ - Schließungsprüfung offen.
7. **Korridor** `mv-rostock-marienehe-schrott-ankauf-alen-rostock`: „NK Schrotthandel Rostock“
   unter derselben Handwerkstraße 5 - Namens-/Dublettenprüfung offen.
