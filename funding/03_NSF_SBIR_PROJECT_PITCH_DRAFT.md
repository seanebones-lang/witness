# Draft: NSF SBIR/STTR Project Pitch

**Status:** internal preparation draft. It has not been submitted to NSF. Do not submit this text until customer discovery, eligibility, technical-defensibility, and team-capability claims are supported by current evidence.

## Proposed title

**Auditable evidence infrastructure for AI-assisted computational research**

## 1. Technology innovation

Scientific teams increasingly rely on AI-assisted pipelines to generate hypotheses, select inputs, run computation, and summarize results. Existing lab notebooks, workflow orchestrators, model logs, and data stores can preserve files and execution metadata, but they do not consistently enforce an inspectable distinction between: (a) source-backed observations, (b) inferences derived from stated premises, and (c) generated suggestions or prose.

NextEleven proposes R&D on a provenance-native evidence layer for computational research. The technical hypothesis is that a typed, append-only record graph can make every consequential claim resolvable to: its exact inputs and transformations; a declared epistemic class; the premises that support an inference; a falsifier or update condition; and a signed producer record with visible limits.

The novel technical problem is not storing immutable records. It is building a practical evidence envelope and verification layer that resists category collapse across heterogeneous producers while supporting correction, supersession, key compromise, partial source availability, and reproducible replay. The initial proof is a local Grid-to-Witness docking demonstration; the proposed Phase I work would test feasibility on an independently specified computational workflow and a second producer path.

## 2. Technical objectives and challenges

**Objective 1: define and test a versioned evidence envelope.** Create a language-neutral schema for producer identity, source references, measurements, transformations, inference premises, generated artifacts, uncertainty, falsifiers, and correction/supersession links. Test it with conformance fixtures and invalid-input cases.

**Objective 2: build a verifier that rejects misleading states.** Develop validation rules that reject missing inference premises, invalid type transitions, malformed records, unauthorized update attempts, and claims that imply stronger verification than the evidence supports. Test tampering, replay, and incomplete-source scenarios.

**Objective 3: demonstrate independently inspectable replay.** With an external domain collaborator, package a pre-specified computational workflow so a second reviewer can reproduce or document failure to reproduce the input-to-record path. Compare the result against current approaches used by the collaborator.

**Core technical risks:** the schema may be too rigid across domain workflows; evidence envelopes may impose unacceptable user burden; public-key identity and correction may be difficult to make understandable; and a second reviewer may not reproduce the workflow. Phase I is intended to determine these facts, not assume them away.

## 3. Market opportunity

Initial customers are organizations that run or sponsor AI-assisted computational research and need to defend how a reported conclusion relates to its inputs, code, transformations, model output, and review history. Candidate customer segments include computational research groups, scientific software platforms, research-data infrastructure providers, and regulated or high-consequence R&D teams.

The customer value proposition is an inspectable evidence record that can reduce the cost of reproducing a result, investigating a discrepancy, documenting a correction, and separating generated material from source-backed claims. The first commercial offering would be integration, deployment, conformance testing, and hosted verification around an open protocol. This draft does not claim validated customer demand, adoption, a pricing model, or a competitive advantage. Those require customer discovery and an external technical assessment before submission.

## 4. Company and team

NextEleven LLC is the proposed U.S. small-business applicant. Sean McDonnell initiated Witness and Humanity Grid, built the existing proof package and public reviewer path, and will lead the engineering and product-definition work.

The present team has material gaps: domain-science leadership, research-software commercialization evidence, formal security review, and an independently demonstrated customer need. The Phase I plan requires named collaborators or advisers for those gaps. Before an SBIR Project Pitch is submitted, the company should document at least three customer-discovery conversations, one domain collaborator or letter of interest, a technical comparison against current tools, and a credible plan for external security and scientific review.

## Submission gate

Submit only after all are true:

- A customer segment and high-value use case are defined in the customer’s own terms.
- At least three discovery conversations support an unmet need without being represented as commitments.
- The technical innovation can be stated as a risky, unproven R&D hypothesis rather than routine software integration.
- A technical-defensibility strategy exists despite the open protocol.
- NextEleven’s eligibility, ownership, and project-lead role have been verified.
- The research plan and team gaps are credible to an outside evaluator.
