#!/usr/bin/env python3
"""Independently verify artifact digests and the recorded extraction."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle", type=Path, required=True)
    args = parser.parse_args()
    manifest = json.loads((args.bundle / "manifest.json").read_bytes())
    failures = []
    for name, expected in manifest["artifacts"].items():
        path = args.bundle / name
        actual = {"bytes": path.stat().st_size, "sha256": sha256(path)}
        if actual != expected:
            failures.append(f"{name}: expected {expected}, got {actual}")

    extracted_path = args.bundle / "extracted-point.json"
    if extracted_path.exists():
        extracted = json.loads(extracted_path.read_bytes())
        if extracted["input"]["sha256"] != sha256(args.bundle / "response.json"):
            failures.append("extracted-point.json references the wrong response digest")
        raw = json.loads((args.bundle / "response.json").read_bytes())
        points = raw["value"]["timeSeries"][0]["values"][0]["value"]
        source = extracted["source_fields"]
        requested_timestamp = extracted["transformation"]["parameters"]["timestamp"]
        if source["source_datetime"] != requested_timestamp:
            failures.append("extracted source timestamp differs from the recorded extraction parameter")
        if not any(
            p["dateTime"] == source["source_datetime"]
            and p["value"] == source["value_exact_source_text"]
            and p.get("qualifiers", []) == source["qualifiers"]
            for p in points
        ):
            failures.append("extracted point does not exactly match a raw source point")

    if failures:
        raise SystemExit("VERIFICATION FAILED\n" + "\n".join(failures))
    print("VERIFIED: artifact sizes and SHA-256 digests match the manifest")
    if extracted_path.exists():
        print("VERIFIED: extracted timestamp, exact value text, and qualifiers occur in the raw response")
    print("NOT VERIFIED: publisher identity, measurement accuracy, calibration, completeness, or truth")


if __name__ == "__main__":
    main()
