# Genesis Studio shell (GE-0016)

A minimal, headless-testable React/TypeScript presentation layer for Genesis
Core scene snapshots. **It is not a packaged Tauri window yet.** The Rust
command wrappers live in `crates/genesis-studio` behind the `tauri-bindings`
feature; this directory contains only the webview-facing UI.

## Data contract

The only input is a versioned `SceneSnapshot` JSON document produced by
`genesis_studio::build_scene_snapshot` in Rust (snapshot schema version 1).
For local development it can be injected as `window.__GENESIS_SNAPSHOT__` before
`src/main.tsx` runs. No domain logic is implemented here:

- IDs, labels, method strings and evidence come from the snapshot.
- Vertices/indices are Core's local-frame triangle mesh, uploaded as-is.
- Translation inputs edit a local preview transform (UI interaction state);
  they do not write to Core or perform any CRS maths.
- Source, attribution, licence, method and confidence are displayed as recorded
  by Core's provenance.

`src/Hierarchy.tsx` uses ARIA `tree`/`treeitem` semantics with a roving tabindex
and arrow/Home/End/Enter/Space keyboard navigation. `src/Inspector.tsx` provides
`type="number"` inputs as the keyboard-numeric alternative for transforms.
`src/Scene.tsx` probes WebGL2/WebGPU and degrades to a text-only metadata view
when WebGL2 is unavailable (WebGPU alone is not used by the WebGL-based R3F
renderer).

## Run

```sh
npm install
npm run typecheck     # npx tsc --noEmit
npm run dev           # Vite dev server; snapshot is empty unless injected
npm run build         # typecheck + production bundle
```

## Verification status (honest)

Verified locally:

- `npm install` succeeded (lockfile generated).
- `npx tsc --noEmit` type-checks the sources against the pinned React 19.3.1,
  TypeScript 7.0.2, three 0.187.0, R3F 9.8.2 and drei 10.7.9 versions.

**UNVERIFIED — do not read this UI as visually or platform validated:**

- No actual Tauri window was built, launched or screenshotted; no Tauri config,
  bundler, updater or signing identity is part of this ticket.
- WebGL2/WebGPU availability in WebView2 (Windows), WKWebView (macOS) and
  WebKitGTK (Linux) was not probed in a real webview. The DOM probe above is
  only a local browser call.
- No frame was rendered, so viewport correctness, camera framing and visual
  appearance were **not** verified.
- GPU resource leaks, long-session memory behaviour, 100k-object picking and
  LOD streaming were not exercised.
- Auto-update and signed release builds were not exercised (require owner
  approval and signing identities).
- No browser-based component test suite was added here; acceptance evidence is
  the Rust snapshot tests plus type-checking only.
