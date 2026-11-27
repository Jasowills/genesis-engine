import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { App } from './App';
import { isSceneSnapshot } from './types';
import type { SceneSnapshot } from './types';

declare global {
  interface Window {
    __GENESIS_SNAPSHOT__?: unknown;
  }
}

const injected: unknown = window.__GENESIS_SNAPSHOT__;
const snapshot: SceneSnapshot | null = isSceneSnapshot(injected) ? injected : null;
const container = document.getElementById('root');

if (!container) throw new Error('Genesis Studio needs a #root element');

createRoot(container).render(
  <StrictMode>
    <App snapshot={snapshot} />
  </StrictMode>,
);
