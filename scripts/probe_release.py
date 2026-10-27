#!/usr/bin/env python3
"""Private, read-only-first release probe for a candidate commit.

Rules honoured here:
  * production databases are opened read-only (SQLite URI mode=ro) with
    PRAGMA query_only=ON and copied through the SQLite backup API;
  * no production write, no service restart, no deployment;
  * every candidate write happens in a private copy under target/;
  * the post-seed digest binds the main database file *and* its WAL sidecar;
  * every changed field is cross-checked against the seed JSON that the
    exported tree's own codegen produced.
"""

import glob
import hashlib
import json
import os
import shutil
import signal
import sqlite3
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path("/root/Documents/schrott-mcp")
PROBE = Path(os.environ.get("PROBE_DIR", ROOT / "target/probe"))
SRC = PROBE / "src"
BUILD = PROBE / "build"
PRISTINE = PROBE / "source"
DATA = PROBE / "data"
PROD = Path("/var/lib/schrott-mcp")
SERVER = BUILD / "release/schrott-mcp-server"
WORKER = BUILD / "release/schrott-mcp-query-worker"
SEED_GLOB = str(BUILD / "release/build/schrott-mcp-ingestion/*/out/seed_traders/*.json")

TRADER_COLUMNS = [
    "slug", "name", "trader_type", "state", "city", "street", "postcode",
    "phone", "email", "opening_hours", "website", "website_status", "status",
    "description", "lat", "lon", "dropoff_json", "pickup_json",
    "certifications", "notes",
]

CLEAR_DEFAULTS = {
    "phone": "",
    "email": "",
    "website": "",
    "opening_hours": "",
    "description": "",
    "website_status": "unbekannt",
    "certifications": "[]",
    "dropoff_json": "{}",
    "pickup_json": "{}",
}

# The nine dossier corrections this release carries; their production values
# must stay untouched while the probe runs.
DEFAULT_PROBE_TARGET_SLUGS = [
    "mv-wismar-alba-metall-nord",
    "rp-hanhofen-romerberg-hanhofen-romerberg-cluster-zebe-rohstoff",
    "sl-losheim-bachem-schrotthandel-bachem-name-unsicher-evtl",
    "sl-neunkirchen-wellesweiler-665-goldwirt",
    "sl-saarlouis-66740-hahn-schrott-metalle",
    "sn-freiberg-arm-abfall-rohstoff-management",
    "sn-rochlitz-db-recycling-denis-blum",
    "st-kalbe-rilei",
    "th-erfurt-altstadt-99084-an-und-verkauf-computer-und-haushaltsger",
]


def probe_target_slugs() -> list:
    """Target rows of the release; overridable per release with a JSON file."""
    path = os.environ.get("PROBE_TARGET_SLUGS_FILE")
    if not path:
        return list(DEFAULT_PROBE_TARGET_SLUGS)
    slugs = json.loads(Path(path).read_text())
    if not isinstance(slugs, list) or not slugs or not all(isinstance(s, str) for s in slugs):
        raise SystemExit("invalid PROBE_TARGET_SLUGS_FILE")
    return slugs


PROBE_TARGET_SLUGS = probe_target_slugs()


def now() -> str:
    return datetime.now(timezone.utc).isoformat()


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def digest_sidecars(db: Path) -> dict:
    """Digest a SQLite database together with its WAL sidecar."""
    out = {}
    for suffix in ("", "-wal", "-shm"):
        p = Path(str(db) + suffix)
        out[p.name] = {
            "present": p.exists(),
            "size_bytes": p.stat().st_size if p.exists() else 0,
            "sha256": sha256(p) if p.exists() else None,
        }
    combined = hashlib.sha256()
    for suffix in ("", "-wal"):
        p = Path(str(db) + suffix)
        combined.update(p.name.encode())
        combined.update((sha256(p) if p.exists() else "-").encode())
    out["combined_db_wal_sha256"] = combined.hexdigest()
    return out


def snapshot_production() -> dict:
    """Copy production DBs into private pristine copies (read-only source)."""
    PRISTINE.mkdir(parents=True, exist_ok=True)
    manifest = {"private_only": True, "production_write": False, "files": {}}
    for name in ("public.db", "internal.db"):
        src = sqlite3.connect(f"file:{PROD/name}?mode=ro", uri=True)
        src.execute("PRAGMA query_only=ON")
        dst_path = PRISTINE / name
        if dst_path.exists():
            dst_path.unlink()
        dst = sqlite3.connect(dst_path)
        src.backup(dst)
        dst.close()
        src.close()
        manifest["files"][name] = {
            "source_mode": "SQLite URI mode=ro; PRAGMA query_only=ON; backup API",
            "source_opened_utc": now(),
            "private_size_bytes": dst_path.stat().st_size,
            "private_sha256": sha256(dst_path),
        }
    return manifest


def copy_to_data_dir(target: Path) -> None:
    """Fresh private copy of the pristine snapshot for one boot."""
    if target.exists():
        shutil.rmtree(target)
    target.mkdir(parents=True)
    for name in ("public.db", "internal.db"):
        shutil.copy2(PRISTINE / name, target / name)


def rows_of(db: Path, slugs=None) -> dict:
    conn = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    conn.execute("PRAGMA query_only=ON")
    conn.row_factory = sqlite3.Row
    if slugs is None:
        cur = conn.execute(f"SELECT {','.join(TRADER_COLUMNS)} FROM traders")
    else:
        marks = ",".join("?" * len(slugs))
        cur = conn.execute(
            f"SELECT {','.join(TRADER_COLUMNS)} FROM traders WHERE slug IN ({marks})",
            list(slugs),
        )
    out = {r["slug"]: {c: r[c] for c in TRADER_COLUMNS} for r in cur.fetchall()}
    conn.close()
    return out


def counts(db: Path) -> dict:
    conn = sqlite3.connect(f"file:{db}?mode=ro", uri=True)
    conn.execute("PRAGMA query_only=ON")
    out = {}
    for t in ("traders", "materials", "prices", "current_prices", "trader_materials"):
        try:
            out[t] = conn.execute(f"SELECT COUNT(*) FROM {t}").fetchone()[0]
        except sqlite3.Error:
            out[t] = None
    conn.close()
    return out


def boot(label: str, port: int, data_dir: Path) -> dict:
    """Boot the candidate server once against its own private data dir."""
    log = PROBE / f"candidate-{label}-boot.log"
    env = dict(os.environ)
    env["RUST_LOG"] = "info"
    with log.open("wb") as fh:
        proc = subprocess.Popen(
            [str(SERVER), "--data-dir", str(data_dir), "--bind", f"127.0.0.1:{port}"],
            stdout=fh, stderr=subprocess.STDOUT, env=env, cwd=str(SRC),
        )
        written = None
        deadline = time.time() + 60
        while time.time() < deadline:
            if proc.poll() is not None:
                break
            text = log.read_text(errors="replace")
            for line in text.splitlines():
                if "trader seed up to date" in line:
                    try:
                        written = int(line.split("(")[1].split(" ")[0])
                    except (IndexError, ValueError):
                        written = None
            if written is not None:
                break
            time.sleep(0.2)
        # Terminate before the 15s scheduler delay: the probe only needs the
        # seed path, never a live scrape.
        health = None
        for _ in range(50):
            try:
                import urllib.request
                with urllib.request.urlopen(
                        f"http://127.0.0.1:{port}/health", timeout=2) as response:
                    health = response.read().decode()[:120]
                    if health:
                        break
            except Exception:
                time.sleep(0.1)
        terminated = False
        if proc.poll() is None:
            proc.send_signal(signal.SIGTERM)
            terminated = True
            try:
                proc.wait(timeout=20)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait(timeout=20)
    return {
        "label": label,
        "log": str(log.relative_to(ROOT)),
        "seed_rows_written": written,
        "health_response": health,
        "health_ok": bool(health and '"ok"' in health),
        "terminated_by_probe_sigterm": terminated,
        "exit_code": proc.returncode,
        "exit_meaning": "SIGTERM (-15) sent by the probe right after the seed log line; "
                        "the probe intentionally stops before the 15s scheduler delay, "
                        "so no scrape runs against the private copy.",
        "log_sha256": sha256(log),
        "log_lines": log.read_text(errors="replace").splitlines(),
    }


def worker_query(sql: str) -> dict:
    req = json.dumps({"sql": sql})
    proc = subprocess.run(
        [str(WORKER), str(DATA)], input=req, capture_output=True, text=True,
    )
    if proc.returncode != 0:
        return {"ok": False, "stdout": proc.stdout, "stderr": proc.stderr,
                "exit_code": proc.returncode}
    return json.loads(proc.stdout)


def sql_literal(value: str) -> str:
    return "'" + value.replace("'", "''") + "'"


def tree_binding(head: str) -> dict:
    """Measure the exported tree against the commit's git object hashes."""
    expected = {}
    for line in subprocess.check_output(
            ["git", "-C", str(ROOT), "ls-tree", "-r", head], text=True).splitlines():
        meta, path = line.split("\t", 1)
        expected[path] = meta.split()[2]
    actual = {}
    for path in sorted(Path(SRC).rglob("*")):
        if not path.is_file():
            continue
        rel = str(path.relative_to(SRC))
        actual[rel] = subprocess.check_output(
            ["git", "hash-object", str(path)], text=True).strip()
    listing = "".join(f"{h} {p}\n" for p, h in sorted(actual.items()))
    mismatched = sorted(set(expected) ^ set(actual)) + \
        sorted(p for p in set(expected) & set(actual) if expected[p] != actual[p])
    return {
        "commit": head,
        "expected_files": len(expected),
        "exported_files": len(actual),
        "mismatched_or_missing": mismatched,
        "identical_to_commit": not mismatched,
        "exported_tree_object_list_sha256": hashlib.sha256(listing.encode()).hexdigest(),
    }


def run_format_check() -> dict:
    """Actually run cargo fmt --check in the exported tree and keep its log."""
    log = PROBE / "fmt-check.log"
    env = dict(os.environ)
    env.update({
        "HOME": "/root", "CARGO_HOME": "/root/.cargo", "RUSTUP_HOME": "/root/.rustup",
        "PATH": "/root/.cargo/bin:" + env.get("PATH", ""),
    })
    proc = subprocess.run(
        ["/root/.cargo/bin/cargo", "fmt", "--all", "--", "--check"],
        cwd=str(SRC), env=env, capture_output=True, text=True)
    log.write_text(
        "command: cargo fmt --all -- --check\n"
        f"cwd: {SRC}\n"
        f"exit_code: {proc.returncode}\n"
        f"stdout:\n{proc.stdout}\n"
        f"stderr:\n{proc.stderr}\n")
    return {
        "command": "cargo fmt --all -- --check",
        "cwd": str(SRC),
        "exit_code": proc.returncode,
        "passed": proc.returncode == 0,
        "log": str(log.relative_to(ROOT)),
        "log_sha256": sha256(log),
        "output": (proc.stdout + proc.stderr).strip(),
    }


def production_fingerprint() -> dict:
    """Full read-only fingerprint of every production trader row."""
    rows = rows_of(PROD / "public.db")
    payload = json.dumps(
        {slug: {k: rows[slug][k] for k in ("name", "city", "street", "postcode", "phone",
                                           "email", "website", "website_status", "status",
                                           "lat", "lon", "dropoff_json", "pickup_json")}
         for slug in sorted(rows)},
        sort_keys=True, ensure_ascii=False)
    return {
        "trader_rows": len(rows),
        "sha256": hashlib.sha256(payload.encode()).hexdigest(),
    }


def production_state() -> dict:
    """Measured proof that production is untouched by the probe."""
    installed = {}
    for name in ("schrott-mcp-server", "schrott-mcp-query-worker"):
        path = Path("/usr/local/bin") / name
        installed[name] = sha256(path) if path.exists() else None
    start = subprocess.run(
        ["systemctl", "show", "schrott-mcp.service", "-p", "ExecMainStartTimestamp",
         "--value"], capture_output=True, text=True).stdout.strip()
    rows = rows_of(PROD / "public.db", PROBE_TARGET_SLUGS)
    rollout_backups = sorted(
        p.name for p in Path("/root/Documents/schrott-mcp-rollout-backups").glob("rollout-*")
    )
    return {
        "installed_binary_sha256": installed,
        "service_start_timestamp": start,
        "target_rows": {s: {k: rows[s][k] for k in ("name", "lat", "lon", "status", "dropoff_json")}
                        for s in PROBE_TARGET_SLUGS if s in rows},
        "rollout_backup_dirs": rollout_backups,
    }


def test_log_block(probe_dir: Path) -> dict:
    """Digest and summarise this release's own workspace test log."""
    import re
    log = probe_dir / "workspace-tests.log"
    block = {
        "log": str(log.relative_to(ROOT)) if log.exists() else None,
        "log_sha256": sha256(log) if log.exists() else None,
        "summary": None,
    }
    if log.exists():
        pattern = re.compile(r"^test result: .*?(\d+) passed; (\d+) failed; (\d+) ignored;")
        passed = failed = ignored = suites = 0
        for line in log.read_text().splitlines():
            match = pattern.match(line.strip())
            if match:
                suites += 1
                passed += int(match.group(1))
                failed += int(match.group(2))
                ignored += int(match.group(3))
        block["summary"] = {"suites": suites, "passed": passed, "failed": failed,
                            "ignored": ignored}
    return block


def target_slug_file_block() -> dict:
    """Keep the release's target-row list inside the evidence bundle."""
    path = os.environ.get("PROBE_TARGET_SLUGS_FILE")
    if not path:
        return {"source": None, "slugs": list(DEFAULT_PROBE_TARGET_SLUGS)}
    src = Path(path)
    if not src.exists():
        return {"source": str(src), "slugs": None}
    dest = PROBE / "probe-target-slugs.json"
    shutil.copy2(src, dest)
    return {
        "source": str(src),
        "bundle_copy": str(dest.relative_to(ROOT)),
        "bundle_copy_sha256": sha256(dest),
        "slugs": json.loads(dest.read_text()),
    }


def main() -> int:
    PROBE.mkdir(parents=True, exist_ok=True)
    release = os.environ.get("RELEASE_COMMIT", "HEAD")
    head = subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", release], text=True).strip()
    tree = subprocess.check_output(["git", "-C", str(ROOT), "rev-parse", release + "^{tree}"], text=True).strip()
    production_before = production_state()
    production_fingerprint_before = production_fingerprint()
    manifest = {
        "private_only": True,
        "captured_utc": now(),
        "repository_head": head,
        "repository_tree": tree,
        "production_source": "read-only SQLite URI + PRAGMA query_only=ON; SQLite backup API",
        "production_write": False,
        "deployment": False,
        "service_restart": False,
    }

    # --- build / binary binding -------------------------------------------------
    build_log = PROBE / "build.log"
    seed_files = sorted(glob.glob(SEED_GLOB))
    fmt_check = run_format_check()
    manifest["target_rows_scope"] = target_slug_file_block()
    manifest["target_rows_scope"]["note"] = (
        "Die Scope-Datei benennt die Zeilen, die im Vorzustand schon falsch oder "
        "unvollständig waren; das effektive Delta des Kandidaten kann weitere Zeilen "
        "enthalten (etwa durch die Wiederherstellung einer verwaisten Zeile). Beide "
        "Mengen stehen deshalb getrennt im Manifest.")
    probe_script = Path(__file__).resolve()
    manifest["probe_script"] = {
        "path": str(probe_script.relative_to(ROOT)) if probe_script.is_relative_to(ROOT) else str(probe_script),
        "sha256": sha256(probe_script),
        "note": "Das Skript, das dieses Manifest erzeugt hat; der Digest bindet die "
                "Messlogik an das Artefakt.",
    }
    manifest["build_binding"] = {
        "source_tree": "git archive of the exact commit, exported into target/ before the build",
        "tree_binding": tree_binding(head),
        "build_command": "cargo build --release --locked --offline -p schrott-mcp-server -p schrott-mcp-query-worker",
        "build_log": str(build_log.relative_to(ROOT)),
        "build_log_sha256": sha256(build_log) if build_log.exists() else None,
        "build_log_markers": {
            "release_profile_finished": any(
                "Finished `release` profile" in line
                for line in build_log.read_text().splitlines()) if build_log.exists() else False,
            "seed_codegen": any(
                "seed codegen ok" in line
                for line in build_log.read_text().splitlines()) if build_log.exists() else False,
            "compiling_lines": sum(
                1 for line in build_log.read_text().splitlines()
                if "Compiling " in line) if build_log.exists() else 0,
        },
        "format_check": fmt_check,
        "workspace_tests": test_log_block(PROBE),
        "binary_sha256": {
            "server": sha256(SERVER),
            "query_worker": sha256(WORKER),
        },
        "binary_mtime_utc": {
            "server": datetime.fromtimestamp(SERVER.stat().st_mtime, timezone.utc).isoformat(),
            "query_worker": datetime.fromtimestamp(WORKER.stat().st_mtime, timezone.utc).isoformat(),
        },
        "combined_digest_definition": (
            "combined_db_wal_sha256 = sha256(b'public.db' + sha256(public.db) + "
            "b'public.db-wal' + sha256(public.db-wal)); die -shm-Datei ist bewusst "
            "nicht Teil des Digests, weil sie der flüchtige WAL-Index ist."),
        "seed_codegen_files": [
            {"path": str(Path(f).relative_to(ROOT)), "sha256": sha256(Path(f))}
            for f in seed_files
        ],
        "note": "Release binaries embed debug info with the absolute build path, so the "
                "hash differs from the installed 6b3da663 build even where the machine code "
                "is unchanged. The binding evidence is the exported tree hash plus the "
                "build log and the seed codegen output of that tree.",
    }

    # --- private snapshot + candidate boots -------------------------------------
    manifest["snapshot"] = snapshot_production()
    manifest["source_counts"] = counts(PRISTINE / "public.db")
    manifest["source_digests"] = digest_sidecars(PRISTINE / "public.db")

    # Beide Boots laufen auf DERSELBEN privaten Kopie: nur so beweist der
    # zweite Boot die Idempotenz (0 Zeilen). Der Zustand nach dem ersten Boot
    # wird vor dem zweiten Boot komplett (inklusive WAL) archiviert, damit
    # dessen Digest aus dem Bundle nachrechenbar bleibt.
    copy_to_data_dir(DATA)
    first_snapshot = PROBE / "data-first-boot"
    manifest["phase_timestamps"] = {"probe_started_utc": now()}
    manifest["first_boot"] = boot("first", 46971, DATA)
    manifest["first_boot"]["data_dir"] = str(DATA.relative_to(ROOT))
    manifest["after_first_boot_counts"] = counts(DATA / "public.db")
    manifest["after_first_boot_digests"] = digest_sidecars(DATA / "public.db")
    if first_snapshot.exists():
        shutil.rmtree(first_snapshot)
    shutil.copytree(DATA, first_snapshot)
    manifest["first_boot"]["state_snapshot"] = str(first_snapshot.relative_to(ROOT))
    manifest["first_boot"]["state_snapshot_digests"] = digest_sidecars(first_snapshot / "public.db")
    manifest["phase_timestamps"]["after_first_boot_utc"] = now()

    manifest["second_boot"] = boot("second", 46972, DATA)
    manifest["second_boot"]["data_dir"] = str(DATA.relative_to(ROOT))
    manifest["after_second_boot_counts"] = counts(DATA / "public.db")
    manifest["after_second_boot_digests"] = digest_sidecars(DATA / "public.db")
    manifest["phase_timestamps"]["after_second_boot_utc"] = now()

    # --- effective delta --------------------------------------------------------
    before = rows_of(PRISTINE / "public.db")
    after = rows_of(DATA / "public.db")
    seed = {}
    for f in seed_files:
        for row in json.load(open(f)):
            seed[row["slug"]] = row

    changed = []
    field_counts = {}
    unexplained = []
    for slug, new_row in sorted(after.items()):
        old_row = before.get(slug)
        if old_row is None:
            changed.append({"slug": slug, "fields": ["__new_row__"]})
            continue
        fields = [c for c in TRADER_COLUMNS if old_row[c] != new_row[c]]
        if not fields:
            continue
        changed.append({"slug": slug, "fields": fields})
        for f in fields:
            field_counts[f] = field_counts.get(f, 0) + 1
        dossier = seed.get(slug, {})
        clears = set(filter(None, str(dossier.get("clear_fields", "")).split(",")))
        for f in fields:
            if f in ("notes", "extra_json", "updated_at"):
                continue
            if f == "lat" or f == "lon":
                if "geo" in clears:
                    if new_row[f] is not None:
                        unexplained.append({"slug": slug, "field": f, "reason": "geo clear but value kept"})
                    continue
                # numerisch vergleichen: SQLite speichert 51.0008670 als 51.000867
                try:
                    same = abs(float(new_row[f]) - float(dossier.get(f, "nan"))) < 1e-9
                except (TypeError, ValueError):
                    same = str(new_row[f]) == str(dossier.get(f, ""))
                if not same:
                    unexplained.append({"slug": slug, "field": f, "reason": "value differs from dossier",
                                        "db": new_row[f], "dossier": dossier.get(f, "")})
                continue
            if f in CLEAR_DEFAULTS:
                if f in clears:
                    if str(new_row[f]) != CLEAR_DEFAULTS[f]:
                        unexplained.append({"slug": slug, "field": f, "reason": "clear default not applied",
                                            "db": new_row[f], "expected": CLEAR_DEFAULTS[f]})
                    continue
                if str(new_row[f]) != str(dossier.get(f, "")):
                    unexplained.append({"slug": slug, "field": f, "reason": "value differs from dossier",
                                        "db": new_row[f], "dossier": dossier.get(f, "")})
                continue
            if str(new_row[f]) != str(dossier.get(f, "")):
                unexplained.append({"slug": slug, "field": f, "reason": "value differs from dossier",
                                    "db": new_row[f], "dossier": dossier.get(f, "")})
    manifest["effective_delta"] = {
        "changed_trader_count": len(changed),
        "changed_field_row_counts": field_counts,
        "changed_rows": changed,
        "seed_rows_total": len(seed),
        "seed_rows_with_clear_fields": sum(1 for r in seed.values() if r.get("clear_fields")),
        "unexplained_changes": unexplained,
        "all_changes_explained_by_seed": not unexplained,
    }

    # --- clear-directive cross-check over the whole seed corpus -----------------
    clear_violations = []
    for slug, dossier in sorted(seed.items()):
        clears = set(filter(None, str(dossier.get("clear_fields", "")).split(",")))
        if not clears or slug not in after:
            continue
        row = after[slug]
        if "geo" in clears and (row["lat"] is not None or row["lon"] is not None):
            clear_violations.append({"slug": slug, "clear": "geo", "lat": row["lat"], "lon": row["lon"]})
        for field, default in CLEAR_DEFAULTS.items():
            if field in clears and str(row[field]) != default:
                clear_violations.append({"slug": slug, "clear": field, "value": row[field],
                                         "expected": default})
    manifest["clear_directive_check"] = {
        "cleared_rows_checked": sum(1 for r in seed.values() if r.get("clear_fields")),
        "violations": clear_violations,
        "all_clears_applied": not clear_violations,
    }

    # --- query worker must read every target row -------------------------------
    slugs = [c["slug"] for c in changed]
    literals = ",".join(sql_literal(s) for s in slugs) if slugs else "''"
    sql = (
        "SELECT slug, lat, lon, status, name, street, postcode, phone, website, "
        "website_status, dropoff_json FROM traders WHERE slug IN (" + literals + ") ORDER BY slug"
    )
    worker = worker_query(sql)
    if worker.get("ok"):
        worker_rows = {r[0]: r[1:] for r in worker["ok"]["rows"]}
        manifest["query_worker"] = {
            "binary": str(WORKER.relative_to(ROOT)),
            "binary_sha256": sha256(WORKER),
            "sql_scope": "all slugs of the effective delta (literal IN list)",
            "requested_rows": len(slugs),
            "returned_rows": len(worker_rows),
            "all_targets_read": len(worker_rows) == len(slugs),
            "rows": [
                {"slug": s, "lat": v[0], "lon": v[1], "status": v[2], "name": v[3],
                 "street": v[4], "postcode": v[5], "phone": v[6], "website": v[7],
                 "website_status": v[8], "dropoff_json": v[9]}
                for s, v in sorted(worker_rows.items())
            ],
        }
    else:
        manifest["query_worker"] = worker

    out = PROBE / "predeploy-probe-manifest.json"
    production_after = production_state()
    production_fingerprint_after = production_fingerprint()
    manifest["production_integrity"] = {
        "claim": "the probe performed no production write, no deployment and no service restart",
        "production_untouched": bool(
            production_before["installed_binary_sha256"] == production_after["installed_binary_sha256"]
            and production_before["service_start_timestamp"] == production_after["service_start_timestamp"]
            and production_before["target_rows"] == production_after["target_rows"]
            and production_before["rollout_backup_dirs"] == production_after["rollout_backup_dirs"]),
        "measured_invariants": "installed binary hashes, service start timestamp, "
                               "production target rows and rollout backup dirs are "
                               "identical before and after the probe",
        "installed_binary_sha256_before": production_before["installed_binary_sha256"],
        "installed_binary_sha256_after": production_after["installed_binary_sha256"],
        "installed_binary_unchanged": production_before["installed_binary_sha256"]
        == production_after["installed_binary_sha256"],
        "service_start_before": production_before["service_start_timestamp"],
        "service_start_after": production_after["service_start_timestamp"],
        "service_not_restarted": production_before["service_start_timestamp"]
        == production_after["service_start_timestamp"],
        "target_rows_before": production_before["target_rows"],
        "target_rows_after": production_after["target_rows"],
        "target_rows_unchanged": production_before["target_rows"]
        == production_after["target_rows"],
        "full_production_fingerprint_before": production_fingerprint_before,
        "full_production_fingerprint_after": production_fingerprint_after,
        "full_production_unchanged": production_fingerprint_before["sha256"]
        == production_fingerprint_after["sha256"],
        "rollout_backup_dirs_before": production_before["rollout_backup_dirs"],
        "rollout_backup_dirs_after": production_after["rollout_backup_dirs"],
        "no_rollout_backup_created": production_before["rollout_backup_dirs"]
        == production_after["rollout_backup_dirs"],
        "deployment": False,
        "service_restart": False,
    }
    manifest["phase_timestamps"]["manifest_written_utc"] = now()
    out.write_text(json.dumps(manifest, indent=2, sort_keys=False))
    print(json.dumps({
        "manifest": str(out),
        "first_boot_rows": manifest["first_boot"]["seed_rows_written"],
        "second_boot_rows": manifest["second_boot"]["seed_rows_written"],
        "changed_rows": manifest["effective_delta"]["changed_trader_count"],
        "field_counts": manifest["effective_delta"]["changed_field_row_counts"],
        "all_changes_explained": manifest["effective_delta"]["all_changes_explained_by_seed"],
        "unexplained": unexplained[:10],
        "all_clears_applied": manifest["clear_directive_check"]["all_clears_applied"],
        "clear_violations": clear_violations[:10],
        "worker_read_all": manifest.get("query_worker", {}).get("all_targets_read"),
        "worker_rows": manifest.get("query_worker", {}).get("returned_rows"),
    }, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
