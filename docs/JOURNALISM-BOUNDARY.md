# Journalism and public-records boundary

**Status:** Interim protective boundary; newsroom profile not implemented
**Basis:** Owner-supplied journalism review dated 2026-09-15
**Decision scope:** Journalism, public records, corrections, and confidential sources

Witness is not a newsroom system of record. During the prototype stage, its
only appropriate journalism-related use is as an experimental sidecar for
artifacts and corrections that are already public.

## Prohibited material

Do not ingest:

- reporter notebooks or unpublished reporting files;
- anonymous-source identities or source maps;
- not-for-attribution or off-record statements;
- confidential premise identifiers or edges;
- unpublished drafts containing person-concerning material; or
- generated restatements of any prohibited material.

The broader person-concerning ingest prohibition remains controlling.

## Required distinctions before the boundary can expand

| Act or object | Required distinction |
| --- | --- |
| Public artifact | Bytes produced by a source, separate from claims about their meaning |
| Retrieval | Who obtained which bytes, from where, when, and with what fixity |
| Extraction | Who read or parsed which part and copied which value |
| Direct observation | What a named person directly witnessed, with method and limits |
| Attributed statement | On-record, background, or restricted attribution semantics |
| Editorial inference | Premises, baselines, definitions, framing choices, and falsifiers |
| Generated assistance | Never treated as a source; visibly and programmatically distinct |
| Correction or note | A role-bound speech act with reason and target |
| Retraction | Withdrawal of authority by the appropriate publisher or source role |
| Contradiction or dispute | Coexisting assertions; neither automatically becomes current truth |
| Sealing | Payload unavailability for safety, separate from factual correction |

## Authority and current state

Holding a valid signing key does not grant authority over every record. The
protocol must specify which role may correct an extraction, revise a source
artifact, correct a published story, retract institutional material, dispute an
inference, or seal harmful content.

Consumers must be able to query `current-for-use` separately from
`history-for-accountability`. Default API and interface behavior must not return
superseded or retracted headlines as if they were current simply because they
remain in append order.

## Corroboration boundary

More parents do not establish independent corroboration. Multiple records may
repeat one press release, wire report, anonymous speaker, or copied dataset.
Witness must expose common origin and claimed independence without producing a
corroboration or truth score.

## Evidence required for a journalism profile

1. Role and object definitions reviewed by newsroom and public-records practitioners.
2. Role-bound correction, revision, retraction, contradiction, dispute, and sealing semantics.
3. Safe representation of a confidential premise that intentionally never enters the graph.
4. API tests proving safe `current-for-use` defaults and explicit historical access.
5. Interface tests showing generated assistance is distinct without relying on color.
6. A public-artifact correction fixture with competing authorities and no private source material.
7. Independent review from journalists and people affected by publication decisions.

Until those conditions are met, Witness must not claim newsroom readiness,
adoption, endorsement, or fitness for confidential reporting.
