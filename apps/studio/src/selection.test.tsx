import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { Hierarchy } from './Hierarchy';
import { Inspector } from './Inspector';
import { App } from './App';
import { isSceneSnapshot } from './types';
import type { SceneNode, SceneSnapshot } from './types';

function makeNode(id: string, label: string, selectable = true, evidence: SceneNode['evidence'] = 'measured'): SceneNode {
  return {
    id,
    label,
    kind: 'building_volume',
    selectable,
    translation: [12.5, -4.0, 0.0],
    origin_lon_lat: [3.3516, 6.6018],
    evidence,
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
}

const first = makeNode('id-building-1', 'Building 1');
const second = makeNode('id-road-2', 'Road 2', true, 'inferred_by_rule');
const disabled = makeNode('id-disabled-3', 'Unselectable 3', false);
const snapshot: SceneSnapshot = {
  snapshot_version: 1,
  world_id: 'id-world',
  frame_id: 'ikeja-local',
  selected_node_id: null,
  nodes: [first, second, disabled],
};

describe('scene selection and provenance', () => {
  it('keyboard Enter selects the exact stable ID of the focused node, not always the first', async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(<Hierarchy nodes={[first, second]} selectedId={null} onSelect={onSelect} />);
    screen.getByRole('treeitem', { name: 'Road 2' }).focus();
    await user.keyboard('{Enter}');
    expect(onSelect).toHaveBeenCalledTimes(1);
    expect(onSelect).toHaveBeenCalledWith('id-road-2');
  });

  it('ArrowDown moves selection to the next stable ID', async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(<Hierarchy nodes={[first, second]} selectedId={null} onSelect={onSelect} />);
    screen.getByRole('treeitem', { name: 'Building 1' }).focus();
    await user.keyboard('{ArrowDown}');
    expect(onSelect).toHaveBeenCalledWith('id-road-2');
  });

  it('does not select nodes marked unselectable by Core', async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(<Hierarchy nodes={[disabled]} selectedId={null} onSelect={onSelect} />);
    screen.getByRole('treeitem', { name: 'Unselectable 3' }).focus();
    await user.keyboard('{Enter}');
    expect(onSelect).not.toHaveBeenCalled();
  });

  it('inspector displays source, attribution, licence, method, evidence and derivation chain', () => {
    render(<Inspector node={first} translation={first.translation} onTranslationChange={vi.fn()} />);
    expect(screen.getByText('osm:pbf:ikeja')).toBeTruthy();
    expect(screen.getByText('© OpenStreetMap contributors')).toBeTruthy();
    expect(screen.getByText('ODbL-1.0')).toBeTruthy();
    expect(screen.getByText('osm:height')).toBeTruthy();
    expect(screen.getByText('measured')).toBeTruthy();
    expect(screen.getByText('clip')).toBeTruthy();
  });

  it('numeric keyboard path accepts negative local positions', async () => {
    const user = userEvent.setup();
    const onChange = vi.fn();
    render(<Inspector node={first} translation={first.translation} onTranslationChange={onChange} />);
    const north = screen.getByLabelText('North (m) for Building 1') as HTMLInputElement;
    await user.clear(north);
    await user.type(north, '-7.5');
    expect(onChange).toHaveBeenCalledWith('id-building-1', [12.5, -7.5, 0]);
  });

  it('selecting a node in the shell reveals its provenance', async () => {
    const user = userEvent.setup();
    render(<App snapshot={snapshot} />);
    screen.getByRole('treeitem', { name: 'Road 2' }).focus();
    await user.keyboard('{Enter}');
    expect(screen.getByText('© OpenStreetMap contributors')).toBeTruthy();
    expect(screen.getByText('inferred_by_rule')).toBeTruthy();
  });
});


describe('keyboard and boundary input', () => {
  it('ArrowUp, Home and End move to clamped stable IDs', async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(<Hierarchy nodes={[first, second]} selectedId={null} onSelect={onSelect} />);
    screen.getByRole('treeitem', { name: 'Road 2' }).focus();
    await user.keyboard('{ArrowUp}');
    expect(onSelect).toHaveBeenLastCalledWith('id-building-1');
    screen.getByRole('treeitem', { name: 'Road 2' }).focus();
    await user.keyboard('{Home}');
    expect(onSelect).toHaveBeenLastCalledWith('id-building-1');
    screen.getByRole('treeitem', { name: 'Building 1' }).focus();
    await user.keyboard('{End}');
    expect(onSelect).toHaveBeenLastCalledWith('id-road-2');
  });

  it('Space selects and aria-selected reflects the selected node', async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(<Hierarchy nodes={[first, second]} selectedId={first.id} onSelect={onSelect} />);
    expect(screen.getByRole('treeitem', { name: 'Building 1' }).getAttribute('aria-selected')).toBe('true');
    screen.getByRole('treeitem', { name: 'Road 2' }).focus();
    await user.keyboard(' ');
    expect(onSelect).toHaveBeenCalledWith('id-road-2');
  });

  it('clicking a selectable node selects it', async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    render(<Hierarchy nodes={[disabled, first]} selectedId={null} onSelect={onSelect} />);
    await user.click(screen.getByRole('treeitem', { name: 'Building 1' }));
    expect(onSelect).toHaveBeenCalledWith('id-building-1');
    await user.click(screen.getByRole('treeitem', { name: 'Unselectable 3' }));
    expect(onSelect).toHaveBeenCalledTimes(1);
  });

  it('intermediate numeric input is not committed and blur restores the original', async () => {
    const user = userEvent.setup();
    const onChange = vi.fn();
    render(<Inspector node={first} translation={first.translation} onTranslationChange={onChange} />);
    const north = screen.getByLabelText('North (m) for Building 1') as HTMLInputElement;
    await user.clear(north);
    await user.type(north, '-');
    expect(onChange).not.toHaveBeenCalled();
    await user.tab();
    expect(north.value).toBe(String(first.translation[1]));
  });

  it('injected snapshots with a bad version or shape are rejected', () => {
    expect(isSceneSnapshot(snapshot)).toBe(true);
    expect(isSceneSnapshot({ ...snapshot, snapshot_version: 2 })).toBe(false);
    expect(isSceneSnapshot({ ...snapshot, nodes: [{ ...first, translation: [1] }] })).toBe(false);
    expect(isSceneSnapshot(null)).toBe(false);
  });
});
