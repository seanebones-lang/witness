# Owner-supplied instrument-measurement review — 2026-09-15

- Provenance status: `owner-supplied-unverified`
- Supplied by: project owner through a private project session
- Author: not provided
- Independence: not established
- Permission to attribute: not established
- Review target: issue #2 and Constitution 0.1
- Project treatment: preserve as review evidence and actionable objections; do
  not count as independent review or endorsement

## Submitted text

Formatting-only trailing spaces were normalized on import; wording is preserved.

This is one complete expert review, filed against the project’s own twenty-minute path and against [issue #2](https://github.com/seanebones-lang/witness/issues/2). It answers question 1. It is not an endorsement.

---

## Witness expert review

**Issue:** #2 — Define the instrument-measurement evidence profile
**Question answered:** 1. Which real observation in your domain does the current three-category model represent badly?
**Constitution version reviewed:** 0.1.0 founding draft
**Code reviewed:** `core/src/types.rs` (`Measurement`, `MeasuredValue`, `Uncertainty`, `Location`, `InstrumentRef`, `Observation`, `ProvenanceNode`), README ingestion example, README guarantee table
**Date:** 2026-09-14

### Perspective

Measurement and research-data practice: how an instrument reading becomes a public number, what must travel with that number, and what later revision does to a record people already used.

### Limits of this review

Read the manifesto, constitution 0.1.0, verification and threat models at summary level, README guarantees, issue #2, and the observation types. Did not run the binary, audit the six tests, review the dashboard for injection, or claim hydrology licensure. Did not inspect a live USGS station. No production measurement is certified here.

### Domain example

A USGS instantaneous gage-height or discharge point that is later revised.

Not a laboratory curiosity. A common public record:

- requested from a documented water-data API
- tied to a site number and parameter code
- stamped with a timezone
- often carrying qualifier codes (estimated, ice-affected, and others)
- first published as provisional
- later approved, and sometimes numerically changed
- sometimes a daily statistic derived from many instantaneous points

The current model represents that badly. The README example makes the failure visible:

```text
observe --quantity air_temperature --value 18.4 --unit celsius
        --instrument-id station-001
```

That command can store a node labeled Observed. It cannot store the observation.

### What the three-category model gets wrong here

**1. The official number is often already inferred.**

A USGS daily mean discharge is computed from instantaneous values. If Witness ingests the daily product and labels it Observed because an institution published it, the protocol performs the collapse it exists to prevent. Category membership cannot be copied from publisher prestige. It has to follow the production process.

**2. `Measurement` is shaped like a single scalar, and this observation is not one.**

`types.rs` asserts:

- `quantity: String`
- `value.numeric: Option<f64>`
- `unit: Option<String>`
- one optional `Uncertainty { value, unit, confidence_level, method }`
- optional lat/lon
- one `measured_at`
- optional `calibration_ref` as a string

A real instrument point also has, at minimum: source vocabulary for the quantity, decimal value that is not IEEE-754, distinct times, sampling regime, quality flags, approval status, detection or censoring behavior, datum or coordinate reference, and the raw bytes from which the point was parsed.

`f64` is the wrong carrier for a measured quantity. `18.4` and `18.399999999999999` are not the same stored observation. Units as free text are not units. A missing uncertainty that becomes absent from the record is indistinguishable from “we looked and there was none.”

**3. One timestamp violates Article IX.**

Article IX is correct: event, observation, recording, ingestion, signing, publication, correction, and knowledge times are different. `ProvenanceNode.timestamp` plus `Measurement.measured_at` silently drops the rest. For a gauge, “when the water was at that height,” “when the logger stored it,” “when USGS published the provisional value,” “when Witness fetched it,” and “when the approved value replaced it” are five facts. Collapsing them is how a later correction looks like the original event moved.

**4. The raw artifact is missing, so Observed cannot be challenged.**

Article II says classification as observed records a claimed relationship to the world and does not establish accuracy. That only works if a later reader can see the source bytes. The present observation type stores a parsed payload and an optional `source_uri`. A URI is a location. It is not the document. Source publishers revise in place. After that, the CID of the Witness node no longer binds to what the institution returned.

**5. Qualifiers and approval are not decoration.**

An estimated, ice-affected, or provisional discharge is not a clean Observed leaf and not a Generated paragraph. It is a reported value with a documented defect or status. If those flags are dropped, the record is more complete-looking than the measurement. That is false confidence produced by the schema, not by the user.

**6. A later approved value is a supersession, not a better observation of the same instant.**

People already acted on the provisional number. Article V requires a linked record that does not erase the earlier one. The current types have no approval state, no supersession edge with a reason, and no way to say “the event time is the same; the publication status changed.” The graph will either overwrite the meaning or mint a second Observed point that looks independent.

### Constitution objections to preserve

- **Article I** puts measurement, artifact, and testimony in one Observed bucket. For instrument work that is too coarse. A raw HTTP body, a parsed quantity-value, and a witness statement are different objects. Keep the three top-level categories if you must, but do not let Observed mean “not generated.”
- **Article II** is right that missing provenance must stay explicit. The implementation still allows a complete-looking Observed node with empty method, empty calibration, empty flags, and no artifact digest.
- **Article VIII** is unimplemented for the only transformation the pilot will perform: bytes-in-from-source to `Measurement`. That parse is an inference-shaped act and must be a recorded transformation, or the extracted number is Generated-from-schema.
- **Article IX** is the article the code most clearly fails.
- **Article VII** lists calibration and replication as verification dimensions. They cannot be dimensions if they are optional strings the producer may omit while the UI still says Observed.

### Existing work to profile, not recreate

Do not invent another unit system, another uncertainty object, or another sensor schema.

- **ISO 19156** Observations & Measurements: distinguish observation, result, procedure, feature of interest.
- **OGC SensorThings / WaterML 2:** gauges already have a public model for time series, parameter, and site.
- **UCUM or QUDT:** units as identifiers, not `"celsius"`.
- **GUM:** uncertainty is not one float plus `"gaussian"`.
- **USGS parameter codes and qualifier vocabularies:** reuse them in the river-gauge profile; do not translate them into friendlier English until a Generated record does that on purpose.
- **W3C PROV:** the parse from artifact to point is a `Derivation`.
- **PREMIS fixity:** digest of the retrieved object, not only digest of the Witness JSON.

Issue #4 should constrain issue #2. If the instrument profile cannot be expressed as a profile of O&M plus PROV plus a fixity record, the profile is still a parallel universe.

### Requested result for issue #2

A versioned profile: `witness-profile-instrument-measurement/0.1`.

It does not certify that a measurement is true. It states what the record is claiming, what it checked, what is missing, and what is inapplicable.

#### Required fields

Every conforming record must either populate the field or set an explicit reason: `missing:not-sought`, `missing:not-published-by-source`, `missing:withheld`, `inapplicable`. Silence is invalid.

| Field | Asserts | Must not invent |
| --- | --- | --- |
| `raw_artifact_cid` | Digest of retrieved bytes | A URI in place of bytes |
| `retrieval` | Time, request, media type, client | That the source still serves the same bytes |
| `quantity_id` | Vocabulary + code (e.g. USGS `00065`) | A display name as the identifier |
| `value_decimal` | Exact decimal text as published | `f64` as the stored value |
| `unit_id` | UCUM/QUDT or source unit code | Guessing unit from quantity name |
| `time_event` | When the measurand is claimed to hold | Ingestion time |
| `time_recorded` | When the instrument or logger stored it | Event time |
| `time_published` | When the source made this version public | Event time |
| `time_ingested` | When Witness stored the artifact | Any of the above |
| `timezone` | Offset or named zone used by the source | UTC-by-default without saying so |
| `sampling` | `instantaneous` / `interval` / `integrated` / `unknown` | Calling a daily mean instantaneous |
| `instrument_id` | Source station or sensor identifier | A local nickname only |
| `quality_flags` | Source flags, including empty list if source said none | Dropping flags to look clean |
| `approval_status` | `provisional` / `approved` / `restricted` / `unknown` | Treating published as approved |
| `value_status` | `reported` / `estimated` / `censored` / `missing` | A number for a censored observation |

#### Optional fields, still constrained

Calibration reference and date, detection limit, quantification limit, uncertainty components, CRS/datum, method identifier, sampling interval, chain-of-custody references, parameter-specific extras (gage datum for stage).

If uncertainty is present, store at least: magnitude as decimal text, unit, coverage or confidence, method, and whether it came from the source. Do not default missing uncertainty to zero.

#### Validation rules

1. A record labeled Observed under this profile is invalid if `raw_artifact_cid` is absent without an explicit missing reason.
2. `value_decimal` must round-trip as text. Binary float alone fails.
3. `sampling: instantaneous` is invalid for a source statistic named daily/mean/min/max unless a transformation record shows the derivation and the result is Inferred.
4. If `approval_status` is missing without a reason, validation fails. It must not display as approved.
5. A second record with the same site, parameter, and `time_event` but a later `time_published` is a supersession candidate, not an independent observation, unless the producer declares a different procedure.
6. The implementation must not fill `unit_id`, `quality_flags`, or `uncertainty` by inference from quantity name.
7. Parse from artifact to profile fields is a transformation (Article VIII) or it is incomplete.

#### Known failure modes

- Source revises the URL in place; CID of old bytes still exists, live GET does not match.
- Qualifier list empty in Witness because the importer did not read the field, not because the source had none.
- Clock on the logger wrong; `time_event` looks precise.
- Datum change makes two stage values incomparable.
- Ice-affected discharge treated as ordinary Observed.
- Dashboard shows “Observed” in one column and hides `approval_status`.
- Generated caption “the river is at flood stage” sitting beside a provisional point with the same visual weight.

#### Fixture A — conforming and still not “true”

A synthetic but schema-complete instantaneous stage point:

- raw artifact CID present
- quantity `USGS:00065`, unit UCUM `[ft_i]`
- value `"12.37"`
- `sampling: instantaneous`
- `approval_status: provisional`
- quality flags `[]` with source saying none
- `time_event` ≠ `time_published` ≠ `time_ingested`
- transformation record from JSON body to fields
- no flood interpretation

This fixture asserts inspectability. It does not assert that the river was 12.37 ft.

#### Fixture B — visibly incomplete (current demo shape)

`quantity: air_temperature`, `numeric: 18.4`, `unit: celsius`, `instrument-id: station-001`, one timestamp.

This fixture must fail `witness-profile-instrument-measurement/0.1` for all of: no artifact CID, float value, free-text unit, one time, no sampling, no approval, no flags, no transformation record.

The point of Fixture B is to keep the README demo from counting as a conforming observation. If the demo passes the profile, the profile is theater.

### What would demonstrate that an inference from this observation is reproducible

Not a second person agreeing in prose. All of:

- the artifact bytes and CID
- the transformation code version and parameters
- the profile fields reconstructed by a second implementation
- the claim text bound to those premise CIDs
- a falsifier that is a later source document, not a sentence: e.g. “an approved USGS value for the same site, parameter, and `time_event` that differs by more than the published uncertainty, or a qualifier the importer dropped”

Until the falsifier is a document-shaped condition, it is commentary.

### Boundary

This review does not say USGS should sign Witness records. It does not say Witness should become the archive of record for water data. It does not rate the founder’s intent. It says the present observation type cannot carry the pilot the project proposed.

### What would change this objection

A raw-artifact node; decimal values; explicit missingness; distinct times; source quality and approval vocabularies; parse recorded as transformation; daily products labeled Inferred; Fixture B failing validation.

Until then, the instrument-measurement profile does not exist, and labeling the README example Observed is the first false confidence the system produces.

---

That is one completed review. It can be pasted as a comment on issue #2. It should not be summarized as “an expert approved the pilot.” The useful sentence for the public record is: the current Observed type cannot represent a provisional USGS point that later changes, and the profile is not done until Fixture B fails.
