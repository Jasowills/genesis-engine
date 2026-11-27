# GE-0004 capability PoC — results (throwaway evidence)

**Date:** 2026-11-27. **Status:** evidence only, not a decision, not product
code. **Host used:** Intel macOS x86_64 (`Darwin ... RELEASE_X86_64`).

## Scope distinction (read first)

The Rust probe is **native GPU evidence**: it enumerates adapters through the
`wgpu` crate (here: Metal adapters). It is **not** evidence that WebGL2 or
WebGPU is available in WebView2, WKWebView, WebKitGTK, or Electron's bundled
Chromium. Those are separate runtimes with separate `navigator.gpu` / WebGL2
capability and flags. No such webview probe was run. The Node probe is
**CPU-only**: it constructs `BatchedMesh` and raycasts with three.js classes;
it never creates a WebGL/WebGPU context or renders a frame. Consequently
FPS, texture limits, GPU upload cost, shader compilation and webview behaviour
are not measured here.

Floating-origin golden vectors at ~1e7 m are **not** reproduced in this PoC;
they belong to GE-0003 (`TICKETS/ge-0003-precision-and-crs-spike.md`,
`prototype/precision-crs/RESULTS.md`). No precision claim is made here.

## Measured summary

| Item | Result | Source |
| --- | --- | --- |
| wgpu version (resolved) | `30.0.2` | `cargo tree -p wgpu --depth 0`, `Cargo.lock` |
| OS / arch | `macos` / `x86_64` | probe JSON, `uname -a` |
| wgpu backends compiled in | `Backends(METAL)` | probe JSON |
| Native adapters found | `2` — AMD Radeon Pro 5300M (discrete), Intel UHD Graphics 630 (integrated), both Metal | probe JSON |
| Graceful no-adapter path | Implemented; emits `status: "no_adapter"`, exit 0. **Not exercised** on a GPU-less runner (all recorded runs had adapters) | code review, not measured |
| three.js / R3F / drei | `0.187.0` (r187) / `9.8.2` / `10.7.9` | `npm ls --depth=0`, probe JSON |
| TypeScript | `7.0.2` | `npx tsc --version` |
| Batched instances | `100000` constructed | probe JSON |
| Raycaster picks | `16`, `matches: 16`, `mismatches: 0`, `pass: true` | probe JSON |
| Timings (single run, NOT a benchmark) | build `141.35 ms`; 16 picks `465.14 ms` total, `29.07 ms`/pick | probe JSON |

## Verbatim transcript — toolchain and host

```console
$ cargo --version
cargo 1.98.1 (797e8a9bc 2026-08-05)
$ rustc --version
rustc 1.98.1 (48a229cea 2026-09-01)
$ node --version
v24.16.0
$ npm --version
11.18.0
$ uname -a
Darwin Jasons-MacBook-Pro-2.local 25.5.0 Darwin Kernel Version 25.5.0: Tue Jun  9 22:26:23 PDT 2026; root:xnu-12377.121.10~1/RELEASE_X86_64 x86_64
```

## Verbatim transcript — wgpu 30 native probe

```console
$ cargo generate-lockfile
    Updating crates.io index
     Locking 125 packages to latest Rust 1.98.1 compatible versions
      Adding pollster v0.4.0 (available: v1.0.3)
```

`Cargo.lock` is committed in this directory. First `cargo generate-lockfile`
before adding `pollster` locked 124 packages; the final lock has 125.

```console
$ cargo tree -p wgpu --depth 0
wgpu v30.0.2
[build-dependencies]
$ cargo build --locked
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
$ cargo fmt -- --check
$ cargo clippy --locked -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.54s
```

`cargo fmt -- --check` produced no output (exit 0). Earlier cold builds
compiled wgpu 30.0.2, naga 30.0.2, wgpu-core 30.0.2 and platform crates
successfully; the commands above are the final clean re-runs.

```console
$ cargo run --locked --quiet
{
  "ticket": "GE-0004",
  "scope": "native wgpu adapter enumeration only; NOT webview WebGL2/WebGPU evidence",
  "wgpu_version": "30.0.2",
  "os": "macos",
  "arch": "x86_64",
  "family": "unix",
  "compiled_backends": "Backends(METAL)",
  "requested_backends": "Backends(NOOP | VULKAN | GL | METAL | DX12 | BROWSER_WEBGPU)",
  "adapter_count": 2,
  "status": "ok",
  "adapters": [
    {
      "index": 0, "backend": "metal", "device_type": DiscreteGpu,
      "name": "AMD Radeon Pro 5300M", "driver": "", "driver_info": "",
      "vendor": "0x0", "device": "0x0", "device_pci_bus_id": "", "subgroup_min_size": 4, "subgroup_max_size": 64,
      "features": "Features { features_wgpu: FeaturesWGPU(SHADER_FLOAT32_ATOMIC | TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | MAPPABLE_PRIMARY_BUFFERS | TEXTURE_BINDING_ARRAY | BUFFER_BINDING_ARRAY | STORAGE_RESOURCE_BINDING_ARRAY | SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING | STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING | PARTIALLY_BOUND_BINDING_ARRAY | ADDRESS_MODE_CLAMP_TO_ZERO | ADDRESS_MODE_CLAMP_TO_BORDER | POLYGON_MODE_LINE | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | MULTIVIEW | TEXTURE_ATOMIC | EXTERNAL_TEXTURE | SHADER_I16 | SHADER_INT64 | SUBGROUP | SUBGROUP_BARRIER | EXPERIMENTAL_MESH_SHADER | EXPERIMENTAL_MESH_SHADER_MULTIVIEW | PASSTHROUGH_SHADERS | SHADER_BARYCENTRICS | EXPERIMENTAL_MESH_SHADER_POINTS | MULTISAMPLE_ARRAY | EXPERIMENTAL_COOPERATIVE_MATRIX | MEMORY_DECORATION_COHERENT), features_webgpu: FeaturesWebGPU(DEPTH_CLIP_CONTROL | DEPTH32FLOAT_STENCIL8 | TEXTURE_COMPRESSION_BC | TEXTURE_COMPRESSION_BC_SLICED_3D | INDIRECT_FIRST_INSTANCE | SHADER_F16 | RG11B10UFLOAT_RENDERABLE | BGRA8UNORM_STORAGE | FLOAT32_FILTERABLE | FLOAT32_BLENDABLE | DUAL_SOURCE_BLENDING | CLIP_DISTANCES | IMMEDIATES | PRIMITIVE_INDEX) }",
      "limits": {"max_texture_dimension_2d": 16384, "max_buffer_size": 3208642560, "max_storage_buffer_binding_size": 3208642560, "max_bind_groups": 8, "max_compute_workgroups_per_dimension": 65535}
    },
    {
      "index": 1, "backend": "metal", "device_type": IntegratedGpu,
      "name": "Intel(R) UHD Graphics 630", "driver": "", "driver_info": "",
      "vendor": "0x0", "device": "0x0", "device_pci_bus_id": "", "subgroup_min_size": 4, "subgroup_max_size": 64,
      "features": "Features { features_wgpu: FeaturesWGPU(SHADER_FLOAT32_ATOMIC | TEXTURE_FORMAT_16BIT_NORM | TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES | MAPPABLE_PRIMARY_BUFFERS | TEXTURE_BINDING_ARRAY | BUFFER_BINDING_ARRAY | STORAGE_RESOURCE_BINDING_ARRAY | SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING | STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING | PARTIALLY_BOUND_BINDING_ARRAY | ADDRESS_MODE_CLAMP_TO_ZERO | ADDRESS_MODE_CLAMP_TO_BORDER | POLYGON_MODE_LINE | VERTEX_WRITABLE_STORAGE | CLEAR_TEXTURE | TEXTURE_ATOMIC | EXTERNAL_TEXTURE | SHADER_I16 | SHADER_INT64 | SUBGROUP | SUBGROUP_BARRIER | EXPERIMENTAL_MESH_SHADER | PASSTHROUGH_SHADERS | SHADER_BARYCENTRICS | EXPERIMENTAL_MESH_SHADER_POINTS | MULTISAMPLE_ARRAY | EXPERIMENTAL_COOPERATIVE_MATRIX | MEMORY_DECORATION_COHERENT), features_webgpu: FeaturesWebGPU(DEPTH_CLIP_CONTROL | DEPTH32FLOAT_STENCIL8 | TEXTURE_COMPRESSION_BC | TEXTURE_COMPRESSION_BC_SLICED_3D | INDIRECT_FIRST_INSTANCE | SHADER_F16 | RG11B10UFLOAT_RENDERABLE | BGRA8UNORM_STORAGE | FLOAT32_FILTERABLE | FLOAT32_BLENDABLE | DUAL_SOURCE_BLENDING | CLIP_DISTANCES | IMMEDIATES | PRIMITIVE_INDEX) }",
      "limits": {"max_texture_dimension_2d": 16384, "max_buffer_size": 2147483648, "max_storage_buffer_binding_size": 2147483648, "max_bind_groups": 8, "max_compute_workgroups_per_dimension": 65535}
    }
  ]
}
```

Notes on this output: `driver`, `driver_info`, `vendor`, `device` and
`device_pci_bus_id` are empty/zero as returned by the Metal backend on this
host; they are reported verbatim, not filled in. `features` is the exact
`Debug` rendering of `Adapter::features()` for wgpu 30.0.2 (the `features_webgpu`
subset is the WebGPU-equivalent feature set exposed natively). No device,
queue, surface or pipeline was created.

## Verbatim transcript — Node/three.js probe

First install (captured through `| tail -40`; all emitted lines are shown):

```console
$ npm install 2>&1 | tail -40
added 67 packages, and audited 68 packages in 2m

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
```

Verification re-run:

```console
$ npm install
up to date, audited 68 packages in 1s

3 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
NPM_EXIT=0
$ npm ls --depth=0
viewport-capability-probe@0.1.0 /Users/jasonamadi/genesis-engine/prototype/viewport-capability
├── @react-three/drei@10.7.9
├── @react-three/fiber@9.8.2
├── three@0.187.0
└── typescript@7.0.2
```

`npm install` succeeded, so `package-lock.json` is included. No install error
to record.

```console
$ node batched-pick.mjs
{
  "ticket": "GE-0004",
  "scope": "CPU-only three.js BatchedMesh/Raycaster batchId mapping; NO WebGL/WebGPU context created",
  "node_version": "v24.16.0",
  "three_version": "0.187.0",
  "r3f_version": "9.8.2",
  "drei_version": "10.7.9",
  "instances": 100000,
  "max_instance_count": 100000,
  "vertex_count_per_geometry": 24,
  "index_count_per_geometry": 36,
  "grid_cols": 317,
  "picks": 16,
  "matches": 16,
  "mismatches": 0,
  "mismatch_details": [],
  "hits_per_pick": [
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2,
    2
  ],
  "build_ms": 141.35,
  "pick_ms_total": 465.14,
  "pick_ms_per_pick": 29.07,
  "pass": true,
  "note": "batchId is the BatchedMesh instance id (three.js r187 BatchedMesh.raycast). Mapping table is slot -> instanceId; no GPU resources involved."
}
```

The script exits 0 on `pass: true`, 1 on any mismatch.

`hits_per_pick: 2` is the two triangles of the box's top face; only the first
hit's `batchId` is asserted. The slot→instanceId map is a deterministic
permutation, so the check is non-trivial even though `addInstance` returns
sequential IDs.

## Verbatim transcript — TypeScript tooling

```console
$ npx tsc --version
Version 7.0.2
```

This checks compiler availability only. It does not verify TS 7 compatibility
with Vite/R3F/drei plugins, React 19 types, generated bindings, test runners or
any bundler configuration.

## Development iterations (not capability evidence)

These failures occurred while adapting to the pinned versions and were fixed;
they are recorded for transparency, not as measured capability results.

- First `cargo build --locked` failed against wgpu 30 API drift: `cannot find
  function backends in crate wgpu`, `cannot find value VERSION in crate wgpu`,
  no `Backend::GL`/`Dx11`/`Null`/`BrowserWebGL`/`BrowserWebGPU` variants,
  `InstanceDescriptor: Default` not satisfied, `Instance::new` takes the
  descriptor by value, and `enumerate_adapters` returns a `Future` (no
  `.len()`/`.iter()`). Fixed with `Instance::enabled_backend_features()`, a
  `build.rs` that reads the exact version from `Cargo.lock`, `Backend::to_str()`,
  direct descriptor construction and `pollster::block_on`.
- Second `cargo build --locked` failed with `no field architecture on type
  AdapterInfo`; wgpu 30 exposes `device_pci_bus_id`, `subgroup_min_size` and
  `subgroup_max_size` instead. Fixed.
- First `node batched-pick.mjs` failed with `ERR_PACKAGE_PATH_NOT_EXPORTED:
  Package subpath './package.json' is not defined by "exports" in .../three/
  package.json`. Fixed by reading `node_modules/<pkg>/package.json` from disk.
- First `cargo clippy --locked -- -D warnings` failed with
  `clippy::collapsible_if` in `build.rs`. Fixed with a let-chain; final clippy
  run is clean (shown above).

## UNVERIFIED — explicitly not measured here

1. **WebView2 (Windows), WKWebView (macOS), WebKitGTK (Linux) WebGL2 and WebGPU
   availability.** No webview was launched, no context was requested, no flags
   or version-specific behaviour was probed. The native Metal adapters above
   do not imply webview WebGPU/WebGL2 support.
2. **Electron Chromium comparison.** No Electron app was built or run; no
   startup, WebGL/WebGPU determinism, bundle size or memory comparison was
   made. The ADR's "consistent Chromium" claim was not tested here.
3. **Actual Tauri app packaging and signing.** No Tauri app, bundler run,
   notarisation, MSI/NSIS signing, AppImage, or updater endpoint was exercised.
   No signing identity was installed or used (ticket forbids paid identities).
4. **GPU resource leak under long sessions.** No renderer, no load/unload
   stress loop, no `renderer.info`, no repeated create/dispose cycle and no
   long-run memory sampling was performed. The three.js disposal/leak concern
   in `research/notes/desktop-viewport-formats.md` remains open.
5. **Cross-platform matrix — measured.** CI runs `45996397293` and
   `45996483515` executed both probes on ubuntu-latest, windows-latest and
   macos-latest: all jobs success. macOS/aarch64 METAL 1 adapter, Windows/x86_64
   1 adapter, Linux/x86_64 0 adapters (`no_adapter`); three.js 100k picking 0
   mismatches and TypeScript 7.0.2 on all three OSes.
6. **FPS, frame time, texture limits, shader compilation, GPU upload and
   rendering correctness.** No WebGL/WebGPU context exists in either probe;
   the 29.07 ms/pick figure is a CPU raycast measurement, not a frame time.
7. **TS 7 ecosystem compatibility** beyond `tsc --version` (see above).
8. **No-adapter runtime path — exercised.** Linux CI reported
   `"adapter_count": 0` and `"status": "no_adapter"` and the job succeeded;
   the no-backend branch remains code-reviewed only.
9. **1M-instance picking, gizmo/snap behaviour, keyboard-numeric transform
   path, LOD hysteresis, 3D Tiles/USD round-trips and streaming.** Not in this
   PoC's scope and not measured.
10. **Independent review and any decision gate.** This document records
    evidence only; the ticket is not complete until reviewed per GE-0004.

## Scope compliance

No tickets, ADRs, root documents, CI workflow, other prototypes or root
manifest were modified. No secrets or credentials were used. Generated
`target/` and `node_modules/` are ignored via `.gitignore`.
