# Security Policy

Witness is an early public prototype for provenance records. Security reports
help define the trustworthiness the project must earn before public deployment.

## Supported versions

Only the current `main` branch receives security fixes. There is no stable or
production-supported release yet.

## Report a vulnerability privately

Use GitHub's **Report a vulnerability** form in the repository Security tab:

<https://github.com/seanebones-lang/witness/security/advisories/new>

Include the affected revision, entry point, realistic impact, prerequisites, and
the smallest safe reproduction you can provide. Do not include private keys,
personal data, confidential evidence, or unnecessary exploit data. You may use a
pseudonym. The maintainer will acknowledge a report when it is reviewed and will
coordinate disclosure timing with the reporter when contact is possible.

Do not open a public issue for an unpatched vulnerability that could expose data,
forge or corrupt records, bypass a trust boundary, or compromise signing keys.

## System and scope

The current repository contains a local CLI, a Rust library, a SQLite database,
a REST and GraphQL server, and a browser dashboard. The CLI accepts files,
record fields, database paths, and optional signing-key paths
(`ingestion/src/main.rs:9-129`). The server exposes unauthenticated reads and a
GraphQL observation mutation (`api/src/main.rs:134-317`). Ingestion validates
JSONL content identifiers and existing signatures and checks inference premises
(`core/src/ingestion.rs:48-112`). Storage persists nodes, graph edges, and
narrative diffs (`core/src/storage.rs:15-253`).

The prototype is intended for local development and binds to `127.0.0.1` by
default. A non-loopback `HOST` override does not add authentication,
authorization, rate limits, transport security, or production support. Direct
network exposure and high-consequence use are unsupported.

## Threat model and trust boundaries

Treat every CLI argument, imported file, API field, source URI, database row,
stored JSON value, and cryptographic field as attacker-controlled. Treat the
local operator, host, database file, configured signing-key file, build pipeline,
and GitHub maintainer account as privileged. A signature does not establish the
signer's real-world identity or honesty.

The reusable architecture and attacker stories are in
[docs/THREAT-MODEL.md](docs/THREAT-MODEL.md).

## Security invariants

- A record must not be silently overwritten or reclassified.
- Malformed or unsupported stored data must return an error, not become an
  observation, disappear silently, or crash a normal read path.
- A content identifier or signature must fail closed when its input changes.
- A signed record must bind every field claimed by the signature contract.
- An inference must not be accepted when a declared premise is absent.
- Generated content must not cross a boundary as observed content without a new,
  explicit observation record.
- Corrections must preserve the record they address.
- Private signing keys must never be logged, committed, returned through an API,
  or placed in fixtures.
- Verification signals must not be represented as proof of factual truth.
- Unauthenticated mutation, unbounded input, or untrusted rendering is
  reportable whenever the affected interface is reachable.

## Reportable findings and severity context

Report vulnerabilities that permit record forgery, undetected mutation or
deletion, epistemic reclassification, signature or identifier bypass, signing-key
exposure, unauthorized access or mutation, injection, unsafe file or network
access, denial of service through realistic inputs, or disclosure of restricted
evidence.

Impact increases when a flaw crosses an authentication or custody boundary,
affects many records, survives export or replication, or causes consumers to
accept invalid evidence. A flaw requiring an operator who already controls the
database has a different severity from a remote unauthenticated path, but silent
corruption and misleading verification remain relevant because independent
inspection is a core purpose of the project.

## Known limitations and accepted prototype risk

The server has no authentication, authorization, rate limiting, tenant isolation,
privacy controls, key revocation, or production operations. The database is a
mutable local SQLite file. Canonical JSON is implementation-specific
(`core/src/signing.rs:93-130`) and has no cross-language conformance guarantee.
These are documented release blockers, not evidence that related vulnerabilities
are harmless. Reports that identify an unexpected capability, bypass, or unsafe
claim within these known boundaries remain welcome.

Dependency vulnerabilities without a reachable security impact may be handled as
maintenance. Social disputes about the factual truth of submitted material are
outside a software vulnerability report unless a security control or documented
trust guarantee is also broken.
