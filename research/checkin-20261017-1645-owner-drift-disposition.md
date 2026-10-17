# Owner-Disposition der aktuellen Upgrade-Drift

17.10.2026. Den vollständigen [176-Zeilen-Bericht](checkin-20261017-1630-upgrade-drift.md) gelesen. Manifest `/root/Documents/upgrade-drift-20261017-1630/manifest.json`, SHA256 `4286d438e1decd86e1af73f7224f1b587874179e778c0fd524d3e84ba4df35f1`, selbst gegen die Bindung und alle **4524** Dateien auf Größe/SHA256 nachgeprüft. Tatsächliche Baselineäquivalenz, Seedwirkung, Kontaktentscheidungen und sämtliche elf geänderten hypothetischen Insert-IDs gelesen. Bericht ist an den gepushten Buildinput `ccd32e29` gebunden; privater synchroner Rustharness ist nicht das endgültige Serverartefakt.

## Zusätzlich zur alten engen Disposition akzeptiert

- **751 Updates/12 Inserts** statt715/12: genau36 zusätzliche Updates aus der geprüften 60er-Welle. 58 Notizfälle und die bereits einzeln freigegebenen Skalarfälle DKE/Edelmetallkontor erklären exakt60 logische Payloadunterschiede zum Kontrollseed. Keine zusätzliche Adresse, GEO-, Status-, Kontakt- oder Serviceänderung aus dieser Welle.
- Genau sechs weitere Metadatenentfernungen `seed_file` bei Ruft/Großlangheim, Rissel/Höchstadt, Kupferdantler/München, Sindlhauser/Roding, Menche/Marburg und Edelmetallkontor/Idar-Oberstein: insgesamt159 statt153. Der eigentliche recherchierte Inhalt und die Herkunft im Dossier bleiben erhalten. Die aktuelle Metadaten-Ersetzungssemantik ist akzeptiert; keine weiteren verschwundenen Schlüssel im Zusatzdiff.
- Trader-Autoincrementsequenz +36 gegenüber dem Kontrolllauf, dadurch elf anders vergebene IDs der zwölf **noch nicht existierenden** Händler. Keine bestehende ID/`first_seen_at` verändert, kein bestehender Preis-/Annahmebezug umgehängt. Die IDs eines nie deployten Gegenlaufs sind keine versprochene öffentliche Identität. Sämtliche Slugs bleiben gleich. Der Rohdiff und die konkreten elf Slug-/ID-Paare bleiben als Beleg erhalten.

Die bereits akzeptierten drei leeren additiven Tabellen, 29 Materialdefinitionen, 83 Materialzeitstempel-/Sequenzrefreshes pro Vollseed und exakt24 vorbestehenden FK-Verletzungen bleiben begrenzter, ausdrücklich dokumentierter Umfang. Keine FK-Reparatur, Löschung oder Neuzuordnung freigegeben. Aktuelle sieben Kontaktfelder Fernau/Graf/Willeführ sind tatsächlich leer; leere Seeds bedeuten Keep und keine dauerhafte Löschgarantie.

Die tatsächliche Produktionsdrift seit der alten Kopie ist anhand ihres separaten Vorhervergleichs regulärer Betrieb und keine Seedwirkung. Durch Open/Seed selbst bleiben alle **21147 Preise, 2426 Currentpointer, 1315 Annahmen** und sämtliche Internalzeilen unverändert.

## Noch kein Installations-PASS

Die aktuelle fachliche/technische Seeddrift ist damit akzeptiert, nicht jedoch ein ungeprüfter neuer Zustand. Gesondert beauftragt ist ein tatsächlicher kurzer normaler Boot des fertig hashgebundenen Release-Serverartefakts auf neuen privaten Arbeitskopien, mit Health, geordnetem Prozessende und vollständigem Vergleich. Die bereits manifestierten Belege werden dafür nicht verändert. Die frühere echte private Hofmanntransition und Sourceidentität sind getrennt vom heutigen Seedharness zu belegen.

Vor einem späteren Produktionsstart bleiben zeitnahe Sicherung und Writerexklusivität nach Stopp erforderlich. Nach Installation folgen Health/Binaryhashes und die regulären Scheduler-/Persistenzprüfungen einschließlich Hofmannledger und MKM-Artikelownership. Kein erzwungener Produktionslauf, keine direkt schreibende SQL-Abfrage, kein automatischer alter Binary-/DB-Restore.
