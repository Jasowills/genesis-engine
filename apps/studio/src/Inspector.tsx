import type { ChangeEvent } from 'react';
import type { SceneNode, Vec3 } from './types';

interface InspectorProps {
  node: SceneNode | null;
  translation: Vec3 | null;
  onTranslationChange: (id: string, value: Vec3) => void;
}

const AXES = ['East (m)', 'North (m)', 'Up (m)'] as const;

export function Inspector({ node, translation, onTranslationChange }: InspectorProps) {
  if (!node || !translation) {
    return (
      <aside aria-label="Inspector">
        <p role="status">Select a scene node to inspect its recorded metadata.</p>
      </aside>
    );
  }

  const handleChange = (axis: number, event: ChangeEvent<HTMLInputElement>) => {
    const next = [...translation] as Vec3;
    const value = Number(event.target.value);
    next[axis] = Number.isFinite(value) ? value : 0;
    onTranslationChange(node.id, next);
  };

  return (
    <aside aria-label="Inspector" className="inspector">
      <h2>{node.label}</h2>
      <p>
        Stable ID: <code>{node.id}</code>
      </p>
      <fieldset>
        <legend>Local translation — keyboard-numeric alternative</legend>
        {AXES.map((label, axis) => (
          <label key={label} className="axis">
            {label}{' '}
            <input
              type="number"
              step="0.1"
              inputMode="decimal"
              value={translation[axis]}
              aria-label={`${label} for ${node.label}`}
              onChange={(event) => handleChange(axis, event)}
            />
          </label>
        ))}
      </fieldset>
      <p className="hint">Edits are a local preview only; Core data is not changed.</p>
      <dl>
        <dt>Source</dt>
        <dd>{node.provenance.source_id}</dd>
        <dt>Attribution</dt>
        <dd>{node.provenance.attribution}</dd>
        <dt>Licence</dt>
        <dd>{node.provenance.licence_elected}</dd>
        <dt>Method</dt>
        <dd>{node.method}</dd>
        <dt>Confidence</dt>
        <dd>{node.evidence}</dd>
        <dt>Local-frame anchor (lon, lat)</dt>
        <dd>{node.origin_lon_lat.join(', ')}</dd>
      </dl>
      <h3>Derivation chain</h3>
      <ul>
        {node.provenance.derivation_chain.map((step, index) => (
          <li key={`${step}-${index}`}>{step}</li>
        ))}
      </ul>
    </aside>
  );
}
