# Contributing to Witness

Thank you for considering a contribution. Witness is trying to preserve the
difference between evidence, interpretation, and generated material. That work
needs technical skill, domain knowledge, skepticism, and lived experience.

The project is at an early stage. Clear questions, failure cases, documentation,
and careful critiques are valuable contributions.

## Who can contribute

You do not need to be a Rust developer. Useful contributors include:

- Scientists and researchers who work with measurement and uncertainty.
- Journalists, archivists, librarians, and investigators who handle sources and
  corrections.
- Human-rights, privacy, safety, and community practitioners who can identify
  risks created by permanent or public records.
- Cryptographers and security engineers who can test the integrity model.
- Data engineers and standards authors who can improve interoperability.
- Designers, technical writers, translators, and accessibility specialists who
  can make provenance understandable.
- Software engineers who can improve Rust, SQLite, APIs, testing, operations,
  and the dashboard.
- Anyone who can describe a real case where the categories or assumptions fail.

## Before contributing

Read:

1. [MANIFESTO.md](MANIFESTO.md), which states why Witness exists.
2. [README.md](README.md), especially the guarantees and limitations.
3. [ROADMAP.md](ROADMAP.md), especially the current phase and release gates.
4. Existing issues and pull requests to avoid duplicating active work.

This repository does not yet contain a license. Public visibility does not by
itself grant permission to copy, distribute, modify, or deploy the code. Please
open an issue about licensing before submitting a substantial code contribution.

## Ways to help now

Good early contributions include:

- Describe an adversarial or harmful use case.
- Find a place where the interface overstates a guarantee.
- Add a focused test for an invariant or previously observed failure.
- Improve error handling without changing public behavior.
- Review terminology from a specific domain or community.
- Improve setup documentation and reproducible examples.
- Audit keyboard navigation, screen-reader behavior, color contrast, or plain
  language.
- Propose a portable test fixture for canonicalization and verification.
- Identify privacy, consent, custody, or correction requirements missing from
  the data model.

Do not add real personal data, confidential records, private keys, or sensitive
evidence to an issue, test, fixture, commit, or pull request.

## Propose the problem first

Open an issue before a large feature, schema change, cryptographic change, new
dependency, or architectural rewrite. A strong proposal includes:

- **Problem:** What can fail today?
- **Affected people:** Who benefits, bears risk, or must be consulted?
- **Example:** A small concrete scenario or record.
- **Proposed guarantee:** What should a user be able to rely on afterward?
- **Boundary:** What will still not be guaranteed?
- **Alternatives:** Other approaches considered.
- **Verification:** Tests, fixtures, or review needed to demonstrate the result.
- **Compatibility:** Effects on existing databases, APIs, signatures, or tools.

For domain-specific work, include a reviewer with relevant experience whenever
possible. Code review alone cannot validate a medical, legal, scientific,
journalistic, archival, or human-rights workflow.

## Development setup

Install a current stable Rust toolchain, clone the repository, and run:

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

Run the API from the repository root so it can find the dashboard template:

```bash
touch witness.db
cargo run --package witness-api
```

The default dashboard is available at <http://localhost:8080/>. The GraphQL
explorer is available at <http://localhost:8080/graphql>.

## Make a focused change

- Keep each pull request centered on one problem.
- Preserve unrelated work and generated SQLx metadata.
- Add tests when behavior, integrity guarantees, parsing, persistence, or public
  contracts change.
- Use fixtures created for testing. Do not describe fixture data as independently
  verified real-world evidence.
- Avoid `unwrap()` and `expect()` on data that can originate outside the process.
- Return explicit errors for malformed, missing, unverifiable, or unsupported
  records.
- Treat schema and canonicalization changes as compatibility changes.
- Document the meaning and limit of any new trust signal.

Format and verify the workspace before requesting review:

```bash
cargo fmt --all
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

## Tests that matter

Prefer tests that demonstrate an externally meaningful promise, such as:

- A modified signed record fails verification.
- The same canonical record produces the same identifier in two implementations.
- A missing parent prevents an inference from being accepted.
- A correction preserves the earlier record and links to it.
- An invalid trust state remains visible through storage, API, and interface.
- A migration preserves existing records and verification results.
- An unauthorized actor cannot read or change a restricted record.

Tests that merely repeat the implementation without protecting a behavior are
less useful.

## Pull request description

Explain the result for a reviewer who has not seen prior discussion:

```text
Problem
Describe the concrete failure or missing behavior.

Result
Describe what users or verifiers can do after this change.

Guarantee and boundary
State what this establishes and what it still cannot establish.

Validation
List the meaningful tests and manual checks performed.

Compatibility and risk
Describe database, API, signature, privacy, or operational effects.
```

## Review expectations

Reviewers should ask:

- Does the change preserve the distinction among observed, inferred, and
  generated?
- Could the interface make a weak signal appear stronger than it is?
- Can malformed or adversarial input cause silent acceptance, corruption, or a
  crash?
- Does the change preserve history and make correction visible?
- Does it expose people or sensitive information to new risk?
- Is the behavior testable and documented?
- Can another implementation reproduce the result?

Maintainers may ask for security, privacy, accessibility, domain, or community
review in addition to code review.

## Communication

Assume good intent, but challenge claims with evidence. Separate critique of an
idea from judgment of a person. State uncertainty. Correct mistakes visibly.
Make room for people who are affected by a system even when they do not write
its code.

Formal conduct and enforcement rules will be added before the project advances
beyond prototype status. Until then, harassment, threats, exposure of private
information, and discriminatory behavior are not acceptable in project spaces.

## Security reports

Do not publish private keys, exploitable vulnerabilities, personal data, or
sensitive evidence in a public issue. Until `SECURITY.md` establishes a dedicated
private channel, use the repository owner's private contact option on GitHub and
share only the minimum information needed to establish contact.

## A note on ambition

Witness has a broad humanitarian intention. Contributions should convert that
intention into narrow, verifiable guarantees. The project earns trust through
transparent limits, careful tests, accountable governance, and willingness to
record where it was wrong.
