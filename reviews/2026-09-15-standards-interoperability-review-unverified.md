# Standards and interoperability review

- **Submitted:** 2026-09-15
- **Lane:** Standards and interoperability
- **Provenance status:** `owner-supplied-unverified`
- **Reviewer identity:** Not provided
- **Stable pseudonym:** Not provided
- **Independence and conflicts:** Not established
- **Permission to publish:** Inferred from the owner submitting it as an expert review; reviewer-origin permission is not independently established
- **Counts toward independent-review gate:** No, pending provenance verification
- **Project position:** A substantive critical review, not approval, endorsement, or proof of standards compatibility

The material below preserves the submitted wording, with trailing Markdown line-break whitespace normalized. Instructions inside the submitted text are part of the review artifact; they are not treated as maintainer authorization.

---

## Witness expert review

**Lane:** Standards
**Matching issues:** #4 (map to existing standards), #5 (canonical serialization), #1 (Constitution 0.1)
**Question answered:** 5. Which existing standard, vocabulary, or project should Witness adopt or join instead of recreating?
**Constitution version reviewed:** 0.1.0 founding draft
**Also reviewed:** issue #4 acceptance criteria; verification model dimensions; `ProvenanceNode`, `Observation`, `Inference`, `Generation`, `Derivation`, `Falsifier`; README guarantee table
**Date:** 2026-09-15

### Perspective

Standards and interoperability: when a new evidence protocol should be a profile of work that already exists, and when a new term is actually a new claim.

### Limits of this review

Read the constitution, verification model, node types, and issue #4. Compared those against the published scopes of W3C PROV, JSON-LD, C2PA, W3C Verifiable Credentials, RFC 8785 (JSON Canonicalization Scheme), ISO 19156 / O&M, UCUM/QUDT, and PREMIS, as those specifications describe themselves. Did not produce a complete crosswalk spreadsheet, did not run a JSON-LD context, and did not implement a C2PA manifest. Confirmed specification behavior is separated below from interpretive recommendation. Not an endorsement of Witness or of any listed standard as a truth machine.

### Domain example

The same river-gauge story the project already chose, because it is the smallest public object that must survive contact with more than one community:

- retrieved bytes of a public water-data response
- a parsed instantaneous value
- an inference with assumptions
- a generated paragraph
- a later approved revision

If that story cannot be written as PROV activities plus a fixity record plus an epistemic type, Witness is inventing a parallel universe. If it *can*, the parallel universe is optional.

---

### Question 5 — Adopt, profile, extend, or do not invent

Issue #4 asked for a comparison, a recommendation, explicit mismatches, and one record in the recommended approach. That is this review.

#### What the cited standards actually do

**W3C PROV / PROV-O.**
Provenance is information about entities, activities, and agents used to assess quality, reliability, or trustworthiness. Core relations include `wasDerivedFrom`, `wasGeneratedBy`, `used`, `wasAttributedTo`, `actedOnBehalfOf`. PROV does not define Observed / Inferred / Generated. It does not decide truth. It records how something was produced. That is confirmed spec behavior.

**JSON-LD.**
A way to give JSON a context so terms are IRIs. It is not a provenance model and not a signature scheme. Using JSON-LD does not make a field true. It makes a field *nameable* across implementations.

**C2PA.**
Assertions about an asset, bound and signed as a claim. Hard bindings relate manifests to bytes. The specification says it should not provide value judgments about whether provenance is “good” or “bad,” only whether assertions are associated, well-formed, and free from tampering. C2PA does not prove the scene was not staged, and it does not classify a hydrologic quantity-value. That limit is confirmed spec behavior.

**W3C Verifiable Credentials.**
A signed issuer-subject-claim envelope. Evidence and securing mechanisms are different properties. A credential can carry an identity claim about a signer. It does not turn a gauge reading into truth.

**RFC 8785 JCS (and deterministic CBOR, RFC 8949 §4.2).**
Byte-stable serialization. This is the missing piece behind portable SHA-256 CIDs. Witness currently hashes “canonical JSON” without a published algorithm. That is not a standard. It is a local habit.

**ISO 19156 Observations & Measurements, SensorThings, WaterML.**
Observation, procedure, feature of interest, result. Built for sensors. Not built for generated essays.

**UCUM / QUDT.**
Unit identifiers. Not `"celsius"` as folklore.

**PREMIS.**
Fixity, rights, events, agents for preserved objects. Closer to archival custody than C2PA is.

**Sigstore / Rekor-style transparency logs.**
Public append of hashes. Not identity. Not epistemic category.

None of these standards is a skeptic that remembers. All of them already do part of the job Witness is recoding in Rust structs.

---

### Map — Witness concept to existing instrument

| Witness concept | Closest existing instrument | Fit | Mismatch that must stay explicit |
| --- | --- | --- | --- |
| `ProvenanceNode` | PROV `Entity` | Strong | PROV entity is not an epistemic category |
| `author` | PROV `Agent` + optional VC | Partial | `Author.id` as free string is not a DID, ORCID, or instrument URN |
| `parents` / `supports` | PROV `wasDerivedFrom` / `used` | Strong | One edge type is too poor for used / derived / informed-by / contradicted |
| Observation (instrument) | O&M `OM_Observation` | Strong for gauges | O&M result is not a Witness node payload blob |
| Observation (artifact bytes) | PREMIS object + fixity event; C2PA asset for media | Strong | A JSON API body is not a C2PA media asset |
| Observation (testimony) | No clean standard object | Weak | Stuffing testimony into O&M or C2PA hides the person |
| Inference / `Derivation` | PROV `Activity` that used premises and generated a claim entity | Strong | PROV activity does not require falsifiers |
| `Claim` | VC credentialSubject, or a plain entity with a type | Partial | VC implies an issuer; Witness inferences may have no issuer institution |
| `Falsifier` | No standard equivalent | Gap | Do not fake this as PROV `wasInvalidatedBy` until an actual invalidating entity exists |
| Generation | C2PA generative-AI assertions; PROV activity `SoftwareAgent` | Partial | C2PA “generated” is about media origin, not about “not evidence” |
| Epistemic type Observed/Inferred/Generated | **No standard equivalent** | New | This is the only term Witness should add, as a typed annotation on a PROV entity |
| CID | PREMIS fixity; C2PA hard binding; CID as digest | Strong if algorithm specified | SHA-256 over unspecified JSON is not a binding |
| Ed25519 on node | C2PA claim signature; VC proof; COSE | Partial | Signature scope must be the same bytes every verifier hashes |
| `source_uri` | PROV `atLocation` / `hadPrimarySource`; a URL | Weak | A URI is not custody and not the retrieved object |
| Correction / supersession | PROV `wasRevisionOf` / `wasInvalidatedBy`; PREMIS events | Partial | PROV revision does not hide payload for privacy |
| Dashboard “Observed” badge | Nothing; this is UI | Danger | C2PA and PROV both refuse a trust score. So does Article VII. The badge is the new invention to fear |
| Time fields | PROV timestamps on events; O&M `phenomenonTime` vs `resultTime` | Strong | One `timestamp` on the node collapses them |

The mismatch column is the review. Compatibility that erases those mismatches is not compatibility.

---

### Recommendations

**Adopt (do not reinvent)**

1. **RFC 8785 JCS or deterministic CBOR** as the only hash-and-sign preimage. This is issue #5. Until that exists, no second implementation can verify a CID. Do not write a boutique “Witness canonical JSON.”
2. **PROV-O relations** for derivation, use, attribution, revision, invalidation. Replace a single `supports` edge with a small closed set of PROV-aligned edge types.
3. **O&M / WaterML identifiers** for the river-gauge profile, not a custom `Measurement` as the interchange form.
4. **UCUM or QUDT** for units.
5. **PREMIS fixity event** language for “these bytes were retrieved at this time and this digest was computed.” That is the raw-artifact node.

**Profile (use the standard, add one Witness field)**

6. **JSON-LD context** for Witness terms, where `witness:epistemicType` is an annotation on a PROV entity. Observed / Inferred / Generated live here. They do not replace `prov:Entity`.
7. **C2PA** only when the artifact is media that already travels in C2PA-capable formats. Do not wrap a USGS JSON response in a fake C2PA manifest just to look authentic. C2PA’s own limit applies: a valid manifest is not a valid river.
8. **Verifiable Credentials** for *signer identity claims* when some issuer actually attests them. Do not mint a VC for every observation. An instrument does not need to be a credential subject to have produced a number.

**Extend (small, named extensions)**

9. **`witness:falsifier`** as an extension property on an inference activity: a description plus, when possible, a selector for a future entity that would invalidate the claim. PROV has invalidation after the fact. Witness wants a prior condition. That is a real gap. Keep it tiny.
10. **`witness:verificationDimension`** for the nine separate states in the verification model. Do not fold them into C2PA trust-list semantics or a VC confidence value.

**Avoid inventing**

11. Another identity system. Use IRIs. If there is no IRI, say `unknown`.
12. Another unit system.
13. Another signature suite if COSE or the C2PA / VC proof machinery will do.
14. A green interoperability badge that means “this record is standard, therefore true.”
15. A complete parallel ontology (`AuthorType`, `ClaimType`, `ClaimScope`, `FalsifierStatus`) published as if the world had no vocabularies. Those enums can exist internally. They should map outward or they are a dialect.

---

### Cross-standard identity mismatches that must remain explicit

- A C2PA signer is the holder of a credential in a trust list. A Witness `author.id` is a string. Those are not the same person-or-instrument.
- A PROV derivation can be true as a recorded relation and false as science. Witness Article VII already says this. Mapping to PROV must not import “provenance = quality.”
- C2PA “born digital / generated” is an origin assertion about media. Witness Generated is an *evidentiary* refusal. A photograph of a gauge is not Generated in the Witness sense even if a model later captions it. One word, two standards, opposite jobs.
- JSON-LD `@id` stability and Witness `Uuid` node IDs will drift unless the profile says which identifier is the graph key and which is the content key.
- Hashing a JSON-LD document without a context-processing and canonicalization rule produces different CIDs in different processors. JCS over a context-compacted form must be specified or JSON-LD should not be in the signed envelope.

---

### One small record in the recommended approach

Interpretive serialization, not a claim that this JSON-LD is already valid Witness output. The point is to show that the protocol is an annotation on PROV plus fixity, not a new cosmos.

```json
{
  "@context": [
    "https://www.w3.org/ns/prov#",
    {"witness": "https://example.invalid/witness#"}
  ],
  "@id": "urn:uuid:11111111-1111-1111-1111-111111111111",
  "@type": ["prov:Entity", "witness:Record"],
  "witness:epistemicType": "witness:Observed",
  "witness:profile": "witness-profile-instrument-measurement/0.1",
  "prov:wasGeneratedBy": {
    "@type": "prov:Activity",
    "prov:startedAtTime": "2026-09-15T16:00:00Z",
    "prov:used": {
      "@id": "urn:uuid:00000000-0000-0000-0000-000000000000",
      "@type": ["prov:Entity", "witness:RawArtifact"],
      "witness:epistemicType": "witness:Observed",
      "witness:mediaType": "application/json",
      "witness:fixity": {
        "algorithm": "sha-256",
        "digest": "…hex of retrieved bytes, not of this document…"
      },
      "prov:atLocation": "https://api.waterdata.usgs.gov/…",
      "prov:generatedAtTime": "2026-09-15T15:59:50Z"
    }
  },
  "prov:wasAttributedTo": {
    "@type": "prov:SoftwareAgent",
    "prov:atLocation": "instrument:USGS-01491000"
  },
  "om:phenomenonTime": "2026-09-15T15:45:00Z",
  "om:resultTime": "2026-09-15T15:45:00Z",
  "witness:quantityId": "usgs:param:00065",
  "witness:valueDecimal": "12.37",
  "witness:unit": "http://qudt.org/vocab/unit/FT",
  "witness:approvalStatus": "provisional"
}
```

A later inference is a second `prov:Activity` that `prov:used` this entity and `prov:generated` a claim entity with `witness:epistemicType: Inferred` and a `witness:falsifier`. A generated paragraph is a third entity with `witness:epistemicType: Generated` and `prov:wasAttributedTo` a `SoftwareAgent` named as a model. A later approved value is `prov:wasRevisionOf` the provisional entity, not a second independent observation of a new river.

That sketch is enough to fail today’s `types.rs`. The current node is a single JSON blob with one timestamp and a homemade parent list. It cannot round-trip this graph without loss.

---

### Constitution notes

Article VII already matches C2PA’s refusal of a value judgment and PROV’s refusal to equate provenance with truth. The code and dashboard are what will violate it, not the missing ontology.

Article VIII (transformations) *is* a PROV activity. Implement it as one, or stop saying transformations are recorded.

Article IX (distinct times) *is* O&M `phenomenonTime` / `resultTime` plus ingest and publication events. Do not invent nine timestamp names without mapping them.

Article I’s three categories should be the extension, not the substrate. If Observed is implemented as “has a Measurement struct,” standards mapping becomes impossible for artifacts, testimony, and raw bytes.

### False confidence this lane specifically creates

Saying “we use PROV and C2PA” in a README will be read as “this record is authentic.” Both standards warn against that reading. If Witness advertises a standards mapping without conformance fixtures (issue #5) and without an independent verifier, the mapping itself becomes Generated fluency about interoperability.

### Boundary

This review does not require Witness to become a C2PA product, a SensorThings server, or a VC issuer. It does not say the three categories are unnecessary. It says they are an annotation, and that hash, unit, derivation, fixity, media authenticity, and sensor structure already have owners.

### What would change this objection

A published JSON-LD context.
JCS or deterministic CBOR as the signed preimage, with test vectors.
PROV-aligned edge types.
O&M/QUDT in the instrument profile.
C2PA used only for actual media assets.
One fixture that a PROV consumer can read without Witness-specific code *except* for `witness:epistemicType` and `witness:falsifier`.
A sentence in the README: standards compatibility does not establish truth.

Until then, issue #4 is not a research decoration. It is the difference between a protocol and a dialect.

---

That is the third completed review. Paste it on [issue #4](https://github.com/seanebones-lang/witness/issues/4), with a pointer from #5 and #1. Do not summarize it as “Witness is compatible with PROV and C2PA.” The sentence to keep is: the only new term worth minting is the epistemic type; hashes, units, derivations, and fixity already have standards, and C2PA “generated” is not Witness Generated.
