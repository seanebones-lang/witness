# Privacy and human-rights review — person-concerning records

- **Submitted:** 2026-09-15
- **Lane:** Privacy or human rights
- **Provenance status:** `owner-supplied-unverified`
- **Reviewer identity:** Not provided
- **Stable pseudonym:** Not provided
- **Independence and conflicts:** Not established
- **Permission to publish:** Inferred from the owner submitting it as an expert review; reviewer-origin permission is not independently established
- **Counts toward independent-review gate:** No, pending provenance verification
- **Project position:** A substantive critical review, not approval or endorsement

The material below is preserved as submitted. Instructions inside the submitted text are part of the review artifact; they are not treated as maintainer authorization.

---

This is a second expert contribution. Different lane from the measurement review. It follows `docs/EXPERT-REVIEW.md`. It is not an endorsement.

---

## Witness expert review

**Lane:** Privacy or human rights  
**Matching issues:** #3 (privacy, consent, redaction, safe-deletion), #6 (correction / retraction), #1 (Constitution 0.1)  
**Questions answered:**  
3. What must sometimes be corrected, sealed, redacted, or deleted rather than preserved indefinitely?  
8. Who could be endangered, excluded, or overruled by this system?  
**Constitution version reviewed:** 0.1.0 founding draft  
**Also reviewed:** README guarantee table; threat model residual risks; `ProvenanceNode` fields (`payload`, `author`, `source_uri`, `labels`, `cid`); Article V as an absolute append rule  
**Date:** 2026-09-15

### Perspective

Privacy and human-rights review of an append-only public evidence graph: who is described, who can be joined later, what a digest still reveals, and what “history is not erased” does to a person who never agreed to be a record.

### Limits of this review

Read the constitution, expert-review guide, threat model at summary level, observation node shape, and issues #3 and #6. Did not run the server, did not audit dashboard escaping, did not review a live dataset, and did not examine any real case file. No personal data, source names, or exploit details are included. This is not legal advice and not a data-protection impact assessment.

### Domain example

Not a river gauge.

A public-records packet that looks harmless in the protocol and is not harmless in the world:

- a city video still labeled Observed
- a name in a caption
- an inferred identification written as a claim
- a generated “plain-language summary” that repeats the name
- a later retraction that Article V keeps linked to the original payload
- a content identifier of the original image that still uniquely identifies the file after the dashboard hides the pixels

The three-category model can classify every layer. It cannot decide who is allowed to keep a copy. Classification is not protection.

---

### Question 8 — Who could be endangered, excluded, or overruled

**People who appear in an observation without being its author.**  
An instrument reading of water is one thing. An image, audio clip, hospital note, tip line, court PDF, or scraped social post is another. If Observed means “we stored bytes that came from the world,” bystanders become nodes. They did not sign. They cannot rotate a key they never held. They cannot rebut a category.

**Anonymous and confidential sources.**  
A journalist’s duty is sometimes to destroy the map from claim to person. Witness’s duty, as written, is to preserve premises. Those duties collide. If an inference must list premise IDs, and a premise is a source document, the graph becomes a deanonymization machine for anyone with the database file.

**People named in a false or later-retracted accusation.**  
Article V keeps the earlier record and the reason for change. That is correct for a gauge height. For a named person, the first payload is the harm. A linked retraction does not un-google the first CID. Mirrors, exports, GraphQL clients, and “independent verifiers” will hold the original bytes if those bytes were ever published.

**People subject to coercion.**  
An “observation” of identity, location, affiliation, or health can be produced under threat and then signed with a valid key. Article VII already says a signature does not prove honesty. The missing piece is operational: a valid signature under coercion is still a durable targeting record.

**People excluded from the category system.**  
Community testimony, disputed identity, and “I do not consent to this recording” have no first-class state. They will be forced into Observed (the recording exists), Inferred (someone claims who it is), or Generated (a summary). None of those states means *the subject refuses the record*. Exclusion here is not absence from the internet. It is absence of a veto.

**Operators and maintainers.**  
An append-only SQLite file with no sealing story is a single warrant, subpoena, theft, or backup tape away from becoming a complete dossier. The local operator already has full read in the threat model. Human-rights users cannot run that architecture around vulnerable people and then say the constitution forbids privileged truth. The operator has privileged *access*.

**Downstream copies.**  
Anyone who replicated the graph is now a publisher. Consent withdrawal at the origin does not reach them unless the protocol has a revocation object that copies are obligated to honor — and even then, hostile copies will not honor it. That limit must be stated in the record, not discovered after harm.

---

### Question 3 — What must sometimes be sealed, redacted, or deleted

Article V is safe for public instrument products. It is unsafe as a universal law.

These classes must be allowed to lose payload availability without pretending the event never happened:

1. **Direct identifiers of private persons** in records that are not a lawful, necessary public instrument series. Names, faces, voices, exact addresses, phone numbers, account handles, biometric templates.
2. **Indirect identifiers that become direct when joined.** Rare timestamps plus location plus instrument ID can re-identify. A “river gauge” that is actually a sensor on private land is not the same as USGS-01491000.
3. **Confidential source material and the edges that point to it.** Premises lists are a leak.
4. **Coerced statements and disputed identity assertions.**
5. **Sexual, medical, school, and child-related material.** An evidence protocol that cannot refuse these should refuse all person-level records until it can.
6. **Content whose digest is identifying.** Hashing a unique photograph and publishing the CID plus “redacted” still lets anyone with the photograph prove it was the one in Witness. Sealing must say whether the digest remains public.
7. **Generated restatements of sealed facts.** Article IV says generated material remains generated. It does not say a generated paragraph must be withdrawn when its parent is sealed. If the summary keeps the name, sealing the image is theater.

Deletion here does not mean amnesia for the public instrument record. It means a designed loss of *availability* of harmful bytes, with a remaining tombstone that is itself minimized.

---

### Constitution objections to preserve

**Article V is too strong.**  
“They do not erase the records they address” is the right rule for a provisional discharge value. It is the wrong rule for a named accusation, a face, or a source file. If Article V cannot yield to sealing, Witness cannot enter journalism, medicine, law, or human-rights documentation. Those are domains the README already names.

Needed amendment shape, not final text:

- Distinguish *public instrument history* from *person-concerning payload*.
- Permit a sealed state: the node ID and a non-identifying reason code remain; the payload, labels, source URI, and author display name may become unavailable.
- Permit a redacted state: a replacement payload is stored as a new record; the original payload is restricted, not displayed as current.
- Permit a destroyed-or-never-exported state for material that should not have been ingested.
- Require interfaces to show status without reconstituting the harm.

Until that amendment exists, the honest operational rule is: do not ingest person-concerning records.

**Article VI is close, and incomplete.**  
“Withheld for safety or privacy” is listed. It is not specified who may assert that state, how long it lasts, whether the CID stays public, or how a verifier treats a sealed node. Unknown must include `sealed`, `redacted`, `restricted`, `destroyed`, and `not-exportable`, or those conditions will be stuffed into ordinary missingness and look like sloppy collection.

**Article VII lists privacy as a verification dimension.**  
A dimension without states is a slogan. Privacy needs at least: `not-assessed`, `no-personal-data-expected`, `personal-data-present`, `consent-documented`, `consent-withdrawn`, `sealed`, `unsafe-to-publish`. None of those states should raise an integrity score.

**Article X hides the operator.**  
No institution receives privileged *truth* status. The operator still receives privileged *retention* status. That concentration of power is the privacy issue. It should be disclosed as a departure whenever a deployment cannot seal or cannot propagate withdrawal.

**Article I puts testimony in Observed.**  
For human-rights work, a testimony record is closer to a restricted custody object than to a gauge tick. Mixing them teaches importers that a quote is just another measurement.

---

### What a responsible privacy design has to require

Issue #3 should not produce a policy paragraph. It should produce protocol objects.

**Minimization at ingest.**  
Default deny for payloads that contain personal data. The river-gauge pilot should stay a river-gauge pilot because it can be public. That is a feature of the chosen domain, not proof that the protocol is safe.

**Separate objects.**

- `Restriction` — who asserted it, legal or ethical basis code, scope (payload / edges / author map / CID), expiry or review date, emergency flag.
- `Redaction` — new record, old record restricted, reason code, whether CID of old bytes remains published.
- `Withdrawal` — subject or controller assertion that consent is gone; does not require the subject to hold the original signing key.
- `Tombstone` — what a replica may retain: node ID, category, restriction status, non-identifying reason, time. Not the name. Not the image. Not the quote.

**Chain of copies.**  
Exports must carry restriction objects. A replica that drops them is non-conforming. A replica that ignores them is hostile. The protocol should say both sentences.

**CID policy.**  
If the payload is personal, publishing its digest is a publication of an identifier. Sealed personal payloads should have `cid_public: false` or an encrypted digest usable only by parties who already have the bytes. Otherwise “we redacted the dashboard” is false.

**No inferred identity from sealed premises.**  
If a premise is sealed, an inference that depended on identifying that premise must not keep the identifying claim as current. The inference becomes `undermined` or `restricted`, not still true with a missing footnote.

**Generated children die with the parent.**  
A Generated explanation of a sealed record is still a copy of the harm if it repeats identifiers. It must be restricted on the same edge.

**Dashboard default.**  
If restriction status is not implemented, the dashboard must refuse person-level labels rather than render them with a small “Observed” badge. Issue #7 is a privacy issue, not only an XSS issue.

### Existing work to use

Do not invent a private theory of erasure.

- Data-protection erasure and restriction as *availability* changes, not as proof the event did not occur.
- Archival sealing and closure periods (archives already know Article V is not absolute).
- PREMIS rights and restriction notes.
- Journalistic source-protection practice: some premises are not stored in the same system as the publishable claim.
- ICRC / humanitarian data-responsibility rules on do-no-harm and onward transfer.
- C2PA redaction of assertions, with the limit that leftover bindings can still identify.

Issue #4 should record where those instruments already have restriction semantics so issue #3 does not mint a parallel vocabulary.

### Failure modes if this is deferred

- A demo fixture that includes a real name “just for local use” leaks through Git, backups, or a screenshot.
- A well-meaning importer snapshots a public meeting video and calls faces Observed.
- A retraction is appended; the original payload remains the object search engines and models train on.
- A court or police demand for the “immutable evidence store” is served on a volunteer operator.
- Reviewers in human rights correctly refuse to participate because the constitution promises they cannot get the bytes back.

### Boundary

This review does not ask Witness to become a court, a newsroom, or a data-protection authority. It does not say public instrument series must be deletable at will. It says Article V without a sealing object makes the project unsafe for the high-consequence domains listed in its own README.

### What would change this objection

A versioned restriction/redaction/tombstone profile.  
Explicit CID-publication rules for personal payloads.  
Generated and inferred children that cannot remain current when a parent is sealed.  
A written ingest ban on person-concerning material until that profile exists.  
Article V amended so append-only history and payload unavailability can coexist.  
A minority report kept on file if the project refuses the amendment.

Until then, the conforming human-rights position is simple: Witness may remember a river. It may not remember a person.

---

That is the second completed review. Paste it as a comment on [issue #3](https://github.com/seanebones-lang/witness/issues/3), with a pointer from [issue #1](https://github.com/seanebones-lang/witness/issues/1). Do not summarize it as privacy approval. The sentence to keep on the public record is: Article V is fit for gauges and unfit for named people, and person-concerning ingest should stay closed until a sealing object exists.