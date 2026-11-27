# Genesis Engine — Benchmark Study Area (D-031)

**Date:** 2026-11-27. **Status:** selected by the agent under owner delegation D-031; recorded as the frozen study area for spec §49 comparative benchmarks and the M1 end-to-end fixture (GE-0020).

## Selected boundary

| Field | Value |
| --- | --- |
| Name | Allen Avenue / Alausa study area, Ikeja, Lagos |
| Bounding box (WGS84, lon/lat) | **west 3.3480, south 6.5985, east 3.3552, north 6.6051** |
| Extent | ~0.735 km N–S × ~0.796 km E–W (~0.59 km²) |
| Centre | 6.6018°N, 3.3516°E |
| Local CRS note | Project CRS/epoch/vertical datum still to be set in GE-0010/GE-0013; bbox is recorded in WGS84/CRS84 for interchange only |

## Selection evidence and rationale

- **Verified place identity:** read-only Nominatim reverse geocode (2026-11-27), User-Agent `GenesisEngineResearch/0.1`, for `lat=6.6018&lon=3.3515&zoom=16` returned `name: Allen Avenue`, `display_name: Allen Avenue, Oregun, Alausa, Ikeja, Lagos, 100282, Nigeria`, `osm_type: way`, `osm_id: 134404795`. This places the centre inside Ikeja as required by spec §49.
- **Data source and licence:** OpenStreetMap (ODbL 1.0 — `© OpenStreetMap contributors`); no restricted or proprietary map product is used. The area is documented by a public OSM object, satisfying "documented, redistributable source".
- **Size:** spec §49 asks for approximately 500 m to 1 km across; the selected box is ~735 m × ~796 m.
- **Reproducibility:** the bbox, centre, OSM object reference and retrieval date are frozen here. Any benchmark must use the same boundary and pinned input revisions; changes require a new dated record.

## Ground truth and honest limits

- **No independent survey, LiDAR or ground-truth dataset is currently available for this area.** Comparative results may establish **completeness and structural quality only**, not real-world positional or height accuracy (spec §49). The "Survey/reference" quality profile must not be claimed for this benchmark.
- Candidate comparisons under spec §49 (Genesis prototype, gen-map, Mapforge, OSM2World, photogrammetry where lawful imagery exists) must use the **same frozen inputs**. gen-map remains unverified (source licence unavailable); OSM2World licence conflict resolved to MIT from the repository (RESEARCH_REGISTER §4 #1), with the wiki discrepancy noted.
- Small, redistributable OSM/footprint fixtures are to be cut from this bbox under the licence rules in `research/LICENCE_AND_DATA_RIGHTS.md`; no raw OSM extract or restricted imagery is committed.

## Follow-up

GE-0020 (E2E vertical slice + benchmark harness) records input checksums, tool versions and run results against this boundary. If the owner later supplies checkpoints or reference data, create a dated amendment here before any accuracy claim is made.
