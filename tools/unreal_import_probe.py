"""Unreal-side GLB import probe for GE-0031.

Run *inside* an installed Unreal Editor, never inside Genesis Core:

    UnrealEditor-Cmd <project> -run=pythonscript \
        -script="tools/unreal_import_probe.py -- <input.glb> <report.json>"

The script records the exact Unreal version, import conversion settings and
what survived the import. It is deliberately defensive: APIs differ between
Unreal versions, so every optional observation is recorded as available or
unavailable rather than assumed. No support claim is valid until this probe
has been executed against a pinned Unreal version and its report committed.
"""

import json
import sys

import unreal


def optional(call, default=None):
    try:
        return call()
    except Exception as error:  # noqa: BLE001 - report, never crash the probe
        return {"unavailable": str(error)[:200]} if default is None else default


def main():
    argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    if len(argv) != 2:
        raise SystemExit("usage: UnrealEditor-Cmd <project> -run=pythonscript -script=... -- IN.glb REPORT.json")
    source, report_path = argv

    tools = unreal.AssetToolsHelpers.get_asset_tools()
    task = unreal.AssetImportTask()
    task.filename = source
    task.destination_path = "/Game/GenesisImport"
    task.destination_name = "GenesisRoundtrip"
    task.automated = True
    task.replace_existing = True
    task.save = True
    tools.import_asset_tasks([task])

    imported_paths = optional(
        lambda: [str(path) for path in task.imported_object_paths], default=[]
    )
    assets = []
    for path in imported_paths:
        asset = unreal.EditorAssetLibrary.load_asset(str(path))
        if asset is None:
            continue
        assets.append(
            {
                "path": str(path),
                "class": optional(lambda: asset.get_class().get_name()),
                "vertices": optional(
                    lambda: unreal.EditorStaticMeshLibrary.get_number_verts(asset, 0)
                ),
                "triangles": optional(
                    lambda: unreal.EditorStaticMeshLibrary.get_number_triangles(asset, 0)
                    if hasattr(unreal.EditorStaticMeshLibrary, "get_number_triangles")
                    else None
                ),
                "collision_primitives": optional(
                    lambda: len(asset.get_editor_property("body_setup").agg_geom)
                    if hasattr(asset, "get_editor_property")
                    else None
                ),
                "material_slots": optional(
                    lambda: [slot.material_slot_name for slot in asset.static_materials]
                    if hasattr(asset, "static_materials")
                    else []
                ),
            }
        )

    report = {
        "schema_version": 1,
        "unreal_version": unreal.SystemLibrary.get_engine_version(),
        "input": source,
        "imported_assets": assets,
        "axis_conversion": "glTF Y-up right-handed -> Unreal Z-up left-handed (importer default)",
        "unit_conversion": "glTF metres -> Unreal centimetres (1 m = 100 cm)",
        "metadata": "glTF extras are not promoted to Unreal asset metadata by the stock importer",
        "limitations": [
            "No support claim is valid until this probe has been run on a pinned Unreal version",
            "Material and collision preservation depend on importer settings and Unreal version",
            "Proprietary Engine SDK is never redistributed by Genesis Core",
        ],
    }
    with open(report_path, "w") as handle:
        handle.write(json.dumps(report, indent=2) + "\n")
    print(json.dumps({
        "unreal_version": report["unreal_version"],
        "imported_assets": len(assets),
    }))


if __name__ == "__main__":
    main()
