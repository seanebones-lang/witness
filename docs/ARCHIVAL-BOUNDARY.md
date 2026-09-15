# Archival preservation and custody boundary

**Status:** Curated package demonstration; archival repository controls incomplete
**Basis:** Owner-supplied archives and libraries review dated 2026-09-15
**Decision scope:** Stored artifacts, fixity, custody, packages, rights, retention, and deaccession

Custody is a sequence of events against stored objects. A URI plus a digest of
Witness JSON is not a chain of custody, and a content-addressed database row is
not proof of preservation.

## Current demonstration

The USGS river-gauge snapshot now contains:

- exact response bytes retained as the preservation master;
- normalized response headers as retrieval metadata;
- an extraction derivative that does not overwrite the master;
- an inventory manifest with byte lengths and SHA-256 digests;
- object roles and representation information;
- a designated community for the demonstration;
- explicit rights, custody-transfer, source-signature, and endorsement limits;
- a recorded intake event; and
- a later local fixity-check event covering four package artifacts.

Witness maintains this curated copy for protocol evaluation. It is not USGS's
archival institution or record of record.

## Conformance boundary

The package is informed by archival review but does not claim conformance to
OAIS, PREMIS, BagIt, WARC, ISO 16363, or any trusted digital repository standard
or certification. It has no independent custody, replicated preservation
storage, source-institution retention schedule, recurring automated fixity,
format migration plan, deaccession workflow, or external event log.

## Required event and policy model

Future archival profiles must represent intake, fixity check, fixity failure,
relocation, migration, restriction, closure, redaction derivative, deaccession,
transfer, and destruction as explicit events. They must distinguish producer,
custodian, rights holder, operator, preservation master, access derivative, and
extraction derivative.

Appraisal must permit non-ingest. Retention and access must follow recorded
authority and policy. A failed checksum must create a failure event; it must not
be repaired by silently rehashing altered bytes. Deaccession must describe what
happened without preserving the removed payload inside a diff.

Until those controls and independent archival review exist, public material may
describe the USGS bundle as a curated source-preservation demonstration, not an
archival deposit, trusted repository, or preserved institutional record.
