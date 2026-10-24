// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { backend, type AssetSlot } from '../api';
import { SlotInfo } from './SlotInfo';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });

async function show(id: string): Promise<HTMLElement> {
  const slot = (await backend.assetsList()).slots.find((s) => s.id === id) as AssetSlot;
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><SlotInfo slot={slot} /></Theme>));
  await flush();
  return host;
}

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
});

describe('SlotInfo', () => {
  it('zeigt Gruppen eines vollständigen Slots', async () => {
    const h = await show('mount:horse');
    expect(h.textContent).toContain('Sheet');
    expect(h.textContent).toContain('Atlas');
    expect(h.textContent).toContain('100×65');
  });

  it('zeigt den Hinweis eines Slots ohne Zuordnung', async () => {
    const h = await show('tavern');
    expect(h.querySelector('[role="note"]')?.textContent).toContain('keine Grafik');
    expect(h.textContent).not.toContain('Atlas');
  });
});
