#!/usr/bin/env python3
"""Instantiate the release-delta verifier templates for one release.

The templates in scripts/ carry placeholders; this script writes the concrete,
digest-pinned verifier and its contract test for a given release.

Usage:
  python3 scripts/install_release_delta_verifier.py --short 7cb04f2d \
      --release <commit> --tree <tree> --expectations <file> --rehearsal <manifest>
"""
import argparse
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VERIFIER_TEMPLATE = ROOT / "scripts/verify-release-delta-SHORT.py"
TEST_TEMPLATE = ROOT / "scripts/test_verify_release_delta_SHORT.py"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--short", required=True)
    parser.add_argument("--release", required=True)
    parser.add_argument("--tree", required=True)
    parser.add_argument("--expectations", type=Path, required=True)
    parser.add_argument("--rehearsal", type=Path, required=True)
    args = parser.parse_args()

    expectations = args.expectations.resolve()
    rehearsal = args.rehearsal.resolve()
    values = {
        "MANIFEST_SHA256_PLACEHOLDER": hashlib.sha256(expectations.read_bytes()).hexdigest(),
        "RELEASE_COMMIT_PLACEHOLDER": args.release,
        "RELEASE_TREE_PLACEHOLDER": args.tree,
        "REHEARSAL_SHA256_PLACEHOLDER": hashlib.sha256(rehearsal.read_bytes()).hexdigest(),
    }

    verifier = ROOT / f"scripts/verify-release-delta-{args.short}.py"
    text = VERIFIER_TEMPLATE.read_text()
    text = text.replace('release-delta-SHORT-seed-expectations.json',
                        f'release-delta-{args.short}-seed-expectations.json')
    for key, value in values.items():
        text = text.replace(key, value)
    if "PLACEHOLDER" in text:
        raise SystemExit("unreplaced placeholder in the verifier template")
    verifier.write_text(text)

    test = ROOT / f"scripts/test_verify_release_delta_{args.short}.py"
    text = TEST_TEMPLATE.read_text()
    text = text.replace('verify-release-delta-SHORT.py', f'verify-release-delta-{args.short}.py')
    text = text.replace('release_delta_SHORT_verifier', f'release_delta_{args.short}_verifier')
    if "SHORT" in text:
        raise SystemExit("unreplaced placeholder in the test template")
    test.write_text(text)

    manifest = json.loads(expectations.read_text())
    print(json.dumps({
        "verifier": str(verifier.relative_to(ROOT)),
        "test": str(test.relative_to(ROOT)),
        "scope_rows": len(manifest["traders"]),
        "verifier_sha256": hashlib.sha256(verifier.read_bytes()).hexdigest(),
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
