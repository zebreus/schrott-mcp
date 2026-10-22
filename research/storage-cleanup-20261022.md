# Speicher-Audit und Bereinigung — 22.10.2026

## Ergebnis

- Auf `/` stieg der freie Speicher von **23 GiB auf 58 GiB** (netto etwa
  **35 GiB**); `/tmp` sank von 1,1 GiB auf 27 MiB belegt.
- `cargo clean` entfernte 13.921 Builddateien (**12,6 GiB**) aus dem
  `target/` des Haupt-Worktrees. Der Quell-Worktree und `.git` blieben erhalten.
- Unter `/root/Documents` wurden 127 alte Check-in-, Recherche-, Seed-,
  Release- und Backup-Verzeichnisse sowie 184 zugehörige Dateien entfernt
  (vorher zusammen ungefähr 17,2 GiB). Diese Verzeichnisse waren keine
  registrierten Git-Worktrees. Die einzigen registrierten Worktrees waren und
  sind `/root/Documents/schrott-mcp`.
- Weitere entfernte Schrott-MCP-Artefakte: sieben alte Root-Test-/Review-
  Verzeichnisse (ca. 5,9 GiB), vier veraltete Recherche-Caches (ca. 245 MiB),
  966 Einträge in `/tmp/opencode`, 31 temporäre Hofmann-Testverzeichnisse,
  263 alte Schrott-Test-/Quellartefakte in `/var/tmp`, zehn alte
  Schrott-Backup-Einträge in `/var/backups` und sechs alte
  `pre-geocoding`-Backup-/Sidecar-Dateien in `/var/lib/schrott-mcp`.
- Nachher enthält `/var/lib/schrott-mcp` weiterhin die produktiven
  `internal.db` und `public.db` sowie deren aktuellen SQLite-WAL-/SHM-Dateien.
  Es wurden keine Produktionsdatenbanken beschrieben; ausschließlich alte
  Backupkopien wurden gelöscht.

## Bewusst erhalten

- Der aktive `schrott-mcp.service`, sein Binary, Worker, Systemd-/Nginx-
  Konfiguration und Zertifikate sowie Check-in-/Systemlogs.
- `offsite-data` (3,9 GiB): eigenständiges Projekt mit aktivem
  `offsite-data.service`; dessen `target/` wurde nicht gelöscht, um einen
  späteren Dienstneustart nicht zu beschädigen. Ebenfalls unangetastet blieben
  die unabhängigen Verzeichnisse `therapie-mcp` und `opencode-scheduler-v2`.
- In `/tmp/opencode` blieben nur ein Therapie-Scratch-Verzeichnis und die
  mögliche OpenCode-Konfigurationssicherung `opencode.json.pre-ultracode.bak`;
  globale OpenCode-Sitzungsdaten und Toolchain-Caches blieben ebenfalls erhalten.

## Verifikation

- `schrott-mcp.service` aktiv; öffentlicher Health-Endpunkt antwortete mit
  `{"ok":true,"service":"schrott-mcp"}`.
- Produktionszählung read-only: 3.893 Händler, 83 Materialien, 2.683 aktuelle
  Preise.
- Keine zusätzlichen registrierten Schrott-MCP-Worktrees. Der Haupt-Worktree
  behielt seine bestehenden Änderungen. Keine Commit-, Push-, Deploy- oder
  Seed-Aktion.
