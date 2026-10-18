# VRD Lübeck: Review des uncommitteten Dossier-Diffs

Stand/Abrufe: **18.10.2026**. Prüfobjekt: `git diff HEAD -- dossiers/sh/sh-lubeck-vrd-schrott-handel.md` sowie vollständiges Dossier und README, insbesondere Quellenhierarchie Zeilen 183–202. Das Dossier wurde ausschließlich gelesen. Keine Produktionszugriffe, keine GEO-Recherche oder -Änderung. Integration entscheidet der Owner.

## Ergebnis

**Status-Downgrade freigabefähig, Textintegration nur nach Datums-/Evidenzkorrektur.** Gegen HEAD ändert der vorliegende Diff ausschließlich `status`, `description`, Überblick und einen neu eingefügten Timeline-Abschnitt. Die frühere Beschreibung behauptet mobilen Ankauf/Abholung als Tatsache; dafür wurde heute kein zugänglicher Betreiberbeleg gelesen. `pruefung` ist angemessen, `geschlossen` wäre unbelegt. Historische Kontaktdaten bleiben ungeklärt und dürfen weder als aktuell bestätigt noch als widerlegt dargestellt werden.

Der neue Abschnitt „Korrektur 12.10.2026“ suggeriert einen eigenen damaligen Direktabruf. Dieser Review hat **keinen Abruf vom 12.10.2026 nachgewiesen**. Die heute gelesenen Seiten legitimieren auch keine Abrufbehauptungen vom 02. oder 04.10.2026 rückwirkend.

## Tatsächlich direkt abgerufene Quellen am 18.10.2026

### Zulässige Quellenklasse, aber kein identifizierter VRD-Betreiberbeleg

- North Data, https://www.northdata.de/?query=VRD%20Schrotthandel%20L%C3%BCbeck — Suchseite mit den ersten 15 angezeigten Treffern gelesen. Kein kongruent identifizierter VRD-Treffer darunter; die breite Suche meldet 1.000+ Resultate, darunter LSH Lübecker Schrotthandel GmbH, HRB 3204 HL. **Kein vollständiger Register-Negativnachweis**, keine Identität/Nachfolge VRD–LSH bewiesen. Heute wurde keine LSH-Detailseite geprüft. Ein möglicherweise nicht eingetragenes Gewerbe lässt sich durch diese Suche nicht ausschließen; die historische Behauptung „Einzelgewerbe“ wird dadurch ebenfalls nicht bestätigt.

### Direkt gelesene Verzeichnisse — ausschließlich Leads, keine Betreiberbestätigung

1. Gelbe Seiten, https://www.gelbeseiten.de/gsbiz/8ebc5465-f328-4933-805c-02a204fdf8e2 — Detailseite gelesen: „VRD Schrott-Handel“, Branche „Schrott“, Schönböckener Str. 74C, 23556 Lübeck-Sankt Lorenz Nord; 0172 9020847; Mo–Fr 08:00–16:00. Kein konkreter E-Mail-Wert oder eigener Website-Link in der gelesenen Darstellung. Generische Schaltflächen „E-Mail“/„Website“ sind kein Nachweis entsprechender Kontaktdaten. Kein gelesener Beleg für mobilen Ankauf oder Abholung.
2. 11880, https://www.11880.com/branchenbuch/luebeck/060440092B112741573/vrd-schrott-u-metallhandel.html — Detailseite gelesen: „VRD Schrott- u. Metallhandel“, Branche „Altmetallrecycling“, Schönböckener Str. 74 **ohne C**, 23556 Lübeck (St. Lorenz Nord); dieselbe Telefonnummer; Mo–Fr 07:30–15:30, Sa 10:00–14:00. „E-Mail-Adresse hinzufügen“/„Website hinzufügen“. Seitentext: „aktualisiert am 04.10.2026“, „Eintragsdaten vom 12.03.2024“. Diese Datumsangaben sind **heute gelesener Seiteninhalt**, weder unser Abrufdatum noch Betreiber-Aktualitätsnachweis. Der allgemeine FAQ-Hinweis auf „vom Inhaber bestätigt“ ist keine konkret nachgewiesene VRD-Inhaberbestätigung. „Geschlossen bis Mo.“ bezeichnet einen Tagesöffnungsstatus, keine Geschäftsaufgabe.

Der Gleichklang von Adresse/Telefon in zwei Verzeichnissen liefert nach README **keinen zulässigen Zweitbeleg**. Hausnummerzusatz und Öffnungszeiten widersprechen sich weiterhin. Keine heutige Bestätigung der im Dossier gespeicherten E-Mail.

### Gesperrte Abrufe — Inhalt nicht gelesen

- https://www.kleinanzeigen.de/s-anzeige/3021501441 — Direktabruf HTTP 403.
- https://www.kleinanzeigen.de/s-vrd-schrotthandel/k0 — Direktabruf HTTP 403.

Heute wurden weder Anzeigeninhalt noch rechtliche Angaben, Inhaber, Steuernummer, Finanzamt, Leistungsangebot oder Ahrensbök-Adresse gelesen. Die Einordnung als frühere Betreiberanzeige stammt ausschließlich aus der Dossierhistorie. HTTP 403 beweist weder Löschung noch Schließung. Der ursprüngliche Statuswechsel zu `aktiv` lässt sich mit diesen Abrufen nicht neu verifizieren.

## Weitere Suchversuche und Grenzen

Die Suchintegration lieferte für alle folgenden Anfragen keine Ergebnisse: `"VRD" "Schrotthandel" Lübeck Betreiber`, `"0172 9020847" OR "vrd-schrotthandel@outlook.de"`, `VRD Schrott Handel Lübeck`, `VRD Schrott Ahrensbök Neustädter Chaussee`. Das ist nur ein eingeschränkter Rechercheausgang, kein Nachweis fehlender Existenz oder fehlender Website.

Kein aktueller eigener Auftritt, betreibergeführtes Sozialprofil oder kommunaler Gewerberegisterauszug wurde in diesem Lauf erschlossen. Keine aktuelle Betreiberidentität, keine zwei unabhängigen zulässigen Belege. Die README-Ausnahme für autoritative Betreiberseiten greift nicht: Es liegt gerade keine geprüfte Betreiberseite mit kongruentem Register-/Standortnachweis vor. Alte Verzeichnisnennungen wie Cylex, Das Örtliche und finde-offen aus der Timeline wurden heute nicht abgerufen und zählen hier nicht als gelesene Quellen. Keine Verbindung zu LSH/Topschrott aus alten Suchleads ableiten.

## Exakte freigabefähige Feldentscheidungen gegen HEAD

```yaml
status: pruefung
description: 'VRD Schrott-Handel in Lübeck: historischer Anzeigen-/Verzeichnislead; aktuelle Betreiberidentität, Geschäftsaktivität und Leistungen nicht bestätigt.'
```

- `status`: `aktiv` → `pruefung` wie im vorhandenen Diff freigeben; Begründung ist unzureichende zulässige Evidenz, nicht Schließung.
- `description`: obigen präziseren Wert statt des vorliegenden Diff-Texts verwenden. „Mobiler Schrottankauf und Abholung“ nicht als gegenwärtige Leistung oder heute gelesener Anzeigeninhalt ausgeben. Allenfalls als frühere Dossierbehauptung in der Historie belassen.
- **Keine weiteren Frontmatter-Änderungen freigegeben:** Name, `trader_type`, Anschrift einschließlich `74 C`, PLZ, Ort, Telefon und E-Mail bleiben unverändert als Altbestand; das ist keine erneute sachliche Bestätigung. Kein Ersatz von `74 C` durch `74` aufgrund eines Verzeichnisses. `website: ''`, `website_status: unbekannt`, leere `opening_hours`, `dropoff_json` und `pickup_json` beibehalten. Kein Preis-, Mengen-, Gebühren- oder Kostenlosigkeitswert gewonnen. Slug und Provenienz unverändert; kein GEO.

## Notwendige Textkorrekturen vor Integration

### Überblick vollständig ersetzen

> Historischer VRD-Anzeigen-/Verzeichnislead in Lübeck. Laut älterer Dossiernotiz stammen Kontaktdaten auch aus einer Betreiberanzeige; deren Inhalt und rechtliche Angaben konnten am 18.10.2026 wegen HTTP 403 nicht erneut gelesen werden. Heute gelesene Verzeichnisse nennen dieselbe Telefonnummer, widersprechen sich aber bei Hausnummerzusatz und Öffnungszeiten und gelten laut README nur als Leads. Anschrift, Telefon und E-Mail bleiben als ungeklärter Altbestand erhalten. Aktuelle Betreiberidentität, Geschäftsaktivität, mobiler Ankauf und Abholung sind nicht bestätigt; Status daher `pruefung`, nicht `geschlossen`. In den heute zugänglichen Quellen keine belastbare Betreiberpreisliste und kein Beleg kostenloser Abholung.

### Neu eingefügten Abschnitt vom 12.10. ersetzen, nicht rückdatieren

Vorgeschlagene Überschrift: `### Korrektur 18.10.2026`

> - [Korrektur 18.10.2026: `status` aktiv → pruefung; Beschreibung auf historischen Anzeigen-/Verzeichnislead ohne aktuelle Betreiber- oder Leistungsbestätigung begrenzt. Die historisch zitierte Kleinanzeigen-Anzeige und die VRD-Suchseite liefern beim heutigen Direktabruf HTTP 403; Inhalte und rechtliche Angaben nicht gelesen. Die heute direkt gelesenen Verzeichnisse Gelbe Seiten und 11880 nennen dieselbe Telefonnummer, aber abweichende Hausnummerzusätze und Öffnungszeiten; laut README sind beide nur Leads, keine unabhängigen zulässigen Betreiberbelege. Die ersten 15 angezeigten North-Data-Suchtreffer identifizieren keinen kongruenten VRD-Betreiber; keine vollständige Registerprüfung und kein Schließungsnachweis. Anschrift, Telefon und E-Mail bleiben unverändert als nicht aktuell bestätigter Altbestand. Keine neue Bestätigung von mobilem Ankauf, Abholung, Preisen oder kostenloser Abholung. Frühere Abruf- und Betreiberangaben der Timeline werden durch diesen Check nicht rückwirkend bestätigt; Quelle(n): https://www.kleinanzeigen.de/s-anzeige/3021501441 ; https://www.kleinanzeigen.de/s-vrd-schrotthandel/k0 ; https://www.gelbeseiten.de/gsbiz/8ebc5465-f328-4933-805c-02a204fdf8e2 ; https://www.11880.com/branchenbuch/luebeck/060440092B112741573/vrd-schrott-u-metallhandel.html ; https://www.northdata.de/?query=VRD%20Schrotthandel%20L%C3%BCbeck (Abruf 18.10.2026)]

Die Abschnitte 02./04.10.2026 als historische Dokumentation erhalten; ihre damaligen Aussagen nicht stillschweigend umdatieren oder als heute verifiziert darstellen. Insbesondere die historische Formulierung „Betreiber + 4 Verzeichnis-Zweitbelege“ erfüllt den README-Standard nicht; der neue Korrektureintrag muss diese Bewertungsgrenze ausdrücklich festhalten. Die dortige GEO-Arbeitsnotiz begründet keinen Auftrag in diesem Review.

## Offener Owner-Gate

Für spätere Aktiv-Freigabe aktuelle rechtliche Betreiberangaben und unabhängigen zulässigen Zweitbeleg beschaffen, z. B. zugänglicher betreibergeführter Auftritt plus kommunale Gewerbeauskunft. Dabei Lübeck-/Ahrensbök-Bezug und Alt-Kontakte einzeln zuordnen; keine fremden Händlerdaten transplantieren. Historische Mail und Mobilnummer nicht ohne Identitätsbeleg für einen aktuellen Betreiber übernehmen. Dieser Bericht integriert nichts und bestätigt keinen Produktionsstand.
