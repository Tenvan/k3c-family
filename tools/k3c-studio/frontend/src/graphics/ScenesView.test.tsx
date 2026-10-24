// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { ScenesView } from './ScenesView';

// B-599: Die Grafik-Szenen nutzen den Filter-Baustein (ListFilter) und merken die Auswahl je Szene.
// credits.ts liest public/**/CREDITS.md?raw; das Vitest des Studios liefert nichts außerhalb seiner Wurzel aus.
vi.mock('@game/tools/credits', () => ({ CREDITS: [] }));
vi.mock('./SlotCards', () => ({ FigurenCards: () => null, ImageCard: ({ r }: { r: { slot: { id: string } } }) => <i data-slot>{r.slot.id}</i> }));

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  localStorage.clear();
});

describe('Grafik-Szenen: Filter (B-599)', () => {
  it('zeigt Suche und Zähler „n von m“ und merkt die Auswahl unter dem Schlüssel der Szene', async () => {
    localStorage.setItem('k3c-studio.grafik.szene', 'objekte');
    vi.stubGlobal('fetch', vi.fn(async () => ({ ok: false })));
    host = document.createElement('div');
    document.body.append(host);
    root = createRoot(host);
    await act(async () => root!.render(<Theme><ScenesView /></Theme>));
    await act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });
    expect(host.querySelector('input[aria-label="Suchen"]')).not.toBeNull();
    expect(host.querySelector('[role="status"]')?.textContent).toMatch(/^\d+ von \d+$/);
    expect(localStorage.getItem('k3c-studio.grafik.objekte.filter')).not.toBeNull();
    vi.unstubAllGlobals();
  });
});
