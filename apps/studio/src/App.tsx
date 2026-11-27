import { useState } from 'react';
import { Hierarchy } from './Hierarchy';
import { Inspector } from './Inspector';
import { Scene } from './Scene';
import type { SceneSnapshot, Vec3 } from './types';

interface AppProps {
  snapshot: SceneSnapshot | null;
}

export function App({ snapshot }: AppProps) {
  const [selectedId, setSelectedId] = useState<string | null>(
    snapshot?.selected_node_id ?? null,
  );
  const [transforms, setTransforms] = useState<Record<string, Vec3>>({});

  if (!snapshot) {
    return (
      <main>
        <h1>Genesis Studio</h1>
        <p role="status">
          No scene loaded. Inject a versioned scene snapshot at{' '}
          <code>window.__GENESIS_SNAPSHOT__</code>. The Core snapshot bridge is
          implemented in the Rust crate; this UI currently accepts injected JSON.
        </p>
      </main>
    );
  }

  const selectedNode = snapshot.nodes.find((node) => node.id === selectedId) ?? null;

  return (
    <main>
      <h1>Genesis Studio</h1>
      <p className="meta">
        World <code>{snapshot.world_id}</code> · frame <code>{snapshot.frame_id}</code> · snapshot v
        {snapshot.snapshot_version}
      </p>
      <div className="shell">
        <Hierarchy nodes={snapshot.nodes} selectedId={selectedId} onSelect={setSelectedId} />
        <Scene snapshot={snapshot} selectedId={selectedId} transforms={transforms} />
        <Inspector
          node={selectedNode}
          translation={
            selectedNode ? (transforms[selectedNode.id] ?? selectedNode.translation) : null
          }
          onTranslationChange={(id, value) =>
            setTransforms((previous) => ({ ...previous, [id]: value }))
          }
        />
      </div>
    </main>
  );
}

export default App;
