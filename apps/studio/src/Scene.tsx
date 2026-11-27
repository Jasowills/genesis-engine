import { Canvas } from '@react-three/fiber';
import { useMemo } from 'react';
import type { SceneNode, SceneSnapshot, Vec3 } from './types';

export interface RenderCapability {
  webgl2: boolean;
  webgpu: boolean;
}

/**
 * Environment probe only. This is not a domain rule and not a claim that any
 * particular webview supports rendering; see apps/studio/README.md.
 */
export function probeRenderCapability(): RenderCapability {
  if (typeof document === 'undefined') return { webgl2: false, webgpu: false };
  let webgl2 = false;
  try {
    webgl2 = document.createElement('canvas').getContext('webgl2') !== null;
  } catch {
    webgl2 = false;
  }
  return {
    webgl2,
    webgpu: typeof navigator !== 'undefined' && 'gpu' in navigator,
  };
}

function MeshNode({
  node,
  selected,
}: {
  node: SceneNode;
  selected: boolean;
}) {
  const { positions, indices } = useMemo(
    () => ({
      positions: new Float32Array(node.mesh.vertices.flat()),
      indices: new Uint32Array(node.mesh.indices),
    }),
    [node.mesh],
  );

  return (
    <mesh position={node.translation}>
      <bufferGeometry>
        <bufferAttribute attach="attributes-position" args={[positions, 3]} />
        <bufferAttribute attach="index" args={[indices, 1]} />
      </bufferGeometry>
      <meshBasicMaterial color={selected ? '#f59e0b' : '#64748b'} />
    </mesh>
  );
}

interface SceneProps {
  snapshot: SceneSnapshot;
  selectedId: string | null;
  transforms: Record<string, Vec3>;
}

export function Scene({ snapshot, selectedId, transforms }: SceneProps) {
  const capability = useMemo(probeRenderCapability, []);

  if (!capability.webgl2) {
    return (
      <section className="viewport" aria-label="3D viewport">
        <p role="status">
          WebGL2 is unavailable here; showing scene metadata only (
          {capability.webgpu ? 'WebGPU reported' : 'no WebGPU reported'}).
        </p>
        <p>{snapshot.nodes.length} mesh node(s) are available in the hierarchy.</p>
      </section>
    );
  }

  return (
    <section className="viewport" aria-label="3D viewport">
      <Canvas camera={{ position: [0, 0, 120], fov: 50 }}>
        <color attach="background" args={['#0b1020']} />
        {snapshot.nodes.map((node) => (
          <MeshNode
            key={node.id}
            node={{ ...node, translation: transforms[node.id] ?? node.translation }}
            selected={node.id === selectedId}
          />
        ))}
      </Canvas>
    </section>
  );
}
