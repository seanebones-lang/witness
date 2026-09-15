# Witness Epistemic Constitution

**Version 0.1.0 — founding draft**

This constitution defines the invariants that a conforming Witness implementation
must preserve. It governs the protocol and its representations; it does not grant
Witness, its maintainers, or any participating institution authority to declare
what is true.

Version 0.1.0 is published for implementation and public criticism. It becomes a
ratified 1.0 contract only after independent technical, domain, privacy, and
community review.

## Article I — Categories retain their meaning

Every record declares one epistemic category:

- **Observed:** a record of an event, measurement, artifact, or testimony, with
  enough provenance to identify what was recorded and how.
- **Inferred:** a claim derived from identified premises through an exposed
  method.
- **Generated:** material synthesized by a model, algorithm, or creative process.

A generated or inferred record cannot become observed through relabeling,
repetition, popularity, or institutional endorsement. A new observation process
must produce a new record.

## Article II — Observations identify their origin

An observation must identify its producer, collection or recording time when
known, method or instrument when applicable, and source location or custody
information when available. Missing provenance remains explicit. Classification
as observed records a claimed relationship to the world; it does not establish
that the claim is accurate or honest.

## Article III — Inferences expose the move

An inference identifies its premises, method, material assumptions, scope,
uncertainty, and the observations that could force revision when those can be
stated. Missing premises and unavailable methods remain visible. Confidence does
not replace this information.

## Article IV — Generated material never serves as its own evidence

Generated material remains labeled generated wherever it is copied, summarized,
or transmitted. It may explain or propose; it cannot validate itself or acquire
evidentiary status through volume. A later observation may examine an event
caused by generated material, but that observation is a separate record with its
own provenance.

## Article V — Corrections append to history

Correction, contradiction, retraction, and supersession create linked records.
They do not erase the records they address. Interfaces must make the current
status understandable while retaining access to the earlier state and the reason
for change.

## Article VI — Unknown remains unknown

Witness distinguishes missing evidence from evidence of absence. It preserves
whether evidence was not sought, not collected, unavailable, destroyed,
restricted, withheld for safety or privacy, or insufficient. Silence is a valid
output when the evidence cannot support a conclusion.

## Article VII — Verification is multidimensional

Integrity, signature validity, identity assurance, source availability, custody,
calibration, replication, privacy, and domain review remain separate states.
Witness must not compress them into a universal truth score. A content identifier
relates a digest to bytes. A signature demonstrates key control over signed bytes.
Neither proves identity, expertise, honesty, or truth by itself.

## Article VIII — Transformations are reproducible or visibly incomplete

Every automated transformation identifies its inputs, software and version,
parameters, execution time, and output when available. If reproducibility is
limited by missing code, data, access, randomness, or environment, that limit is
part of the record.

## Article IX — Time is named

Event, observation, recording, ingestion, signing, publication, correction, and
knowledge times are distinct. Implementations name the time they display and do
not silently collapse these meanings into one timestamp.

## Article X — Power and departures remain visible

No lab, government, company, model, maintainer, funder, or majority receives
privileged truth status. Implementations disclose material departures from this
constitution, including moderation, access, retention, ranking, and governance
choices. Dissenting technical judgments and compatible forks remain linkable.

## Amendments

A constitutional amendment requires a public proposal that includes:

1. the concrete failure or harm being addressed;
2. affected people and consultation performed;
3. the old and proposed text;
4. compatibility, migration, and privacy consequences;
5. authority gained or concentrated by the change;
6. objections and minority reports; and
7. tests or review evidence supporting the change.

The proposal must remain open for at least 30 days. Until Witness has an
independent governing body, the maintainer records the decision and rationale in
the repository. Amendments increment the constitution version. Implementations
and exported bundles identify the version they follow; history is never rewritten
to imply that a later rule governed an earlier record.

Emergency changes may protect people or infrastructure before the review period
ends. They must be narrowly scoped, dated, publicly recorded without disclosing
sensitive exploit details, and expire within 30 days unless adopted through the
ordinary amendment process.
