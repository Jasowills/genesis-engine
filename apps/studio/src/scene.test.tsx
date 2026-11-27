import { afterEach, describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Scene, probeRenderCapability } from './Scene';
import type { SceneSnapshot } from './types';

const snapshot: SceneSnapshot = {
  snapshot_version: 1,
  world_id: 'world-test',
  frame_id: 'frame-test',
  selected_node_id: null,
  nodes: [{
    id: 'node-1',
    label: 'Building',
    kind: 'building_volume',
    selectable: true,
    translation: [0, 0, 0],
    origin_lon_lat: [3.35, 6.6],
    evidence: 'measured',
    method: 'osm:height',
    mesh: { vertices: [[0, 0, 0], [1, 0, 0], [0, 1, 0]], indices: [0, 1, 2] },
    provenance: {
      source_id: 'osm:pbf:ikeja',
      licence_elected: 'ODbL-1.0',
      attribution: '© OpenStreetMap contributors',
      publisher: 'OpenStreetMap contributors',
      url: 'https://www.openstreetmap.org/copyright',
      version: '2026-11-27',
      licence_url: null,
      vintage: null,
      filters: [],
      checksum: null,
      derivation_chain: [],
      source_feature_ids: [],
      derivative_database_status: null,
      display_obligations: [],
      third_party_terms: [],
      recorded_at_unix: null,
    },
  }],
};

afterEach(() => {
  vi.restoreAllMocks();
});

describe('render capability', () => {
  it('reports WebGL2 and WebGPU from the environment probe', () => {
    vi.spyOn(HTMLCanvasElement.prototype, 'getContext').mockReturnValue({} as CanvasRenderingContext2D);
    Object.defineProperty(navigator, 'gpu', { value: {}, configurable: true });
    expect(probeRenderCapability()).toEqual({ webgl2: true, webgpu: true });
  });

  it('shows an explicit metadata-only fallback when WebGL2 is unavailable', () => {
    vi.spyOn(HTMLCanvasElement.prototype, 'getContext').mockReturnValue(null);
    Reflect.deleteProperty(navigator, 'gpu');
    render(<Scene snapshot={snapshot} selectedId={null} transforms={{}} />);
    const status = screen.getByRole('status');
    expect(status.textContent).toContain('WebGL2 is unavailable here');
    expect(status.textContent).toContain('no WebGPU reported');
    expect(screen.getByText('1 mesh node(s) are available in the hierarchy.')).toBeTruthy();
  });

  it('states WebGPU-reported in the fallback when only WebGPU is available', () => {
    vi.spyOn(HTMLCanvasElement.prototype, 'getContext').mockReturnValue(null);
    Object.defineProperty(navigator, 'gpu', { value: {}, configurable: true });
    render(<Scene snapshot={snapshot} selectedId={null} transforms={{}} />);
    expect(screen.getByRole('status').textContent).toContain('WebGPU reported');
  });
});
