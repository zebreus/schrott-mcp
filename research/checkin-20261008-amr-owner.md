# Owner-Check-in — AMR-Abrufstörung, 08.10.2026

Service aktiv, sechs Stunden Warning-Journal ohne Einträge. 3879 Händler,
52 Materialien, 2280 aktuelle Preise. Keine neuen Feedbacks seit #5517;
letzte zehn erneut geprüft, offene Händler-Leads nicht pauschal erledigt.
Working Tree zu Beginn sauber. NORDKAT nach Reparatur weiterhin ohne neuen
regulären Schritt; kein Erfolg aus dem früheren Test erfunden.

Letzter Lauf 1146 partial: 105 Preise übernommen, ein Händlerfehler.
Fehler auf AMR beschränkt, HTTP 502; Läufe 1143–1145 sonst ok.

## Reproduktion und Gegenprüfung

Red-capable Abrufkommando, tatsächlich ausgeführt:

```sh
curl --fail --location --silent --show-error --max-time 25 \
  -o /tmp/opencode/amr-checkin-body.html \
  -w 'HTTP %{http_code}; final URL %{url_effective}\n' \
  http://www.amr-schrottplatz.de
```

Ausgabe: curl 22, HTTP 502, finale URL http://www.amr-schrottplatz.de/.
Drei vor Gegenproben benannte Möglichkeiten: Quellstörung, HTTPS-Wechsel,
www-/Hostwechsel. Gleicher HTTP-Abruf ohne www ebenfalls 502. HTTPS mit
Zertifikatsprüfung liefert curl 60, certificate has expired, HTTP 000.
Der HTTPS-Kandidat ist also kein sicher freigegebener Fallback. Kein -k,
keine Zertifikatsumgehung und keine neue URL bloß auf Vermutung umgestellt.

Konkrete Grenze: Betreiberquelle derzeit über die geprüften Varianten
nicht nutzbar. Wir können weder den entfernten Webserver noch sein TLS-
Zertifikat reparieren; eine präzisere Ursache des HTTP 502 ist nicht belegt.
Handlerfehler bleibt sichtbar, kein stilles Weiterreichen erfundener Preise.
Aktuellen Inhalt/Preise konnten diese Abrufe nicht verifizieren.

## Ergebnis

Technischen Nachtrag im bestehenden AMR-Dossier ergänzt, Historie und
Frontmatter unverändert. Keine Geschäftsschließung aus Abrufstörung
abgeleitet. Das historische aktiv ist keine heutige Liveverifikation.
Keine direkte DB-Änderung und keine unnötige Rust-/Handleränderung.
Bekannte Agentenlimits nicht durch Wiederholungsstarts belastet;
weitere Händler-, Geo- und Handlerfälle bleiben im dokumentierten Backlog.

Owner-Gate: YAML-Frontmatter unverändert (damit Enums, Root-Website,
Service-JSON und Koordinaten nicht überschrieben), alle alten Timeline-
Bullets erhalten, Diffcheck/Rustfmt grün; acht Seedtests bestanden.
