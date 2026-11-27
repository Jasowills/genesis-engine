# Genesis Engine — Performance Budgets (estimates, not measurements)

**Date:** 2026-11-27. **Status:** Planning baselines. Every numeric range below is **[E] an estimate**, not a Genesis benchmark. Replace with measured fixture results before putting numbers in UI, docs or marketing.

## 1. Verified anchors

| Anchor | Verified fact | Source |
| --- | --- | --- |
| GitHub standard runners | Linux/Windows x64 4 vCPU, 16 GB RAM, 14 GB SSD; Linux/Windows ARM64 similar; macOS Intel 4/14/14; macOS M1 3/7/14 | `research/notes/quality-ci-gpu.md` §3 |
| GPU larger runner | 4 vCPU, Tesla T4, 28 GB RAM, 16 GB VRAM, 176 GB SSD; Team/Enterprise only; billed even for public repos | same |
| COLMAP SIFT memory | Approximately `4m² + 4m×256` bytes for `m = max_num_matches`; 10,000 matches ≈ 400 MB GPU memory | `research/notes/quality-ci-gpu.md` §4 (COLMAP FAQ) |
| COLMAP CPU behaviour | All functionality except dense reconstruction can run on CPU with flags; "significant slow-down"; CPU feature extraction can consume excessive RAM for large images | same |
| CI costs | Public standard runners free; private $0.006 Linux / $0.010 Windows / $0.062 macOS per minute; T4 $0.052 Linux / $0.102 Windows | same |
| Signing costs | Apple $99/yr; Windows Artifact Signing ~$9.99/mo, OV $150–300/yr, EV $400+/yr | same |
| Viewer precision | glTF float32 ULP ≈ 0.5 m at Earth radius, ≈ 1 m at 1e7 m (derived); f64 vertex attributes need a draft extension | `research/FORMAT_AND_STANDARDS.md` §2 |

## 2. Proposed profile budgets [E — unmeasured]

| Profile | Area / inputs | Time | Peak RAM | VRAM | Disk |
| --- | --- | --- | --- | --- | --- |
| Preview | ≤0.1 km², ≤1k images, SfM + coarse mesh | ~10–60 min SfM; dense 1–6 h or disabled | 8–16 GB | 4–8 GB | 5–30 GB |
| Game-ready | ≤1 km², ~5k–20k images, dense + textured mesh + collision/LOD | SfM ~0.5–3 h; dense ~2–12 h | 32–64 GB | 8–16 GB | 50–300 GB |
| High fidelity | ~1–10+ km², 50k–200k images | SfM ~4–48 h; dense days | 64–256 GB | 16–48 GB multi-GPU | 0.5–5 TB |

These are planning ranges only. Content dependence (texture, overlap, occlusion) dominates. Do not advertise them as supported requirements.

## 3. Budget policy (design rules, not measured limits)

1. **Preflight outputs ranges + confidence**, never a single promise. Include approximate data volume, storage, required model downloads, CPU/GPU suitability, processing range and any external API/cloud charge.
2. **Explicit confirmation** before a job incurs paid external cost.
3. **CI fixtures must fit 14 GB disk** on standard runners; heavy reconstruction E2E goes to small fixtures or self-hosted runners.
4. **Bounded resources:** no unbounded queues/caches; enforce per-job CPU/memory/time caps and plugin Store/WASI limits.
5. **Compression is delivery-only:** source IR stays uncompressed; KTX2/Draco/meshopt applied at export.
6. **Memory leak tests:** load/unload stress tests are mandatory for viewport candidates (three.js r187 and Babylon 9.38.0 both show active disposal fixes).
7. **Accuracy is not a budget:** survey/reference claims require independent checkpoints (ASPRS Ed2 V2), not more polygons or prettier renders.

## 4. Measurement plan (to become tickets)

- GE-0003: floating-origin error measurements and golden vectors.
- GE-0004: webview capability probe, 100k-object picking/batching, load/unload leak test.
- GE-0006: model load/inference/cancellation timing on target hardware.
- GE-0020: E2E fixture with wall clock, peak RAM, peak VRAM and disk recorded; frozen inputs; ranges reported.
- GE-0024: calibrate preflight estimator against measured fixtures.
- GE-0030: budget verification per named target profile.

Record tool versions, OS, CPU/GPU, driver versions, fixture checksums and full residual distributions (p50/p95/max), not just means.
