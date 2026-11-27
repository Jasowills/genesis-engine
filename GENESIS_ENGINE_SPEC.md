# Genesis Engine --- Foundational Project Specification

## Research-first product definition, architecture discovery, ticket-driven implementation, and engineering governance

**Document status:** Foundational specification / implementation
charter\
**Project name:** Genesis Engine\
**Working description:** An open-source, cross-platform platform for
reconstructing real-world environments into editable, validated,
game-ready 3D worlds.\
**Specification rule:** This document establishes intent, constraints,
decision principles, and the process for producing the implementation
PRD. It is **not** permission to skip research or immediately generate a
large codebase.\
**Primary implementation agent:** OpenCode\
**Human authority:** Project owner\
**Date prepared:** 27 November 2026

------------------------------------------------------------------------

# PART I --- PRE-PROMPT FOR OPENCODE

> Copy the prompt in this section into OpenCode as the initial
> instruction. OpenCode must read this entire file first. The prompt
> deliberately requires research and planning before implementation.

## 1. Initial instruction

You are the principal product architect, staff-level systems engineer,
technical researcher, engineering manager, and implementation
coordinator for **Genesis Engine**.

Your first task is **not to write application code**. Your first task is
to understand this specification, inspect the available development
environment and repository state, research the existing ecosystem,
challenge assumptions, identify unknowns, ask the project owner focused
questions, and produce an evidence-backed implementation plan.

Read this entire `GENESIS_ENGINE_SPEC.md` before doing anything else.
Then inspect the workspace and available tools. Do not assume that the
workspace is empty, that a GitHub repository already exists, that a
particular framework is installed, or that any named agent skill is
available.

The project owner expects a high-quality, maintainable, open-source
engineering platform. Optimise for correctness, architectural clarity,
extensibility, testability, reproducibility, developer experience, and
long-term maintainability. Do not optimise for producing the largest
amount of code quickly.

### Non-negotiable operating rules

1.  **Research before architecture.** Research existing open-source
    projects, commercial products, standards, APIs, datasets, libraries,
    formats, and relevant academic/industry approaches before
    recommending a design.
2.  **Verify before adopting.** Check current repository activity,
    actual capabilities, licence, compatibility, platform support,
    release state, maintenance signals, and integration cost. Do not
    infer these from a project name or README headline.
3.  **Do not reinvent mature capabilities without a reason.** Prefer
    integrating or wrapping mature projects when their licence,
    architecture, quality, and maintenance profile are suitable. Record
    why each major capability is reused, wrapped, forked, or implemented
    internally.
4.  **No speculative implementation.** Do not implement unresolved
    architectural decisions. Identify them, explain their consequences,
    and ask the owner.
5.  **Plan before code.** Produce the research report, architecture
    proposal, ADRs, product requirements, milestones, and ticket backlog
    before starting implementation.
6.  **Use small, verifiable tickets.** Every implementation unit must
    have a clear scope, acceptance criteria, tests, dependencies, and a
    definition of done.
7.  **Review every ticket.** Every ticket that changes code must receive
    an independent review by a reviewer subagent before it is considered
    complete. The author cannot approve its own work.
8.  **Test what changed.** Run appropriate tests, type checks,
    formatting, linting, builds, integration checks, and
    security/licence checks. Record actual commands and results.
9.  **Do not claim unverified success.** Never mark a ticket complete
    because code was written. Mark it complete only after its acceptance
    criteria, validation, and independent review pass.
10. **Keep durable project memory.** Maintain `progress.md`, engineering
    reports, ADRs, research records, ticket files, and a clear handoff
    state. Do not rely on conversation context as the only memory.
11. **Ask questions when needed.** Ask the owner when a decision
    materially changes product scope, architecture, cost, licensing, UX,
    security, or delivery. Do not ask about details that can be
    researched or safely deferred.
12. **No uncontrolled scope expansion.** Capture useful ideas in the
    backlog, but do not silently add them to the current milestone.
13. **No false precision.** Distinguish verified facts, estimates,
    assumptions, hypotheses, and recommendations.
14. **Respect user approval.** Never publish releases, deploy services,
    incur paid cloud/model/API costs, purchase domains, or make
    irreversible public changes without explicit approval.
15. **Preserve project ownership.** The owner makes product and
    strategic decisions. You may recommend strongly, but must present
    trade-offs and ask for approval where required.

## 2. Required first-run workflow

Execute these phases in order. You may use parallel research subagents
where useful, but consolidate their findings and resolve contradictions
before presenting conclusions.

### Phase 0 --- Read, inspect, and establish the working state

-   Read this entire specification.
-   Inspect the current directory, Git state, existing files, available
    runtimes, package managers, compilers, GPU/OS information where
    accessible, and installed OpenCode tools/skills.
-   Check whether `gh` and `git` are installed and authenticated. Do not
    print or expose tokens.
-   Check whether an existing GitHub repository is already configured.
-   Identify the current branch, remotes, uncommitted changes, and
    repository ownership.
-   Do not overwrite or delete existing work.
-   Record environment limitations in
    `engineering/ENVIRONMENT_REPORT.md`.
-   Create a concise initial `progress.md` only after confirming the
    project directory and repository state. If the repo already contains
    a progress file, preserve and update it rather than replacing it.

### Phase 1 --- Skill and workflow discovery

The owner has requested the **Matty Pocock skill/workflow** for breaking
work into well-defined, independently verifiable tasks.

-   Search the installed OpenCode skills and available skill registries
    for the exact Matty Pocock skill the owner means.
-   Inspect its actual instructions and usage requirements before
    invoking it.
-   Do not invent a skill name, fabricate its contents, or claim it was
    used if it is unavailable.
-   If several similarly named skills exist, report the candidates and
    ask the owner which one they intended.
-   If it is unavailable, explain the limitation and propose an
    equivalent ticket decomposition process without falsely attributing
    it to Matty Pocock.
-   Follow the skill's instructions where compatible with this
    specification. This specification remains the project-level
    authority.
-   Record the exact skill name, source, version/commit if available,
    and how it was applied in `engineering/WORKFLOW.md`.

Use subagents for independent research and review where the installed
OpenCode environment supports them. Discover actual agent capabilities
and tool permissions rather than assuming names or capabilities.

### Phase 2 --- Ecosystem and technical research

Research the project domains in Part IV of this document. Start with
primary sources:

-   Official documentation.
-   Official GitHub repositories and release notes.
-   Standards specifications.
-   Official API and dataset licensing pages.
-   Technical papers where they materially inform a choice.
-   Maintainer discussions and issue trackers for unresolved
    implementation limitations.

Use secondary articles, videos, forums, and community discussions as
supporting evidence, not as the sole basis for important decisions.

For every candidate technology, record:

-   What problem it solves.
-   What it actually supports today.
-   Repository and official documentation URLs.
-   Licence and obligations.
-   Latest meaningful release and maintenance signals.
-   Supported operating systems and architectures.
-   Language/runtime and dependency footprint.
-   CPU/GPU requirements and acceleration backends.
-   Import/export formats.
-   Extensibility and embedding/integration options.
-   Known limitations and unresolved issues.
-   Whether Genesis should integrate, wrap, adapt, fork, or build an
    alternative.
-   Estimated integration complexity: low, medium, high, or unknown.
-   Confidence and evidence quality.

Do not rely on old research notes as proof that a project still has the
same capabilities or licence. Re-verify all material claims as of the
research date.

### Phase 3 --- Competitive and open-source landscape

Study relevant projects and products, including but not limited to the
candidates listed in Part IV. Find additional candidates rather than
limiting research to this list.

Compare them against real Genesis workflows. Avoid superficial feature
checklists. Where possible, run a small proof of concept or inspect
sample outputs.

Create:

-   `research/ECOSYSTEM_RESEARCH.md`
-   `research/COMPETITIVE_ANALYSIS.md`
-   `research/LICENCE_AND_DATA_RIGHTS.md`
-   `research/FORMAT_AND_STANDARDS.md`
-   `research/RESEARCH_REGISTER.md`

Cite source URLs and access dates. Mark any unverified claim explicitly.

### Phase 4 --- Architecture discovery and design

Only after research, propose a system architecture. Consider:

-   Product boundaries and runtime modes.
-   Core engine and domain boundaries.
-   Desktop UI and viewport architecture.
-   World representation and schema evolution.
-   Geospatial processing.
-   Reconstruction backends.
-   AI/model adapters.
-   Job orchestration and resource scheduling.
-   Storage, caching and versioning.
-   Plugin SDK and sandbox.
-   Engine/DCC integrations.
-   Validation and quality gates.
-   Cross-platform packaging and updates.
-   Security and licence provenance.

Create architecture diagrams and data-flow diagrams. Identify failure
modes, bottlenecks, platform risks, and hard-to-reverse choices.

Use Architecture Decision Records (ADRs) for important decisions. Each
ADR must include context, options, decision, rationale, consequences,
status, and evidence. Do not turn every minor implementation detail into
an ADR.

### Phase 5 --- Product discovery and owner questions

Prepare a focused question list for the owner. Group questions by:

-   Product identity and scope.
-   Primary user workflows.
-   First release and MVP boundaries.
-   World and asset editing.
-   Quality expectations.
-   Supported data sources and licensing policy.
-   Hardware and performance targets.
-   Export and integration priorities.
-   Local/cloud execution.
-   AI model distribution.
-   Open-source governance and monetisation.

For each question include: - Why it matters. - Available options. - Your
recommendation. - Consequences of each option. - Whether it blocks
architecture or can be deferred.

Ask a small, prioritised batch of questions rather than overwhelming the
owner with every unresolved detail. The owner may answer partially.
Record approved decisions in `DECISIONS.md` and relevant ADRs.

### Phase 6 --- PRD and ticket generation

Do not generate implementation tickets until the research, architecture
and product boundaries are sufficiently understood.

Create:

-   `PRD.md` --- product requirements and user journeys.
-   `ARCHITECTURE.md` --- approved system design.
-   `ROADMAP.md` --- milestone sequence and dependencies.
-   `DECISIONS.md` --- approved decisions and unresolved questions.
-   `TICKETS/README.md` --- ticket workflow and status definitions.
-   One Markdown file per ticket under `TICKETS/`.
-   `progress.md` --- authoritative live progress and next action.

Tickets must be small enough to implement, test, review, and merge
independently. A ticket may represent a feature, engineering task,
research spike, bug, refactor, security issue, documentation task, or
integration.

Each ticket must contain the schema in Part VII. Build dependency links
and milestone groupings. Include research and architecture tasks as
tickets where they have distinct deliverables.

### Phase 7 --- GitHub project setup

After the owner approves the repository name, visibility, description,
licence direction, and initial plan:

-   Use `gh` CLI to create or connect the GitHub repository.
-   Confirm the GitHub account and organisation/owner before creating
    anything.
-   Default to a public repository only after explicit owner
    confirmation.
-   Initialise Git if needed; do not overwrite existing Git history.
-   Add the project documentation and initial scaffold only after
    approval.
-   Configure sensible branch protection and repository settings if
    supported and approved.
-   Create GitHub labels, milestones and issues from the approved ticket
    files.
-   Link each issue to its local ticket file.
-   Use conventional commits or an agreed commit convention.
-   Never force-push, rewrite shared history, or push secrets.
-   Never commit generated datasets, large model weights, API keys,
    credentials, private imagery, or user data.
-   Add `.gitignore`, secret scanning guidance, and an appropriate Git
    LFS policy if needed.
-   Make a small initial commit, verify it, then push.
-   Report the exact repository URL, branch, commit hash, issue count,
    and any setup actions that remain.

If the repository already exists, inspect it and propose a safe
migration rather than creating a duplicate.

### Phase 8 --- Owner approval checkpoint

Present a decision package containing:

1.  Research summary and sources.
2.  Competitive landscape and differentiation.
3.  Recommended architecture and alternatives.
4.  Product requirements and non-goals.
5.  Data/licensing risks.
6.  Hardware and cost implications.
7.  Milestone plan.
8.  Ticket count by category and priority.
9.  Initial GitHub setup plan.
10. Questions requiring owner decisions.

**Stop here and wait for explicit approval.** Do not start implementing
the full product before the owner approves the foundational plan.

------------------------------------------------------------------------

# PART II --- PROJECT VISION AND PRODUCT DEFINITION

## 3. What Genesis Engine is

Genesis Engine is an open-source, cross-platform environment
reconstruction and development platform. It combines geospatial data
processing, procedural generation, photogrammetry, computer vision,
AI-assisted reconstruction, editable 3D asset workflows, quality
validation, optimisation, and game-engine export.

A user should be able to select a real-world geographic area, provide or
choose permitted source data, reconstruct selected environment layers,
inspect and correct the resulting scene, validate it against available
evidence, optimise it for a target runtime, and export it into a game
engine or 3D content tool.

Genesis is intended to serve both:

-   **Game developers**, who need world geometry, collision, LOD,
    streaming, performance budgets and reliable exports.
-   **3D and technical artists**, who need inspectable, editable
    geometry, materials, object hierarchy, provenance and correction
    tools.

The platform should be progressively developed as both:

1.  **Genesis Core** --- the engine-agnostic reconstruction, world-data,
    processing, validation and export foundation.
2.  **Genesis Studio** --- the desktop visual environment for creating,
    inspecting, editing, validating and exporting worlds.

The platform should also expose developer interfaces progressively: CLI,
SDK, headless execution, plugin interfaces and integration APIs.

## 4. Product vision

Make it dramatically easier for developers and artists to turn permitted
real-world geographic and visual data into structured, editable,
geographically grounded 3D environments.

The long-term ambition is not merely to create attractive 3D renderings.
It is to produce usable, structured, optimised, inspectable and
reproducible environment assets.

Genesis should help answer:

-   Where did this asset come from?
-   What evidence supports its shape and location?
-   Which parts were measured, reconstructed, procedural or
    AI-estimated?
-   What is its positional and geometric confidence?
-   What dependencies does it have?
-   How expensive is it to render?
-   Can I regenerate only the affected part?
-   Can I export it without losing structure or metadata?
-   Can I reproduce this world later using the same source revisions and
    settings?

## 5. Product principles

1.  **Geographic grounding:** real-world placement and coordinate
    correctness matter.
2.  **Editable output:** produce useful geometry and structured scene
    data, not only a beautiful uneditable render.
3.  **Evidence and provenance:** retain source, licence, method, model,
    version and confidence metadata.
4.  **Honest uncertainty:** never present estimates as measured facts.
5.  **Local-first:** the core product must be useful without mandatory
    hosted services.
6.  **Optional cloud:** remote compute and collaboration may extend the
    product, not gate basic use.
7.  **Engine agnostic:** the internal world representation must not be
    tied to Unreal, Unity, Godot or Blender.
8.  **Cross-platform:** Windows, macOS and Linux are foundational
    targets.
9.  **Composable architecture:** specialised tools should be integrated
    behind stable interfaces where sensible.
10. **Reproducibility:** jobs, inputs, settings and outputs should be
    traceable.
11. **Incremental regeneration:** users should not have to rebuild an
    entire city after changing one object or layer.
12. **Quality gates:** exports should be validated against explicit
    target profiles.
13. **Open ecosystem:** document APIs, encourage plugins and maintain
    clear contribution practices.
14. **Respect data rights:** source-data and model licences must be
    understood and preserved.
15. **Progressive complexity:** guided workflows for newcomers, advanced
    controls for experts.

## 6. What Genesis is not

Genesis is not:

-   A Google Maps or Street View scraping tool.
-   A service that bypasses source-provider terms or access controls.
-   Merely an OpenStreetMap extrusion tool.
-   Merely a photogrammetry GUI.
-   Merely a NeRF or Gaussian-splat viewer.
-   A black-box text-to-3D generator.
-   A game engine.
-   A replacement for Blender, Houdini, Unreal, Unity or Godot.
-   A guarantee that all real-world details can be reconstructed from
    satellite imagery.
-   A claim of survey-grade accuracy without suitable ground truth and
    validation.

## 7. Intended user journey

A complete user journey should eventually look like this:

1.  Create a Genesis project.
2.  Search for a place or enter coordinates.
3.  Draw or import a geographic boundary.
4.  Inspect available source data, coverage, dates, licences and
    estimated costs.
5.  Choose layers: terrain, roads, buildings, sidewalks, bridges,
    vegetation, street furniture, road markings, water, and other
    supported categories.
6.  Select a quality profile and target platform.
7.  Run a data and resource preflight.
8.  Review dependencies and missing inputs.
9.  Generate a preview.
10. Inspect the 3D result and its source/provenance.
11. Correct or regenerate selected assets.
12. Run geographic, geometric, visual, structural and game-readiness
    validation.
13. Optimise chunks, LODs, collision, materials and textures.
14. Export the world or selected assets.
15. Reopen the project later and reproduce or incrementally update the
    result.

The first milestone should deliver a small but complete version of this
workflow, not every layer and capability.

------------------------------------------------------------------------

# PART III --- FOUNDATIONAL DECISIONS ALREADY MADE

These decisions are approved as the current product direction. They may
be revisited only when new evidence exposes a material problem. Any
change must be recorded in an ADR and approved by the owner.

  -----------------------------------------------------------------------
  Area                                Decision
  ----------------------------------- -----------------------------------
  Product identity                    Both reconstruction engine and full
                                      environment platform, developed
                                      progressively

  Application                         Desktop-first visual Studio

  Compute                             Local-first, cloud optional

  AI strategy                         Both curated/bundled open models
                                      and optional external
                                      providers/custom models

  Target users                        Game developers and 3D/technical
                                      artists equally

  Initial operating systems           Windows, macOS and Linux
  -----------------------------------------------------------------------

## 8. Consequences of these decisions

-   The engine must be usable without the desktop UI in the long term.
-   The desktop UI must not own domain logic that belongs in the core.
-   Local processing and model execution must be designed as first-class
    paths.
-   Hosted AI and cloud compute must be adapters/providers, not hard
    dependencies.
-   Cross-platform compatibility must influence library selection from
    the beginning.
-   UI workflows must support both guided and advanced operation.
-   The first end-to-end vertical slice should exercise the same
    architecture that later SDK/CLI/headless workflows will use.

## 9. Decisions not yet approved

Do not silently assume the following:

-   Exact Rust workspace/crate structure.
-   Exact viewport renderer or UI framework beyond the desktop-first
    direction.
-   Whether CLI ships in the first public release or a later early
    milestone.
-   Exact first-release game engine integration.
-   Whether Blender integration is a first milestone or an early
    follow-up.
-   Exact minimum/recommended hardware.
-   Exact GPU backend strategy.
-   Exact world interchange schema and versioning approach.
-   Whether PostGIS is required, optional or only used in specific
    workflows.
-   Whether cloud compute is self-hostable, Genesis-hosted or both.
-   Exact AI model list and distribution approach.
-   Exact project-file storage layout.
-   Asset marketplace or collaboration scope.
-   Final licence, pending licence compatibility review and owner
    approval.

------------------------------------------------------------------------

# PART IV --- REQUIRED RESEARCH PROGRAMME

Research must be broad enough to reveal existing solutions, but focused
on decisions that can materially affect Genesis. The following are
research starting points, not an approved dependency list.

## 10. Geospatial data and processing

Research and compare:

-   OpenStreetMap data and ecosystem.
-   `geo`, `geozero`, `proj`/PROJ, GDAL and related Rust/C/C++
    geospatial libraries.
-   PostGIS and spatial indexing options.
-   GeoPackage, GeoJSON, FlatGeobuf, GeoParquet, Shapefile and other
    relevant formats.
-   Coordinate reference systems, vertical datums, geodesic calculations
    and local coordinate frames.
-   Raster and elevation formats, including GeoTIFF, COG and terrain
    tiles.
-   Data clipping, reprojection, topology repair and spatial joins.
-   Large-area streaming and tiled processing.

Questions to answer:

-   Which coordinate system should the project use internally?
-   How do we avoid precision loss far from the origin?
-   How should vertical datum and units be represented?
-   How should we handle antimeridian boundaries and unusual
    projections?
-   Which formats should be native, imported or exported?
-   Which operations can be done in Rust and which are better delegated
    to mature libraries?

## 11. Terrain reconstruction

Research:

-   DEM/DSM sources and their resolutions, coverage, licensing and
    vertical accuracy.
-   Terrain mesh generation, triangulation, heightfield and adaptive
    mesh approaches.
-   Hydrology and water-body modelling.
-   Road/terrain intersection and terrain blending.
-   Terrain tiling, LOD and streaming.
-   Quantised mesh and 3D Tiles terrain approaches where relevant.

Do not claim high-resolution terrain from a low-resolution elevation
source. Store source resolution and accuracy.

## 12. Building reconstruction

Research:

-   Microsoft building footprints and other open building datasets.
-   Google Open Buildings and current terms.
-   OSM building tags and completeness.
-   Footprint cleaning and polygon repair.
-   Building height estimation from tags, shadows, elevation, imagery,
    LiDAR or other evidence.
-   Procedural roof and facade generation.
-   Building typology classification.
-   Instance-level versus procedural building models.
-   Facade reconstruction from user-owned or properly licensed ground
    imagery.
-   Building segmentation and object detection models.

Separate: - Footprint known. - Height measured. - Height estimated. -
Roof inferred. - Facade observed. - Facade procedural. - Facade
AI-generated.

A building footprint does not establish its true height, floor count,
roof form or facade.

## 13. Road and transport infrastructure reconstruction

Research:

-   OSM road networks, tags, turn restrictions and connectivity.
-   Road graph topology and lane modelling.
-   Centreline-to-road-surface generation.
-   Junctions, roundabouts, ramps, bridges and tunnels.
-   Curbs, sidewalks, drainage, markings, signs and street furniture.
-   Road surface classification and damage/pothole reconstruction.
-   Road geometry from imagery, LiDAR, video and user capture.
-   Vehicle and pedestrian collision geometry.
-   Road-network validation and connectivity.

Do not assume a road centreline is a complete road model. Record which
road attributes are sourced versus inferred.

## 14. Photogrammetry, scanning and reconstruction

Investigate at minimum:

-   COLMAP.
-   AliceVision/Meshroom.
-   OpenMVG and OpenMVS.
-   Nerfstudio and Gaussian splatting pipelines.
-   Open3D.
-   Blender processing and mesh tooling.
-   Other current open-source or permissively licensed reconstruction
    projects.

For each, determine: - Whether it supports images, video, depth, LiDAR
or combinations. - Camera calibration and pose estimation
capabilities. - Dense reconstruction and mesh output. - Texture
generation. - GPU dependencies. - Headless operation and automation
interfaces. - Cross-platform availability. - Licence and model/data
restrictions. - Suitability for integration as an external worker.

Understand the difference between: - Point clouds. - Textured meshes. -
Heightfields. - NeRF representations. - Gaussian splats. - Semantic
scene graphs.

Do not assume a splat is an editable game-ready mesh. Research
conversion methods and quality loss.

## 15. Procedural generation

Research:

-   OSM2World.
-   CityEngine and CGA concepts.
-   Houdini procedural workflows.
-   Blender Geometry Nodes.
-   Existing road/building/terrain generators.
-   Rule systems, grammar systems and node-based generation.
-   Deterministic procedural generation and seed management.

Determine what Genesis should implement internally versus integrate. Pay
attention to topology, rule authoring, determinism, asset instancing and
incremental regeneration.

## 16. 3D rendering, scene and interchange formats

Evaluate:

-   glTF/GLB.
-   USD and USDZ where relevant.
-   OBJ, FBX and other interchange formats.
-   3D Tiles.
-   OpenUSD composition, variants and references.
-   Mesh, point-cloud and splat formats.
-   Material standards and texture compression.
-   Coordinate, scale and axis conventions.
-   Metadata preservation.

glTF/GLB is a strong candidate for an early interchange target, but it
is not automatically the best internal project format. Investigate USD
and a purpose-built World IR separately.

## 17. Desktop and viewport technology

Research options for:

-   Tauri and alternatives.
-   React/TypeScript desktop UI.
-   Three.js, Babylon.js, Bevy, wgpu and native viewport approaches.
-   GPU resource management and large-scene rendering.
-   Picking, selection, gizmos, snapping, hierarchy, layers and
    inspector UX.
-   Large-world precision.
-   Scene streaming and visibility.
-   Cross-platform packaging and auto-update.
-   Accessibility and keyboard workflows.

The desktop framework and viewport renderer are separate decisions. Do
not assume one dictates the other.

## 18. AI and computer vision

Research:

-   Image segmentation and object detection.
-   Building/road extraction.
-   Monocular depth and multi-view depth.
-   Image-to-3D and multi-view reconstruction.
-   Texture synthesis and inpainting.
-   Material recognition.
-   3D mesh generation/refinement.
-   Local inference runtimes.
-   Model quantisation and hardware compatibility.
-   Model licence and commercial-use restrictions.

Potential runtime ecosystems to investigate include ONNX Runtime,
Candle, Burn, llama.cpp where relevant, and other current inference
runtimes. Do not adopt a runtime simply because it is written in Rust.
Compare actual model coverage, performance, portability and operational
maturity.

Define an AI adapter contract so providers can be swapped. Include model
identity, version, checksum, input/output schema, device requirements,
licence, attribution, and inference settings.

## 19. Existing candidate projects to verify

The following URLs are research leads only. Verify current state,
licence, maintenance and technical capability before recommending use.

-   Microsoft Nigeria Building Footprints:
    https://github.com/microsoft/KenyaNigeriaBuildingFootprints
-   Google Open Buildings:
    https://sites.research.google/gr/open-buildings/
-   OpenStreetMap: https://www.openstreetmap.org/
-   OSM copyright/licence: https://www.openstreetmap.org/copyright
-   COLMAP: https://colmap.github.io/
-   AliceVision: https://alicevision.org/
-   OSM2World: https://osm2world.org/
-   Nerfstudio: https://docs.nerf.studio/
-   Cesium: https://cesium.com/
-   ArcGIS Reality:
    https://www.esri.com/en-us/arcgis/products/arcgis-reality/overview
-   Bentley iTwin Capture:
    https://www.bentley.com/en/products/itwin-capture-modeler/
-   ArcGIS CityEngine:
    https://www.esri.com/en-us/arcgis/products/arcgis-cityengine/overview
-   gen-map: https://gen-map.tarapata.net/en/
-   Mapforge: https://github.com/stonyp/mapforge
-   Houdini: https://www.sidefx.com/products/houdini/
-   RealityScan: https://www.realityscan.com/
-   Blender: https://www.blender.org/
-   Open3D: https://www.open3d.org/

Search for additional projects and newer alternatives. Include inactive
or abandoned projects only when they offer useful code, algorithms or
historical context, and clearly state their status.

## 20. Research deliverable format

Every research finding should be recorded in a structured table:

  Field            Description
  ---------------- ------------------------------------------------------
  Capability       The specific capability under investigation
  Candidate        Project, library, dataset, service or standard
  Primary source   Official URL/repository
  Licence          Verified licence and obligations
  Status           Active, maintained, experimental, inactive, unknown
  Platform         OS/architecture support
  Integration      API, library, CLI, service, file exchange
  Strengths        Evidence-backed advantages
  Limitations      Evidence-backed constraints
  Genesis fit      High, medium, low, uncertain
  Recommendation   Adopt, wrap, adapt, fork, build, defer, reject
  Confidence       High, medium, low
  Follow-up        Proof of concept, owner decision or further research

Do not collapse uncertain information into a definitive recommendation.

------------------------------------------------------------------------

# PART V --- PRODUCT REQUIREMENTS TO INVESTIGATE AND SPECIFY

## 21. Project and world management

The PRD must define:

-   Project creation and templates.
-   Geographic region and boundary management.
-   Project coordinate reference system.
-   Source data catalogue.
-   World layers and layer visibility.
-   Asset hierarchy and collections.
-   Project settings and target profiles.
-   Local file layout and cache management.
-   Project migration and schema compatibility.
-   Autosave, crash recovery and backups.
-   Import/export and project portability.

## 22. Geographic selection and data discovery

The UX should support:

-   Place-name search where a permitted geocoding provider is
    configured.
-   Coordinate entry.
-   Boundary drawing and editing.
-   Importing geographic boundaries.
-   Area size and coverage estimates.
-   Source availability and date information.
-   Licence and attribution visibility.
-   Resolution/accuracy information.
-   Preflight warnings for missing or incompatible inputs.

Do not make a provider mandatory if the user can supply local data. Do
not scrape restricted map products.

## 23. Generation controls

The system should eventually support selecting individual layers and
objects, for example:

-   Terrain.
-   Buildings.
-   Roads.
-   Lanes and junctions.
-   Sidewalks and curbs.
-   Bridges and tunnels.
-   Water and drainage.
-   Vegetation.
-   Streetlights and signs.
-   Billboards and street furniture.
-   Road markings.
-   Traffic infrastructure.
-   Other plugin-defined categories.

Generation should be selective. A user may want only a road network, or
only buildings within a selected district.

The system should identify dependencies. For example, generating a road
surface may require road edges, terrain alignment and junction handling.
Suggest dependencies, explain why they are required, and allow expert
overrides where safe.

## 24. Editing model

Investigate and propose a clear boundary between:

-   Lightweight corrections inside Genesis Studio.
-   Procedural parameter changes.
-   Regeneration of selected objects/layers.
-   Advanced modelling delegated to Blender or another DCC tool.
-   Round-trip editing and metadata preservation.

Do not attempt to rebuild Blender inside Genesis. The Studio should
focus on correcting, organising and validating reconstructed worlds.
Complex sculpting and modelling can be delegated to specialist tools.

## 25. Asset management

Define:

-   Asset identity and stable IDs.
-   Object hierarchy and parent/child relationships.
-   Geometry and material references.
-   Texture and material variants.
-   Collision meshes.
-   LOD chains.
-   Instancing.
-   Dependencies.
-   Provenance and licensing.
-   Generation parameters.
-   Source revision.
-   Asset version and content hash.
-   Selective regeneration.
-   Asset replacement and overrides.

## 26. Quality profiles

Research and propose explicit profiles, initially considering:

-   **Preview:** fast, approximate, low resource cost.
-   **Game-ready:** suitable geometry, collision, LOD and budgets for a
    defined target.
-   **High fidelity:** more detailed reconstruction with higher compute
    and storage costs.
-   **Survey/reference:** only available when source data and
    independent ground truth support explicit accuracy tolerances.

These names are provisional. Do not advertise survey-grade accuracy
without evidence.

Quality must be separated into dimensions:

1.  Geographic accuracy.
2.  Geometric accuracy.
3.  Visual fidelity.
4.  Structural completeness.
5.  Game/runtime readiness.
6.  Evidence quality and confidence.

Potential validation metrics include positional error, footprint
overlap, road-centreline deviation, height error, mesh topology,
degenerate triangles, normal consistency, UV validity, texture coverage,
reprojection error, point-cloud-to-mesh deviation, collision coverage,
LOD completeness, triangle/material/texture budgets, chunk size and
memory estimates.

Metrics must state their reference data and limitations. Image
similarity alone does not prove geographic accuracy.

## 27. Export and integrations

Research and prioritise:

-   Blender.
-   Unreal Engine.
-   Unity.
-   Godot.
-   glTF/GLB.
-   USD.
-   3D Tiles.
-   Other formats where there is a clear use case.

For each integration specify: - Supported versions. - Installation and
discovery. - Export/import direction. - Scene hierarchy mapping. -
Materials and textures. - Collision and LOD. - Coordinate conversion. -
Metadata and provenance retention. - Round-trip limitations. - Automated
validation fixtures.

Do not claim an integration is supported until it has an automated or
documented compatibility test.

------------------------------------------------------------------------

# PART VI --- PROPOSED TECHNICAL ARCHITECTURE TO VALIDATE

This section expresses design intent and candidate boundaries, not a
mandate to use a particular crate or framework. Research may change the
implementation choices.

## 28. Architectural shape

The likely system consists of:

1.  Desktop shell and UI.
2.  Viewport/rendering subsystem.
3.  Rust domain/core engine.
4.  Geospatial adapters.
5.  Reconstruction adapters and external workers.
6.  AI/model runtime adapters.
7.  Job orchestration and resource management.
8.  Project storage and asset cache.
9.  World IR and schema/versioning.
10. Quality/validation subsystem.
11. Export and integration adapters.
12. Plugin SDK and runtime.
13. CLI and headless execution, progressively.

The UI must communicate with the engine through explicit commands,
events and versioned data contracts. Avoid embedding core business logic
in React components.

## 29. Rust-first principle

Rust should be the primary language for core domain logic,
orchestration, data contracts and performance-sensitive processing where
it is a good fit.

Do not force all specialist processing into Rust. Mature C++, Python or
external tools may be wrapped behind stable adapters if that gives
Genesis better capability, quality or development speed.

Any non-Rust worker must have: - A documented reason to exist. - A
stable input/output contract. - Version detection. - Clear
installation/runtime requirements. - Cancellation and timeout behaviour
where possible. - Progress reporting. - Structured logs and error
mapping. - Licence and security review. - Integration tests and fixture
data.

Python may be used for optional model/reconstruction adapters,
experiments and ecosystem integration. It must not become an accidental
mandatory dependency of the core desktop application unless explicitly
approved.

## 30. World IR

Design a versioned, engine-agnostic internal representation for
reconstructed worlds.

The World IR should be evaluated for fields such as:

-   Stable world, layer, object and asset IDs.
-   Geographic position and CRS.
-   Local transform and coordinate frame.
-   Geometry and topology references.
-   Object type and semantic tags.
-   Dimensions and units.
-   Materials and textures.
-   Collision representation.
-   LOD references.
-   Parent/child and dependency relationships.
-   Source dataset and source feature IDs.
-   Source licence and attribution.
-   Reconstruction method.
-   Model/tool identity and version.
-   Confidence/evidence metadata.
-   Generation settings and random seed.
-   Content hashes.
-   Schema version.
-   Validation results and warnings.

The design must account for: - Large worlds and local-origin
precision. - Multiple CRS and vertical datums. - Stable IDs across
regeneration. - Partial updates. - Schema migrations. - Deterministic
output where possible. - External edits and overrides. - Export mapping
to different engines.

Do not store all geometry inline in a giant JSON document. Research an
appropriate split between structured metadata, binary geometry,
textures, indexes and project manifests.

## 31. Job orchestration

Design jobs as explicit, inspectable units with:

-   Job type and version.
-   Input references and hashes.
-   Parameters and random seed.
-   Resource requirements.
-   Local/remote execution choice.
-   Dependencies.
-   Progress stages.
-   Cancellation.
-   Retry policy.
-   Checkpointing where useful.
-   Logs and structured errors.
-   Output manifests.
-   Validation results.
-   Reproducibility metadata.

Jobs should support selective regeneration and caching. Avoid opaque
long-running tasks that cannot explain what they are doing.

## 32. Storage and caching

Research a storage model that separates:

-   Project metadata.
-   World/layer manifests.
-   Geometry and mesh data.
-   Textures and materials.
-   Source data references or permitted local copies.
-   Intermediate reconstruction artefacts.
-   Model files and caches.
-   Job records and logs.
-   Validation reports.
-   Export artefacts.

Consider content-addressed storage and deduplication, but measure
complexity and disk overhead. Define cache eviction and user-visible
disk usage controls.

A project should remain understandable and portable. Do not make the
only copy of important project state dependent on a hidden global cache.

## 33. Plugin architecture

Investigate a safe extension model. A candidate is Rust/WASM with
Wasmtime and a capability-based permission model, but validate
feasibility and ecosystem maturity before adopting.

Plugin requirements should consider:

-   Manifest and semantic version.
-   Plugin identity and publisher.
-   Supported Genesis API version.
-   Declared capabilities and permissions.
-   Input/output schemas.
-   Resource limits.
-   Cancellation.
-   Logs and progress.
-   Sandboxing.
-   Signing/trust policy.
-   Dependency resolution.
-   Compatibility checks.
-   Uninstall and rollback.

Plugins must not silently gain filesystem, network, process or
credential access. Require explicit capability grants.

## 34. Security and privacy

Threat-model:

-   Malicious plugins.
-   Malicious project files and archives.
-   Malicious model files.
-   External process execution.
-   Untrusted downloaded datasets.
-   Path traversal and archive extraction.
-   Resource exhaustion and decompression bombs.
-   Credential leakage.
-   Remote worker compromise.
-   Supply-chain attacks.
-   Prompt injection or hostile content in data/metadata consumed by AI
    agents.

Never place secrets in source code, prompts, logs, tickets or commits.
Use local secret stores or environment-based configuration as
appropriate.

## 35. Cross-platform and GPU strategy

Research a realistic support matrix for Windows, macOS and Linux.
Investigate:

-   CPU-only fallback.
-   NVIDIA CUDA.
-   Vulkan.
-   Metal.
-   DirectX where needed.
-   AMD and Intel GPU support.
-   Apple Silicon.
-   x86_64 and ARM64.
-   Driver and runtime detection.
-   Container support for workers.
-   Packaging and signing requirements.

Do not promise every backend on every platform until tested. Define
graceful capability detection and communicate when a feature is
unavailable.

------------------------------------------------------------------------

# PART VII --- ENGINEERING WORKFLOW AND TICKET SYSTEM

## 36. Work breakdown philosophy

Break the programme into the smallest meaningful, independently
verifiable units that provide useful progress.

Use the requested Matty Pocock workflow/skill only after verifying the
exact skill and its instructions. The ticket system below remains
mandatory regardless of the skill.

Ticket categories:

-   `research`
-   `feature`
-   `engineering`
-   `bug`
-   `refactor`
-   `security`
-   `performance`
-   `documentation`
-   `test`
-   `integration`
-   `release`
-   `spike`

Suggested priorities:

-   `P0` --- blocks safe project progress or critical architecture.
-   `P1` --- required for the next milestone.
-   `P2` --- important, but not currently blocking.
-   `P3` --- useful improvement or later scope.

Suggested states:

-   `proposed`
-   `needs-owner-input`
-   `ready`
-   `in-progress`
-   `in-review`
-   `changes-requested`
-   `verified`
-   `blocked`
-   `deferred`
-   `cancelled`

Do not use "done" as a synonym for "code written". `verified` means
acceptance criteria passed and independent review approved.

## 37. Mandatory ticket template

Every ticket must be a Markdown file with this structure:

``` markdown
---
id: GE-0001
title: Short imperative title
type: feature
priority: P1
status: proposed
milestone: M0
labels: []
depends_on: []
blocks: []
created: YYYY-MM-DD
updated: YYYY-MM-DD
owner_approval_required: false
---

# GE-0001 — Title

## Problem
What problem does this solve?

## User / engineering value
Why does this matter?

## Scope
### Included
- ...

### Excluded
- ...

## Context and research
- Relevant sources:
- Relevant ADRs:
- Related tickets:

## Proposed approach
Describe the intended outcome, not an unnecessarily rigid implementation.

## Acceptance criteria
- [ ] Observable criterion
- [ ] Observable criterion
- [ ] Tests and documentation updated

## Test plan
- Unit:
- Integration:
- End-to-end:
- Manual:
- Platform coverage:

## Security / licensing / data considerations
...

## Dependencies and risks
...

## Implementation notes
...

## Reviewer checklist
- [ ] Correctness
- [ ] Architecture and boundaries
- [ ] Error handling
- [ ] Tests
- [ ] Performance/resource use
- [ ] Security
- [ ] Licensing/provenance
- [ ] Documentation
- [ ] No unrelated scope creep

## Verification evidence
Commands, output summaries, fixture references and reviewer result.

## Changelog
- YYYY-MM-DD: ...
```

Ticket IDs must be stable and unique. Do not reuse IDs. If work is
split, create child tickets and preserve the parent relationship.

## 38. Ticket sizing

Prefer tickets that can be implemented and reviewed in a focused unit of
work. A ticket should generally: - Change one coherent capability. -
Have a small and understandable file/module footprint. - Be
independently testable. - Avoid mixing unrelated refactors with
features. - Have clear dependencies. - Be small enough that the reviewer
can understand the complete diff.

If a ticket is too large, split it before implementation. If it is too
small to provide meaningful verification, combine it with a closely
related task.

Do not create hundreds of fake tickets for hypothetical details. Create
a comprehensive backlog, but keep implementation tickets actionable and
evidence-backed.

## 39. Subagent responsibilities

Use subagents where supported. Discover the actual tool names,
capabilities and permission boundaries. Do not assume subagents can
access the same filesystem, browser, terminal, GitHub credentials or
context.

Suggested roles:

### Research agent

-   Investigates one bounded technology or capability.
-   Uses primary sources.
-   Produces evidence, licence and limitations.
-   Does not make final architecture decisions.

### Architecture agent

-   Evaluates system boundaries and design alternatives.
-   Identifies coupling, failure modes and migration risks.
-   Produces ADR proposals.
-   Does not implement without an approved ticket.

### Product/UX agent

-   Maps user journeys and interaction requirements.
-   Identifies usability risks for artists and developers.
-   Produces wireframe descriptions and acceptance criteria.

### Implementation agent

-   Implements one assigned ticket only.
-   Reads relevant project docs and ticket dependencies first.
-   Writes tests and updates documentation.
-   Reports changed files and validation evidence.
-   Does not mark its own ticket verified.

### Reviewer agent --- mandatory

An independent reviewer must inspect every code-changing ticket.

The reviewer must: - Read the ticket and acceptance criteria. - Read
relevant architecture docs and ADRs. - Inspect the full diff, not just
the summary. - Inspect tests and run relevant checks where possible. -
Look for correctness bugs, race conditions, edge cases, data corruption,
security issues, resource leaks, unbounded memory/CPU use,
coordinate/precision errors, licence violations, poor abstractions and
hidden coupling. - Check error paths and failure recovery. - Check
compatibility and migration impact. - Check whether tests meaningfully
prove behaviour. - Identify missing tests. - Return `APPROVED`,
`CHANGES_REQUESTED` or `BLOCKED`, with findings ranked by severity. -
Never approve based solely on the implementation agent's explanation.

A reviewer should not rewrite the implementation while reviewing unless
explicitly assigned a separate fix ticket. The implementation agent
addresses review findings, then the reviewer rechecks the revised diff.

If independent review cannot be run, the ticket remains `in-review` or
`blocked`; do not silently waive the requirement.

### QA/test agent

-   Creates test scenarios and fixtures.
-   Checks acceptance criteria.
-   Runs integration and cross-platform checks.
-   Reports reproducible failures.

### Documentation agent

-   Keeps docs aligned with actual behaviour.
-   Checks examples and commands.
-   Does not document planned features as if they already exist.

## 40. Ticket lifecycle

For each ticket:

1.  Read the ticket, project docs, relevant ADRs and dependency tickets.
2.  Confirm the ticket is `ready`.
3.  If ambiguous, stop and ask or create a research spike.
4.  Assign one implementation agent.
5.  Implement only the ticket scope.
6.  Add/update tests.
7.  Run formatter, linter, type checker, tests and relevant build.
8.  Update docs and changelog where required.
9.  Update ticket with files changed and evidence.
10. Assign independent reviewer subagent.
11. Reviewer returns findings.
12. If changes are requested, fix and repeat review.
13. Run final verification after all changes.
14. Update local ticket and `progress.md`.
15. Commit using the agreed commit convention.
16. Push and update the GitHub issue only when the repository workflow
    and approval allow it.
17. Mark `verified` only after every gate passes.

Never bundle multiple unrelated tickets into one commit merely to save
time. A commit may cover a parent ticket only when the ticket explicitly
describes a cohesive change.

## 41. Commit conventions

Use Conventional Commits unless research or owner preference indicates
otherwise.

Examples:

-   `docs: add Genesis Engine product specification`
-   `feat(core): add versioned world metadata model`
-   `feat(geodata): import GeoJSON boundaries`
-   `fix(geometry): handle invalid polygon rings`
-   `test(terrain): add deterministic mesh fixtures`
-   `refactor(jobs): isolate worker execution adapter`
-   `chore(ci): add cross-platform Rust checks`

Each commit should: - Correspond to a ticket or a clearly documented
setup task. - Pass relevant checks. - Avoid secrets and generated
data. - Include meaningful messages. - Be reviewable and revertible.

Do not use giant "initial implementation" commits containing the entire
product.

------------------------------------------------------------------------

# PART VIII --- PROJECT MEMORY, DOCUMENTATION AND REPORTING

## 42. Required repository documents

At minimum, maintain:

``` text
/
├── README.md
├── LICENSE
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── CHANGELOG.md
├── DECISIONS.md
├── PRD.md
├── ARCHITECTURE.md
├── ROADMAP.md
├── progress.md
├── GENESIS_ENGINE_SPEC.md
├── .gitignore
├── .editorconfig
├── .github/
│   ├── ISSUE_TEMPLATE/
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── workflows/
├── research/
│   ├── ECOSYSTEM_RESEARCH.md
│   ├── COMPETITIVE_ANALYSIS.md
│   ├── LICENCE_AND_DATA_RIGHTS.md
│   ├── FORMAT_AND_STANDARDS.md
│   └── RESEARCH_REGISTER.md
├── engineering/
│   ├── WORKFLOW.md
│   ├── ENVIRONMENT_REPORT.md
│   ├── ENGINEERING_REPORT.md
│   ├── TEST_STRATEGY.md
│   ├── SECURITY_MODEL.md
│   ├── PERFORMANCE_BUDGETS.md
│   └── RELEASE_PROCESS.md
├── adr/
│   └── 0001-template.md
├── TICKETS/
│   ├── README.md
│   └── GE-0001-example.md
├── docs/
│   ├── user/
│   ├── developer/
│   ├── architecture/
│   └── integrations/
├── apps/
├── crates/
├── integrations/
├── plugins/
├── tools/
├── tests/
├── fixtures/
└── scripts/
```

This is a suggested documentation and code layout, not an instruction to
create empty directories for every future subsystem. Research and
architecture should determine the actual code workspace structure. Do
not scaffold a large number of empty crates.

## 43. `progress.md` requirements

`progress.md` is the primary handoff document for the AI agent. It must
be concise enough to read at the start of every session, but detailed
enough to restore the exact project state.

Maintain these sections:

``` markdown
# Genesis Engine — Live Progress

## Current phase
## Current milestone
## Current objective
## Overall status
## Last updated

## Approved product decisions
## Approved architecture decisions
## Open owner questions

## Current repository state
- Repository URL:
- Branch:
- Latest commit:
- Working tree:
- Toolchain:
- Platforms tested:

## Completed and verified tickets
## In-progress tickets
## Blocked tickets
## Ready next tickets

## Current architecture snapshot
## Important module boundaries
## Key data contracts and schema versions

## Research completed
## Research still required

## Tests and verification
- Last full test run:
- Platform coverage:
- Known failures:
- Commands:

## Known bugs and technical debt
## Risks and assumptions
## Files/docs that must be read next

## Exact next action
## Session handoff notes
```

Rules: - Update it after each verified ticket and at the end of every
work session. - Do not delete history that explains important decisions;
use concise dated entries or link to reports. - Never say a feature is
complete unless it is verified. - Keep the next action explicit and
small. - Record blockers and the person/decision required to unblock
them. - Link to ticket files, GitHub issues, ADRs and reports. - Do not
paste huge logs into progress; link to or summarise detailed reports.

## 44. Engineering report

Maintain `engineering/ENGINEERING_REPORT.md` as an evolving engineering
assessment. It should cover:

-   Current architecture and implemented boundaries.
-   What is working and verified.
-   What is incomplete.
-   Quality of implementation and known limitations.
-   Performance measurements and benchmark methodology.
-   Cross-platform status.
-   Security and licence risks.
-   Dependency health and upgrade risks.
-   Technical debt.
-   Reliability and failure handling.
-   Test coverage and gaps.
-   Deviations from approved architecture.
-   Recommended engineering improvements.
-   Next engineering priorities.

Separate measured facts from opinions. Include dates, commit references,
commands and test environment.

For major milestones, create dated reports under `engineering/reports/`.

## 45. Session start and end protocol

At the beginning of every OpenCode session:

1.  Read `progress.md`.
2.  Read the current milestone and assigned ticket.
3.  Read relevant PRD, architecture sections and ADRs.
4.  Inspect `git status`, branch and recent commits.
5.  Confirm no uncommitted user work will be overwritten.
6.  Continue only from the documented next action.

At the end of every session:

1.  Update ticket states.
2.  Update `progress.md`.
3.  Update engineering report if architecture, quality or risks changed.
4.  Record tests and exact outcomes.
5.  Record open questions and blockers.
6.  Commit only verified, review-approved work.
7.  Provide a short handoff summary and next action.

------------------------------------------------------------------------

# PART IX --- QUALITY, TESTING AND DEFINITION OF DONE

## 46. Engineering quality standards

Code should be:

-   Correct and understandable.
-   Modular without excessive abstraction.
-   Explicit about ownership and lifetimes.
-   Resilient to malformed input.
-   Observable through structured logs and useful diagnostics.
-   Testable without requiring the entire desktop app.
-   Efficient for large datasets.
-   Safe around filesystem, process and network boundaries.
-   Documented at public API and architectural boundaries.
-   Consistent with project formatting and linting rules.

Avoid: - Giant modules and god objects. - Unbounded channels, queues or
caches. - Silent fallback that hides data loss. - Panics for expected
runtime errors. - Unclear coordinate or unit conventions. - Hidden
network access. - Untracked temporary files. - Global mutable state
without a strong reason. - Premature microservices. - Unnecessary
abstraction layers. - Copying large third-party code without licence
review. - Dead code and placeholder implementations presented as working
features.

## 47. Testing strategy

Define a layered strategy:

### Unit tests

-   Coordinate conversion.
-   Geometry operations.
-   Schema validation.
-   Deterministic procedural generation.
-   ID and hash behaviour.
-   Job state transitions.
-   Cache/index behaviour.

### Property and fuzz tests

-   Invalid polygon input.
-   Degenerate geometry.
-   Coordinate edge cases.
-   Large and malformed files.
-   Parser robustness.
-   Serialization round trips.

### Integration tests

-   Geospatial data import.
-   External worker adapters.
-   World IR persistence.
-   Selective regeneration.
-   Export pipelines.
-   Plugin boundaries.

### End-to-end tests

-   Create project.
-   Import sample region.
-   Generate a supported layer.
-   Inspect result.
-   Run validation.
-   Export and reopen in a target tool.

### Cross-platform tests

-   Windows.
-   macOS.
-   Linux.

### Performance tests

-   Increasing feature counts.
-   Large polygons and road graphs.
-   Memory use and peak allocation.
-   Geometry generation throughput.
-   Export time.
-   Viewport frame time.
-   Cache hit rate.
-   Chunk loading and unloading.

Use small, redistributable fixtures. Never check in large restricted
datasets or private imagery.

## 48. Definition of done

A code-changing ticket is verified only when:

-   Scope matches the ticket.
-   Acceptance criteria pass.
-   Relevant tests exist and pass.
-   Formatting and lint checks pass.
-   Type/build checks pass where applicable.
-   Error paths are handled.
-   Security and licence implications are reviewed.
-   Documentation is updated.
-   No unrelated changes are included.
-   Independent reviewer approves.
-   Verification evidence is recorded.
-   `progress.md` and ticket status are updated.
-   The commit is created according to project policy.

A ticket may be explicitly marked blocked or partially complete, but
must not be misrepresented as verified.

------------------------------------------------------------------------

# PART X --- BENCHMARK AND FIRST VERTICAL SLICE

## 49. Benchmark-first principle

Before claiming Genesis is better than existing solutions, build a
reproducible benchmark using the same geographic region and input data
for candidate tools.

A useful initial study area is a small, bounded section of Ikeja, Lagos,
approximately 500 metres to 1 kilometre across. The exact boundary must
be selected and recorded by the owner.

Potential comparison candidates include: - Genesis prototype. -
gen-map. - Mapforge. - OSM2World. - Relevant photogrammetry pipeline
where suitable source imagery exists.

Do not compare outputs from different source data and call the result an
objective benchmark.

Measure: - Geographic alignment. - Road-network coverage and
connectivity. - Building footprint coverage. - Height accuracy where
reference measurements exist. - Geometry validity. - Visual quality at
matched viewpoints. - Texture coverage and resolution. - Export
compatibility. - Runtime and memory. - Asset count, triangle count and
file size. - Ease of editing and selective regeneration.

Document unavailable ground truth. A benchmark without reference data
may compare completeness and structural quality, but cannot establish
real-world accuracy.

## 50. First vertical slice --- proposed, subject to research and owner approval

A candidate first vertical slice:

1.  Create a local Genesis project.
2.  Import a permitted small-area GeoJSON boundary.
3.  Import a small, documented OSM extract or equivalent fixture.
4.  Validate CRS and coordinate transformation.
5.  Generate a basic road network and simple building footprints/volumes
    from supported attributes.
6.  Create a structured World IR.
7.  Display the world in the desktop viewport.
8.  Select and inspect a road or building.
9.  Show source, method and confidence metadata.
10. Run basic geometry and structural validation.
11. Export a GLB or another researched interchange format.
12. Re-import or open the export in a verified target tool.
13. Record reproducible test and benchmark results.

This is deliberately limited. It validates the core architecture before
advanced photogrammetry, AI facade reconstruction, detailed street
objects, cloud workers or a full city pipeline.

Do not start this vertical slice until research has confirmed the
minimum architecture and the owner has approved the plan.

------------------------------------------------------------------------

# PART XI --- COST, PERFORMANCE AND RELEASE STRATEGY

## 51. Cost principles

Genesis should not require a paid account for local core functionality.

Research and report: - Minimum and recommended CPU/RAM/storage. - GPU
requirements by capability. - Model storage sizes. - Dataset sizes and
download costs. - Estimated processing time by area and layer. - Peak
memory and disk usage. - Cloud compute alternatives and approximate cost
ranges. - Optional hosted AI costs and provider limits.

Do not fabricate precise estimates. Use benchmarks or clearly labelled
assumptions and ranges.

Provide a preflight estimate before expensive operations: - Approximate
data volume. - Approximate storage required. - Required model
downloads. - CPU/GPU suitability. - Estimated processing range. -
Potential external API or cloud charges.

Require explicit user confirmation before a job incurs paid external
costs.

## 52. Release strategy

Research and propose release stages, for example:

-   Research and architecture prototype.
-   Core data and geometry foundation.
-   First end-to-end vertical slice.
-   Early developer preview.
-   Public alpha.
-   Public beta.
-   Stable release.

Each release must have: - Scope and non-goals. - Supported OS/platform
matrix. - Known limitations. - Installation and migration notes. - Test
evidence. - Security/licence review. - Reproducible build information
where practical.

Do not announce stable support for untested features.

------------------------------------------------------------------------

# PART XII --- OPEN-SOURCE GOVERNANCE AND GITHUB

## 53. Repository principles

Use GitHub as the public engineering source of truth after owner
approval.

The repository should contain: - Product and architecture
documentation. - Research and ADRs. - Source code. - Tests and small
fixtures. - Contribution guidance. - Security reporting instructions. -
Issue and PR templates. - Roadmap and release notes.

Avoid committing: - Credentials or tokens. - Proprietary imagery or
datasets. - Large model weights unless explicitly licensed and
approved. - Generated city-scale output. - Personal data. - Temporary
caches and build artefacts.

## 54. Licence research

Apache-2.0 is a candidate for the core because of its express patent
grant; MIT is another candidate. Do not select a licence until
dependencies, contributions, model assets and intended ecosystem are
reviewed with the owner.

Treat these as separate licence questions: - Core source code. - Plugin
SDK and examples. - Documentation. - Test fixtures. - Sample generated
assets. - Model weights. - Third-party datasets. - User-generated
project content.

Do not assume that an open-source processing tool makes its input
datasets or model weights open for all uses.

## 55. GitHub issue management

After owner approval: - Create labels for type, priority, area,
milestone and status. - Create milestones aligned with the roadmap. -
Create issues from approved ticket files. - Include ticket ID in issue
title. - Link local ticket file and relevant ADRs. - Keep GitHub and
local ticket status synchronised. - Avoid creating duplicate issues. -
Use issue dependencies where supported; otherwise document dependencies
clearly.

Suggested labels: - `type:feature` - `type:bug` - `type:research` -
`type:engineering` - `type:security` - `type:documentation` -
`priority:P0` - `priority:P1` - `priority:P2` - `priority:P3` -
`status:ready` - `status:blocked` - `area:core` - `area:geospatial` -
`area:geometry` - `area:studio` - `area:viewport` - `area:ai` -
`area:quality` - `area:export` - `area:plugins` - `area:infra`

Use labels consistently and do not create dozens of overlapping labels.

------------------------------------------------------------------------

# PART XIII --- HOW TO RESPOND TO THE PROJECT OWNER

## 56. Communication expectations

Be a candid technical collaborator.

When presenting a decision: - State the recommendation first. - Explain
why. - Present meaningful alternatives. - Explain trade-offs and
consequences. - Identify what is known and unknown. - Ask one focused
question when a decision is required.

Do not bury the owner in generic technical jargon. Explain unfamiliar
3D/geospatial concepts accurately, using practical examples.

Do not agree with every idea automatically. Challenge technically
unsound assumptions respectfully and explain the evidence.

Do not present an enormous unprioritised list of possible technologies.
Narrow choices using explicit criteria.

## 57. Required first response after reading this file

Your first response must:

1.  Confirm that you have read the specification.
2.  Summarise the six approved foundational decisions.
3.  Report the repository and environment state.
4.  Report whether the requested Matty Pocock skill was found and
    verified.
5.  Explain the research plan and the subagents you can actually use.
6.  Identify the most consequential unresolved decisions.
7.  Ask the first small batch of owner questions, if necessary.
8.  State explicitly that you have not begun product implementation.
9.  Give the exact next step.

Do not claim to have inspected or researched anything until you actually
have.

------------------------------------------------------------------------

# APPENDIX A --- INITIAL DECISION REGISTER

  -----------------------------------------------------------------------
  ID                      Decision                Current state
  ----------------------- ----------------------- -----------------------
  D-001                   Product identity:       Approved
                          engine and platform,    
                          progressively           

  D-002                   Desktop-first visual    Approved
                          Studio                  

  D-003                   Local-first, cloud      Approved
                          optional                

  D-004                   Open models plus        Approved
                          external AI providers   

  D-005                   Game developers and     Approved
                          technical artists       
                          equally                 

  D-006                   Windows, macOS and      Approved
                          Linux initial targets   

  D-007                   Full platform           Open
                          boundaries: Studio,     
                          Core, SDK, CLI,         
                          headless                

  D-008                   CLI release timing      Open

  D-009                   First engine/DCC        Open
                          integration             

  D-010                   World IR and project    Research required
                          storage                 

  D-011                   Viewport technology     Research required

  D-012                   GPU backend strategy    Research required

  D-013                   First-release layer     Owner approval required
                          scope                   

  D-014                   Core and plugin licence Research and owner
                                                  approval

  D-015                   Cloud execution model   Open

  D-016                   Collaboration and asset Defer unless research
                          marketplace scope       changes architecture
  -----------------------------------------------------------------------

# APPENDIX B --- RESEARCH AND DECISION QUALITY CHECKLIST

Before recommending a major technology:

-   [ ] Primary documentation inspected.
-   [ ] Official repository inspected.
-   [ ] Licence verified from authoritative source.
-   [ ] Current maintenance/release state checked.
-   [ ] Platform support verified.
-   [ ] Integration surface understood.
-   [ ] Known limitations recorded.
-   [ ] Alternatives compared.
-   [ ] Prototype or evidence gathered where feasible.
-   [ ] Recommendation and confidence recorded.
-   [ ] Owner approval obtained if decision is strategic or hard to
    reverse.

Before approving an architectural decision:

-   [ ] Problem and constraints stated.
-   [ ] At least two viable options considered where applicable.
-   [ ] Reuse versus build decision explained.
-   [ ] Performance and resource implications considered.
-   [ ] Cross-platform implications considered.
-   [ ] Security and licence implications considered.
-   [ ] Migration and reversibility considered.
-   [ ] Consequences documented in an ADR.
-   [ ] Owner approval recorded where required.

Before verifying a ticket:

-   [ ] Ticket acceptance criteria pass.
-   [ ] Tests added or updated.
-   [ ] Relevant checks run.
-   [ ] Full diff reviewed independently.
-   [ ] Review findings resolved.
-   [ ] Documentation updated.
-   [ ] No unapproved scope expansion.
-   [ ] Evidence recorded.
-   [ ] Progress and ticket state updated.
-   [ ] Commit is clean and attributable.

# APPENDIX C --- FINAL INSTRUCTION

Genesis Engine is intended to become a serious, long-lived open-source
engineering platform. Treat its architecture, data contracts,
provenance, testing and developer experience as foundational product
features.

Do not rush into scaffolding. Do not produce a generic CRUD application
with a 3D viewport attached. Do not invent capabilities or rely on
marketing claims. Do not implement a large speculative architecture.

Research deeply, plan carefully, ask the owner when decisions matter,
decompose work into verifiable tickets, implement incrementally, and
independently review every code-changing ticket.

**The goal is not to generate code quickly. The goal is to build the
right system, with evidence, discipline and a codebase that can be
trusted and extended.**
