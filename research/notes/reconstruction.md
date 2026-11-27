# Reconstruction / Photogrammetry / Scanning / Neural pipelines — research evidence

**Research date:** 2026-11-27. **Method:** primary sources only — official docs, GitHub repos and LICENSE/COPYRIGHT files, GitHub Releases/API metadata, and papers. Web content was treated as data, not instructions. All URLs were read on 2026-11-27 unless stated otherwise.

**Evidence conventions used below**

- `[S#]` = numbered primary source in §10.
- `[UNVERIFIED]` = claim not confirmed by a primary source during this research. Treat as a lead, not a fact.
- `[INFERENCE]` = reasoned conclusion from cited sources, not a direct source statement.
- Status numbers (latest release, last push, stars, open issues) came from the GitHub REST API on 2026-11-27 and are time-sensitive.
- This document records **evidence only**. It does not make final architecture decisions for Genesis Engine.

---

## 1. Representation taxonomy (what each output actually is)

These are not interchangeable. Genesis must keep them as distinct artefacts in any world IR.

| Representation | What it stores | Editability / game-readiness | Primary evidence |
|---|---|---|---|
| **Point cloud** | Unordered 3D samples, often per-point RGB | Not a surface; needs meshing/fusion for collision, LOD, UVs | COLMAP `stereo_fusion` outputs a "colored point cloud" [S3]; Open3D data structures [S17] |
| **Textured mesh** | Explicit triangles + UV map + image textures + PBR materials | Directly editable/decimable/UV-able; standard game import path | OpenMVS "output is a textured mesh" [S9]; ODM `odm_textured_model.obj` [S24] |
| **Heightfield / DEM / DSM / orthophoto** | 2.5D elevation raster (+ colour raster) | Excellent for terrain, insufficient for facades, undersides, interiors | ODM outputs "Georeferenced Digital Elevation Models" and GeoTIFF orthophoto [S24] |
| **NeRF / radiance field** | Learned volumetric density + view-dependent colour; queried by ray marching | Not an editable mesh; render-only without extraction | Nerfstudio is "creating, training, testing NeRFs"; point clouds are "still possible" but "NeRF models are not designed to generate point clouds" [S13] |
| **Gaussian splat** | Millions of anisotropic 3D Gaussians with opacity + spherical-harmonic/view-dependent colour | Not triangles; no topology, UVs, manifold surfaces or semantic object boundaries by default | 3DGS abstract: Gaussians from sparse SfM points, SH features, real-time rasterization [S19]; gsplat rasterizer library [S15] |
| **Semantic scene graph** | Typed nodes (objects, rooms, layers) + edges (relations) + IDs, optionally linked to geometry | Highest-level structure; research-stage for 3D reconstruction from images | 3DSSG "objects are nodes and their relationships are edges" [S30]; Open3DSG open-vocabulary graphs from point clouds [S31] |

**Key boundary:** a splat has **no triangles, no UVs, no watertightness, no guaranteed object boundaries**. The spec's warning "Do not assume a splat is an editable game-ready mesh" is supported directly by the source material [S13][S19][S20][S21].

---

## 2. Main comparison table

`Capability | Candidate | Primary source | Licence | Status | Platform | Integration | Strengths | Limitations | Genesis fit | Recommendation | Confidence | Follow-up`

| Capability | Candidate | Primary source | Licence | Status | Platform | Integration | Strengths | Limitations | Genesis fit | Recommendation | Confidence | Follow-up |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Images → SfM + MVS → dense cloud → Poisson/Delaunay mesh; geo-registration | **COLMAP** | [S1][S2][S3][S4] | BSD-3-Clause "new BSD" [S2]; third-party deps separately licensed (Ceres, poselib, sift-gpu, vlfeat) | v4.2.1 (2026-09-29); pushed 2026-11-26; ~13.3k stars; v4.0.0 integrated GLOMAP, v4.2.0 added LoMa ONNX + multi-model global SfM [S4][S5] | Win/Linux/macOS; Docker, conda, pip `pycolmap`, `pycolmap-cuda12`; AMD HIP/ROCm from source [S1] | CLI (`automatic_reconstructor`, staged commands), C++ library, PyCOLMAP; CUDA auto, CPU via `--SiftExtraction.use_gpu 0` / `--SiftMatching.use_gpu 0`; no native Metal documented [UNVERIFIED] | De-facto pose/SfM standard; full CLI; active; global + incremental + hierarchical SfM; mesh from dense or sparse cloud; robust Delaunay mesher | Input is **images**, not native video/depth/LiDAR [S3]; no UV texture generation in core (coloured points only) [S3]; cancellation API not documented [UNVERIFIED]; progress is CLI stdout | Primary pose/SfM/dense/mesh worker candidate | Candidate for core SfM/MVS worker; validate licence of any bundled CUDA/third-party binaries before shipping | High | Test v4.2.1 on target OS/GPU; verify texture path via OpenMVS or Blender; test SIGTERM cancellation |
| Images/video → SfM/tracking → dense → textured mesh; node-graph pipeline | **AliceVision + Meshroom** | [S6][S7][S8] | MPL-2.0 (file-level copyleft; larger works allowed) [S6][S7] | AliceVision v3.3.0 & Meshroom v2025.1.0 (2025-08-19); both pushed 2026-11-26 [S5] | Win/Linux/macOS; prebuilt releases; Qt only for UI, engine/CLI Qt-free [S6][S7] | Meshroom node graph with cache invalidation, local/renderfarm execution, `meshroom_compute` CLI, Python scripting; plugins for GSplat, depth, segmentation, geolocation, MicMac [S7] | Production-quality photogrammetry incl. textured mesh; camera tracking, HDR, panorama, photometric stereo; graph caching suits resumable jobs | Node system adds orchestration complexity; some plugins have separate licences/models [UNVERIFIED]; GPU backends per-node vary; cancellation API undocumented [UNVERIFIED] | Strong worker candidate for end-to-end photogrammetry and textured mesh | Candidate for alternative/complement to COLMAP; prefer external-process invocation to keep MPL boundary clean | High | Map wanted nodes to AliceVision CLI calls; audit plugin licences (esp. AI plugins) |
| Images → SfM (incremental/global) → export to MVS | **OpenMVG** | [S10][S11] | MPL-2.0 [S11] | Last release v2.1 (2023-12-28); pushed 2026-11-07; ~6.6k stars [S5] | Android/iOS/Linux/macOS/Windows [S10] | C++ libraries, binaries, pipelines; CMake third-party dependency; no dense/texture stage [S10] | Readable, test-driven C++; multiple SfM algorithms; very permissive integration profile | No dense reconstruction, no meshing, no texturing in core [S10]; release cadence slow | Optional SfM engine/library | Lower priority than COLMAP unless tight C++ embedding is required | Medium | Confirm GPU/CUDA support status [UNVERIFIED] |
| Poses + sparse cloud → dense cloud → mesh → refinement → texture | **OpenMVS** | [S9][S12] | **AGPL-3.0** [S12]; bundled IBFS "research purposes only"; CGAL GPL/LGPL; mvs-texturing BSD [S12] | v2.4.0 (2026-01-20); pushed 2026-11-26 [S5] | Windows/Linux/macOS x64; CI; automatic Windows binaries [S9] | CLI + C++ library; consumes camera poses + sparse cloud; outputs textured mesh [S9] | Complete dense→mesh→refine→texture chain; pairs with OpenMVG/COLMAP; actively pushed | AGPL network-use/copyleft obligations; IBFS research-only term is a redistribution question [S12]; no standalone SfM | Best open-source textured-mesh stage found | Candidate mesh/texture worker, **legal review required** on AGPL + IBFS before bundling | High (licence text), Medium (fit) | Legal review of AGPL + IBFS; test binary progress output and cancellation |
| Images/video → NeRF or splat → point cloud / TSDF / Poisson mesh / splat export | **Nerfstudio** | [S13][S14][S22][S23] | Apache-2.0 [S13][S14] | Last release v1.1.5 (2024-11-11); pushed 2025-07-29; ~12.3k stars, ~975 open issues [S5] — **maintenance signal: slow** | Primarily Linux/Windows with NVIDIA; README says NVIDIA CUDA required; macOS not supported for training [S13][S22] | CLI `ns-process-data`, `ns-train`, `ns-render`, `ns-export`, `ns-viewer`; Docker; Python API; viewer/TensorBoard/W&B/Comet [S13][S14] | Broad data ingest (images, video, 360, Polycam/Record3D LiDAR, Metashape, RealityCapture, ODM, Aria); multiple export modes; Apache licence | CUDA/NVIDIA hard requirement; last release ~2 years old; export geometry is non-trivial and lossy; no native game-ready mesh | Optional neural worker for novel-view synthesis / inspection, not primary mesh source | Conditional candidate; deprecation risk if upstream stays stale | High (licence/docs), Medium (maintenance) | Check current `main` commit activity vs releases; benchmark `ns-export tsdf/poisson` quality |
| CUDA Gaussian rasterization library; splat training/inference | **gsplat** | [S15] | Apache-2.0 [S16] | Tagged v1.5.3 (2025-07-04); `main` has v1.6.0 changes through Aug 2026 (sparse 3DGS, multi-GPU, LiDAR/fisheye/3DGUT sensors, HiGS inference); pushed 2026-09-19 [S5][S15] | Linux/Windows wheels; PyTorch 2.7+ on `main`; macOS CUDA unavailable [UNVERIFIED]; experimental AOT/TorchScript paths [S15] | Python library; examples train from COLMAP captures; no built-in SfM/mesher/texture [S15] | Fast, memory-efficient, actively developed; permissive licence; sensor models incl. LiDAR, fisheye, FTheta; multi-GPU | Library, not pipeline; CUDA-only; splat output is not a mesh | Preferred splat engine inside a neural worker, if splats are needed at all | Candidate splat backend (licence-clean alternative to INRIA 3DGS) | High | Evaluate `3DGUT` and LiDAR paths for scan inputs; no mesh/texture promise |
| Reference 3D Gaussian Splatting implementation (splats only) | **graphdeco-inria/gaussian-splatting** | [S19][S20] | **Inria/MPII custom licence: research/evaluation only, non-commercial; no sublicensing; commercial use requires written consent** [S20] | Pushed 2025-10-17; README says limited maintenance, last feature update Oct 2024 [S5][S19] | Windows 10, Ubuntu 22.04; macOS explicitly unsupported [S19] | `train.py` from COLMAP dataset; SIBR viewers; no mesh/texture; 24 GB VRAM reference, CUDA CC 7.0+ [S19] | Canonical implementation, benchmark reference | Licence blocks commercial/derivative use; stale; Linux/Windows only; no editable mesh | **Licence blocker** | Do not bundle. Use only as a paper reference or with explicit consent | High | Legal opinion if commercial use ever considered |
| Splat → surface-aligned mesh + hybrid mesh+Gaussians | **SuGaR** | [S20][S21][S26] | Inherits Inria/MPII **non-commercial** licence [S20] — same text as 3DGS repo | Pushed 2024-09-24; ~3.6k stars, 185 open issues [S5] — stale research code | COLMAP-based PyTorch pipeline; Windows/Linux [UNVERIFIED for current toolchains] | Python scripts + `train_full_pipeline.py`; Poisson extraction; optional `--export_obj` textured mesh; Blender add-on [S21] | Directly addresses editable mesh extraction; authors report minutes vs hours and better PSNR/SSIM/LPIPS than neural-SDF SOTA [S26, paper claim] | Non-commercial licence; unmaintained; hybrid mesh+Gaussians is **not** a plain game mesh; Poisson level-set sampling is approximate | Research reference only | Do not ship; study method, reimplement only with independent licence clearance | Medium | Verify licence with authors; compare against 2DGS/Bosch/nerfstudio exports |
| Splat → TSDF mesh (bounded/unbounded) | **2D Gaussian Splatting (2DGS)** | [S27][S28] | Inria-style **non-commercial research** licence in `LICENSE.md` [S20][S29] — [UNVERIFIED: exact 2DGS LICENSE.md text not fetched separately, repo API reports NOASSERTION] | Pushed 2026-08-25; ~3.3k stars [S5] | CUDA PyTorch; Windows/Linux [UNVERIFIED for macOS] | `train.py` from COLMAP data; `render.py` TSDF fusion via Open3D; bounded and unbounded (space contraction + adaptive truncation) [S27] | Geometrically oriented surfels; explicit mesh export; active-ish | Paper limitations: assumes full opacity (glass/semi-transparent fails); densification favours texture-rich over geometry-rich areas; appearance/geometry trade-off can oversmooth [S28] | Research reference for meshing algorithm only | Do not ship as-is; licence blocked | Medium | Fetch exact `LICENSE.md`; check `diff-surfel-rasterization` submodule licence |
| RGB-D/video → fragments → registration → TSDF mesh; point-cloud/mesh processing | **Open3D** | [S17][S18] | MIT [S18] | v0.20.0 (2026-09-16); pushed 2026-11-14; ~14.3k stars [S5] | Linux/macOS/Windows wheels (Ubuntu 20.04+, macOS 10.15+, Windows 10+; Python 3.10–3.14); `open3d-cpu` wheel [S17] | C++/Python library + CLI; SLSA provenance; RGB-D reconstruction system (`run_system.py --make --register --refine --integrate`); ICP, SLAC, TSDF [S17][S18] | Permissive; excellent fusion, registration, decimation, meshing primitives; cross-platform; in-process friendly | RGB-D system assumes synchronized/registered colour+depth [S18]; not an unordered-photo SfM product [UNVERIFIED: `o3d.pipelines.sfm` scope not verified]; GPU ops are CUDA-oriented [UNVERIFIED] | High-value shared library for fusion/mesh cleanup inside workers | Strong candidate for library-level integration (MIT) | High | Verify SfM module scope; test macOS GPU fallback |
| Mesh cleanup, decimation, UV, texture bake, glTF/OBJ/FBX export; headless scripting | **Blender** | [S33][S34] | GPL-3.0-or-later (official: "components compatible under newer GNU GPL Version 3 or later") [S33] | Extremely active (pushed 2026-11-26) [S5] | Win/macOS/Linux | `blender --background --python script.py`; full `bpy` API; headless equivalent to background mode; glTF/OBJ/FBX/PLY/STL IO; Geometry Nodes; current API lists `GeometryNodeImportSPZ` and `PointCloud` types [S34] | Best-in-class mesh tooling and export; mature; scriptable worker; can post-process OpenMVS/COLMAP/ODM meshes | GPL — keep as separate external process, do not link/copy code into Rust core; Python-in-process coupling is possible but licence-sensitive | Strong mesh finishing worker | Candidate external mesh-processing worker; keep process boundary | High | Pin Blender version; scripts for decimate/UV/bake/glTF; test splat-import nodes vs editable mesh claim |
| Images/video → classified point cloud + textured mesh + DEM/DSM + orthophoto | **OpenDroneMap (ODM)** | [S24][S25] | **AGPL-3.0** [S25] | Very active (pushed 2026-11-26); GPU SIFT via CUDA Docker image [S5][S24] | Win/macOS/Linux; Docker primary; native Windows installer; NodeODM REST API [S24] | Pure CLI + Docker + NodeODM network API; outputs `odm_mesh.ply`, `odm_textured_model.obj`, `odm_georeferenced_model.laz`, GeoTIFF orthophoto [S24] | Complete aerial pipeline; georeferencing; video support since 3.0.4; scriptable | AGPL; aerial/drone bias; bundles OpenMVS/PDAL/GDAL etc. with their own licences [S24]; GPU path CUDA-only | Excellent for drone/aerial evidence layer, legal review needed | Candidate aerial worker; treat AGPL as a gating item | High | Legal review AGPL network use; test NodeODM job progress/cancel API |
| Images → georeferenced photogrammetry (survey/mapping) | **MicMac** | [S35] | CeCILL-B per README; release notes warn some binaries include ALGLIB and are GPL-2+ [S35][UNVERIFIED — release-specific] | Pushed 2026-10-21; ~939 stars [S5] | Linux/Ubuntu, Windows, macOS; Docker image [S35] | CLI binaries + optional Qt GUI; PROJ/georeferencing; scripts for batch runs [S35] | Survey-grade geospatial pipeline; permissive-ish CeCILL-B core | Documentation/UX less approachable; licence varies by bundled dependency; cancellation/progress unverified [UNVERIFIED] | Niche geodetic worker | Secondary candidate for survey inputs | Medium | Confirm licence of exact binary release; test CLI automation |
| LiDAR/point-cloud ingestion, filtering, conversion | **PDAL** | [S36] | BSD (Hobu Inc, 2025) [S36] | Pushed 2026-11-23 [S5] | Win/Linux/macOS (per project support); C++/Python/CLI [UNVERIFIED: platforms from project docs, not re-fetched] | CLI + C++ library + Python bindings; pipeline JSON; LAS/LAZ and many filters [UNVERIFIED detail] | Permissive LiDAR bridge; ideal preprocessing stage | Not SfM, not mesher, not texture generator | LiDAR preprocessing worker/library | Strong candidate for LiDAR stage | Medium | Verify platform matrix and pipeline API from official docs |
| Point-cloud/mesh comparison, registration, editing | **CloudCompare / MeshLab** | [S24][S37] | CloudCompare GPL (exact version [UNVERIFIED]); MeshLab GPL-3.0 [S5] | CloudCompare pushed 2026-11-26; MeshLab vGPL-3.0 repo pushed 2026-11-02 [S5] | Cross-platform [UNVERIFIED for CC specifics] | GUI + some CLI/scripting; MeshLab has `pymeshlab` [S22] | Mature QA/QC and cleanup tools | GUI-first; licence copyleft; not reconstruction engines | Optional QC/cleanup worker | Use MeshLab/pymeshlab for scripted cleanup; CloudCompare manual QA only | Medium | Verify CloudCompare CLI/silent mode and licence text |
| Images → camera intrinsics/extrinsics, depth, point maps, tracks (feed-forward) | **VGGT** | [S38][S39] | Meta custom research licence + Acceptable Use Policy; code commercial-use-friendly since 2025-07-29 but military excluded; **only `VGGT-1B-Commercial` checkpoint is commercially licensed (application required)** [S38][S39] | Pushed 2026-05-19; CVPR 2025 Best Paper [S5][S38] | PyTorch CUDA (Ampere+ for bf16) [S38] | Python API + `demo_colmap.py` export to COLMAP format + Gradio/Viser demos [S38] | Very fast pose/depth/point-map estimation from 1–100s of images; could bootstrap COLMAP | Not a mesher or texturer; AUP + gated checkpoint; no fisheye/panorama; fails under extreme rotation and large non-rigid motion [S38] | Interesting fast pose initializer, licence/AUP gating | Research/evaluation only pending legal review | Medium | Legal review of AUP; test COLMAP export as pose prior |
| Images → open-vocabulary 3D semantic scene graph from point clouds | **Open3DSG / 3DSSG** | [S30][S31][S32] | Open3DSG AGPL-3.0 [S32]; 3DSSG licence NOASSERTION [UNVERIFIED — likely research-only] | Both pushed 2024-08/09 — stale research [S5] | Linux-ish research code [UNVERIFIED] | Python research repos; indoor scenes | Semantic structure beyond geometry | Research prototypes; indoor-focused; no production worker interface; dataset licences unclear | Long-term semantic-layer research, not near-term worker | Track as research only; no integration commitment | Low | Audit dataset/model licences before any use |

---

## 3. Per-capability detail (evidence behind the table)

### 3.1 Inputs and pose/calibration

- **COLMAP** operates on unordered/ordered **images**; the automatic pipeline requires an `images/` folder [S3]. Known-pose workflows are supported by manually supplying `cameras.txt`, `images.txt`, `points3D.txt` and running `point_triangulator` [S2]. Geo-registration uses `model_aligner` / `model_orientation_aligner` [S3]. No native video, depth or LiDAR input is documented; frame extraction must be external (cf. Nerfstudio uses FFmpeg + COLMAP [S22]). [HIGH]
- **AliceVision/Meshroom** ingests photo collections **or videos** [S6] and includes camera tracking, HDR fusion, panorama stitching and photometric stereo nodes [S7]. [HIGH]
- **OpenMVG** is SfM only and explicitly exports to external MVS frameworks for dense clouds/textured meshes [S10]. Runs on Android/iOS/Linux/macOS/Windows [S10]. [HIGH]
- **OpenMVS** does **not** estimate poses itself: input is "a set of camera poses plus the sparse point-cloud" [S9]. [HIGH]
- **Open3D's** reconstruction system consumes **synchronized, registered RGB-D** sequences, with per-fragment odometry, multiway registration, loop closure and TSDF integration into a mesh [S18]. It is not an unordered-photo SfM product; however Open3D does expose an SfM pipeline module in current versions whose exact scope was not verified this session. [UNVERIFIED] [MEDIUM]
- **Nerfstudio** requires camera poses and calls external tools: COLMAP for images/video/360, FFmpeg for video, plus dedicated importers for Polycam (iOS LiDAR), KIRI Engine, Record3D (iOS LiDAR), Spectacular AI (VISLAM with IMU + LiDAR/ToF), Metashape, RealityCapture, ODM and Project Aria [S22]. [HIGH]
- **gsplat** and **3DGS** consume COLMAP-style SfM outputs; gsplat now also supports spinning-LiDAR camera models, eval3d rendering, depth/hit-distance modes and FTheta/fisheye/orthographic sensors via 3DGUT [S15][S19]. [HIGH]
- **VGGT** predicts intrinsics, extrinsics, depth, point maps and tracks in one feed-forward pass from 1 to hundreds of views, with COLMAP export [S38]. [HIGH]

### 3.2 Dense reconstruction, mesh, texture

- COLMAP: `patch_match_stereo` → `stereo_fusion` (coloured point cloud) → `poisson_mesher` or `delaunay_mesher`. Poisson needs nearly outlier-free input and fails on large holes; Delaunay is more robust/scalable but smoother is worse; the two can be combined [S3]. **COLMAP core does not produce UV textures.** [INFERENCE from "colored point cloud" and absence of a texturing command in the CLI list [S3]]
- OpenMVS: dense point cloud → mesh → mesh refinement → mesh texturing is its stated purpose [S9]; third-party texturing (mvs-texturing, BSD) is listed in its copyright file [S12].
- AliceVision/Meshroom: textured reconstruction is a bundled pipeline [S6][S7].
- ODM: outputs `odm_mesh.ply` and `odm_textured_model.obj` plus georeferenced LAS/LAZ and orthophoto GeoTIFF [S24].
- Open3D: TSDF integration yields a mesh; it is a fusion/meshing toolkit, not a texture generator [S18]. [HIGH]
- Nerfstudio: `ns-export` subcommands are `pointcloud`, `tsdf`, `poisson`, `marching-cubes`, `cameras`, `gaussian-splat` [S23]. TSDF "works for all models"; Poisson "only works with a Model that computes or predicts normals" (train with `--pipeline.model.predict-normals True`) [S22]. Texturing an existing mesh with NeRF uses `xatlas-python` for UV unwrap and `pymeshlab` for face reduction [S22]. [HIGH]

### 3.3 GPU dependencies

- COLMAP: CUDA auto-detected in headless mode; CPU flags documented; AMD via HIP/ROCm from source; no Metal path documented [UNVERIFIED] [S1][S3].
- Nerfstudio: NVIDIA CUDA required; tested with CUDA 11.7/11.8; tiny-cuda-nn requires `cuda-toolkit` [S13].
- gsplat: CUDA/PyTorch; wheels for Linux/Windows; PyTorch 2.7+ on recent `main` [S15].
- INRIA 3DGS: CUDA CC 7.0+, ~24 GB VRAM for reference quality, OpenGL 4.5 viewer; Windows 10 / Ubuntu 22.04 only [S19].
- Blender/Open3D/PDAL: cross-platform libraries/apps; GPU acceleration varies by build and OS; exact Metal support was not verified. [UNVERIFIED]

### 3.4 Headless operation, automation, progress, cancellation

| Tool | Headless/API evidence | Progress evidence | Cancellation evidence |
|---|---|---|---|
| COLMAP | Full CLI for every core function; C++ library; PyCOLMAP [S1][S3] | CLI/stdout; viewer is GUI-only | [UNVERIFIED] process-level termination only |
| Meshroom/AliceVision | Node CLI + Python scripts; renderfarm mode; Qt is UI-only [S6][S7] | Node logs, statistics, status tabs [S7] | [UNVERIFIED] UI stop button; no documented external cancel API |
| OpenMVS | CLI + library [S9] | [UNVERIFIED] | [UNVERIFIED] |
| Nerfstudio | `ns-*` CLI, Python API, viewer, Docker [S13][S14] | Viewer, TensorBoard, W&B, Comet [S13] | [UNVERIFIED] checkpoint-based resume observed; explicit cancel API not documented |
| gsplat | Python library + example scripts [S15] | Example training logs [UNVERIFIED for formal API] | [UNVERIFIED] |
| Open3D | C++/Python + CLI example `run_system.py` [S17][S18] | Script-stage output | [UNVERIFIED] |
| Blender | `blender --background --python` [S34] | Script-controlled | Script-controlled process |
| ODM | CLI + Docker + NodeODM API [S24] | [UNVERIFIED] NodeODM job status exists but was not inspected | [UNVERIFIED] |

**Evidence gap:** no inspected primary source documents a cancellation token/API. Genesis should assume **external process supervision (start/kill + checkpoint resume)** is the only portable cancellation mechanism until proven otherwise. [INFERENCE]

---

## 4. Splat / NeRF → editable mesh conversion and quality loss — critical findings

### 4.1 Concrete conversion methods found

1. **SuGaR (CVPR 2024)** — regularizes 3D Gaussians to align to a surface, samples level-set points from splat depth maps with analytic density normals, then runs **Poisson reconstruction**; optional refinement binds new Gaussians to mesh triangles for jointly optimized **hybrid mesh+Gaussians**, enabling editing/sculpting/rigging in Blender/Unity/Unreal by manipulating the mesh. An **optional traditional textured mesh** (`--export_obj`) is produced via nvdiffrast for editing/animation [S21][S26]. Authors claim minutes vs hours for neural-SDF meshes and better PSNR/SSIM/LPIPS [S26 — **paper claim, not independently verified**]. Licence is the non-commercial Inria/MPII text [S20]. Last pushed 2024-09-24 [S5].
2. **2D Gaussian Splatting (SIGGRAPH 2024)** — surfels with depth/normal regularization; renders median depth maps and fuses them with **TSDF (Open3D)**; supports bounded and unbounded extraction (space contraction + adaptive truncation) [S27][S28]. Paper reports median depth better than expected depth, and TSDF better than screened Poisson using 2D Gaussian centres/normals because Poisson ignores opacity and primitive size [S28]. Licence is INRIA-style non-commercial [S20][S29 — exact file [UNVERIFIED]].
3. **Nerfstudio exports** — `ns-export tsdf` (all models), `ns-export poisson` (normals-predicting models only), `ns-export marching-cubes`, `ns-export pointcloud`, `ns-export gaussian-splat` (PLY) [S22][S23]. Texturing an already simplified mesh is supported via `xatlas-python` + `pymeshlab` [S22]. Apache-2.0 [S14].
4. **Blender-side workflows** — Nerfstudio has a Blender VFX add-on and Maya plug-in [S22]; current Blender Python API includes `GeometryNodeImportSPZ` and `PointCloud` types [S34]. These render/import *splat representations*; they do **not** by themselves produce a UV-mapped editable triangle mesh. [INFERENCE from API listing; no documented one-click "splat → editable mesh" command found in Blender docs reviewed]

### 4.2 Emerging 2025–2026 research (thin evidence — no production claim)

- **MILo: Mesh-In-the-Loop Gaussian Splatting** (TOG 2025, arXiv:2506.24096) targets compact high-fidelity meshes and explicitly cites "loss of fine geometric details" from costly post-processing extraction [S40]. Licence/maturity [UNVERIFIED].
- **MeshSplats** (arXiv:2502.07754) converts GS to a mesh-like format; paper notes limitations around shadows/reflections [S41]. Licence [UNVERIFIED].
- **TopSDF** (arXiv:2611.18708, Nov 2026) states that extracting "clean, watertight, explicit surface meshes from 3DGS remains a critical challenge" [S42]. [THIN — new preprint, not independently reproduced]
- **AnyGS2Mesh** (arXiv:2609.03304, Sep 2026) feed-forward mesh reconstruction from splats with arbitrary-resolution views [S43]. [THIN]
- **Mesh Splatting for End-to-end Multiview Surface Reconstruction** (arXiv:2601.21400) bridges surface and volumetric representations differentiably [S44]. [THIN]

### 4.3 Documented quality-loss mechanisms

- **Surface is inferred, not observed.** 3DGS starts from sparse SfM points and optimizes anisotropic Gaussians for *rendering* quality, not topology [S19]; any mesh is a level-set/TSDF approximation [S21][S28].
- **View-dependent radiance is lost in a plain textured mesh.** Gaussian SH features encode view-dependent colour [S19]; SuGaR's own hybrid mesh+Gaussians representation exists precisely to retain high-quality rendering while editing the mesh [S21][S26]. A single albedo texture cannot reproduce SH/view-dependent effects. [INFERENCE from [S19][S21]]
- **Oversmoothing and appearance/geometry trade-off** are explicitly listed as 2DGS limitations; densification favours texture-rich over geometry-rich regions, and semi-transparent surfaces such as glass fail under the full-opacity assumption [S28].
- **Grid/Poisson parameter sensitivity**: TSDF voxel size and truncation must be tuned (2DGS paper uses 0.004 / 0.02; broken meshes require `depth_trunc` tuning) [S27][S28]; Poisson needs near outlier-free input and produces bad surfaces with large holes [S3].
- **Topology/collision**: watertight explicit mesh extraction is described as an open challenge in Nov 2026 literature [S42]. Collision-ready, LOD-ready, UV-valid meshes therefore require downstream QA, not just conversion. [INFERENCE]
- **Scale**: SIBR/3DGS targets 1–1.6K-pixel images and notes multi-scale city driving scenes struggle [S19]; relevant to large outdoor Genesis scenes.

**Bottom line from evidence:** a splat is a renderable approximation; a game-ready editable mesh requires a *surface reconstruction step* (Poisson/TSDF/mesh-in-the-loop) plus UV/texture baking plus QA. Pure converted meshes lose view-dependent effects, fine detail and transparency fidelity; hybrid mesh+Gaussian retains rendering quality but is not a conventional game mesh. [S19][S21][S26][S28][S42]

---

## 5. Licence blockers and obligations (evidence summary)

| Blocker class | Tools | Evidence |
|---|---|---|
| **Non-commercial / research-only** | INRIA 3DGS, SuGaR, 2DGS [S20][S21][S29] | Explicit: "non-commercially, research and/or evaluation purposes only"; commercial use needs written consent from Inria |
| **Strong copyleft** | OpenMVS AGPL-3.0 [S12], ODM AGPL-3.0 [S25], Open3DSG AGPL-3.0 [S32], Blender GPL-3.0+ [S33], MeshLab GPL-3.0 [S5] | Acceptable as separately invoked external programs; linking/bundling into a Rust core changes obligations. [INFERENCE — not legal advice] |
| **File-level copyleft** | AliceVision, Meshroom, OpenMVG (MPL-2.0) [S6][S7][S11] | Modified MPL files must remain MPL; larger works may use other terms |
| **Custom/AUP + gated weights** | VGGT [S38][S39] | Code use permitted commercially since 2025-07-29 but AUP and military exclusion apply; only `VGGT-1B-Commercial` checkpoint is commercially licensed, via application |
| **Research-only bundled component** | OpenMVS's IBFS [S12] | "can be used for research purposes only" — needs legal review if redistributing binaries |
| **Permissive / low-friction** | COLMAP BSD [S2], gsplat & Nerfstudio Apache-2.0 [S16][S14], Open3D MIT [S18], PDAL BSD [S36] | Preferred licence profile for in-process or redistributed components |

Model/data restrictions: pretrained/demo datasets (MipNeRF360, Tanks&Temples, DTU, 3RScan) carry their own terms; INRIA README notes two MipNeRF360 datasets "cannot be openly shared" [S19]. [UNVERIFIED for all dataset licences]

---

## 6. Worker-architecture evidence (not a decision)

Evidence supports a **staged external-worker model**, because:

- Mature tools expose CLI + library interfaces and are already process-oriented: COLMAP staged CLI [S3], Meshroom node/renderfarm execution [S7], ODM Docker/CLI/NodeODM [S24], Blender background mode [S34].
- No cancellation API was found in any primary source (§3.4); process supervision + checkpoint resume is the only cross-tool pattern evidenced.
- Stage boundaries correspond to representation boundaries: pose/SfM (COLMAP/VGGT candidate) → dense/fusion (COLMAP MVS, Open3D RGB-D) → mesh/texture (OpenMVS, Meshroom, ODM, Blender) → optional splat/NeRF branch (gsplat/Nerfstudio) → splat-to-mesh conversion if editable geometry is required (research methods only, licence-restricted).
- Licences favour **separate processes** for AGPL/GPL/MPL tools [S12][S25][S33][S11], while BSD/MIT/Apache tools are candidates for in-process libraries [S2][S18][S16][S14].
- Progress reporting is heterogeneous (stdout, node logs, viewer, W&B), implying a need for a normalised progress/event adapter. [INFERENCE from §3.4]

No final choice of orchestration, worker protocol or representation IR is made here.

---

## 7. Maturation signals (as of 2026-11-27)

| Signal | Evidence |
|---|---|
| **Strong/active** | COLMAP v4.2.1 + pushed 2026-11-26 [S4][S5]; ODM pushed 2026-11-26 [S5]; Blender pushed 2026-11-26 [S5]; Open3D v0.20.0 + pushed 2026-11-14 [S5]; PDAL pushed 2026-11-23 [S5]; gsplat `main` features through Aug 2026 [S15]; OpenMVG pushed 2026-11-07 [S5]; OpenMVS v2.4.0 + pushed 2026-11-26 [S5]; AliceVision/Meshroom pushed 2026-11-26 [S5] |
| **Slow/stale** | Nerfstudio last release 2024-11-11, last push 2025-07-29 [S5]; INRIA 3DGS last push 2025-10-17, README admits limited resources [S5][S19]; SuGaR last push 2024-09-24 [S5]; Open3DSG/3DSSG 2024 [S5] |
| **Release lag vs code** | gsplat tagged v1.5.3 (2025-07-04) but `main` documents v1.6.0 changes through Aug 2026 [S5][S15]; OpenMVG no release since 2023 but recent pushes [S5] |
| **Emerging** | 2026 preprints on splat→mesh (TopSDF, AnyGS2Mesh, Mesh Splatting) [S42][S43][S44] |

---

## 8. Thin evidence / explicit unknowns

1. **Cancellation** for every candidate — no documented API found. [UNVERIFIED]
2. **Metal/Vulkan GPU support** for COLMAP, Nerfstudio, Open3D and gsplat on macOS — no primary evidence found. [UNVERIFIED]
3. **Exact 2DGS `LICENSE.md` text** and `diff-surfel-rasterization` submodule licence — not fetched; repository API reports NOASSERTION. [UNVERIFIED]
4. **MicMac licence of specific released binaries** — README says CeCILL-B, release note warns ALGLIB → GPL-2+ [S35]. [UNVERIFIED release-specific]
5. **Open3D's `o3d.pipelines.sfm` scope** — not inspected. [UNVERIFIED]
6. **Semantic scene graph licences/datasets** — 3DSSG NOASSERTION; dataset terms not reviewed [S32][S5]. [UNVERIFIED]
7. **Quality comparisons** between converted meshes and native photogrammetric meshes — only paper claims found (SuGaR [S26], 2DGS [S28]); no independent benchmark in primary sources reviewed. [THIN]
8. **CloudCompare licence version and silent CLI mode** — not verified from primary source. [UNVERIFIED]
9. **Blender native splat-to-editable-mesh conversion** — not found in reviewed docs; import nodes appear to be render/visualisation oriented [S34]. [UNVERIFIED]

---

## 9. Short answer to the spec's required questions

- **Images/video/depth/LiDAR:** COLMAP images only; AliceVision images+video; Nerfstudio images/video/360 + LiDAR phone/AR devices via importers; ODM aerial images/video; Open3D RGB-D; MicMac survey images; PDAL LiDAR/point clouds; gsplat/3DGS consume COLMAP plus newer LiDAR/fisheye sensor models [S1][S6][S17][S22][S24][S35][S15][S19].
- **Camera calibration/pose:** COLMAP, AliceVision, OpenMVG, ODM, VGGT native; OpenMVS consumes poses; Open3D RGB-D odometry/registration; Blender video tracking (not photogrammetric SfM) [S1][S6][S10][S24][S38][S9][S18].
- **Dense + mesh + texture:** COLMAP dense+mesh (no UV texture); OpenMVS full chain incl. texture; AliceVision/Meshroom full chain; ODM full chain; Open3D fusion/mesh (no texture); Nerfstudio export meshes/splats [S3][S9][S7][S24][S18][S22].
- **Worker suitability:** all major CLI tools are headless-capable; GPL/AGPL tools should remain external processes; MIT/BSD/Apache tools can be considered in-process [§5][§6].
- **Cross-platform:** COLMAP, AliceVision/Meshroom, OpenMVG, Open3D, ODM, Blender, PDAL, MicMac cover Win/Linux/macOS; neural CUDA pipelines are effectively NVIDIA-only [S1][S6][S10][S17][S24][S33][S13][S15][S19][S35].

---

## 10. Primary sources (all accessed 2026-11-27)

- [S1] COLMAP GitHub README — https://github.com/colmap/colmap / https://raw.githubusercontent.com/colmap/colmap/main/README.md
- [S2] COLMAP licence — https://colmap.github.io/license.html
- [S3] COLMAP CLI docs — https://colmap.github.io/cli.html (legacy mirror read: https://colmap.github.io/legacy/3.11/cli.html)
- [S4] COLMAP releases — https://github.com/colmap/colmap/releases/tag/4.2.1 ; v4.0.0 and v4.2.0 release notes via GitHub API
- [S5] GitHub REST API repository/release metadata (`pushed_at`, `stargazers_count`, `open_issues_count`, `license.spdx_id`, `releases/latest`) — https://api.github.com/repos/{owner}/{repo} ; accessed 2026-11-27
- [S6] AliceVision README + COPYING — https://github.com/alicevision/AliceVision ; https://raw.githubusercontent.com/alicevision/AliceVision/master/README.md
- [S7] Meshroom README — https://github.com/alicevision/Meshroom ; https://raw.githubusercontent.com/alicevision/Meshroom/master/README.md
- [S8] AliceVision website — https://alicevision.org/
- [S9] OpenMVS README — https://github.com/cdcseacave/openMVS ; https://raw.githubusercontent.com/cdcseacave/openMVS/master/README.md
- [S10] OpenMVG README — https://github.com/openMVG/openMVG ; https://raw.githubusercontent.com/openMVG/openMVG/master/README.md
- [S11] OpenMVG LICENSE (MPL-2.0) — https://github.com/openMVG/openMVG/blob/master/LICENSE
- [S12] OpenMVS COPYRIGHT.md — https://github.com/cdcseacave/openMVS/blob/master/COPYRIGHT.md
- [S13] Nerfstudio README — https://github.com/nerfstudio-project/nerfstudio ; https://raw.githubusercontent.com/nerfstudio-project/nerfstudio/main/README.md
- [S14] Nerfstudio LICENSE (Apache-2.0) — https://github.com/nerfstudio-project/nerfstudio/blob/master/LICENSE
- [S15] gsplat README — https://github.com/nerfstudio-project/gsplat ; https://raw.githubusercontent.com/nerfstudio-project/gsplat/main/README.md
- [S16] gsplat LICENSE (Apache-2.0) — https://github.com/nerfstudio-project/gsplat/blob/main/LICENSE
- [S17] Open3D README — https://github.com/isl-org/Open3D ; https://raw.githubusercontent.com/isl-org/Open3D/main/README.md
- [S18] Open3D LICENSE (MIT) — https://github.com/isl-org/Open3D/blob/main/LICENSE ; reconstruction system overview — https://github.com/isl-org/Open3D/blob/main/docs/tutorial/reconstruction_system/system_overview.rst
- [S19] 3D Gaussian Splatting official repo/README — https://github.com/graphdeco-inria/gaussian-splatting ; project page https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/
- [S20] INRIA/MPII Gaussian-Splatting licence (also used by SuGaR) — https://github.com/graphdeco-inria/gaussian-splatting/blob/main/LICENSE.md ; https://github.com/Anttwo/SuGaR/blob/main/LICENSE.md
- [S21] SuGaR README — https://github.com/Anttwo/SuGaR ; https://raw.githubusercontent.com/Anttwo/SuGaR/main/README.md
- [S22] Nerfstudio "Using custom data" + "Export geometry" docs — https://docs.nerf.studio/quickstart/custom_dataset.html ; https://docs.nerf.studio/quickstart/export_geometry.html
- [S23] Nerfstudio `ns-export` CLI reference — https://docs.nerf.studio/reference/cli/ns_export.html
- [S24] OpenDroneMap README — https://github.com/OpenDroneMap/ODM ; https://raw.githubusercontent.com/OpenDroneMap/ODM/master/README.md
- [S25] ODM LICENSE (AGPL-3.0) — https://github.com/OpenDroneMap/ODM/blob/master/LICENSE
- [S26] SuGaR paper (CVPR 2024) — https://arxiv.org/abs/2311.12775 ; https://openaccess.thecvf.com/content/CVPR2024/papers/Guedon_SuGaR_Surface-Aligned_Gaussian_Splatting_for_Efficient_3D_Mesh_Reconstruction_and_CVPR_2024_paper.pdf
- [S27] 2D Gaussian Splatting README — https://github.com/hbb1/2d-gaussian-splatting ; https://raw.githubusercontent.com/hbb1/2d-gaussian-splatting/main/README.md
- [S28] 2DGS paper (SIGGRAPH 2024) — https://arxiv.org/html/2403.17888v1
- [S29] 2DGS licence metadata — GitHub repo API `license: NOASSERTION`, LICENSE.md present — https://github.com/hbb1/2d-gaussian-splatting
- [S30] 3DSSG project/paper — https://3dssg.github.io/ ; https://github.com/ShunChengWu/3DSSG
- [S31] Open3DSG — https://github.com/boschresearch/Open3DSG (CVPR 2024)
- [S32] Open3DSG licence metadata (AGPL-3.0) — GitHub repo API — https://github.com/boschresearch/Open3DSG
- [S33] Blender licence — https://blender.org/about/license ; https://docs.blender.org/manual/en/3.0/getting_started/about/license.html
- [S34] Blender Python API tips/background mode + API reference — https://docs.blender.org/api/current/info_tips_and_tricks.html ; https://docs.blender.org/api/current/index.html
- [S35] MicMac README/LICENSE — https://github.com/micmacIGN/micmac ; https://raw.githubusercontent.com/micmacIGN/micmac/master/README.md ; https://github.com/micmacIGN/micmac/blob/master/LICENSE.md
- [S36] PDAL LICENSE.txt (BSD) — https://github.com/PDAL/PDAL/blob/master/LICENSE.txt
- [S37] CloudCompare — https://github.com/CloudCompare/CloudCompare ; MeshLab — https://github.com/cnr-isti-vclab/meshlab
- [S38] VGGT README — https://github.com/facebookresearch/vggt ; https://raw.githubusercontent.com/facebookresearch/vggt/main/README.md
- [S39] VGGT LICENSE.txt + acceptable use policy — https://github.com/facebookresearch/vggt/blob/main/LICENSE.txt
- [S40] MILo — https://arxiv.org/abs/2506.24096 (TOG 2025)
- [S41] MeshSplats — https://arxiv.org/abs/2502.07754
- [S42] TopSDF — https://arxiv.org/abs/2611.18708
- [S43] AnyGS2Mesh — https://arxiv.org/abs/2609.03304
- [S44] Mesh Splatting for End-to-end Multiview Surface Reconstruction — https://arxiv.org/abs/2601.21400

---

**End of evidence record. No architecture decisions are made in this document.**
