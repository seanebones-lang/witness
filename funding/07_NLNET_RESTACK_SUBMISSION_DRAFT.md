# Draft: NLnet Restack proposal

**Status:** preparation draft only. It is not submitted. Before submission, Sean must verify the European-dimension explanation, provide the required generative-AI disclosure and conversation logs if AI assistance is used, and review every answer in his own words.

## Proposed fund

**NLnet Restack** — current deadline: November 3, 2026, 12:00 CET.

## Proposed title

**Witness Protocol: open verification records for reproducible software and research workflows**

## Request

**€35,000 over six months.**

## Short project summary

Witness is an open-source protocol and reference implementation for preserving an inspectable distinction between source-backed observations, derived inferences, and generated material. It addresses a practical reproducibility problem: researchers and software users can preserve files and logs, yet still lose the relationship between the exact source artifact, the transformation performed, the claim derived from it, and the conditions that would update or falsify that claim.

This project will produce Witness Protocol v1: a language-neutral evidence envelope, open conformance fixtures, an independent command-line verifier, and a reproducible source-preservation reference package. It will add explicit correction and supersession records, signed producer metadata with documented limits, and a local replay path that rejects missing premises and misleading type transitions. The result will be an openly licensed building block for reproducible software workflows, evidence preservation, and trust-enhancing technical infrastructure.

The project does not train, deploy, or integrate an AI model. It records the provenance boundary when a workflow contains generated material so that generated output cannot be silently presented as evidence.

## Proposed work and budget

| Work | Effort | Amount |
| --- | ---: | ---: |
| Protocol v1 schema, canonical serialization, and profile documentation | 6 weeks | €10,500 |
| Conformance fixtures and standalone verifier | 5 weeks | €8,750 |
| Source-preserving reference package and reproducible replay | 4 weeks | €7,000 |
| Correction, supersession, key-compromise, and invalid-state handling | 3 weeks | €5,250 |
| Documentation, packaging, release, and project management | 2 weeks | €3,500 |
| **Total** | **20 weeks** | **€35,000** |

This is a cost-recovery planning estimate. It excludes unrelated commercial work, speculative AI-model development, and unverified scientific claims.

## Relevant existing work

The public Witness repository already includes:

- an MPL-2.0 Rust implementation with automated core tests;
- append-only storage and explicit Observed / Inferred / Generated records;
- premise links, falsifiers, and validation that rejects missing or invalid relationships;
- a curated USGS source-preservation reference bundle that deliberately does not overstate an extracted value as a conforming observation;
- a frozen local Grid-to-Witness Proof A package with checksummed artifacts and visible limitations;
- public contribution, provenance, licensing, citation, reviewer, and disagreement-handling materials.

The new work is a portable protocol and verifier, not a repackaging of existing documentation.

## Technical challenge

Existing provenance systems and workflow tools can describe artifacts, but a usable cross-producer record requires more than a database table or a hash:

- source material can be incomplete, corrected, withheld, or transformed;
- inferences need stated premises and update conditions;
- generated material must remain visibly separate from source-backed observations;
- producers may differ in identity assurance and authorization;
- a later correction must not erase the original record;
- a verifier must reject a record that looks complete but lacks required evidence.

The project will test whether a compact common envelope and conformance suite can handle those cases without making normal use impractical. That feasibility is unproven.

## Open-source and ecosystem plan

All project outputs will be publicly available under existing recognized open-source terms. The protocol, fixtures, verifier, and documentation will be developed in public repositories. The project will seek feedback from reproducible-research, research-software, and provenance communities. The intended users are maintainers of research workflows, open-source software projects, and organizations that need inspectable records across automated transformation steps.

## European dimension — required owner input

A U.S.-based applicant must not invent a European dimension. Before submitting, replace this section with a factual statement supported by a named European partner, user, contributor, standards participant, deployment context, or other qualifying relationship.

Possible legitimate paths, if they become true:

- an EU or Horizon Europe-associated research/reproducibility partner agrees to review or test the protocol;
- the work contributes to an open standards or open-source infrastructure effort with documented European users;
- a European institution commits to an open pilot or conformance review.

Until one is real and documentable, **do not submit** this application.

## Comparison with related work

Witness should be compared honestly with research-data repositories, electronic lab notebooks, workflow provenance systems, software supply-chain metadata, W3C PROV-style provenance models, and cryptographic transparency logs. The proposal must identify where existing systems already solve the problem and where Witness adds value: explicit epistemic type separation, premise and falsifier requirements for inferences, correction without erasure, and a verifier that exposes unsupported states.

This comparison requires final research and should not claim novelty without citations and domain review.

## Technical risks

- The protocol may be too domain-specific or too burdensome for routine users.
- Cross-language canonicalization may be difficult to specify without ambiguity.
- Signatures can create false confidence if identity and authorization limits are unclear.
- Existing open standards may be sufficient, making a separate protocol unnecessary.
- External reviewers may conclude that a narrower interoperability contribution is more valuable than a new protocol.

Each risk will be published with its resulting decision rather than hidden.

## Required pre-submission checklist

- [ ] NLnet confirms or the applicant can truthfully state a qualifying European dimension.
- [ ] The selected Restack fund is confirmed as a better fit than CodeSupply.
- [ ] Sean rewrites/reviews all final form answers in his own words.
- [ ] Any generative-AI assistance is disclosed with the required prompts/interactions/logs.
- [ ] The proposal identifies all third-party material and software terms.
- [ ] A final factual comparison with adjacent projects and standards is completed.
- [ ] The €35,000 rate, tax, entity, and payment details are confirmed.
- [ ] An exact submitted copy is retained in a private dated record.
