# Expert Review Guide

Witness is looking for criticism from people who understand measurement,
archives, journalism, security, privacy, standards, accessibility, human rights,
community governance, and open-source infrastructure.

This is an early open-source prototype. It is not a production evidence
authority, and a request for review is not a request for endorsement.

## The question

Can a shared protocol preserve the difference among:

1. **Observed:** something measured or recorded with inspectable provenance;
2. **Inferred:** a claim derived from identified premises through an exposed
   method; and
3. **Generated:** synthetic material that may be useful but is not evidence
   merely because it is convincing?

Witness tries to keep those categories visible while preserving uncertainty,
correction, disagreement, and the limits of cryptographic verification.

## A 20-minute review path

1. Read the [manifesto](../MANIFESTO.md) — 5 minutes.
2. Read the guarantees and limits in the [README](../README.md#what-the-guarantees-mean)
   — 4 minutes.
3. Read the [Epistemic Constitution](EPISTEMIC-CONSTITUTION.md) — 6 minutes.
4. Choose one question below and respond — 5 minutes.

For a deeper technical review, continue with the
[verification model](VERIFICATION-MODEL.md), [threat model](THREAT-MODEL.md),
[roadmap](../ROADMAP.md), and source code.

## Questions where criticism would help

You do not need to answer every question.

1. Which real observation in your domain does the current three-category model
   represent badly?
2. Where could Witness create false confidence despite its warnings?
3. What must sometimes be corrected, sealed, redacted, or deleted rather than
   preserved indefinitely?
4. What does a responsible chain of custody require in your work?
5. Which existing standard, vocabulary, or project should Witness adopt or join
   instead of recreating?
6. How should testimony and community knowledge differ from instrument readings
   without treating either as automatically superior?
7. What evidence would demonstrate that an inference is reproducible?
8. Who could be endangered, excluded, or overruled by this system?
9. What would make the interface accessible and comprehensible without reducing
   verification to a misleading score?
10. What small public, reversible pilot would be worth running?

## Review lanes

| Experience | A useful first review |
| --- | --- |
| Measurement or science | Units, calibration, uncertainty, missingness, and replication |
| Archives or libraries | Original artifacts, custody, fixity, retention, correction, and deletion |
| Journalism | Sources, corroboration, anonymous material, corrections, and editorial inference |
| Security | Attacker stories, key lifecycle, storage integrity, rendering, and release boundaries |
| Privacy or human rights | Consent, immutable harm, access, redaction, sealing, and coercive use |
| Standards | W3C PROV, JSON-LD, C2PA, evidence vocabularies, canonicalization, and interoperability |
| Accessibility or design | Category comprehension, non-color signals, keyboard use, and screen readers |
| AI evaluation | Cases where a model invents derivations, omits uncertainty, or flattens categories |
| Community governance | Appeals, minority reports, institutional power, representation, and forks |
| Engineering | Fallible parsing, conformance fixtures, corrections, verification, export, and replay |

## How to respond

- Open a [problem or proposal](https://github.com/seanebones-lang/witness/issues/new/choose)
  for public feedback.
- Comment on a matching scoped issue when one exists.
- Use [GitHub private vulnerability reporting](https://github.com/seanebones-lang/witness/security/advisories/new)
  for exploitable security details.
- Use a private method on the
  [maintainer profile](https://github.com/seanebones-lang) when feedback contains
  personal, confidential, or safety-sensitive information.

Please identify your perspective and the limits of your review. Pseudonymous
feedback is welcome. Do not submit personal data, confidential evidence,
credentials, private keys, or records that could endanger someone.

## What happens to feedback

Feedback may lead to a documented decision, roadmap item, test, evidence-profile
proposal, constitutional amendment, or minority report. Acceptance is not
automatic. The project will preserve material objections and explain decisions
without representing a reviewer as an endorser.

## Current review outcome

No independent expert review has been completed yet. The
[submitted-review register](../reviews/README.md) includes owner-supplied review
material whose authorship and independence have not been established. Its
objections may guide work, but it does not count toward the independent-review
gate.
