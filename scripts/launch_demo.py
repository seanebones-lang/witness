#!/usr/bin/env python3
"""Run a disposable, loopback-only Witness demo using the real CLI and API.

All measurements and narrative in this demo are synthetic fixtures. No provider
API, external account, production database, or scientific result is involved.
"""

import argparse
import json
import os
from pathlib import Path
import re
import socket
import subprocess
import sys
import tempfile
import time
from urllib.error import HTTPError, URLError
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]


def request(base_url, route, body=None, as_json=True):
    data = None if body is None else json.dumps(body).encode()
    req = Request(base_url + route, data=data,
                  headers={"Content-Type": "application/json"} if data else {})
    with urlopen(req, timeout=5) as response:
        return json.load(response) if as_json else response.read().decode()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--port", type=int, default=4360)
    parser.add_argument("--check-only", action="store_true",
                        help="Verify the demo, then stop and remove its data")
    parser.add_argument("--skip-build", action="store_true",
                        help="Use existing debug binaries instead of building")
    parser.add_argument("--report", type=Path,
                        help="Save a JSON verification report (no signing key)")
    args = parser.parse_args()
    if not 1024 <= args.port <= 65535:
        parser.error("Use a port between 1024 and 65535")
    # Refuse an occupied port instead of accidentally seeding another server.
    with socket.socket() as probe:
        probe.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        probe.bind(("127.0.0.1", args.port))

    env = dict(os.environ, SQLX_OFFLINE="true")
    env.pop("WITNESS_KEY", None)
    env.pop("WITNESS_DB", None)
    if not args.skip_build:
        subprocess.run(["cargo", "build", "--workspace", "--locked"],
                       cwd=ROOT, env=env, check=True)
    target = Path(env.get("CARGO_TARGET_DIR", str(ROOT / "target")))
    if not target.is_absolute():
        target = ROOT / target
    suffix = ".exe" if os.name == "nt" else ""
    cli = target / "debug" / ("witness-ingestion" + suffix)
    api = target / "debug" / ("witness-api" + suffix)
    if not cli.is_file() or not api.is_file():
        raise RuntimeError("Debug binaries missing; rerun without --skip-build")

    base_url = "http://127.0.0.1:" + str(args.port)
    with tempfile.TemporaryDirectory(prefix="witness-launch-demo-") as temp:
        private = Path(temp)
        os.chmod(private, 0o700)
        database = private / "demo.db"
        db_url = "sqlite://" + str(database)
        key = private / "demo.key"
        # Never print the private key or inherit a user's signing configuration.
        subprocess.run([str(cli), "--database", db_url, "gen-key", "--output", str(key)],
                       cwd=ROOT, env=env, check=True, capture_output=True)
        os.chmod(key, 0o600)

        def observe(value, measured_at):
            command = [str(cli), "--database", db_url, "--key-file", str(key),
                       "--dataset-status", "demo-fixture", "observe",
                       "--quantity", "air_temperature", "--value", str(value),
                       "--unit", "celsius", "--measured-at", measured_at,
                       "--instrument-id", "synthetic-demo-instrument",
                       "--author-id", "nexteleven-demo",
                       "--author-name", "NextEleven synthetic demo",
                       "--domain", "launch-demo",
                       "--labels", "synthetic-measurement",
                       "--source-uri", "https://github.com/seanebones-lang/witness/blob/main/scripts/launch_demo.py"]
            result = subprocess.run(command, cwd=ROOT, env=env, check=True,
                                    capture_output=True, text=True)
            match = re.search(r"Ingested observation: ([0-9a-f-]{36})", result.stdout)
            if not match:
                raise RuntimeError("Observation ingestion did not return a record ID")
            return match.group(1)

        first = observe(20, "2026-10-01T09:00:00Z")
        second = observe(22, "2026-10-01T10:00:00Z")
        server_env = dict(env, DATABASE_URL=db_url, HOST="127.0.0.1", PORT=str(args.port))
        with (private / "server.log").open("w") as log:
            server = subprocess.Popen([str(api)], cwd=ROOT, env=server_env,
                                      stdout=log, stderr=subprocess.STDOUT)
            try:
                for _ in range(100):
                    if server.poll() is not None:
                        raise RuntimeError("Witness server exited during startup")
                    try:
                        if request(base_url, "/health", as_json=False) != "OK":
                            raise RuntimeError("Unexpected health response")
                        break
                    except (URLError, TimeoutError):
                        time.sleep(0.1)
                else:
                    raise RuntimeError("Witness server did not become healthy")

                # Confirm we reached this isolated database before making writes.
                initial = request(base_url, "/api/nodes")
                if {n["id"] for n in initial["nodes"]} != {first, second}:
                    raise RuntimeError("Demo database identity check failed")

                # Exercise the public HTTP adapters, not just service methods.
                for value, measured_at in (("20", "not-a-date"), ("NaN", "2026-10-01T09:00:00Z"), (" ", "2026-10-01T09:00:00Z")):
                    try:
                        request(base_url, "/api/ingest/observation", {
                            "quantity": "temperature", "value": value,
                            "measured_at": measured_at, "instrument_id": "fixture", "author_id": "demo"
                        })
                    except HTTPError as error:
                        if error.code != 400:
                            raise
                    else:
                        raise RuntimeError("Malformed observation was unexpectedly accepted")
                page_query = "{ observations(first: 1) { nodes { id } pageInfo { endCursor } } }"
                page = request(base_url, "/graphql", {"query": page_query})
                if page.get("errors"):
                    raise RuntimeError("GraphQL first page failed")
                cursor = page["data"]["observations"]["pageInfo"]["endCursor"]
                next_query = "{ observations(first: 1, after: " + json.dumps(cursor) + ") { nodes { id } } }"
                next_page = request(base_url, "/graphql", {"query": next_query})
                if next_page.get("errors") or not next_page["data"]["observations"]["nodes"]:
                    raise RuntimeError("GraphQL next page failed")
                if page["data"]["observations"]["nodes"][0]["id"] == next_page["data"]["observations"]["nodes"][0]["id"]:
                    raise RuntimeError("GraphQL cursor repeated the first record")

                inference_body = {
                    "claim": "The mean of the two synthetic readings is 21 degrees Celsius.",
                    "methodology": "Arithmetic mean: (20 + 22) / 2 = 21. Demonstration only.",
                    "premises": [first, second],
                    "author_id": "nexteleven-demo", "author_name": "NextEleven synthetic demo",
                    "domain": "launch-demo", "labels": ["demo-fixture", "declared-method"],
                    "falsifiers": [{"description": "A different source value or arithmetic error would invalidate this mean.",
                                    "measurement_type": "source-and-arithmetic-review"}]
                }
                invalid = dict(inference_body, premises=["00000000-0000-0000-0000-000000000001"])
                try:
                    request(base_url, "/api/ingest/inference", invalid)
                except HTTPError as error:
                    if error.code != 400:
                        raise
                    missing_premise_rejected = True
                else:
                    raise RuntimeError("Missing-premise inference was unexpectedly accepted")
                inference = request(base_url, "/api/ingest/inference", inference_body)
                generation = request(base_url, "/api/ingest/generation", {
                    "content": "Illustrative summary: two synthetic readings average 21 degrees Celsius. This text is not a new measurement.",
                    "generator": "hand-authored-demo", "model_version": "fixture-v1",
                    "prompt": "Explain the illustrative result while keeping it separate from evidence.",
                    "human_reviewed": False,
                    "author_id": "nexteleven-demo", "author_name": "NextEleven synthetic demo",
                    "domain": "launch-demo", "labels": ["demo-fixture", "synthetic-narrative"]
                })
                all_nodes = []
                counts = {}
                for kind, expected in (("observed", 2), ("inferred", 1), ("generated", 1)):
                    filtered = request(base_url, "/api/nodes?type=" + kind)
                    if filtered["total_count"] != expected or len(filtered["nodes"]) != expected:
                        raise RuntimeError("Unexpected record count for " + kind)
                    counts[kind] = expected
                    all_nodes.extend(filtered["nodes"])
                if len(all_nodes) != 4:
                    raise RuntimeError("Rejected inference left an unexpected record")
                inferred = next(n for n in all_nodes if n["id"] == inference["id"])
                if set(inferred["parents"]) != {first, second}:
                    raise RuntimeError("Declared premises were not preserved")
                falsifiers = request(base_url, "/api/nodes/" + inference["id"] + "/falsifiers")
                if len(falsifiers["falsifiers"]) != 1:
                    raise RuntimeError("Falsifier was not preserved")
                observed = [n for n in all_nodes if n["epistemic_type"].lower() == "observed"]
                if not all(n["signature"] for n in observed):
                    raise RuntimeError("CLI did not preserve demo signature data")
                report = {"url": base_url, "counts": counts,
                          "missing_premise_rejected": missing_premise_rejected,
                          "declared_parents_preserved": True, "falsifier_preserved": True,
                          "signature_data_present_on_observations": True,
                          "signature_verified_by_dashboard": False,
                          "fixture_only": True, "isolated_temporary_database": True,
                          "malformed_observations_rejected": True,
                          "graphql_cursor_roundtrip_verified": True,
                          "inference_id": inference["id"], "generation_id": generation["id"],
                          "boundary": "Synthetic fixtures. Integrity and declared links do not establish truth, identity, or scientific validity."}
                if args.report:
                    args.report.parent.mkdir(parents=True, exist_ok=True)
                    args.report.write_text(json.dumps(report, indent=2) + "\n")
                print("VERIFIED: 2 Observed / 1 Inferred / 1 Generated fixture records", flush=True)
                print("VERIFIED: missing premise rejected; declared parents and falsifier preserved", flush=True)
                print("BOUNDARY: demo signature data is present; the dashboard does not verify it", flush=True)
                if not args.check_only:
                    print("\nOpen " + base_url + "/", flush=True)
                    print("Choose each category, open a record, and inspect Payload, Provenance, and Falsifiers.", flush=True)
                    print("Synthetic fixtures only. Control-C stops the server and removes the demo database and key.", flush=True)
                    while server.poll() is None:
                        time.sleep(0.5)
                    if server.returncode:
                        raise RuntimeError("Witness server stopped unexpectedly")
            finally:
                if server.poll() is None:
                    server.terminate()
                    try:
                        server.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        server.kill()
                        server.wait()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nDemo stopped; temporary data removed.")
    except (OSError, RuntimeError, subprocess.CalledProcessError) as error:
        print("Demo failed: " + str(error), file=sys.stderr)
        sys.exit(1)
