# Project response — journalism and public-records review

**Date:** 2026-09-15
**Review:** [Journalism and public records](2026-09-15-journalism-public-records-review-unverified.md)
**Decision status:** Interim boundary accepted; journalism profile remains open
**Review provenance:** Owner-supplied, reviewer identity and independence not established

## Decision

The project accepts the review's central objection:

> A PDF, a reporter's reading of the PDF, an anonymous official, a lede, and a
> correction are different acts, and Observed cannot carry all five without
> hiding the distinctions on which newsroom accountability depends.

Witness will remain outside confidential newsroom storage. During the prototype
stage, any journalism-related experiment must be limited to already-public
artifacts and published corrections. Reporter notebooks, anonymous-source
files, confidential premise maps, not-for-attribution material, and unpublished
person-concerning records are prohibited.

## Accepted requirements

1. Separate raw artifact, retrieval, extraction, direct observation, testimony,
   attributed statement, editorial inference, and generated assistance.
2. Do not rank human testimony and instrument measurements on one prestige or
   truth scale; expose their different roles and failure modes.
3. Allow a confidential premise to be recorded as intentionally
   present-but-not-in-graph without publishing a source identity or hidden edge.
4. Distinguish extraction correction, story correction, source revision,
   institutional retraction, independent contradiction, dispute, and sealing.
5. Bind authority to roles and targets; possession of a signing key alone does
   not authorize every corrective act.
6. Separate `current-for-use` from `history-for-accountability` and make API and
   interface defaults safe for downstream reuse.
7. Represent common origin and claimed source independence so parent count does
   not masquerade as corroboration.
8. Make generated newsroom text visibly and programmatically distinct from
   source artifacts, with accessible non-color labels.
9. Inherit the privacy review's restriction, sealing, and deletion requirements
   rather than create a weaker journalism exception.

## Immediate changes

- Publish the interim journalism boundary.
- Add the required roles, authority model, source-independence model, current
  state query, and interface behavior to the roadmap.
- Preserve the complete objection and its unverified provenance status.

## Unresolved questions

- Which existing newsroom metadata and correction vocabularies should be
  profiled rather than recreated?
- How can a present-but-not-in-graph premise be useful without becoming an
  unverifiable authority signal or a source-identification side channel?
- Which roles can contest or override `current-for-use`, and how are disputes
  presented without a truth score?
- What should remain visible after a published artifact becomes unsafe or
  unlawful to retain?
- How should public corrections propagate to generated and inferred downstream
  records without rewriting history?

These questions require newsroom, public-records, privacy, legal, accessibility,
and affected-community review before the boundary can change.
