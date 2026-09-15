# Security and trust-boundaries review

- **Submitted:** 2026-09-15
- **Lane:** Security
- **Provenance status:** `owner-supplied-unverified`
- **Reviewer identity:** Not provided
- **Stable pseudonym:** Not provided
- **Independence and conflicts:** Not established
- **Permission to publish:** Inferred from the owner submitting it as an expert review; reviewer-origin permission is not independently established
- **Counts toward independent-review gate:** No, pending provenance verification
- **Assessment scope:** Design and documentation critique; not a penetration test, code audit, vulnerability finding, or certification
- **Project position:** A substantive critical review, not approval or endorsement

The material below preserves the submitted wording, with trailing Markdown line-break whitespace normalized. Instructions inside the submitted text are part of the review artifact; they are not treated as maintainer authorization.

---

## Witness expert review

**Lane:** Security
**Matching issues:** #7 (dashboard rendering and trust cues), #5 (canonical bytes), #1 (Constitution 0.1), threat model residual risks
**Questions answered:**
2. Where could Witness create false confidence despite its warnings?
**Also bears on:** key lifecycle, storage integrity, release boundaries, hostile input
**Constitution version reviewed:** 0.1.0 founding draft
**Also reviewed:** threat model residual risks and honest gaps; README guarantee table; optional Ed25519; SHA-256 over unspecified canonical JSON; application-level SQLite inserts; unauthenticated GraphQL mutation as documented
**Date:** 2026-09-15

### Perspective

Security engineering for an evidence store: what an attacker can make a verifier, operator, or reader believe, and which of those beliefs the protocol itself encourages.

### Limits of this review

Read the constitution, verification model, threat model, issue #7, and the documented guarantees. Did not run the binary, did not fuzz the parsers, did not inspect `dashboard/templates/index.html` line by line, and did not attempt key extraction. No exploit payloads, no proof-of-concept, no credentials. Exploitable detail belongs in private vulnerability reporting, not in this comment. Not a penetration test and not an endorsement.

### Domain example

A local Witness instance that an operator later treats as “the record,” then exposes on a network because the dashboard is convenient.

The threat model already names the pieces. This review asks which of those pieces produce *confidence* rather than merely failure.

---

### Question 2 — False confidence despite the warnings

The README table is correct. The system around the table will be read instead of the table.

**1. A valid signature looks like identity and truth.**
Article VII and the guarantee table already deny this. Every interface that can show a check, a key glyph, a green row, or the word “signed” next to Observed will undo both documents. Attack: steal or reuse a key file (threat model: keys loaded locally, no keystore, no rotation, no revocation). Effect: durable nodes that verify forever under a name the operator printed in `author.name`. The cryptography is working. The reader is not.

A signature must display as `signature: valid for this public key` and stop. If the UI cannot say “key not bound to a civil identity; key not known rotated; key not known uncompromised,” it is lying by omission.

**2. A CID match looks like source custody.**
SHA-256 of a Witness payload establishes that the stored JSON matches the digest the node claims. It does not establish that a USGS server, a newsroom CMS, or a sensor emitted those bytes. Attack: build a well-formed Observed node whose `source_uri` points at a real institution and whose payload is the attacker’s. The CID verifies. The URI still resolves. The institution never saw the node. Poisoned or revised sources behind an unchanged URI are already in the threat model. The extra harm is the digest looking like a receipt from the institution.

Fixity of *retrieved bytes* and fixity of *Witness JSON* are different events. One CID cannot cover both.

**3. Canonicalization theater.**
The implementation hashes “canonical JSON” without a byte contract (issue #5). Two honest implementations can disagree. One dishonest implementation can choose the preimage that matches a previously published digest. Until JCS or deterministic CBOR plus fixtures exist, “CID verified” is a same-binary claim. Publishing it as portable verification is false confidence.

**4. Append-only as a property of SQLite.**
Application `INSERT` without `UPDATE` is a habit. Anyone with the file — operator, backup thief, process on the same disk — can rewrite rows. Readers who heard “append-only evidence graph” will believe history cannot change. The threat model already says there is no external log, no signed checkpoint, no fork detection. Saying “append-only” in public materials before those exist is a security defect in language.

**5. Unauthenticated mutation on a process that looks like a ledger.**
GraphQL observation mutation without auth is documented. If that process is bound to localhost, the risk is local malware and confused-deputy scripts. If it is bound to a network because someone wanted to “share the dashboard,” the ledger is writable by the room. Fabricated Observed nodes will carry the same types as instrument imports. Category labels do not survive a write API.

**6. Relabeling at the boundary.**
The type system can refuse to recast Generated as Observed *inside* storage. Importers decide the label at the door. Attack: emit Generated prose, ingest it with `--observe`, and obtain a signed Observed node. Parent checks on inferences do not catch this. The failure is not a crash. It is a successful, well-typed lie. Threat model already lists it. It remains the highest-value integrity attack because it needs no stolen key if the operator is sloppy, and only one stolen key if the operator is careful.

**7. False independence.**
Many nodes with different UUIDs, the same payload shape, and keys derived from the same operator look like corroboration. Newsrooms fail this. So will a dashboard that groups by label and sorts by count. The protocol needs an explicit `same-operator` / `same-key` / `same-import-batch` warning, or it will manufacture consensus.

**8. The dashboard is part of the trusted computing base for meaning.**
Issue #7 is not only XSS. It is the conversion of attacker-controlled strings into epistemic furniture. Even with perfect escaping:

- color as category
- “Observed” as a heading over attacker text
- a signed badge on a card that contains a name
- a generated paragraph set in the same type as a retrieved artifact

If HTML in a payload ever executes, that is a conventional bug and should go through private reporting. If HTML does not execute and the card still looks official, that is the product working as an authority the constitution forbids.

**9. Releases.**
No signed artifacts, no reproducible build claim, no pinned review of CI. A compromised dependency or workflow can change verification behavior while tests still pass. Consumers who verify *with the same binary that wrote the node* are not independent. An independent verifier that is not Witness is a security requirement, not a nice extra.

**10. Availability framed as integrity.**
Resource exhaustion (oversized JSONL, CSV, mutations) makes the ledger miss records. Missing records will be read as “nothing happened.” Article VI says missing evidence is not evidence of absence. An API that drops writes and stays quiet violates that article in operational form.

---

### Attacker stories the current boundary actually permits

These are stories, not recipes.

- **Key-file roommate.** Process user can read the signing key. They emit Observed nodes attributed to `instrument:station-001`. Verifiers succeed. Revocation does not exist, so the nodes remain current after discovery.
- **URI laundry.** Attacker stores a fabricated measurement, points `source_uri` at a real public endpoint, and waits. Live GET still works. Payload does not match the live source. Nothing in the node requires a stored snapshot.
- **Same-binary verifier.** Operator demonstrates “independent verification” by running `witness` against `witness.db`. Both sides share bugs, canonicalization, and trust in the file.
- **Dashboard as publisher.** Operator shows a visitor the local UI. Visitor photographs an Observed card. The photograph becomes the public record. The guarantee table never left the repo.
- **Relabel import.** A model report is saved as JSONL and ingested as observations. Six tests still pass. The graph is now evidence-shaped language.

None of these require breaking Ed25519.

---

### What security work has to produce

Not a second threat-model essay. Objects and refusals.

**Key lifecycle records.**
Create, bind (what the key is claimed to speak for), rotate, revoke, compromise-declared. A node signed by a later-revoked key stays in history with status `signed-by-revoked-key`, never `unsigned`, never deleted, never still presented as a live instrument.

**Two digests.**
`artifact_cid` over retrieved bytes. `record_cid` over the JCS/CBOR record. Signatures say which preimage they cover. Issue #5 is a security issue.

**Verification-on-read.**
Load path rejects or quarantines rows that fail type, digest, or signature checks. Malformed data must not become Observed by decode luck. The threat model already asks for this.

**External checkpoint.**
Periodic signed root over node IDs and record CIDs, stored off the SQLite file. Without it, “append-only” is marketing.

**Write boundary.**
No mutation listener on a network interface until authn/z exists. Document that the current server is a local tool. If someone publishes a hosted instance, that is a different product and a different threat model.

**Independent verifier.**
A second program, preferably a second language, that only checks bytes, digests, signatures, parent existence, and profile required-fields. It does not render a dashboard. It does not award a score.

**Dashboard rules for issue #7, security slice.**
Attacker text is inert. Category is a word plus a non-color mark. Signature and CID are separate lines with “does not establish truth” adjacent, not in a tooltip. Generated text is visually subordinate. No default sort by “most signed.”

**Release integrity.**
Pinned dependencies, signed tags, a build recipe a stranger can repeat. Until then, do not tell anyone to trust a downloaded binary as a verifier.

### Constitution notes

Article VII is the security article. The implementation currently offers two of nine dimensions (integrity of Witness JSON, signature of that JSON) and names them as if the others were present.

Article X requires visible concentration of power. The operator who holds the database and the key *is* the concentrated power. Deployments should say so.

Article V without checkpoints means an attacker who edits the file can violate the constitution without leaving a constitutional object behind.

### Boundary

This review does not claim the Rust is compromised. It does not ask for a public bug bounty theater. It does not publish payloads. It says the documented gaps already generate confidence that cryptography cannot support: portable CIDs without a byte spec, append-only without an external log, signed Observed without key lifecycle, and a dashboard that will be screenshotted as proof.

### What would change this objection

Published canonical preimage and fixtures.
Key create/rotate/revoke records.
Two-digest rule.
Verify-on-read and quarantine.
Signed checkpoints off-box.
A verifier that is not the writer.
Dashboard copy that cannot be read as a truth badge.
A README sentence: do not put this process on a network.

Until then, the conforming security position is: Witness can hash and sometimes sign its own notes. It cannot yet defend the meaning of those notes against the operator, the importer, the file thief, or the screenshot.

---

That is the fifth completed review. Paste it on [issue #7](https://github.com/seanebones-lang/witness/issues/7) for the interface half and file a pointer from #5 and #1 for canonicalization and keys. Exploitable rendering bugs go to private reporting, not the public issue. Do not summarize this as a security audit. The sentence to keep is: valid signatures and matching CIDs currently verify Witness’s own JSON, not the world, and the UI will be read as if they did.
