/** Mirror of the Rust `SceneSnapshot` data contract (GE-0016). Presentation only. */
export type Vec3 = [number, number, number];

export type Evidence =
  | 'measured'
  | 'derived_with_confidence'
  | 'inferred_by_rule'
  | 'unknown';

export type SceneNodeKind = 'building_volume' | 'road_surface' | 'terrain' | 'vegetation';

export interface Provenance {
  source_id: string;
  licence_elected: string;
  attribution: string;
  publisher: string | null;
  url: string | null;
  version: string | null;
  licence_url: string | null;
  vintage: string | null;
  filters: string[];
  checksum: string | null;
  source_feature_ids: string[];
  derivation_chain: string[];
  derivative_database_status: string | null;
  display_obligations: string[];
  third_party_terms: string[];
  recorded_at_unix: number | null;
}

export interface SceneMesh {
  vertices: Vec3[];
  indices: number[];
}

export interface SceneNode {
  id: string;
  label: string;
  kind: SceneNodeKind;
  selectable: boolean;
  translation: Vec3;
  origin_lon_lat: [number, number];
  mesh: SceneMesh;
  evidence: Evidence;
  method: string;
  provenance: Provenance;
}

export interface SceneSnapshot {
  snapshot_version: number;
  world_id: string;
  frame_id: string;
  selected_node_id: string | null;
  nodes: SceneNode[];
}

/** Boundary check for an injected JSON snapshot; no domain rules are applied. */
export function isSceneSnapshot(value: unknown): value is SceneSnapshot {
  if (typeof value !== 'object' || value === null) return false;
  const candidate = value as Partial<SceneSnapshot>;
  return (
    candidate.snapshot_version === 1 &&
    typeof candidate.world_id === 'string' &&
    typeof candidate.frame_id === 'string' &&
    Array.isArray(candidate.nodes) &&
    candidate.nodes.every(
      (node) =>
        typeof node.id === 'string' &&
        typeof node.label === 'string' &&
        typeof node.kind === 'string' &&
        typeof node.selectable === 'boolean' &&
        typeof node.method === 'string' &&
        typeof node.evidence === 'string' &&
        Array.isArray(node.translation) &&
        node.translation.length === 3 &&
        node.translation.every((v) => typeof v === 'number' && Number.isFinite(v)) &&
        Array.isArray(node.mesh?.vertices) &&
        node.mesh.vertices.every((v) => Array.isArray(v) && v.length === 3) &&
        Array.isArray(node.mesh?.indices) &&
        node.mesh.indices.every((i) => typeof i === 'number' && Number.isInteger(i)) &&
        typeof node.provenance === 'object' &&
        node.provenance !== null &&
        typeof node.provenance.source_id === 'string' &&
        typeof node.provenance.attribution === 'string' &&
        typeof node.provenance.licence_elected === 'string' &&
        Array.isArray(node.provenance.derivation_chain),
    )
  );
}
