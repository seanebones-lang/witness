# Working on Witness

## Start here

Read [README.md](README.md), [SECURITY.md](SECURITY.md) and
[the threat model](docs/THREAT-MODEL.md). This is a local provenance prototype,
not a production service or a truth-verification system. Inspect the working tree
before editing and preserve unrelated changes.

## Build and checks

Use current stable Rust/Cargo, Node 22+ for dashboard checks and Python 3 for
local acceptance helpers. From the root:

```sh
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
node --test dashboard/tests/*.test.cjs
python3 reference/usgs-river-gauge/tools/verify.py --bundle reference/usgs-river-gauge/snapshot
```

For dashboard changes, also run:

```sh
npm ci --ignore-scripts --prefix dashboard
npm audit --prefix dashboard
npm run build --prefix dashboard
git diff --exit-code -- dashboard/assets
```

[CI](.github/workflows/ci.yml) includes Windows storage regressions, a disposable
first-use demo and `cargo audit --deny warnings`. Dependency advisory access can
require network connectivity. The public source-reference verifier checks stored
bytes; it does not establish fresh upstream data or factual truth.

## Invariants

- Keep Observed, Inferred and Generated records distinct. Generated text cannot
  become an observation without an explicit observation record.
- Preserve immutable records, correction links, premise existence, canonical
  identifiers and the complete signed-field contract. Malformed/unsupported
  storage must return an error, not disappear or silently become valid data.
- A matching hash or signature establishes a bounded integrity relationship,
  not the signer's identity, honesty or a scientific claim's validity.
- Keep private signing keys out of logs, APIs and fixtures. Use fresh disposable
  SQLite databases and fictional records; do not mutate an existing review store.
- The prototype lacks production authentication, authorization, tenant isolation
  and operations. Run local servers on loopback only. Do not expose write routes
  publicly or suggest that a successful local demo closes those release blockers.

## Handoff

Report the exact revision, changed contracts, commands and outcomes, including
platform and skipped gates. Preserve MPL-2.0 and third-party attribution. Treat
source-content verification, replay, independent reproduction and scientific
validation as separate evidence claims.
