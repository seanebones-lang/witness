#!/usr/bin/env python3
"""Retrieve and preserve one bounded USGS instantaneous-values response."""
from __future__ import annotations

import argparse
import hashlib
import json
from datetime import datetime, timezone
from pathlib import Path
from urllib.request import Request, urlopen

REQUEST_URL = (
    "https://waterservices.usgs.gov/nwis/iv/"
    "?format=json&sites=09380000&parameterCd=00060"
    "&startDT=2024-07-01&endDT=2024-07-02&siteStatus=all"
)
TOOL_VERSION = "witness-usgs-retriever/0.1.0"


def canonical_json(value: object) -> bytes:
    return (json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode()


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)

    retrieved_at = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    request = Request(REQUEST_URL, headers={"User-Agent": TOOL_VERSION, "Accept": "application/json"})
    with urlopen(request, timeout=30) as response:
        raw = response.read()
        final_url = response.geturl()
        status = response.status
        headers = {key.lower(): value for key, value in response.headers.items()}

    (args.output / "response.json").write_bytes(raw)
    header_bytes = canonical_json(headers)
    (args.output / "response-headers.json").write_bytes(header_bytes)

    manifest = {
        "schema": "witness-source-artifact-manifest/0.1",
        "dataset_status": "curated-demo",
        "source": {
            "publisher_claimed": "U.S. Geological Survey",
            "request_url": REQUEST_URL,
            "final_url": final_url,
            "http_status": status,
            "retrieved_at": retrieved_at,
            "media_type": headers.get("content-type"),
            "site_code": "09380000",
            "parameter_code": "00060",
            "requested_interval": {"start": "2024-07-01", "end": "2024-07-02"},
        },
        "retrieval": {
            "tool": TOOL_VERSION,
            "request_headers": {"accept": "application/json", "user-agent": TOOL_VERSION},
        },
        "artifacts": {
            "response.json": {"bytes": len(raw), "sha256": sha256(raw)},
            "response-headers.json": {"bytes": len(header_bytes), "sha256": sha256(header_bytes)},
        },
        "claims_and_limits": {
            "usgs_signature_present": False,
            "usgs_review_or_endorsement": False,
            "retrieval_proves": "These bytes were returned by the recorded HTTPS request at the recorded time, subject to the limits of the local client and TLS trust store.",
            "retrieval_does_not_prove": "USGS authorship of every field, measurement accuracy, station identity, calibration, completeness, or Witness conformance.",
        },
    }
    (args.output / "manifest.json").write_bytes(canonical_json(manifest))
    print(args.output / "manifest.json")


if __name__ == "__main__":
    main()
