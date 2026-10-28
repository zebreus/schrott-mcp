#!/usr/bin/env python3
"""Instantiate the release-pinned Wave E verifier (and its contract test).

Bisher wurde der gepinnte Wave-E-Verifier je Release per Handkopie des
Vorgaenger-Stand erzeugt (Short im Dateinamen, Manifest-Name, Digests). Das war
nicht reproduzierbar: ein Release-Zyklus, der den Verifier nicht vorab als
Kopie hatte, brach ab.

Dieses Werkzeug erzeugt beide Dateien aus den versionierten Vorlagen
``scripts/verify-wave-e-postrollout-SHORT.py`` und
``scripts/test_verify_wave_e_postrollout-SHORT.py``:

1. ``scripts/verify-wave-e-postrollout-<short>.py``
2. ``scripts/test_verify_wave_e_postrollout_<short>.py``

Die Digest- und Release-Konstanten setzt anschliessend
``scripts/release/pin_digests.py`` (siehe ``run_cycle.sh``); dieses Werkzeug
schreibt nur die Dateinamen und den Short. Es faellt sofort, wenn in der
Vorlage noch ein Platzhalter oder ein fremder Short steht.

Usage:
  python3 scripts/release/install_wave_e_verifier.py --short 8e0bfb8
"""
import argparse
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
VERIFIER_TEMPLATE = ROOT / "scripts/verify-wave-e-postrollout-SHORT.py"
TEST_TEMPLATE = ROOT / "scripts/test_verify_wave_e_postrollout-SHORT.py"


def instantiate(template: Path, short: str, replacements: dict) -> str:
    text = template.read_text(encoding="utf-8")
    for old, new in replacements.items():
        text = text.replace(old, new)
    # Der Short-Platzhalter muss ersetzt sein. Die Digest-Konstanten bleiben
    # absichtlich auf Hex-Null gesetzt: pin_digests.py fuellt sie im
    # naechsten Schritt mit den realen Werten des Releases.
    if re.search(r"(?<![\w-])SHORT(?![\w-])", text):
        raise SystemExit(f"unreplaced SHORT placeholder in {template.name}")
    if re.search(r"(?<![\w-])[0-9a-f]{7}(?![\w-])", text):
        raise SystemExit(f"foreign short-like token left in {template.name}")
    return text


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--short", required=True)
    args = parser.parse_args()
    short = args.short
    if not re.fullmatch(r"[0-9a-f]{7,12}", short):
        raise SystemExit(f"invalid short: {short}")

    verifier = ROOT / f"scripts/verify-wave-e-postrollout-{short}.py"
    text = instantiate(VERIFIER_TEMPLATE, short, {
        "wave-e-SHORT-seed-expectations.json": f"wave-e-{short}-seed-expectations.json",
        "verify-wave-e-postrollout-SHORT.py": f"verify-wave-e-postrollout-{short}.py",
        "wave_e_SHORT_verifier": f"wave_e_{short}_verifier",
    })
    verifier.write_text(text, encoding="utf-8")

    test = ROOT / f"scripts/test_verify_wave_e_postrollout_{short}.py"
    text = instantiate(TEST_TEMPLATE, short, {
        "verify-wave-e-postrollout-SHORT.py": f"verify-wave-e-postrollout-{short}.py",
        "wave_e_SHORT_verifier": f"wave_e_{short}_verifier",
        "wave_e_SHORT": f"wave_e_{short}",
    })
    test.write_text(text, encoding="utf-8")

    print(f"verifier={verifier.relative_to(ROOT)}")
    print(f"test={test.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
