# Handler-Backlog, quantifiziert und auf Eignung geprüft (Stand 27.10.2026)

Read-only Auswertung der Produktionsdatenbank (`mode=ro`, `PRAGMA query_only=ON`) gegen
`crates/ingestion/src/traders/handlers/`, anschließend eine Stichprobe der Domains gegen
das live Web. Keine Datei- und keine Produktionsänderung.

## Ausgangslage

- **3.936** Händler in der Produktion, **128** Handler in `crates/ingestion/src/traders/handlers/`.
- **1.773** Händler haben eine Website mit `website_status: aktiv`.
- Davon haben **1.636** keinen eigenen Handler (kein Slug im Handler-Quelltext referenziert),
  verteilt auf **1.390** Domains.

## Erste Einschätzung (korrigiert)

Die naive Zahl „1.381 Domains mit 1.616 Händlern sind der echte Backlog" ist **falsch**.
Die Mehrzahl dieser Domains veröffentlicht gar keine maschinenlesbare Preisliste. Ein
Handler braucht aber genau das: eine öffentlich erreichbare Seite mit Materialbezeichnung,
Preis, Einheit und Datum.

## Eignungsprüfung (Stichprobe der größten Gruppen, 27.10.2026)

| Domain | Händler | Befund |
|---|---|---|
| scholz-recycling.com | 33 | `/de/preise/`, `/de/metallpreise/`, `/de/preisliste/` liefern **HTTP 500**; die Sitemap (`/sitemap.xml`, 20 URLs) enthält **keine Preisseite** |
| metall.alba.info | 21 | `/ankauf/sorten/` listet nur „Diese Schrotte können Sie bei uns abgeben“, **kein** €, kein EUR, keine Preistabelle |
| tsr-recycling.de | 16 | jede URL liefert eine 1.873-Byte-„Challenge Validation“-Seite (JavaScript-Bot-Challenge) |
| proaurum.de, schrott24.de, steil.de, isr-recycling.de, ziems-recycling.de, jakob-becker.de, kunze-group.de, riwald.de, goetz-neu-ulm.de, rmr-recycling.de | 5–10 je Gruppe | an den üblichen Pfaden (`/preise/`, `/metallpreise/`, `/tagespreise/`, `/ankauf/preise/`, `/preise/metall/`, `/schrottpreise/`, `/aktuelle-preise/`, `/de/preise/`) **keine erreichbare Preisseite mit Preisangaben**; `schrott24.de/preise/` ist ein Blog über LME-Kursbewegungen, keine Preisliste |
| mcschrott.de | 1 | Homepage nennt neunmal €/kg, **einziger** Treffer der Stichprobe |

Von 22 geprüften Domains zeigte **eine** erkennbare €/kg-Angaben. Das Verhältnis ist damit
kein Zufall: Die großen Betreiber betreiben Preislisten über B2B-Portale, Kundensysteme
oder telefonische Abfragen, nicht über öffentliche Seiten.

## Konsequenz für die Backlog-Planung

1. Der **Handler-Backlog ist erheblich kleiner** als die Domains-Zahl vermuten lässt. Vor
   jeder neuen Handler-Arbeit ist die **Preisseite** zu belegen: URL erreichbar, Preis,
   Einheit und Datum maschinenlesbar, Materialbezeichnungen eindeutig.
2. **Betreiber mit Filialfinder** (ALBA, Scholz, TSR, Pro Aurum, Theo Steil, ISR) lohnen
   einen Sammel-Handler erst, wenn eine Preisquelle existiert. Derzeit gibt es keine.
   Der Hebel ist dort nicht der Handler, sondern die **Preisquelle** (öffentliche
   Preisliste, offene API, Marktplatzanbindung).
3. **Einkaufsseitige Betreiber** (schrott24.de) veröffentlichen bewusst keine Preise
   („Preis auf Anfrage“); für sie ist ein Handler fachlich nicht sinnvoll.
4. Die belastbare Restmenge sind **Einzelbetreiber mit eigener Preisseite**. Sie ist
   domainweise zu prüfen (Screening wie oben), und jeder Treffer ist mit echtem
   HTML-Fixture nach `docs/adding-a-trader-handler.md` umzusetzen.

## Empfohlene Reihenfolge

1. Screening der Domains mit 2–4 Händlern (der 1.365er-Gruppe) auf eine erreichbare
   Preisseite; Ergebnis als Liste der **eligible** Domains dokumentieren.
2. Für die ersten Treffer je Handler: Fixture-Test, Live-Prüfung mit
   `cargo run -p schrott-mcp-ingestion --example live_handlers <slug>`, dann Registrierung.
3. Für Betreiber ohne Preisquelle: **kein** Handler, sondern eine dokumentierte
   Preis-Quelle-Recherche (was der Betreiber veröffentlicht, wo, in welchem Format).

## Abgrenzung

- Diese Liste ist **kein** Freigabe- und kein Deploy-Dokument.
- Ein neuer Handler braucht nach `docs/adding-a-trader-handler.md` ein echtes HTML-Fixture
  mit Materialbezeichnung, Preis, Einheit und Datum, dann die Verdrahtung mit
  `Schedule::every_6h()`; die Skip-Regeln müssen lauten, nicht raten.
- `website_status: aktiv` belegt nur, dass die Domain erreichbar war, nicht dass eine
  scrape-fähige Preisseite existiert.
