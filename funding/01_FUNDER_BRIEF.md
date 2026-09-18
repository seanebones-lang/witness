# Funding partner brief: an inspectable record for AI-assisted science

## The problem

AI-assisted research can produce hypotheses, calculations, summaries, and confident conclusions faster than researchers can inspect their basis. The practical failure is not that models generate language. It is that measurements, derived claims, and generated suggestions can become indistinguishable in the record.

A research team needs to be able to ask: **what was observed, what was inferred from it, what was generated, and what would change the claim?**

## What has been built

Witness and Humanity Grid form an early, open-source demonstration of that distinction.

- **Witness** is an append-only record system that stores Observed, Inferred, and Generated material as distinct types, with explicit premise links and falsifiers.
- **Humanity Grid** is the first local computational producer and reproducibility demonstration.
- **EXP-001 Proof A** is frozen in the annotated `proof-a-v1` tag in both repositories.

The proof package contains two specified local AutoDock Vina score vectors for a CDK2 example, an agreement calculation with one recorded outlier, and a fresh replay that writes 12 Observed records, two Inferred records, and one Generated hypothesis into Witness. The package is checksummed and locally verifiable.

Proof A demonstrates the infrastructure and record mechanics. It does **not** demonstrate a drug candidate, biological activity, docking accuracy, independent execution, external validation, or scientific discovery.

## The funded next step

Fund a six-month **Proof B**: an independently designed, pre-specified computational-research experiment with a domain collaborator and an independent reproducibility review.

The work will produce:

1. A domain-defined question, method, source-rights inventory, and acceptance criteria written before execution.
2. A reproducible execution package with preserved inputs, tool versions, parameters, and raw outputs.
3. A Witness record that keeps observations, inferences, generated material, failures, corrections, and unresolved objections distinct.
4. At least one independent reproduction attempt.
5. A public review and project response whether the result passes, fails, or is inconclusive.

## Why this matters

The project is not trying to create an oracle. It is trying to make the chain between a measurement and a conclusion harder to erase, rewrite, or confuse with generated text.

If Proof B succeeds, the result is not “AI discovered something.” The result is a tested way for computational research to remain inspectable when AI participates in proposing, running, or explaining work.

## The ask

We seek a funding partner for a **$100,000, six-month Proof B program**, or an equivalent combination of direct support, domain-collaborator time, compute, and independent review.

We also seek one domain collaborator who will help define the experiment before execution and retain the freedom to criticize the result publicly.

## Public materials

- Reviewer entry point: <https://github.com/seanebones-lang/witness/blob/main/REVIEW_REQUEST.md>
- Frozen computational package: <https://github.com/seanebones-lang/Humanity-Grid/blob/main/docs/PROOF_A_REVIEW.md>
- Proof B requirements: <https://github.com/seanebones-lang/witness/blob/main/PROOF_B_COLLABORATION_BRIEF.md>
- Repositories: <https://github.com/seanebones-lang/witness> and <https://github.com/seanebones-lang/Humanity-Grid>

**Builder:** Sean McDonnell, Founder, NextEleven LLC, DFW, Texas.
