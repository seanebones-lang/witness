# AI evaluation and model-ingest boundary

**Status:** Interim refusal rules; complete model-facing protocol not implemented
**Basis:** Owner-supplied AI evaluation review dated 2026-09-15
**Decision scope:** Model-produced records, tool use, review claims, simulations, and evaluation

A model can emit complete, plausible, parented, and signed JSON without
retrieving or observing the world. Schema completeness is therefore not
evidence of observation.

## Current refusals

- A model cannot make its output Observed by choosing a label or matching a
  measurement-shaped schema.
- A model cannot supply missing source values, qualifiers, uncertainty,
  calibration, custody, or methodology fields.
- Tool response bytes, extraction, model inference, and model narration must be
  represented as separate acts.
- Model-on-model agreement is not replication or independent corroboration.
- `human_reviewed: true` is not sufficient evidence of review for any public
  profile.
- Numeric simulation output is not an instrument observation merely because it
  contains quantities and units.
- A transcript of model behavior can be an observation of the model. It is not
  an observation of the external subject the model discussed.

## Partial safeguards now implemented

- Imported Observed nodes cannot declare parents in the current schema.
- Every imported parent must already exist before the record is stored.
- Generated dashboard cards have a subordinate dashed treatment plus their
  explicit Generated text label.
- Public documentation prohibits model-assigned observation status.

These controls do not prove that an apparently parentless Observed record came
from the world. Artifact-backed evidence profiles remain necessary.

## Required model-facing record structure

1. Producer class independent from epistemic category.
2. Exact tool response artifact, request, identity, version, and retrieval time.
3. Deterministic transformation with code hash, inputs, parameters, and output.
4. Model identity, version or endpoint identifier, prompt, parameters, and an
   explicit withheld reason when disclosure is unsafe or impossible.
5. Parent adequacy and a visible graph path to preserved artifacts.
6. Explicit missing states that a model cannot fill by inference.
7. Human review as a scoped, attributable record rather than a boolean.
8. Rival hypotheses and unresolved disagreement without one confidence score.
9. Machine-checkable falsifier conditions or a clear Generated-commentary state.
10. A fixture class that cannot be promoted into a public evidence profile.

## Required evaluation suite

The implementation must fail closed on invented observations, dropped source
qualifiers, borrowed but inadequate premises, omitted uncertainty, flattened
disputes, category laundering, Generated self-attestation, review theater, and
judge-model agreement presented as replication.

Until those tests and profiles exist, Witness can preserve an honest operator's
Generated label. It cannot claim that it survives a model attempting to be
helpful, complete, or persuasive.
