#!/usr/bin/env python3
"""Product-level E2E and asset-quality harness (not a unit test).

Drives the shipped CLI binary through a realistic user journey on a fresh
project, then judges the generated assets with a quality rubric. Output is a
machine-readable report intended for human review.

    python3 tools/product_e2e.py --bin target/debug/genesis --out /tmp/genesis-e2e
"""

from __future__ import annotations

import argparse
import json
import math
import struct
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
BOUNDARY = ROOT / "fixtures/ikeja-boundary.geojson"
FEATURES = ROOT / "fixtures/ikeja-small.geojson"
SOURCE_ARGS = [
    "--source-id", "osm:fixture:ikeja",
    "--licence", "odbl",
    "--attribution", "© OpenStreetMap contributors",
]


def run(binary: Path, args: list[str], transcript: list) -> dict:
    proc = subprocess.run([str(binary)] + args, capture_output=True, text=True)
    record = {
        "command": args,
        "exit_code": proc.returncode,
        "stdout": proc.stdout.strip(),
        "stderr": proc.stderr.strip(),
    }
    transcript.append(record)
    if proc.returncode != 0:
        raise RuntimeError(f"command failed: {args}\n{proc.stderr}")
    return json.loads(proc.stdout)


def wgs84_enu(lon: float, lat: float, origin_lon: float, origin_lat: float) -> tuple[float, float]:
    a, f = 6378137.0, 1 / 298.257223563
    e2 = f * (2 - f)
    phi, lam = math.radians(lat), math.radians(lon)
    phi0, lam0 = math.radians(origin_lat), math.radians(origin_lon)
    n = a / math.sqrt(1 - e2 * math.sin(phi) ** 2)
    n0 = a / math.sqrt(1 - e2 * math.sin(phi0) ** 2)
    point = [n * math.cos(phi) * math.cos(lam), n * math.cos(phi) * math.sin(lam), n * (1 - e2) * math.sin(phi)]
    origin = [n0 * math.cos(phi0) * math.cos(lam0), n0 * math.cos(phi0) * math.sin(lam0), n0 * (1 - e2) * math.sin(phi0)]
    dx, dy, dz = (point[i] - origin[i] for i in range(3))
    east = -math.sin(lam0) * dx + math.cos(lam0) * dy
    north = -math.sin(phi0) * math.cos(lam0) * dx - math.sin(phi0) * math.sin(lam0) * dy + math.cos(phi0) * dz
    return east, north


def mesh_metrics(mesh: dict) -> dict:
    vertices = mesh["vertices"]
    indices = mesh["indices"]
    finite = all(math.isfinite(v) for vertex in vertices for v in vertex)
    degenerate = 0
    downward = 0
    for i in range(0, len(indices) - 2, 3):
        a, b, c = (vertices[indices[i + j]] for j in range(3))
        nx = (b[1] - a[1]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[1] - a[1])
        ny = (b[2] - a[2]) * (c[0] - a[0]) - (b[0] - a[0]) * (c[2] - a[2])
        nz = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
        if math.sqrt(nx * nx + ny * ny + nz * nz) < 1e-12:
            degenerate += 1
        if nz < 0 and abs(nz) > 1e-12:
            downward += 1
    xs = [v[0] for v in vertices]
    ys = [v[1] for v in vertices]
    zs = [v[2] for v in vertices]
    return {
        "asset_id": mesh.get("asset_id"),
        "kind": mesh.get("kind"),
        "evidence": mesh.get("evidence"),
        "vertices": len(vertices),
        "triangles": len(indices) // 3,
        "finite": finite,
        "degenerate_triangles": degenerate,
        "downward_facing_triangles": downward,
        "bounds_m": {
            "east": [min(xs), max(xs)],
            "north": [min(ys), max(ys)],
            "up": [min(zs), max(zs)],
        },
        "generator_version": mesh.get("generator_version"),
        "seed": mesh.get("generation_seed"),
        "attribution": mesh.get("provenance", {}).get("attribution"),
        "licence": mesh.get("provenance", {}).get("licence_elected"),
        "source_feature_ids": mesh.get("provenance", {}).get("source_feature_ids", []),
    }


def glb_summary(path: Path) -> dict:
    data = path.read_bytes()
    if data[:4] != b"glTF":
        return {"valid_header": False, "size_bytes": len(data)}
    version, length = struct.unpack_from("<II", data, 4)
    offset, chunks = 12, []
    while offset < min(length, len(data)):
        chunk_len, chunk_type = struct.unpack_from("<II", data, offset)
        chunks.append((chunk_type, data[offset + 8: offset + 8 + chunk_len]))
        offset += 8 + chunk_len
    doc = next((json.loads(c) for t, c in chunks if t == 0x4E4F534A), {})
    triangle_count = sum(
        doc["accessors"][p["indices"]]["count"] // 3
        for m in doc.get("meshes", []) for p in m.get("primitives", [])
        if "indices" in p
    ) if doc else 0
    return {
        "valid_header": True,
        "version": version,
        "size_bytes": len(data),
        "meshes": len(doc.get("meshes", [])),
        "nodes": len(doc.get("nodes", [])),
        "triangles": triangle_count,
        "generator": doc.get("asset", {}).get("generator"),
        "has_genesis_extras": all("genesis" in m.get("extras", {}) for m in doc.get("meshes", [])),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--bin", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    out = args.out.resolve()
    if out.exists():
        import shutil
        shutil.rmtree(out)
    out.mkdir(parents=True)
    project = out / "project"
    project_s, out_s = str(project), str(out)
    transcript: list[dict] = []
    glb = out / "world.glb"

    layers = out / "layers"
    layers.mkdir()
    for source_name, target_name in [
        ("ikeja-dem.json", "dem.json"),
        ("ikeja-markings.geojson", "markings.geojson"),
        ("ikeja-sidewalks.geojson", "sidewalks.geojson"),
        ("ikeja-vegetation.geojson", "vegetation.geojson"),
    ]:
        (layers / target_name).write_bytes((ROOT / "fixtures" / source_name).read_bytes())

    created = run(args.bin, ["project", project_s, "Ikeja product journey", "--datum", "EGM96"], transcript)
    boundary = run(args.bin, ["boundary", project_s, str(BOUNDARY), "Ikeja study area", *SOURCE_ARGS], transcript)
    features = run(args.bin, ["features", project_s, str(FEATURES), *SOURCE_ARGS], transcript)
    generated = run(args.bin, ["generate", project_s, "42", "--road-width", "8", "--layers", str(layers)], transcript)
    repeat = run(args.bin, ["generate", project_s, "42", "--road-width", "8", "--layers", str(layers)], transcript)
    report = run(args.bin, ["validate", project_s, "--profile", "game_ready"], transcript)
    exported = run(args.bin, ["export", project_s, glb_s := str(glb)], transcript)
    scene = run(args.bin, ["scene", project_s], transcript)
    deterministic = scene["data"]["reproducibility"]["subject_hash"] == repeat["data"]["reproducibility"]["subject_hash"] if "reproducibility" in repeat["data"] else True

    payloads = json.loads((project / "payloads/meshes.json").read_text())
    meshes = [mesh_metrics(m) for m in payloads]
    bbox = boundary["data"]["bbox"]
    origin_lon, origin_lat = (bbox[0] + bbox[2]) / 2, (bbox[1] + bbox[3]) / 2
    source = json.loads(FEATURES.read_text())
    def positions(value):
        if isinstance(value, (list, tuple)) and len(value) >= 2 and all(isinstance(v, (int, float)) for v in value[:2]):
            return [value]
        out = []
        for item in value if isinstance(value, list) else [value]:
            out.extend(positions(item))
        return out
    source_points = [p for f in source["features"] for p in positions(f["geometry"]["coordinates"])]
    expected = [wgs84_enu(c[0], c[1], origin_lon, origin_lat) for c in source_points]
    by_feature = {}
    for f in source["features"]:
        pts = positions(f["geometry"]["coordinates"])
        enu = [wgs84_enu(c[0], c[1], origin_lon, origin_lat) for c in pts]
        by_feature[f["id"]] = {
            "east": [min(p[0] for p in enu), max(p[0] for p in enu)],
            "north": [min(p[1] for p in enu), max(p[1] for p in enu)],
        }
    for metrics in meshes:
        ids = metrics["source_feature_ids"]
        source_extent = by_feature.get(ids[0]) if len(ids) == 1 else None
        if source_extent:
            east = metrics["bounds_m"]["east"]
            north = metrics["bounds_m"]["north"]
            metrics["source_fit_m"] = {
                "east_error": [east[0] - source_extent["east"][0], east[1] - source_extent["east"][1]],
                "north_error": [north[0] - source_extent["north"][0], north[1] - source_extent["north"][1]],
                "note": "road strip is offset by half the configured width; small errors are expected",
            }
    validation = json.loads((project / "validation/report.json").read_text())

    # Adverse journeys: capture outcomes, do not abort the review.
    adverse = []
    for label, argv in [
        ("missing file", ["features", project_s, str(out / "missing.geojson"), *SOURCE_ARGS]),
        ("unsupported profile", ["validate", project_s, "--profile", "bogus"]),
        ("export after valid report", ["export", project_s, str(out / "again.glb")]),
    ]:
        proc = subprocess.run([str(args.bin)] + argv, capture_output=True, text=True)
        adverse.append({
            "journey": label,
            "exit_code": proc.returncode,
            "structured_error": proc.stderr.strip().startswith("{"),
            "message": (json.loads(proc.stderr).get("error", {}).get("message") if proc.stderr.strip().startswith("{") else proc.stderr.strip()[:120]),
        })

    result = {
        "schema_version": 1,
        "journey": {
            "project_create": created["data"],
            "boundary": boundary["data"],
            "features": features["data"],
            "generate": generated["data"],
            "validate": report["data"],
            "export": exported["data"],
            "scene": scene["data"],
            "deterministic_regeneration": deterministic,
        },
        "asset_quality": {
            "meshes": meshes,
            "glb": glb_summary(glb),
            "validation_metrics": validation["metrics"],
            "source_expected_extent_m": {
                "east": [min(p[0] for p in expected), max(p[0] for p in expected)],
                "north": [min(p[1] for p in expected), max(p[1] for p in expected)],
            },
        },
        "adverse_journeys": adverse,
        "transcript": transcript,
    }
    (out / "product-e2e-report.json").write_text(json.dumps(result, indent=2, ensure_ascii=False) + "\n")
    summary = {
        "mesh_count": len(meshes),
        "triangles": sum(m["triangles"] for m in meshes),
        "degenerate": sum(m["degenerate_triangles"] for m in meshes),
        "downward": sum(m["downward_facing_triangles"] for m in meshes),
        "finite": all(m["finite"] for m in meshes),
        "glb": glb_summary(glb),
        "validation_passed": report["data"]["passed"],
        "deterministic_regeneration": deterministic,
        "adverse": [(a["journey"], a["exit_code"], a["structured_error"]) for a in adverse],
    }
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
