import { useRef } from 'react';
import type { KeyboardEvent } from 'react';
import type { SceneNode } from './types';

interface HierarchyProps {
  nodes: SceneNode[];
  selectedId: string | null;
  onSelect: (id: string) => void;
}

/** Flat ARIA tree with a roving tabindex; keyboard-only operation is supported. */
export function Hierarchy({ nodes, selectedId, onSelect }: HierarchyProps) {
  const items = useRef<Array<HTMLDivElement | null>>([]);

  const focusIndex = (index: number) => {
    const clamped = Math.max(0, Math.min(index, nodes.length - 1));
    items.current[clamped]?.focus();
    const node = nodes[clamped];
    if (node) onSelect(node.id);
  };

  const handleKeyDown = (event: KeyboardEvent<HTMLDivElement>, index: number) => {
    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        focusIndex(index + 1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        focusIndex(index - 1);
        break;
      case 'Home':
        event.preventDefault();
        focusIndex(0);
        break;
      case 'End':
        event.preventDefault();
        focusIndex(nodes.length - 1);
        break;
      case 'Enter':
      case ' ':
        event.preventDefault();
        if (nodes[index]) onSelect(nodes[index].id);
        break;
      default:
        break;
    }
  };

  return (
    <nav aria-label="Scene hierarchy">
      <div role="tree" aria-label="Scene nodes" className="hierarchy">
        {nodes.map((node, index) => {
          const selected = node.id === selectedId;
          return (
            <div
              key={node.id}
              ref={(element) => {
                items.current[index] = element;
              }}
              role="treeitem"
              aria-level={1}
              aria-selected={selected}
              tabIndex={selected || (selectedId === null && index === 0) ? 0 : -1}
              className={selected ? 'treeitem selected' : 'treeitem'}
              onClick={() => onSelect(node.id)}
              onKeyDown={(event) => handleKeyDown(event, index)}
            >
              {node.label}
            </div>
          );
        })}
      </div>
    </nav>
  );
}
