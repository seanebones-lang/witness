# USGS river-gauge source-preservation reference

This is a **curated demonstration**, not independently reviewed evidence and not
a claim that USGS reviewed, signed, or endorsed Witness. It demonstrates the
first half of a reference story: preserve an exact public response, identify the
request and retrieval, extract a source point without silently changing its
representation, and make tampering detectable.

The bundle intentionally stops before ingestion as a conforming Witness
observation. The current scalar `Measurement` type cannot yet represent all of
the source semantics demanded by the draft instrument profile. Calling the
extracted point `Observed` now would hide that gap.

## Bounded source request

- Publisher claimed by the endpoint: U.S. Geological Survey
- Site: `09380000` — Colorado River at Lees Ferry, Arizona
- Parameter: `00060` — streamflow
- Requested interval: `2024-07-01` through `2024-07-02`
- Service: USGS NWIS Instantaneous Values web service
- Retrieval recorded in: [`snapshot/manifest.json`](snapshot/manifest.json)

The committed response is a dated snapshot. Re-running retrieval may produce
different bytes or headers even for the same historical interval. A new
retrieval must be stored as a new bundle, not written over evidence already
referenced by a manifest.

## Files and epistemic roles

| File | Role |
| --- | --- |
| `snapshot/response.json` | Exact response body returned by the recorded HTTPS request |
| `snapshot/response-headers.json` | Final HTTP response headers normalized into JSON |
| `snapshot/manifest.json` | Request, retrieval, status, sizes, digests, and explicit limits |
| `snapshot/extracted-point.json` | Versioned extraction result; explicitly not a conforming observation |
| `tools/retrieve.py` | Retriever `0.1.0`; writes a new source-artifact bundle |
| `tools/extract.py` | Extractor `0.1.0`; selects one timestamp and preserves value text and qualifiers |
| `tools/verify.py` | Independent local digest and extraction verifier |

## Verify the committed bundle

From the repository root:

```bash
python3 reference/usgs-river-gauge/tools/verify.py \
  --bundle reference/usgs-river-gauge/snapshot
```

Expected output:

```text
VERIFIED: artifact sizes and SHA-256 digests match the manifest
VERIFIED: extracted timestamp, exact value text, and qualifiers occur in the raw response
NOT VERIFIED: publisher identity, measurement accuracy, calibration, completeness, or truth
```

The verifier checks that the committed body and header files match their
recorded byte counts and SHA-256 digests. It also checks that the extracted
timestamp, exact source value text, and qualifier array occur together in the
raw response. It does not rely on a running Witness server or database.

## Make a new retrieval

Use a new output directory so the committed snapshot remains immutable:

```bash
python3 reference/usgs-river-gauge/tools/retrieve.py \
  --output /tmp/witness-usgs-retrieval
python3 reference/usgs-river-gauge/tools/extract.py \
  --bundle /tmp/witness-usgs-retrieval
python3 reference/usgs-river-gauge/tools/verify.py \
  --bundle /tmp/witness-usgs-retrieval
```

The default request is deliberately fixed in `retrieve.py` for reviewability.
A future version should accept a constrained request specification while
canonicalizing and validating every parameter.

## Exact extraction demonstrated

The committed extraction selects this source point:

```json
{
  "site_code": "09380000",
  "parameter_code": "00060",
  "source_datetime": "2024-07-01T00:00:00.000-07:00",
  "value_exact_source_text": "12300",
  "unit_code": "ft3/s",
  "qualifiers": ["A"]
}
```

The response itself defines qualifier `A` as “Approved for publication -- Processing and review completed.” The extractor preserves that definition, its `NWIS` network, and its `uv_rmk_cd` vocabulary as a source-declared approval state. It does not turn that source status into a Witness approval, accuracy finding, signature, or endorsement. The extraction also records `valueType` as
`Derived Value`; Witness must not present that point as a direct instrument
reading without representing the derivation and source semantics.

## What remains before a conforming observation

1. Ratify an instrument-measurement evidence profile with domain review.
2. Add first-class source-artifact and transformation records to the protocol.
3. Replace binary floating-point as the authoritative value carrier with an
   exact, vocabulary-bound representation.
4. Preserve and interpret qualifiers, approval state, missingness, sampling
   regime, datum, and all relevant times.
5. Record a versioned transformation from this source artifact to an
   observation, rather than hiding normalization inside an importer.
6. Add a correction/supersession scenario without overwriting the first bundle.
7. Obtain independent reproduction and document objections and project response.

Discussion and acceptance criteria are tracked in
[issue #8](https://github.com/seanebones-lang/witness/issues/8).

## Source and rights note

This bundle contains a response retrieved from a U.S. government service. The
repository records the source and retrieval context but makes no independent
rights determination for every field or upstream contribution. Review the
applicable USGS policies before redistributing a different dataset or expanding
this pilot.
