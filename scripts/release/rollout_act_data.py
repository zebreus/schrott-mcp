#!/usr/bin/env python3
"""Die **Werte** ✓✓ der Rollout-Akte ✓✓ aus dem Rollout-Manifest ✓✓ lesen ✓✓

Nach einem Rollout ✓✓ liegen die **gemessenen** ✓✓ Werte ✓✓ im
Backup-Ordner ✓✓ (`/root/Documents/schrott-mcp-rollout-backups/rollout-<short>-<stempel>`) ✓✓
— im `manifest.json` ✓✓ und im `post-rollout-verification.json` ✓✓ ✓ ✓

Dieses Werkzeug ✓✓ druckt sie als **Markdown** ✓✓ ✓ ✓ **Read-only** ✓✓ ✓ ✓

Beispiel:
    python3 scripts/release/rollout_act_data.py \
        /root/Documents/schrott-mcp-rollout-backups/rollout-8946797-20261124T125000Z
"""
import json
import sys
from pathlib import Path


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    ordner = Path(sys.argv[1])
    manifest = json.loads((ordner / "manifest.json").read_text(encoding="utf-8"))
    pruefung = json.loads(
        (ordner / "post-rollout-verification.json").read_text(encoding="utf-8"))

    tests = (manifest.get("workspace_tests") or {}).get("summary") or {}
    print("**Backup:** `%s`" % ordner)
    print()
    print("| Feld | Wert |")
    print("|---|---|")
    print("| `release_source_commit` | `%s` |" % manifest.get("release_source_commit"))
    print("| `release_source_tree` | `%s` |" % manifest.get("release_source_tree"))
    print("| Binary `schrott-mcp-server` | `%s` |" % (
        (manifest.get("release_binary_sha256") or {}).get("server")))
    print("| Binary `schrott-mcp-query-worker` | `%s` |" % (
        (manifest.get("release_binary_sha256") or {}).get("query_worker")))
    print("| Workspace-Tests | **%s** Suiten · **%s** passed · **%s** failed · **%s** ignored |" % (
        tests.get("suites"), tests.get("passed"), tests.get("failed"), tests.get("ignored")))
    print("| Der Dienst **vor** dem Rollout | `%s` (Start `%s`) |" % (
        manifest.get("pre_rollout_service_state"), manifest.get("pre_rollout_service_start")))
    print("| `production_write` | `%s` |" % manifest.get("production_write"))
    print("| Das **Backup** von `public.db` | %s Bytes · sha256 `%s` |" % (
        ((manifest.get("database_backups") or {}).get("public.db") or {}).get("size_bytes"),
        ((manifest.get("database_backups") or {}).get("public.db") or {}).get("sha256")))
    print("| Das **Backup** von `internal.db` | %s Bytes |" % (
        ((manifest.get("database_backups") or {}).get("internal.db") or {}).get("size_bytes")))
    print("| Der Rollout **nach** dem Neustart | `%s` |" % pruefung.get("checked_utc"))
    print("| Die **Zielzeilen** | **%s** |" % len(pruefung.get("target_rows") or []))
    print("| Die Punkte **entfernt** | **%s** |" % len(pruefung.get("geo_cleared") or []))
    print("| Die Punkte **noch da** | **%s** |" % (pruefung.get("geo_still_present") or []))
    print("| `dropoff_json` **nicht** leer | **%s** |" % (
        pruefung.get("dropoff_not_allowed") or []))
    zaehl = pruefung.get("counts") or {}
    print("| Die **Zählwerte** | traders **%s** · materials **%s** · current_prices **%s** · user_version **%s** |" % (
        zaehl.get("traders"), zaehl.get("materials"), zaehl.get("current_prices"),
        zaehl.get("user_version")))
    print("| Die **Verletzungen** der Fremdschluessel | **%s** |" % (
        zaehl.get("foreign_key_violations")))
    return 0


if __name__ == "__main__":
    sys.exit(main())
