#!/usr/bin/env python3
"""GE-0006 named-model smoke harness. Evidence only, not product code.

Loads the three owner-approved named weights (SAM ViT-B quantised encoder,
YOLOX-S, Depth Anything V2 Small), verifies their published SHA-256 digests,
runs one deterministic inference per graph on CPU, and writes a machine-readable
report. No external AI provider is contacted and no cost is incurred.

Run from the prototype directory with the local venv:
    .venv/bin/python named_models_smoke.py | tee named-models-output.log
"""

from __future__ import annotations

import hashlib
import json
import platform
import sys
import zipfile
from pathlib import Path

import numpy as np
import onnx
import onnxruntime as ort

HERE = Path(__file__).resolve().parent
WEIGHTS = HERE / "weights"
REPORT = HERE / "named-models-results.json"

MODELS = [
    {
        "name": "sam_vit_b_01ec64_quant(encoder)",
        "path": WEIGHTS / "sam_vit_b_quant" / "sam_vit_b_01ec64.encoder.quant.onnx",
        "archive": WEIGHTS / "sam_vit_b_01ec64_quant.zip",
        "source": "https://huggingface.co/vietanhdev/segment-anything-onnx-models",
        "licence": "Apache-2.0 (upstream SAM ViT-B)",
        "expected_sha256": None,  # archive digest is checked; ONNX digest recorded in the report
    },
    {
        "name": "yolox_s",
        "path": WEIGHTS / "yolox_s.onnx",
        "source": "https://huggingface.co/inference4j/yolox-s (official YOLOX 0.1.1rc0 export)",
        "licence": "Apache-2.0",
        "expected_sha256": "c5c2d13e59ae883e6af3b45daea64af4833a4951c92d116ec270d9ddbe998063",
    },
    {
        "name": "depth_anything_v2_vits",
        "path": WEIGHTS / "depth_anything_v2_vits.onnx",
        "source": "https://huggingface.co/AXERA-TECH/Depth-Anything-V2",
        "licence": "BSD-3-Clause (AXERA-TECH ONNX/NPU conversion); upstream Depth-Anything-V2-Small is Apache-2.0",
        "expected_sha256": "443e95f17819f347f5f987384b8cb7d7d18ed6af6ac46dec9b0152748ba7dfd0",
    },
]


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def unpack_sam() -> None:
    archive = WEIGHTS / "sam_vit_b_01ec64_quant.zip"
    target = WEIGHTS / "sam_vit_b_quant"
    if not archive.is_file():
        return
    if not (target / "sam_vit_b_01ec64.encoder.quant.onnx").is_file():
        with zipfile.ZipFile(archive) as bundle:
            bundle.extractall(target)


def input_value(shape: list, name: str) -> np.ndarray:
    # Deterministic, finite, small-valued input. Variable dimensions are set to 1.
    dims = [1 if (isinstance(d, str) or d is None or d <= 0) else min(int(d), 1024) for d in shape]
    if "point" in name.lower() or "coord" in name.lower():
        return np.zeros(dims, dtype=np.float32)
    return np.full(dims, 0.5, dtype=np.float32)


def run_model(spec: dict) -> dict:
    path = spec["path"]
    result = {
        "name": spec["name"],
        "source": spec["source"],
        "licence": spec["licence"],
        "present": path.is_file(),
    }
    if not path.is_file():
        result["status"] = "missing"
        return result
    digest = sha256(path)
    result["size_bytes"] = path.stat().st_size
    result["sha256"] = digest
    if spec["expected_sha256"]:
        result["digest_matches_published"] = digest == spec["expected_sha256"]
    try:
        onnx.checker.check_model(str(path))
        result["onnx_checker"] = "pass"
    except Exception as error:  # noqa: BLE001 - recorded as evidence
        result["onnx_checker"] = f"unavailable: {str(error)[:200]}"
    session = ort.InferenceSession(str(path), providers=["CPUExecutionProvider"])
    inputs = session.get_inputs()
    outputs = session.get_outputs()
    result["inputs"] = [{"name": i.name, "shape": str(i.shape)} for i in inputs]
    result["outputs"] = [{"name": o.name, "shape": str(o.shape)} for o in outputs]
    feeds = {i.name: input_value(i.shape, i.name) for i in inputs}
    first = session.run(None, feeds)
    second = session.run(None, feeds)
    result["outputs_finite"] = all(
        np.all(np.isfinite(value)) for value in first
    )
    result["deterministic_exact"] = all(
        np.array_equal(a, b) for a, b in zip(first, second)
    )
    result["output_shapes"] = [list(np.asarray(value).shape) for value in first]
    result["status"] = "ran"
    return result


def main() -> None:
    unpack_sam()
    archive = WEIGHTS / "sam_vit_b_01ec64_quant.zip"
    published_archive_digest = "9d59db12affc009f71c9ddeab8fc833507510883e7f6bc4c0cc6c45e0d4f5862"
    report = {
        "archive_checksums": {
            "sam_vit_b_01ec64_quant.zip": {
                "sha256": sha256(archive) if archive.is_file() else None,
                "published_sha256": published_archive_digest,
                "matches": archive.is_file() and sha256(archive) == published_archive_digest,
                "source": "https://huggingface.co/vietanhdev/segment-anything-onnx-models",
            },
        },
        "schema_version": 1,
        "ort_version": ort.__version__,
        "onnx_version": onnx.__version__,
        "python_version": sys.version.split()[0],
        "platform": platform.platform(),
        "machine": platform.machine(),
        "execution_provider": "CPUExecutionProvider",
        "models": [run_model(spec) for spec in MODELS],
        "limitations": [
            "Synthetic small-valued inputs, not real imagery; this is an inference smoke test, not an accuracy benchmark",
            "SAM ViT-B is the quantised ONNX export (encoder only); decoder/prompt path is not exercised here",
            "Cooperative cancellation is evidenced separately with the locally generated loop model",
            "No GPU execution provider, no external provider and no paid call is used",
        ],
    }
    REPORT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({
        "ort_version": report["ort_version"],
        "models": [(m["name"], m["status"]) for m in report["models"]],
    }))


if __name__ == "__main__":
    main()
