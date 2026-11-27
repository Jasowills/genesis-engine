# Unreal export fixture contract (GE-0031)

Unreal is the owner-selected first target engine (D-034). **No Unreal support is claimed yet**: the fixture has not been executed because no Unreal Editor is installed on this host and Epic account login is a manual step.

## Required mapping (to be measured, not assumed)

| Concern | Expected mapping | Verification status |
| --- | --- | --- |
| Axis | glTF Y-up right-handed → Unreal Z-up left-handed | UNVERIFIED until probe runs |
| Units | glTF metres → Unreal centimetres (×100) | UNVERIFIED |
| Materials | glTF PBR → Unreal material slots | UNVERIFIED |
| Collision | glTF mesh → Unreal simple/complex collision | UNVERIFIED |
| Metadata | glTF `extras.genesis` → Unreal asset metadata | Expected **loss** with stock importer; must be recorded, not hidden |

## Probe

`tools/unreal_import_probe.py` runs *inside* an installed Unreal Editor via `UnrealEditor-Cmd -run=pythonscript`, imports a Core-exported GLB and writes a JSON report with the pinned engine version, imported assets, vertex/triangle counts, material slots, collision primitives and explicit limitations. It is defensive across Unreal versions: unavailable observations are reported as such rather than guessed.

The script is an external process boundary. No Unreal SDK, binary or proprietary code is vendored into Genesis Core.

## Gate

GE-0031 may only be marked `verified` after the probe has been executed on a pinned Unreal version, its report committed, and metadata loss documented. Acceptance criterion "each fixture passes on all supported OSes" cannot be met on a host without Unreal; do not reinterpret the contract as a passing fixture.
