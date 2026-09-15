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

    package_profile = {
        "schema": "witness-curated-preservation-package/0.1",
        "status": "curated-demo-not-oais-premis-bagit-warc-or-trusted-repository-conformance",
        "designated_community": "Witness protocol contributors evaluating source preservation and transformation semantics",
        "custody_and_authority": {
            "witness_role": "Maintainer of this curated demonstration copy, not archival institution of record for USGS",
            "source_custody_transfer_claimed": False,
            "source_endorsement_claimed": False,
            "witness_signature_from_source_present": False,
        },
        "objects": [
            {
                "path": "response.json",
                "role": "preservation-master-as-received",
                "representation_information": {
                    "format": "JSON",
                    "media_type": headers.get("content-type"),
                    "source_schema_or_vocabulary": "USGS WaterML 1.1 JSON response as identified inside the preserved response",
                },
            },
            {
                "path": "response-headers.json",
                "role": "normalized-retrieval-metadata",
                "representation_information": {
                    "format": "JSON",
                    "normalization": "HTTP response header names lowercased and serialized as sorted, indented UTF-8 JSON",
                },
            },
            {
                "path": "extracted-point.json",
                "role": "derivative-extraction",
                "representation_information": {
                    "format": "JSON",
                    "transformation": "witness-usgs-extractor/0.1.0",
                },
            },
        ],
        "rights_and_terms": {
            "determination_status": "not-independently-adjudicated",
            "note": "The response came from a U.S. government service. This package makes no blanket rights determination for every field or upstream contribution and records no custody transfer or endorsement.",
            "source_policy_review_required_before_expansion": True,
        },
    }
    package_bytes = canonical_json(package_profile)
    (args.output / "package-profile.json").write_bytes(package_bytes)

    events = {
        "schema": "witness-preservation-events/0.1",
        "events": [
            {
                "event_type": "intake",
                "recorded_at": retrieved_at,
                "agent": TOOL_VERSION,
                "source_location": final_url,
                "object": "response.json",
                "outcome": "stored-as-received",
                "authority_and_terms": "Retrieved from a public endpoint for a curated demonstration; no custody transfer, source signature, endorsement, or archival-institution status claimed.",
            }
        ],
    }
    event_bytes = canonical_json(events)
    (args.output / "preservation-events.json").write_bytes(event_bytes)

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
            "package-profile.json": {"bytes": len(package_bytes), "sha256": sha256(package_bytes)},
            "preservation-events.json": {"bytes": len(event_bytes), "sha256": sha256(event_bytes)},
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
