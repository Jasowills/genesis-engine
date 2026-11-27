import { useEffect, useState, type ChangeEvent } from 'react';
import type { SceneNode, Vec3 } from './types';

interface InspectorProps {
  node: SceneNode | null;
  translation: Vec3 | null;
  onTranslationChange: (id: string, value: Vec3) => void;
}

const AXES = ['East (m)', 'North (m)', 'Up (m)'] as const;

export function Inspector({ node, translation, onTranslationChange }: InspectorProps) {
  const [raw, setRaw] = useState<[string, string, string]>(['0', '0', '0']);
  useEffect(() => {
    if (node && translation) setRaw([String(translation[0]), String(translation[1]), String(translation[2])]);
  }, [node?.id, translation]);

  if (!node || !translation) {
    return (
      <aside aria-label="Inspector">
        <p role="status">Select a scene node to inspect its recorded metadata.</p>
      </aside>
    );
  }

  const handleChange = (axis: number, event: ChangeEvent<HTMLInputElement>) => {
    const text = event.target.value;
    const nextRaw = [...raw] as [string, string, string];
    nextRaw[axis] = text;
    setRaw(nextRaw);
    // Keep intermediate states ('-', '.', '1e') editable; commit only finite values.
    if (text.trim() !== '') {
      const value = Number(text);
      if (Number.isFinite(value)) {
        const next = [...translation] as Vec3;
        next[axis] = value;
        onTranslationChange(node.id, next);
      }
    }
  };

  const handleBlur = (axis: number) => {
    if (raw[axis].trim() === '' || !Number.isFinite(Number(raw[axis]))) {
      const nextRaw = [...raw] as [string, string, string];
      nextRaw[axis] = String(translation[axis]);
      setRaw(nextRaw);
    }
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
              value={raw[axis]}
              aria-label={`${label} for ${node.label}`}
              onChange={(event) => handleChange(axis, event)}
              onBlur={() => handleBlur(axis)}
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
