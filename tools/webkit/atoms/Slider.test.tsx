// @vitest-environment happy-dom
import { Slider as ThemesSlider, Theme } from '@radix-ui/themes';
import { act, useState } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { Slider as RadixSlider } from './index';
import { ValueSlider } from './Slider';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  document.body.innerHTML = '';
});

function Probe({ start = 3, seen }: { start?: number; seen: number[] }) {
  const [v, setV] = useState(start);
  return <ValueSlider aria-label="Frame" value={v} min={0} max={7} onChange={(x) => { seen.push(x); setV(x); }} />;
}

async function mount(seen: number[], start?: number): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><Probe start={start} seen={seen} /></Theme>));
  return host.querySelector<HTMLElement>('[role="slider"]')!;
}

const key = (el: HTMLElement, k: string) => act(async () => { el.dispatchEvent(new KeyboardEvent('keydown', { key: k, bubbles: true })); });

describe('ValueSlider', () => {
  it('lässt den rohen Radix-Slider der Atome unverändert (Studio nutzt die Listen-API)', () => {
    expect(RadixSlider).toBe(ThemesSlider);
  });

  it('zeigt einen Wert mit Grenzen und Namen (aria)', async () => {
    const thumb = await mount([]);
    expect(thumb.getAttribute('aria-valuenow')).toBe('3');
    expect(thumb.getAttribute('aria-valuemin')).toBe('0');
    expect(thumb.getAttribute('aria-valuemax')).toBe('7');
    expect(document.querySelector('[aria-label="Frame"]')?.contains(thumb)).toBe(true);
  });

  it('meldet Änderungen als Zahl und bleibt in den Grenzen', async () => {
    const seen: number[] = [];
    const thumb = await mount(seen, 6);
    await key(thumb, 'ArrowRight');
    expect(thumb.getAttribute('aria-valuenow')).toBe('7');
    await key(thumb, 'ArrowRight');
    await key(thumb, 'Home');
    expect(seen).toEqual([7, 0]);
  });
});
