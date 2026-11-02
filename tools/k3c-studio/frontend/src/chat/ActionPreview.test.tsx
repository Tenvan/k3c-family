// @vitest-environment happy-dom
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import type { ChatAction } from '../api/chatTypes';
import { ActionCard } from './ActionList';

// WZ37.2 (B-683): Override-Karten zeigen je Feld Basiswert → neuer Wert, gesperrte mit Grund und ohne „Ausführen“.
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

async function mountCard(action: ChatAction): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  const noop = async () => {};
  await act(async () => root!.render(<Theme><ul><ActionCard action={action} onRun={noop} onDrop={noop} /></ul></Theme>));
  return host;
}
afterEach(() => { act(() => root?.unmount()); host?.remove(); root = null; host = null; });

const base: ChatAction = { id: 'a1', art: 'override_setzen', titel: 'Override setzen: kind › buildings › wall.cost.wood', details: [], status: 'offen' };
const previews = (h: HTMLElement) => [...h.querySelectorAll('[data-testid="action-preview"]')].map((e) => e.textContent);
const buttons = (h: HTMLElement) => [...h.querySelectorAll('button')].map((b) => b.textContent);

describe('Override-Karten (WZ37.2)', () => {
  it('zeigt Basiswert → neuer Wert und den bisherigen Override', async () => {
    const h = await mountCard({ ...base, vorschau: [
      { feld: 'buildings › wall.cost.wood', basis: '10', alt: '–', neu: '7' },
      { feld: 'waves › perExtraPlayer', basis: '0.3', alt: '0.37', neu: '0.5' },
    ] });
    expect(previews(h)).toEqual(['buildings › wall.cost.wood: Basis 10 → 7', 'waves › perExtraPlayer: Basis 0.3 (bisher 0.37) → 0.5']);
    expect(buttons(h)).toContain('Ausführen');
  });

  it('sperrt einen ungültigen Wert mit Meldung, ohne Ausführen', async () => {
    const h = await mountCard({ ...base, gesperrt: 'data/worlds/kind.json: overrides › data/buildings.json › wall.cost.wood: Ganzzahl erwartet' });
    expect(h.textContent).toContain('Gesperrt: data/worlds/kind.json: overrides › data/buildings.json › wall.cost.wood');
    expect(buttons(h)).not.toContain('Ausführen');
    expect(previews(h)).toEqual([]);
  });
});
