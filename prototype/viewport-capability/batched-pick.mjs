// GE-0004 throwaway capability probe — CPU-only three.js r187 BatchedMesh +
// Raycaster batchId mapping. NO WebGL/WebGPU context is created: only core
// three.js math/geometry/raycasting classes are used. This is evidence that
// r187's BatchedMesh/Raycaster batchId mapping works at ~100k instances on the
// Node CPU path; it is NOT evidence about any webview's GPU capabilities.

import * as THREE from 'three';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const readPkgVersion = (name) =>
  JSON.parse(readFileSync(join(here, 'node_modules', name, 'package.json'), 'utf8')).version;

const INSTANCES = Number.parseInt(process.env.GE4_INSTANCES ?? '100000', 10);
const PICKS = Number.parseInt(process.env.GE4_PICKS ?? '16', 10);
const SPACING = 2;
const COLS = Math.ceil(Math.sqrt(INSTANCES));

// Deterministic permutation: assigns grid slots to instance IDs in a
// non-trivial order so the slot -> instanceId map is genuinely exercised.
let seed = 0x9e3779b9;
const random = () => {
  seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0;
  return seed / 0x100000000;
};
const slots = Array.from({ length: INSTANCES }, (_, i) => i);
for (let i = INSTANCES - 1; i > 0; i--) {
  const j = Math.floor(random() * (i + 1));
  [slots[i], slots[j]] = [slots[j], slots[i]];
}

const slotPosition = (slot) => ({
  x: (slot % COLS) * SPACING,
  z: Math.floor(slot / COLS) * SPACING,
});

const t0 = performance.now();
const geometry = new THREE.BoxGeometry(1, 1, 1);
const vertexCount = geometry.attributes.position.count;
const indexCount = geometry.index.count;
const batch = new THREE.BatchedMesh(
  INSTANCES,
  vertexCount,
  indexCount,
  new THREE.MeshBasicMaterial(),
);
const geometryId = batch.addGeometry(geometry);

// logical grid slot -> instance id returned by addInstance()
const logicalToInstance = new Map();
const matrix = new THREE.Matrix4();
let assigned = 0;
for (let instanceId = 0; instanceId < INSTANCES; instanceId++) {
  const slot = slots[instanceId];
  const { x, z } = slotPosition(slot);
  matrix.makeTranslation(x, 0, z);
  const id = batch.addInstance(geometryId);
  if (id !== instanceId) {
    throw new Error(`addInstance returned ${id}, expected ${instanceId}`);
  }
  batch.setMatrixAt(id, matrix);
  logicalToInstance.set(slot, id);
  assigned++;
}
batch.computeBoundingBox();
batch.computeBoundingSphere();
batch.updateMatrixWorld(true);
const buildMs = performance.now() - t0;

// Pick a spread of distinct logical slots (deterministic).
const pickSlots = Array.from({ length: PICKS }, (_, k) =>
  Math.floor(((k + 0.5) / PICKS) * INSTANCES),
);
const raycaster = new THREE.Raycaster();
const origin = new THREE.Vector3();
const down = new THREE.Vector3(0, -1, 0);

let matches = 0;
const mismatches = [];
const hitsFound = [];
const t1 = performance.now();
for (const slot of pickSlots) {
  const expected = logicalToInstance.get(slot);
  const { x, z } = slotPosition(slot);
  origin.set(x, 10, z);
  raycaster.set(origin, down);
  raycaster.near = 0;
  raycaster.far = 100;
  const hits = raycaster.intersectObject(batch, false);
  const first = hits[0];
  if (first) hitsFound.push(hits.length);
  if (first && first.batchId === expected) {
    matches++;
  } else {
    mismatches.push({
      slot,
      expectedInstanceId: expected,
      gotBatchId: first ? first.batchId : null,
      hitCount: hits.length,
    });
  }
}
const pickMs = performance.now() - t1;

const report = {
  ticket: 'GE-0004',
  scope: 'CPU-only three.js BatchedMesh/Raycaster batchId mapping; NO WebGL/WebGPU context created',
  node_version: process.version,
  three_version: readPkgVersion('three'),
  r3f_version: readPkgVersion('@react-three/fiber'),
  drei_version: readPkgVersion('@react-three/drei'),
  instances: assigned,
  max_instance_count: INSTANCES,
  vertex_count_per_geometry: vertexCount,
  index_count_per_geometry: indexCount,
  grid_cols: COLS,
  picks: PICKS,
  matches,
  mismatches: mismatches.length,
  mismatch_details: mismatches,
  hits_per_pick: hitsFound,
  build_ms: Number(buildMs.toFixed(2)),
  pick_ms_total: Number(pickMs.toFixed(2)),
  pick_ms_per_pick: Number((pickMs / PICKS).toFixed(2)),
  pass: assigned === INSTANCES && matches === PICKS && mismatches.length === 0,
  note: 'batchId is the BatchedMesh instance id (three.js r187 BatchedMesh.raycast). Mapping table is slot -> instanceId; no GPU resources involved.',
};
console.log(JSON.stringify(report, null, 2));
process.exitCode = report.pass ? 0 : 1;
