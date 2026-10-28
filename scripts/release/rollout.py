#!/usr/bin/env python3
"""Rollout of the reviewed candidate release f40d1dbb (only after a green gate).

Sequence: backup (read-only source, SQLite backup API) -> install binaries ->
restart service -> wait for health -> wait for a natural scheduler run ->
read-only verification of every target row. No direct production DB writes:
all data corrections come from the seed inside the restarted service.
"""

import hashlib
import json
import os
import re
import shutil
import sqlite3
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]  # Repo-Wurzel
SHORT = os.environ.get("RELEASE_SHORT", "0770ad70")
PROBE = ROOT / f"target/predeploy-{SHORT}"
BUILD = PROBE / "build/release"
PROD = Path("/var/lib/schrott-mcp")
BIN_DIR = Path("/usr/local/bin")
BACKUP_ROOT = Path("/root/Documents/schrott-mcp-rollout-backups")
COMMIT = os.environ.get("RELEASE_COMMIT", "0770ad70ab9ddef47837c99f632195ff67451856")
TREE = os.environ.get("RELEASE_TREE", "")
PROBE_MANIFEST = PROBE / "predeploy-probe-manifest.json"
SERVER = BUILD / "schrott-mcp-server"
WORKER = BUILD / "schrott-mcp-query-worker"


def gate_check() -> dict:
    """Fail fast unless the reviewed artifacts still match the candidate."""
    problems = []
    manifest = json.loads(PROBE_MANIFEST.read_text())
    if manifest.get("repository_head") != COMMIT:
        problems.append(f"probe manifest head {manifest.get('repository_head')} != {COMMIT}")
    if TREE and manifest.get("repository_tree") != TREE:
        problems.append(f"probe manifest tree {manifest.get('repository_tree')} != {TREE}")
    binding = manifest.get("build_binding", {})
    expected = binding.get("binary_sha256", {})
    if expected.get("server") != sha256(SERVER):
        problems.append("server binary digest differs from the reviewed manifest")
    if expected.get("query_worker") != sha256(WORKER):
        problems.append("query worker binary digest differs from the reviewed manifest")
    if not binding.get("tree_binding", {}).get("identical_to_commit"):
        problems.append("exported tree is not identical to the candidate commit")
    fmt = binding.get("format_check", {})
    if fmt.get("exit_code") != 0 or not fmt.get("passed"):
        problems.append("format check in the reviewed manifest did not pass")
    if manifest.get("first_boot", {}).get("seed_rows_written") is None:
        problems.append("no first-boot seed measurement in the reviewed manifest")
    if manifest.get("second_boot", {}).get("seed_rows_written") != 0:
        problems.append("second boot was not idempotent in the reviewed manifest")
    if not manifest.get("effective_delta", {}).get("all_changes_explained_by_seed"):
        problems.append("reviewed manifest contains unexplained changes")
    if not manifest.get("clear_directive_check", {}).get("all_clears_applied"):
        problems.append("reviewed manifest contains clear-directive violations")
    if not manifest.get("query_worker", {}).get("all_targets_read"):
        problems.append("query worker did not read all target rows in the reviewed manifest")
    if not manifest.get("production_integrity", {}).get("production_untouched"):
        problems.append("reviewed manifest does not certify an untouched production")
    return {"problems": problems, "probe_manifest_sha256": sha256(PROBE_MANIFEST)}

TARGETS = json.loads(Path(os.environ["RELEASE_TARGET_SLUGS"]).read_text())


def now() -> str:
    return datetime.now(timezone.utc).isoformat()


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def sh(cmd: list[str]) -> str:
    return subprocess.check_output(cmd, text=True).strip()


def db_counts(path: Path) -> dict:
    conn = sqlite3.connect(f"file:{path}?mode=ro", uri=True)
    conn.execute("PRAGMA query_only=ON")
    out = {}
    for t in ("traders", "materials", "prices", "current_prices", "trader_materials"):
        try:
            out[t] = conn.execute(f"SELECT COUNT(*) FROM {t}").fetchone()[0]
        except sqlite3.Error:
            out[t] = None
    try:
        out["user_version"] = conn.execute("PRAGMA user_version").fetchone()[0]
    except sqlite3.Error:
        out["user_version"] = None
    out["foreign_key_violations"] = len(conn.execute("PRAGMA foreign_key_check").fetchall())
    conn.close()
    return out


def backup_database(name: str, dest: Path) -> dict:
    src = sqlite3.connect(f"file:{PROD/name}?mode=ro", uri=True)
    src.execute("PRAGMA query_only=ON")
    if dest.exists():
        dest.unlink()
    dst = sqlite3.connect(dest)
    src.backup(dst)
    dst.close()
    src.close()
    return {
        "sha256": sha256(dest),
        "size_bytes": dest.stat().st_size,
        "counts": db_counts(dest),
        "source": "SQLite online backup API from a mode=ro production connection; no production write",
    }


def test_summary(log: Path) -> dict:
    """Aggregate the workspace test log into explicit counters."""
    pattern = re.compile(
        r"^test result: .*?(\d+) passed; (\d+) failed; (\d+) ignored;")
    passed = failed = ignored = 0
    suites = 0
    for line in log.read_text().splitlines():
        match = pattern.match(line.strip())
        if match:
            suites += 1
            passed += int(match.group(1))
            failed += int(match.group(2))
            ignored += int(match.group(3))
    return {"suites": suites, "passed": passed, "failed": failed, "ignored": ignored}


def main() -> int:
    gate = gate_check()
    if gate["problems"]:
        print(json.dumps({"gate": "BLOCKED", "problems": gate["problems"]}, indent=2))
        return 2
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    out_dir = BACKUP_ROOT / f"rollout-{SHORT}-{stamp}"
    data_dir = out_dir / "data"
    data_dir.mkdir(parents=True)
    tests_log = PROBE / "workspace-tests.log"
    manifest = {
        "captured_utc": now(),
        "release_source_commit": COMMIT,
        "release_source_tree": TREE,
        "reviewed_probe_manifest_sha256": gate["probe_manifest_sha256"],
        "release_binary_sha256": {
            "server": sha256(SERVER),
            "query_worker": sha256(WORKER),
        },
        "workspace_tests": {
            "log": str(tests_log.relative_to(ROOT)) if tests_log.exists() else None,
            "log_sha256": sha256(tests_log) if tests_log.exists() else None,
            "summary": test_summary(tests_log) if tests_log.exists() else None,
        },
        "pre_rollout_service_state": sh(["systemctl", "is-active", "schrott-mcp.service"]),
        "pre_rollout_service_start": sh(
            ["systemctl", "show", "schrott-mcp.service", "-p", "ExecMainStartTimestamp", "--value"]),
        "pre_rollout_installed_binary_sha256": {
            "server": sha256(Path("/usr/local/bin/schrott-mcp-server")),
            "query_worker": sha256(Path("/usr/local/bin/schrott-mcp-query-worker")),
        },
        "production_write": False,
        "database_backups": {},
    }
    for name in ("public.db", "internal.db"):
        manifest["database_backups"][name] = backup_database(name, data_dir / name)
    manifest["installed_binary_backups"] = {}
    for name in ("schrott-mcp-server", "schrott-mcp-query-worker"):
        dest = out_dir / name
        shutil.copy2(BIN_DIR / name, dest)
        os.chmod(dest, 0o700)
        manifest["installed_binary_backups"][name] = {
            "sha256": sha256(dest),
            "size_bytes": dest.stat().st_size,
        }
    (out_dir / "manifest.json").write_text(json.dumps(manifest, indent=2))
    print(f"backup_dir={out_dir}")
    print(json.dumps(manifest["database_backups"]["public.db"], indent=2))

    # install binaries (keep the file mode of the currently installed release)
    for src, name in ((SERVER, "schrott-mcp-server"), (WORKER, "schrott-mcp-query-worker")):
        target = BIN_DIR / name
        mode = target.stat().st_mode & 0o777
        tmp = BIN_DIR / f".{name}.new"
        shutil.copy2(src, tmp)
        os.chmod(tmp, mode)
        os.replace(tmp, target)
        print(f"installed {name} sha256={sha256(target)} mode={oct(mode)}")

    started = now()
    sh(["systemctl", "restart", "schrott-mcp.service"])
    print(f"restart_requested_utc={started}")

    # wait for the service to come back and answer /health
    health = None
    for _ in range(60):
        time.sleep(1)
        try:
            out = subprocess.run(
                ["curl", "--silent", "--show-error", "--max-time", "5",
                 "--write-out", "\nHTTP %{http_code}",
                 "http://127.0.0.1:4001/health"],
                capture_output=True, text=True)
            if "HTTP 200" in out.stdout:
                health = out.stdout.strip()
                break
        except subprocess.SubprocessError:
            pass
    print(f"health={health}")
    print("service_state=" + sh(["systemctl", "is-active", "schrott-mcp.service"]))

    # wait for a natural scheduler run after the restart
    run = None
    for _ in range(120):
        conn = sqlite3.connect(f"file:{PROD}/internal.db?mode=ro", uri=True)
        conn.execute("PRAGMA query_only=ON")
        row = conn.execute(
            "SELECT id, started_at, finished_at, status, detail FROM ingestion_runs "
            "WHERE started_at > ? ORDER BY id DESC LIMIT 1", (started,)).fetchone()
        conn.close()
        if row and row[3] in ("ok", "partial", "warning"):
            run = {"id": row[0], "started_at": row[1], "finished_at": row[2],
                   "status": row[3], "detail": row[4]}
            break
        time.sleep(5)
    print("post_rollout_run=" + json.dumps(run))

    # read-only verification of every target row
    conn = sqlite3.connect(f"file:{PROD}/public.db?mode=ro", uri=True)
    conn.execute("PRAGMA query_only=ON")
    conn.row_factory = sqlite3.Row
    literals = ",".join("'" + s.replace("'", "''") + "'" for s in TARGETS)
    rows = conn.execute(
        "SELECT slug, name, street, postcode, phone, website, website_status, status, "
        "lat, lon, dropoff_json FROM traders WHERE slug IN (" + literals + ") ORDER BY slug"
    ).fetchall()
    geo_cleared = [r["slug"] for r in rows if r["lat"] is None and r["lon"] is None]
    geo_present = [r["slug"] for r in rows if r["lat"] is not None or r["lon"] is not None]
    counts = db_counts(PROD / "public.db")
    conn.close()
    verification = {
        "checked_utc": now(),
        "target_rows": [dict(r) for r in rows],
        "geo_cleared": geo_cleared,
        "geo_still_present": geo_present,
        "all_geo_cleared": not geo_present,
        "dropoff_not_allowed": [r["slug"] for r in rows if r["dropoff_json"] != "{}"],
        "counts": counts,
    }
    (out_dir / "post-rollout-verification.json").write_text(json.dumps(verification, indent=2))
    print(json.dumps({
        "geo_cleared": len(geo_cleared),
        "geo_still_present": geo_present,
        "dropoff_not_allowed": verification["dropoff_not_allowed"],
        "counts": counts,
    }, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
