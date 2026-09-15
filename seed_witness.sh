#!/usr/bin/env bash
# Seed Witness with signed demonstration fixtures across all three epistemic categories.
# These records are illustrative and were not fetched from the named institutions.

set -euo pipefail

trap 'rm -f /tmp/witness-seed.key' EXIT

INGEST="cargo run --quiet --package witness-ingestion --"

echo "=== Seeding Witness database ==="

# Generate keypair for signing
echo "WARNING: demonstration fixtures only; not independently verified evidence"
KEY_OUTPUT=$($INGEST gen-key --output /tmp/witness-seed.key 2>&1 | tail -1)
echo "Key: $KEY_OUTPUT"
SIGNED_INGEST="$INGEST --key-file /tmp/witness-seed.key --dataset-status demo-fixture"

# ============================================================
# OBSERVED: Real-world measurements
# ============================================================

echo "--- Ingesting OBSERVED data ---"

TEMP_SFO=$($SIGNED_INGEST observe \
  --quantity "air_temperature" \
  --value "23.4" \
  --unit "celsius" \
  --instrument-id "NOAA-ASOS-KSFO" \
  --author-id "NOAA" \
  --author-name "National Weather Service" \
  --domain "weather" \
  --labels "surface" \
  --labels "recent" \
  --latitude 37.619 \
  --longitude 122.375 \
  --station-id "KSFO" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  KSFO temp: $TEMP_SFO"

TEMP_JFK=$($SIGNED_INGEST observe \
  --quantity "air_temperature" \
  --value "21.8" \
  --unit "celsius" \
  --instrument-id "NOAA-ASOS-KJFK" \
  --author-id "NOAA" \
  --author-name "National Weather Service" \
  --domain "weather" \
  --labels "surface" \
  --labels "recent" \
  --latitude 40.641 \
  --longitude 73.778 \
  --station-id "KJFK" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  KJFK temp: $TEMP_JFK"

TEMP_MIA=$($SIGNED_INGEST observe \
  --quantity "air_temperature" \
  --value "28.2" \
  --unit "celsius" \
  --instrument-id "NOAA-ASOS-KMIA" \
  --author-id "NOAA" \
  --author-name "National Weather Service" \
  --domain "weather" \
  --labels "surface" \
  --labels "recent" \
  --latitude 25.793 \
  --longitude 80.291 \
  --station-id "KMIA" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  KMIA temp: $TEMP_MIA"

# CO2 measurements (Mauna Loa - historical and recent)
CO2_RECENT=$($SIGNED_INGEST observe \
  --quantity "atmospheric_co2" \
  --value "421.5" \
  --unit "ppm" \
  --instrument-id "NOAA-MLO-01" \
  --author-id "NOAA-GML" \
  --author-name "Global Monitoring Laboratory" \
  --domain "climate" \
  --labels "keeling-curve" \
  --labels "recent" \
  --latitude 19.539 \
  --longitude 155.579 \
  --station-id "MLO" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  MLO CO2 recent: $CO2_RECENT"

CO2_HIST=$($SIGNED_INGEST observe \
  --quantity "atmospheric_co2" \
  --value "419.8" \
  --unit "ppm" \
  --instrument-id "NOAA-MLO-01" \
  --author-id "NOAA-GML" \
  --author-name "Global Monitoring Laboratory" \
  --domain "climate" \
  --labels "keeling-curve" \
  --labels "historical" \
  --latitude 19.539 \
  --longitude 155.579 \
  --station-id "MLO" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  MLO CO2 historical: $CO2_HIST"

CO2_1958=$($SIGNED_INGEST observe \
  --quantity "atmospheric_co2" \
  --value "315.2" \
  --unit "ppm" \
  --instrument-id "SCRIPPS-MLO-01" \
  --author-id "SCRIPPS" \
  --author-name "Scripps Institution of Oceanography" \
  --domain "climate" \
  --labels "keeling-curve" \
  --labels "historical" \
  --labels "1958" \
  --latitude 19.539 \
  --longitude 155.579 \
  --station-id "MLO" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  MLO CO2 1958: $CO2_1958"

# River gauge data (USGS)
STREAM_RECENT=$($SIGNED_INGEST observe \
  --quantity "streamflow" \
  --value "12400" \
  --unit "cfs" \
  --instrument-id "USGS-09380000" \
  --author-id "USGS" \
  --author-name "US Geological Survey" \
  --domain "hydrology" \
  --labels "surface-water" \
  --labels "recent" \
  --latitude 36.099 \
  --longitude 112.096 \
  --station-id "09380000" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  Colorado River recent: $STREAM_RECENT"

STREAM_HIST=$($SIGNED_INGEST observe \
  --quantity "streamflow" \
  --value "8920" \
  --unit "cfs" \
  --instrument-id "USGS-09380000" \
  --author-id "USGS" \
  --author-name "US Geological Survey" \
  --domain "hydrology" \
  --labels "surface-water" \
  --labels "historical" \
  --latitude 36.099 \
  --longitude 112.096 \
  --station-id "09380000" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  Colorado River historical: $STREAM_HIST"

# Seismic data (USGS)
QUAKE=$($SIGNED_INGEST observe \
  --quantity "earthquake_magnitude" \
  --value "4.2" \
  --unit "Mw" \
  --instrument-id "USGS-NEVADA-SEISMIC" \
  --author-id "USGS-NEIC" \
  --author-name "National Earthquake Information Center" \
  --domain "seismology" \
  --labels "tectonic" \
  --labels "recent" \
  --latitude 39.163 \
  --longitude 119.767 \
  --station-id "NV.RNO" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  Reno quake: $QUAKE"

# Satellite radiance (CERES)
SAT=$($SIGNED_INGEST observe \
  --quantity "toa_radiance" \
  --value "239.4" \
  --unit "W/m2" \
  --instrument-id "CERES-FM5-AQUA" \
  --author-id "NASA-LARC" \
  --author-name "NASA Langley Research Center" \
  --domain "earth-observation" \
  --labels "satellite" \
  --labels "radiation-budget" \
  --labels "recent" \
  --latitude 0 \
  --longitude 0 \
  --station-id "AQUA" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  CERES radiance: $SAT"

# Hospital census
HOSP=$($SIGNED_INGEST observe \
  --quantity "hospital_bed_occupancy" \
  --value "847" \
  --unit "beds" \
  --instrument-id "HHS-PROTECT" \
  --author-id "HHS" \
  --author-name "US Dept Health & Human Services" \
  --domain "public-health" \
  --labels "covid" \
  --labels "capacity" \
  --labels "recent" \
  --latitude 38.907 \
  --longitude 77.037 \
  --station-id "DC-HOSP-001" 2>&1 | grep "Ingested observation" | awk '{print $NF}')
echo "  DC hospital: $HOSP"

# ============================================================
# INFERRED: Derived claims with explicit premises
# ============================================================

echo "--- Ingesting INFERRED data ---"

# Climate inference from CO2 observations
$SIGNED_INGEST infer \
  --claim "Global mean surface temperature has risen 1.1°C since pre-industrial" \
  --methodology "Optimal fingerprinting detection/attribution using CMIP6 ensemble" \
  --premise "$CO2_RECENT" \
  --premise "$CO2_HIST" \
  --premise "$CO2_1958" \
  --author-id "IPCC-AR6-WG1" \
  --author-name "IPCC Working Group I" \
  --domain "climate" \
  --labels "attribution" \
  --labels "consensus" \
  --labels "policy-relevant" 2>&1 | grep "Ingested inference" | awk '{print $NF}'

# Streamflow trend inference
$SIGNED_INGEST infer \
  --claim "Colorado River at Lees Ferry shows 20% decline in annual flow since 2000" \
  --methodology "Mann-Kendall trend test on USGS daily discharge records (2000-2024)" \
  --premise "$STREAM_RECENT" \
  --premise "$STREAM_HIST" \
  --author-id "USBR-LC" \
  --author-name "US Bureau of Reclamation Lower Colorado Region" \
  --domain "hydrology" \
  --labels "trend-analysis" \
  --labels "water-rights" \
  --labels "policy-relevant" 2>&1 | grep "Ingested inference" | awk '{print $NF}'

# Temperature anomaly inference
$SIGNED_INGEST infer \
  --claim "San Francisco Bay Area July 2024 average temperature 1.8°C above 1991-2020 normal" \
  --methodology "Anomaly calculation vs NOAA NCEI 1991-2020 climate normals" \
  --premise "$TEMP_SFO" \
  --author-id "NOAA-NCEI" \
  --author-name "National Centers for Environmental Information" \
  --domain "weather" \
  --labels "anomaly" \
  --labels "climate-monitoring" \
  --labels "recent" 2>&1 | grep "Ingested inference" | awk '{print $NF}'

# Hospital capacity inference
$SIGNED_INGEST infer \
  --claim "DC metro area ICU capacity at 92% — surge protocols activated" \
  --methodology "Threshold exceedance: >90% ICU occupancy triggers HHS Tier 2 surge" \
  --premise "$HOSP" \
  --author-id "HHS-ASPR" \
  --author-name "Administration for Strategic Preparedness & Response" \
  --domain "public-health" \
  --labels "surge" \
  --labels "operations" \
  --labels "actionable" 2>&1 | grep "Ingested inference" | awk '{print $NF}'

# Seismic hazard inference
$SIGNED_INGEST infer \
  --claim "M4.2 Reno sequence has 12% probability of M5+ within 7 days (UCERF3-ETAS)" \
  --methodology "Epidemic-Type Aftershock Sequence model calibrated to Nevada seismic catalog" \
  --premise "$QUAKE" \
  --author-id "USGS-PAGER" \
  --author-name "Prompt Assessment of Global Earthquakes for Response" \
  --domain "seismology" \
  --labels "forecast" \
  --labels "operational" \
  --labels "time-dependent" 2>&1 | grep "Ingested inference" | awk '{print $NF}'

# ============================================================
# GENERATED: Synthetic / model outputs (using Generate command)
# ============================================================

echo "--- Ingesting GENERATED data ---"

# Climate model ensemble output
$SIGNED_INGEST generate \
  --content "SSP2-4.5 ensemble projects 2.7°C warming by 2100 (5-95%: 2.1-3.5°C)" \
  --generator "CMIP6" \
  --model "ensemble-mean" \
  --author-id "WCRP-CMIP6" \
  --author-name "World Climate Research Programme CMIP Panel" \
  --domain "climate" \
  --labels "projection" \
  --labels "scenario" \
  --labels "generated" \
  --labels "model-ensemble" 2>&1 | grep "Ingested generation" | awk '{print $NF}'

# Weather forecast (GFS)
$SIGNED_INGEST generate \
  --content "GFS 00z run: KSFO 2m temp 22.1°C at 2024-09-15 18:00Z" \
  --generator "GFS" \
  --model "v16.2" \
  --author-id "NCEP-EMC" \
  --author-name "Environmental Modeling Center" \
  --domain "weather" \
  --labels "forecast" \
  --labels "nwp" \
  --labels "generated" \
  --labels "00z-run" 2>&1 | grep "Ingested generation" | awk '{print $NF}'

# Synthetic population dataset
$SIGNED_INGEST generate \
  --content "Census tract 06075012300 synthetic population: 4,231 persons, 1,847 households" \
  --generator "IPF-Synthesizer" \
  --model "v3.1" \
  --author-id "RTI-ITOP" \
  --author-name "RTI International ITOP" \
  --domain "demographics" \
  --labels "synthetic" \
  --labels "agent-based-model" \
  --labels "generated" 2>&1 | grep "Ingested generation" | awk '{print $NF}'

# LLM-generated summary (explicitly labeled generated)
$SIGNED_INGEST generate \
  --content "Summary: Colorado River basin faces structural deficit of 1.2 MAF/yr under current allocations" \
  --generator "GPT-4o" \
  --prompt "Synthesize USBR 2024 CRSS results with tribal water rights" \
  --author-id "LLM-GPT4O" \
  --author-name "OpenAI GPT-4o (auto-labeled generated)" \
  --domain "hydrology" \
  --labels "llm-summary" \
  --labels "generated" \
  --labels "needs-verification" 2>&1 | grep "Ingested generation" | awk '{print $NF}'

echo "=== Seeding complete ==="
echo ""
echo "Verify at:"
echo "  http://localhost:8080/"
echo "  curl http://localhost:8080/api/nodes"
echo "  curl http://localhost:8080/api/nodes?type=observed"
echo "  curl http://localhost:8080/api/nodes?type=inferred"
echo "  curl http://localhost:8080/api/nodes?type=generated"
