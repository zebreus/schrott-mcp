# Owner-Check-in 14.10.2026 — BERK-/TSR-Follow-ups

## Frischer Check-in

Anleitung gelesen; zehn jüngste Feedbacks read-only abgefragt, IDs
7381–7390 unverändert. Service aktiv, HTTPS-Health ok; 3881 Händler,
54 Materialien, 2426 Preise. Keine systemd-Prioritätswarnungen/Geo-Logs.
App-JSON weiterhin Hofmann (noch alte Produktion), Münster-Preisleiter,
AMR HTTP502. Keine erzwungene Ingestion oder Produktionswrites.

## BERK — Owner-Quellenprüfung und begrenzte Freigabe

Agentenbericht vollständig gelesen. Selbst erneut per HTTPS mit normaler
TLS-Prüfung abgerufen und inhaltlich gelesen: Impressum, Kontakt,
Kevelaer-Schrotthandelsdetail, Sitemap und NorthData-Unternehmensdetail.
Echte Inhalte, keine Challenge oder bloße Statuscode-Prüfung. Temporäre
Leseartefakte: `/tmp/opencode/owner-tsr-berk-20261014/berk-*.txt`.

Registergegencheck bestätigt Firma, HRB18672, Marius Kohl und
Geschäftsadresse Velderdyck 21/47625. Betreiberadresse Velder Dyck 21/47624.
Owner übernimmt Straße, Mail, Zeiten und entfernt ausschließlich die
unbelegte Schlussklammer des Telefons. **PLZ bleibt wegen Konflikt leer**.
Keine Umfirmierungs-/Fusionsbehauptung zu Kohl Containerdienst/EGN.
Keine Zertifizierungen oder BImSchG-Genehmigung als geprüft erfasst.

Für eigene Standortfakten wird die README-Primärquellen-Ausnahme akzeptiert:
registerkongruentes Impressum; lebende Registergesellschaft; echte
per-site Adresse/Telefon/Zeiten; spezifische datierte Sitemapänderungen
27.09.2026 (Schrotthandelsdetail) und 05.10.2026 (Container Kevelaer).
Letztere sind ausdrücklich Betreiber-Pflegesignale, kein Vor-Ort-Audit.
Owner hat **keinen** eigenen amtlichen AD/CD-Abruf behauptet; Agentenbefund
und eigene NorthData-Gegenprüfung bleiben getrennt. Gleichnamige
Grefrath-Niederlassung mit anderen Zeiten/Nummer nicht übertragen.

Bestehende DB-GEO read-only 51.58322/6.24828, kein Koordinatenpaar im
Dossier ergänzt. Neuer Seed erhält bisherige GEO; die Kontaktfreigabe
validiert keinen Kartenpunkt. Ankauf/Verkauf/Gebühren bleiben getrennt:
keine Zahlen-Ankaufsliste, Containertransport nicht allgemein gratis,
Shop nicht ausgewertet, Zertifikatsumfang weiterhin offen.

## TSR — clientabhängiger Zugriff und abschließende begrenzte Freigabe

Agentenbericht vollständig gelesen: Recycling→Group, eigenständiges
Deutschland; drei konkret verlinkte Zertifikate und genaue Beleggrenzen.
Owner-Nachabruf mit User-Agent `schrott-mcp-ingestion/0.1` ergibt aber:
Standort/Impressum/Group-News HTTP200 mit **nur Challenge Validation**,
keinen Betriebsinhalt; alle drei PDF-URLs HTTP403. HTTP200 allein ist
keine erfolgreiche Quellenprüfung. GLEIF-API tatsächlich gelesen,
Deutschland HRA19743 ACTIVE/ISSUED, Update 28.05.2026; das ist kein
Neumünster-Annahmestellenbeleg.

Nachauftrag abgeschlossen, vollständigen Bericht samt Beleganhang gelesen.
Agent sichert echte Antworten/Extrakte in
`/tmp/opencode/tsr-owner-evidence-20261014/`. Owner reproduziert danach
selbst mit gewöhnlichem Python-urllib-Standardclient echte HTTP200-Inhalte:
Standort, Impressum und alle drei Original-PDFs, gespeichert als
`/tmp/opencode/owner-tsr-berk-20261014/tsr-standard-*.body` plus Extrakte.
Keine TLS-/Challenge-/Sperrenumgehung; normale Server-/CDN-Caches sind
kein Beweis eines frisch gerenderten Ursprungsinhalts. Der dokumentierte
Ingestion-Client-Gegenbefund bleibt bestehen, website_status blockiert bleibt.

Owner liest Standortdaten, technischen veröffentlichten Mail-Empfänger,
EFB-Titel/Neumünster-Anlagen, ISO-Titel/Standort7 und A|U|F-Neumünster-Eintrag.
Eigenständige aktuelle Deutschland-Gesellschaft zusätzlich mit tatsächlich
gelesenem NorthData-Unternehmensdetail und GLEIF gegengeprüft. Aktueller Name,
Straße, PLZ, Telefon, Mail und Zeiten nach README-Ausnahme freigegeben.
Recycling HRA18511→Group und Deutschland HRA19743 bleiben getrennt;
vollständiger Ausgliederungsvertrag/konkreter historischer Standorttransfer
nicht behauptet. Zertifikatsgültigkeit/örtlicher Umfang in Timeline begrenzt,
keine ElektroG-/Altauto-Konzernanlage auf Neumünster übertragen.
DB-GEO read-only 54.082725/9.988375, keine Koordinaten im Dossier ergänzt;
Kontaktfreigabe bestätigt keine bisherigen Koordinaten.

## Gate / Veröffentlichung

Nur zwei Dossiers, zwei zugehörige abgeschlossene Agentenberichte und dieser
Ownerbericht für Commit vorgesehen. Fremde Dossierarbeit bleibt unberührt.
Abschließendes Gate bestanden: exakt vier BERK- und sechs TSR-Felder geändert,
beide vollständigen alten Timelines erhalten, Slugs/Status/Website-Status und
Typen unverändert, keine GEO-/clear_fields-Ergänzungen. Alle 155 geschützten
fremden Pfade SHA256-unverändert; Index vor Staging leer, scoped diff --check
sauber. Quellen-/Deep-Link- und Zertifikatsgrenzen wie oben dokumentiert.
`cargo test -p schrott-mcp-ingestion seed`: 14 bestanden, 0 fehlgeschlagen,
647 ausgefiltert, nach beiden Dossieränderungen. Kein Rust geändert;
dieses Seed-Gate ist kein neuer vollständiger Workspace-/Produktionslauf.
Danach scoped Commit und tatsächlicher Push-Versuch.
Kein Redeploy oder Release-Spot vorweggenommen. Git-Authentifizierung
war bisher konkret blockiert; Hofmann-Korrektur bleibt lokal, nicht produktiv.

Tatsächlicher Push dieses Check-ins: `git -c credential.interactive=false
push origin main` endet mit Exit128, `fatal: unable to get password from user`.
Änderungen lokal in main committet, nicht veröffentlicht; kein Redeploy.
