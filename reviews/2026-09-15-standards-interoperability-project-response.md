# Project response — standards and interoperability review

**Date:** 2026-09-15
**Review:** [Standards and interoperability](2026-09-15-standards-interoperability-review-unverified.md)
**Decision status:** Design direction accepted; conformance not established
**Review provenance:** Owner-supplied, reviewer identity and independence not established

## Decision

The project accepts the review's central architectural recommendation:

> The epistemic type is the narrow Witness extension; hashes, units,
> derivations, fixity, identity attestations, and applicable media provenance
> should use established standards instead of a parallel Witness vocabulary.

This decision does not assert that the current Rust structs, JSON, database,
dashboard, signatures, or USGS reference conform to any named standard. A
source-cited crosswalk, normative mapping decisions, test vectors, external
processors, and independent review are still required.

## Accepted requirements

1. Select RFC 8785 JCS or deterministic CBOR for the hash-and-sign preimage;
   do not publish a boutique canonical JSON algorithm.
2. Map interchange provenance to PROV entities, activities, agents, use,
   derivation, attribution, revision, and invalidation.
3. Treat instrument observations through an O&M/WaterML-informed profile and
   use UCUM or QUDT identifiers for units.
4. Use PREMIS concepts for fixity and preservation events.
5. Publish a JSON-LD context for narrow Witness extensions.
6. Preserve `witness:falsifier` as a genuine prospective-condition gap rather
   than mislabel it as an invalidation that has already occurred.
7. Limit C2PA to applicable media assets and keep its “generated” semantics
   distinct from Witness Generated.
8. Use Verifiable Credentials only for real issuer attestations, not as a
   default envelope for every observation.
9. Publish conformance and negative fixtures before making compatibility claims.
10. State publicly that standards compatibility does not establish truth.

## Immediate changes

- Publish the accepted standards boundary and the evidence required for claims.
- Add concrete crosswalk, canonicalization, edge, context, and fixture work to
  the trust-ordered roadmap.
- Preserve the complete objection and its provenance status in the review
  register.

## Unresolved questions

- Which canonical preimage standard best fits the protocol's exact requirements?
- Which PROV relations belong in the closed core, and which remain profile-level?
- What JSON-LD form, if any, is inside the signed envelope?
- Which exact O&M, WaterML, SensorThings, UCUM, QUDT, and PREMIS versions and
  identifiers apply to the USGS reference?
- What validation technology and independent consumer will demonstrate the
  first real interoperability result?

These questions require specification work and domain review. The submitted
review supplies a direction and a test for false confidence, not completed
conformance evidence.
