---
id: GE-0006
title: AI runtime and model smoke test
type: spike
priority: P1
status: in-review
milestone: M0
labels: [spike, area:ai]
depends_on: []
blocks: [GE-0007, GE-0024, GE-0026]
created: 2026-11-27
updated: 2026-11-27
owner_approval_required: false
---

# GE-0006 — AI runtime and model smoke test

## Problem
Model code and weight licences diverge; runtime operator coverage and cancellation need verification in our environment.

## User / engineering value
Gates A04 and prevents a licence or capability surprise in later milestones.

## Scope
### Included
- Smoke-test ONNX Runtime 1.32.0 with SAM ViT-B, YOLOX, Depth Anything V2 Small
- Verify RunOptions::SetTerminate cancellation and EP fallback logging
- Record checksums, licences and hardware requirements

### Excluded
- Production AI features or model bundling

## Context and research
- Relevant sources:
  - `research/notes/ai-ml.md`
  - `research/LICENCE_AND_DATA_RIGHTS.md section 1`
- Relevant ADRs:
  - `A04`
- Related tickets: `TICKETS/README.md` (dependency index)

## Proposed approach
Small headless harness; no desktop dependency; record per-run evidence fields.

## Acceptance criteria
- [x] A locally generated ONNX model loads and produces typed output on ORT 1.32.0 in Ubuntu CI (`exact_match: true`, `deterministic: true`).
- [ ] SAM ViT-B, YOLOX and Depth Anything V2 Small are loaded — **NOT MET**: no licensed model weights downloaded; requires owner cost/licence decision.
- [x] Cancellation demonstrated via `RunOptions.terminate` on ORT 1.23.2 locally and ORT 1.32.0 in Ubuntu CI (`46001781425`).
- [x] Checksum/licence record complete for locally generated artefacts and PyPI metadata
- [x] Prototype scripts, README and RESULTS.md updated (committed in `b53d0c2`)

## Test plan
- Unit: n/a
- Integration: n/a
- End-to-end: n/a
- Manual: Manual output inspection
- Platform coverage: macOS host; Linux/Windows CI

## Security / licensing / data considerations
No non-commercial weights; never unpickle untrusted files; no external API costs without approval.

## Dependencies and risks
ORT GitHub repo was unreachable during research; re-verify from another network.

## Implementation notes
No paid cloud inference.

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

GitHub issue: **#9** — https://github.com/Jasowills/genesis-engine/issues/9
- Files: `prototype/ai-runtime/` (`smoke.py`, `smoke-results.json`, `RESULTS.md`, `README.md`, PyPI metadata, model checksums; `.venv/`/`wheels/` ignored).
- `pip install onnxruntime==1.32.0 onnx` → **FAILED**: PyPI 1.32.0 publishes macOS **arm64** wheels only; no macOS x86_64 wheel. PyPI registry reports version 1.32.0 and `license: MIT License`; GitHub repo was not used (404, RESEARCH_REGISTER §4 #4).
- Fallback `onnxruntime==1.23.2 onnx` installed; `smoke.py` re-run by the independent reviewer → exact output match, deterministic inference, cooperative cancellation via `RunOptions.terminate`.
- Model SHA-256 verified by reviewer: `model_add4.onnx` `c8bdea77…eed2b8`, `model_loop.onnx` `45b3491d…742ac33`.
- Static inspection of the downloaded 1.32.0 Linux wheel found `RunOptions.terminate` but no Python `set_terminate`; labelled static, never executed.
- Reviewer verdict for the original macOS evidence: `APPROVED` for recording only (session `ses_de3db3b9affemBI3BQCzIZcj4i`), explicitly **not** `verified`. Ubuntu CI run **`46001781425`** (step "ONNX Runtime 1.32 smoke test (Ubuntu)", conclusion success) later executed ORT 1.32.0: `onnxruntime_actual: 1.32.0`, `is_1_32_0: true`, `exact_match: true`, `deterministic: true`, `verdict: CANCELLED (cooperative terminate via RunOptions.terminate)`. Named models were still never executed.
- Residual UNVERIFIED: SAM/YOLOX/DA-V2 Small weights; CoreML/DirectML/GPU/EP fallback; Windows/macOS hosts; 1.32.0 behaviour on macOS x86_64 (no wheel). Ubuntu x86_64 1.32.0 runtime now MEASURED in CI.
- **Status rationale:** `in-review` / deliberately partial. The 1.32.0 runtime lane now exists in Ubuntu CI (`46001781425`); the named-model sub-criterion remains unmet, so this ticket is **not** `verified`.

## Changelog
- 2026-11-27: Ticket created from the proposed research/architecture programme; current status: `blocked`; ORT 1.32.0 wheel unavailable on this host.
- 2026-11-27: Independent review APPROVED for evidence; ticket partially complete and blocked on a compatible host.
- 2026-11-27: CI executed ORT 1.32.0 on Ubuntu; evidence updated and ticket moved to in-review.
- 2026-11-27: CI run link and partial-status rationale recorded; ticket remains in-review pending named-model decision.
