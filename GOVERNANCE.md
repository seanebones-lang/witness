# Governance

Witness is currently a founder-maintained public prototype. Sean McDonnell is the
initial maintainer and final decision maker during this stage. This concentration
of authority is a disclosed limitation, not the intended permanent structure.

## Decision principles

Decisions are evaluated against the [manifesto](MANIFESTO.md), the
[epistemic constitution](docs/EPISTEMIC-CONSTITUTION.md), demonstrated user harm,
interoperability, reversibility, and testable guarantees. Popularity, funding, or
institutional status does not override those constraints.

## Changes and decisions

- Focused implementation fixes use ordinary pull requests.
- Schema, cryptography, protocol, privacy, moderation, ranking, and trust-model
  changes begin with a public issue and a written decision record.
- Constitutional amendments follow the process in the constitution.
- Maintainers disclose financial, institutional, personal, or professional
  conflicts that a reasonable reviewer would consider material.
- A reviewer may publish a minority report alongside an accepted decision.
- A material domain, privacy, or security objection that is refused receives a
  decision record linking the objection, rejected alternative, constitutional
  basis, date, and any dissent. Closure is not erasure.
- Security reports use the private process in [SECURITY.md](SECURITY.md).

Decision records should state the problem, affected people, evidence, selected
option, rejected alternatives, limits, compatibility effects, dissent, review
date, and whether the work remains unreviewed. Decisions may be revisited by new
evidence without deleting the old record. They live in
[docs/decisions](docs/decisions/README.md); minority reports live in
[docs/minority-reports](docs/minority-reports/README.md).

## Maintainer authority

Maintainers may merge changes, moderate project spaces, coordinate releases, and
protect confidential security reports. They may not silently alter published Git
history, conceal material protocol departures, represent fixtures as verified
evidence, or claim institutional endorsement without evidence. This stage has
one decision maker, not a review board; requested reviews do not bind the
maintainer and absent domain review is recorded as unreviewed, not approval.

Emergency action must be necessary to protect people, credentials, evidence, or
infrastructure. The public record must describe the action and scope as soon as
safe, and temporary authority expires within 30 days unless renewed through the
normal process.

## Participation and conduct

Participation is governed by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). People
affected by a domain deployment should have a route to review the rules applied
to their records even when they do not write code. The public route is the
[affected-person concern form](.github/ISSUE_TEMPLATE/affected-person-concern.yml);
the private route is the maintainer profile for material unsafe to publish.
Domain expertise does not
replace community consent, and community participation does not replace technical
or scientific validation.

## Funding and infrastructure disclosure

The repository is currently hosted on GitHub and maintained without a published
institutional sponsor or governing foundation. Material sponsorship, grants,
employment relationships, hosted services, moderation vendors, or infrastructure
control that could influence the project will be disclosed here or in linked
decision records.

## Path toward shared stewardship

Before production or high-consequence deployment, Witness should establish a
multi-stakeholder governing body with technical, archival, privacy, security,
domain, and affected-community representation; defined terms; removal and appeal
procedures; public minutes; conflict rules; and a process that permits compatible
forks. Formation of that body is a release gate, not a present claim.
