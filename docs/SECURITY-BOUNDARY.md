# Security and trust-display boundary

**Status:** Immediate local safeguards implemented; security architecture incomplete
**Basis:** Owner-supplied security review dated 2026-09-15
**Decision scope:** Network exposure, verification meaning, storage integrity, keys, interface trust cues, and releases

Witness currently hashes and can sign its own records. It cannot establish that
the source produced the referenced bytes, bind a key to a real identity, detect
every database rewrite, revoke a compromised key, or provide independent
verification through a second implementation.

## Current refusals

- The unauthenticated API binds to loopback by default. A non-loopback override
  produces a warning and remains unsupported.
- The dashboard describes signature data as present or absent. It does not
  claim to verify signatures on read.
- The dashboard states that a valid signature would establish control of a
  public key for signed bytes, not identity, authority, accuracy, custody, or
  truth.
- Parent count is labeled as declared relationships, not corroboration.
- The database is described as mutable local SQLite storage. Application-level
  insertion rules do not make the file cryptographically append-only.
- Current CIDs are implementation-specific hashes of Witness payload JSON.
  They are not portable until a canonical preimage and test vectors exist, and
  they do not bind retrieved source bytes.

## Required security objects and evidence

1. Distinct artifact and record digest contracts.
2. A standardized canonical signature preimage with cross-language vectors.
3. Key creation, binding, rotation, revocation, expiry, recovery, and compromise records.
4. Verification-on-read with explicit invalid or quarantined states.
5. Signed checkpoints stored outside the SQLite file and fork-detection tests.
6. An independent verifier that does not share the writer or dashboard code.
7. Authentication, authorization, quotas, resource limits, and transport
   security before any supported network deployment.
8. Release provenance, pinned automation, signed artifacts, and a reproducible
   build recipe before a downloaded binary is presented as a trusted verifier.
9. Adversarial interface tests for inert attacker text, non-color categories,
   adjacent trust limits, and visually subordinate generated material.

Until these exist, Witness is a local design and evaluation prototype. It must
not be represented as an immutable ledger, identity system, source receipt,
independent verifier, or production evidence authority.
