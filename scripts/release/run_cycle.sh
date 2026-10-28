#!/usr/bin/env bash
# Release cycle for a candidate commit: export -> build -> test -> probe -> pin.
# Usage: run_cycle.sh <commit> <short-sha> <target-slugs.json>
set -euo pipefail

ROOT=/root/Documents/schrott-mcp
COMMIT=${1:?commit required}
SHORT=${2:?short sha required}
SLUGS=${3:?target slug file required}
PROBE_DIR="$ROOT/target/predeploy-$SHORT"
export PATH=/root/.cargo/bin:$PATH
export HOME=/root CARGO_HOME=/root/.cargo RUSTUP_HOME=/root/.rustup

cd "$ROOT"
echo "== candidate $COMMIT =="
git cat-file -e "$COMMIT^{commit}"
TREE=$(git rev-parse "$COMMIT^{tree}")

mkdir -p "$PROBE_DIR/src" "$PROBE_DIR/build"
git archive "$COMMIT" | tar -x -C "$PROBE_DIR/src"

# tree binding: every exported file must hash to the commit's blob hash
git ls-tree -r "$COMMIT" | awk '{print $3" "$4}' | LC_ALL=C sort > "/tmp/opencode/tree-expected-$SHORT.txt"
( cd "$PROBE_DIR/src" && find . -type f -print0 | sort -z | \
  while IFS= read -r -d '' f; do printf '%s %s\n' "$(git hash-object "$f")" "${f#./}"; done | \
  LC_ALL=C sort > "/tmp/opencode/tree-actual-$SHORT.txt" )
diff -q "/tmp/opencode/tree-expected-$SHORT.txt" "/tmp/opencode/tree-actual-$SHORT.txt"
echo "TREE_MATCH_OK $TREE"

echo "== release build =="
BUILD_CMD="cargo build --release --locked --offline -p schrott-mcp-server -p schrott-mcp-query-worker"
{
  echo "command: $BUILD_CMD"
  echo "cwd: $PROBE_DIR/src"
  echo "target_dir: $PROBE_DIR/build"
  echo "started_utc: $(date -u '+%Y-%m-%dT%H:%M:%S.%NZ')"
  echo "---"
} > "$PROBE_DIR/build.log"
( cd "$PROBE_DIR/src" && CARGO_TARGET_DIR="$PROBE_DIR/build" \
  /root/.cargo/bin/cargo build --release --locked --offline \
  -p schrott-mcp-server -p schrott-mcp-query-worker ) 2>&1 | tee -a "$PROBE_DIR/build.log" | tail -2
{
  echo "exit_code: ${PIPESTATUS[0]}"
  echo "finished_utc: $(date -u '+%Y-%m-%dT%H:%M:%S.%NZ')"
} >> "$PROBE_DIR/build.log"
sha256sum "$PROBE_DIR/build/release/schrott-mcp-server" \
          "$PROBE_DIR/build/release/schrott-mcp-query-worker" | tee "$PROBE_DIR/binary-sha256.txt"

echo "== workspace tests =="
TEST_CMD="cargo test --workspace --locked --offline"
{
  echo "command: $TEST_CMD"
  echo "cwd: $PROBE_DIR/src"
  echo "target_dir: $PROBE_DIR/build"
  echo "started_utc: $(date -u '+%Y-%m-%dT%H:%M:%S.%NZ')"
  echo "---"
} > "$PROBE_DIR/workspace-tests.log"
( cd "$PROBE_DIR/src" && CARGO_TARGET_DIR="$PROBE_DIR/build" CARGO_BUILD_JOBS=1 \
  /root/.cargo/bin/cargo test --workspace --locked --offline ) 2>&1 | tee -a "$PROBE_DIR/workspace-tests.log" | tail -3
{
  echo "exit_code: ${PIPESTATUS[0]}"
  echo "finished_utc: $(date -u '+%Y-%m-%dT%H:%M:%S.%NZ')"
} >> "$PROBE_DIR/workspace-tests.log"

echo "== private probe =="
cp "$ROOT/scripts/probe_release.py" "$PROBE_DIR/probe_release.py"
PROBE_DIR="$PROBE_DIR" RELEASE_COMMIT="$COMMIT" PROBE_TARGET_SLUGS_FILE="$SLUGS" \
  python3 "$PROBE_DIR/probe_release.py" | tail -20

echo "== release-pinned wave E expectations =="
python3 "$ROOT/scripts/release/make_wave_e_expectations.py" \
  --rehearsal "$PROBE_DIR/data/public.db" \
  --probe-manifest "$PROBE_DIR/predeploy-probe-manifest.json" \
  --release "$COMMIT" --tree "$TREE" \
  --base "$ROOT/scripts/wave-e-20261028-0600-expectations.json" \
  --out "$ROOT/scripts/wave-e-$SHORT-seed-expectations.json"

echo "== pin wave E verifier =="
python3 "$ROOT/scripts/release/pin_digests.py" \
  --release "$COMMIT" --tree "$TREE" \
  --verifier "$ROOT/scripts/verify-wave-e-postrollout-$SHORT.py" \
  --expectations "$ROOT/scripts/wave-e-$SHORT-seed-expectations.json" \
  --historical "$ROOT/scripts/wave-e-20261011-expectations.json" \
  --rehearsal "$PROBE_DIR/predeploy-probe-manifest.json"

echo "== release delta expectations + verifier =="
DELTA_ROWS=$(python3 -c "import json;print(json.load(open('$PROBE_DIR/predeploy-probe-manifest.json'))['effective_delta']['changed_trader_count'])")
if [ "$DELTA_ROWS" = "0" ]; then
  echo "kein Seed-Delta in diesem Release (reine Code-Änderung) - kein release-delta-Verifier"
else
python3 "$ROOT/scripts/make_release_delta_expectations.py" \
  --rehearsal "$PROBE_DIR/data/public.db" \
  --probe-manifest "$PROBE_DIR/predeploy-probe-manifest.json" \
  --release "$COMMIT" --tree "$TREE" \
  --out "$ROOT/scripts/release-delta-$SHORT-seed-expectations.json"
python3 "$ROOT/scripts/install_release_delta_verifier.py" \
  --short "$SHORT" --release "$COMMIT" --tree "$TREE" \
  --expectations "$ROOT/scripts/release-delta-$SHORT-seed-expectations.json" \
  --rehearsal "$PROBE_DIR/predeploy-probe-manifest.json"
fi

echo "== pin pending corrections gate =="
python3 "$ROOT/scripts/release/pin_pending_corrections.py" \
  --release "$COMMIT" --tree "$TREE" \
  --rehearsal "$PROBE_DIR/data/public.db" \
  --manifest "$PROBE_DIR/predeploy-probe-manifest.json" \
  --short "$SHORT"

echo "== contract tests =="
python3 "$ROOT/scripts/test_verify_wave_e_postrollout_$SHORT.py" | tail -3
if [ "$DELTA_ROWS" != "0" ]; then
  python3 "$ROOT/scripts/test_verify_release_delta_$SHORT.py" | tail -3
fi
echo "== cycle artifacts ready under $PROBE_DIR =="
