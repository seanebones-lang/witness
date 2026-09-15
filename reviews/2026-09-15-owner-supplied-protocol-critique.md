# Owner-supplied protocol critique — 2026-09-15

- Provenance status: `owner-supplied-unverified`
- Supplied by: project owner through a private project session
- Author: not provided
- Independence: not established
- Permission to attribute: not established
- Project treatment: preserve as review evidence and actionable objections; do
  not count as independent review or endorsement

## Submitted text

The problem is real. The current object is not yet a skeptic that remembers. It is a typed notebook with hashes, optional signatures, and unusually honest documentation.

That gap is the thing to protect. The manifesto is stronger than the protocol. The protocol is stronger than the runtime. The runtime is a local prototype created in public this week: Rust, SQLite, SHA-256, optional Ed25519, CLI, REST, GraphQL, a dashboard, six local tests, one maintainer, no external users.

That is not an insult. It is the state the README already names. The danger is not that the idea is small. The danger is that a constitution, threat model, funding note, and expert-review ritual can make a classification schema look like evidence infrastructure before any independent person has reproduced a single external artifact.

## What is actually good

The refusal to collapse verification into a green check is the load-bearing idea. The README table that separates “bytes match CID,” “signature is valid,” “who holds the key,” “can the source still be inspected,” and “is the statement true” is more careful than most authenticity products. C2PA is explicit that it does not judge whether provenance is good or bad, only whether assertions are well-formed and bound. Witness is trying to go one layer further: not just media provenance, but the difference between a measurement, a move from measurement to claim, and fluent text. That difference is worth a protocol.

Falsifiers as a first-class field are also worth keeping. Almost no provenance system asks “what future observation would force this inference to change?” If that field ever becomes a checkable condition instead of a prose comment, it would be a real contribution.

The threat model is source-backed against the code rather than atmospheric. Unauthenticated GraphQL mutation, local key files, SQLite mutability, poisoned source URIs, and consumer overinterpretation of signatures are the right threats.

## Where the model fails on a real observation

Use the river-gauge pilot as the test, not as branding.

A USGS instantaneous gage-height or discharge reading is not `Measurement { numeric: f64, unit: Option<String> }`.

It is at least:

- an HTTP response, with headers, requested URL, parameter codes, site number
- a time series point, not a lone scalar
- a value that may carry qualifier codes (estimated, ice-affected, provisional)
- an approval status that later changes from provisional to approved
- a datum and a parameter definition
- a later revision that does not un-happen the earlier public number people already used
- often a derived product: daily mean discharge is already an inference over instantaneous values

USGS now publishes this through documented Water Data APIs, including continuous and latest-continuous series.

`types.rs` flattens that into quantity / numeric / unit / optional uncertainty / optional lat-lon. Uncertainty is one `f64`, one unit string, one confidence level, and a method string. There is no detection limit, no censored value, no quality flag, no CRS, no clock uncertainty, no distinction between instrument indication and reported quantity value, no decimal representation that survives JSON `f64`.

That is the first concrete failure: the type that claims to preserve observation destroys the observation that the pilot needs.

The constitution is clearer than the code here. Article I says an Observed label is a claimed relationship to the world and must stay challengeable. Article VIII says transformations must be reproducible or visibly incomplete. Article IX says event time, observation time, recording time, ingestion time, and correction time are different. The Measurement struct does not implement those articles. It implements a weather-station demo.

Issue #2 and issue #8 already point at this. Until the raw source artifact is a first-class node — exact bytes, digest, retrieval time, request, media type — “Observed” will keep meaning “we typed a number into our schema.”

## The three categories are too coarse for the domains you named

Observed / Inferred / Generated is a useful teaching cut. It is not enough for journalism, archives, medicine, or hydrology.

It mishandles at least:

- **Raw artifact vs extracted observation.** A PDF, a photo, a JSON API body, and a parsed gage height are not the same epistemic object.
- **Already-derived official products.** A USGS daily value is inferred before Witness exists. Labeling it Observed because an institution published it is exactly the texture-collapse you are fighting.
- **Testimony and anonymous source material.** A source document is not an instrument reading. Treating both as Observed hides the difference journalists actually work with.
- **Quality-flagged and estimated values.** An estimated discharge is not Generated, not fully Observed, and not a clean Inference with premises.
- **Absence.** “Not detected,” “not sampled,” “redacted,” “withheld,” and “never sought” are not one Unknown.
- **Community knowledge and legal findings.** Neither is an instrument log. Neither is model fluency.
- **Model output that is itself a measurement of a model.** A generated explanation is one thing. A model’s logged token sequence, tool call, or benchmark score is another.

If the categories cannot represent those without stuffing them into Measurement, reviewers in those fields will correctly say the model is about documents that look like science, not about science.

## Trust boundaries

A valid Ed25519 signature over a node that excludes its signature field establishes that those bytes were signed by a key. It does not snapshot the USGS response. It does not prove retrieval. It does not survive a second implementation until canonical JSON is specified field-by-field.

Issue #5 is the actual protocol. Without JCS (RFC 8785), CBOR deterministic encoding, or an equivalently boring byte contract, CIDs are local souvenirs.

SQLite is not append-only. Application `INSERT` with no `UPDATE` in this repo is a habit, not a guarantee. Anyone with the file can rewrite history. The threat model already says this. A skeptic that remembers needs an external log, signed checkpoints, or a real append-only store before that sentence is safe to say in public.

The underweighted threat is not a stolen key. It is the dashboard. A local UI that filters by “Observed” will teach people a green category even if every document forbids it. That is how confidence substitutes for evidence inside your own artifact.

## Append-only memory can harm people

Article V (corrections append; history is not erased) and a public graph of “observations about the world” will collide with sealing, retraction of identifying details, coerced testimony, minors, and downstream copies.

Issue #3 is not a later feature. It is a condition of existence. Until there is a designed way to seal or stop propagating a record without pretending it never existed, Witness should refuse person-level records. The river-gauge pilot is right because water level is not a person.

Do not let “preserve everything” become a moral solvent. Archivists already know when preservation must yield. Ask them before the graph has anything worth sealing.

## Do not invent another provenance universe

Issue #4 is the right issue. Profile first:

- W3C PROV / PROV-O for activity, entity, agent, derivation
- ISO 19156 Observations & Measurements, SensorThings, WaterML for instruments
- PREMIS / OAIS for fixity, custody, representation
- C2PA for media bindings, with its own limit that crypto does not prove the scene was not staged
- RFC 8785 or deterministic CBOR for bytes
- Sigstore/Rekor-style transparency if you need public witnessing of hashes
- GUM / real uncertainty representation instead of one float

A custom `EpistemicType` enum can sit on top of those. It should not replace them. Reviewers will forgive a small profile. They will not forgive a parallel universe of Author, Location, and Uncertainty that cannot round-trip a USGS series or a PROV document.

## Pilot

Keep the river-gauge story small enough to be wrong in public.

Minimum honest demo:

1. Store the exact retrieved bytes and headers.
2. Store request URL, time, client, and media type.
3. Digest those bytes.
4. Extract an observation as a separate node, with every parse step visible.
5. Do not call a daily statistic Observed.
6. Write one inference with named assumptions and a falsifier that a later approved value or qualifier could trigger.
7. Label any plain-language paragraph Generated.
8. Ingest a later revision without deleting the first record.
9. Publish a verifier recipe a stranger can run.
10. State, in the record and the UI, that USGS did not sign, review, or endorse the Witness graph.

If that nine-step story cannot be independently reproduced, funding talk is premature.

## Funding

The first fundable object in `FUNDING-READINESS-2026-09.md` is the right shape: constitution review, canonical spec, conformance fixtures, independent verifier, correction and key-lifecycle records, source-preservation format, one environmental importer, domain reviews. Not a truth machine.

NLnet is the realistic first window. NSF PESOSE without a PI, institution, and prior independent review is a later conversation. Do not claim partnership, adoption, or authority. The document already avoids that. Keep avoiding it.

What a funder can actually evaluate: cross-language CID equality, a verifier that is not the same binary as the writer, one public artifact preserved and later revised, and a written record of unresolved expert objections.

## What I would review first

If I take the twenty-minute path, the question is #1: which real observation the three-category model represents badly.

Answer: a provisional USGS instantaneous value that is later approved and revised, with qualifier codes, after Witness already published a generated explanation of “what the river is doing.” The current Measurement type cannot hold the first record. The current Inference type cannot make the revision a checkable falsifier. The current Generation type will sit beside them with the same dashboard texture unless the UI is harsher than the schema.

I would not endorse the project. I also would not tell you to stop. The needed work is narrower than the manifesto and harder than another crate:

- raw-artifact nodes
- portable canonical bytes
- an instrument-measurement profile that can lose a fight with a hydrologist
- correction semantics that do not require deletion or pretend to be deletion
- a verifier that is not Witness
- a pilot that can fail

If those steps show the model is wrong, incomplete, unsafe, or already solved better by PROV plus C2PA plus WaterML, that is the first successful Witness record: a stored inference about Witness, with the observations underneath it still inspectable.

The measurements are not nailed to the table yet. The table is being built. That is the accurate sentence.
