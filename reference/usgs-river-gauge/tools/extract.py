#!/usr/bin/env python3
"""Extract one source point without treating the extraction as an observation."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path

TOOL_VERSION = "witness-usgs-extractor/0.1.0"


def canonical_json(value: object) -> bytes:
    return (json.dumps(value, indent=2, sort_keys=True, ensure_ascii=False) + "\n").encode()


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle", type=Path, required=True)
    parser.add_argument("--timestamp", default="2024-07-01T00:00:00.000-07:00")
    args = parser.parse_args()

    raw = (args.bundle / "response.json").read_bytes()
    document = json.loads(raw, parse_float=str, parse_int=str)
    series = document["value"]["timeSeries"]
    if len(series) != 1:
        raise SystemExit(f"expected exactly one time series, found {len(series)}")
    series = series[0]
    points = series["values"][0]["value"]
    matches = [point for point in points if point["dateTime"] == args.timestamp]
    if len(matches) != 1:
        raise SystemExit(f"expected exactly one matching point, found {len(matches)}")
    point = matches[0]
    variable = series["variable"]
    source = series["sourceInfo"]
    qualifier_definitions = {
        item["qualifierCode"]: {
            "description": item.get("qualifierDescription"),
            "network": item.get("network"),
            "vocabulary": item.get("vocabulary"),
        }
        for item in series["values"][0].get("qualifier", [])
        if item.get("qualifierCode") in point.get("qualifiers", [])
    }

    extracted = {
        "schema": "witness-source-extraction/0.1",
        "dataset_status": "curated-demo",
        "not_a_conforming_observation": True,
        "input": {"path": "response.json", "sha256": hashlib.sha256(raw).hexdigest()},
        "transformation": {
            "tool": TOOL_VERSION,
            "operation": "Select the single time-series point whose dateTime exactly equals the requested timestamp; copy source fields without numeric conversion.",
            "parameters": {"timestamp": args.timestamp},
        },
        "source_fields": {
            "site_code": source["siteCode"][0]["value"],
            "site_name": source.get("siteName"),
            "latitude": str(source["geoLocation"]["geogLocation"]["latitude"]),
            "longitude": str(source["geoLocation"]["geogLocation"]["longitude"]),
            "geospatial_srs": source["geoLocation"]["geogLocation"].get("srs"),
            "parameter_code": variable["variableCode"][0]["value"],
            "variable_name": variable.get("variableName"),
            "value_type": variable.get("valueType"),
            "unit_code": variable["unit"].get("unitCode"),
            "no_data_value": str(variable.get("noDataValue")),
            "value_exact_source_text": point["value"],
            "qualifiers": point.get("qualifiers", []),
            "qualifier_definitions": qualifier_definitions,
            "source_declared_approval_state": qualifier_definitions.get("A", {}).get("description"),
            "source_datetime": point["dateTime"],
        },
        "claims_and_limits": {
            "approval_interpretation": "The source response defines qualifier A as approved for publication. This is preserved as a source-declared state; it is not a Witness approval, accuracy finding, or endorsement.",
            "measurement_profile_status": "fails-draft-profile",
            "reason": "The current Witness prototype lacks a ratified instrument profile, authoritative exact-decimal carrier, complete retrieval/custody semantics, and verified datum/calibration interpretation.",
        },
    }
    output = args.bundle / "extracted-point.json"
    output_bytes = canonical_json(extracted)
    output.write_bytes(output_bytes)

    manifest_path = args.bundle / "manifest.json"
    manifest = json.loads(manifest_path.read_bytes())
    manifest["artifacts"]["extracted-point.json"] = {
        "bytes": len(output_bytes),
        "sha256": hashlib.sha256(output_bytes).hexdigest(),
    }
    manifest_path.write_bytes(canonical_json(manifest))
    print(output)


if __name__ == "__main__":
    main()
