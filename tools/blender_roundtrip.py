"""Blender-side GLB round-trip probe for GE-0029.

Run as an external process; never import Blender or GPL code into Genesis Core:

    blender --background --python tools/blender_roundtrip.py -- \
        <input.glb> <output.glb> <report.json>

The script records Blender's version, hierarchy, material names and custom
properties before and after an import/export cycle, then writes a JSON report
describing exactly what survived. It does not modify the input file.
"""

import json
import sys
from pathlib import Path

import bpy


def plain(value):
    """Convert Blender ID properties (including nested groups) to JSON values."""
    if value is None or isinstance(value, (str, int, float, bool)):
        return value
    if hasattr(value, "items"):
        return {str(k): plain(v) for k, v in value.items()}
    if isinstance(value, (list, tuple)):
        return [plain(v) for v in value]
    return str(value)


def scene_state():
    objects = []
    for obj in bpy.data.objects:
        parent = obj.parent.name if obj.parent else None
        props = {}
        # glTF extras can land on the object or on its mesh data depending on
        # the importer version; record both, object values take precedence.
        for source in (getattr(obj, "data", None), obj):
            if source is None:
                continue
            for key in source.keys():
                if key == "_RNA_UI":
                    continue
                props[key] = plain(source[key])
        materials = [slot.material.name for slot in obj.material_slots if slot.material]
        objects.append(
            {
                "name": obj.name,
                "type": obj.type,
                "parent": parent,
                "materials": materials,
                "custom_properties": props,
            }
        )
    return {
        "objects": sorted(objects, key=lambda o: o["name"]),
        "material_count": len(bpy.data.materials),
    }


def main():
    argv = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    if len(argv) != 3:
        raise SystemExit("usage: blender --background --python blender_roundtrip.py -- IN.glb OUT.glb REPORT.json")
    src, dst, report_path = (Path(p).resolve() for p in argv)
    if not src.is_file():
        raise SystemExit(f"input GLB missing: {src}")

    bpy.ops.wm.read_factory_settings(use_empty=True)
    bpy.ops.import_scene.gltf(filepath=str(src))
    before = scene_state()

    bpy.ops.export_scene.gltf(
        filepath=str(dst),
        export_format="GLB",
        export_extras=True,
        export_yup=True,
        export_apply=True,
    )
    after = scene_state()

    before_names = {o["name"] for o in before["objects"]}
    after_names = {o["name"] for o in after["objects"]}
    props_before = {k for o in before["objects"] for k in o["custom_properties"]}
    props_after = {k for o in after["objects"] for k in o["custom_properties"]}
    meta_before = {o["name"]: o["custom_properties"] for o in before["objects"]}
    meta_after = {o["name"]: o["custom_properties"] for o in after["objects"]}
    metadata_loss = sorted(
        name for name in meta_before if meta_before[name] != meta_after.get(name)
    )

    report = {
        "schema_version": 1,
        "blender_version": bpy.app.version_string,
        "input": src.name,
        "output": dst.name,
        "objects_before": len(before["objects"]),
        "objects_after": len(after["objects"]),
        "object_names_preserved": before_names == after_names,
        "missing_object_names": sorted(before_names - after_names),
        "materials_before": before["material_count"],
        "materials_after": after["material_count"],
        "custom_property_keys_before": sorted(props_before),
        "custom_property_keys_after": sorted(props_after),
        "custom_properties_preserved": props_before.issubset(props_after),
        "metadata_identical": not metadata_loss,
        "metadata_loss": metadata_loss,
        "before": before,
        "after": after,
        "limitations": [
            "Blender may rename objects or merge materials across versions",
            "Custom property preservation depends on glTF extras support",
            "This report describes one pinned Blender version only",
        ],
    }
    Path(report_path).write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({
        "blender_version": report["blender_version"],
        "objects_before": report["objects_before"],
        "objects_after": report["objects_after"],
        "custom_properties_preserved": report["custom_properties_preserved"],
    }))


if __name__ == "__main__":
    main()
