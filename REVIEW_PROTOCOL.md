# Proof A independent-review protocol

This is an inspection protocol, not a request for endorsement or a claim of scientific validation.

## 1. Identify the exact state

Record the commit and tag you reviewed:

```bash
git -C witness show --no-patch --format=fuller proof-a-v1
git -C Humanity-Grid show --no-patch --format=fuller proof-a-v1
```

The tags identify the frozen proof state. Record a later `main` commit separately if you inspect current documentation or implementation.

## 2. Verify the frozen computational package

From a Humanity Grid checkout:

```bash
python3 proofs/EXP-001-v1/tools/verify.py
```

Expected results include 43 manifest-matching artifacts, the fixed consensus values, one recorded outlier, and a 12 Observed / 2 Inferred / 1 Generated Witness export. Read the verifier and package `LICENSES.md`; this is a check of the preserved package, not a re-execution or a scientific conclusion.

## 3. Replay the current record-contract path

Follow the exact two-terminal instructions in [Humanity Grid’s Proof A guide](https://github.com/seanebones-lang/Humanity-Grid/blob/main/docs/PROOF_A_REVIEW.md).

Check that a fresh local Witness database receives the stated record counts, that consensus links the twelve observations, and that a second submission is idempotent. Note your operating system, Python/Rust versions, commands, output, and any deviation.

## 4. Inspect the boundaries

Review these questions directly:

- Does each claimed observation name its local source, method, time, and limits?
- Does each inference identify its premises and a possible falsifier?
- Is the generated hypothesis visibly separate from evidence?
- Are hashes and append-only storage presented as fixity mechanisms rather than as proof of truth, identity, custody, or independent review?
- Are source and data rights distinguished from the project source-code license?

## 5. Publish a finding

Use [REVIEW_TEMPLATE.md](REVIEW_TEMPLATE.md). Include enough detail for another person to reproduce an error. Mark a finding as **confirmed**, **unconfirmed**, or **opinion/proposal**. If publishing would expose a security vulnerability, follow [SECURITY.md](SECURITY.md) instead of a public issue.

## Interpretation rule

Passing this protocol only supports the exact checks it reports. It cannot turn an unreviewed demonstration into independent science. A failed replay or a critical review is a useful result and must remain visible.
