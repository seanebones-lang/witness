#!/usr/bin/env python3
"""Record a fixity-check event for the bounded USGS demonstration bundle."""
from __future__ import annotations

import argparse
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path

TOOL_VERSION = "witness-fixity-recorder/0.1.0"
EVENTS_FILE = "preservation-events.json"


def canonical_json(value: object) -> bytes:
    return (json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode()


def digest(path: Path) -> dict[str, object]:
    data = path.read_bytes()
    return {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle", type=Path, required=True)
    parser.add_argument("--checked-at", help="ISO-8601 time; defaults to current UTC time")
    args = parser.parse_args()

    manifest_path = args.bundle / "manifest.json"
    manifest = json.loads(manifest_path.read_bytes())
    expected = {
        name: record
        for name, record in manifest["artifacts"].items()
        if name != EVENTS_FILE
    }
    checks = []
    all_match = True
    for name, recorded in sorted(expected.items()):
        actual = digest(args.bundle / name)
        matches = actual == recorded
        all_match = all_match and matches
        checks.append({"artifact": name, "expected": recorded, "actual": actual, "matches": matches})

    checked_at = args.checked_at or datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    events_path = args.bundle / EVENTS_FILE
    if events_path.exists():
        events = json.loads(events_path.read_bytes())
    else:
        events = {"schema": "witness-preservation-events/0.1", "events": []}
    events["events"].append(
        {
            "event_type": "fixity-check",
            "recorded_at": checked_at,
            "tool": TOOL_VERSION,
            "outcome": "match" if all_match else "mismatch",
            "checks": checks,
            "limits": "This local check compares bytes with the package manifest; it does not establish source authorship, independent custody, or archival certification.",
        }
    )
    event_bytes = canonical_json(events)
    events_path.write_bytes(event_bytes)
    manifest["artifacts"][EVENTS_FILE] = {
        "bytes": len(event_bytes),
        "sha256": hashlib.sha256(event_bytes).hexdigest(),
    }
    manifest_path.write_bytes(canonical_json(manifest))

    if not all_match:
        raise SystemExit("FIXITY CHECK FAILED; mismatch event recorded")
    print(f"RECORDED: {len(checks)} artifact fixity checks matched the package manifest")


if __name__ == "__main__":
    main()
