# GE-0006 — AI runtime smoke test: RESULTS (throwaway evidence)

Local run date: **2026-11-27**. Host: **macOS 26.5.2, x86_64**. This file documents the **local host run only**; separate CI evidence for Ubuntu x86_64 with ORT 1.32.0 is in GitHub Actions run `46001781425` (step "ONNX Runtime 1.32 smoke test (Ubuntu)").
Python venv: **3.13.15** (an initial 3.15.2 venv was also tried). The prototype
source, scripts and evidence files are committed (commit `b53d0c2`); only the
`.venv/` and wheel cache are ignored by the local `.gitignore`.

## Verdict summary

| Question | Verdict |
| --- | --- |
| Install `onnxruntime==1.32.0` from PyPI on this host | **FAILED — no compatible wheel.** PyPI has 1.32.0 wheels for macOS arm64, Linux and Windows, but **no macOS x86_64 wheel**. Last macOS x86_64 wheel is 1.23.2. |
| 1.32.0 licence from PyPI metadata | **VERIFIED: MIT License** (classifier `License :: OSI Approved :: MIT License`). Source: `https://pypi.org/pypi/onnxruntime/json`. GitHub repo deliberately NOT fetched (404 per RESEARCH_REGISTER §4 #4). |
| Load a locally generated ONNX model, exact + deterministic inference | **VERIFIED locally on ORT 1.23.2; ORT 1.32.0 verified separately in Ubuntu CI run `46001781425`.** 1.32.0 runtime inference = **UNVERIFIED** on this host. |
| Cancellation | **VERIFIED on fallback ORT 1.23.2** via `RunOptions.terminate = True`. For 1.32.0: **UNVERIFIED at runtime**; Python `set_terminate` = **not found** in static inspection. |
| Model weights / external providers / GPU EPs | **NOT TESTED** (out of scope; see limits). |

## 1. Environment and venv

```sh
$ python3 --version
Python 3.15.2
$ python3 -m venv prototype/ai-runtime/.venv
$ .venv/bin/python -m pip install 'onnxruntime==1.32.0' onnx
ERROR: Could not find a version that satisfies the requirement onnxruntime==1.32.0 (from versions: none)

[notice] A new release of pip is available: 26.2.2 -> 26.3
[notice] To update, run: /Users/jasonamadi/genesis-engine/prototype/ai-runtime/.venv/bin/python -m pip install --upgrade pip
ERROR: No matching distribution found for onnxruntime==1.32.0
```

`--verbose` and an explicit `--index-url https://pypi.org/simple` reproduced the
same error; `pip index versions onnxruntime` reported `No matching distribution
found`. `curl -sS -I --max-time 15 https://pypi.org/simple/onnxruntime/` returned
`HTTP/2 200`, so this was a **wheel-availability problem, not a network outage**.

Venv was then recreated with the newest Python that has macOS x86_64 ORT wheels:

```sh
$ python3.13 -m venv .venv
$ .venv/bin/python --version
Python 3.13.15
$ .venv/bin/python -m pip install 'onnxruntime==1.32.0' onnx
ERROR: Could not find a version that satisfies the requirement onnxruntime==1.32.0 (from versions: 1.20.0, 1.20.1, 1.21.0, 1.21.1, 1.22.0, 1.22.1, 1.23.0, 1.23.1, 1.23.2)

[notice] A new release of pip is available: 26.2 -> 26.3
[notice] To update, run: /Users/jasonamadi/genesis-engine/prototype/ai-runtime/.venv/bin/python -m pip install --upgrade pip
ERROR: No matching distribution found for onnxruntime==1.32.0
```

## 2. PyPI registry evidence (verbatim summary; `pypi-metadata-summary.log`)

```sh
$ curl -sS https://pypi.org/pypi/onnxruntime/json -o pypi-onnxruntime-metadata.json
$ .venv/bin/python - <parse metadata>     # see pypi-metadata-summary.log
source: https://pypi.org/pypi/onnxruntime/json (accessed 2026-11-27)
name: onnxruntime
version: 1.32.0
license: 'MIT License'
license classifiers: ['License :: OSI Approved :: MIT License']
requires_python: >=3.11
project_urls: {'Download': 'https://github.com/microsoft/onnxruntime/tags', 'Homepage': 'https://onnxruntime.ai'}
1.32.0 wheel filenames:
  onnxruntime-1.32.0-cp311-cp311-macosx_14_0_arm64.whl
  onnxruntime-1.32.0-cp311-cp311-manylinux_2_28_aarch64.whl
  onnxruntime-1.32.0-cp311-cp311-manylinux_2_28_x86_64.whl
  onnxruntime-1.32.0-cp311-cp311-win_amd64.whl
  onnxruntime-1.32.0-cp311-cp311-win_arm64.whl
  onnxruntime-1.32.0-cp312-cp312-macosx_14_0_arm64.whl
  onnxruntime-1.32.0-cp312-cp312-manylinux_2_28_x86_64.whl
  onnxruntime-1.32.0-cp313-cp313-macosx_14_0_arm64.whl
  onnxruntime-1.32.0-cp313-cp313-manylinux_2_28_x86_64.whl
  onnxruntime-1.32.0-cp314-cp314-macosx_14_0_arm64.whl
  onnxruntime-1.32.0-cp314-cp314-manylinux_2_28_x86_64.whl
  ... (24 files total; full list in pypi-metadata-summary.log)
latest macOS x86_64/universal2 wheel: ('1.23.2', 'onnxruntime-1.23.2-cp313-cp313-macosx_13_0_x86_64.whl')
onnx source: https://pypi.org/pypi/onnx/1.23.2/json (accessed 2026-11-27)
onnx name/version: onnx 1.23.2
onnx license: None license_expression: 'Apache-2.0'
```

The GitHub repository 404 was **not** used as evidence; licence came from PyPI
only, as required by RESEARCH_REGISTER §4 resolution #4.

## 3. Static inspection of the real 1.32.0 wheel (never executed on this host)

```sh
$ .venv/bin/python -m pip download --no-deps --only-binary=:all: \
    --platform manylinux_2_28_x86_64 --python-version 3.13 \
    --implementation cp --abi cp313 --dest wheels 'onnxruntime==1.32.0'
Collecting onnxruntime==1.32.0
  Downloading onnxruntime-1.32.0-cp313-cp313-manylinux_2_28_x86_64.whl.metadata (5.8 kB)
Downloading onnxruntime-1.32.0-cp313-cp313-manylinux_2_28_x86_64.whl (27.8 MB)
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 27.8/27.8 MB 3.1 MB/s  0:00:09
Saved ./wheels/onnxruntime-1.32.0-cp313-cp313-manylinux_2_28_x86_64.whl
Successfully downloaded onnxruntime
$ shasum -a 256 wheels/onnxruntime-1.32.0-cp313-cp313-manylinux_2_28_x86_64.whl
02ea12a28c069ea8510093d95a7f6ff8126bfd9fc5c0d3381c03b80fb2b424ba  wheels/onnxruntime-1.32.0-cp313-cp313-manylinux_2_28_x86_64.whl
```

Wheel metadata and API strings (from `inspect-1.32.0-wheel-api.log`):

```
--- onnxruntime-1.32.0.dist-info/METADATA (metadata fields) ---
Name: onnxruntime
Version: 1.32.0
License: MIT License
Classifier: License :: OSI Approved :: MIT License
Requires-Python: >=3.11
--- onnxruntime-1.32.0.dist-info/WHEEL ---
Wheel-Version: 1.0
Generator: setuptools (84.2.1)
Root-Is-Purelib: false
Tag: cp313-cp313-manylinux_2_28_x86_64
...
onnxruntime/__init__.py:49: RunOptions,  # noqa: F401
onnxruntime/capi/onnxruntime_inference_collection.py:392: :param run_options: See :class:`onnxruntime.RunOptions`.
onnxruntime/backend/backend_rep.py:15: # 'terminate' excluded: setting it True would deny the current inference call.
```

From the pybind `.so` (`strings ... | grep -i terminate`):

```
Set to True to terminate any currently executing calls that are using this
RunOptions instance. The individual calls will exit gracefully and return an error status.
Exiting due to terminate flag being set to true.
```

**Exact API surface found for 1.32.0:** `onnxruntime.RunOptions` is exported;
its terminate control is a `terminate` attribute with the docstring above;
`InferenceSession.run(output_names, input_feed, run_options=None)` accepts it.
A literal `set_terminate` symbol was **not found** in the Python binding strings.
Whether `RunOptions.terminate` works at runtime in 1.32.0 is **UNVERIFIED** here
(the Linux wheel's ELF extension cannot be imported on macOS x86_64).

## 4. Installed packages (fallback for runtime checks)

```sh
$ .venv/bin/python -m pip install 'onnxruntime==1.23.2' onnx
...
Successfully installed coloredlogs-15.0.1 flatbuffers-25.12.19 humanfriendly-10.0 ml_dtypes-0.7.0 mpmath-1.3.0 numpy-2.5.5 onnx-1.23.2 onnxruntime-1.23.2 packaging-26.4 protobuf-7.36.2 sympy-1.14.0 typing_extensions-4.16.0
$ .venv/bin/python -m pip freeze
coloredlogs==15.0.1
flatbuffers==25.12.19
humanfriendly==10.0
ml_dtypes==0.7.0
mpmath==1.3.0
numpy==2.5.5
onnx==1.23.2
onnxruntime==1.23.2
packaging==26.4
protobuf==7.36.2
sympy==1.14.0
typing_extensions==4.16.0
```

Installed wheel: `onnxruntime-1.23.2-cp313-cp313-macosx_13_0_x86_64.whl`;
`onnx-1.23.2-cp312-abi3-macosx_13_0_universal2.whl` (from pip log).
Runtime API surface (`api-surface-1.23.2.log`, verbatim):

```
ort version: 1.23.2
RunOptions dir: ['_pybind11_conduit_v1_', 'add_active_adapter', 'add_run_config_entry', 'get_run_config_entry', 'log_severity_level', 'log_verbosity_level', 'logid', 'only_execute_path_to_fetches', 'terminate']
has set_terminate: False
has terminate attr: True
run signature: (self, output_names, input_feed, run_options=None) -> 'Sequence[np.ndarray | SparseTensor | list | dict]'
available providers: ['CoreMLExecutionProvider', 'AzureExecutionProvider', 'CPUExecutionProvider']
```

## 5. Model artefacts and checksums

Generated locally by `smoke.py` with `onnx.helper` (opset 17, IR version 8;
no downloaded weights):

```sh
$ shasum -a 256 model_add4.onnx model_loop.onnx
c8bdea77412146e0e66f9ef3666440375c2ff9a6b25cb2dc173af57067eed2b8  model_add4.onnx
45b3491d115ee03df2a7791df47f110bf88889825b4321b0e5826f6b5742ac33  model_loop.onnx
```

`model_add4.onnx` (106 bytes): `y = x + [1,1,1,1]`, float32[4].
`model_loop.onnx` (729 KB): 20 000 chained `Add` nodes, used only to give the
cancellation test enough runtime to observe termination.

## 6. Smoke test (verbatim `smoke-output.log`)

```sh
$ .venv/bin/python smoke.py | tee smoke-output.log
```

```json
{
  "ticket": "GE-0006",
  "purpose": "throwaway local evidence only",
  "host": {
    "platform": "macOS-26.5.2-x86_64-i386-64bit-Mach-O",
    "machine": "x86_64",
    "python": "3.13.15"
  },
  "versions": {
    "onnxruntime_actual": "1.23.2",
    "onnx": "1.23.2"
  },
  "is_1_32_0": false,
  "model_add4_sha256": "c8bdea77412146e0e66f9ef3666440375c2ff9a6b25cb2dc173af57067eed2b8",
  "model_loop_sha256": "45b3491d115ee03df2a7791df47f110bf88889825b4321b0e5826f6b5742ac33",
  "add_inference": {
    "input": [
      1.0,
      2.0,
      3.0,
      4.0
    ],
    "expected": [
      2.0,
      3.0,
      4.0,
      5.0
    ],
    "output_run_1": [
      2.0,
      3.0,
      4.0,
      5.0
    ],
    "output_run_2": [
      2.0,
      3.0,
      4.0,
      5.0
    ],
    "exact_match": true,
    "deterministic": true,
    "elapsed_seconds_run_1": 0.000438,
    "elapsed_seconds_run_2": 7.8e-05,
    "note": "Measured on this host; not a performance budget or benchmark."
  },
  "cancellation": {
    "terminate_set_after_sleep_seconds": 0.002,
    "completed": false,
    "error_type": "Fail",
    "error_message": "[ONNXRuntimeError] : 1 : FAIL : Exiting due to terminate flag being set to true.",
    "elapsed_seconds": 0.003317,
    "verdict": "CANCELLED (cooperative terminate via RunOptions.terminate)"
  },
  "limitations": [
    "Runtime version printed above is the version actually imported.",
    "No model weights, external AI providers or paid API calls were used.",
    "No EP/CoreML/DirectML/GPU/ARM64 tests; CPUExecutionProvider only.",
    "No SAM/YOLO/Depth Anything weights were tested."
  ]
}
```
Wrote smoke-results.json

Output dtype was asserted `float32`; two runs produced bit-identical arrays.
Timings above are **measured local values**, not budgets. A separate probe
(`probe-cancel.log`) reproduced cancellation 3/3 times before `smoke.py` was
written.

## 7. Limits and what was NOT proven

* **No **local macOS x86_64** ONNX Runtime 1.32.0 runtime evidence (no wheel); 1.32.0 is evidenced in Ubuntu CI run `46001781425`** on this host: no install,
  no model load, no inference, no runtime cancellation. Only PyPI licence
  metadata and static wheel inspection are verified for 1.32.0.
* `RunOptions::SetTerminate()` (C++/C ABI) was **not** exercised; Python 1.23.2
  uses the `terminate` attribute, not a `set_terminate()` method.
* **No SAM ViT-B / YOLOX / Depth Anything V2 Small weights** were downloaded or
  tested; no checksums or weight licences for those artefacts were gathered here.
* **No EP tests**: CPUExecutionProvider only. CoreML, DirectML, OpenVINO,
  CUDA/GPU, ARM64, Linux and Windows remain **UNVERIFIED** on this host.
* **No external provider calls, no API keys/secrets, no paid costs, no GitHub
  repo access.** Only PyPI package registry fetches occurred.
* Single macOS x86_64 host; no Linux/Windows **local** evidence; Ubuntu CI is recorded in run `46001781425`, Windows CI not executed for this spike.
* This is a throwaway prototype; no product code, no commit, no documentation
  changes outside `prototype/ai-runtime/`.


## Named model execution (2026-11-27, D-033)

Owner approved use of named open weights. Commands and machine-readable results
are in `named_models_smoke.py` and `named-models-results.json`:

```
.venv/bin/python named_models_smoke.py
{"ort_version": "1.23.2", "models": [["sam_vit_b_01ec64_quant(encoder)", "ran"], ["yolox_s", "ran"], ["depth_anything_v2_vits", "ran"]]}
```

Published digests matched local SHA-256 for YOLOX (`c5c2d13e…`) and Depth
Anything V2 Small (`443e95f1…`). Licence provenance: the Depth Anything V2
Small ONNX artefact comes from `AXERA-TECH/Depth-Anything-V2`, declared
**BSD-3-Clause**; the upstream base model is Apache-2.0. This is a licence
divergence worth recording — both terms are permissive and D-033's acceptance
covers the artefacts' actual licences. the SAM archive digest matched
`9d59db12…` from the Hugging Face blobs API. All three `onnx.checker` checks
passed; outputs were finite and bit-exact across two runs. The SAM quantised
encoder was exercised only — the prompt encoder/decoder path is not part of
this smoke test. No ORT 1.32.0 claim is made for named weights on macOS x86_64
(no wheel exists); 1.32.0 runtime evidence remains the Ubuntu CI run.
