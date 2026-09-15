# Standards boundary and design direction

**Status:** Accepted direction; mappings and conformance are not implemented
**Basis:** Owner-supplied standards review dated 2026-09-15
**Decision scope:** Architecture and interoperability planning

Witness should add the smallest vocabulary needed to preserve an epistemic
distinction while adopting existing standards for the structures they already
define. This direction does not establish conformance to any named standard.

## Proposed division of responsibility

| Concern | Candidate established vocabulary or mechanism | Witness-specific need |
| --- | --- | --- |
| Entities, activities, agents, use, derivation, attribution, revision, invalidation | W3C PROV / PROV-O | Epistemic annotation and prospective falsifier |
| Linked term identifiers | JSON-LD | A stable Witness context and signed-envelope rules |
| Portable hash and signature preimage | RFC 8785 JCS or deterministic CBOR | A single selected profile and cross-language vectors |
| Instrument observations and time semantics | ISO 19156 O&M, SensorThings, WaterML | A bounded evidence profile and explicit gaps |
| Units and quantities | UCUM or QUDT | No competing unit vocabulary |
| Preserved-object fixity, events, agents, and rights | PREMIS | Epistemic classification of the resulting records |
| Signed media provenance | C2PA, when applicable to the asset | Witness Generated remains an evidentiary category, not a C2PA origin synonym |
| Attested identity claims | Verifiable Credentials, when a real issuer exists | Separation of key control, claimed author, and identity assurance |

## Proposed Witness extensions

- `witness:epistemicType` with Observed, Inferred, and Generated values.
- `witness:falsifier` for a stated future condition or entity that would force
  an inference to be revised or rejected.
- Separate verification dimensions that cannot be collapsed into a truth score.

The exact IRIs, shapes, allowed values, and processing rules remain undecided.
They must be specified through testable profiles rather than inferred from this
planning document.

## Semantic boundaries that must survive mapping

- A PROV derivation records a relationship; it does not establish scientific
  validity or truth.
- A C2PA assertion and valid binding do not establish that a depicted event was
  honest or unstaged.
- C2PA use of “generated” describes media origin. Witness Generated describes
  evidentiary status. Implementations must not equate them.
- A Verifiable Credential can carry an issuer's attestation. It does not turn
  an observation into truth or make a key holder the observed event's author.
- A source URI identifies a location. It is not a retrieved artifact, custody
  history, or fixity event.
- O&M observation times, result times, retrieval events, ingestion events,
  signatures, and publication events must remain distinct.
- A JSON-LD identifier, graph node UUID, and content digest serve different
  purposes and must not silently substitute for one another.
- A content digest over unspecified serialization is not a portable binding.

## Evidence required before compatibility claims

1. A source-cited and independently reviewed crosswalk.
2. A selected canonicalization standard with normative test vectors.
3. A published JSON-LD context and processing rules.
4. PROV-aligned edge semantics with validation fixtures.
5. One standards-readable instrument example derived from the USGS bundle.
6. Negative fixtures demonstrating the semantic mismatches above.
7. Verification by an implementation that does not share Witness's code.

Until that evidence exists, public material should say only that Witness is
**evaluating and designing toward** these standards.
