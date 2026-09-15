# Project response — security and trust-boundaries review

**Date:** 2026-09-15
**Review:** [Security and trust boundaries](2026-09-15-security-trust-boundaries-review-unverified.md)
**Decision status:** Central objection accepted; immediate local safeguards implemented
**Review provenance:** Owner-supplied, reviewer identity and independence not established

## Decision

The project accepts the review's central objection:

> Valid signatures and matching CIDs currently verify Witness's own JSON, not
> the world, and the interface can cause readers to grant those signals more
> authority than they carry.

The current system is a local prototype. Its cryptographic features are useful
for bounded tamper evidence but do not yet defend record meaning against the
operator, importer, database attacker, compromised key, copied source, or
misleading screenshot.

## Immediate implementation

1. Change the default API bind address from all interfaces to `127.0.0.1`.
2. Retain a `HOST` override for controlled development, with an explicit warning
   on every non-loopback bind.
3. Remove the green signature cue from dashboard cards.
4. Label signatures as data present or absent and state that the dashboard does
   not verify them on read.
5. Place the identity, authority, accuracy, custody, and truth limits adjacent
   to signature information.
6. State beside parent counts that declared parents do not establish
   independence or corroboration.
7. Document that current record CIDs cover implementation-specific Witness JSON,
   not retrieved source bytes.

## Accepted requirements

- Separate artifact and record digest contracts.
- Select and publish a canonical hash-and-sign preimage with compatibility vectors.
- Add complete key lifecycle and compromise records.
- Verify and quarantine invalid records on every read path.
- Publish signed checkpoints outside the mutable database.
- Build a verifier that does not share the writer or dashboard implementation.
- Require authentication, authorization, resource controls, and transport
  security before supporting network deployment.
- Add release-integrity evidence and adversarial interface testing.
- Treat generated-to-observed relabeling at ingestion as a critical integrity threat.

## Limits

This response is not a penetration test, security certification, independent
audit, production-readiness finding, or claim that the implemented changes
resolve the wider threat model. The submitted review did not test running code
or disclose a specific exploitable vulnerability.
