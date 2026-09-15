# Project response — AI evaluation review

**Date:** 2026-09-15
**Review:** [AI evaluation](2026-09-15-ai-evaluation-review-unverified.md)
**Decision status:** Central objection accepted; two import and interface safeguards implemented
**Review provenance:** Owner-supplied, reviewer identity and independence not established

## Decision

The project accepts the review's central objection:

> A model can emit a complete, parented, signed Observed graph that never
> retrieved an artifact, and schema completeness cannot establish contact with
> the world.

Model-produced content remains Generated unless an independent,
profile-conforming transformation reconstructs the claimed observation fields
from preserved source artifacts. A model may write an inference only when its
premises and method remain inspectable and the result does not hide the
epistemic status of model involvement.

## Immediate implementation

1. Validate imported relationships before JSONL storage.
2. Reject imported Observed records that declare parents in the current schema.
3. Reject any imported relationship whose parent is absent.
4. Render Generated cards with a subordinate dashed treatment while retaining
   their explicit text label.
5. Publish the model-ingest refusal and required evaluation suite.

## Accepted requirements

- Add producer class independently from epistemic category.
- Require artifact-backed Observed profiles.
- Separate raw tool output, deterministic extraction, model inference, and narration.
- Validate parent adequacy and show whether a claim has a path to an artifact.
- Replace boolean human review with an attributable, scoped review record.
- Preserve explicit missing states and prohibit model completion of absent evidence.
- Prevent fixtures and model-imitated fixtures from satisfying public profiles.
- Treat prose-only falsifiers as commentary until they become machine-checkable.
- Represent simulations, replays, and model-behavior observations without
  laundering them as observations of the external world.
- Run the nine-case fail-closed evaluation suite described in the review.

## Limits

The new import checks do not prove that a parentless Observed record came from
an instrument or artifact. The current system still lacks artifact-mandatory
profiles, producer class, parent adequacy, model execution records, graph-path
evaluation, and a complete model-facing response contract. This response is
not a model evaluation, red-team result, model card, certification, or claim
that Witness handles AI provenance.
