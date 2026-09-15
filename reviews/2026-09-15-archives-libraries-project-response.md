# Project response — archives and libraries review

**Date:** 2026-09-15
**Review:** [Archives and libraries](2026-09-15-archives-libraries-review-unverified.md)
**Decision status:** Central objection accepted; reference package extended
**Review provenance:** Owner-supplied, reviewer identity and independence not established

## Decision

The project accepts the review's central objection:

> Custody is events against stored bytes; a URI plus a hash of Witness JSON is
> not a chain of custody.

Witness will distinguish artifacts from wrappers, preservation masters from
derivatives, producers from custodians, and preservation events from descriptive
fields. It will not claim that application-level append-only insertion or a CID
turns SQLite into an archive.

## Immediate implementation

1. Add an explicit package profile to the USGS demonstration.
2. Identify the exact response as preservation master and the extraction as a derivative.
3. Record representation information for the master, headers, and extraction.
4. Record the designated community and explicit custody, rights, source-signature, and endorsement limits.
5. Record the original intake event.
6. Add a separate local fixity-check event covering the existing artifacts.
7. Add a reusable tool for recording future local fixity checks and bind its
   event file into the package manifest.
8. State that Witness is not USGS's archival institution or record of record.

## Accepted requirements

- Use PREMIS and OAIS concepts for objects, events, agents, rights, roles, and designated community.
- Evaluate BagIt for portable multi-file packages and WARC for HTTP retrievals.
- Record repeated fixity checks and failures without silently replacing digests.
- Represent intake, migration, restriction, closure, deaccession, transfer, and destruction as events.
- Preserve original representations beside derivatives.
- Define appraisal, non-ingest, retention schedules, access restrictions, and lawful deaccession.
- Add replicated storage, external event evidence, restoration testing, and format migration planning.

## Limits

The extended USGS bundle is a curated demonstration, not an OAIS information
package, PREMIS implementation, BagIt bag, WARC record, ISO 16363 assessment,
trusted digital repository, archival deposit, or institutional record. The
fixity events are maintained in the same Git repository and do not establish
independent custody or immutable preservation history.
