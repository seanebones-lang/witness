# Project response — privacy and human-rights review

**Date:** 2026-09-15
**Review:** [Person-concerning records](2026-09-15-privacy-human-rights-review-unverified.md)
**Decision status:** Interim protective response; constitutional amendment remains open
**Review provenance:** Owner-supplied, reviewer identity and independence not established

## Decision

The project accepts the review's central safety objection:

> Article V is fit for gauges and unfit for named people, and person-concerning
> ingest should stay closed until a sealing object exists.

Witness will keep the river-gauge pilot limited to public, non-personal
instrument data. The prototype must not be used to ingest person-concerning
payloads. This includes identifiable images, audio, testimony, medical or school
records, confidential-source material, identity or location claims, and
generated restatements of those records.

This is an interim operational restriction. It does not silently rewrite the
0.1.0 founding draft. Article V needs a public amendment that reconciles durable
history with payload unavailability, restriction, redaction, withdrawal, and
minimized tombstones.

## Accepted requirements

1. Add first-class `Restriction`, `Redaction`, `Withdrawal`, and `Tombstone`
   protocol objects before person-concerning ingest can open.
2. Define restriction scope independently for payload, edges, author mapping,
   labels, source URI, and public CID.
3. Treat a digest of unique personal material as potentially identifying.
4. Carry restriction objects with exports; dropping them is non-conforming and
   ignoring them is hostile behavior that the protocol cannot technically erase.
5. Propagate an undermined or restricted state to identifying inferences and
   generated children when a premise is sealed.
6. Define privacy states without folding them into an integrity or truth score.
7. Disclose operator access and retention power as a deployment property.
8. Separate testimony and confidential custody profiles from public instrument
   measurement profiles.

## Immediate changes

- Publish an explicit person-concerning ingest prohibition for the prototype.
- Keep the first source-preserving reference story limited to a public USGS
  river-gauge response.
- Add the restriction-object and Article V amendment work to the trust-ordered
  roadmap.
- Preserve this objection even if reviewer provenance cannot be verified.

## Unresolved questions

- Who may assert or contest a restriction, and through what appeal process?
- What minimal tombstone is safe across different threat models?
- When may a CID remain public, be access-controlled, or be destroyed?
- Which legal, archival, humanitarian, journalistic, and community vocabularies
  should be adopted rather than recreated?
- How can conforming cooperative replicas demonstrate restriction propagation
  while honestly acknowledging that hostile copies cannot be recalled?

## Amendment path

A proposed Article V amendment must include concrete old and new language,
compatibility and migration effects, authority over sealing decisions, affected
community consultation, replica behavior, tests, and any minority report. It
must follow the constitution's public review period before ratification unless a
narrow emergency measure is required to protect people.
