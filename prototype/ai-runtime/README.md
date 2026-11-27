# GE-0006 — AI runtime evidence prototype (throwaway)

This directory is **evidence-only scratch**, not product code and not a shipped
component. It exists to answer one narrow question for ticket GE-0006: can this
project install ONNX Runtime from PyPI, load a locally generated ONNX model, run
deterministically, and cancel a run — with no model-weight downloads and no
external AI providers?

Date of run: 2026-11-27. Host: macOS 26.5.2, x86_64, Python 3.13.15 venv.

## Headline result

* **ONNX Runtime 1.32.0 is NOT installable on this host from PyPI.** PyPI
  publishes 1.32.0 wheels for macOS **arm64**, Linux and Windows only — no
  macOS x86_64 wheel exists for that version (the last macOS x86_64 wheel is
  1.23.2). Exact pip errors are in `RESULTS.md`.
* Licence for 1.32.0 was verified from **PyPI metadata**: `MIT License`. The
  GitHub repository was deliberately **not** fetched (404 per
  `research/RESEARCH_REGISTER.md` §4 resolution #4).
* Runtime inference/determinism/cancellation were demonstrated with the newest
  installable fallback, **ONNX Runtime 1.23.2**, not with 1.32.0. This is stated
  everywhere; no 1.32.0 runtime claim is made.
* Static inspection of the real 1.32.0 wheel (downloaded for Linux x86_64,
  never executed here) found `RunOptions` and a `terminate` attribute with a
  docstring about terminating running calls; no Python `set_terminate` method
  was found. Runtime cancellation on 1.32.0 stays **UNVERIFIED**.

## Files

| File | Purpose |
| --- | --- |
| `smoke.py` | Self-contained test: generates two ONNX graphs, checks exact output, determinism, cooperative cancellation; writes `smoke-results.json`. |
| `model_add4.onnx` | Locally generated 4-element float32 `Add` graph (106 bytes). |
| `model_loop.onnx` | Locally generated 20 000-node `Add` chain (used only to make cancellation observable). |
| `smoke-output.log`, `smoke-results.json` | Smoke run transcript and machine-readable measured results. |
| `RESULTS.md` | Verbatim commands/output, versions, checksums, licence source, verdicts, limits. |
| `install-*.log`, `download-1.32.0-linux-wheel.log`, `inspect-1.32.0-wheel-api.log`, `api-surface-1.23.2.log`, `pypi-*.log` | Raw evidence logs. |
| `wheels/` | Downloaded 1.32.0 wheel + extracted `.so` for static inspection only; ignored by `.gitignore`. Not runnable on macOS. |
| `.venv/` | Local venv; ignored by `.gitignore`; never commit. |
| `probe-cancel.py`, `probe-cancel.onnx`, `probe-cancel.log` | Earlier cancellation probe, superseded by `smoke.py`. |

## Reproduce

```sh
cd prototype/ai-runtime
python3.13 -m venv .venv
.venv/bin/python -m pip install 'onnxruntime==1.32.0' onnx   # fails on this host: see RESULTS.md
.venv/bin/python -m pip install 'onnxruntime==1.23.2' onnx   # fallback, clearly NOT 1.32.0
.venv/bin/python smoke.py | tee smoke-output.log
shasum -a 256 model_add4.onnx model_loop.onnx
```

`smoke.py` prints the runtime version it actually imported and sets
`is_1_32_0` accordingly; it never claims 1.32.0 unless that version is loaded.

## Boundaries

No model weights were downloaded. No external AI provider, paid API, or GitHub
repository was used for evidence. No secrets were created. No EP/CoreML/DirectML/
GPU/ARM64 tests were run — CPU provider only. Timing values are local
measurements, **not** performance budgets or benchmarks. The venv and wheel
cache are ignored by the local `.gitignore`; the source and evidence files are
committed in this repository (commit `b53d0c2`).
