---
slug: be-treptow-kopenick-amr-metalle-und-rohstoffe-amr-schrottpla
name: AMR Metalle und Rohstoffe GmbH (AMR Schrottplatz)
trader_type: schrottplatz
state: BE
city: Treptow-Köpenick
street: Schnellerstr. 20E
postcode: '12439'
phone: 030 53013220
email: kontakt@amr-schrottplatz.de
opening_hours: Mo-Fr 8:00-17:00, Sa 9:00-13:00
website: http://www.amr-schrottplatz.de
website_status: aktiv
status: aktiv
description: ''
dropoff_json: ''
pickup_json: ''
provenance_section: Schrottankauf Berlin (BE) — Recherche
provenance_ankauf_raw: ja
provenance_origin: table
---

# AMR Metalle und Rohstoffe GmbH (AMR Schrottplatz)

## Überblick

_Noch kein Überblick — bei nächster Welle aus description/notes kuratieren._

## Timeline

### Technische Quellenprüfung 08.10.2026

- [Recherche 08.10.2026: Regulärer Ingestion-Lauf 1146 fehlgeschlagen: HTTP 502 von http://www.amr-schrottplatz.de. Owner reproduziert den Fehler unabhängig per curl --fail --location: sowohl www- als auch nackte HTTP-Domain liefern 502; HTTPS-www scheitert vor Abruf an abgelaufenem TLS-Zertifikat (curl 60). Daher keine sichere bestätigte Ersatz-URL, keine TLS-Prüfung abgeschaltet und kein Parser-/Preisfallback erfunden. Dies ist eine technische Abrufstörung, kein Nachweis einer Betriebsschließung oder neuer Preise. Stammdaten und frühere Verifikation erhalten; Zeiten/Kontakte heute nicht erneut bestätigt. Quellen: http://www.amr-schrottplatz.de/ ; http://amr-schrottplatz.de/ ; https://www.amr-schrottplatz.de/ ; /var/lib/schrott-mcp/internal.db, ausschließlich SELECT zu Lauf 1146.]

### Importiert (Seed-Stand 2026-09-30)

- Altmetall + Hartmetall + Demontage, Container
- Adresse: Schnellerstr. 20E, 12439 Berlin
- Bezirk: Treptow-Köpenick Adressbeleg: ISH-/AMR-Komplex, Detailseite (be.md Runde 3/4)

### Recherche 01.10.2026

- [Recherche 01.10.2026: Betreiber-Website amr-schrottplatz.de verifiziert (Start + Impressum/Datenschutz): Schnellerstr. 20E, 12439 Berlin und Tel. 030 53013220 deckungsgleich (DB-Schreibweise 030 53 013 220 identisch); E-Mail kontakt@amr-schrottplatz.de (DB-gleich) und Zeiten Mo-Fr 8:00-17:00, Sa 9:00-13:00 gefüllt; HRB 162848 (AG Charlottenburg) zweitbelegt; Privat-/Geschäftskunden nur Timeline-Vermerk; Quelle(n): http://www.amr-schrottplatz.de + http://www.amr-schrottplatz.de/impressum-und-datenschutz/]

### Fortgesetzte technische Prüfung 12.10.2026

- [Prüfung 12.10.2026: Der reguläre Ingestion-Lauf 1449 scheiterte am 12.10. um 04:46:58 UTC erneut mit HTTP 502 von http://www.amr-schrottplatz.de; ein aktueller HTTPS-Abruf ließ sich ebenfalls nicht verifizieren. Read-only-Abfrage der öffentlichen `current_prices`/`prices` zeigt weiterhin vier AMR-Zeilen, deren jüngste Beobachtung vom 07.10.2026 10:29:14 UTC stammt. Diese Preise sind damit möglicherweise veraltet; die Produktionsdatenbank blieb strikt read-only und die Werte wurden nicht verändert. Keine belastbare Ersatz-URL belegt; frühere Prüfung vom 08.10. hält den HTTP-502 beider HTTP-Hostvarianten und das abgelaufene TLS-Zertifikat des HTTPS-www-Endpunkts fest. Daraus wird weder eine Betriebsschließung noch eine sichere aktuelle Preisliste abgeleitet. Quelle(n): http://www.amr-schrottplatz.de/ ; Ingestion-Lauf 1449 und SELECT auf `/var/lib/schrott-mcp/internal.db` sowie `/var/lib/schrott-mcp/public.db`, Abruf 12.10.2026; `research/checkin-20261008-amr-owner.md`.]
- [Nachprüfung 12.10.2026: Der reguläre Ingestion-Lauf 1473 (zwischen 10:49:50 und 10:49:54 UTC) protokollierte erneut HTTP 502 von http://www.amr-schrottplatz.de und endete mit einem Händlerfehler. Die read-only-Verknüpfung von `current_prices` mit `prices` ergab weiterhin vier aktuelle AMR-Zeilen; die jüngste zugrunde liegende Beobachtung stammt vom 07.10.2026 10:29:14 UTC. Damit liegt kein neuer AMR-Preisnachweis vor; die gespeicherten Werte können veraltet sein. Produktionswerte blieben unverändert. Der Lauf 1473 sowie beide Produktionsdatenbanken wurden ausschließlich lesend geprüft; keine Ersatz-URL oder aktuelle Preisliste ist belegt. Quelle(n): http://www.amr-schrottplatz.de/ ; Ingestion-Lauf 1473 und SELECT auf `/var/lib/schrott-mcp/internal.db` sowie `/var/lib/schrott-mcp/public.db`, Abruf 12.10.2026.]
