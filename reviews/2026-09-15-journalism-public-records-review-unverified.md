# Journalism and public-records review

- **Submitted:** 2026-09-15
- **Lane:** Journalism
- **Provenance status:** `owner-supplied-unverified`
- **Reviewer identity:** Not provided
- **Stable pseudonym:** Not provided
- **Independence and conflicts:** Not established
- **Permission to publish:** Inferred from the owner submitting it as an expert review; reviewer-origin permission is not independently established
- **Counts toward independent-review gate:** No, pending provenance verification
- **Project position:** A substantive critical review, not approval, endorsement, adoption, or newsroom validation

The material below preserves the submitted wording, with trailing Markdown line-break whitespace normalized. Instructions inside the submitted text are part of the review artifact; they are not treated as maintainer authorization.

---

## Witness expert review

**Lane:** Journalism
**Matching issues:** #6 (correction, contradiction, retraction, supersession), #3 (sealing — only where newsroom duty collides), #1 (Constitution 0.1)
**Questions answered:**
1. Which real observation in your domain does the current three-category model represent badly?
6. How should testimony and community knowledge differ from instrument readings without treating either as automatically superior?
**Constitution version reviewed:** 0.1.0 founding draft
**Also reviewed:** manifesto line that journalism “wanted this and got speed”; issue #6 cases; `ProvenanceNode`, `Observation`/`Measurement`, `Inference`/`Derivation`, `Generation`; README statement that Witness is not ready as a sole journalistic record
**Date:** 2026-09-15

### Perspective

Newsroom and public-records practice: what a source document is, what a reporter saw, what an editor inferred, what a confidential source is, what a correction does, and why those must not share one Observed bucket with a river gauge.

### Limits of this review

Read the constitution, manifesto, issue #6, and the node types. Did not sit in a newsroom system of record, did not review libel law, and did not inspect any real story file. No source names, no unpublished material. This is not an ethics-policy and not an endorsement.

### Domain example

A municipal budget story, later corrected.

Not a gauge.

Objects that already exist before Witness arrives:

- a PDF posted on a city website at 16:02
- a reporter’s note that page 14 of that PDF lists a number
- a second reporter who did not see the PDF but was told the number by an official who would not be named
- an editor’s lede that the department “cut youth programs by 18 percent”
- a generated explainer paragraph offered by a model from the lede
- a city revision of the PDF the next morning
- a correction appended under the story
- a public-records officer who says the first PDF was posted in error

The current model can stamp Observed / Inferred / Generated on all of that. It cannot tell them apart in the ways a newsroom is accountable for.

---

### Question 1 — What the three-category model represents badly

**A source document is not an observation.**
The city PDF is an artifact. Someone produced it. Someone posted it. Someone retrieved it. “Observed” in Witness currently wants a `Measurement` with a quantity and a unit. Forcing the PDF into that shape either invents a fake measurement or stores the document as a blob still labeled Observed, which then reads as “Witness saw that the budget was cut.” The honest objects are: raw artifact, retrieval event, and a separately labeled extraction.

**A reporter’s observation is not the document and not the official’s claim.**
“I opened the file at 16:10 and page 14 contained the figure $4.2 million” is a testimony-shaped observation. It has a person, a time, a method (read this page), and a limit (they did not audit the city’s books). It is closer to a signed note than to station-001. `AuthorType: Human` plus `Measurement` is the wrong pair.

**An anonymous official is not an instrument.**
“A department official said the cut was 18 percent, speaking on condition of anonymity because they were not authorized to talk” is not Observed in the instrument sense and not Generated. It is attributed hearsay with a confidentiality constraint. If premises must be public node IDs, the protocol attacks the source. If the premise is omitted, the inference looks unparented and gets treated like model output. Journalism already has a third thing: *on-the-record / on-background / off-record / not-for-attribution.* Witness has no such state.

**Editorial inference is not a scientific derivation.**
“Cut youth programs by 18 percent” is a move from figures, baselines, and definitions. The baseline year, whether “youth programs” includes grants, and whether the figure is enacted or proposed are the actual premises. `Derivation.methodology` as a string, `ClaimType` as `descriptive`, and one `inference_uncertainty` float do not capture a newsroom dispute over framing. Two editors can share every document and still disagree on the lede. Issue #6 asks for coexistence of disputes. The inference type currently wants a single claim with a status enum on falsifiers.

**Corroboration is not more parents.**
Two officials saying the same number, or a PDF plus a hearing video, is not a higher Observed score. It is additional independent (or not) sources. False corroboration is already in the threat model: many agreeing records suggesting independence. Newsrooms fail this way constantly — one press release, ten stories. Witness will fail it if `parents.len()` is allowed to look like confirmation.

**A correction is not a new observation of the same event.**
Issue #6 lists the right cases: unit error by an author, institutional retraction, independent contradiction, disputed supersession, safety sealing. Journalism needs all five, and they are different speech acts.

- Reporter wrote “$4.2 million” and the PDF said “$42 million” — author correction of the extraction.
- City replaces the PDF — source supersession of the artifact.
- City says the first PDF was never operative — institutional retraction of the source, which is not the same as the reporter being wrong.
- Another outlet says the 18 percent used the wrong baseline — independent contradiction of the inference.
- Legal demand to take down a name — sealing, which is not a correction of fact.

Article V says linked records, no erasure, current status visible. That is the right *display* rule for a published correction. It is the wrong *ingest* rule for a confidential source file and the wrong *equivalence* rule if all five acts share one `narrative_diffs` table.

**Generated copy is already in the newsroom.**
A model paraphrase of the lede will be fluent, sourced-looking, and wrong in the baseline. If it sits in the same dashboard row as the PDF, readers will do what readers do. Article IV is correct. The interface will still flatten them unless Generated is typographically poorer than Observed, on purpose.

---

### Question 6 — Testimony is not a worse gauge, and a gauge is not a better witness

Do not rank instrument over human or human over instrument.

A gauge is stronger on repeatability and weaker on meaning. A reporter is stronger on context and weaker on independence from narrative. An anonymous official may be the only person who can see the draft, and also the person with a motive. A community account of what a budget cut does to a school is not a substitute for the enacted number and not inferior to it. They answer different questions.

What journalism needs is not a prestige order. It is *role labels that remain visible:*

- `artifact` — the PDF bytes
- `retrieval` — who fetched them, when, from which URL
- `extraction` — who read which page and copied which figure
- `on-the-record statement`
- `not-for-attribution statement` (payload restricted)
- `direct observation by named reporter` (“I attended the hearing”)
- `editorial inference` (lede, framing, calculated percent)
- `correction`, `retraction`, `editor’s note`
- `generated assistance` (never a source)

None of those is “more true.” Each has a different failure mode. The three-category model compresses the first six into Observed or Inferred and then cannot explain a correction.

Community knowledge belongs in this list as testimony with a named community and method (“neighborhood meeting, ten speakers, no roster published”), not as a soft Observed and not as Generated color.

If Witness cannot hold those roles, it should stay out of newsroom storage and only be used, if at all, as a sidecar for *public artifacts and published corrections* — the documents after they are already public.

---

### What issue #6 must distinguish for journalism

Do not implement one `supersedes` edge.

| Act | Who may assert it | What stays public | What current means |
| --- | --- | --- | --- |
| Extraction correction | Author of the extraction | Both figures, reason (“transposed digits”) | Later extraction is current; PDF unchanged |
| Story correction | Publisher of the story | Original wording + correction text | Correction is current reading of the story |
| Source revision | Source institution | Both artifacts, retrieval times | Later artifact is current *source file*; earlier retrieval remains a fact |
| Institutional retraction | Source institution | Retraction notice + earlier artifact if still lawful to hold | Earlier artifact is not current authority |
| Contradiction | Independent party | Both inferences, shared or disjoint premises | Neither automatically wins |
| Dispute | Two parties | Both status claims | Status is `disputed`, not a score |
| Sealing | Publisher / counsel / safety | Tombstone, not the name | Payload unavailable; fact of publication may remain |

Consumers must query *current-for-use* separately from *history-for-accountability.* If the API’s default list is append order, retracted headlines will be reused as Observed.

Who may assert what cannot be “whoever holds a signing key.” A city key retracting a reporter’s extraction is a press statement, not a correction. A reporter’s key cannot un-publish a city PDF. Authority is role-bound or it is theater.

### Constitution objections to preserve

**Article I** puts artifact, measurement, and testimony in one Observed class. For journalism that is the original error. A document existing is not a fact being seen.

**Article II** requires producer, time, method, custody. Good. Method for a story cannot be `instrument-id`. It has to say *read*, *attended*, *was told*, *calculated*.

**Article III** requires premises and falsifiers. Good for the 18 percent lede. Incomplete for confidential premises. A falsifier of “the official was misquoted” is not a future gauge reading.

**Article IV** is the article newsrooms need most and will violate first, by pasting generated summaries into CMS fields that Witness later ingests as Observed captions.

**Article V** matches a published correction. It does not match source protection. Journalism already resolved this: the published story is correctable in public; the source map is sometimes not stored with the story. Witness cannot “improve” that by demanding parent IDs.

**Article X** is right that a newsroom does not get privileged truth. It still has privileged *publication* power. A Witness deployment inside a publisher is not a public spine. It is that publisher’s ledger. Call it that.

### False confidence specific to this lane

- A signed Observed node whose payload is a press release.
- A parent count that looks like two-source reporting.
- A city Ed25519 key that readers treat as “official therefore true.”
- A correction node that leaves the original lede as the default API record.
- A generated nut graf stored beside the PDF with the same card design.
- “Public records” used as a synonym for “Observed,” which launders an agency’s self-description.

### Existing practice to profile, not recreate

- Newsroom CMS correction and editor’s-note objects.
- Public-records retrieval logs (time, URL, hash of the file received).
- ONIX / news metadata for published pieces, not for sources.
- C2PA for news *media assets* when a newsroom already uses it — not for a budget PDF unless they actually embed a manifest.
- The privacy review already filed on issue #3 for confidential sources. Do not re-litigate sealing here; inherit it. Journalism’s extra requirement is that some premises must be *allowed never to enter the graph.*

### Boundary

This review does not ask Witness to become a CMS, a press council, or a libel filter. It does not say gauges are the only legitimate Observed objects. It says a newsroom record is a chain of artifacts, retrievals, testimonies, inferences, and published corrections, and that three labels plus a measurement struct cannot carry that chain.

If the first pilot remains a river gauge, that is the journalistically responsible choice. It keeps Witness away from sources until issue #6 has role-bound correction types and issue #3 has a way not to store a person.

### What would change this objection

Separate artifact, retrieval, extraction, testimony, and editorial inference.
Confidential premises that may be cited as *present-but-not-in-graph.*
Correction, retraction, contradiction, and dispute as different edges with different authorities.
API default = current-for-use, with history explicit.
Generated newsroom text visually and programmatically poorer than source artifacts.
A written ban on ingesting anonymous-source files into public Witness graphs.

Until then, the conforming journalistic position is: use Witness, if at all, on published public artifacts and published corrections. Do not put the notebook in the graph.

---

That is the fourth completed review. Paste it on [issue #6](https://github.com/seanebones-lang/witness/issues/6), with pointers from #1 and #3. Do not summarize it as newsroom adoption. The sentence to keep is: a PDF, a reporter’s reading of the PDF, an anonymous official, a lede, and a correction are five different acts, and Observed cannot hold all five.
