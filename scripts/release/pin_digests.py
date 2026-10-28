#!/usr/bin/env python3
"""Pin a release-pinned verifier to a release, without hand-edited digests.

The earlier hand-patch of the 6b3da663 verifier constants produced two real
bugs: a wrong commit id and a digest that was copied into the wrong
constant. This script rewrites each constant individually with an anchored
pattern and then verifies the result against the real files.

Usage:
  python3 pin_digests.py --release <commit> --tree <tree> \
      --verifier scripts/verify-wave-e-postrollout-<short>.py \
      --expectations scripts/wave-e-<short>-seed-expectations.json \
      --historical scripts/wave-e-20261011-expectations.json \
      --rehearsal target/<dir>/predeploy-probe-manifest.json
"""
import argparse
import hashlib
import re
import subprocess
from pathlib import Path


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def set_constant(text: str, name: str, value: str) -> str:
    pattern = re.compile(rf'(?m)^{re.escape(name)} = "[0-9a-f]{{64}}"$')
    if len(pattern.findall(text)) != 1:
        raise SystemExit(f"expected exactly one {name} line, found "
                         f"{len(pattern.findall(text))}")
    return pattern.sub(f'{name} = "{value}"', text)


def set_text_constant(text: str, name: str, value: str) -> str:
    pattern = re.compile(rf'(?m)^{re.escape(name)} = "[0-9a-f]{{40}}"$')
    if len(pattern.findall(text)) != 1:
        raise SystemExit(f"expected exactly one {name} line, found "
                         f"{len(pattern.findall(text))}")
    return pattern.sub(f'{name} = "{value}"', text)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--release", required=True)
    parser.add_argument("--tree", required=True)
    parser.add_argument("--verifier", type=Path, required=True)
    parser.add_argument("--expectations", type=Path, required=True)
    parser.add_argument("--historical", type=Path, required=True)
    parser.add_argument("--rehearsal", type=Path, required=True)
    args = parser.parse_args()

    # the release must exist and its tree must match
    head = subprocess.check_output(["git", "rev-parse", args.release], text=True).strip()
    tree = subprocess.check_output(["git", "rev-parse", args.release + "^{tree}"], text=True).strip()
    if head != args.release or tree != args.tree:
        raise SystemExit(f"release identity mismatch: {head} / {tree}")

    digests = {
        "MANIFEST_SHA256": sha256(args.expectations),
        "HISTORICAL_MANIFEST_SHA256": sha256(args.historical),
        "REHEARSAL_MANIFEST_SHA256": sha256(args.rehearsal),
    }
    if len(set(digests.values())) != len(digests):
        raise SystemExit(f"digest collision, refusing to pin: {digests}")

    text = args.verifier.read_text()
    for name, value in digests.items():
        text = set_constant(text, name, value)
    text = set_text_constant(text, "RELEASE_COMMIT", args.release)
    text = set_text_constant(text, "RELEASE_TREE", args.tree)
    args.verifier.write_text(text)

    # read back and prove every constant matches the file it names
    import json
    expectations = json.loads(args.expectations.read_text())
    checks = {
        "MANIFEST_SHA256": digests["MANIFEST_SHA256"],
        "HISTORICAL_MANIFEST_SHA256": digests["HISTORICAL_MANIFEST_SHA256"],
        "REHEARSAL_MANIFEST_SHA256": digests["REHEARSAL_MANIFEST_SHA256"],
    }
    readback = dict(re.findall(r'(?m)^([A-Z0-9_]+) = "([0-9a-f]+)"$', args.verifier.read_text()))
    for name, value in checks.items():
        if readback.get(name) != value:
            raise SystemExit(f"readback mismatch for {name}")
    if expectations["release_commit"] != args.release or expectations["release_tree"] != args.tree:
        raise SystemExit("expectation file does not carry the release identity")
    if expectations["rehearsal_manifest_sha256"] != digests["REHEARSAL_MANIFEST_SHA256"]:
        raise SystemExit("expectation file does not pin the rehearsal manifest")

    print(json.dumps({"verifier": str(args.verifier), "release": args.release,
                      "tree": args.tree, "digests": digests, "readback_ok": True},
                     indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
