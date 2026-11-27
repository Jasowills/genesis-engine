import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Hierarchy } from './Hierarchy';
import { Inspector } from './Inspector';
import { App } from './App';
import type { SceneNode, SceneSnapshot } from './types';

const node: SceneNode = {
  id: 'id-building-1',
  label: 'Building 1',
  kind: 'building_volume',
  selectable: true,
  translation: [12.5, -4.0, 0.0],
  origin_lon_lat: [3.3516, 6.6018],
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
    licence_url: 'https://opendatacommons.org/licenses/odbl/1-0/',
    vintage: '2026-11-27',
    filters: [],
    checksum: null,
    source_feature_ids: ['way/1'],
    derivation_chain: ['clip'],
    derivative_database_status: null,
    display_obligations: [],
    third_party_terms: [],
    recorded_at_unix: null,
  },
};

const snapshot: SceneSnapshot = {
  snapshot_version: 1,
  world_id: 'id-world',
  frame_id: 'ikeja-local',
  selected_node_id: null,
  nodes: [node],
};

describe('scene selection and provenance', () => {
  it('keyboard path selects the stable node ID', async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(<Hierarchy nodes={[node]} selectedId={null} onSelect={onSelect} />);
    const item = screen.getByRole('treeitem', { name: 'Building 1' });
    item.focus();
    await user.keyboard('{Enter}');
    expect(onSelect).toHaveBeenCalledWith('id-building-1');
  });

  it('inspector displays source, attribution, licence and method', () => {
    render(<Inspector node={node} translation={node.translation} onTranslationChange={vi.fn()} />);
    expect(screen.getByText('osm:pbf:ikeja')).toBeTruthy();
    expect(screen.getByText('© OpenStreetMap contributors')).toBeTruthy();
    expect(screen.getByText('ODbL-1.0')).toBeTruthy();
    expect(screen.getByText('osm:height')).toBeTruthy();
  });

  it('numeric keyboard path accepts negative local positions', async () => {
    const user = userEvent.setup();
    const onChange = vi.fn();
    render(<Inspector node={node} translation={node.translation} onTranslationChange={onChange} />);
    const north = screen.getByLabelText('North (m) for Building 1') as HTMLInputElement;
    await user.clear(north);
    await user.type(north, '-7.5');
    expect(onChange).toHaveBeenCalledWith('id-building-1', [12.5, -7.5, 0]);
  });

  it('selecting a node in the shell reveals its provenance', async () => {
    const user = userEvent.setup();
    render(<App snapshot={snapshot} />);
    const item = screen.getByRole('treeitem', { name: 'Building 1' });
    item.focus();
    await user.keyboard('{Enter}');
    expect(screen.getByText('© OpenStreetMap contributors')).toBeTruthy();
  });
});
