# Verification Model

Witness reports evidence about records. It does not produce a verdict of truth.

## Verification dimensions

| Dimension | Question | Example states |
| --- | --- | --- |
| Content integrity | Do the stored bytes match their identifier? | verified, invalid, unavailable |
| Signature | Does the signature validate for the stated key? | valid, invalid, unsigned, revoked, unknown |
| Identity | What connects the key or author ID to an actor? | self-asserted, attested, independently verified, disputed |
| Source availability | Can an evaluator inspect the referenced source? | available, restricted, missing, destroyed |
| Custody | Is handling from collection to record documented? | complete, partial, absent, disputed |
| Calibration | Was the instrument or method fit for its claimed use? | current, expired, unavailable, inapplicable |
| Replication | Has an independent process reproduced the observation or inference? | untested, replicated, failed, contested |
| Privacy | What access and retention limits apply? | public, restricted, redacted, sealed, consent withdrawn |
| Domain review | Has an appropriate reviewer assessed domain-specific meaning? | unreviewed, reviewed, disputed, inapplicable |

Interfaces may summarize these states for comprehension, but must preserve their
individual values and supporting records. They must not combine them into a
single truth score.

## Current prototype guarantees

The current implementation can recompute a SHA-256 identifier over the JSON value
serialized by Rust `serde_json`, validate Ed25519 signatures created by this
implementation, preserve declared parent relationships, and reject a duplicate
node identifier. These are local implementation properties.

The current format is not yet a cross-language canonicalization standard.
Object ordering, numeric representation, Unicode treatment, schema versions, and
normalization rules require a written interoperable specification and independent
conformance fixtures before portable verification can be claimed.

## Interpretation rules

- A valid content identifier establishes byte relationship, not factual accuracy.
- A valid signature establishes signing-key control, not civil identity or truth.
- A source URI is a reference, not proof that the source supplied the record.
- An epistemic label is a producer claim that remains open to challenge.
- A provenance edge records a declared relationship, not valid reasoning.
- A timestamp inside a record is not independent proof of event time.
- Multiple agreeing records may share one source and are not necessarily
  independent corroboration.
- Verification failure remains visible. Implementations must not silently drop,
  repair, or reclassify the record.

## Required future behavior

The protocol must eventually return a structured verification result containing
each applicable dimension, the verifier and software version, evaluation time,
supporting record IDs, explicit unknowns, and errors. That result should itself
be recordable and challengeable. See [ROADMAP.md](../ROADMAP.md) for the work
required before a stable verification contract exists.
