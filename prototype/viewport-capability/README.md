# GE-0004 — viewport capability PoC (throwaway evidence)

**This is throwaway evidence, not product code.** Nothing here is imported by
Genesis Core or the Studio. It exists only to record measured capability facts
and version numbers so the shell decision (ADR-0003 D-011) and the viewport
decision (D-012) can stay separate. Results and exact transcripts are in
[RESULTS.md](RESULTS.md).

## What is measured, and what is not

| Part | Measures | Does **not** measure |
| --- | --- | --- |
| `src/main.rs` (wgpu 30 probe) | Native GPU adapters, backends, adapter features/limits on the host OS/arch | **WebGL2/WebGPU inside WebView2, WKWebView, WebKitGTK or Electron Chromium.** These are different runtimes. A native Metal/Vulkan/DX12 adapter says nothing about a webview's `navigator.gpu` or WebGL2 context. |
| `batched-pick.mjs` (three.js r187 probe) | CPU-side `BatchedMesh` construction at ~100k instances and `Raycaster` `batchId` mapping; R3F/drei/three version metadata | Any rendered frame, GPU upload, shader compilation, FPS, texture limits, or webview behaviour. No WebGL/WebGPU context is created. |

Floating-origin / ~1e7 m golden vectors are **not** duplicated here — see
`TICKETS/ge-0003-precision-and-crs-spike.md` and
`prototype/precision-crs/RESULTS.md`. This PoC does not re-measure precision.

## Files

- `Cargo.toml`, `Cargo.lock`, `build.rs`, `src/main.rs` — standalone wgpu probe.
  The empty `[workspace]` table keeps it out of the repository root workspace
  without modifying the root manifest. `build.rs` reads the resolved wgpu
  version out of `Cargo.lock` (wgpu 30 no longer exports `VERSION`).
- `package.json`, `package-lock.json`, `batched-pick.mjs` — Node/ESM probe.
- `RESULTS.md` — verbatim commands, outputs and explicit UNVERIFIED items.
- `.gitignore` — generated `target/` and `node_modules/` are intentionally not
  source files.

## Run locally (macOS x86_64 host used for the recorded evidence)

Prerequisites: Rust toolchain (recorded: cargo/rustc 1.98.1), Node.js
(recorded: v24.16.0), npm (recorded: 11.18.0). First run needs registry access
to fetch crates and npm packages.

```sh
cd prototype/viewport-capability

# Native GPU evidence
cargo generate-lockfile
cargo build --locked
cargo run --locked --quiet          # structured JSON on stdout, exit 0 even with no adapter

# CPU-only three.js evidence
npm install
node batched-pick.mjs               # JSON on stdout, exit 0 on pass, 1 on mismatch

# TypeScript 7 tooling availability
npx tsc --version

# Optional hygiene checks
cargo fmt -- --check
cargo clippy --locked -- -D warnings
```

### Knobs

- `GE4_INSTANCES` — instance count for the batched probe (default `100000`).
- `GE4_PICKS` — number of raycast picks (default `16`).

The probe uses a fixed PRNG seed and a deterministic grid/permutation, so
`matches`, `mismatches` and `pass` are reproducible; timings are not.

## Running in the existing GitHub Actions matrix

The `.github/workflows/ci.yml` matrix is `ubuntu-latest`, `windows-latest`,
`macos-latest`. **This prototype does not modify CI.** To gather cross-OS
evidence, add these steps to the existing `prototype-tests` job (or run them in
a temporary branch/workflow); no GPU runner or secrets are required:

```yaml
      - name: GE-0004 native GPU probe
        shell: bash
        run: cargo run --locked --quiet --manifest-path prototype/viewport-capability/Cargo.toml

      - name: GE-0004 three.js CPU batch/pick probe
        working-directory: prototype/viewport-capability
        run: |
          npm ci
          node batched-pick.mjs
          npx tsc --version
```

Notes for CI interpretation:

- Public runners have no guaranteed GPU. `cargo run` may report
  `adapter_count: 0` / `status: "no_adapter"`; that is a **valid, graceful**
  result and must be recorded as "native GPU capability unverified on that
  runner", not as a failure.
- Linux headless runners commonly expose no usable adapter without Mesa/Vulkan
  setup; do not install system packages or change the runner image for this
  throwaway probe.
- Windows will use DX12/Vulkan backends, macOS Metal, Linux Vulkan/GL if an
  adapter exists. Compare with the macOS x86_64 evidence in RESULTS.md; do not
  merge results into a single "all-OS verified" claim unless each OS is actually
  measured.

## Honesty notes

- Timings in RESULTS.md are single-run wall-clock numbers on one Intel macOS
  host; they are not benchmarks and carry no statistical confidence.
- `hits_per_pick` of 2 is expected: a vertical ray crosses both triangles of a
  box's top face; the first hit's `batchId` is the value checked.
- No signing identities, credentials or secrets are used or needed.
- Status: evidence only. Independent review and any decision gate belong to
  the ticket, not to this prototype.
