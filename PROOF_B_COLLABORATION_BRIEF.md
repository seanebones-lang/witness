# Proof B collaboration brief

## Purpose

Proof B should test whether a domain-defined computational experiment can be made easier to inspect, challenge, and reproduce when its evidence, inference, generated proposals, failures, and corrections are kept in a visible record.

It is **not** a drug-discovery, medical, diagnostic, regulatory, or efficacy claim. A negative, inconclusive, or failed result is an acceptable outcome.

## Collaboration sought

We seek one independent domain collaborator and one reproducibility or research software reviewer. The domain collaborator helps define the question and acceptance criteria before work begins. The reviewer helps assess whether the package can be reproduced and inspected independently.

## Minimum design requirements

1. Write the research question, dataset/source selection, method, success criteria, and limits before execution.
2. Name a domain expert who can object to the experimental design.
3. Use inputs whose provenance, access conditions, and reuse terms are recorded.
4. Preserve exact software versions, parameter files, environment assumptions, and raw outputs.
5. Run at least one independently performed replication or clearly state why it is not feasible.
6. Record Observed, Inferred, and Generated material separately in Witness.
7. Record failed attempts, deviations, corrections, and unresolved objections.
8. Publish the review package and response whether the result passes, fails, or is inconclusive.

## Candidate first shape

A narrowly scoped retrospective benchmark in computational chemistry may be a reasonable starting point: a collaborator selects a documented target and benchmark dataset; evaluation metrics and replication rules are pre-specified; the output is compared to a defined reference; and the package states exactly what the computation did and did not support.

The collaborator may propose another field if it offers a smaller, better specified, legally reusable, and independently reproducible first experiment.

## Deliverables

- A dated protocol and explicit success/failure criteria.
- A source and rights inventory.
- A reproducible execution package.
- Witness records with premise links and falsifiers.
- At least one independent reproduction or documented reproduction attempt.
- A public review, project response, and unresolved-findings list.

## First conversation

Interested collaborators should first review [REVIEW_REQUEST.md](REVIEW_REQUEST.md) and respond with their discipline, relevant expertise, conflicts of interest, the smallest defensible experiment they would consider, and conditions required before associating their name with the work.
