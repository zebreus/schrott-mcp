# Handler-Backlog, quantifiziert (Stand 27.10.2026, 12:30 UTC)

Read-only Auswertung der Produktionsdatenbank (`mode=ro`, `PRAGMA query_only=ON`) gegen
`crates/ingestion/src/traders/handlers/`. Keine Datei- und keine Produktionsänderung.

## Ausgangslage

- **3.936** Händler in der Produktion, **128** Handler in `crates/ingestion/src/traders/handlers/`
  (133 Dateien inklusive Modulen ohne eigene `handler()`).
- **1.773** Händler haben eine Website mit `website_status: aktiv`.
- Davon haben **1.636** keinen eigenen Handler (kein Slug im Handler-Quelltext referenziert).
- Diese 1.636 verteilen sich auf **1.390** Domains. Nur **9** Domains sind bereits durch
  einen Handler abgedeckt (20 Trader, meist Filialen desselben Betreibers).
- **Echter Backlog: 1.381 Domains mit zusammen 1.616 Händlern**, jeweils ohne Handler.

Die Dispositionslogik ist die gleiche wie bei den Dossiers: `website_status: aktiv` belegt
nur, dass die Domain erreichbar war, nicht dass eine scrape-fähige Preisseite existiert.
Ein Handler ist erst sinnvoll, wenn eine öffentlich erreichbare, maschinell lesbare
Preis- oder Angebotsseite mit eindeutigem Material und Preis vorliegt.

## Priorisierung nach Händlerzahl je Domain

Ein Handler pro Betreiber deckt alle Filialen desselben Auftritts mit. Die größten Lücken
sind daher Betreibergruppen, nicht Einzelhändler:

| Domain | Händler | Anmerkung |
|---|---|---|
| scholz-recycling.com | 33 | Großbetreiber mit Standort-Filialfinder; Preise je Standort, Sammel-Handler sinnvoll |
| metall.alba.info | 21 | ALBA Metall Nord, Standortliste je Filiale; Sammel-Handler analog zum bestehenden Muster |
| tsr-recycling.de | 16 | TSR Deutschland, Standorte |
| proaurum.de | 10 | Pro Aurum, bundesweit |
| steil.de | 8 | Theo Steil, mehrfach |
| isr-recycling.de | 8 | |
| schrott24.de | 7 | Online-Ankauf, Abholung |
| ziems-recycling.de | 6 | |
| jakob-becker.de | 6 | Jakob Becker Gruppe |
| riwald.de | 5 | |
| kunze-group.de | 5 | |
| goetz-neu-ulm.de | 4 | |
| rmr-recycling.de | 4 | |
| foell-rohstoffhandel.de | 4 | |
| fegert-recycling.de | 4 | |
| kuehl-entsorgung.de | 4 | |

Die restlichen 1.365 Domains haben je ein bis drei Händler; sie sind einzeln zu prüfen und
lohnen einen Handler nur bei vorhandener Preisseite.

## Empfohlene Reihenfolge für den nächsten Handler-Zyklus

1. **Großbetreiber mit Filialfinder** (Scholz, ALBA Metall Nord, TSR, Pro Aurum, Theo
   Steil): ein Handler über die Standortliste, Preise je Filialseite. Das schließt rund
   88 Händler mit fünf Handlern.
2. **Einzelbetreiber mit erkennbarer Preisseite** aus der 1-3-Händler-Gruppe, priorisiert
   nach Ausschreibungshäufigkeit und Materialabdeckung.
3. **Keine Handler** für Domains ohne lesbare Preisquelle, auch wenn `website_status: aktiv`
   gesetzt ist; dort bleibt der Standort eine reine Recherchezeile im Timeline.

## Abgrenzung

- Diese Liste ist **kein** Freigabe- und kein Deploy-Dokument. Ein neuer Handler braucht
  nach `docs/adding-a-trader-handler.md` zuerst ein echtes HTML-Fixture mit
  Materialbezeichnung, Preis, Einheit und Datum, dann die Verdrahtung mit
  `Schedule::every_6h()`, und die Skip-Regeln müssen lauten, nicht raten.
- Für jeden Handler gilt dieselbe Quellenhierarchie wie in den Dossiers: Betreiber-Primärquelle
  zuerst, Preisquelle mit Datum und Einheit, Material ohne Preis wird übersprungen und
  laut gezählt.
