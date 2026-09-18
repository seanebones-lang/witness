# Proof A — Infrastructure & Reproducibility (Frozen)

## Frozen evidence and runtime commits
- **Proof A artifact baseline (Witness)**: `2052dc2`
- **Reconciled Witness runtime**: `7a1d372`
- **Humanity Grid**: `3f863c9` (tagged `proof-a-v1`)

The local `proof-a-v1` tag is moved only after the locked build, tests, lint,
format check, source-bundle verifier, and EXP-001 route smoke test pass. It
identifies this complete Witness proof state; the artifact baseline remains
separately named so the displayed experiment can be traced to its original
frozen files.

## How to Start Witness
```bash
cd ~/witness
DATABASE_URL=sqlite://data/witness.db ./target/release/witness-api
```
Server starts at `http://127.0.0.1:8080`

## EXP-001 Dashboard
**URL**: `http://127.0.0.1:8080/experiments/EXP-001`

## Witness Records (EXP-001)
- **12 Observed** — 6 CDK2 compounds × 2 replicates (seeds 101, 202), real docking scores
- **2 Inferred** — Consensus (Pearson r = 0.885, threshold ≥ 0.80, PASS, outlier CHEMBL495686 |Δ| = 0.570) + Failed attempt provenance
- **1 Generated** — Research Scout hypothesis, `human_reviewed: false`

## Tests
**9/9 PASS** (Witness core library)
