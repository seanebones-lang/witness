# Install and run Witness

Witness is a local prototype. It binds to loopback by default and is not safe
to expose to a network: its current write APIs do not provide authentication or
authorization.

## Fastest safe local start

Prerequisite: current stable [Rust](https://rustup.rs/) with Cargo.

```bash
git clone https://github.com/seanebones-lang/witness.git && cd witness && \
touch witness.db && SQLX_OFFLINE=true cargo run --package witness-api
```

Open <http://127.0.0.1:8080/>. The server migrates the local `witness.db` file.
Stop it with `Control-C`.

## Try a populated demonstration

With Rust/Cargo and Python 3 installed, run this from the cloned repository:

```bash
python3 scripts/launch_demo.py
```

Open <http://127.0.0.1:4360/>. The launcher builds the locked Rust workspace,
creates a separate temporary database and demo signing key, and exercises the
real CLI and REST API. It creates two **synthetic** temperature readings, one
inference with declared premises and a falsifier, and one hand-authored synthetic
narrative classified as Generated. It also checks that an inference referencing
a missing premise is rejected. No model API or external account is used.

Choose Observed, Inferred, and Generated; open a card to inspect its payload,
declared provenance, and falsifiers. Signature data is present on the two
observations, but the dashboard does not verify signatures on read. These
fixtures establish no real measurement, identity, scientific finding, or
independent review.

Stop with `Control-C`; the launcher removes its temporary database and key.
Your existing `witness.db` is not used. Choose another port with `--port 4361` if
4360 is occupied. Dashboard styling and interactivity currently require access
to third-party CDNs; the CLI and API demonstration run locally.

For an automated first-use check after building:

```bash
python3 scripts/launch_demo.py --skip-build --check-only --port 18081
```

## Verify the repository before use

```bash
SQLX_OFFLINE=true cargo fmt --all -- --check && \
SQLX_OFFLINE=true cargo build --workspace --locked && \
SQLX_OFFLINE=true cargo test --workspace --locked && \
SQLX_OFFLINE=true cargo clippy --workspace --all-targets --locked -- -D warnings && \
python3 reference/usgs-river-gauge/tools/verify.py --bundle reference/usgs-river-gauge/snapshot
```

The final verifier checks committed demonstration bytes and extraction against
their recorded digest. It does **not** establish source identity, measurement
accuracy, calibration, completeness, or truth.

## Run a separate local database

Use a named database for a review, experiment, or integration test rather than
mixing it with another local run:

```bash
touch review.db && DATABASE_URL="sqlite://$PWD/review.db" PORT=18080 \
  cargo run --release --package witness-api
```

Then check that it is alive:

```bash
curl --fail http://127.0.0.1:18080/health
```

## REST ingestion contract

The public REST contract and copyable examples are in [README.md](README.md#rest-ingest-endpoints).
Use only loopback URLs while the prototype remains unauthenticated. A returned
CID establishes a relationship to stored bytes; it does not establish that a
record is true, independently sourced, or fit for a consequential decision.

## Proof A integration

The Humanity Grid integration guide is in its
[Proof A review guide](https://github.com/seanebones-lang/Humanity-Grid/blob/main/docs/PROOF_A_REVIEW.md).
It can replay the frozen 12 Observed / 2 Inferred / 1 Generated record graph
into a fresh local Witness database. That replay verifies the ingestion contract
and graph structure. It does not independently rerun or validate molecular
docking.

## Troubleshooting

| Symptom | Cause and resolution |
| --- | --- |
| `sqlx` cannot open the database | Use a writable location, create the file with `touch review.db`, then use the explicit `DATABASE_URL` command above. |
| `address already in use` | Pick another port, for example `PORT=18080`. |
| Browser shows no records | A new database is empty by design. Ingest a record, run the demo seed script, or use the Proof A replay guide. |
| REST request returns `400` | Check required fields, UUID premise IDs, and supported type strings. Inferences cannot reference missing premises. |
| REST request returns `500` | Treat it as a prototype failure. Keep the request body and local logs, then open a public issue only if it contains no sensitive material. |
| You need network access | Do not expose this server. Authentication, authorization, rate limits, transport configuration, and deployment review are not complete. |

## Frequently asked questions

**Does Witness prove a scientific claim?** No. It records classifications,
payloads, signatures where present, declared provenance edges, and explicit
methods or falsifiers. It does not prove a source, identity, measurement, or
conclusion is true.

**Can I use it with sensitive or person-concerning material?** No. Follow the
public boundaries in [README.md](README.md) and the private reporting process in
[SECURITY.md](SECURITY.md).

**Is this a production service?** No. It is a reviewer-accessible prototype.
The [roadmap](ROADMAP.md) lists the implementation, review, governance, and
security gates before any high-consequence use.
