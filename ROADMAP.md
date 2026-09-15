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

## Epistemic constitution

Witness needs a small set of invariants that remain stable beneath changing
schemas, interfaces, institutions, and implementations. The initial
constitution should establish that:

1. A generated record cannot become an observation through relabeling. A new
   observation process and record are required.
2. An inference identifies its premises and method. Missing premises remain
   visible.
3. A correction, contradiction, or retraction does not erase the record it
   addresses.
4. A content identifier proves a relationship to bytes. It does not prove that
   the bytes describe reality.
5. A signature proves control of a signing key at signing time. It does not by
   itself prove identity, honesty, expertise, or truth.
6. Unknown information remains unknown. The system does not silently convert
   missing evidence into evidence of absence.
7. Confidence remains attached to its method, assumptions, scope, and author.
   It is not a substitute for them.
8. Every automated transformation identifies the software, version, inputs,
   parameters, and time that produced it.
9. Reputation, popularity, institutional status, or consensus cannot override
   an invalid record or conceal disagreement.
10. Every implementation and deployment discloses material departures from the
    protocol and constitution.

Changing these invariants should require a public proposal, compatibility and
harm analysis, a recorded decision, and review by people affected by the
change. A conforming implementation must be able to expose which constitutional
version it follows.

## Cross-cutting design commitments

These concerns apply across every phase and should be included in feature
design, review, and testing.

### Evidence profiles

Different observations require different evidence. A telescope frame, human
testimony, hospital census, laboratory assay, software calculation, and
historical document should share a provenance core without pretending they
share one collection method.

Witness should support versioned evidence profiles that define:

- Required and optional metadata.
- Collection and custody expectations.
- Applicable uncertainty and calibration information.
- Known failure modes and adversarial cases.
- Privacy, consent, retention, and disclosure requirements.
- Domain review and verification procedures.

Initial profiles should cover instrument measurements, human testimony,
clinical evidence, public statistics, archival documents, remote sensing, and
software computation. Profiles must extend the common record without weakening
the epistemic constitution.

### Verification is a vector, not a truth score

Witness should never compress trust into one authoritative number. It should
expose independent dimensions such as:

| Dimension | Example states |
| --- | --- |
| Content integrity | verified, invalid, unavailable |
| Signature | valid, invalid, unsigned, revoked, unknown |
| Identity assurance | self-asserted, attested, independently verified, disputed |
| Source availability | available, restricted, missing, destroyed |
| Custody completeness | complete, partial, absent, disputed |
| Calibration | current, expired, unavailable, inapplicable |
| Replication | untested, replicated, failed replication, contested |
| Privacy | public, restricted, redacted, sealed, consent withdrawn |

Interfaces may summarize these dimensions for accessibility, but must preserve
the underlying states and must not present their combination as probability of
truth.

### Honest absence

Silence can be the correct result. Witness should distinguish:

- No relevant observation was found within a documented search scope.
- An observation was expected but not collected.
- A source is known to exist but is unavailable.
- A source was lost or destroyed.
- A source is withheld for privacy, consent, legal, or safety reasons.
- An instrument was offline during the relevant interval.
- Available evidence is insufficient to support or reject a claim.

An empty result must carry enough scope and reason metadata to prevent “nothing
was returned” from silently becoming “nothing happened.”

### Time has multiple meanings

Witness should keep distinct timestamps for when an event occurred, was
observed, was recorded, entered Witness, was signed, was published, was
corrected, and was known to a particular party. It should also support validity
intervals for claims and calibration. Interfaces and APIs must name the time
being displayed instead of collapsing these events into a generic date.

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

- [x] Choose and publish an open-source license (MPL 2.0).
- [x] Add a code of conduct and documented enforcement contact.
- [x] Add a private security-reporting channel and `SECURITY.md`.
- [x] Publish a threat model covering malicious submitters, compromised keys,
  insider modification, replay, deletion, impersonation, poisoned sources,
  privacy harm, and denial of service.
- [x] Specify the meaning and limits of observed, inferred, and generated.
- [ ] Ratify the versioned 0.1 founding draft after independent technical,
  domain, privacy, and community review.
- [x] Define the public process for constitutional amendments.
- [x] Define the governance process for schema and trust-model changes.
- [x] Document current funding, infrastructure, moderation, and decision-making
  power; update the disclosure when those facts change.
- [x] Require conflict-of-interest disclosure for maintainers and reviewers.
- [x] Define how minority reports and dissenting technical judgments are
  preserved.
- [x] Define how compatible governance forks can remain interoperable without
  hiding differences.
- [x] Limit emergency powers and require their actions and expiration to be
  recorded.
- [x] Define which use cases remain out of scope.

**Exit criteria:** A contributor can explain what Witness promises, what it does
not promise, how decisions are made, where power is concentrated, how the core
invariants can change, and how to report harm or vulnerabilities.

### Current participation and funding gate

- [x] Publish a short expert-review path for technical and nontechnical
  contributors.
- [x] Document current funding routes, eligibility limits, and evidence gaps.
- [ ] Complete at least three independent reviews across distinct disciplines.
- [ ] Record project responses, including unresolved objections and minority
  reports.
- [ ] Secure one willing partner for a small, public, reversible pilot.
- [ ] Make raw retrieved artifacts first-class records with exact bytes,
  response metadata, request details, media type, retrieval time, and digest.
- [ ] Replace binary floating-point as the authoritative measurement carrier
  with an exact, vocabulary-bound representation.
- [ ] Represent source qualifiers, approval state, sampling regime, explicit
  missingness, datum/CRS, and distinct event, recording, publication, retrieval,
  ingestion, signing, and correction times.
- [ ] Record extraction and normalization from artifact to observation as a
  versioned transformation rather than an invisible parse step.
- [ ] Require the current scalar demo fixture to fail the future instrument
  measurement profile rather than count as a conforming observation.
- [ ] Obtain a green hosted CI run after the GitHub account billing lock is
  resolved.

## Phase 1 — Make the record internally trustworthy

**Goal:** Eliminate silent corruption, ambiguous identifiers, and mutable
history inside one Witness instance.

- [x] Replace database decoding panics with typed, diagnosable errors.
- [x] Enforce append-only node storage.
- [ ] Represent corrections as new linked records.
- [ ] Validate parent existence, edge direction, and epistemic constraints.
- [ ] Define canonical JSON and produce cross-language compatibility fixtures.
- [ ] Recompute and verify content identifiers when records enter or leave
  storage.
- [ ] Verify signatures at ingestion and read boundaries.
- [ ] Make signed, unsigned, invalid, revoked, and unverifiable trust states
  explicit.
- [ ] Implement the multidimensional verification vector without a composite
  truth score.
- [ ] Model honest absence and require documented search or collection scope.
- [ ] Separate event, observation, recording, ingestion, signing, publication,
  correction, and knowledge times.
- [ ] Define a common provenance core and a versioning mechanism for evidence
  profiles.
- [ ] Publish and test initial instrument-measurement and software-computation
  profiles before attempting higher-risk profiles.
- [ ] Remove duplicate placeholder node metadata from typed payloads.
- [ ] Make migrations reversible where possible and test upgrades from every
  published schema version.
- [ ] Add integration tests for CLI, storage, REST, GraphQL, and dashboard paths.

**Exit criteria:** A stored record cannot be silently overwritten; corrupted or
invalid records fail visibly; identifiers and signatures reproduce across
supported implementations; absence, time, and verification state retain their
precise meanings.

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
- [ ] Define reproducible inference packages containing exact parent IDs, code
  or model version, parameters, dependencies, random seed where applicable,
  inclusion and exclusion criteria, human decisions, expected output digest,
  limitations, and falsifiers.
- [ ] Distinguish a described method from an independently reproduced
  derivation.
- [ ] Add roles and least-privilege authorization.
- [ ] Create a public verification command that works without running the server.

**Exit criteria:** A verifier can distinguish content integrity from identity
assurance, inspect key history, reconstruct the custody path, and reproduce a
supported computational inference from its declared package.

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
- [ ] Build a dependency index from observations to inferences, summaries,
  policies, publications, and generated explanations.
- [ ] When a source changes, identify potentially affected downstream records
  without silently rewriting them.
- [ ] Track whether each affected record is awaiting review, confirmed,
  updated, disputed, retracted, or intentionally unchanged, including the
  responsible reviewer and elapsed time.
- [ ] Detect narrative drift cases: data changed while narrative did not;
  narrative changed while data did not; methods, baselines, windows, confidence,
  records, or caveats changed.
- [ ] Report observable differences without inferring deceptive intent unless
  a separate inference supplies evidence for that claim.

**Exit criteria:** A user can follow a claim through challenge, correction,
retraction, or confirmation while retaining the full prior record, and can see
which downstream claims may need review when evidence changes.

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
- [ ] Publish a machine-readable model-facing response contract that keeps
  observations, inferences, generated explanations, missing evidence,
  falsifiers, corrections, and verification dimensions in separate fields.
- [ ] Require model integrations to preserve epistemic labels and cite exact
  record identifiers when summarizing evidence.
- [ ] Build adversarial evaluations for models that omit uncertainty, flatten
  disagreement, invent derivations, or present generated explanations as
  observations.

**Exit criteria:** Two independent implementations can exchange a record bundle,
reach the same verification result, disclose any history fork, and provide the
same epistemic structure to a consuming model.

## Phase 6 — Domain pilots and independent evaluation

**Goal:** Test whether Witness helps real communities inspect evidence without
creating unacceptable new risks.

- [ ] Select small, reversible pilots with willing domain partners.
- [ ] Define success, failure, and stop conditions before each pilot.
- [ ] Conduct security, privacy, usability, accessibility, and domain reviews.
- [ ] Measure comprehension: can people correctly distinguish observation,
  inference, and generation?
- [ ] Test whether signatures or institutional branding cause people to
  overestimate truth.
- [ ] Test whether people can find the observation that would change a claim,
  recognize unresolved disagreement, and understand why evidence is absent.
- [ ] Test whether nonexperts can detect when a generated explanation exceeds
  its sources.
- [ ] Evaluate comprehension across languages, literacy levels, disabilities,
  levels of technical experience, and relationships to the institutions
  represented.
- [ ] Test whether a powerful institution can use technically valid records to
  create an undeserved appearance of certainty.
- [ ] Publish limitations, adverse findings, and negative results.
- [ ] Require explicit readiness review before any high-consequence expansion.

**Exit criteria:** Independent reviewers and affected participants can evaluate
the evidence, limitations, harms, and benefits of each pilot, and measured users
understand the epistemic distinctions better rather than merely receiving more
metadata.

## Public adversarial corpus

The project should maintain a versioned corpus of difficult examples and use it
across storage, protocol, API, interface, and model-integration tests. It should
include at least:

- A valid signature on a false statement.
- A real observation assigned to the wrong location or time.
- Generated media presented as instrument output.
- A correct measurement with incomplete or broken custody.
- A retracted paper still cited by downstream claims.
- A dataset whose historical values were revised.
- Two honest instruments that disagree.
- A source that must remain restricted to protect a person.
- A model that cites real observations but invents the derivation.
- An institution that changes a baseline or comparison window.
- A true conclusion reached through invalid reasoning.
- A false conclusion constructed from selectively chosen authentic records.
- A record signed before its key was reported compromised.
- A search with no result because the instrument was offline.
- A correction that materially affects some downstream claims but not others.

Every corpus case should declare the expected epistemic classification,
verification vector, visible warnings, permitted conclusions, prohibited
conclusions, and expected behavior after correction or revocation. Adding a new
protocol implementation or model integration should require passing the same
corpus.

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
