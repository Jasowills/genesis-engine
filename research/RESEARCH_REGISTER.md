# Genesis Engine — Research Register

**Research date:** 2026-11-27
**Rule:** No recommendation is final until it is reconciled against a primary source and recorded here with a source URL and access date. Unverified claims are labelled explicitly.

## 1. Active research briefs

| ID | Brief | Agent | Output file | Status |
| --- | --- | --- | --- | --- |
| R-01 | Geospatial data, CRS, formats, licences | `general` subagent | `research/notes/geospatial.md` | **Complete** — file read and verified after restart; session resumed successfully |
| R-02 | Reconstruction / photogrammetry / NeRF / splat pipelines | `general` subagent | `research/notes/reconstruction.md` | **Complete** — file read and verified 2026-11-27 |
| R-03 | Desktop shell, viewport, 3D interchange formats | `general` subagent | `research/notes/desktop-viewport-formats.md` | **Complete** — file read and verified 2026-11-27 |
| R-04 | AI runtimes and models | `general` subagent | `research/notes/ai-ml.md` | **Complete** — file read and verified 2026-11-27 |
| R-05 | Competitive landscape and data/model rights | `general` subagent | `research/notes/competitive-data-rights.md` | **Complete** — file read and verified 2026-11-27 |
| R-06 | Procedural generation and OSM-derived reconstruction | `general` subagent | `research/notes/procedural.md` | **Complete** — file read and verified 2026-11-27 |
| R-07 | Plugin architecture, sandboxing, security and supply chain | `general` subagent | `research/notes/plugin-security.md` | **Complete** — file read and verified 2026-11-27 |
| R-08 | Quality validation, cross-platform CI/GPU, performance/release | `general` subagent | `research/notes/quality-ci-gpu.md` | **Complete** — file read and verified after restart; session resumed successfully |

Each brief was told to use primary sources, record the spec §20 field set, cite URLs with access date 2026-11-27, distinguish verified facts from estimates, and provide evidence only — no final architecture decisions.

## 2. Consolidated deliverables (spec Phase 3)

| File | Status | Contents |
| --- | --- | --- |
| `research/ECOSYSTEM_RESEARCH.md` | **Written 2026-11-27** (all eight briefs incorporated) | Technology evidence tables, reuse-vs-build rationale, integration complexity |
| `research/COMPETITIVE_ANALYSIS.md` | **Written 2026-11-27** | Real-workflow comparison, differentiation, benchmark design |
| `research/LICENCE_AND_DATA_RIGHTS.md` | **Written 2026-11-27** (geospatial section appended) | Licence obligations for code, data, model weights, generated assets |
| `research/FORMAT_AND_STANDARDS.md` | **Written 2026-11-27** (geospatial CRS/formats appended) | Format/CRS/standard capabilities, metadata preservation |
| `research/RESEARCH_REGISTER.md` | This file | Briefs, sources, conflicts, confidence |

## 3. Source quality policy

1. Primary first: official docs, official repositories, LICENSE files, standards specifications, release notes, papers.
2. Secondary sources (articles, videos, forums) are supporting evidence only.
3. Re-verify material licence/capability claims as of 2026-11-27; do not inherit old notes.
4. Any claim that cannot be tied to a primary source is marked "unverified" and is not used to justify an architecture decision.
5. Contradictions between subagents are logged below and resolved with a primary source before conclusions are presented.

## 4. Conflicts and resolutions

1. **OSM2World licence conflict (unresolved):** repository `LICENSE.txt` and GitHub API say **MIT**; OSM Wiki infobox says **LGPL**. Primary-source evidence favours MIT, but a maintainer confirmation is needed before treating OSM2World code as MIT for any vendoring. Recorded in `research/notes/procedural.md`.
2. **Microsoft GlobalMLBuildingFootprints count inconsistency (unresolved):** README introduction states 1.4B buildings; FAQ states 999M. Does not affect the licence conclusion (CDLA Permissive 2.0) but affects dataset-size estimates.
3. **WASI TCP/UDP default conflict (unresolved, resolve defensively):** `WasiCtxBuilder::new()` docs state TCP/UDP allowed but all addresses denied by default; `allow_tcp` docs state it is disabled by default. Mitigation: explicitly deny all network in host code regardless. Recorded in `research/notes/plugin-security.md`.
4. **ONNX Runtime GitHub repository unreachable (2026-11-27):** `microsoft/onnxruntime` returned 404 via web/API in the research session; version 1.32.0 and MIT licence were verified via PyPI and official docs instead. Re-check from another network before relying on repo metadata.
5. **OSM2World CLI stability:** site/docs describe a mature tool; OSM Wiki (older text) calls the CLI syntax unstable and elevation "very fragile and deactivated by default". Treat as a PoC gate, not an established fact about 0.4.0.
6. **Vulkan-on-Metal (unresolved):** Khronos has deprecated MoltenVK and recommends KosmicKrisp on Apple Silicon, while the wgpu README still says macOS Vulkan requires MoltenVK. Prefer native Metal on macOS; treat Vulkan-on-Metal as unverified until a PoC.
7. **`geo` far-from-origin precision:** fixes are documented on unreleased `main`, not in released 0.33.1. Treat as an open risk with regression fixtures, not a solved problem.
8. **quantized-mesh / 3D Tiles repository licences:** no LICENSE file found in either repository root (checked 2026-11-27). The OGC standards documents are royalty-free to implement, but deriving code from those repos is unverified.

The accepted ADRs (D-025) were approved on the research evidence as it stood. The conflicts listed above remain open and are assigned to linked tickets for closure before implementation; none may be treated as settled by ADR acceptance alone. Where a conflict touches an accepted ADR (OSM2World licence → GE-0014/GE-0028; WASI TCP defaults → GE-0005/GE-0025; ORT repo availability → GE-0006; Vulkan-on-Metal → GE-0004; `geo` precision fixes → GE-0003/GE-0011; quantized-mesh/3D Tiles repo licences → later export tickets), the ticket must resolve it before that work proceeds.

## 5. Follow-ups / proof of concepts to propose as tickets

Converted to tickets: GE-0003 (precision/CRS golden vectors), GE-0004 (shell/viewport PoC), GE-0005 (WASM plugin feasibility), GE-0006 (AI runtime/model smoke test), GE-0019 (glTF round-trip), GE-0020 (E2E benchmark). Conflicts in §4 are assigned to the tickets named in that section.
