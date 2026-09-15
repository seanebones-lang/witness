# Archives and libraries review

- **Submitted:** 2026-09-15
- **Lane:** Archives or libraries
- **Provenance status:** `owner-supplied-unverified`
- **Reviewer identity:** Not provided
- **Stable pseudonym:** Not provided
- **Independence and conflicts:** Not established
- **Permission to publish:** Inferred from the owner submitting it as an expert review; reviewer-origin permission is not independently established
- **Counts toward independent-review gate:** No, pending provenance verification
- **Assessment scope:** Design critique; not a transfer review, OAIS ingest, storage-media inspection, trusted-repository assessment, or certification
- **Project position:** A substantive critical review, not approval, adoption, or endorsement

The material below preserves the submitted wording, with trailing Markdown line-break whitespace normalized. Instructions inside the submitted text are part of the review artifact; they are not treated as maintainer authorization.

---

## Witness expert review

**Lane:** Archives or libraries
**Matching issues:** #3 (retention / deletion), #6 (correction without silent replacement), #4 (PREMIS / OAIS, not only PROV), #8 (source-preserving pilot), #1 (Constitution 0.1)
**Questions answered:**
3. What must sometimes be corrected, sealed, redacted, or deleted rather than preserved indefinitely?
4. What does a responsible chain of custody require in your work?
**Constitution version reviewed:** 0.1.0 founding draft
**Also reviewed:** README prototype limits; application-level append-only SQLite; `source_uri` without stored bytes; optional signature; single node timestamp
**Date:** 2026-09-15

### Perspective

Archival practice: original artifacts, fixity, provenance, custody, retention, and the cases where preservation must yield. The job is not to believe the file. The job is to keep the file as received, say what happened to it, and refuse to call a database row the object.

### Limits of this review

Read the constitution, README limits, issues #3 and #6, and the node shape. Did not review a real transfer package, did not run an OAIS ingest, and did not inspect storage media. No donor files, no restricted series. Not a certification of the repository as a trusted digital archive. Not an endorsement.

### Domain example

A public agency posts a water-data file. A researcher saves “the record” into Witness. Five years later the agency URI returns a different body, the researcher’s laptop is gone, and a student verifies the Witness CID and calls the number official.

That is not custody. That is a souvenir with a hash.

---

### Question 4 — What responsible custody requires

Chain of custody is not a list of names on `ChainOfCustody { collectors, handlers, seals, timestamps }`. That struct is a caption. Custody is a sequence of *events against a stored object*.

For an archival object, the minimum inspectable chain is:

1. **The object as received.** Exact bytes, media type, filename as received, byte length. Not a parsed `Measurement`. Not a URI.
2. **The intake event.** Who acquired it, from where, when, by what tool, under what authority or terms. A GET to a public API is an intake event. So is a donor transfer. They are not the same event.
3. **Fixity at intake and at every later check.** Algorithm, digest, time of check, who checked, match or mismatch. PREMIS already names this. A single `cid` on the Witness JSON is fixity of the *wrapper*, not of the artifact.
4. **Identity of the object across copies.** Which copy is the preservation master, which is an access derivative, which is an extraction. Derivatives are new objects. They do not replace the master.
5. **Representation information.** Enough to open the bytes later: format, encoding, schema version of the source file. JSON from an API that later changes shape is a known archival failure.
6. **Event log that cannot be the same writable file as the object store without a second copy.** Relocation, checksum failure, format migration, restriction, deaccession. SQLite habit-insert is not that log.
7. **Terms.** Public domain, license, agency terms of service, donor restriction, embargo. Custody without terms is a collection that cannot say why it may keep or must not show.
8. **Designated community.** Who this copy is for. A local demo dashboard has no designated community. A public evidence spine claims the world and then cannot support the claim.

Witness today stores a payload JSON, an optional URI, an optional signature, and an application promise that IDs are not reused. That is an application notebook. It is not a preservation repository.

Specific failures against custody:

- **`source_uri` as stand-in for the object.** Archivists already lost this fight with web archives. The live resource is not the received resource. If the pilot does not store the response body and headers, there is no artifact to keep custody *of*.
- **One digest covering the wrong bytes.** Hashing Witness’s struct does not prove the agency file is intact. Two fixities, or it is not fixity.
- **Silent normalization.** Unit strings, float conversion, timezone coercion, pretty-printed JSON are format migrations. Article VIII requires them to be recorded transformations. An archive that migrates without keeping the source representation has broken the chain even if the number looks nicer.
- **Author as custodian.** `Author` is a claimed producer. Custodian is the institution that accepted the bytes. Mixing them produces fake chain-of-custody lists generated at ingest.
- **No package.** OAIS ingest is a Submission Information Package becoming an Archival Information Package. Witness ingest is a CLI flag. There is no package identifier, no inventory, no agreement, no checksum manifest over multiple files. A gauge story that includes response body, headers, and a parsed extraction is already three files. They need a bag or equivalent, not three rows that happen to share labels.

A responsible chain for the river-gauge pilot, and nothing larger, would look like:

- store the HTTP body and selected headers as the master object
- record intake time, request URL, client identity
- compute fixity over those bytes
- store the parsed point as a *derivative* with a transformation event
- record that USGS did not transfer custody, sign, or designate Witness as a repository
- record terms: public U.S. government work, no endorsement
- refuse to call the Witness node the record of record

Until that package exists, do not tell librarians that Witness preserves sources.

---

### Question 3 — When preservation must yield

Archives are not the party that says “keep everything forever.” They are the party that says *what* is kept, *for how long*, *under which terms*, and *what remaining description is honest after a closure*.

Article V (linked correction, no erasure) is normal for the *descriptive record* of a public instrument series. It is not a license to ignore:

**Appraisal and non-ingest.**
Not every retrieved byte belongs in a preservation system. An archive that cannot refuse ingest is a dump. Person-concerning material, donor-restricted material, and accidental capture of credentials or session tokens must be stoppable at the door. Deletion of something that should never have been accessioned is not a constitutional crisis. It is appraisal.

**Closure and sealing.**
Personnel files, student records, legally embargoed series, and culturally restricted knowledge have closure periods. The descriptive record may remain (“series closed until DATE, reason code”). The payload does not remain an access copy. The privacy review already demanded a tombstone. Archives have used that pattern for a long time. Issue #3 should copy it rather than invent a crypto-only story.

**Redaction as a derivative.**
A redacted access copy is a new representation. The unredacted master, if lawfully held, stays restricted. The public graph should not hold both and call them one node. If the master must not be held, the honest event is deaccession of payload, not a black box over the same CID.

**Deaccession.**
Collections give things back, transfer them, or destroy them under policy. Destruction is rare, documented, and not the same as “the URI 404’d.” Witness needs a deaccession event that explains whether bytes were never held, were transferred, or were destroyed, without resurrecting the bytes in a diff.

**Fixity failure.**
When a checksum no longer matches, the archive does not quietly rehash the new bytes and keep the old CID. It records a failure. If Witness “repairs” a row to match a new digest, it has forged custody.

**Retention schedules.**
Public agencies already have schedules. A sidecar that promises eternal append of agency data can violate the source’s schedule and the operator’s. “We remember forever” is not always lawful and not always fundable. Retention is a recorded policy on a series, not a vibe about civilization.

What must not be deleted: the fact that a public instrument value was published and later superseded, if that series was accessioned as public. What must sometimes leave the access graph: payloads that endanger, payloads held without authority, and masters that exist only as restricted copies.

The measurement and journalism reviews said gauges can stay public. This review agrees — *if* the object stored is the received public file plus events, and *if* person-concerning accidentals are appraised out.

---

### Constitution objections to preserve

**Article II** wants source location or custody information. Location is not custody. The article should be read as requiring *which of those is present*, and as treating a URI-only observation as incomplete custody.

**Article V** matches archival description of corrections in an open series. It does not match restricted series or failed appraisal. Needs the same sealed/deaccessioned states already argued on issue #3, written in archival event language, not only privacy language.

**Article VIII** is the migration article. Archivists will believe it only when the source representation survives beside the derivative.

**Article IX** maps to intake time vs creation time vs publication time. Archives already lose cases on this. One `timestamp` is an unclassified date. Unclassified dates are how objects become undatable.

**Article X** — a library or national archive that ran Witness would have concentrated custodial power. That is normal for archives and must be named. Witness-as-everywhere-spine pretends there is no custodian. There is always a custodian.

### False confidence this lane produces

- “Content-addressed” heard as “preserved.”
- “Append-only” heard as “trusted repository.”
- A CID match heard as “this is still what the agency holds.”
- Fixtures heard as holdings. The README forbids that; screenshots will not.
- Mapping to PREMIS in a slide without packages, fixity checks over time, or a preservation policy.

### Existing work to use

- OAIS for the roles: producer, management, ingest, archival storage, access, designated community. Witness is currently a slice of ingest plus access UI.
- PREMIS for object, event, agent, rights — especially fixity events and restriction.
- BagIt or an equivalent manifest for multi-file intake (body + headers + extraction).
- WARC for web-retrieved artifacts if the pilot is an HTTP GET. A JSON field is not a WARC record.
- Retention schedules of the source institution, cited, not replaced.
- Do not pretend C2PA is an archival deposit format for agency JSON.

Issue #4 should include PREMIS and OAIS as first-class rows, not footnotes under PROV.

### Boundary

This review does not ask Witness to become a national archive or to seek ISO 16363 certification. It does not say public gauge files must expire. It says preservation is a package plus events plus policy, and a hashed application row is none of those.

If the project wants one archival-quality object in the world, make the river-gauge pilot a tiny SIP: received bytes, intake event, two fixities, derivative extraction, written terms, no endorsement by the source. That is issue #8 seen from a loading dock rather than from a schema.

### What would change this objection

Stored masters, not URIs.
Fixity of artifact distinct from fixity of wrapper, checked more than once.
Transformation derivatives that do not overwrite the received representation.
Intake, restriction, deaccession, and checksum-failure events.
A package manifest for the pilot.
A sentence operators must ship: Witness is not the archival institution of record for the source.

Until then, the conforming archival position is: Witness may point at a memory. It does not yet keep one.

---

That is the seventh completed review. Paste it on [issue #8](https://github.com/seanebones-lang/witness/issues/8) for the package shape of the pilot, with pointers from #3 and #4. Do not summarize it as archival adoption. The sentence to keep is: custody is events against stored bytes; a URI plus a hash of Witness JSON is not a chain of custody.
