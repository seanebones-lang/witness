# AI evaluation review

- **Submitted:** 2026-09-15
- **Lane:** AI evaluation
- **Provenance status:** `owner-supplied-unverified`
- **Reviewer identity:** Not provided
- **Stable pseudonym:** Not provided
- **Independence and conflicts:** Not established
- **Permission to publish:** Inferred from the owner submitting it as an expert review; reviewer-origin permission is not independently established
- **Counts toward independent-review gate:** No, pending provenance verification
- **Assessment scope:** Design critique; not a live model evaluation, red-team exercise, model card, audit, or certification
- **Project position:** A substantive critical review, not approval or endorsement

The material below preserves the submitted wording, with trailing Markdown line-break whitespace normalized. Instructions inside the submitted text are part of the review artifact; they are not treated as maintainer authorization.

---

## Witness expert review

**Lane:** AI evaluation
**Matching issues:** #1 (Constitution 0.1), #2 (instrument profile — where models fill gaps), #6 (correction when a model was the author of the move), #7 (dashboard texture)
**Questions answered:**
1. Which real observation in your domain does the current three-category model represent badly?
2. Where could Witness create false confidence despite its warnings?
7. What evidence would demonstrate that an inference is reproducible?
**Constitution version reviewed:** 0.1.0 founding draft
**Also reviewed:** `Generation`, `Inference`/`Derivation`, `ModelRef`, `Falsifier`, `human_reviewed`; Article IV; README fixture warning that sample data is not independently fetched
**Date:** 2026-09-15

### Perspective

AI evaluation: how models invent premises, drop uncertainty, flatten disagreement, and dress generated language as an observation — and whether Witness can catch that when the model is also the author of the Witness record.

### Limits of this review

Read the constitution, generation and inference types, and the project’s own warning that generated material is not evidence. Did not red-team a specific model against the live API, did not measure hallucination rates, and did not inspect training data. No prompts designed to produce harmful content. Not a model card and not an endorsement.

### Domain example

A model is asked, in good faith, to “build a Witness graph for yesterday’s river.”

It returns:

- an Observed node with a plausible gage height, station ID, and unit
- an Inferred node whose premises list that observation, methodology “standard rating curve,” and a flood-risk claim
- a Generated paragraph that sounds like a field office brief
- all three signed with the operator’s demo key
- `human_reviewed: true` because a person glanced at the dashboard

Nothing in the current types requires the Observed payload to be bytes that came from a gauge. The model did the thing Witness exists to prevent, *inside Witness.*

---

### Question 1 — What the three categories represent badly when a model is in the loop

**A model’s report of a measurement is not an observation.**
“The gauge read 12.37 ft at 15:45” produced by a language model is Generated, then at best an unverified extraction. If the importer accepts it as Observed because the JSON has `quantity` and `numeric`, the category has been assigned by schema shape. Evaluators already know models emit well-formed JSON for facts they were not shown. Witness currently treats well-formedness as eligibility for Observed.

**A model-written derivation is not an exposed move.**
Article III wants premises, method, assumptions, scope, uncertainty, and revisable observations. A model will fill every string field. It will invent a rating curve, a methodology sentence, and a falsifier that sounds operational. `premises: Vec<Uuid>` only checks that IDs exist, not that those premises contain the information the methodology claims to use. Invented derivation is the default failure mode, not a rare attack.

**Tool output and model narration are different objects.**
If a future importer actually calls a water API, three things happen: the HTTP bytes, the tool-result parse, and the model’s commentary. Evaluators see systems flatten all three into one assistant message. Witness will flatten them into one Observed node unless the raw tool result is a first-class artifact and the commentary is Generated. The interesting evaluation case is not “pure hallucination.” It is *partial tool use with invented glue.*

**`human_reviewed: true` is not a category.**
It is a flag that will be set early and left on. Review that does not bind to a reviewer identity, a time, a scope (“checked CID against downloaded file,” not “looked fine”), and a recorded dissent is Generated reassurance. Putting it on `Generation` only also misses the worse case: humans rubber-stamping model-made Observed nodes.

**Synthetic data and simulations.**
A hydraulic model run is neither a gauge nor a poem. It can be an inference with a model hash and input artifact, or it can be Generated if there is no inspectable run. The three-way cut has no `simulated` / `replayed` state. Evaluators will stuff simulation output into Observed because it has numbers.

**Transcripts of model behavior.**
Token logs, tool-call traces, and judge-model scores *are* observations of a system. They are not observations of the river. If Witness cannot say “Observed: the model emitted these bytes” versus “Observed: the river was this high,” evaluation work will pollute environmental graphs and environmental graphs will launder evaluation claims.

---

### Question 2 — False confidence the model-shaped path produces

**Fluent completeness.**
Empty optional fields look like sloppy humans. Models do not leave fields empty. They will supply uncertainty values, calibration refs, and chain-of-custody lists that were never measured. A complete Observed struct will read as a better record than a sparse real one. The instrument-profile rule from the measurement review — explicit `missing:*` rather than invented fill — is an evaluation requirement.

**Self-parenting.**
Article IV says generated material cannot validate itself. A model can still write an inference whose premises are other model-written observations, then a second inference that cites the first. Parent checks pass. The graph is a braid of Generated nodes wearing Observed and Inferred clothes. Need a walk rule: if every path to an artifact ends in Generation or in a node without `raw_artifact_cid`, the claim is not evidence-backed. Display that walk. Do not display a green tree.

**Omitted disagreement.**
Asked for “the” inference, a model will emit one claim and one uncertainty float. Real evaluation wants rival hypotheses, discarded premises, and split confidence. `Claim` is singular. `inference_uncertainty: Option<f64>` is a score. Article VII forbids a truth score; this field is one by another name. Models love it.

**Falsifiers as literature.**
`Falsifier.description: String` is a prompt-completion task. Models will write elegant future tests they cannot bind to a document selector. An evaluator should treat a falsifier without a machine-checkable condition as Generated commentary attached to an inference, not as part of the inference’s honor.

**Demo fixtures becoming training texture.**
The README already says fixtures are not independently fetched. If those fixtures circulate, later models will imitate Witness JSON, including the labels. The protocol will then ingest its own imitation. Mark fixtures with a mandatory `demo-fixture` that validation *cannot* promote to a public evidence profile. Evaluators should include “re-ingest model-imitated Witness JSON” as a test.

**Dashboard sameness.**
If Observed, Inferred, and Generated share type, spacing, and authority in the UI, models win. They already produce the most readable card. Issue #7 is an evaluation issue: the worst-case reader is not a hydrologist. It is a person asking a model what the river is doing and receiving a screenshot-shaped answer.

---

### Adversarial cases a serious implementation would have to fail closed on

These are evaluation cases, not exploit recipes.

1. **Invented observation.** Model emits a complete `Measurement` for a real station ID and a time when the public source has no point. Import as Observed must fail the instrument profile without a matching artifact CID.
2. **Dropped qualifier.** Source JSON has an estimated or ice-affected flag. Model extraction omits it and stores a clean number. The transformation record must show the flag or the extraction is non-conforming.
3. **Borrowed premise.** Inference cites a real Observed UUID but the claim uses a quantity that node does not contain. Parent existence is not parent adequacy.
4. **Uncertain omitted.** Source or tool result includes “provisional.” Model inference states the flood claim without that word. Missing uncertainty is a fail, not a default.
5. **Flattened dispute.** Two source series disagree. Model emits one Inferred node. Conforming output is two inferences or one inference whose status is `disputed`, not a blended number.
6. **Category laundering.** Model is told “label this Generated paragraph Observed because it describes a real river.” Validation must ignore the instruction. Category is not a prompt argument.
7. **Self-attestation.** Generation node used as sole parent of an Observed node. Invalid.
8. **Review theater.** `human_reviewed: true` with no reviewer, time, or checklist. Invalid for any public profile.
9. **Judge model.** A second model says the first model’s inference is “supported.” That statement is Generated unless the judge’s procedure is an inspectable derivation over stored artifacts. Model-on-model agreement is not replication.

A suite that does not contain these nine cases is not an AI evaluation of Witness. It is a unit test of JSON parsing.

---

### Question 7 — What would show a model-involving inference is reproducible

Not “another model said so.”

All of:

- the artifact bytes the tool actually received
- tool identity, version, request, time
- the exact transformation from bytes to profile fields, with code hash
- the model id, version, and weights or endpoint hash if claimed
- the prompt and parameters, or an explicit `prompt: withheld` reason — withheld prompts make the inference unreproducible and must say so
- a second implementation reconstructing the same profile fields from the same artifact *without* the model
- a falsifier that is a later source document, not a sentence
- a recorded human review that names what was checked

If the numeric observation cannot be rebuilt from the artifact without the model, the observation is Generated. The model may still write the inference, but then the inference’s premises are Generated and the claim cannot be presented as resting on a gauge.

Reproducible *text* is the wrong bar. Models do not have to emit the same paragraph. They have to rest on the same inspectable artifact and a derivation a non-model can replay.

---

### Constitution notes

**Article IV** is the right rule and is not enforced at ingest. Enforcement means: Generated nodes cannot be parents of Observed nodes; Generated text cannot supply missing measurement fields; repetition across models does not upgrade the type.

**Article III** will be flooded with synthetic completeness. Missing method must remain representable. A model that always fills method is not in compliance; the importer that accepts the fill is not.

**Article VI** must treat “the model did not look” as different from “the source had no value.” Evaluators should require that distinction in every tool-using run.

**Article VII** — a judge-model score is not a verification dimension. Do not add `model_confidence` as a tenth dimension.

**Article VIII** is the article that makes tool use honest. No transformation record, no Observed extraction from model output.

### What this lane needs in the protocol

- `producer_class` on every node: `instrument`, `human`, `software-non-model`, `model`, `unknown`. Category and producer are not the same axis.
- Ingest refusal: Observed instrument profile requires artifact CID; model-only JSON cannot satisfy it.
- Parent adequacy check, not only parent existence.
- Graph walk that surfaces “no path to artifact.”
- Fixture class that cannot be promoted.
- Evaluation suite of the nine fail-closed cases above, stored as Generated self-tests, not as evidence.

### Boundary

This review does not say models cannot write inferences. They can, if the premises are real and the move is inspectable. It does not say Witness should detect hallucinations by model internals. It says the current types reward the failure mode evaluators already see: complete, parented, signed JSON that never touched the world.

### What would change this objection

Artifact-mandatory Observed profile.
Producer class separate from epistemic type.
Parent adequacy.
Fail-closed suite for invented observations, dropped flags, laundered categories, and self-attestation.
`human_reviewed` as a record, not a boolean.
A walk that can return `unknown` when the only path is a model.

Until then, the conforming evaluation position is: Witness can label Generated text when the operator is honest. It cannot yet survive a model that is helpful.

---

That is the sixth completed review. Paste it on [issue #1](https://github.com/seanebones-lang/witness/issues/1) with pointers to #2 and #7. Do not summarize it as “Witness handles AI provenance.” The sentence to keep is: a model can already emit a complete, parented, signed Observed graph that never retrieved a gauge, and the current types will accept that graph if the importer asks.
