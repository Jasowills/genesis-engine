#!/usr/bin/env python3
"""GE-0006 throwaway smoke harness. Evidence only, not product code.

Generates two tiny ONNX graphs locally (no downloaded weights), runs inference
with the locally installed ONNX Runtime, checks exact/deterministic output, and
demonstrates cooperative cancellation via RunOptions.terminate.

Run from the prototype directory with the local venv:
    .venv/bin/python smoke.py | tee smoke-output.log

This script reports the ONNX Runtime version it actually used. It does NOT
claim that ONNX Runtime 1.32.0 was executed unless that version is installed.
"""

from __future__ import annotations

import hashlib
import json
import os
import platform
import sys
import threading
import time
from pathlib import Path

import numpy as np
import onnx
import onnxruntime as ort
from onnx import TensorProto, helper

HERE = Path(__file__).resolve().parent
ADD_MODEL = HERE / "model_add4.onnx"
LOOP_MODEL = HERE / "model_loop.onnx"
RESULTS_JSON = HERE / "smoke-results.json"


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def make_add_model() -> None:
    """y = x + [1,1,1,1], a four-element float32 Add graph (opset 17, IR 8)."""
    x = helper.make_tensor_value_info("x", TensorProto.FLOAT, [4])
    y = helper.make_tensor_value_info("y", TensorProto.FLOAT, [4])
    one = helper.make_tensor("one", TensorProto.FLOAT, [4], [1.0] * 4)
    node = helper.make_node("Add", ["x", "one"], ["y"], name="add_one")
    graph = helper.make_graph([node], "add4", [x], [y], [one])
    model = helper.make_model(
        graph, opset_imports=[helper.make_opsetid("", 17)], ir_version=8
    )
    model.ir_version = 8  # ORT 1.23.x supports IR <= 11; onnx 1.23 defaults to 14.
    onnx.checker.check_model(model)
    onnx.save(model, ADD_MODEL)


def make_loop_model(n: int = 20_000) -> None:
    """Chain of n Add nodes, used only to make a run long enough to terminate."""
    x = helper.make_tensor_value_info("x", TensorProto.FLOAT, [4])
    y = helper.make_tensor_value_info("y", TensorProto.FLOAT, [4])
    one = helper.make_tensor("one", TensorProto.FLOAT, [4], [1.0] * 4)
    nodes = [
        helper.make_node(
            "Add",
            ["x" if i == 0 else f"t{i - 1}", "one"],
            [f"t{i}" if i < n - 1 else "y"],
            name=f"add_{i}",
        )
        for i in range(n)
    ]
    graph = helper.make_graph(nodes, "loop", [x], [y], [one])
    model = helper.make_model(
        graph, opset_imports=[helper.make_opsetid("", 17)], ir_version=8
    )
    model.ir_version = 8
    onnx.checker.check_model(model)
    onnx.save(model, LOOP_MODEL)


def cpu_session(model: Path, disable_optimizations: bool = False) -> ort.InferenceSession:
    so = ort.SessionOptions()
    if disable_optimizations:
        so.graph_optimization_level = ort.GraphOptimizationLevel.ORT_DISABLE_ALL
    return ort.InferenceSession(str(model), so, providers=["CPUExecutionProvider"])


def test_add_and_determinism() -> dict:
    make_add_model()
    session = cpu_session(ADD_MODEL)
    x = np.array([1.0, 2.0, 3.0, 4.0], dtype=np.float32)
    expected = np.array([2.0, 3.0, 4.0, 5.0], dtype=np.float32)

    t0 = time.perf_counter()
    first = np.asarray(session.run(["y"], {"x": x})[0])
    elapsed_first = time.perf_counter() - t0
    assert first.dtype == np.float32, f"unexpected output dtype: {first.dtype}"
    assert np.array_equal(first, expected), f"expected {expected}, got {first}"

    t0 = time.perf_counter()
    second = np.asarray(session.run(["y"], {"x": x})[0])
    elapsed_second = time.perf_counter() - t0
    assert np.array_equal(second, first), f"non-deterministic output: {first} vs {second}"

    return {
        "input": x.tolist(),
        "expected": expected.tolist(),
        "output_run_1": first.tolist(),
        "output_run_2": second.tolist(),
        "exact_match": True,
        "deterministic": True,
        "elapsed_seconds_run_1": round(elapsed_first, 6),
        "elapsed_seconds_run_2": round(elapsed_second, 6),
        "note": "Measured on this host; not a performance budget or benchmark.",
    }


def test_cancellation() -> dict:
    make_loop_model()
    session = cpu_session(LOOP_MODEL, disable_optimizations=True)
    x = np.zeros(4, dtype=np.float32)
    options = ort.RunOptions()
    assert hasattr(options, "terminate"), "RunOptions.terminate attribute missing"
    assert not hasattr(options, "set_terminate"), (
        "unexpected set_terminate method; record the real API rather than assuming it"
    )

    outcome: dict = {}

    def worker() -> None:
        t0 = time.perf_counter()
        try:
            out = session.run(["y"], {"x": x}, options)
            outcome.update(completed=True, output=np.asarray(out[0]).tolist())
        except Exception as exc:  # ORT raises onnxruntime_pybind11_state.Fail
            outcome.update(
                completed=False,
                error_type=type(exc).__name__,
                error_message=str(exc).strip(),
            )
        finally:
            outcome["elapsed_seconds"] = round(time.perf_counter() - t0, 6)

    thread = threading.Thread(target=worker)
    thread.start()
    time.sleep(0.002)  # let the run start; 20k Add nodes take ~20ms on this host
    options.terminate = True
    outcome["terminate_set_after_sleep_seconds"] = 0.002
    thread.join(timeout=60)
    assert not thread.is_alive(), "run did not return within 60s after terminate"

    assert not outcome.get("completed"), f"run completed before terminate: {outcome}"
    assert "terminate flag" in outcome.get("error_message", "").lower(), (
        f"unexpected cancellation error: {outcome}"
    )
    outcome["verdict"] = "CANCELLED (cooperative terminate via RunOptions.terminate)"
    return outcome


def main() -> None:
    make_add_model()
    make_loop_model()
    report = {
        "ticket": "GE-0006",
        "purpose": "throwaway local evidence only",
        "host": {
            "platform": platform.platform(),
            "machine": platform.machine(),
            "python": sys.version.split()[0],
        },
        "versions": {
            "onnxruntime_actual": ort.__version__,
            "onnx": onnx.__version__,
        },
        "is_1_32_0": ort.__version__ == "1.32.0",
        "model_add4_sha256": sha256(ADD_MODEL),
        "model_loop_sha256": sha256(LOOP_MODEL),
        "add_inference": test_add_and_determinism(),
        "cancellation": test_cancellation(),
        "limitations": [
            "Runtime version printed above is the version actually imported.",
            "No model weights, external AI providers or paid API calls were used.",
            "No EP/CoreML/DirectML/GPU/ARM64 tests; CPUExecutionProvider only.",
            "No SAM/YOLO/Depth Anything weights were tested.",
        ],
    }
    RESULTS_JSON.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    print(json.dumps(report, indent=2))
    print(f"\nWrote {RESULTS_JSON.relative_to(HERE)}")


if __name__ == "__main__":
    main()
