# Witness Roadmap

This roadmap moves Witness from an early local prototype toward a system that
can support careful public evaluation. It is ordered by trust dependency:
features that make Witness easier to use must not outrun the guarantees needed
to use it responsibly.

The roadmap implements the intention in [MANIFESTO.md](MANIFESTO.md): keep the
observations available, expose the move from evidence to claim, label generated
material, and allow the next observation to challenge what came before.

Roadmap items describe intended work, not shipped capabilities. Completion
requires implementation, review, tests, documentation, and evidence that the
acceptance criteria hold.

## Current state: prototype

Available today:

- Rust types for observed, inferred, and generated provenance nodes.
- SQLite persistence and graph edges.
- SHA-256 content identifiers and optional Ed25519 signatures.
- CLI ingestion for observations, CSV, JSONL, inferences, and generations.
- REST and GraphQL read paths, GraphQL observation ingestion, and a local
  dashboard.
- Initial signature and tamper-detection tests.

The prototype is suitable for local development and design evaluation. It is
not approved for high-consequence or public production use.

## Phase 0 — Define the public contract

**Goal:** Make the project's promises, ownership, and safe-use boundary explicit.

- [ ] Choose and publish an open-source license.
- [ ] Add a code of conduct and documented enforcement contact.
- [ ] Add a private security-reporting channel and `SECURITY.md`.
- [ ] Publish a threat model covering malicious submitters, compromised keys,
  insider modification, replay, deletion, impersonation, poisoned sources,
  privacy harm, and denial of service.
- [ ] Specify the meaning and limits of observed, inferred, and generated.
- [ ] Define the governance process for schema and trust-model changes.
- [ ] Define which use cases remain out of scope.

**Exit criteria:** A contributor can explain what Witness promises, what it does
not promise, how decisions are made, and how to report harm or vulnerabilities.

## Phase 1 — Make the record internally trustworthy

**Goal:** Eliminate silent corruption, ambiguous identifiers, and mutable
history inside one Witness instance.

- [ ] Replace database decoding panics with typed, diagnosable errors.
- [ ] Enforce append-only node storage; represent corrections as new linked
  records.
- [ ] Validate parent existence, edge direction, and epistemic constraints.
- [ ] Define canonical JSON and produce cross-language compatibility fixtures.
- [ ] Recompute and verify content identifiers when records enter or leave
  storage.
- [ ] Verify signatures at ingestion and read boundaries.
- [ ] Make signed, unsigned, invalid, revoked, and unverifiable trust states
  explicit.
- [ ] Remove duplicate placeholder node metadata from typed payloads.
- [ ] Make migrations reversible where possible and test upgrades from every
  published schema version.
- [ ] Add integration tests for CLI, storage, REST, GraphQL, and dashboard paths.

**Exit criteria:** A stored record cannot be silently overwritten; corrupted or
invalid records fail visibly; identifiers and signatures reproduce across
supported implementations.

## Phase 2 — Identity, custody, and accountability

**Goal:** Allow users to understand who controlled a key and how a record moved
from source to system.

- [ ] Define pluggable identity attestations for people, institutions,
  instruments, models, and software.
- [ ] Add key rotation, expiration, revocation, recovery, and compromise events.
- [ ] Model collection, transformation, custody, and import as signed events.
- [ ] Preserve source retrieval time, source digest, media type, and importer
  version.
- [ ] Add reproducible transformation records for derived data.
- [ ] Add roles and least-privilege authorization.
- [ ] Create a public verification command that works without running the server.

**Exit criteria:** A verifier can distinguish content integrity from identity
assurance, inspect key history, and reconstruct the custody path.

## Phase 3 — Correction, disagreement, and falsification

**Goal:** Preserve changing knowledge without hiding earlier states.

- [ ] Implement explicit `updates`, `contradicts`, `retracts`, `replicates`, and
  `supersedes` relationships.
- [ ] Require reasons and authorship for corrections and retractions.
- [ ] Make falsifiers first-class records with status history.
- [ ] Support multiple competing inferences over the same evidence.
- [ ] Display unresolved disagreement without collapsing it into one score.
- [ ] Record reviews, challenges, responses, and adjudication without deleting
  the original exchange.
- [ ] Add reproducible narrative diffs with signed editor attribution.

**Exit criteria:** A user can follow a claim through challenge, correction,
retraction, or confirmation while retaining the full prior record.

## Phase 4 — Privacy, consent, and protection from harm

**Goal:** Support public accountability without treating permanent exposure as
an automatic good.

- [ ] Create a privacy and human-rights impact assessment with outside review.
- [ ] Define data minimization and purpose-limitation rules.
- [ ] Support consent, consent withdrawal, restricted records, and graduated
  disclosure.
- [ ] Design redaction and safe-deletion governance that preserves an auditable
  event without preserving harmful content indefinitely.
- [ ] Add protections for minors, vulnerable people, witnesses, and sensitive
  locations.
- [ ] Add abuse reporting, rate limits, moderation workflows, and appeals.
- [ ] Document jurisdictional and domain-specific retention requirements.

**Exit criteria:** A deployment can state whose data it holds, why it holds it,
who can access it, how harm is reported, and how privacy rights are exercised.

## Phase 5 — Interoperability and resilient public infrastructure

**Goal:** Let independent implementations exchange and verify records without
depending on one operator.

- [ ] Publish a versioned protocol and JSON schemas.
- [ ] Build conformance fixtures and an implementation test suite.
- [ ] Add export, import, backup, restoration, and deterministic replay.
- [ ] Support replication with conflict and fork visibility.
- [ ] Define portable bundles that include records, edges, signatures, schemas,
  and verification metadata.
- [ ] Add observability, capacity testing, recovery exercises, and operational
  runbooks.
- [ ] Complete accessibility and internationalization reviews.

**Exit criteria:** Two independent implementations can exchange a record bundle,
reach the same verification result, and disclose any history fork.

## Phase 6 — Domain pilots and independent evaluation

**Goal:** Test whether Witness helps real communities inspect evidence without
creating unacceptable new risks.

- [ ] Select small, reversible pilots with willing domain partners.
- [ ] Define success, failure, and stop conditions before each pilot.
- [ ] Conduct security, privacy, usability, accessibility, and domain reviews.
- [ ] Measure comprehension: can people correctly distinguish observation,
  inference, and generation?
- [ ] Publish limitations, adverse findings, and negative results.
- [ ] Require explicit readiness review before any high-consequence expansion.

**Exit criteria:** Independent reviewers and affected participants can evaluate
the evidence, limitations, harms, and benefits of each pilot.

## Release gates

These labels prevent implementation from being mistaken for readiness:

| Stage | Meaning |
| --- | --- |
| Prototype | Local experimentation; APIs and schemas may change |
| Alpha | Trust invariants tested; controlled, noncritical evaluation only |
| Beta | Governance, privacy, security, recovery, and interoperability reviewed in limited pilots |
| Candidate | Independent assessments complete; documented release blockers closed |
| Stable | Versioned contract with supported upgrades and an operating governance process |

No calendar date overrides an unmet release gate.

## How to choose roadmap work

Prefer work that strengthens an invariant and can be tested. A useful proposal
answers:

1. What failure or harm does this prevent?
2. Which people or systems are affected?
3. What guarantee will Witness make afterward?
4. What remains outside that guarantee?
5. How will tests and reviewers determine whether it works?

See [CONTRIBUTING.md](CONTRIBUTING.md) before starting implementation.
